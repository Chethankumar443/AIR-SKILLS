# AIR.SKILLS — Technical Requirements Document (TRD)

**Version:** 2.0.0 (Production-Grade Draft)
**Status:** Architecture Approved for Implementation
**Project Type:** Cross-Platform CLI Application
**Language:** Rust
**License:** MIT
**Architecture:** Modular Clean Architecture + Plugin-Oriented Skill System
**Supersedes:** TAD v1.0.0

---

## Changelog from TAD v1.0.0

- Concrete data contracts for domain types (§7).
- Merge Engine conflict-resolution algorithm specified (§10).
- Supply-chain security mechanism specified: checksums, pinned tags, zip-slip protection (§14).
- Async/concurrency model fixed: tokio (§4).
- Dependency resolution scoped to semver-range checks for V1 (§11).
- Self-update mechanism for the CLI binary specified (§16).
- Config schema versioning and migration strategy specified (§13).
- Error handling contract with stable error codes (§15).
- CI/testing requirements expanded to production bar (§19, §20).

---

## 1. Vision

AIR.SKILLS is a keyboard-first CLI application that bootstraps AI-powered software projects by downloading verified official Skill Packs from GitHub and generating a unified, auditable AI-ready project workspace.

The architecture must be modular, extensible, testable, offline-friendly (post-installation), cross-platform, fast, memory-efficient, and — the addition this version makes explicit — **verifiable and operable in production/CI contexts**, not just interactive solo use.

---

## 2. Architecture Principles

### Clean Architecture

```
Presentation (TUI/CLI)
        │
Application (Services)
        │
Domain (Pure models)
        │
Infrastructure (GitHub, SQLite, FS, HTTP)
```

Business logic never depends on the terminal UI. Domain crates have zero I/O.

### Dependency Rule

```
TUI → Commands → Application Services → Domain → Infrastructure
```

UI never accesses storage directly; all storage access goes through Application Services.

### Single Responsibility
- GitHub Client → only downloads and verifies skills.
- Merge Engine → only merges and attributes markdown.
- Workspace Generator → only creates project files.

---

## 3. High-Level Architecture

```
                         User
                           │
                           ▼
                   AIR.SKILLS CLI
                           │
     ┌──────────────┬──────────────┬──────────────┐
     ▼              ▼              ▼
 Installer      Commands        TUI
     │              │              │
     └──────────────┼──────────────┘
                    ▼
          Application Services (async, tokio)
                    │
 ┌──────────┬──────────┬──────────┬──────────┐
 ▼          ▼          ▼          ▼
Workspace GitHub     Merge      Registry
Manager   Client     Engine     Client
                    │
                    ▼
             Local SQLite
                    │
                    ▼
             Generated Workspace
```

---

## 4. Concurrency Model

**Decision: tokio, async throughout.**

Rationale: the workload combines network I/O (GitHub downloads, registry refresh), disk I/O (SQLite, archive extraction, template copying), and a TUI that must render live progress bars during long-running downloads. A synchronous model would require manual thread management to keep the UI responsive during downloads; tokio's async model handles this natively and is the standard choice for `reqwest` + `ratatui` combinations in the Rust ecosystem.

Implementation requirements:
- All I/O-bound crates (`air-github`, `air-storage`, `air-workspace`) expose `async fn` APIs.
- `air-domain` remains fully synchronous and I/O-free (pure data + pure functions) — no `async` leaks into domain logic.
- `air-tui` runs its own event loop on the tokio runtime, using `tokio::select!` to multiplex keyboard input against download-progress channels.
- Long-running operations (download, extraction, merge) report progress via `tokio::sync::mpsc` channels consumed by the TUI's progress bar widgets.

---

## 5. Repository & Workspace Structure

```
air-skills/
├── .github/
├── docs/
├── crates/
│   ├── air-cli/
│   ├── air-core/
│   ├── air-domain/
│   ├── air-storage/
│   ├── air-github/
│   ├── air-merge/
│   ├── air-workspace/
│   ├── air-tui/
│   ├── air-utils/
│   ├── air-config/
│   └── air-registry/
├── examples/
├── scripts/
├── assets/
├── website/
├── README.md
└── Cargo.toml
```

---

## 6. Crate Responsibilities

| Crate | Responsibility | Key dependencies |
|---|---|---|
| `air-cli` | Entry point, CLI parsing, command routing, banner (with TTY-capability detection), self-update trigger | `clap`, `tokio` |
| `air-tui` | Installer UI, arrow navigation, space selection, progress bars, plain-text fallback mode | `ratatui`, `crossterm`, `inquire` |
| `air-core` | Application layer: Installer, Update, Add/Remove/Search Skill, Doctor services | — |
| `air-domain` | Pure models: `Skill`, `StarterKit`, `Workspace`, `Manifest`, `InstallPlan`, `Configuration` — zero I/O | `semver`, `serde` |
| `air-storage` | SQLite persistence: installed_skills, workspace, cache, history | `sqlx`, SQLite |
| `air-github` | Download, checksum verification, pinned-tag resolution, release lookup, auth (PAT) | `reqwest`, `git2`, `sha2` |
| `air-merge` | Read markdown, detect conflicts, attribute sources, merge deterministically, emit audit trail | — |
| `air-workspace` | Creates `.air/`, `config.json`, `lock.json`, `merged/`, `templates/` | — |
| `air-config` | `config.toml`, environment handling, settings, registry URL, `schema_version` migration | `serde`, `toml` |
| `air-registry` | Registry index cache, search, semver-range compatibility checks | `semver` |
| `air-utils` | Filesystem helpers, hashing, compression, path utilities, version comparison, zip-slip guards | `sha2`, `zip` |

---

## 7. Domain Data Contracts

Concrete shapes:

```rust
// air-domain

pub struct SkillId(String); // e.g. "astro", must match ^[a-z0-9-]+$

pub struct Skill {
    pub id: SkillId,
    pub version: semver::Version,
    pub source_tag: String,        // pinned git tag/release, never a branch
    pub checksum_sha256: String,
    pub category: SkillCategory,
    pub compatible_with: Vec<VersionConstraint>, // semver ranges, not flat strings
    pub files: Vec<SkillFile>,
}

pub struct VersionConstraint {
    pub skill_id: SkillId,
    pub range: semver::VersionReq,
}

pub struct StarterKit {
    pub id: String,
    pub version: semver::Version,     // Starter Kits are versioned artifacts (PRD §10.1)
    pub skills: Vec<(SkillId, semver::VersionReq)>,
}

pub struct InstallPlan {
    pub resolved_skills: Vec<Skill>,
    pub conflicts: Vec<MergeConflict>, // populated pre-install by dry-run merge check
}

pub struct MergeConflict {
    pub section_heading: String,
    pub contributing_skills: Vec<SkillId>,
    pub resolution: ConflictResolution, // AppendWithAttribution | FailStrict
}

pub struct Manifest {
    pub schema_version: u32,
    pub installed_skills: Vec<Skill>,
    pub starter_kit: Option<StarterKit>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

`air-domain` has no knowledge of GitHub, SQLite, or the filesystem — these types are pure data, constructed and consumed by `air-core` application services.

---

## 8. `skill.yaml` Schema (V1)

```yaml
name: astro
version: 1.0.0
description: Astro Frontend
category: frontend
checksum_sha256: <sha256 of release archive>
source_tag: v1.0.0            # pinned; branches are never accepted
compatible_with:
  - skill: rust
    range: ">=1.0.0, <2.0.0"
  - skill: nodejs
    range: ">=18.0.0"
files:
  - system_instructions.md
  - design.md
  - templates/
```

`checksum_sha256` and `source_tag` are **required** fields; a `skill.yaml` missing either fails validation at registry-index build time, never at install time.

---

## 9. Installation Flow (Sequenced)

```
air init
  → capability-detect terminal (ANSI/TUI vs plain-text fallback)
  → banner
  → keyboard help
  → workspace detection (cwd only, no IDE detection)
  → directory confirmation
  → disclaimer
  → Starter Kit OR Custom Setup
  → skill resolution (semver compatibility check against InstallPlan)
  → dry-run merge check → populate InstallPlan.conflicts
  → installation summary (shows conflicts if any, requires explicit ack)
  → download skills (checksum + pinned-tag verified, zip-slip-safe extraction)
  → merge engine (append-with-attribution; write merge-audit.json)
  → workspace generator
  → complete
```

---

## 10. Merge Engine — Conflict Resolution Algorithm

**Section identification:** sections are matched by normalized heading text (case-folded, whitespace-trimmed) within each of the four merge targets (`system_instructions.md`, `design.md`, `architecture.md`, `coding_rules.md`). Content hashing is used only to detect *identical* duplicate sections (safe to silently dedupe — no information loss); textually differing sections under the same heading are treated as a conflict, never silently overwritten.

**Resolution (default — `AppendWithAttribution`):**
1. All contributing skills' versions of the conflicting section are retained.
2. Each is emitted under a sub-heading: `### <Original Heading> (from: <skill-id>)`.
3. An entry is appended to `.air/merge-audit.json`:
   ```json
   {
     "section": "Testing Standards",
     "target_file": "coding_rules.md",
     "contributing_skills": ["rust", "python-backend"],
     "resolution": "append_with_attribution"
   }
   ```
4. `air doctor` reads `merge-audit.json` and surfaces any multi-contributor sections as warnings for manual reconciliation.

**Resolution (`--strict` flag):**
- Any detected conflict aborts the merge step before any output file is written; the CLI exits non-zero with a message identifying the conflicting sections and contributing skills.

**Ordering determinism:** within a merge target, sections are ordered first by the install order of Starter Kit/Custom Setup selection, then alphabetically by skill ID as a stable tie-break — ensuring identical inputs always produce byte-identical output.

---

## 11. Dependency Resolution (V1 Scope)

- Each skill declares `compatible_with` as a list of `(skill_id, VersionReq)` pairs.
- At resolution time, `air-registry` checks that every installed/selected skill's declared ranges are satisfied by the actual versions of its declared peers.
- Flat, one-hop check in V1.

---

## 12. GitHub Integration

- Organization: `github.com/AIR-SKILLS` (hardcoded default; overridable only for internal/enterprise forks via explicit config).
- Every skill is downloaded via its **release archive at a pinned tag**.
- Auth: supports a GitHub PAT via `AIR_GITHUB_TOKEN` env var or `air config set github.token`.
- Retry policy: exponential backoff (base 500ms, max 3 retries).

---

## 13. Configuration & Schema Migration

`.air/config.json` includes a mandatory `schema_version: u32` field from V1 onward.

- Comparison on every CLI invocation with expected version.
- Sequential in-process migration chain with backup before write (`.air/config.json.bak`).

---

## 14. Supply-Chain Security Mechanism

- **Checksum verification:** SHA-256 validation before extraction.
- **Pinned references only:** source_tag enforced.
- **Zip-slip protection:** paths checked against extraction directory.
- **No script execution:** purely static templates & docs.
- **Self-update integrity:** checksummed release artifacts for `air update --self`.

---

## 15. Error Handling

- Libraries: `anyhow`, `thiserror`.
- Stable error code contract: `AIR-GH-001`, `AIR-MRG-002`, `AIR-CFG-003`, etc.

---

## 16. Self-Update Mechanism (CLI Binary)

- `air update` updates skill packs.
- `air update --self` updates CLI binary via `cargo-dist` release artifacts.

---

## 17. SQLite Schema

Tables: `installed_skills`, `workspace`, `cache`, `history`, `starter_kits`.
`sqlx` migrations under version control.

---

## 18. Terminal UI

Library: `ratatui` + `crossterm`.
Capability detection fallback to plain text.

---

## 19. Logging

Library: `tracing`. Log path: `.air/logs/`.

---

## 20. Testing & CI Requirements

- Unit tests: ≥ 90% coverage on `air-domain`.
- Integration tests: End-to-end flows with mocked HTTP layer.
- Snapshot tests: Golden files for markdown merge outputs.
- Security tests: Negative payload and traversal tests.
- Cross-platform CI: Linux, macOS, Windows matrix.

---

## 21. CI/CD Pipeline

Lint -> Test -> Security Audit -> Cross-platform Build -> Package -> Release.

---

## 22. Module Dependency Graph

```
air-cli -> air-tui -> air-core -> [air-registry, air-workspace] -> [air-github, air-merge] -> air-storage -> air-domain
```

---

## 23. Development Phases (V1)

Detailed in PRD & Implementation Plan.

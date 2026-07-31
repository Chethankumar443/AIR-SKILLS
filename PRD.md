# AIR.SKILLS — Product Requirements Document (PRD)

**Version:** 2.0.0 (Production-Grade Draft)
**Status:** Approved for Implementation Planning
**Project Name:** AIR.SKILLS
**Product Type:** Open Source CLI Tool
**License:** MIT
**Owner:** AIR.SKILLS Project
**Supersedes:** PRD v1.0.0 (Foundation Draft)

---

## Changelog from v1.0.0

- Resolved the official-only vs. community-contribution ambiguity (§4.2, §22).
- Added competitive landscape (§6).
- Added risk register (§14).
- Added quantified success metrics (§13), replacing qualitative-only criteria.
- Added privacy/telemetry policy (§15).
- Added uninstall guarantees (§16).
- Added terminal compatibility requirements (§17).
- Added supply-chain security requirements at the product level (§18).
- Elevated "Custom Setup" and CLI commands to full acceptance-criteria specs (§10, §11).

---

## 1. Executive Summary

AIR.SKILLS is an **AI Project Bootstrap & Skill Manager**. It enables developers to bootstrap AI-assisted software projects in minutes by installing curated **Skill Packs** from the official AIR.SKILLS GitHub organization.

Unlike template generators that only scaffold code, AIR.SKILLS installs **development intelligence**:

- Project architecture
- AI system instructions
- Design documents
- Coding standards
- Development rules
- Templates
- Best practices
- Project metadata

AIR.SKILLS creates a standardized, reproducible project workspace that any AI coding assistant can immediately understand — and it is built to production standards from V1: verifiable downloads, deterministic merges, typed error handling, and cross-platform CI, not a scaffolding script that happens to work on the maintainer's machine.

---

## 2. Vision

> **Build Once. Install Anywhere.**

AIR.SKILLS aims to become the official package manager for AI software engineering skills — reusable engineering knowledge, not just reusable code.

---

## 3. Mission

Reduce project setup time from hours to minutes while guaranteeing every project begins with:

- Proper architecture
- Consistent, non-contradictory AI instructions
- Modern development standards
- Officially maintained, verifiable engineering practices
- A supply chain a security-conscious team can actually trust

---

## 4. Product Philosophy

### 4.1 Zero Configuration
The user is productive immediately after installation with no manual setup step.

### 4.2 Official, Closed Ecosystem — Permanently
All Skill Packs are maintained by the AIR.SKILLS team and hosted under `github.com/AIR-SKILLS`. **This is a permanent architectural property, not a V1-only restriction.** In V3, a community *contribution* workflow is introduced (see §22), but it works by review-and-merge into the official org — contributors submit skills for maintainer approval, and only approved skills ever become installable. Direct installation from arbitrary third-party repositories is explicitly and permanently out of scope. This keeps every installable skill on a single, auditable trust boundary.

### 4.3 AI First
Every generated workspace is designed specifically for AI-assisted development.

### 4.4 Modular
Projects are built from independent, versioned Skill Packs.

### 4.5 Safe by Default
AIR.SKILLS never modifies existing project files without explicit user approval, never executes downloaded scripts without validation, and never silently discards a skill's guidance during a merge (see §9, Merge Conflict Policy).

### 4.6 Keyboard First
Every screen is fully navigable using the keyboard, with a documented plain-text fallback where the terminal doesn't support ANSI/TUI rendering (see §17).

### 4.7 Verifiable
Every artifact AIR installs — skill archives, releases, the CLI binary itself — must be checksum-verified and traceable to a pinned, immutable source reference. "Trust, but verify" is a hard product requirement, not an aspiration deferred to V4.

---

## 5. Target Users

- Software engineers
- AI-assisted developers
- Students
- Open source contributors
- Startups
- Hackathon teams
- Freelancers
- **Engineering teams with security/compliance requirements** — a new explicit segment, since production-grade trust guarantees (§18) are a prerequisite for any team-adopted tool

---

## 6. Competitive Landscape

| Tool | What it distributes | Gap AIR.SKILLS fills |
|---|---|---|
| Yeoman / Cookiecutter | Code scaffolding templates | No AI-context layer; no merge engine for combining multiple stacks' guidance |
| `create-react-app` / `create-*` generators | Single-framework starter code | Single-purpose, not composable across stacks |
| `degit` | Git repo templating (file copy only) | No versioning, no dependency/compatibility model, no merge logic |
| AI IDE built-in project init (e.g. Cursor, Copilot Workspace) | Ad hoc, per-session AI instructions | Not portable across tools/editors; not reproducible; no shared official standards |
| Internal company boilerplate repos | Company-specific scaffolds | Not maintained as a product; no cross-project consistency tooling |

**Differentiator:** AIR.SKILLS is the only tool in this set that treats *engineering knowledge* (architecture docs, coding standards, AI instructions) as a first-class, versioned, composable, mergeable artifact — independent of which code editor or AI assistant the developer uses.

---

## 7. Goals

- Standardize AI project initialization across stacks.
- Eliminate repetitive project setup.
- Provide official, versioned engineering guidance.
- Simplify onboarding for new contributors to any project bootstrapped with AIR.
- Maintain a high-quality, curated ecosystem of Skill Packs with a verifiable supply chain.
- Be trustworthy enough for team/company adoption, not just solo hobby use.

---

## 8. Non-Goals

AIR.SKILLS is **not**:

- An IDE extension
- An AI chatbot
- A code editor
- A package manager for programming libraries (npm, cargo, pip, etc.)
- A Git client
- A project hosting platform
- A general-purpose CI/CD system
- A direct installer of unreviewed third-party skill repositories (see §4.2 — permanent, not just V1)

---

## 9. High-Level Workflow

```text
air init
      │
      ▼
Display AIR.SKILLS Banner (ANSI, with plain-text fallback — see §17)
      │
      ▼
Display Keyboard Help
      │
      ▼
Detect Current Workspace
      │
      ▼
Confirm Installation Directory
      │
      ▼
Display Installation Disclaimer
      │
      ▼
Choose Setup Method
      │
      ├──────────────┐
      │              │
      ▼              ▼
 Starter Kit     Custom Setup
      │              │
      └──────┬───────┘
             ▼
Skill Resolution (semver compatibility check — see TRD §Dependency Resolution)
             ▼
Installation Summary
             ▼
Download Official Skill Packs (checksum + pinned-tag verified)
             ▼
Merge Skill Content (append-with-attribution; audit trail written — see §Merge Conflict Policy)
             ▼
Generate Project Workspace
             ▼
Installation Complete
```

### Merge Conflict Policy (product-level requirement)

When two or more installed Skill Packs define content under the same section heading (e.g., two skills both define `## Testing Standards`), AIR.SKILLS **must never silently discard either skill's content**. Default behavior:

- Both versions are retained, each under a sub-heading naming its source skill.
- A merge audit entry is recorded in `.air/merge-audit.json` documenting which skill contributed which section.
- `air doctor` surfaces any unresolved duplicate/contradictory sections as warnings for the user to manually reconcile.
- An opt-in `--strict` flag (for CI or scripted installs) turns any such conflict into a hard installation failure instead of a merge, so automated pipelines never silently ship contradictory AI instructions.

This is a deliberate trade-off: the generated files may occasionally contain redundant content, but the product must never lose engineering guidance without the user's knowledge.

---

## 10. Setup Modes — Acceptance Criteria

### 10.1 Starter Kit (Recommended)

Installs a curated, pre-validated set of mutually compatible Skill Packs in one step.

Example Starter Kits: Modern React Web App, Astro Website, Desktop App (Rust + Tauri), Python Backend API, AI Agent, CLI Tool, MCP Server, Empty Project.

**Acceptance criteria:**
- Every Starter Kit ships with a pinned, tested combination of skill versions — Starter Kits are themselves versioned artifacts, not just a list of "latest" skills.
- Selecting a Starter Kit and confirming must produce a fully installed, mergeable workspace with zero unresolved compatibility errors, or the installer must fail with a clear, actionable error before any files are written.

### 10.2 Custom Setup

Lets users compose their own stack from: Project Type, Frontend, Backend, Database, AI Stack, Testing, Documentation, Deployment.

**Acceptance criteria:**
- Incompatible selections (per each skill's `compatible_with` / semver constraints) are blocked at selection time, not discovered after download.
- Any combination that passes selection must successfully merge or produce a clearly attributed conflict per §9's Merge Conflict Policy — never a silent partial install.

---

## 11. CLI Commands (V1) — Acceptance Criteria

| Command | Purpose | Must satisfy |
|---|---|---|
| `air init` | Initialize a new AIR workspace | Full flow in §9; idempotent — re-running on an already-initialized directory must detect and offer repair, not duplicate state |
| `air add` | Install additional official Skill Packs | Re-runs compatibility check and merge against the *current* installed set, not just the new skill in isolation |
| `air remove` | Remove installed Skill Packs | Regenerates merged output excluding the removed skill's contributed sections; updates `merge-audit.json` |
| `air update` | Update installed Skill Packs to newer compatible versions | Respects semver constraints from other installed skills; never silently jumps a major version |
| `air list` | Display installed Skill Packs | Shows installed version, available update, and source commit/tag pinned |
| `air search` | Search available official Skill Packs | Works offline against a locally cached registry index; refreshes index opportunistically |
| `air doctor` | Validate workspace health and configuration | Detects: schema drift, unresolved merge conflicts, checksum mismatches, stale lock file, missing cache entries |
| `air uninstall` | Remove AIR from the current project | See §16 for exact guarantees |
| `air version` | Display CLI version | — |
| `air help` | Show available commands | — |

---

## 12. Skill Source & Structure

Skill Packs are downloaded **only** from the official AIR.SKILLS GitHub organization (`github.com/AIR-SKILLS`), each following a fixed layout (`skill.yaml`, `README.md`, `system_instructions.md`, `design.md`, `architecture.md`, `coding_rules.md`, plus `templates/`, `examples/`, `assets/`, `snippets/`).

Every download is resolved to a **pinned tag or release**, never a moving branch, and verified against a published checksum before extraction (see §18).

---

## 13. Success Metrics

Quantified targets for V1 (baseline to be measured against, not aspirational prose):

- **Time to working workspace:** ≥ 90% of Starter Kit installs complete in under 5 minutes on a standard broadband connection.
- **Install success rate:** ≥ 98% of `air init` runs with valid input complete without an unhandled error.
- **Merge integrity:** 0% silent content loss across any supported skill combination (validated by golden/snapshot tests — see TRD).
- **Doctor accuracy:** `air doctor` false-positive rate (flagging a healthy workspace as broken) < 2%.
- **Cross-platform parity:** 100% of V1 acceptance tests pass on Windows, macOS, and Linux in CI before release.
- **Supply-chain integrity:** 100% of downloaded skill archives pass checksum verification; any mismatch halts installation (never a warn-and-continue).

---

## 14. Risk Register

| Risk | Impact | Mitigation |
|---|---|---|
| A published Skill Pack ships broken/contradictory instructions | Corrupts generated workspaces downstream | Version pinning + `air update` rollback; Starter Kits are pre-tested combinations, not ad hoc "latest" picks |
| GitHub API rate limiting (unauthenticated: 60 req/hr) | Blocks installs for active users/CI | Support GitHub PAT via config/env var for authenticated higher limits; local registry index caching |
| GitHub outage | Blocks all installs | Offline mode using local skill cache for already-downloaded skills (§ Offline Support) |
| Malicious or compromised release artifact | Supply-chain compromise of user projects | Checksum verification, pinned tags, no execution of downloaded scripts (§18); signed packs planned V4 but checksum baseline is non-negotiable from V1 |
| Merge engine silently drops guidance | Users unknowingly ship incomplete/incorrect AI instructions | Append-with-attribution default, audit trail, `--strict` mode (§9) |
| ANSI banner/TUI fails on legacy Windows terminals | Broken first-run experience | Plain-text fallback required (§17) |
| Config schema changes break existing projects on CLI upgrade | Silent breakage across the installed base | `schema_version` field + migration logic (see TRD) |

---

## 15. Privacy & Telemetry Policy

- **No telemetry is collected by default.**
- An **opt-in** anonymous usage stats mode may be enabled by the user (e.g. `air config set telemetry true`), collecting only: command invoked, success/failure, anonymized skill pack IDs used. No file contents, paths, or project-identifying data are ever collected, opt-in or not.
- Network requests required for operation (GitHub downloads, registry index refresh) are not considered telemetry and occur regardless, but never include project content.

---

## 16. Uninstall Guarantees

`air uninstall` **will**:
- Remove the `.air/` directory and all AIR-managed configuration/cache/lock files.
- Leave `merged/` output files in place unless the user explicitly passes `--purge-generated`, since those files may already be referenced by the user's own AI tooling.

`air uninstall` **will not**:
- Modify, revert, or delete any file outside `.air/` (including files copied out of `templates/` into the user's source tree).
- Attempt to undo edits the user made to generated files after installation.

This distinction must be stated to the user at uninstall time, not just documented.

---

## 17. Terminal Compatibility Requirements

- Primary experience: full `ratatui`-based keyboard-navigable TUI with ANSI banner.
- **Required fallback:** on terminals without ANSI/TUI capability detection (e.g., legacy `cmd.exe`), AIR.SKILLS must degrade to a plain-text, line-based prompt flow rather than rendering broken escape sequences. Capability detection happens at startup, before the banner is drawn.

---

## 18. Supply-Chain Security (Product-Level Requirements)

These are product requirements, not just implementation details (full mechanism in TRD):

- Every skill archive download is checksum-verified against a value published in the registry index.
- Every download resolves to a pinned tag/release, never a floating branch reference.
- No downloaded script or template content is ever executed by AIR.SKILLS itself.
- The CLI binary's own release artifacts are checksummed and the install script verifies them before replacing an existing binary (`air update` for self).
- Signed Skill Packs (cryptographic signatures, not just checksums) are on the V4 roadmap, but checksum + pinned-tag verification is a V1 launch requirement, not deferred.

---

## 19. Safety Principles

AIR.SKILLS must never:
- Delete user files outside `.air/` (except with explicit `--purge-generated` on uninstall).
- Modify application source code automatically.
- Execute downloaded scripts without validation.
- Install unverified Skill Packs (checksum/pin failure halts installation).
- Upload project data automatically, ever, regardless of telemetry setting.

---

## 20. Future Roadmap

### V1 — Production Launch
Core CLI, official Skill Packs, Starter Kits, Custom Setup, Merge Engine with attribution + audit trail, workspace generation, checksum/pinned-tag verification, cross-platform CI, opt-in telemetry.

### V2
Skill version pinning refinements, full transitive dependency resolution (beyond V1's semver-range checks), compatibility validation improvements, interactive search, GitHub PAT-based auth flow polish.

### V3
Official Skill Registry website, documentation portal, opt-in skill analytics, **community contribution workflow with maintainer approval** — contributed skills are reviewed and, if approved, merged into the official `AIR-SKILLS` org. This does not relax §4.2: only org-hosted, approved skills are ever installable.

### V4
Signed Skill Packs (cryptographic signing on top of V1's checksum baseline), incremental updates, expanded offline cache, workspace synchronization, enterprise support (SSO for private skill registries, audit logging for compliance).

---

## 21. Product Vision (Restated)

AIR.SKILLS distributes structured engineering knowledge — architecture, documentation, coding standards, and AI context — through an officially curated, cryptographically verifiable ecosystem of Skill Packs. It is designed from V1 to be adopted by teams with real security and reproducibility requirements, not just as a convenience script for solo projects: every download is verified, every merge is auditable, every configuration is safely migratable, every error is stably coded, and every platform is tested in CI before release.

# AIR.SKILLS — Agent Execution Memory Log

**Project**: AIR.SKILLS  
**Status**: Active — v1.1 Production Hardening & V1 Release Candidate Complete (Phases 0–35 100% Verified) ✅  
**Last Updated**: 2026-07-31  

---

## Overview

This memory file tracks all architectural decisions, implemented components, crate structures, domain models, service traits, error taxonomies, and completed/pending milestones for **AIR.SKILLS**. Any AI agent can read this file to immediately resume development without missing context.

---

## Pin-to-Pin Execution Log (Phases 0 through 35)

### Phase 0 — Project Planning & Specifications
- Created comprehensive architecture specifications: `PRD.md`, `TRD.md`, `PROJECT_STRUCTURE.md`, `DECISIONS.md`, `ROADMAP.md`, `SECURITY.md`, `GOVERNANCE.md`.
- Defined AIR's core philosophy: *"AIR is not an IDE plugin. AIR is not an AI assistant. AIR is a Project Bootstrap & Skill Manager for AI-driven software engineering."*

### Phase 1 — GitHub Setup & CI/CD Templates
- Established repository structure, `.gitignore`, `.github/workflows/ci.yml`, `.github/workflows/audit.yml`, `.github/workflows/release.yml`.
- Created GitHub issue templates (`bug_report.md`, `feature_request.md`) and pull request template (`pull_request_template.md`).

### Phase 2 — Cargo Workspace & Toolchain Config
- Configured root `Cargo.toml` with Cargo Workspace workspace dependencies (`tokio`, `serde`, `serde_json`, `crossterm`, `clap`, `thiserror`, `sha2`, `semver`, `chrono`).
- Pinned toolchain in `rust-toolchain.toml` and configured `rustfmt.toml`, `clippy.toml`, `deny.toml`, `.taplo.toml`.

### Phase 3 & 4 — Folder Structure & Core Crates Creation
- Scaffolding 11 modular crates:
  - `crates/air-domain`
  - `crates/air-core`
  - `crates/air-config`
  - `crates/air-storage`
  - `crates/air-github`
  - `crates/air-registry`
  - `crates/air-merge`
  - `crates/air-workspace`
  - `crates/air-utils`
  - `crates/air-tui`
  - `crates/air-cli`

### Phase 5 — Shared Domain Models (`air-domain`)
- Built pure zero-I/O domain models: `Skill`, `SkillId`, `SkillCategory`, `StarterKit`, `Workspace`, `Manifest`, `ProgressEvent`, `ValidationReport`, `ValidationResult`, `RegistryIndex`.

### Phase 6 — Public Interfaces & Service Traits (`air-core`)
- Defined application service traits: `InstallService`, `RegistryService`, `RepositoryService`, `MergeService`, `WorkspaceService`, `UninstallService`.

### Phase 7 — Configuration Schemas (`air-config`)
- Built `WorkspaceConfig` schema parser with version migrations and schema compliance checks.

### Phase 8 — Storage & Persistence Engine (`air-storage`)
- Implemented `StorageRepository` trait and `InMemoryStorageRepository` for local caching and skill pack storage.

### Phase 9 — GitHub Downloader & Checksum Manager (`air-github`)
- Implemented `GitHubClient` with archive release tarball downloading, SHA-256 digest validation (`sha256_digest`), and pinned release tag verification.

### Phase 10 — Registry Index & Semver Resolver (`air-registry`)
- Built `RegistryClient` supporting `search()`, `get_skill()`, `get_starter_kit()`, `validate()`, and semver range compatibility resolution.

### Phase 11 — AST Markdown Merge Engine (`air-merge`)
- Built heading section AST parser (`#`, `##`, `###`), paragraph and bullet point deduplication engine, source attribution comments (`<!-- Source: <skill_id> -->`), and strict conflict detection (`StrictConflict`).

### Phase 12 — Target Workspace Generator (`air-workspace`)
- Built `WorkspaceGenerator::generate()` producing deterministic project metadata (`.air/lock.json`, `.air/config.json`, `.air/merge-audit.json`, `system_instructions.md`, `design.md`, `README.md`).

### Phase 13 — Core Orchestrator Services (`air-core`)
- Built `DefaultInstallService`, `DefaultRegistryService`, `DefaultRepositoryService`, `DefaultWorkspaceService`, and `DefaultMergeService`.

### Phase 14 — CLI Command Parser & App Services (`air-cli`)
- Implemented `clap` command parser routing CLI arguments to application services.

### Phase 15 — Core Domain TUI Wizard Implementation (`air-tui`)
- Implemented Crossterm keyboard navigation (`↑`/`↓`, `←`/`→`, `Space`, `Enter`, `Esc`, `Tab`, `Ctrl+C`).
- Created initial step sequence: `Splash` → `EnvCheck` → `Welcome` → `WorkspaceDetect` → `Disclaimer` → `SetupModeChoice` → `KitOrSkillSelect` → `PreInstallSummary` → `Downloading` → `FinalScreen`.

### Phase 16 — Registry Format, Search & PRD §229 Validation (`air-registry`)
- Enforced Skill Pack directory format (`skill.yaml`, `README.md`, `system.md`, `design.md`, `templates/`, `hooks/`).
- Strictly enforced PRD §229 rules requiring `checksum_sha256` and `source_tag` in `skill.yaml`.

### Phase 17 — RepositoryService & Checksum Verification (`air-github` / `air-core`)
- Abstracted `RepositorySource` enum supporting `GitHub`, `GitLab`, `LocalFolder`, `ZipFile`, and `PrivateRegistry`.

### Phase 18 — Async Event Progress Pipeline (`air-core` / `air-tui`)
- Connected `DefaultInstallService::install_with_progress()` over Tokio `mpsc::channel` streaming `ProgressEvent` states to TUI in real-time.

### Phase 19 — Deterministic Workspace Scaffolding (`air-workspace`)
- Ensured workspace generation produces reproducible file outputs with stable sorting and metadata attribution.

### Phase 20 — Advanced Section Attribution & Audit Logging (`air-merge`)
- Generated `.air/merge-audit.json` reporting total merged sections, deduplicated paragraphs, and conflicting headers.

### Phase 21 — Complete 11-Command CLI Suite (`air-cli`)
- Implemented all 11 subcommands (`init`, `add`, `remove`, `search`, `update`, `doctor`, `list`, `clean`, `cache`, `uninstall`, `version`) using application service interfaces exclusively.

### Phase 22 — TUI Keyboard Matrix & Live Progress Channel (`air-tui`)
- Fixed line-wrap cursor drift, prevented ghost text stacking, and connected live progress rendering for install events.

### Phase 23 — Testing Suite (Unit, Integration, Snapshot, Golden)
- Created golden snapshot test suite (`tests/golden/golden_tests.rs`) and snapshot merge deduplication tests (`tests/snapshot/merge_snapshot_test.rs`).

### Phase 24 — Documentation & Example Skill Packs (`docs/`, `examples/`)
- Created example skill pack (`examples/skills/react-skill/`), tutorial guide (`examples/WALKTHROUGH.md`), and CLI reference manual (`docs/cli/reference.md`).

### Phase 25 — Package Managers & Multi-Target Release Pipelines
- Created Unix install script (`scripts/install.sh`), Windows PowerShell script (`scripts/install.ps1`), Homebrew formula (`packaging/homebrew/air.rb`), Scoop manifest (`packaging/scoop/air.json`), and GitHub Actions release workflow (`.github/workflows/release.yml`).

### Phase 26 — Safe Uninstall & Product Hardening (`crates/air-core/src/services/uninstall_service.rs`)
- Implemented `DefaultUninstallService` and `air uninstall [--purge-generated]`. Purges `.air/` while leaving **100% of user project code untouched**.

### Phase 27 & 28 — Installer Finalization & Reinstall Safety
- Standardized wizard transitions with plain-text fallback (`--plain`). Enforced reinstall-safe workspace generation.

### Phase 29 & 30 — Merge Audit Diagnostics & Registry Validation Audit
- Integrated `MergeAudit` analysis into `air doctor`. Enforced PRD §229 validation for Skill Packs (`checksum_sha256`, `source_tag`).

### Phase 31 — CLI Command Completion & Uniform Formatting
- Standardized command output styling and error recovery across all 11 CLI subcommands.

### Phase 32 — Negative & Security Testing Expansion (`tests/integration/phase32_hardening_test.rs`)
- Added negative tests covering safe uninstall code isolation, checksum mismatch rejection, invalid YAML manifest recovery, duplicate skill addition, and broken lock file detection.

### Phase 33 & 34 — Documentation & CODEOWNERS Governance Matrix
- Updated `README.md` and created `.github/CODEOWNERS` establishing crate code ownership.

### Phase 35 — V1 Release Candidate & Cross-Target Verification
- Ran full workspace test suite (`cargo test --workspace` & `cargo test --tests`) — 100% passed cleanly.

---

## Phase Execution Checklist

- [x] **Phase 0 — Project Planning & Documentation Suite**
- [x] **Phase 1 — GitHub Setup & Templates**
- [x] **Phase 2 — Cargo Workspace & Toolchain Config**
- [x] **Phase 3 & 4 — Folder Structure & Core Crates Creation**
- [x] **Phase 5 — Shared Domain Models (`air-domain`)**
- [x] **Phase 6 — Public Interfaces / Traits (`air-core`)**
- [x] **Phase 7 — Configuration Schemas (`air-config`)**
- [x] **Phase 8 — Storage & SQLite Engine (`air-storage`)**
- [x] **Phase 9 — GitHub Downloader & Tarball Manager (`air-github`)**
- [x] **Phase 10 — Registry Index & Semver Range Resolver (`air-registry`)**
- [x] **Phase 11 — Markdown Merge Engine (`air-merge`)**
- [x] **Phase 12 — Workspace Generator (`air-workspace`)**
- [x] **Phase 13 — Core Orchestrator Services (`air-core`)**
- [x] **Phase 14 — CLI Commands (`air-cli`)**
- [x] **Phase 15 — Core Domain Implementation (`air init` TUI Installer)**
- [x] **Phase 16 — Registry Format, Search, Lookup & Validation Services (`air-registry`)**
- [x] **Phase 17 — GitHub Downloader & RepositoryService (`air-github` & `air-core`)**
- [x] **Phase 18 — Installer Engine End-to-End Event Pipeline (`air-core` & `air-tui`)**
- [x] **Phase 19 — Deterministic Workspace Generator (`air-workspace`)**
- [x] **Phase 20 — Markdown Merge Engine & Section Attribution (`air-merge`)**
- [x] **Phase 21 — Complete CLI Commands Suite (`air-cli`)**
- [x] **Phase 22 — TUI & Real-time Progress Pipeline (`air-tui`)**
- [x] **Phase 23 — Complete Testing Suite (Unit, Integration, Snapshot, Golden)**
- [x] **Phase 24 — Documentation & Examples Suite (`docs/cli`, `examples/`)**
- [x] **Phase 25 — Release Automation & Package Managers (`scripts/`, `packaging/`, `.github/workflows/release.yml`)**
- [x] **Phase 26 — Safe Uninstall & Product Hardening (`DefaultUninstallService`)**
- [x] **Phase 27 & 28 — Installer Finalization & Reinstall-Safe Workspace Generation**
- [x] **Phase 29 & 30 — Merge Audit Diagnostics & Registry Validation Audit**
- [x] **Phase 31 — CLI Command Completion & Uniform Formatting**
- [x] **Phase 32 — Negative & Security Testing Expansion (`phase32_hardening_test.rs`)**
- [x] **Phase 33 & 34 — Documentation & CODEOWNERS Governance Matrix**
- [x] **Phase 35 — V1 Release Candidate & Cross-Target Verification**

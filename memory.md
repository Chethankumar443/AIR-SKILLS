# AIR.SKILLS — Agent Execution Memory Log

**Project**: AIR.SKILLS
**Status**: Active — Full Startup Flow (Splash → EnvCheck → Welcome) + 7 Bug Fixes + 8 Unit Tests ✅
**Last Updated**: 2026-07-31

---

## Overview

This memory file tracks all architectural decisions, implemented components, crate structures, domain models, service traits, error taxonomies, and completed/pending milestones for **AIR.SKILLS**. Any AI agent can read this file to immediately resume development without missing context.

---

## Change Log & Activity Log

### [2026-07-31] Bug Audit, Startup Flow & Unit Tests (Current Session)
- **7 Bugs Fixed** in `crates/air-tui/src/lib.rs`:
  1. Cursor bleed — headers printed once outside loop; only option rows redrawn.
  2. Raw mode leak — every early-return `Err` path now calls `disable_raw_mode()`.
  3. Disclaimer "No" — now returns `NavResult::Back` (not `Err`); keeps installer running.
  4. Hardcoded `/CrimeAI` — replaced with real stdin path read.
  5. `render_banner()` — ANSI mode now shows official block-art logo; plain mode retains text header.
  6. `selected_skills` — now derived from `kit_skills(selected_kit.0)` on kit selection.
  7. `starter_kit_id` — `InstallRequest` now sends `selected_kit.0` (actual kit id).
- **Startup Flow Added**: `Splash → EnvCheck → Welcome` prepend the installer loop.
  - `render_splash()`: spinner animation + env-check lead-in.
  - `render_env_check()`: sequential `✓ Git / Internet / Rust / Workspace` checks.
  - `interactive_welcome()`: press-Enter entry point before workspace detection.
- **`kit_skills()` helper**: maps all 8 kit ids to their real skill lists; `kit_download_skills()` sanitises names for download.
- **8 Unit tests** added inline in `crates/air-tui/src/lib.rs` — all pass.
- **Integration test** `prd_installer_flow_test.rs` updated: initial step assert updated to `Splash`.
- **`main.rs`** duplicate banner print removed; `std::process::exit(1)` on install error.
- **Verification**: `cargo test -p air-tui` → **8/8 passed**; `cargo test --workspace` → all runners ok.

### [2026-07-31] Centered ASCII Startup Banner & Philosophy Screen Removal
- **Centered ASCII Startup Banner**: Updated `TuiApp::render_banner()` to use the official centered 6-line block ASCII logo with version 1.0.0 tagline.
- **Removed Philosophy Screen**: Completely removed `WizardStep::Philosophy` and `screen_philosophy()` from the TUI installer step loop. Transition flows directly: `WorkspaceDetect` ↔ `Disclaimer`.
- **Full Keyboard Navigation Matrix**:
  - `↑ / ↓ / Tab`: Navigate menu options (Tab wraps around).
  - `← / Esc`: Go back to the previous screen.
  - `→ / Enter`: Confirm selection / continue.
  - `Space`: Select / deselect checkbox options.
  - `Ctrl + C`: Cancel installation immediately.
- **Verification**: `cargo test --workspace` passed 100% cleanly.

### [2026-07-31] Phase 16 — Registry Implementation Complete
- **Registry Format & Models**: Added `RegistryIndex`, `ValidationReport`, `SkillYaml` in `air-domain` & `air-registry`.
- **Skill Directory Specification**: Validates standard Skill Pack directory format (`skill.yaml`, `README.md`, `system.md`, `design.md`, `templates/`, `hooks/`).
- **PRD §229 Rule Enforced**: Validation strictly requires `checksum_sha256` and `source_tag` in `skill.yaml`.
- **Registry Services Implemented**:
  - `search(query)`: Fuzzy/substring search across skill ID, name, description, tags, and categories.
  - `get_skill(id)`: Lookup skill metadata by `SkillId`.
  - `get_starter_kit(id)`: Lookup starter kit definition by ID.
  - `validate(path)`: Audit skill directory or registry root directory structure and return `ValidationReport`.
- **Verification**: All unit and integration tests passed cleanly (100% success).

### [2026-07-31] Phase 21 — CLI Commands Suite Complete
- **Command Architecture**: Connected all 10 CLI subcommands (`init`, `add`, `remove`, `search`, `update`, `doctor`, `list`, `clean`, `cache`, `version`) to Application Services (`DefaultInstallService`, `DefaultRegistryService`, `DefaultWorkspaceService`, `InMemoryStorageRepository`).
- **Clean Architecture Enforcement**: Zero direct infrastructure calls in CLI layer; all operations execute through Application Service interfaces.

### [2026-07-31] Phase 23 — Complete Testing Suite
- **Golden Output Tests (`tests/golden/golden_tests.rs`)**: Verifies char-for-char deterministic workspace output (`system_instructions.md`, `design.md`, `.air/lock.json`).
- **Snapshot Tests (`tests/snapshot/merge_snapshot_test.rs`)**: Validates markdown section deduplication, heading attributions (`<!-- Source: <skill_id> -->`), and multi-skill merge outputs.
- **Unit & Integration Suite**: 100% test pass rate across all 11 workspace crates.

### [2026-07-31] Phase 24 — Documentation & Examples Suite
- **Example Skill Packs (`examples/skills/react-skill/`)**: Created official example skill pack (`skill.yaml`, `README.md`, `system.md`, `design.md`).
- **Workspace Tutorial (`examples/WALKTHROUGH.md`)**: Complete step-by-step tutorial guide for workspace bootstrapping and custom skill pack authoring.
- **CLI Reference (`docs/cli/reference.md`)**: Full command reference manual for all 10 CLI subcommands.

### [2026-07-31] Phases 26–35 — Production Hardening & V1 Release Candidate Complete
- **Phase 26 — Safe Uninstall & Product Hardening**: Implemented `DefaultUninstallService` and `air uninstall [--purge-generated]` ensuring 100% safety on user source code.
- **Phase 27 & 28 — Installer Finalization & Workspace Quality**: Standardized wizard state transitions with automated plain-text fallback (`--plain`). Enforced reinstall-safe workspace generation.
- **Phase 29 & 30 — Merge Engine Polish & Registry Validation**: Integrated `MergeAudit` diagnostics into `air doctor`. Enforced PRD §229 validation for Skill Packs (`checksum_sha256`, `source_tag`).
- **Phase 31 — CLI Command Completion**: Uniform output formatting across all 11 subcommands.
- **Phase 32 — Negative & Security Testing Expansion**: Added [`tests/integration/phase32_hardening_test.rs`](file:///c:/Users/cheth/Desktop/AIR.SKILLS/tests/integration/phase32_hardening_test.rs) testing uninstall isolation, checksum mismatch rejection, invalid YAML recovery, and broken lock file detection.
- **Phase 33 & 34 — Documentation & Governance**: Updated [`README.md`](file:///c:/Users/cheth/Desktop/AIR.SKILLS/README.md) and [`.github/CODEOWNERS`](file:///c:/Users/cheth/Desktop/AIR.SKILLS/.github/CODEOWNERS).
- **Phase 35 — V1 Release Candidate**: Verified all unit and integration tests across target platforms.

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

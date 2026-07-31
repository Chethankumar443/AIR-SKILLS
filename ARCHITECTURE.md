# AIR.SKILLS System Architecture

---

## Clean Architecture Layers

AIR.SKILLS is structured strictly around **Clean Architecture** principles to ensure high testability, maintainability, and clear separation of concerns.

```text
Presentation Layer
┌─────────────────────────────────────────────────────────┐
│ air-cli (Clap CLI Router)                               │
│ air-tui (Ratatui TUI Views & Plain-Text Fallback)        │
└───────────────────────────┬─────────────────────────────┘
                            │
Application Layer           ▼
┌─────────────────────────────────────────────────────────┐
│ air-core (InstallService, RepositoryService, etc.)      │
└───────────────────────────┬─────────────────────────────┘
                            │
Domain Layer                ▼
┌─────────────────────────────────────────────────────────┐
│ air-domain (Skill, StarterKit, InstallPlan, etc.)       │
└───────────────────────────┬─────────────────────────────┘
                            │
Infrastructure Layer        ▼
┌─────────────────────────────────────────────────────────┐
│ air-github (GitHub API & Verifiable Downloader)         │
│ air-storage (SQLite Database & Local Cache)             │
│ air-workspace (Filesystem Generator)                    │
│ air-merge (Markdown Merge Engine)                       │
│ air-config (Configuration & Schema Migration)           │
│ air-registry (Registry Index & Semver Solver)           │
│ air-utils (Crypto, Logging, Zip-Slip Guard, Errors)     │
└─────────────────────────────────────────────────────────┘
```

---

## Dependency Rule

- The **Domain layer** (`air-domain`) has **ZERO dependencies** on I/O, network, or external storage libraries.
- The **Application layer** (`air-core`) defines public service traits and orchestrates workflows using domain models.
- **Infrastructure crates** (`air-github`, `air-storage`, `air-merge`, `air-workspace`) implement details behind service traits.
- **Presentation crates** (`air-cli`, `air-tui`) route user input to application services.

---

## Primary Flow Sequences

### Initialization (`air init`)
1. **Terminal Detection**: `air-cli` detects ANSI capability. If missing, engages plain-text prompt mode.
2. **Interactive Wizard**: `air-tui` prompts user to select a Starter Kit or Custom Setup.
3. **Plan Resolution**: `air-core` queries `air-registry` to check semver compatibility and builds an `InstallPlan`.
4. **Dry-Run Check**: `air-merge` performs dry-run merge check and populates conflict details.
5. **Download & Verify**: `air-github` fetches skill archives by pinned tags and verifies SHA-256 checksums.
6. **Zip-Slip Safe Extraction**: `air-utils` extracts archives while ensuring paths stay in target root.
7. **Deterministic Merge**: `air-merge` combines markdown sections, appending attributions for conflicts and writing `.air/merge-audit.json`.
8. **Workspace Injection**: `air-workspace` creates `.air/` folder structure, `config.json`, and `lock.json`.

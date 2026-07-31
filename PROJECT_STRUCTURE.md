# AIR.SKILLS Project Structure

```text
AIR.SKILLS/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   ├── workflows/
│   │   ├── audit.yml
│   │   ├── ci.yml
│   │   └── release.yml
│   ├── CODEOWNERS
│   └── pull_request_template.md
│
├── assets/
│   ├── ascii/
│   ├── icons/
│   └── logos/
│
├── crates/
│   ├── air-cli/         # Entry point, CLI parsing, command routing, TTY capability detection
│   ├── air-tui/         # Ratatui TUI installer views & plain-text fallback
│   ├── air-core/        # Application service traits & core business engine
│   ├── air-domain/      # Pure domain models, zero-I/O structs & traits
│   ├── air-config/      # Configuration management & schema versioning
│   ├── air-storage/     # SQLite storage & local cache interfaces
│   ├── air-github/      # GitHub downloader with SHA-256 & pin verification
│   ├── air-registry/    # Skill registry index lookup & semver compatibility checking
│   ├── air-merge/       # Deterministic markdown merge engine & attribution auditor
│   ├── air-workspace/   # Target workspace generator & injector
│   └── air-utils/       # Hashing, zip-slip protection, logging & unified error system
│
├── docs/
│   ├── adr/             # Architecture Decision Records
│   ├── api/             # API & contract specifications
│   ├── architecture/    # System architecture guides
│   ├── design/          # UI/UX design documents
│   ├── development/     # Developer onboarding & setup guides
│   └── releases/        # Release notes & migration procedures
│
├── examples/            # Example configurations & sample skill packs
├── scripts/             # Development, build, & packaging scripts
├── tests/
│   ├── integration/     # End-to-end multi-crate integration tests
│   ├── snapshot/        # Markdown merge snapshot golden tests
│   └── unit/            # Shared unit test helpers
│
├── website/             # Documentation portal source
│
├── Cargo.toml           # Root workspace Cargo manifest
├── rust-toolchain.toml  # Pinned Rust toolchain configuration
├── rustfmt.toml         # Formatting standards
├── clippy.toml          # Linting rules
├── deny.toml            # Dependency audit & license compliance config
├── .taplo.toml          # TOML formatting rules
├── .gitignore           # Git ignore exclusions
├── memory.md            # Execution tracking log for AI agents
│
├── README.md
├── LICENSE
├── CHANGELOG.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── GOVERNANCE.md
├── ROADMAP.md
├── PROJECT_STRUCTURE.md
├── ARCHITECTURE.md
├── PRD.md
├── TRD.md
└── DECISIONS.md
```

# AIR.SKILLS

> **Build Once. Install Anywhere.**  
> The official AI Project Bootstrap & Skill Manager for AI-driven software engineering.

[![CI](https://github.com/Chethankumar443/AIR-SKILLS/actions/workflows/ci.yml/badge.svg)](https://github.com/Chethankumar443/AIR-SKILLS/actions/workflows/ci.yml)
[![Release](https://github.com/Chethankumar443/AIR-SKILLS/actions/workflows/release.yml/badge.svg)](https://github.com/Chethankumar443/AIR-SKILLS/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](rust-toolchain.toml)

---

## What is AIR.SKILLS?

**AIR.SKILLS** enables developers to bootstrap AI-assisted software projects in minutes by installing curated **Skill Packs** from the official AIR.SKILLS registry repository.

Unlike code generators that only scaffold source files, AIR.SKILLS installs **development intelligence**:

* **AI System Instructions** (`system_instructions.md`)
* **Architecture Specifications & Design Guidelines** (`design.md`)
* **Project Metadata & Deterministic Manifests** (`.air/lock.json`, `.air/config.json`)
* **Merge Audit Logs** (`.air/merge-audit.json`)

It provides a standardized, reproducible project workspace (`.air/`) that any AI coding assistant (Cursor, Copilot, Antigravity, Claude, ChatGPT) can immediately consume.

---

## Features

* 🚀 **Zero Configuration**: Bootstrap production-ready workspace intelligence in seconds.
* 📦 **Curated Starter Kits**: Pre-packaged, validated stacks (React SaaS, AI Chatbot, Desktop Rust+Tauri, Python API, MCP Server, etc.).
* 🔀 **Deterministic Merge Engine**: Safely combines instructions from multiple skill packs without silent drops, producing audit trails (`.air/merge-audit.json`).
* 🔒 **Supply-Chain Verification**: SHA-256 checksum validation and pinned release tags (`v1.0.0`) for all downloaded skills.
* ⌨️ **Keyboard-First TUI & CLI**: Pure Crossterm keyboard navigation with automated plain-text fallback for legacy environments (`--plain`).
* 🛡️ **Safe by Default**: Never mutates user source files; `air uninstall` guarantees 100% safety on user project code.

---

## Installation

### Unix / macOS / Linux (Bash)
```bash
curl -fsSL https://raw.githubusercontent.com/Chethankumar443/AIR-SKILLS/dev/scripts/install.sh | bash
```

### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/Chethankumar443/AIR-SKILLS/dev/scripts/install.ps1 | iex
```

### Via Homebrew (macOS/Linux)
```bash
brew install Chethankumar443/tap/air
```

### Via Scoop (Windows)
```powershell
scoop install https://raw.githubusercontent.com/Chethankumar443/AIR-SKILLS/dev/packaging/scoop/air.json
```

### Via Cargo
```bash
cargo install --path crates/air-cli
```

---

## Command Reference Summary

| Command | Usage | Description |
| :--- | :--- | :--- |
| `air init` | `air init [-d <path>] [--plain]` | Launch TUI/Plain installer wizard to initialize workspace |
| `air add` | `air add <skill...>` | Add one or more Skill Packs to current workspace |
| `air remove` | `air remove <skill...>` (alias `air rm`) | Remove Skill Pack(s) from current workspace |
| `air search` | `air search [query]` | Search official Skill Packs registry catalog |
| `air list` | `air list` (alias `air ls`) | List installed skill packs and versions in workspace |
| `air doctor` | `air doctor` | Run workspace health diagnostics and merge audit checks |
| `air update` | `air update [--self]` | Update workspace skill packs or AIR CLI binary |
| `air clean` | `air clean` | Clean temporary workspace audit logs and cache |
| `air cache` | `air cache [list\|clear]` | Manage local skill pack storage cache |
| `air uninstall` | `air uninstall [--purge-generated]` | Safely remove AIR metadata while preserving user code |
| `air version` | `air version` | Display AIR CLI version, target OS, and workspace status |

---

## Architecture Overview

AIR.SKILLS is structured as a Rust workspace with Clean Architecture principles:

```text
crates/
├── air-cli        # CLI entry point, Clap commands, TTY detection & Application Services wiring
├── air-tui        # Crossterm TUI installer wizard views & plain-text fallback
├── air-core       # Application service traits & core orchestrators (Install, Registry, Workspace, Uninstall)
├── air-domain     # Pure data models & zero-I/O entities (Skill, StarterKit, Manifest, ProgressEvent)
├── air-config     # Configuration parser & schema migrations
├── air-storage    # SQLite persistence & local cache management
├── air-github     # GitHub API client with checksum & pinned release tag verification
├── air-registry   # Skill registry lookup & semver compatibility engine
├── air-merge      # Deterministic markdown merge & heading attribution engine
├── air-workspace  # Target project workspace generator (.air/lock.json, system_instructions.md, design.md)
└── air-utils      # SHA-256 crypto, filesystem security, logging & typed error codes
```

---

## License

Distributed under the [MIT License](LICENSE).

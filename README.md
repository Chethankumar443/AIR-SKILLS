# AIR.SKILLS

> **Build Once. Install Anywhere.**
> The official AI Project Bootstrap & Skill Manager.

[![CI](https://github.com/AIR-SKILLS/air-skills/actions/workflows/ci.yml/badge.svg)](https://github.com/AIR-SKILLS/air-skills/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](rust-toolchain.toml)

---

## What is AIR.SKILLS?

**AIR.SKILLS** enables developers to bootstrap AI-assisted software projects in minutes by installing curated **Skill Packs** from the official AIR.SKILLS repository.

Unlike code generators that only scaffold source files, AIR.SKILLS installs **development intelligence**:

* **Architecture Specifications** (`architecture.md`)
* **AI System Instructions** (`system_instructions.md`)
* **Design Guidelines** (`design.md`)
* **Coding Standards & Rules** (`coding_rules.md`)
* **Project Templates & Metadata** (`templates/`, `skill.yaml`)

It provides a standardized, reproducible project workspace (`.air/`) that any AI coding assistant (Cursor, Copilot, Antigravity, Claude, ChatGPT) can immediately consume.

---

## Features

* 🚀 **Zero Configuration**: Get up and running in seconds.
* 📦 **Curated Starter Kits**: Pre-packaged, validated stacks (React, Astro, Tauri, Python API, MCP Server, CLI, etc.).
* 🔀 **Deterministic Merge Engine**: Safely combines instructions from multiple skill packs without silent drops, producing audit trails (`.air/merge-audit.json`).
* 🔒 **Supply-Chain Verification**: SHA-256 checksum validation and pinned release tags for all downloaded skills.
* ⌨️ **Keyboard-First TUI & CLI**: Powered by `ratatui` with automated plain-text fallback for legacy environments.
* 🛡️ **Safe by Default**: Never mutates source files outside `.air/` without permission; no unverified script execution.

---

## Installation

### Via Cargo

```bash
cargo install air-cli
```

### Pre-compiled Binaries

Download the latest release binary for Windows, macOS, or Linux from [Releases](https://github.com/AIR-SKILLS/air-skills/releases).

---

## Quick Start

Initialize a new workspace in your project directory:

```bash
cd my-new-project
air init
```

Follow the interactive TUI wizard to select a **Starter Kit** or build a **Custom Setup**.

### Useful Commands

```bash
# Add a skill pack to an existing workspace
air add python-backend

# Check workspace health and configuration integrity
air doctor

# List installed skills
air list

# Update installed skills to latest compatible versions
air update

# Update the CLI binary itself
air update --self
```

---

## Architecture Overview

AIR.SKILLS is structured as a Rust workspace with Clean Architecture principles:

```text
crates/
├── air-cli        # CLI entry point, Clap commands, TTY detection
├── air-tui        # Ratatui TUI installer views & plain-text fallback
├── air-core       # Application service traits & core orchestrator
├── air-domain     # Pure data models & zero-I/O entities
├── air-config     # Configuration parser & schema migrations
├── air-storage    # SQLite persistence & local cache management
├── air-github     # GitHub API client with checksum & pin verification
├── air-registry   # Skill registry lookup & semver compatibility engine
├── air-merge      # Deterministic markdown merge & attribution engine
├── air-workspace  # Target project workspace generator
└── air-utils      # Hashing, zip-slip protection, logging & unified error handling
```

See [ARCHITECTURE.md](ARCHITECTURE.md) and [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) for details.

---

## Roadmap

Check out our [ROADMAP.md](ROADMAP.md) to see upcoming features, milestones, and release plans.

---

## Contributing

We welcome contributions! Please review [CONTRIBUTING.md](CONTRIBUTING.md) before submitting Pull Requests.

---

## Security

Security is paramount. Please review our [SECURITY.md](SECURITY.md) to report vulnerabilities privately.

---

## License

This project is licensed under the [MIT License](LICENSE).

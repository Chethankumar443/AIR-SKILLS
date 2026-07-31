# AIR.SKILLS Walkthrough & Tutorial Guide

This guide walks you through bootstrapping an AI-driven workspace using **AIR.SKILLS**.

---

## Quickstart Step-by-Step

### 1. Initialize Workspace Interactive Wizard
Run the interactive TUI installer in your project directory:
```bash
air init
```
Navigate using `↑`/`↓` or `Tab`, press `Enter` to select a **Starter Kit** (e.g., *Modern React SaaS* or *Python API*), or choose **Custom Setup**.

### 2. Verify Generated Workspace Intelligence
After installation completes, check your project root directory:
```bash
ls -la .air/
```
You will find:
- `.air/lock.json` — Lock manifest tracking installed skills, versions, and SHA-256 checksums.
- `.air/config.json` — Local workspace configuration settings.
- `.air/merge-audit.json` — Detailed merge operation audit log.
- `system_instructions.md` — Merged AI system instructions compiled from all skills.
- `design.md` — Merged architecture and coding rules for your AI assistant.

### 3. Search & Add Additional Skill Packs
Search the official registry catalog for specific skills:
```bash
air search python
```
Add new skills to your existing workspace:
```bash
air add fastapi pydantic
```

### 4. Workspace Diagnostics
Run a health diagnostic check at any time:
```bash
air doctor
```

---

## Authoring Custom Skill Packs

Every Skill Pack consists of:
```text
my-skill/
├── skill.yaml         # Required metadata (id, version, checksum_sha256, source_tag)
├── README.md          # User documentation
├── system.md          # AI System instructions
├── design.md          # Architecture & coding standards
├── templates/         # Scaffolding templates
└── hooks/             # Lifecycle hooks
```
Validate your custom skill directory:
```bash
air doctor
```

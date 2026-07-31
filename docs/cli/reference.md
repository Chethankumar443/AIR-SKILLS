# AIR.SKILLS CLI Command Reference

Comprehensive command-line manual for `air` CLI v1.0.0.

---

## Commands Summary

| Command | Usage | Description |
| :--- | :--- | :--- |
| `air init` | `air init [-d <path>] [--plain]` | Launch TUI/Plain installer wizard to initialize an AIR workspace |
| `air add` | `air add <skill_id...>` | Add one or more Skill Packs to current workspace |
| `air remove` | `air remove <skill_id...>` (or `air rm`) | Remove Skill Pack(s) from current workspace |
| `air search` | `air search [query]` | Search official Skill Packs registry catalog |
| `air list` | `air list` (or `air ls`) | List installed skill packs and versions in current workspace |
| `air doctor` | `air doctor` | Run workspace health diagnostics and schema validation |
| `air update` | `air update [--self]` | Update workspace skill packs or AIR CLI binary |
| `air clean` | `air clean` | Clean temporary workspace audit logs and cache |
| `air cache` | `air cache [list\|clear]` | Manage local skill pack storage cache |
| `air version` | `air version` | Display AIR CLI version, target OS, and workspace status |

---

## Detailed Command Descriptions

### `air init`
Launches the keyboard-driven TUI installer to select starter kits or custom skills and bootstrap `.air/`, `system_instructions.md`, and `design.md`.

**Flags:**
- `-d, --path <PATH>`: Specify target directory (defaults to current directory `.`).
- `--plain`: Disable ANSI TUI animations and use plain-text fallback mode.

### `air add <skills...>`
Installs specified skill packs into the current workspace, resolves semver compatibility, and updates merged intelligence files.

```bash
air add react tailwind typescript
```

### `air remove <skills...>`
Removes specified skill packs from `.air/lock.json` and regenerates merged markdown intelligence documents.

```bash
air remove tailwind
```

### `air search [query]`
Queries the official registry for available starter kits and skill packs.

```bash
air search ai
```

### `air doctor`
Validates workspace health, ensuring `.air/lock.json`, `.air/config.json`, and required merged files exist and conform to schema version 1.

```bash
air doctor
```

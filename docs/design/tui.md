# Terminal UI & Fallback Specification

- TUI Engine: `ratatui` + `crossterm`.
- Capability Detection: Capability detection runs prior to rendering. Fallback to plain text on non-ANSI terminals (e.g. legacy `cmd.exe`).

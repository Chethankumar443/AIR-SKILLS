# AIR.SKILLS Product Roadmap

---

## Phase 0: Project Planning & Foundation (Completed)
- Document initial architecture, PRD, TRD, ADRs, and project structure.
- Setup 11-crate Cargo workspace structure and toolchain configuration.
- Define domain models, service traits, configuration schemas, and error taxonomy.
- Create CLI & TUI skeletons and cross-platform CI pipelines.

---

## Milestone 1: Core Skill Engine & Installer (Current Phase)
- Implement `air-github` archive downloading with SHA-256 checksum verification.
- Implement `air-registry` semver compatibility checking.
- Connect `air-tui` wizard to `air-core` `InstallService`.
- Implement `air-workspace` file system injector.

---

## Milestone 2: Deterministic Merge Engine
- Implement heading-matched markdown merger with append-with-attribution strategy.
- Generate `.air/merge-audit.json` audit logs.
- Add `--strict` mode flag for CI environments.
- Create snapshot/golden test suite verifying byte-identical output.

---

## Milestone 3: Health & Maintenance Tools
- Complete `air doctor` diagnosis engine (detecting drift, lockfile state, cache state).
- Implement `air add`, `air remove`, `air update`, `air search`, `air list`.
- Build self-update mechanism (`air update --self`).

---

## Milestone 4: Release v1.0.0
- Cross-platform release builds (Windows, macOS, Linux) via `cargo-dist`.
- Official website & documentation portal.
- Launch initial official Skill Packs repository (`github.com/AIR-SKILLS`).

---

## Milestone 5: Community Contribution & Ecosystem (v2.0+)
- Community contribution workflow via review & merge into `AIR-SKILLS` organization.
- Signed Skill Packs (cryptographic signatures).
- Advanced offline cache synchronization.

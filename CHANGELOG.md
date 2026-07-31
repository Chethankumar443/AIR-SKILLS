# Changelog

All notable changes to **AIR.SKILLS** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-07-31

### Added
- Initialized production-grade Rust workspace with 11 core crates:
  - `air-cli`, `air-tui`, `air-core`, `air-domain`, `air-config`, `air-storage`, `air-github`, `air-registry`, `air-merge`, `air-workspace`, `air-utils`.
- Defined frozen domain contracts (`Skill`, `StarterKit`, `Workspace`, `InstallPlan`, `Manifest`, `Dependency`, `Conflict`, `ValidationResult`, `WorkspaceResult`, `ProgressEvent`).
- Implemented core service traits (`InstallService`, `RepositoryService`, `MergeService`, `WorkspaceService`).
- Created unified typed error system (`AirError`) with stable error codes (`AIR-XXX-000`).
- Configured CLI binary skeleton supporting `air init`, `air version`, `air help`.
- Implemented TUI installer view skeletons with plain-text fallback capabilities.
- Added full project documentation suite (`PRD.md`, `TRD.md`, `ARCHITECTURE.md`, `PROJECT_STRUCTURE.md`, `DECISIONS.md`, `GOVERNANCE.md`, `ROADMAP.md`, `SECURITY.md`, `CONTRIBUTING.md`).
- Setup GitHub Actions CI/CD workflows (`ci.yml`, `release.yml`, `audit.yml`).

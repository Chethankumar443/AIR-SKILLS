# AIR.SKILLS Governance Model

---

## 1. Project Roles

### Maintainers
Maintainers are individuals who have write access to the repository and are responsible for reviewing pull requests, managing releases, and steering architectural direction.

### Contributors
Contributors are community members who submit pull requests, report issues, improve documentation, or participate in discussions.

---

## 2. Code Ownership

Component ownership is managed via `.github/CODEOWNERS`:
- Core Architecture (`crates/air-core`, `crates/air-domain`): Lead Maintainers
- UI/UX (`crates/air-tui`, `crates/air-cli`): DX Maintainers
- Security & Storage (`crates/air-github`, `crates/air-storage`, `crates/air-utils`): Security Lead

---

## 3. Decision-Making Process

- **Minor Changes**: Resolved via standard PR reviews (requires 1 maintainer approval).
- **Major Architectural Changes**: Requires an Architecture Decision Record (ADR) added to `docs/adr/` or `DECISIONS.md`, followed by maintainer consensus.

---

## 4. Official Skill Registry Boundary

AIR.SKILLS maintains a permanent closed-boundary policy for skill installations: only skills approved and hosted under `github.com/AIR-SKILLS` can be installed by default. This guarantees supply-chain trust and single-boundary security. Community skills are accepted via pull requests to the official org after rigorous maintainer review.

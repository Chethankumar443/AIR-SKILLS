# Architecture Decision Records (ADRs)

---

## ADR-001: Rust Selected as Primary Implementation Language

### Status
Accepted

### Context
AIR.SKILLS requires cross-platform execution (Windows, macOS, Linux), high performance, zero runtime dependencies, low memory footprint, and robust safety guarantees.

### Decision
Implement AIR.SKILLS entirely in Rust.

### Consequences
- Single static binary distribution via `cargo-dist`.
- Memory safety without garbage collection overhead.
- Strong typing for error handling and domain models.

---

## ADR-002: Async Concurrency with Tokio

### Status
Accepted

### Context
Downloading multiple skill archives, updating registry caches, querying SQLite, and keeping the TUI responsive with progress bars requires concurrent execution.

### Decision
Adopt `tokio` as the asynchronous runtime across all I/O-bound crates.

### Consequences
- Non-blocking network and disk operations.
- TUI event loop multiplexes user keyboard input and progress events smoothly.
- Domain crate (`air-domain`) remains pure and synchronous.

---

## ADR-003: Closed-Boundary Official GitHub Organization Ecosystem

### Status
Accepted

### Context
Allowing arbitrary third-party URL skill downloads introduces supply-chain security risks (malicious script injection, unverified instructions).

### Decision
Restrict default skill downloads strictly to official repositories under `github.com/AIR-SKILLS`. Community skills must be submitted via pull request to the official organization.

### Consequences
- Cryptographically verifiable single trust boundary.
- Predictable skill structure and maintainer review.

---

## ADR-004: Append-with-Attribution Merge Engine Conflict Policy

### Status
Accepted

### Context
When multiple skill packs define guidance under the same markdown heading (e.g. `## Testing Standards`), silently dropping one skill's guidance is unacceptable.

### Decision
Retain content from all contributing skills under attributed sub-headings (e.g. `### Testing Standards (from: rust)`), generate `.air/merge-audit.json`, and provide `--strict` failure mode for CI.

### Consequences
- Zero silent data loss during skill composition.
- Transparent audit trails for developers and AI agents.

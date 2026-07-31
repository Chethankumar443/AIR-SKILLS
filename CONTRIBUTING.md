# Contributing to AIR.SKILLS

Thank you for your interest in contributing to **AIR.SKILLS**! This document outlines our development process, standards, and guidelines.

---

## Branch Strategy

- `main`: Production-ready branch. All releases are cut from `main`.
- `feature/*`: Feature development branches.
- `fix/*`: Bug fix branches.
- `docs/*`: Documentation updates.

---

## Pull Request Rules

1. All PRs must target `main`.
2. Every PR must pass all CI checks (`cargo fmt`, `cargo clippy`, `cargo test`, `cargo-audit`).
3. PRs require approval from at least one core maintainer before merging.
4. Keep PRs focused and modular. Small PRs are reviewed faster.

---

## Commit Message Format

We follow Conventional Commits:

```text
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Formatting, missing semi-colons, etc. (no code logic change)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Build tasks, package manager configs, etc.

---

## Coding Style & Standards

- **Formatting**: Run `cargo fmt --all` prior to committing.
- **Linting**: Ensure `cargo clippy --workspace --all-targets` passes with no warnings.
- **Errors**: Use typed domain errors (`thiserror`) and stable error codes (`AIR-XXX-000`).
- **Dependencies**: Keep dependencies minimal and audited.

---

## Code Review Process

1. Automated CI tests execute automatically on PR creation/update.
2. Maintainers review code quality, architecture adherence, and security.
3. Once approved and CI passes, maintainers merge via rebase or squash.

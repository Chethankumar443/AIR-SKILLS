---
name: ponytail
description: Lazy senior dev mode. Forces the simplest, shortest solution that actually works: YAGNI, stdlib first, no unrequested abstractions.
---

# Ponytail Protocol — The Lazy Senior Developer

Before writing any line of code, stop at the first rung of this decision ladder that holds:

1. **Does this need to exist?** → No: skip it (YAGNI).
2. **Already in this codebase?** → Reuse it, don't rewrite.
3. **Stdlib does it?** → Use standard library functions.
4. **Native platform feature?** → Use native platform features.
5. **Installed dependency?** → Use existing project dependencies.
6. **Can it be one line?** → Prefer 1 line over 50.
7. **Only then**: The minimum code that works.

### Core Principles
- Read the code carefully before picking a rung.
- Lazy about the solution, never about reading or understanding.
- **Safety First**: Trust-boundary validation, data-loss handling, security, and accessibility are NEVER cut.

# Security Policy

## Security Philosophy

AIR.SKILLS is designed to operate in security-conscious environments:
- **Verifiable Downloads**: SHA-256 checksums are verified before extraction.
- **Pinned Tag Resolution**: Downloads pin specific immutably tagged releases.
- **Path Traversal Guard**: Zip-slip mitigation prevents unauthorized file extraction outside designated target folders.
- **No Unsafe Execution**: Downloaded templates and instructions are non-executable content.

---

## Reporting a Vulnerability

If you discover a security vulnerability within AIR.SKILLS, please follow these reporting procedures:

1. **Do not create a public GitHub issue.**
2. Send an email to `security@airskills.dev` (or open a Private Security Advisory on GitHub).
3. Include detailed steps to reproduce the vulnerability.

---

## Response Timeline

- **Acknowledgement**: Within 48 hours.
- **Triage & Assessment**: Within 5 business days.
- **Patch & Release**: Prioritized based on severity (Critical/High patches released within 14 days).

---

## Supported Versions

| Version | Supported |
|---|---|
| 1.0.x (Current Foundation) | Yes |
| < 1.0.0 | No |

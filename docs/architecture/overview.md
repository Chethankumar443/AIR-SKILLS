# Architectural Overview

AIR.SKILLS separates concerns into Clean Architecture layers:
- `air-domain`: Pure entities and contracts.
- `air-core`: Application service traits.
- `air-storage`, `air-github`, `air-merge`, `air-workspace`: Infrastructure implementations.
- `air-tui`, `air-cli`: User interface & command router.

# React Architecture & Design Guidelines

## State Management
- Keep state local to components whenever possible (`useState`, `useReducer`).
- Use React Context only for global application settings (theme, user auth).

## Folder Structure
- `src/components/ui/` — Base design system primitives (Button, Modal, Input).
- `src/components/features/` — Domain-specific feature modules.
- `src/lib/` — Shared utilities and API abstraction layers.

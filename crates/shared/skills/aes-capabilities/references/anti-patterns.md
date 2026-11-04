# Anti-patterns

Common structural and import mistakes that a reader must catch — the linter covers naming, imports, and primitives.

- **Stateless helper with no protocol behaviour in a capability file**: extract to `utility_*` — it belongs in `shared/`, not the feature folder.
- **Business logic mixed with a pure helper**: split — business rules stay in the capability; stateless logic moves to `utility_*`.
- **Capabilities importing each other**: shared needs go through the contract or root, not sibling imports.
- **Local domain model defined in a capability file**: move to `taxonomy_*_vo` in `shared/`.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

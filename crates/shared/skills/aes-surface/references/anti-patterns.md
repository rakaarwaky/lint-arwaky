# Anti-patterns

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **`_entry` used as a surface suffix**: entry points are root (`root_*_entry`).
- **Surface importing capabilities or protocol directly**: Smart → aggregate only; Utility/Passive → taxonomy only.
- **Silently discarded errors**: forbidden in every tier.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

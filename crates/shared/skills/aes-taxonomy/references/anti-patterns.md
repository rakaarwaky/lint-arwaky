# Anti-patterns

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Primitives in domain fields**: wrap in a VO (AES401).
- **I/O or upward imports in taxonomy**: forbidden — move to capabilities/utility.
- **Computed "constants"**: pure literals only; computed values go to Utility.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

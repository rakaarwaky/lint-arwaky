# Anti-patterns

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Business / orchestration / parsing / UI logic in root**: forbidden — route down a layer.
- **Concretions visible above root**: bind to contracts so agents/surfaces see interfaces, not classes.
- **Wrong suffix / undocumented name**: `_container` or `_entry` only (exceptions listed in HOW-TO).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

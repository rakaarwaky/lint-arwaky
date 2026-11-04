# Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules).

- **Class / `self` / `this` / `struct` / `impl` / `trait` / `type` alias in utility** — free functions only (AES404).
- **Domain-specific or single-consumer helper extracted here** — keep private until ≥2 consumers.
- **Imports from capabilities/agent/surface/contract** — forbidden (AES201).
- **Utility placed in a feature folder that only one consumer uses** — private helper, or move the consumer up.
- **Restating HOW-TO rules in SKILL.md** — delegate; the body only routes.

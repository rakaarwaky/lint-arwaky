# Anti-patterns

The linter covers naming, imports, and primitives. These need a reader (language playbook § Troubleshooting):

- **Skipping taxonomy**: jumping to capabilities without VOs → primitive leakage (AES401).
- **Thin-protocol**: one protocol per method per feature — instead, one file per feature, one trait per capability seam, each trait rich.
- **Fat-aggregate**: aggregate with many methods — instead, exactly one `execute()` method returning a taxonomy-defined response enum.
- **Import inversion**: capabilities importing agent, or surface importing capabilities → route through contract (AES201).
- **Utility with state**: class / `self` / `this` in utility → free functions only (AES404).
- **Agent with business logic**: computation in agent → move to capabilities (AES405).
- **Surface calling capabilities directly**: skip agent → surface must call aggregate (AES201 purpose).
- **Root with business logic**: logic in container/entry → move down to capabilities/agent.
- **Restating playbook content in SKILL.md**: delegate — this file only routes.

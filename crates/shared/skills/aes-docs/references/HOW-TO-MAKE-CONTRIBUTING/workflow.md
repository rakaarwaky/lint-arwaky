# CONTRIBUTING Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Identify the contribution paths.** Before writing, list the distinct kinds of change this project accepts — a new feature module, a bug fix, a vendored/upstream dependency, a tooling change, or a docs-only update. One section per path. Delete the ones that do not apply.
2. **Create file** → `CONTRIBUTING.md` at repo root.
3. **Section: Contribution Overview** — how to contribute, what mindset contributors should have, and what "ready to merge" looks like.
4. **Section: Contribution Guidelines** — contribution rules.
5. **Section: Workflow** — `Step | Action | Detail` contribution pipeline.
6. **Section: Commit Convention** — `Type | Usage | Example` conventional commit prefixes.
7. **Section: Reference** — link to PRD.md and ARCHITECTURE.md.
8. **Verify** → `lint-arwaky-cli docs` passes; every command block runs without absolute personal paths; templates use placeholders.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every command runs; templates use placeholders; no absolute personal paths.
```

# README Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `README.md` at repo or package root.
2. **Section: Project Overview** — what this project does, who it serves, core value.
3. **Section: Prerequisites** — pinned build dependencies.
4. **Section: Quick Start** — `Step | Action | Command` table.
5. **Section: Reference** — required H2 with empty placeholder table.
6. **Verify** → `lint-arwaky-cli docs` passes; quick start works.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: readme-section-missing, dead-link, absolute-path, secret-in-docs, doc-length.
```

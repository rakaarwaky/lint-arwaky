# BACKLOG Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `BACKLOG.md` in feature directory.
2. **Section: Progress Overview** — document baseline state.
3. **Section: Backlog Status** — `{<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}` items, each carrying State / Done / In Progress / Blocked / Next / Risk / Updated.
4. **Section: Reference** — link to PRD.md, FRD.md, ROADMAP.md, ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; all sections present.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: pairing, columns, unknown states, unevidenced Done, scenario coverage.
```

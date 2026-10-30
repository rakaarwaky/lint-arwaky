# DATA Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `DATA.md` in the shared/ directory.
2. **Section: Data Overview** — one paragraph: what the kernel manages, who reads/writes it.
3. **Items: `### DT-SHARED-NNN: <short imperative name>`** — one H3 per data concept under `## Data Requirements`; fill the item's Type/Description table.
4. **Section: Reference** — link to PRD.md, BACKLOG.md, ROADMAP.md, ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; section order is correct.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```

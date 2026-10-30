# DESIGN Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `DESIGN.md` in the surface/ folder.
2. **Section: Design Overview** — one paragraph of what the design looks like.
3. **Section: Design Requirements** — one H3 per requirement, `### DS-<DESIGN-NAME-ID>-NNN: <short imperative name>`; fill the item's table.
4. **Section: Reference** — link to PRD.md, BACKLOG.md, ROADMAP.md, ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; section order is correct.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: IDs, orphan refs, section order, status leak, links.
```

# ROADMAP Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `ROADMAP.md` at repo root.
2. **Section: Progress Overview** — one paragraph of progress summary.
3. **Section: Roadmap Status** — `PR-<PROJECT-NAME-ID>-NNN` items, each carrying Priority / State / Dependencies / Updated.
4. **Section: Reference** — link to PRD.md and ARCHITECTURE.md.
5. **Verify** → `lint-arwaky-cli docs` passes; every required section is present.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: master root presence, feature pairing, unknown states, missing sections.
```

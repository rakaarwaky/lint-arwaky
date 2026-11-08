# ARCHITECTURE Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Create file** → `ARCHITECTURE.md` at repo root (note: spelling `ARCHITECHTURE` is intentionally variant here only if your team prefers it; the checker uses the heading text, not the file name).
2. **Section: Purpose** — one paragraph stating what the architecture guards against.
3. **Section: Workspace Organization** — define the workspace terms and member types.
4. **Section: Naming Convention** — the `layer_concern_role.<ext>` rule and its language suffixes.
5. **Section: Vertical Slicing Folder Structure** — feature layout and shared layout.
6. **Section per layer** — Taxonomy, Contract, Utility, Capabilities, Agent, Surface, Root. Each section: Overview, Component, Rules.
7. **Verify** → `lint-arwaky-cli docs` passes; every dependency table has a line for each named layer.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, absolute-path, secret-in-docs, doc-length.
# Manual: every layer states its allowed dependencies; no implementation detail;
#   file names follow layer_concern_role.ext; no per-file exceptions without a subsection.
```

---
name: aes-surface
description: AES surface command and UI scaffolding for Python Rust TS. Use when creating surface files.
when-to-use: "Use when creating or auditing a surface command, controller, hook, store, or component file in Python, Rust, or TypeScript, or when picking a Smart/Utility/Passive tier for a surface role."
argument-hint: "<layer-path>"
---

# aes-surface

Scaffold AES surface files — map input to VOs, call the aggregate, render results; zero business logic in every tier. The **tier** decides the suffix, the imports, and the structure.

## Global rules

- **Placement (AES703)**: `surface_*` files belong in a **surface folder** — a subdirectory of a member directory (`crates/`, `modules/`, `packages/`) whose name matches the surface kind it carries (`api`, `mcp`, `cli`, `desktop`, `tui`). The folder carries surface files only; a `capabilities_*` or `agent_*` file found there is an AES703 violation.
- **DESIGN.md**: every surface folder documents how its surface looks and behaves in `DESIGN.md` at the folder root — six sections: Kind, Entry Points, Request Shape, States, Error States, Invariants, Change Checklist. A surface-dominated folder with no `DESIGN.md` is an AES703 violation.
- **Naming (AES101/AES102)**: file `surface_<domain>_<role>` — suffix in the surface allow-list; `_entry` is root, not surface.
- **Tiers (AES406 / AES201)**: Smart (`_command`/`_controller`/`_page`) imports taxonomy + `_aggregate` + utility; Utility (`_hook`/`_store`/`_action`/`_screen`/`_router`) and Passive (`_component`/`_view`/`_layout`) import taxonomy only.
- **Behaviour**: zero business logic and zero computation in every tier; Smart delegates through the injected aggregate only.
- **Errors**: never silently discarded — no `or None` / `unwrap_or_default()` / `?? <fallback>`; return Ok/Err or update an error-state VO.
- **State**: all state fields use shared VOs — no primitives, no local domain models.
- **Register**: package module export where required.
- **Verify**: `lint-arwaky-cli scan <layer-path>` → 0; language compile is fallback only.
- **Related**: aes-contract (aggregate target), aes-agent (orchestrates capabilities), aes-root (wires surface), aes-taxonomy (shared VOs) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-surface.md](references/python-surface.md) | Python surface — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/rust-surface.md](references/rust-surface.md) | Rust surface — rules, template, section contract, workflow, verify |
| Language HOW-TOs | [references/typescript-surface.md](references/typescript-surface.md) | TypeScript surface — rules, template, section contract, workflow, verify |
| DESIGN.md sections | [references/design-doc.md](references/design-doc.md) | The six sections every surface folder's DESIGN.md carries |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a broken surface file |
| Workflow | [references/workflow.md](references/workflow.md) | Step sequence from tier-selection to register |
| Verification | [references/verification.md](references/verification.md) | Primary gate + fallback language compile |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before drafting a surface file |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common tier, import, and error-handling mistakes |

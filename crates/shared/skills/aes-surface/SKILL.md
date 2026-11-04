---
name: aes-surface
description: "AES surface command and UI scaffolding for Python Rust TS. Use when creating surface files."
when-to-use: "create, add, or audit an AES surface file (Python, Rust, TypeScript); add a command, controller, hook, store, component, or view; pick the Smart/Utility/Passive tier; fix surface structure."
argument-hint: '<python | rust | typescript> [file path]'
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - surface
    - smart
    - utility
    - passive
    - di
    - vo
  related_skills:
    - aes-lint-arwaky
    - aes-agent
    - aes-taxonomy
    - aes-contract
    - aes-root
---

# aes-surface

Scaffold AES surface files — map input to shared VOs, call the aggregate, render results; zero business logic and zero computation in every tier. The audience is the agent creating or validating a surface command/controller/component; the **tier** decides the suffix, the imports, and the structure. The layer chain reads Smart → `contract_*_aggregate`, Utility/Passive → taxonomy only; surface never calls capabilities or protocol directly. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Naming: file `surface_<domain>_<role>` — suffix in the surface allow-list; `_entry` is root, never surface (AES101/AES102).
- Tiers: Smart (`_command`/`_controller`/`_page`) imports taxonomy + `_aggregate` + utility; Utility (`_hook`/`_store`/`_action`/`_screen`/`_router`) and Passive (`_component`/`_view`/`_layout`) import taxonomy only (AES406 / AES201).
- Behaviour: Smart delegates through the injected aggregate only; no business logic, no computation, in any tier.
- Errors: never silently discarded — no `or None` / `unwrap_or_default()` / `?? <fallback>`; return Ok/Err or update an error-state VO.
- State: every state field uses shared VOs — no primitives, no local domain models.
- Placement: surface files live in a surface folder named after the surface kind (`api`, `mcp`, `cli`, `desktop`, `tui`); the folder carries a `DESIGN.md` (AES703).
- Register: package module export where required.
- Verify: `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only.
- One fact, one home: the language HOW-TO carries the language's tier templates and section contract — read it, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-surface.md](references/python-surface.md) | Python surface HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/rust-surface.md](references/rust-surface.md) | Rust surface HOW-TO — rules, template, section contract, workflow steps, review, verify |
| [references/typescript-surface.md](references/typescript-surface.md) | TypeScript surface HOW-TO — rules, template, section contract, workflow steps, review, verify |
| Related skills | aes-contract, aes-agent, aes-root, aes-taxonomy, aes-lint-arwaky |

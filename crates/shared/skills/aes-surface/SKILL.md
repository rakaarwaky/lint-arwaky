---
name: aes-surface
description: AES surface command and UI scaffolding for Python Rust TS. Use when creating surface files.
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
  triggers:
    - create surface
    - create surface python
    - create surface rust
    - create surface typescript
    - add surface
    - add surface python
    - add surface rust
    - fix surface structure
    - create command
    - create command python
    - create controller python
    - create controller rust
    - check surface
    - audit surface
    - audit surface python
    - audit surface rust

when-to-use: Use when creating AES surface command/UI scaffolding — Smart/Utility/Passive tiers mapping input to VOs, calling the aggregate, rendering results; triggers include `create surface`, `create command`, `create controller`, `audit surface`, `check surface`.
argument-hint: "[python | rust | typescript] [--check]"
---

# aes-surface

> **Purpose**: Scaffold AES surface files — map input to VOs, call the aggregate, render results; no business logic.
> **Audience**: The agent creating or validating a surface command/controller/component.
> **Scope**: Python, Rust, and TypeScript `surface_<domain>_<role>` files — Smart / Utility / Passive tiers.

The **layer** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO|
| -------- | ----- | --------- | ------|
| Python | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-PYTHON-SURFACE.md](references/HOW-TO-MAKE-PYTHON-SURFACE.md)|
| Rust | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-RUST-SURFACE.md](references/HOW-TO-MAKE-RUST-SURFACE.md)|
| TypeScript | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-TYPESCRIPT-SURFACE.md](references/HOW-TO-MAKE-TYPESCRIPT-SURFACE.md)|

**The layer chain:**

surface (Smart → `_aggregate`, Utility/Passive → taxonomy) → agent → capabilities — never surface→capabilities directly

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
The full invariant table, placement rules, diagnostic tree, workflow, verification, and mistake
patterns live in [`references/invariants.md`](references/invariants.md). Cite the linter, not
this file, when pointing at a rule.

## Placement (AES703)

A `surface_*` file belongs in a surface folder — see [`references/invariants.md`](references/invariants.md) for the full placement rules, surface-folder rules, and `DESIGN.md` requirements.

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action. Full tree in [`references/invariants.md`](references/invariants.md):

1. **What tier is this — Smart / Utility / Passive?** → Smart injects aggregate; Utility/Passive use taxonomy-only imports.
2. **Is the suffix in the surface allow-list (never `_entry`)?** → Rename or use `root_*_entry`.
3. **Any business logic, computation, or direct capability calls?** → Move logic down; surface only maps and delegates.
4. **Any silently discarded errors or primitive state fields?** → Propagate errors; wrap state in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?** → Fix findings, re-scan.

## Workflow

1. Determine the tier (Smart / Utility / Passive) and pick the matching suffix.
2. Resolve the feature surface dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft from the language HOW-TO § Template / § Section Contract; Smart injects the aggregate via DI.
4. Enforce that tier's import rules; strip business logic and computation.
5. Check error handling; state fields are VOs.
6. Register the module, verify with `lint-arwaky-cli scan`, then wire via `aes-root`.

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)
```

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

## Common Mistakes (Anti-Patterns)

- **`_entry` used as a surface suffix**: entry points are root (`root_*_entry`).
- **Surface importing capabilities or protocol directly**: Smart→aggregate only; Utility/Passive→taxonomy only.
- **Silently discarded errors**: forbidden in every tier.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-agent`
- `aes-contract`
- `aes-root`

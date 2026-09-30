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
---

# aes-surface

> **Purpose**: Scaffold AES surface files — map input to VOs, call the aggregate, render results; no business logic.
> **Audience**: The agent creating or validating a surface command/controller/component.
> **Scope**: Python, Rust, and TypeScript `surface_<domain>_<role>` files — Smart / Utility / Passive tiers.

The **layer** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-PYTHON-SURFACE.md](references/HOW-TO-MAKE-PYTHON-SURFACE.md) |
| Rust | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-RUST-SURFACE.md](references/HOW-TO-MAKE-RUST-SURFACE.md) |
| TypeScript | Command / controller / component | Smart / Utility / Passive tiers; no business logic | [references/HOW-TO-MAKE-TYPESCRIPT-SURFACE.md](references/HOW-TO-MAKE-TYPESCRIPT-SURFACE.md) |

**The layer chain:**

surface (Smart → `_aggregate`, Utility/Passive → taxonomy) → agent → capabilities — never surface→capabilities directly

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `surface_<domain>_<role>` — suffix in the surface allow-list; **`_entry` is root, not surface** (AES101/AES102). |
| Tiers | Smart (`_command`/`_controller`/`_page`): taxonomy + `_aggregate` + utility. Utility (`_hook`/`_store`/`_action`/`_screen`/`_router`): taxonomy only. Passive (`_component`/`_view`/`_layout`): taxonomy only (AES406 / AES201). |
| Behaviour | Zero business logic and zero computation in every tier; Smart delegates through the injected aggregate only. |
| Errors | Never silently discard — no `or None` / `unwrap_or_default()` / `?? UiState…`; return Ok/Err or update an error-state VO. |
| State | All state fields use shared VOs — no primitives, no local domain models. |
| Register | Package module export where required. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Placement (AES703, AES705)

A `surface_*` file belongs in a **surface folder** — a subdirectory of a member directory
(`crates/`, `modules/`, `packages/`) whose name matches the kind of surface it carries:

```
crates/api/src/surface_*_handler.rs        # HTTP API
crates/mcp/src/surface_*_tool.rs           # MCP server
crates/cli/src/surface_*_command.rs        # CLI
crates/desktop/src/surface_*_window.rs      # desktop UI
crates/tui/src/surface_*_screen.rs         # terminal UI
```

The folder name says what kind of surface it is. `api`, `mcp`, `cli`, `desktop`, and `tui`
are the usual names; a surface folder whose name says nothing about the surface it serves
gives the reader no way to tell which entry point uses it.

A surface folder carries **surface files only**. A `capabilities_*` or `agent_*` file found
in one is misplaced — the folder is surface-dominated (more surface files than everything
else combined), and that file must move to a feature folder. This is an **AES703** violation.

Alongside the surface files a surface folder may keep `utility_*` helpers, `root_*` wiring,
and barrels (`lib.rs`, `mod.rs`, `__init__.py`, `index.ts`). Those are allowed.

**Surface folders carry a `DESIGN.md`.** Every surface folder documents how its surface
looks and behaves in `DESIGN.md` at the folder root, next to its source. A surface-dominated
folder carrying no `DESIGN.md` is an **AES705** violation — the linter names the folder and
you write the file.

The file is written for the developer who arrives before you and has to change something
without reading every source file. It carries six sections:

| Section | Answers |
| --- | --- |
| `## Kind` | Which surface this is — `api`, `mcp`, `cli`, `desktop`, `tui` — and what the layer does and does not own. |
| `## Entry Points` | One row per entry: the file, what the user or caller invokes, and which aggregate it reaches. |
| `## Request Shape` | What a request looks like once it arrives, and what has already been resolved. |
| `## States` | Each state a caller can observe, the condition that produces it, and what the user sees. |
| `## Error States` | Each failure mode, where it is caught, and what the caller gets. |
| `## Invariants` | The rules a change must not break. |
| `## Change Checklist` | Which files to touch for which kind of change. |

Write each row against the real code. A `DESIGN.md` that restates the layer definition
instead of naming this folder's files has told the reader nothing they could not get from
`ARCHITECTURE.md`, and AES705 will pass while the file stays useless.

Feature folders carry two documents instead — `FRD.md` and `BACKLOG.md` — recording what the
feature does and where its work stands. `shared/` folders carry neither.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **What tier is this — Smart / Utility / Passive?**
   - *Smart* → inject aggregate; *Utility/Passive* → taxonomy-only imports.
2. **Is the suffix in the surface allow-list (never `_entry`)?**
   - *No* → rename to a legal surface suffix (or `root_*_entry` if it bootstraps).
3. **Any business logic, computation, or direct capability calls?**
   - *Yes* → move logic down; surface only maps and delegates.
4. **Any silently discarded errors or primitive state fields?**
   - *Yes* → propagate errors; wrap state in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Determine the tier (Smart / Utility / Passive) and pick the matching suffix.
2. Resolve the feature surface dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft from the language HOW-TO § Template / § Section Contract; Smart injects the aggregate via DI.
4. Enforce that tier's import rules; strip business logic and computation.
5. Check error handling: nothing discarded, no empty fallbacks; state fields are VOs.
6. Register the module where required, verify with `lint-arwaky-cli scan`, then wire via `aes-root`.

---

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)

```text

A pass means naming, imports, primitives, and roles are clean. Structural judgement
(tier choice, block order, helper-vs-utility, "orchestration only") is **manual** — see HOW-TO § Rules.

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **`_entry` used as a surface suffix**: entry points are root (`root_*_entry`).
- **Surface importing capabilities or protocol directly**: Smart→aggregate only; Utility/Passive→taxonomy only.
- **Silently discarded errors**: forbidden in every tier.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-agent`
- `aes-contract`
- `aes-root`

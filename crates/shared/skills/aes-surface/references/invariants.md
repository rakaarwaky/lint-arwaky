# aes-surface — Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>`.

| Rule | Detail |
|------|--------|
| Naming | File `surface_<domain>_<role>` — suffix in the surface allow-list; **`_entry` is root, not surface** (AES101/AES102). |
| Tiers | Smart (`_command`/`_controller`/`_page`): taxonomy + `_aggregate` + utility. Utility (`_hook`/`_store`/`_action`/`_screen`/`_router`): taxonomy only. Passive (`_component`/`_view`/`_layout`): taxonomy only (AES406 / AES201). |
| Behaviour | Zero business logic and zero computation in every tier; Smart delegates through the injected aggregate only. |
| Errors | Never silently discard — no `or None` / `unwrap_or_default()` / `?? UiState…`; return Ok/Err or update an error-state VO. |
| State | All state fields use shared VOs — no primitives, no local domain models. |
| Register | Package module export where required. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

## Placement (AES703)

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
folder carrying no `DESIGN.md` is an **AES703** violation — the linter names the folder and
you write the file.

Feature folders carry two documents instead — `FRD.md` and `BACKLOG.md` — recording what the
feature does and where its work stands. `shared/` folders carry neither.

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

## Workflow

1. Determine the tier (Smart / Utility / Passive) and pick the matching suffix.
2. Resolve the feature surface dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft from the language HOW-TO § Template / § Section Contract; Smart injects the aggregate via DI.
4. Enforce that tier's import rules; strip business logic and computation.
5. Check error handling: nothing discarded, no empty fallbacks; state fields are VOs.
6. Register the module where required, verify with `lint-arwaky-cli scan`, then wire via `aes-root`.

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

---
name: create-surface
description: AES surface command and UI scaffolding for Python Rust TS. Use when creating surface files.
metadata:
  tags: [python, rust, typescript, aes, surface, smart, utility, passive, di, vo]
  triggers:
    - "create surface"
    - "create surface python"
    - "create surface rust"
    - "create surface typescript"
    - "add surface"
    - "add surface python"
    - "add surface rust"
    - "fix surface structure"
    - "create command"
    - "create command python"
    - "create controller python"
    - "create controller rust"
    - "check surface"
    - "audit surface"
    - "audit surface python"
    - "audit surface rust"
  dependencies: []
  related:
    - create-agent
    - create-taxonomy
    - create-contract
    - create-root
---
# Create Surface (AES)

The **surface layer is how a human or process drives the system**: CLI commands, controllers,
pages, hooks, stores, components. It contains **no business logic** — it maps input events to
VOs, calls an aggregate contract, and renders the result.

Naming (AES101/AES102): `surface_<domain>_<role>.<ext>`; the allowed suffix set is strict:
`_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`,
`_action`, `_screen`. **`_entry` is not a surface suffix** — entry points are the root layer
(`root_<concept>_entry`).

## Three surface types (AES406 tiers, import permissions per AES201)

| Type      | Suffixes                                       | Allowed imports                            | Forbidden                                                |
| ----------- | ------------------------------------------------ | -------------------------------------------- | ---------------------------------------------------------- |
| **Smart**   | `_command`, `_controller`, `_page`            | taxonomy, `contract_*_aggregate`, utility  | capabilities, agents, `contract_*_protocol`, root        |
| **Utility** | `_hook`, `_store`, `_action`, `_screen`, `_router` | taxonomy only                              | smart surfaces, capabilities, agents, `contract_*_protocol`, root |
| **Passive** | `_component`, `_view`, `_layout`              | taxonomy only                              | every other surface, contract, capabilities, agents, root |

Router placement: AES201 groups `router` with hook/store/action/screen (taxonomy-only imports),
while AES406 groups it with the active surfaces for the role check — it therefore appears in the
Utility row above for import purposes.

## Rules

- **Smart**: inject the aggregate (`I<Name>Aggregate` / `Arc<dyn I<Name>Aggregate>`), delegate,
  and return a result VO. It never implements domain rules itself.
- **Utility**: map events → VOs, hold minimal UI state, compose passive surfaces.
- **Passive**: render from VOs only — no computation, no orchestration.
- **Never silently discard errors.** Forbidden shapes:
  `result = self.runner.run(r) or None` (Python), `self.runner.run(&r).unwrap_or_default()` (Rust),
  `this.runner.run(r) ?? UiState.idle()` (TypeScript). Return `Ok`/`Err` or update an error-state VO.
- **All state fields use shared VOs** — no primitives, no local domain models.
- Keep surface-specific mapping in the file (it uses instance state); extract a function to the
  taxonomy utility layer only when it uses no instance state and is pure, domain-agnostic, reusable.

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for the template,
the language's forbidden error expression, and its verify command.

## Workflow

1. Determine the type (Smart / Utility / Passive) and pick the matching suffix.
2. Enforce that type's import rules; wire the aggregate for Smart surfaces.
3. Check error handling: nothing discarded, no empty fallbacks.
4. Register the module where the package requires it, then run the verify command.

## Checklist

- [ ] Correct suffix for the surface type (no `_entry` in this layer).
- [ ] Smart: only taxonomy + `contract_*_aggregate` (+ utility) imports.
- [ ] Utility: taxonomy-only imports; no smart surfaces.
- [ ] Passive: taxonomy imports only.
- [ ] Smart delegates to the aggregate through the injected interface (`Arc<dyn Trait>` in Rust).
- [ ] Zero business logic and zero computation in any surface.
- [ ] No silently discarded errors.
- [ ] All state fields use shared VOs.
- [ ] Verify command passes (`python -c "import <module>"`, `cargo check -p <crate-name>`, `npx tsc --noEmit`).

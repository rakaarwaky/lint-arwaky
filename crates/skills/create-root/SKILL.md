---
name: create-root
description: AES composition root and entry wiring for Python Rust TS. Use when creating container entry files.
metadata:
  tags: [python, rust, typescript, aes, root, container, entry, composition, di, wiring]
  triggers:
    - "create root"
    - "create root python"
    - "create root rust"
    - "create root typescript"
    - "add root"
    - "add root python"
    - "add root rust"
    - "create container"
    - "create container python"
    - "create container rust"
    - "create entry"
    - "create entry python"
    - "create entry rust"
    - "wire dependencies"
    - "wire dependencies python"
    - "check root"
    - "audit root"
    - "audit root typescript"
  dependencies: []
  related:
    - create-capabilities
    - create-agent
    - create-contract
    - create-taxonomy
    - create-surface
---
# Create Root (AES)

The **root layer is the composition layer**. It assembles the system: containers bind concrete
capabilities to contract types, entry points compose the containers and start the process. Root is
the only layer allowed to depend on everything below it, and the only layer permitted to
*construct* implementations.

Naming (AES101/AES102): `root_<concept>_<suffix>.<ext>` with strictly `_container` or `_entry`.
Documented naming exceptions (verbatim from `internal/lint-arwaky/RULES_AES.md`):
**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `root_cli_main_entry.rs`, `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`, `root_composition_container.rs`, `__init__.py`, `index.ts`, `index.js`, barrel/entry files.

## Two root roles

| Role        | Suffix       | Responsibility                                    |
| ------------- | -------------- | --------------------------------------------------- |
| Container   | `_container` | Wire one feature's Capabilities to Contracts      |
| Entry       | `_entry`     | Bootstrap application, compose feature containers |

## Hard rules (Definition of Done, all languages)

1. Correct suffix: `_container` or `_entry`.
2. Container wires capabilities to contract protocols/aggregates; Entry bootstraps the
   application and composes feature containers.
3. Root may instantiate and wire components — that is its whole job.
4. **No business logic.** Domain rules live in capabilities.
5. **No orchestration policy.** Sequencing lives in the agent layer.
6. **No technical parsing or UI behaviour.** Parsing → utility/capabilities; UI → surface.
7. Verify command passes.

## Language split

| Language   | Wiring target                       | Register      | Verify                       |
| ------------ | ------------------------------------- | --------------- | -------------------------------- |
| Python     | Contract ABCs / aggregates            | package `__init__.py` if public | `python -c "import <module>"`  |
| Rust       | Contract traits via `Arc::new(impl)` exposed as `Arc<dyn Trait>` | crate `mod.rs`  | `cargo check -p <crate-name>`  |
| TypeScript | Contract interfaces                 | `index.ts`    | `npx tsc --noEmit`             |

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for that
language's per-role Definition of Done wording and workflow.

## Workflow

1. Determine role — Container (wire one feature) or Entry (bootstrap all)?
2. Create `root_<concept>_<suffix>.<ext>`.
3. Wire dependencies: instantiate capabilities, bind them to contract types/aggregates.
4. Register the module (`mod.rs` for Rust, `index.ts` for TypeScript; Python needs no barrel
   step for private modules).
5. Compose: the entry builds every container and starts the surface or CLI loop.
6. Run the verify command.

## Checklist

- [ ] File is `root_<concept>_container` or `root_<concept>_entry` (or a documented exception name).
- [ ] Container wires capabilities to contract protocols/traits/interfaces and aggregates.
- [ ] Entry bootstraps the application and composes feature containers only.
- [ ] No business logic, no orchestration policy, no parsing, no UI behaviour.
- [ ] Rust only: bindings use `Arc<dyn Trait>` so agents/surfaces see contracts, not concretions.
- [ ] Nothing below root imports from root (it is the top of the dependency arrow).
- [ ] Registered where required; verify command passes.

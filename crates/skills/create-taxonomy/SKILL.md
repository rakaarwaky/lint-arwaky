---
name: create-taxonomy
description: AES taxonomy VO and domain type scaffolding for Python Rust TS. Use when creating taxonomy files.
metadata:
  tags: [python, rust, typescript, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy"
    - "create taxonomy python"
    - "create taxonomy rust"
    - "create taxonomy typescript"
    - "add taxonomy"
    - "move dataclass to taxonomy"
    - "move dataclass to taxonomy typescript"
    - "create vo"
    - "create vo python"
    - "create vo rust"
    - "create vo typescript"
    - "create error taxonomy"
    - "create error taxonomy python"
    - "create error taxonomy rust"
    - "create error taxonomy typescript"
    - "create constant taxonomy"
    - "create constant taxonomy python"
    - "create constant taxonomy rust"
    - "create constant taxonomy typescript"
    - "check taxonomy"
    - "audit taxonomy"
    - "audit taxonomy python"
    - "check taxonomy typescript"
  dependencies: []
  related:
    - create-capabilities
    - create-agent
    - create-contract
---
# Create Taxonomy (AES)

Taxonomy is the **stable language of the domain**: the single source of truth for value objects,
entities, errors, events, and constants. It is the bottom AES layer — it may reference nothing
above it, carries no behaviour beyond its own invariants, and performs no I/O.

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for the
templates, suffix tables, and verify commands of the language you are writing.

## Language split

| Language   | Source root                 | Extension | Register in    | Verify                       | Forbidden runtime refs                              |
| ------------ | ----------------------------- | ----------- | ---------------- | -------------------------------- | ----------------------------------------------------- |
| Python     | `modules/shared/src/<domain>/` | `.py`     | `__init__.py`  | `python -c "import <module>"`  | `open()`/`Path()`/`os.*`/`requests.*`/DB clients |
| Rust       | `crates/shared/src/<domain>/`  | `.rs`     | `mod.rs`       | `cargo check -p <crate-name>`  | `std::fs`/`reqwest`/`sqlx`/`rusqlite`               |
| TypeScript | `packages/shared/src/<domain>/` | `.ts`     | `index.ts`     | `npx tsc --noEmit`             | `fs.*`/`fetch`/database                             |

## File naming (AES101)

`taxonomy_<domain>_<suffix>.<ext>` — lowercase, underscore-separated, minimum 3 words.
Strict suffix set for taxonomy: `_vo`, `_entity`, `_error`, `_event`, `_constant`
(`_utility` files belong to the Utility layer, not taxonomy).

| Type         | Holds                                   | Constraint                                       |
| -------------- | ----------------------------------------- | ------------------------------------------------ |
| Value Object | A single validated, immutable value     | Validate on construction; no I/O               |
| Entity       | Something with identity                 | Identity field must be a VO, not a raw primitive |
| Error        | A domain failure                        | Language error base type; VO payload fields only |
| Event        | A fact that happened                  | Immutable; VO payload fields                     |
| Constant     | Compile-time literal (config, limits) | Pure literal values only — no functions, no I/O |

## Hard rules

1. **No upward imports.** Never import from capabilities, agents, surface, root, or contracts.
   Import other taxonomy types and the standard library only.
2. **No I/O.** No filesystem, network, or database access in VOs, entities, errors, events,
   or constants.
3. **No primitives for domain fields** (AES401). A VO field must not be a raw string, int,
   float, or their collection form — wrap it in another VO. Composite VOs use other VOs.
   Booleans are allowed only as semantic toggles.
4. **Validate on construction.** A VO cannot exist in an invalid state: the constructor (or
   `new()` / `__post_init__`) rejects bad input.
5. **Constants stay literal.** Pure values only; computed values go to the Utility layer.
6. **Immutable.** Public fields are read-only / `final` / have no setters.
7. **Register the file** in the shared barrel (`__init__.py` / `mod.rs` / `index.ts`) or the
   module is dead code.

## Workflow

1. Determine the type (VO / Entity / Error / Event / Constant).
2. Create `taxonomy_<domain>_<type>` with the language extension in the shared source root.
3. Apply the type template from your language reference.
4. Register in the shared barrel.
5. Run the language verify command from the table above.

## Checklist

- [ ] File name is `taxonomy_<domain>_<type>` with a strict-suffix ending and ≥3 words.
- [ ] Correct suffix for the content type.
- [ ] VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ] VOs are immutable; expose the value through a read-only accessor.
- [ ] Errors use the language error base (`Exception` / `std::error::Error` + `Display` / `extends Error`).
- [ ] Constants are pure literal values.
- [ ] Events/Entities payloads are VOs; entities carry an identity VO.
- [ ] No import from capabilities, agents, surface, root, contracts.
- [ ] No I/O, network, or database in taxonomy files.
- [ ] Registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Verify command passes (`python -c "import ..."`, `cargo check -p ...`, `npx tsc --noEmit`).

---
name: create-utility
description: AES stateless utility scaffolding for Python Rust TS. Use when creating utility helper files.
metadata:
  tags: [python, rust, typescript, aes, utility, shared, stateless, pure-function, domain-agnostic, reusability]
  triggers:
    - "create utility"
    - "create utility python"
    - "create utility rust"
    - "create utility typescript"
    - "add utility"
    - "add utility python"
    - "add utility rust"
    - "extract utility"
    - "extract utility python"
    - "extract to utility rust"
    - "move to utility"
    - "move to utility rust"
    - "create helper function"
    - "create helper function python"
    - "check utility"
    - "check utility python"
    - "audit utility"
    - "audit utility python"
  dependencies: []
  related:
    - create-taxonomy
    - create-capabilities
    - create-agent
    - cleanup-consolidate
---
# Create Utility (AES)

The **utility layer holds stateless standalone functions**: no class or struct, no `self`/`this`,
no contract implementation, no domain rules. It sits beside taxonomy as shared, reusable
mechanics (serialization, path handling, formatting, hashing).

Naming (AES101/AES102): `utility_<domain>_<role>.<ext>`. The suffix set is **flexible** — any role
name that describes the file's technical responsibility — but these endings are **forbidden**
(AES102): `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_protocol`, `_aggregate`.

**Import rule (all languages): taxonomy only.** Capabilities, agents, surface, and contract are
forbidden; Rust additionally forbids importing another utility module.

## Role naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the
technical responsibility and concern of the file.

## Rules (AES404 — Utility Role)

1. **Structure:** free/module-level functions only — no `class` (Python), no `struct`/`impl`/trait
   (Rust), no `class` (TypeScript).
2. **State & side effects:** stateless and deterministic — no `random`/`datetime.now()` (Python),
   no `Math.random()`/`Date.now()` (TypeScript), no global mutable state. Side effects are limited
   to domain-agnostic operations (e.g. serialization, hashing, file IO in Python/TS).
3. **Domain awareness:** domain-agnostic — no business rules, no knowledge of layer names.
4. **Reusability:** used by ≥2 modules. A single-consumer function stays a private helper where it
   is used.
5. **I/O:** allowed only when 1–4 all hold (Rust states this explicitly: "I/O is allowed").

Keep as a private helper if **any** of: uses instance state, domain-specific, single consumer.
Extract into utility only if **all** of: no instance state, pure or I/O-safe, domain-agnostic,
≥2 consumers.

## Language split

| Language   | Form                        | No-go constructs              | Register in   | Verify                       |
| ------------ | ----------------------------- | -------------------------------- | ---------------- | -------------------------------- |
| Python     | module-level `def`          | `class`, `self`, `@staticmethod` side tables | `__init__.py` | `python -c "import <module>"`  |
| Rust       | `pub fn` free function      | `struct`, `impl`, traits       | `mod.rs`      | `cargo check -p <crate-name>`  |
| TypeScript | `export function`           | `class`, `this`                | `index.ts`    | `npx tsc --noEmit`             |

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for the file
template (doc-comment style differs) and the language-specific rule wording.

## Workflow

1. Confirm the function is stateless, domain-agnostic, and has ≥2 consumers.
2. Create `utility_<domain>_<role>.<ext>` from the language template.
3. Move constants it needs to `taxonomy_<domain>_constant`; keep business rules in capabilities.
4. Register in `__init__.py` / `mod.rs` / `index.ts`.
5. Run the verify command.

## Checklist

- [ ] File is `utility_<domain>_<role>` with a non-forbidden role suffix.
- [ ] Only free / module-level / exported functions — no class, struct, `impl`, or trait.
- [ ] No instance state (`self` / `&self` / `this`) and no global mutable state.
- [ ] Pure and deterministic, or I/O justified as domain-agnostic.
- [ ] No business rules or layer-name knowledge.
- [ ] Used by ≥2 modules (not a single-consumer helper).
- [ ] Imports taxonomy only — nothing from capabilities, agent, surface, contract (Rust: not from other utilities).
- [ ] No magic constants — literals belong in `taxonomy_*_constant`.
- [ ] Registered in the shared barrel; verify command passes.

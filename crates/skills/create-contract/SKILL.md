---
name: create-contract
description: AES contract protocol scaffolding for Python Rust TS. Use when creating contract ABC trait files.
metadata:
  tags: [python, rust, typescript, aes, contract, protocol, aggregate, abc, trait, interface, vo]
  triggers:
    - "create contract"
    - "create contract python"
    - "create contract rust"
    - "create contract typescript"
    - "add contract"
    - "add contract python"
    - "add contract rust"
    - "create protocol"
    - "create protocol python"
    - "create protocol rust"
    - "create protocol typescript"
    - "create aggregate"
    - "create aggregate python"
    - "create aggregate rust"
    - "create aggregate typescript"
    - "contract missing"
    - "validate contract"
    - "check contract"
    - "check contract rust"
  dependencies: []
  related:
    - create-capabilities
    - create-agent
    - create-taxonomy
---
# Create Contract (AES)

The **contract layer states public promises and implements nothing**. It is the seam that lets the
agent layer drive capabilities and the surface layer drive agents without depending on
concretions. Contracts live in the shared domain package/crate next to taxonomy.

Naming (AES101/AES102): `contract_<concept>_<suffix>.<ext>`, suffix **strictly** `_protocol` or
`_aggregate`. Type names: `I<Name>Protocol`, `I<Name>Aggregate`.

## Contract roles

| Suffix       | Implemented by | Consumed by     |
| -------------- | ---------------- | ----------------- |
| `_protocol`  | Capabilities   | Agent           |
| `_aggregate` | Agent          | Surface / Root |

**Golden rule:** a contract contains only the methods that outer layers actually call. No
private-helper signatures, no convenience API, no default behaviour.

## Import rules (all languages)

**Allowed:** taxonomy types and other contract types.
**Forbidden:** capabilities, agents, surface, root. A contract that imports an implementation
layer inverts the dependency arrow (AES201/AES205).

## Language forms

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for that
language's templates, structure-rule wording, and verify command.

| Language   | Declaration                                  | Body rule                          | Register / verify |
| ------------ | ---------------------------------------------- | ------------------------------------- | ------------------- |
| Python     | `class I<Name>Protocol(ABC):` + `@abstractmethod` | `...` or `pass` — never real code | `__init__.py`, `python -c "import <module>"` |
| Rust       | `pub trait I<Name>Protocol: Send + Sync`      | Method ends with `;`, no default bodies; object-safe | `mod.rs`, `cargo check -p <crate-name>` |
| TypeScript | `export interface I<Name>Protocol`             | Signature only, no class implementation | `index.ts`, `npx tsc --noEmit` |

Signatures must use shared VOs, never primitives: `str`/`int`/`float`/`list[str]`/`dict` (Python),
`String`/`i32`..`u64`/`f32`/`f64`/`Vec<String>` (Rust), `string`/`number`/`string[]`/
`Record<string,T>` (TypeScript). Booleans are allowed only as semantic toggles; `&str` in Rust only
for borrowed non-domain input.

## Workflow

1. Decide the role: implemented by capabilities → `_protocol`; implemented by the agent → `_aggregate`.
2. Apply the Golden Rule to the method list.
3. Create `contract_<concept>_<suffix>` in the shared domain (see the language reference for the template).
4. Register it in `__init__.py` / `mod.rs` / `index.ts`.
5. Run the language verify command, then implement it with `create-capabilities` or `create-agent`.

## Checklist

- [ ] File is `contract_<concept>_<suffix>` with `_protocol` or `_aggregate` only.
- [ ] Pure declaration: `@abstractmethod` only / `pub trait` with `;` methods / `export interface`.
- [ ] No implementations, no default bodies, no private-helper signatures.
- [ ] All methods fully type-annotated.
- [ ] Rust only: trait object-safe (generic members `where Self: Sized`) and `Send + Sync` bounded.
- [ ] Signatures use shared VOs — no raw primitives for domain values.
- [ ] No imports from capabilities, agents, surface, or root.
- [ ] Registered in `__init__.py` / `mod.rs` / `index.ts`; verify command passes.

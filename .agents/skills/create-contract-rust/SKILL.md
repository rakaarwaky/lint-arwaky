---
name: create-contract-rust
description: "Create and validate Rust contract layer files in shared domain: pure trait definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [rust, aes, contract, protocol, aggregate, trait, vo]
  triggers:
    - "create contract rust"
    - "add contract rust"
    - "create protocol rust"
    - "create aggregate rust"
    - "contract missing rust"
    - "validate contract rust"
    - "check contract rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-taxonomy-rust
---

# create-contract-rust

Contract layer = **pure trait definitions** for the shared domain. No implementation. No layer imports. File: `contract_<concept>_<suffix>.rs`.

## Contract Roles

| Suffix | Implemented By | Used By | Example |
| --- | --- | --- | --- |
| `_protocol` | Capabilities | Agent | `contract_import_forbidden_protocol.rs` |
| `_aggregate` | Agent | Surface | `contract_import_runner_aggregate.rs` |

Trait naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contains only trait definitions — no method implementations (methods are `fn ...;` only).
3. No private helper method signatures.
4. All methods have proper type annotations.
5. Traits are `pub trait`.
6. Imports only taxonomy and other contract types.
7. Signatures use shared VOs for domain data.
8. Error types from shared taxonomy.
9. Module registered in shared `mod.rs`.
10. `cargo check -p <crate-name>` passes.

---

## Purity and Import Restrictions (AES201)

| Contract File | May Import From | Must Not Import From |
| --- | --- | --- |
| `contract_*_protocol.rs` | taxonomy types, other contract types | capabilities, agents, surface, root |
| `contract_*_aggregate.rs` | taxonomy types, other contract types | capabilities, agents, surface, root |

---

## Trait Structure Rules

1. Contracts contain trait definitions only.
2. No default method implementations (methods end with `;`).
3. No private helper method signatures.
4. All methods MUST have proper type annotations.
5. Traits MUST be `pub trait`.
6. Error types from shared taxonomy.
7. Naming: `I<Name>Protocol`, `I<Name>Aggregate`.
8. Object-safe by default — avoid generics unless needed.

---

## VO Rules

Contract signatures must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `String`, `i32`..`u64`, `f32`/`f64`, `char` | Forbidden for domain fields/contract values. Use VO. |
| `bool` | Allowed for semantic toggles only. |
| `&str` | May be used for borrowed low-level input; domain identifiers → VO. |
| `Vec<String>` | Forbidden for domain collections. Use VO. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/contract_name_protocol.rs` | Protocol trait definition |
| `templates/contract_name_aggregate.rs` | Aggregate trait definition |
| `templates/mod.rs` | Module registration |

---

## Workflow

1. **Determine role** — Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. **Identify public methods** — Golden Rule: called by outer layers? YES → keep. NO → private helper (not in trait).
3. **Create file** → `contract_<concept>_<suffix>.rs` in shared domain.
4. **Register** → update `mod.rs`.
5. **Verify** → `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ] Correct suffix: `_protocol` or `_aggregate`.
- [ ] Only trait definitions — no implementations.
- [ ] No private helper signatures.
- [ ] All methods type-annotated.
- [ ] Traits are `pub trait`.
- [ ] Imports only taxonomy and contract types.
- [ ] No import from capabilities, agents, surface.
- [ ] Signatures use shared VOs.
- [ ] Error types from shared taxonomy.
- [ ] Registered in shared `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

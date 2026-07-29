---
name: create-taxonomy-rust
description: "Create and validate Rust taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
  tags: [rust, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy rust"
    - "add taxonomy rust"
    - "move dataclass to taxonomy rust"
    - "create vo rust"
    - "create error taxonomy rust"
    - "create constant taxonomy rust"
    - "check taxonomy rust"
    - "audit taxonomy rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-contract-rust
---

# create-taxonomy-rust

Taxonomy = stable domain language. Single source of truth for VOs, entities, errors, events, constants. Location: `crates/shared/src/<domain>/`.

**Allowed imports:** other taxonomy types, std.
**Forbidden:** capabilities, agents, surface, root, contracts, `std::fs`/network/database (in VOs/entities/errors/events/constants).

## File Types

| Suffix | Content | Key constraint |
| --- | --- | --- |
| `_vo.rs` | Value Objects | Validate in `new()`, immutable fields, no I/O |
| `_entity.rs` | Entities with identity | Identity VO field required |
| `_error.rs` | Domain errors | Implement `std::error::Error` + `Display` |
| `_event.rs` | Domain events | Immutable, VO payload fields |
| `_constant.rs` | Compile-time constants | `pub const` only — no functions |
| `_utility.rs` | Stateless helpers | No struct, no `impl`, domain-agnostic |

## VO Rules (AES401/AES402)

Forbidden for domain fields: `String`, `i32`..`u64`, `f32`/`f64`, `Vec<String>`.
`bool` and `&str` (for non-domain borrowed input) allowed with care.

## Templates

| File | Purpose |
| --- | --- |
| `templates/taxonomy_name_vo.rs` | Value Object |
| `templates/taxonomy_name_entity.rs` | Entity |
| `templates/taxonomy_name_error.rs` | Error type |
| `templates/taxonomy_name_constant.rs` | Constants |

## Workflow

1. Determine type (VO/Entity/Error/Event/Constant/Utility).
2. Create `taxonomy_<domain>_<type>.rs` in `shared/src/<domain>/`.
3. VOs: `fn new(...) -> Result<Self, DomainError>` or invariant check in `new`.
4. Errors: impl `std::error::Error` + `Display`.
5. Constants: `pub const NAME: Type = value;` only.
6. Register in `mod.rs`.
7. `cargo check -p <crate-name>`.

## Checklist

- [ ] Correct suffix.
- [ ] VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ] Errors implement `std::error::Error`.
- [ ] Constants are `pub const` pure literal values.
- [ ] No import from capabilities, agents, surface, root, contracts.
- [ ] No I/O, network, or database in taxonomy files.
- [ ] Registered in shared `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

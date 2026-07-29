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

Taxonomy = **stable language of the domain**. Single source of truth for VOs, entities, errors, events, constants. Free from technical/behavioral concerns. Location: `crates/shared/src/<domain>/`.

## Taxonomy Types

| File Suffix | Content | Rules |
| --- | --- | --- |
| `_vo.rs` | Value Objects | Validate on construction, immutable (`readonly` fields), no I/O |
| `_entity.rs` | Entities with identity | Identity field required (VO), no I/O |
| `_error.rs` | Domain error types | Implement `std::error::Error`, VO fields only |
| `_event.rs` | Domain event types | Immutable, VO payload fields |
| `_constant.rs` | Compile-time constants | `pub const` only, no functions, no I/O |
| `_utility.rs` | Stateless helper functions | No struct, no impl, pure, domain-agnostic |

## Definition of Done

1. Correct file suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).
2. VOs validate on construction when invariants exist (return `Result` or panic on invalid).
3. No raw primitives in struct fields — use other VOs.
4. No I/O, no side effects, no business logic in VOs/entities/errors/events/constants.
5. Constants: `pub const` pure literal values only.
6. Taxonomy imports only other taxonomy types or std.
7. No import from capabilities, agents, surface, root, contracts.
8. Registered in shared `mod.rs`.
9. `cargo check -p <crate-name>` passes.

---

## Purity and Import Restrictions (AES201/AES401)

| Taxonomy Type | May Import From | Must Not Import From |
| --- | --- | --- |
| `_vo`, `_entity`, `_error`, `_event` | other taxonomy types, std | capabilities, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values | external layer imports, I/O |

**Taxonomy MAY contain:** value validation, domain invariants in constructors, pure transformations between taxonomy types.

**Taxonomy MUST NOT contain:** file I/O (`std::fs`), network (`reqwest`), database, env mutation, side effects, business orchestration.

---

## VO Rules (AES401/AES402)

Domain data MUST use VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `String`, `i32`..`u64`, `f32`/`f64`, `char` | Forbidden for domain fields. Use VO. |
| `bool` | Allowed for semantic toggles only. |
| `&str` | May be used for borrowed low-level input; domain identifiers → VO. |
| `Vec<String>` | Forbidden for domain collections. Use VO. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/taxonomy_name_vo.rs` | Value Object template |
| `templates/taxonomy_name_entity.rs` | Entity template |
| `templates/taxonomy_name_error.rs` | Error type template |
| `templates/taxonomy_name_constant.rs` | Constants template |

---

## Workflow

1. **Determine type** — VO / Entity / Error / Event / Constant / Utility?
2. **Create file** → `taxonomy_<domain>_<type>.rs` in `shared/src/<domain>/`.
3. **VOs**: validate in `new()` → `Result<Self, DomainError>` or `Self` with invariant check.
4. **Entities**: add identity VO field.
5. **Errors**: implement `std::error::Error` + `Display`.
6. **Constants**: `pub const NAME: Type = value;` only.
7. **Register** → update `mod.rs`.
8. **Verify** → `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ] Correct file suffix.
- [ ] VOs validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types implement `std::error::Error`.
- [ ] Constants are `pub const` pure literal values.
- [ ] No import from capabilities, agents, surface, root, contracts.
- [ ] No I/O, no network, no database in taxonomy files.
- [ ] Registered in shared `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

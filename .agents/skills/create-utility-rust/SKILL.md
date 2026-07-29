---
name: create-utility-rust
description: "Create and validate Rust utility layer files following AES rules: stateless standalone functions, no struct, no trait impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags: [rust, aes, utility, stateless, pure-functions, domain-agnostic, reusability, taxonomy]
  triggers:
    - "create utility rust"
    - "add utility rust"
    - "extract to utility rust"
    - "move to utility rust"
    - "check utility rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - cleanup-consolidate-rust
---

# create-utility-rust

Utility layer = **stateless standalone functions**. No struct, no `impl`, no domain rules. Pure, domain-agnostic, reusable. File: `utility_<domain>_<role>.rs`.

## Role Naming

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

## Definition of Done

1. Only free functions — no `struct`, no `impl` blocks.
2. No `&self`, no struct fields, no instance state.
3. Pure: same input → same output (except I/O utilities).
4. Domain-agnostic: no business rules, no architecture knowledge.
5. Reusable: used by ≥2 modules (otherwise keep as private helper).
6. I/O allowed only if stateless + domain-agnostic + reusable.
7. No `use` from Capabilities, Agent, Surface, Contract modules.
8. May use from Taxonomy only.
9. `cargo check -p <crate-name>` passes.

---

## Stateless Rules

1. **No structs** — no `struct`, no `impl`, no `&self`.
2. **Pure functions** — deterministic: same input → same output. No `rand`, no `SystemTime::now()`, no global mutable state.
3. **Domain-agnostic** — must NOT know about: architecture layer names (agent, capabilities, contract), business domain rules, specific capability logic.
4. **Reusable** — if only one module uses it → keep as private helper in that module.

### I/O Exception

Utility CAN perform I/O if ALL conditions met: stateless (no `self`), domain-agnostic, reusable across multiple modules.

---

## Keep vs Extract Decision

**Keep as private helper** if ANY: accesses `&self`/struct state, domain-specific, only one consumer.

**Extract to utility** only if ALL: stateless, pure (I/O allowed), domain-agnostic, ≥2 consumers.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Stateless free functions (`pub fn`) | Struct definitions |
| Pure computation (input → output) | `&self` / `self` / struct state |
| I/O (if domain-agnostic + reusable) | Business rules / domain knowledge |
| Taxonomy imports (`shared::taxonomy_*`) | `use` from Capabilities, Agent, Surface |
| File walking, pattern matching, parsing | Trait implementations |
| Environment access (if stateless + reusable) | Magic constants (→ `taxonomy_*_constant.rs`) |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/utility_name.rs` | Utility function module template |

---

## Workflow

1. **Confirm reusability** — Is this used by ≥2 modules? If no → keep as private helper.
2. **Confirm stateless** — No `self`, no struct, no global mutation?
3. **Confirm domain-agnostic** — No business rules, no architecture knowledge?
4. **Create file** → `utility_<domain>_<role>.rs`.
5. **Register** → update `mod.rs`.
6. **Verify** → `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ] Only free functions — no struct, no impl.
- [ ] No `&self`, no instance state.
- [ ] Pure / deterministic (or I/O with domain-agnostic + reusable justification).
- [ ] Domain-agnostic — no business rules, no layer-name knowledge.
- [ ] Used by ≥2 modules (otherwise keep as private helper).
- [ ] No `use` from Capabilities, Agent, Surface, Contract.
- [ ] No magic constants (extracted to `taxonomy_*_constant.rs`).
- [ ] `cargo check -p <crate-name>` passes.

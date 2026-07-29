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

Utility = stateless standalone functions. No struct, no `impl`, no domain rules. File: `utility_<domain>_<role>.rs`.

**Allowed imports:** Taxonomy only (`shared::taxonomy_*`).
**Forbidden:** `use` from Capabilities, Agent, Surface, Contract.

## Role Naming

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

## Rules

1. Only `pub fn` free functions — no `struct`, no `impl`.
2. Pure + deterministic — no `rand`, no `SystemTime::now()`, no global mutable state.
3. Domain-agnostic — no business rules, no layer-name knowledge.
4. Reusable — used by ≥2 modules; if single consumer → keep as private helper.
5. I/O allowed only if all above hold.

**Keep as private helper** if ANY: uses `&self`, domain-specific, single consumer.
**Extract here** only if ALL: no `self`, pure/I/O-safe, domain-agnostic, ≥2 consumers.

## Templates

`templates/utility_name.rs`

## Workflow

1. Confirm ≥2 consumers, stateless, domain-agnostic.
2. Create `utility_<domain>_<role>.rs`.
3. Register in `mod.rs`.
4. `cargo check -p <crate-name>`.

## Checklist

- [ ] Only free functions — no struct, no impl.
- [ ] No `&self`, no instance state.
- [ ] Pure/deterministic (or I/O justified: domain-agnostic + reusable).
- [ ] No business rules or layer-name knowledge.
- [ ] Used by ≥2 modules.
- [ ] No `use` from Capabilities, Agent, Surface, Contract.
- [ ] No magic constants (→ `taxonomy_*_constant.rs`).
- [ ] `cargo check -p <crate-name>` passes.

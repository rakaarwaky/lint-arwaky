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
**Forbidden:** `use` from Capabilities, Agent, Surface, Contract, or other Utility modules.

## Role Naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

## Templates

### utility_name.rs

```rust
// PURPOSE: <Domain> utility functions — stateless, pure, domain-agnostic
// Free functions only — no struct, no impl blocks.
use shared::taxonomy::<domain>_vo::<VO>;

/// <Description of what this function does>
///
/// # Arguments
/// * `<param_name>` — <description>
///
/// # Returns
/// <description of return value>
pub fn <function_name>(<param_name>: &<Type>) -> <ReturnType> {
    // pure function logic here
}
```

## Rules

1. **Structure:** Only `pub fn` free functions — absolutely no `struct`, no `impl` blocks, no traits.
2. **State & Side Effects:** Stateless & deterministic. Side-effects are strictly limited to domain-agnostic operations 
3. **Domain Awareness:** Domain-agnostic — no business rules, no layer-name knowledge.
4. **Reusability:** Must be used by ≥2 modules. If it has a single consumer, keep it as a private helper in the consuming module.
5. **I/O Constraint:** I/O is allowed 

## Helper vs Utility Decision Matrix

**Keep as private helper** (in Capabilities/Agent) if ANY of these apply:

- Domain-specific (contains business rules).
- Single consumer.

**Extract to Utility** ONLY if ALL of these apply:

- No `self` (stateless free function).
- Pure / deterministic (or domain-agnostic I/O).
- Domain-agnostic (no business rules).
- ≥2 consumers (reusable across modules).

## Workflow

1. Confirm ≥2 consumers, stateless, and domain-agnostic.
2. Create `utility_<domain>_<role>.rs`.
3. Register in `mod.rs`.
4. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Only free functions — no struct, no impl, no traits.
- [ ]  No `&self`, no instance state.
- [ ]  Pure/deterministic (or I/O strictly limited to domain-agnostic ops like serialization/hashing).
- [ ]  No business rules or layer-name knowledge.
- [ ]  Used by ≥2 modules (not a single-consumer helper).
- [ ]  No `use` from Capabilities, Agent, Surface, or Contract.
- [ ]  No magic constants (→ move to `taxonomy_*_constant.rs`).
- [ ]  `cargo check -p <crate-name>` passes.

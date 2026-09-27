# HOW TO MAKE UTILITY RUST

> **Purpose**: Hold stateless, domain-agnostic helper functions shared across the system.
>
> **Audience**: Agents and engineers scaffolding AES utility files in the shared domain.
>
> **Scope**: Python, Rust, and TypeScript `utility_<domain>_<role>` files — free functions only, no class/struct/impl.
>
> **Location**: Shared domain source root next to taxonomy, registered in the shared barrel.
>
> **Length**: Free functions only; used by ≥2 modules; taxonomy-only imports.

---

## Rules

### Import rules

**Allowed imports:** Taxonomy only (`shared::taxonomy_*`).
**Forbidden:** `use` from Capabilities, Agent, Surface, Contract, or other Utility modules.

1. **Structure:** Only `pub fn` free functions — absolutely no `struct`, no `impl` blocks, no traits.
2. **State & Side Effects:** Stateless & deterministic. Side-effects are strictly limited to domain-agnostic operations
3. **Domain Awareness:** Domain-agnostic — no business rules, no layer-name knowledge.
4. **Reusability:** Must be used by ≥2 modules. If it has a single consumer, keep it as a private helper in the consuming module.
5. **I/O Constraint:** I/O is allowed

### Helper vs Utility Decision Matrix

**Keep as private helper** (in Capabilities/Agent) if ANY of these apply:

- Domain-specific (contains business rules).
- Single consumer.

**Extract to Utility** ONLY if ALL of these apply:

- No `self` (stateless free function).
- Pure / deterministic (or domain-agnostic I/O).
- Domain-agnostic (no business rules).
- ≥2 consumers (reusable across modules).

### Workflow

1. Confirm ≥2 consumers, stateless, and domain-agnostic.
2. Create `utility_<domain>_<role>.rs`.
3. Register in `mod.rs`.
4. `cargo check -p <crate-name>`.

---

## Template

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

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Only free functions — no struct, no impl, no traits. | Required by AES layer rules and the linter; missing it is a defect. |
| No `&self`, no instance state. | Required by AES layer rules and the linter; missing it is a defect. |
| Pure/deterministic (or I/O strictly limited to domain-agnostic ops like serialization/hashing). | Required by AES layer rules and the linter; missing it is a defect. |
| No business rules or layer-name knowledge. | Required by AES layer rules and the linter; missing it is a defect. |
| Used by ≥2 modules (not a single-consumer helper). | Required by AES layer rules and the linter; missing it is a defect. |
| No `use` from Capabilities, Agent, Surface, or Contract. | Required by AES layer rules and the linter; missing it is a defect. |
| No magic constants (→ move to `taxonomy_*_constant.rs`). | Required by AES layer rules and the linter; missing it is a defect. |
| `cargo check -p <crate-name>` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): stateless, domain-agnostic, ≥2 consumers; no class/`self`/`this`/struct/impl.
# Fallback compile gate: cargo check -p <crate-name>
```

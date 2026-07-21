---
name: create-contract-rust
description: "Create and validate Rust contract layer files in shared domain: pure trait definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
    tags: [rust, aes, contract, protocol, aggregate, trait, shared, di, vo]
    triggers:
        - "create contract rust"
        - "add contract rust"
        - "create protocol rust"
        - "create aggregate rust"
        - "fix contract rust"
        - "check contract rust"
        - "audit contract rust"
    dependencies: []
    related:
        - create-taxonomy-rust
        - create-capabilities-rust
        - create-agent-rust
---

# create-contract-rust

## Purpose

Create and validate Rust **contract layer** files in shared domain.

Contracts are pure trait definitions.

They define the **WHAT**: public promises, stable interfaces, polymorphism boundaries, DI boundaries.

They MUST NOT define the **HOW**: no implementation, no private helpers, no I/O, no business logic, no layer imports.

Two contract suffixes serve different roles:

- `_protocol` → implemented by Capabilities (inbound behavior interface)
- `_aggregate` → implemented by Agent (facade for Surface to access feature behavior)

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contract contains only trait definitions.
3. Contract contains no `impl` blocks or default method bodies.
4. Contract contains no private helper signatures.
5. Trait includes `Send + Sync` bounds.
6. Trait is object-safe when intended for `Arc<dyn Trait>`.
7. Contract imports only taxonomy and contract types.
8. Contract signatures use shared VOs for domain data.
9. New contract module is registered in `mod.rs`.
10. `cargo check -p shared` passes.

## References

| File | Content |
|------|---------|
| `references/contract-roles.md` | Two suffix types and naming convention |
| `references/purity-imports.md` | AES201 import restrictions |
| `references/trait-structure-rules.md` | 7 trait structure rules |
| `references/primitive-vo-policy.md` | Primitive policy table |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/contract_name_protocol.rs` | New protocol trait definition |
| `templates/contract_name_aggregate.rs` | New aggregate trait definition |

## Workflow

### Step 1: Determine the Contract Role

Ask: **"Which layer will implement this interface?"**

| Implemented By | Suffix |
|----------------|--------|
| Capabilities | `_protocol` |
| Agent | `_aggregate` |

### Step 2: Identify Public Methods

Apply the Golden Rule: Is this method called by outer layers? YES → keep in contract. NO → make it a private helper.

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.rs` in the appropriate shared domain folder.

### Step 4: Register Module

Update the domain `mod.rs`.

### Step 5: Verify

```bash
cargo check -p shared
```

## Quick Commands

```bash
# List contract traits
rg -n "^\s*pub trait" crates/shared/src/**/contract_*.rs

# Check forbidden imports
rg -n "^\s*use\s+.*(capabilities_|agent_|surface_)" crates/shared/src/**/contract_*.rs
```

## Common Mistakes

- Putting implementation logic in contract files.
- Adding default method bodies to contract traits.
- Importing concrete layer types into contracts.
- Using wrong suffix for contract files.
- Leaking implementation details into contract traits.
- Forgetting `Send + Sync` bounds for DI traits.
- Forgetting object safety for `Arc<dyn Trait>` usage.
- Using raw `String` for domain values in contract signatures.
- Forgetting to register contract modules in `mod.rs`.

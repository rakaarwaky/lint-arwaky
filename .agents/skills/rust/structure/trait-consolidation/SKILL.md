````markdown
---
name: trait-consolidation-rust
description: "Idiomatic Rust refactoring for clean architecture using the 3-Block Structure."
version: 4.0.0
category: refactoring
tags:
  [
    rust,
    trait,
    protocol,
    port,
    interface-segregation,
    encapsulation,
    di,
    3-block-structure,
  ]
triggers:
  - "consolidate trait methods"
  - "refactor to 3-block structure"
  - "split god trait"
  - "organize impl blocks"
  - "extract trait contract"

related:
  - fix-cross-import
  - enforce-1-class-per-file
  - rust-error-handling
---

# trait-consolidation-rust

## Purpose

Refactor Rust implementation files into a clean, idiomatic **3-Block Structure**. This ensures that public contracts (Traits) are clearly separated from internal implementation details (Private Helpers), improving readability, testability, and encapsulation.

**CRITICAL: The 3-Block Structure** — Every implementation file MUST follow this exact order:

1. `struct Definition`
2. `impl Trait for Struct` (Public Contract)
3. `impl Struct` (Constructors & Private Helpers)

## Rules

- **Trait = Public Contract Only**: Only methods that form the external API or `pub(crate)` boundary belong in the trait.
- **Encapsulate Private Helpers**: Private helpers, utilities, and internal logic **MUST** stay in the inherent `impl Struct` block (Block 3). Do NOT pollute the trait with implementation details.
- **Interface Segregation**: If a struct has multiple distinct responsibilities, split them into multiple focused traits (e.g., `ICoreProtocol`, `IHelperTrait`) and implement them separately.
- **Constructors in Block 3**: `new()`, `default()`, and builder methods stay exclusively in the inherent `impl Struct` block.
- **Object Safety**:
  - Add `where Self: Sized` to generic trait methods.
  - Do NOT artificially add `&self` to pure utility functions. If a function doesn't need state, make it an inherent method without `self` or a module-level function.
- **Extract Shared Logic**: When multiple public methods share logic, extract it into a private inherent method (e.g., `fn shared_helper(&self)`) in Block 3, called by the public trait methods in Block 2.

## Contract Rules

- **If contract exists**: Update the existing trait to include ONLY the public/contract method signatures. Remove any private helpers that were incorrectly placed there.
- **If contract doesn't exist**: Create a new trait file following the naming convention, containing only the public contract.
- **Register trait module**: Add the new trait file to `mod.rs` in the shared crate so it's exported.

## Naming Convention

| Layer              | Trait File                     | Trait Name         | Impl File                  |
| ------------------ | ------------------------------ | ------------------ | -------------------------- |
| **Capabilities**   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  | `capabilities_<name>.rs`   |
| **Infrastructure** | `contract_<name>_port.rs`      | `I<Name>Port`      | `infrastructure_<name>.rs` |
| **Agents**         | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` | `agent_<name>.rs`          |

## When to Use

- A struct has methods scattered across multiple `impl` blocks without clear separation.
- A trait has become a "God Trait" containing private helpers.
- You want to improve readability by placing the public contract above internal details.
- Refactoring to align with Ports & Adapters / Hexagonal Architecture in an idiomatic Rust way.

## The 3-Block Pattern

### 1. Trait File (`contract_<name>_protocol.rs`)

_Contains ONLY the public contract. No private helpers._

```rust
pub trait I<Name>Protocol: Send + Sync {
    // Public API methods only
    fn public_method(&self, input: &str) -> Result<String, MyError>;

    // Async methods
    async fn async_method(&self, id: u32) -> Option<Data>;

    // Generic methods (must be Sized for object safety)
    fn generic_fn<F>(&self, mapper: F) -> Vec<String>
    where
        Self: Sized,
        F: Fn(&str) -> String;
}
```
````

### 2. Implementation File (`capabilities_<name>.rs`)

_Follows the strict 3-Block Structure._

```rust
use shared::contract_<name>_protocol::I<Name>Protocol;

// BLOCK 1: STRUCT DEFINITION
pub struct <Type> {
    // fields only
}

// BLOCK 2: IMPL TRAIT FOR STRUCT (Public Contract)
// Placed here so readers see the "Role" of the struct first.
impl I<Name>Protocol for <Type> {
    fn public_method(&self, input: &str) -> Result<String, MyError> {
        // Call private helper from Block 3
        self.shared_internal_logic(input)?;
        Ok(format!("Processed: {}", input))
    }

    async fn async_method(&self, id: u32) -> Option<Data> {
        None
    }

    fn generic_fn<F>(&self, mapper: F) -> Vec<String>
    where
        Self: Sized,
        F: Fn(&str) -> String,
    {
        vec![]
    }
}

// BLOCK 3: IMPL STRUCT (Constructors & Private Helpers)
// Internal details are kept below the public contract.
impl <Type> {
    pub fn new(...) -> Self {
        Self { ... }
    }

    // Private helper (encapsulated, not in trait)
    fn shared_internal_logic(&self, param: &str) -> Result<(), MyError> {
        // Extracted shared logic
        Ok(())
    }
}
```

## Step-by-Step Process

### Step 1: Audit Existing Methods

Identify which methods are truly part of the external contract (used by other modules/traits) and which are internal implementation details.

### Step 2: Extract Shared Logic

If multiple public methods share logic, create a `fn shared_logic(&self)` in Block 3 (Inherent impl). Do NOT put this in the trait unless other structs need to override it.

### Step 3: Define/Update the Trait

Add only the public/contract method signatures to the trait file. Add `where Self: Sized` to generic methods.

### Step 4: Reorganize into 3 Blocks

Structure the implementation file strictly:

1. `pub struct <Type>`
2. `impl I<Name>Protocol for <Type>` (All public contract methods)
3. `impl <Type>` (Constructors and all private/helpers)

### Step 5: Register and Verify

Add the trait to `mod.rs` and run verification commands.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Struct -> Impl Trait -> Impl Struct).
- [ ] Trait contains **only** public/contract methods (no private helpers).
- [ ] Private helpers and utilities are in Block 3 (`impl Struct`).
- [ ] Constructors (`new`, builders) are in Block 3.
- [ ] Generic trait methods include `where Self: Sized`.
- [ ] Pure utility functions do not artificially force `&self`.
- [ ] Trait module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes without warnings or errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^impl\|^pub struct" crates/<crate>/src/capabilities_<name>.rs

# Ensure trait does NOT contain private helper keywords
grep -E "fn (helper|util|private|internal)" crates/shared/src/contract_*.rs || echo "Clean: No helpers in trait"

# Check for object safety violations
cargo check -p <crate-name> 2>&1 | grep "cannot be made into an object"

# Ensure all trait methods are implemented
cargo check -p <crate-name> 2>&1 | grep "not a member of trait"
```

## Common Mistakes (AVOID)

- ❌ **Putting private helpers in the trait**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave trait methods and private helpers. Keep them in separate `impl` blocks.
- ❌ **Creating "God Traits"**: If a trait has >10 methods or mixes unrelated concerns, split it into multiple traits.
- ❌ **Forgetting `where Self: Sized`**: This will break `dyn Trait` usage for the rest of the trait.
- ❌ **Placing `new()` in the trait impl**: Constructors must stay in the inherent `impl Struct` block (Block 3).

```

```

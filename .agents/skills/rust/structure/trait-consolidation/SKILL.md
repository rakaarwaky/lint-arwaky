---
name: trait-consolidation-rust
description: "rait-consolidation-rust"
version: 2.0.0
category: refactoring
tags: [aes, trait, protocol, port, consolidation, rust, interface, single-impl]
triggers:
  - "consolidate trait methods"
  - "add fn to trait protocol"
  - "trait consolidation"
  - "merge fn signatures into trait"
  - "move helper to trait"

related:
  - fix-cross-import
  - enforce-1-class-per-file
---
# trait-consolidation-rust

## Purpose

Consolidate ALL function signatures from a capability/infrastructure/agent implementation file into its corresponding trait. Makes every method part of the contract for DI and verifiability.

**CRITICAL: Single `impl Trait` block only** — every fn in the impl file MUST be inside exactly one `impl I<Name>Protocol/Port/Aggregate for <Type>` block. Zero exceptions.

## Rules

- **ONE impl block**: All methods (public, private, helpers, utilities) go into a single `impl I<Name>Protocol for <Type>` block. No separate inherent `impl Type` blocks for methods.
- **EVERY fn in trait**: Including private helpers and utility functions. Zero exceptions.
- **Only `new()` in inherent impl**: The constructor is the sole exception — it stays in its own `impl Type { pub fn new(...) }` block. Everything else goes in trait impl.
- **Same names, no prefixes**: All trait methods keep their original names — no `do_`, `pure_`, or any other prefix.
- **All trait methods have `&self`**: Sync methods use `&self`. Async methods use `async fn(&self, ...)`.
- **Generic bounds need `where Self: Sized`**: Add this clause to generic trait methods.
- **Extract shared logic**: When two+ methods share common logic, extract it into one consolidated helper that all callers use. This eliminates duplication and creates a single trait method for the shared logic.

## Contract Rules

- **If contract exists**: Update the existing contract to include ALL fn signatures from impl file (including private helpers)
- **If contract doesn't exist**: Create a new contract file following naming convention
- **If contract needs replacement**: Replace old contract with unified contract (merge methods from multiple traits)
- **Register trait module**: Add the new trait file to `mod.rs` in the shared crate so it's exported

## Naming Convention


| Layer              | Trait File                     | Trait Name         | Impl File                  |
| -------------------- | -------------------------------- | -------------------- | ---------------------------- |
| **Capabilities**   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  | `capabilities_<name>.rs`   |
| **Infrastructure** | `contract_<name>_port.rs`      | `I<Name>Port`      | `infrastructure_<name>.rs` |
| **Agents**         | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` | `agent_<name>.rs`          |

## When to Use

- Capability/Infra/Agent impl file has fn methods NOT in the trait (including private helpers)
- You want to make all methods part of the contract interface
- Refactoring and want to document new functionality in trait first
- Need to create a new contract for an impl file that doesn't have one yet
- Need to replace/update an existing contract with merged methods
- Two+ methods share common logic — extract into one shared helper

## The Pattern

### Trait File (`contract_<name>_protocol.rs`, `contract_<name>_port.rs`, or `contract_<name>_aggregate.rs`)

```rust
pub trait I<Name>Protocol: Send + Sync {    // Protocol for capabilities
pub trait I<Name>Port: Send + Sync {        // Port for infrastructure
pub trait I<Name>Aggregate: Send + Sync {   // Aggregate for agents

    // ALL methods — same name as in impl, no prefixes
    // Includes public API, private helpers, and utility functions
    fn public_method(&self, ...) -> ...;
    fn another_method(&self, ...) -> ...;
    fn shared_helper(&self, ...) -> ...;     // extracted from multiple callers
    fn internal_utility(&self, ...) -> ...;  // even private helpers

    // Generic functions — add where Self: Sized
    fn generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...;

    // Async methods
    async fn async_method(&self, ...) -> ...;
}
```

### Implementation File (`capabilities_<name>.rs`, `infrastructure_<name>.rs`, or `agent_<name>.rs`)

```rust
pub struct <Type> {
    // fields only
}

impl <Type> {
    pub fn new(...) -> Self {
        Self { ... }
    }
    // NOTHING ELSE — no methods here
}

impl I<Name>Protocol for <Type> {
    // ALL methods — same name as trait, implementation directly here
    fn public_method(&self, ...) {
        // actual implementation logic
    }

    fn another_method(&self, ...) {
        // actual implementation logic
    }

    fn shared_helper(&self, ...) {
        // extracted shared logic — called by multiple methods
    }

    fn internal_utility(&self, ...) {
        // even private helpers are in trait impl
    }

    fn generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...,
    {
        // actual implementation logic
    }

    async fn async_method(&self, ...) {
        // actual implementation logic
    }
}
```

**Key**: Only `new()` stays in its own inherent `impl Type` block. ALL other methods — public, private, helpers, utilities — go into the single trait impl block. No `do_`, `pure_`, or any prefix modifications.

## Step-by-Step Process

### Step 1: Count fn in Impl File

```bash
# Count all fn signatures (pub, private, async)
grep -c "^    fn \|^    pub fn \|^    async fn " path/to/impl_file.rs
```

### Step 2: Count fn in Trait File

```bash
grep -c "^    fn \|^    async fn " path/to/contract_<name>_protocol_or_port_or_aggregate.rs
```

### Step 3: Extract Shared Logic (Consolidate)

When two+ methods share common patterns (e.g., same loop structure, same parsing logic), extract it into one shared helper:

```rust
// Before: duplicated logic in check_forbidden_imports and check_scope_forbidden_imports
fn check_forbidden_imports(&self, ...) {
    // ... parse imports ...
    for (line_num, line) in import_lines {
        // common loop body
    }
}

fn check_scope_forbidden_imports(&self, ...) {
    // ... parse imports ...
    for (line_num, line) in import_lines {
        // SAME common loop body — duplicate!
    }
}

// After: extract shared logic into one helper
fn check_forbidden_imports(&self, ...) {
    let import_lines = self.parser.read_import_lines(&file_path);
    self.check_imports_against_forbidden(&import_lines, &forbidden_list, ...);
}

fn check_scope_forbidden_imports(&self, ...) {
    let import_lines = self.parser.read_import_lines(&file_path);
    self.check_imports_against_forbidden(&import_lines, &rule.forbidden.values, ...);
}

// Shared helper — ONE method that both call
fn check_imports_against_forbidden(&self, ..., violations: &mut Vec<LintResult>) {
    for (line_num, line) in import_lines {
        // the common loop body — extracted once
    }
}
```

### Step 4: Add ALL fn Signatures to Trait File

For each fn in impl file:

- Keep **original name** (no prefixes)
- All methods: use `&self`
- Async methods: use `async fn(&self, ...)`
- Generic functions: add `where Self: Sized`

```rust
pub trait I<Name>Protocol: Send + Sync {
    // Public API
    fn public_method(&self, ...) -> ...;
    async fn async_api(&self, ...) -> ...;

    // Private helpers (still in trait!)
    fn shared_helper(&self, ...) -> ...;
    fn internal_utility(&self, ...) -> ...;
}
```

### Step 5: Move ALL Methods to Single Trait Impl Block

```rust
impl I<Name>Protocol for <Type> {
    fn public_method(&self, ...) { /* impl */ }
    async fn async_api(&self, ...) { /* impl */ }
    fn shared_helper(&self, ...) { /* impl */ }
    fn internal_utility(&self, ...) { /* impl */ }
}

impl <Type> {
    pub fn new(...) -> Self { /* constructor only */ }
}
```

### Step 6: Register Trait Module in Shared Crate

Add the new trait file to `mod.rs` so it's exported:

```rust
// In crates/shared/src/<layer>/mod.rs
pub mod contract_<name>_protocol;  // add this line
```

### Step 7: Verify Compilation

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Verification Checklist

- [ ] ONE `impl Trait for Type` block — no other method impl blocks
- [ ] ONLY `new()` in separate inherent `impl Type` block
- [ ] ALL fn from impl file are in trait (same name, no prefixes)
- [ ] Private helpers and utilities also have trait signatures
- [ ] All trait methods have `&self`
- [ ] Generic bounds include `where Self: Sized`
- [ ] All trait methods have implementations
- [ ] Trait module registered in shared crate's `mod.rs`
- [ ] Shared logic extracted into one consolidated helper (no duplication)
- [ ] `cargo check` passes

## File Locations

```
# Capabilities (Protocol)
crates/shared/src/<layer>/contract_<name>_protocol.rs    # Trait (ALL fn — public + private)
crates/<crate>/src/capabilities_<name>.rs                 # Impl (only new() in inherent impl)

# Infrastructure (Port)
crates/shared/src/<layer>/contract_<name>_port.rs        # Trait (ALL fn — public + private)
crates/<crate>/src/infrastructure_<name>.rs               # Impl (only new() in inherent impl)

# Agents (Aggregate)
crates/shared/src/<layer>/contract_<name>_aggregate.rs   # Trait (ALL fn — public + private)
crates/<crate>/src/agent_<name>.rs                        # Impl (only new() in inherent impl)
```

## Quick Commands

```bash
# Count fn in trait impl block (should include ALL methods)
grep -c "^    fn \|^    async fn " crates/import-rules/src/capabilities_<name>.rs

# Verify only new() in inherent impl
grep -A5 "^impl <Name>Checker {" crates/import-rules/src/capabilities_<name>.rs

# Check trait has all methods (no "not a member of trait" errors)
cargo check -p <crate-name> 2>&1 | grep "not a member of trait"

# Check no separate impl blocks with methods
grep "^impl [A-Z]" crates/import-rules/src/capabilities_<name>.rs | grep -v "I<Name>"
```

## Common Mistakes (AVOID)

- ❌ Creating separate `impl Type { fn helper(...) }` for private helpers — they go in trait impl
- ❌ Renaming helpers with `do_` or `pure_` prefixes — keep original names
- ❌ Forgetting to register the trait in shared crate's `mod.rs`
- ❌ Having methods split across multiple `impl` blocks — everything in ONE trait impl
- ❌ Leaving shared logic duplicated across two public methods — extract into one helper

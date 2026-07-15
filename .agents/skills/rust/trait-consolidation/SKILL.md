---
name: trait-consolidation-rust
version: 1.0.0
category: refactoring
tags: [aes, trait, protocol, port, consolidation, rust, interface]
triggers:
  - "consolidate trait methods"
  - "add fn to trait protocol"
  - "trait consolidation"
  - "merge fn signatures into trait"
dependencies: []
related:
  - fix-cross-import
  - enforce-1-class-per-file
---
# trait-consolidation-rust

## Purpose

Consolidate ALL function signatures from a capability/infrastructure/agent implementation file into its corresponding trait. Makes every method part of the contract for DI and verifiability. **1:1 fn-to-trait matching** — every fn in impl MUST have exactly one corresponding fn in the trait.

## Rules

- **1:1 fn-to-trait matching**: Every fn in impl file MUST have exactly one corresponding fn in the trait
- Trait MUST contain ALL fn signatures from the impl file
- Instance methods keep original name in trait
- Free functions get `pure_` prefix in trait
- All trait methods MUST have `&self` parameter
- Generic bounds need `where Self: Sized`
- Rename inherent methods with `do_` prefix if they conflict with trait method names

## Contract Rules

- **If contract exists**: Update the existing contract to include all fn signatures from impl file
- **If contract doesn't exist**: Create a new contract file following naming convention
- **If contract needs replacement**: Replace old contract with unified contract (merge methods from multiple traits)
- Follow the naming convention below for trait files and names

## Naming Convention


| Layer              | Trait File                     | Trait Name         | Impl File                  |
| -------------------- | -------------------------------- | -------------------- | ---------------------------- |
| **Capabilities**   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  | `capabilities_<name>.rs`   |
| **Infrastructure** | `contract_<name>_port.rs`      | `I<Name>Port`      | `infrastructure_<name>.rs` |
| **Agents**         | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` | `agent_<name>.rs`          |

## When to Use

- Capability/Infra/Agent impl file has fn methods NOT in the trait
- You want to make all methods part of the contract interface
- Refactoring and want to document new functionality in trait first
- Need to create a new contract for an impl file that doesn't have one yet
- Need to replace/update an existing contract with merged methods

## The Pattern

### Trait File (`contract_<name>_protocol.rs`, `contract_<name>_port.rs`, or `contract_<name>_aggregate.rs`)

```rust
pub trait I<Name>Protocol: Send + Sync {    // Protocol for capabilities
pub trait I<Name>Port: Send + Sync {        // Port for infrastructure
pub trait I<Name>Aggregate: Send + Sync {   // Aggregate for agents

    // Instance methods — keep original name
    fn instance_method(&self, ...) -> ...;

    // Free functions — add pure_ prefix
    fn pure_free_function(&self, ...) -> ...;

    // Generic functions — add where Self: Sized
    fn pure_generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...;
}
```

### Implementation File (`capabilities_<name>.rs`, `infrastructure_<name>.rs`, or `agent_<name>.rs`)

```rust
impl I<Name>Protocol for <CapabilityType> {   // or I<Name>Port / I<Name>Aggregate
    // Instance method wrapper — calls renamed inherent method
    fn instance_method(&self, ...) {
        self.do_instance_method(...)
    }

    // Free function wrapper — calls free function directly
    fn pure_free_function(&self, ...) -> ... {
        free_function(...)
    }

    // Generic function wrapper
    fn pure_generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...,
    {
        generic_fn(...)
    }
}

// Inherent methods — rename with do_ prefix if conflict
impl <Type> {
    pub(crate) fn do_instance_method(&self, ...) { ... }
}

// Free functions — keep original name (no changes needed)
fn free_function(...) -> ... { ... }
```

## Step-by-Step Process

### Step 1: Count fn in Impl File

```bash
grep -c "^fn \|^pub(crate) fn " path/to/capabilities_or_infrastructure_or_agent_file.rs
```

### Step 2: Count fn in Trait File

```bash
grep -c "^    fn \|^    async fn " path/to/contract_<name>_protocol_or_port_or_aggregate.rs
```

### Step 3: Extract Signatures from Impl File

Read each fn and extract its signature (without body):

```rust
// From impl file
fn check_dummy_imports(&self, file: &str, ...) -> ...;
fn filepath_or_default(result: Result<...>) -> ...;
fn python_class_inherits(line: &str, agg_type: &str) -> bool;
```

### Step 4: Add to Trait File

For each fn:

- Instance methods: add `&self` if missing, keep name
- Free functions: add `pure_` prefix and `&self` parameter
- Generic functions: add `where Self: Sized` and `&self`

```rust
pub trait I<Name>Protocol: Send + Sync {    // or I<Name>Port / I<Name>Aggregate
    fn check_dummy_imports(&self, ...) -> ...;
    fn pure_filepath_or_default(&self, ...) -> ...;
    fn pure_python_class_inherits(&self, ...) -> bool;
}
```

### Step 5: Implement in Impl File

Add to `impl I<Name>Protocol/Port/Aggregate for <Type>` block:

```rust
impl IDummyImportCheckerProtocol for DummyImportChecker {   // or I<Name>Port / I<Name>Aggregate
    fn check_dummy_imports(&self, ...) {
        self.do_check_dummy_imports(...)
    }

    fn pure_filepath_or_default(&self, ...) -> ... {
        filepath_or_default(...)
    }
}
```

### Step 6: Rename Conflicting Inherent Methods

If inherent method has same name as trait method:

```rust
impl <Type> {
    pub(crate) fn do_check_dummy_imports(&self, ...) { ... }  // renamed with do_ prefix
}
```

### Step 7: Verify Compilation

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Verification Checklist

- [ ]  All fn from impl file are in trait
- [ ]  All trait methods have `&self`
- [ ]  Free functions use `pure_` prefix
- [ ]  Generic bounds include `where Self: Sized`
- [ ]  All trait methods have implementations
- [ ]  Inherent methods renamed with `do_` if conflict
- [ ]  `cargo check` passes

## File Locations

```
# Capabilities (Protocol)
crates/shared/src/<layer>/contract_<name>_protocol.rs    # Trait (ALL fn)
crates/<crate>/src/capabilities_<name>.rs                 # Impl (trait methods + inherent)

# Infrastructure (Port)
crates/shared/src/<layer>/contract_<name>_port.rs        # Trait (ALL fn)
crates/<crate>/src/infrastructure_<name>.rs               # Impl (trait methods + inherent)

# Agents (Aggregate)
crates/shared/src/<layer>/contract_<name>_aggregate.rs   # Trait (ALL fn)
crates/<crate>/src/agent_<name>.rs                        # Impl (trait methods + inherent)
```

## Quick Commands

```bash
# Count fn in impl file (capabilities, infrastructure, or agent)
grep -c "^fn \|^pub(crate) fn " crates/import-rules/src/capabilities_dummy_import_checker.rs
grep -c "^fn \|^pub(crate) fn " crates/external-lint/src/infrastructure_rs_clippy_adapter.rs
grep -c "^fn \|^pub(crate) fn " crates/import-rules/src/agent_import_orchestrator.rs

# Count fn in trait file (Protocol, Port, or Aggregate)
grep -c "^    fn \|^    async fn " crates/shared/src/import-rules/contract_dummy_import_checker_protocol.rs
grep -c "^    fn \|^    async fn " crates/shared/src/external-lint/contract_external_lint_port.rs
grep -c "^    fn \|^    async fn " crates/shared/src/import-rules/contract_import_analyzer_port.rs

# Check missing implementations
cargo check -p <crate-name> 2>&1 | grep "not implemented"
```

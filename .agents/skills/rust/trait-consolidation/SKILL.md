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
- All trait methods keep the **same name** as their impl counterparts (no `do_` or `pure_` prefixes)
- All trait methods MUST have `&self` parameter
- Generic bounds need `where Self: Sized`
- If impl has a separate inherent `impl Type` block with conflicting methods, split into trait method + inherent method with same name (trait calls inherent directly)

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

    // All methods — same name as in impl, no prefixes
    fn instance_method(&self, ...) -> ...;
    fn another_method(&self, ...) -> ...;

    // Generic functions — add where Self: Sized
    fn generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...;
}
```

### Implementation File (`capabilities_<name>.rs`, `infrastructure_<name>.rs`, or `agent_<name>.rs`)

```rust
impl I<Name>Protocol for <CapabilityType> {   // or I<Name>Port / I<Name>Aggregate
    // All methods — same name as trait, implementation directly here
    fn instance_method(&self, ...) {
        // actual implementation logic
    }

    fn another_method(&self, ...) {
        // actual implementation logic
    }

    fn generic_fn<F, G>(&self, ...) -> ...
    where
        Self: Sized,
        F: Fn(...) -> ...,
        G: Fn(...) -> ...,
    {
        // actual implementation logic
    }
}
```

**Key**: Methods in trait impl blocks keep their **original names** — no `do_` or `pure_` prefixes. The trait and impl share the same name.

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

- All methods: add `&self` if missing, keep **original name** (no prefixes)
- Generic functions: add `where Self: Sized`

```rust
pub trait I<Name>Protocol: Send + Sync {    // or I<Name>Port / I<Name>Aggregate
    fn check_dummy_imports(&self, ...) -> ...;
    fn filepath_or_default(&self, ...) -> ...;
    fn python_class_inherits(&self, ...) -> bool;
}
```

### Step 5: Implement in Impl File

Add to `impl I<Name>Protocol/Port/Aggregate for <Type>` block:

```rust
impl IDummyImportCheckerProtocol for DummyImportChecker {   // or I<Name>Port / I<Name>Aggregate
    fn check_dummy_imports(&self, ...) {
        // actual implementation logic
    }

    fn filepath_or_default(&self, ...) -> ... {
        // actual implementation logic
    }
}
```

### Step 7: Verify Compilation

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Verification Checklist

- [ ]  All fn from impl file are in trait (same name, no prefixes)
- [ ]  All trait methods have `&self`
- [ ]  Generic bounds include `where Self: Sized`
- [ ]  All trait methods have implementations
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

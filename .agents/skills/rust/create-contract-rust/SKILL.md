---
name: create-contract-rust
description: "Create and validate contract layer files (contract_*.rs) — port, protocol, aggregate traits that decouple layers without implementing any logic."
version: 1.0.0
category: refactoring
tags:
  [
    rust,
    aes,
    contract,
    protocol,
    port,
    aggregate,
    interface,
    shared,
    structure,
  ]
triggers:
  - "create contract rust"
  - "add contract rust"
  - "create trait rust"
  - "create port rust"
  - "create protocol rust"
  - "create aggregate rust"
  - "missing contract rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-agent-rust
  - create-taxonomy-rust
  - trait-consolidation-rust
  - enforce-1-struct-per-file-rust
  - create-missing-protocols-rust
---

# create-contract-rust

## Purpose

Create and validate Rust **contract layer** files in `crates/shared/src/<domain>/`. Contracts are pure trait definitions — they decouple layers by defining interfaces without implementing any logic. Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

**This skill consolidates rules from:** `create-missing-protocols`, `trait-consolidation`, and `module_logic_validator` — applied specifically to the contract layer.

## Rules

### The Fundamental Question

> **"Is this a pure trait definition or does it contain implementation?"**

- **Contract (trait only)** → **MUST be in shared/taxonomy as `contract_*.rs`**. No impl blocks, no logic.
- **Implementor (struct + trait impl)** → belongs in layer file (`capabilities_*.rs`, `infrastructure_*.rs`, `agent_*.rs`).

### Contract Layer Structure

```
crates/shared/src/<domain>/
├── mod.rs                    # Module exports for this domain
├── contract_*_port.rs        # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.rs    # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.rs   # Composition facades — implemented by Agents
```

### Three Suffix Types and Their Roles

| Suffix | Role | Implemented By | Example |
|--------|------|-----------------|---------|
| `_port` | Outbound interface | Infrastructure layer | `contract_system_port.rs`, `contract_import_parser_port.rs` |
| `_protocol` | Inbound interface | Capabilities layer | `contract_import_forbidden_protocol.rs`, `contract_naming_checker_protocol.rs` |
| `_aggregate` | Composition facade | Agent layer | `contract_import_runner_aggregate.rs`, `contract_tui_aggregate.rs` |

**CRITICAL:** These suffixes are **strict** — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Naming Convention

`contract_<concept_word(s)>_<role_suffix>.rs`

| Concept | File Name | Trait Name | Implemented By |
|---|---|---|---|
| System operations | `contract_system_port.rs` | `IFileSystemPort` | Infrastructure adapters |
| Forbidden import checking | `contract_import_forbidden_protocol.rs` | `IImportForbiddenProtocol` | Capabilities checkers |
| Import runner orchestration | `contract_import_runner_aggregate.rs` | `IImportRunnerAggregate` | Agent orchestrators |

### Import Restrictions (AES201)

Contract files must remain **completely pure**:

| Can Import From | Cannot Import From |
| --- | --- |
| `taxonomy_*` files | capabilities, infrastructure, agents, surfaces |
| Other `contract_*` files | Any layer files (*.rs without contract_ or taxonomy_ prefix) |

**Contracts define interfaces only — zero implementation logic.**

### Trait Structure

Every contract trait follows the 3-Block Pattern structure (even though it's a separate file):

```rust
// contract_system_port.rs
pub trait IFileSystemPort: Send + Sync {
    // Async methods (must be Sized for object safety)
    async fn read_file(&self, path: &str) -> Result<String, Self::Error>;

    // Generic methods (must be Sized for object safety)
    fn glob_files<F>(&self, pattern: &str, callback: F) -> usize
    where
        Self: Sized,
        F: FnMut(&str);

    // Associated types for error handling
    type Error;
}
```

## Detection Patterns

### BAD: Contract Contains Implementation

```rust
// BAD: Contract file contains impl block with logic
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &str) -> Result<String, Self::Error>;
}

impl IFileSystemPort for FileAdapter {  // ← IMPLEMENTATION belongs in infrastructure_*.rs
    async fn read_file(&self, path: &str) -> String {
        std::fs::read_to_string(path).unwrap()  // ← I/O in contract!
    }
}
```

### BAD: Contract Imports Non-Taxonomy Types

```rust
// BAD: Contract imports capability types
use crate::capabilities_my_checker::MyChecker;  // ← FORBIDDEN

pub trait IMyProtocol: Send + Sync {
    fn check(&self, checker: &MyChecker);  // ← Should use taxonomy types only
}
```

### GOOD: Pure Contract Trait

```rust
// contract_system_port.rs — pure trait definition
use shared::common::taxonomy_path_vo::FilePath;

pub trait IFileSystemPort: Send + Sync {
    type Error;

    async fn read_file(&self, path: &FilePath) -> Result<String, Self::Error>;
    async fn write_file(&self, path: &FilePath, content: &str) -> Result<(), Self::Error>;
    fn glob_files<F>(&self, pattern: &str, callback: F) -> usize
    where
        Self: Sized,
        F: FnMut(&str);
}

// Implementation belongs in infrastructure_adapter.rs — NOT here
```

## Workflow

### Step 1: Determine the Contract Role

Ask: **"Which layer will implement this interface?"**

- Infrastructure implements → `_port` (outbound)
- Capabilities implements → `_protocol` (inbound)
- Agent implements → `_aggregate` (composition facade)

### Step 2: Identify Public Methods

List all methods that other layers need to call. These become trait method signatures.

```bash
# Find methods used across layers
grep -rn "fn " crates/<crate>/src/ | grep -v "shared/" | head -50
```

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.rs` in the appropriate domain under `crates/shared/src/<domain>/`.

**Rules:**
- Trait must include `Send + Sync` bounds
- Generic methods need `where Self: Sized`
- Use associated error types (`type Error`) for flexible error handling
- Import only `taxonomy_*` and other `contract_*` files

```rust
// contract_<name>_<suffix>.rs
use shared::common::taxonomy_path_vo::FilePath;

pub trait I<Name><Suffix>: Send + Sync {
    type Error;

    fn public_method(&self, input: &FilePath) -> Result<String, Self::Error>;
    async fn async_method(&self, id: u32) -> Option<Data>
    where
        Self: Sized;
}
```

### Step 4: Register Module

Update the domain's `mod.rs` to export the new contract module:

```rust
// shared/src/<domain>/mod.rs
pub mod contract_<name>_<suffix>;  // ← Add this line
pub mod taxonomy_<name>_vo;
```

### Step 5: Implement in Layer File

The implementing layer file imports and implements the trait:

```rust
// Infrastructure layer implements _port
use shared::<domain>::contract_system_port::IFileSystemPort;

pub struct FileAdapter {
    // ...
}

impl IFileSystemPort for FileAdapter {
    type Error = std::io::Error;

    async fn read_file(&self, path: &FilePath) -> Result<String, Self::Error> {
        std::fs::read_to_string(path.value()).await
    }
}
```

### Step 6: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only trait definitions** — no impl blocks, no implementation logic.
- [ ] Trait includes `Send + Sync` bounds.
- [ ] Generic trait methods include `where Self: Sized`.
- [ ] Contract imports only `taxonomy_*` and other `contract_*` files.
- [ ] No capabilities, infrastructure, agents, or surface imports in contract files.
- [ ] Domain's `mod.rs` exports new contract module — `pub mod contract_<name>_<suffix>`.
- [ ] Layer file implements the trait (infrastructure for _port, capabilities for _protocol, agent for _aggregate).
- [ ] `cargo check -p shared` passes without warnings or errors.

## Quick Commands

```bash
# Find contracts without implementations
grep -rn "^pub trait" crates/shared/src/*/contract_*.rs | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    trait=$(echo "$line" | grep -oP 'pub trait \K[a-zA-Z_]+')
    grep -q "impl.*for.*Adapter\|impl.*for.*Checker\|impl.*for.*Orchestrator" crates/<crate>/src/*.rs || echo "UNIMPLEMENTED: $trait in $file"
done

# Check for forbidden imports in contract files
grep -n "use crate::capabilities_\|use crate::infrastructure_\|use crate::agent_" crates/shared/src/*/contract_*.rs

# Find contracts that don't have Send + Sync bounds
grep -rn "^pub trait" crates/shared/src/*/contract_*.rs | grep -v ": Send\|: Sync"

# Verify contract module exports are registered
grep -n "^pub mod contract_" crates/shared/src/*/mod.rs

# Check for unregistered contract files (exist on disk but not in mod.rs)
ls crates/shared/src/<domain>/contract_*.rs | while read f; do
    basename=$(basename "$f" .rs)
    grep -q "pub mod $basename" crates/shared/src/<domain>/mod.rs || echo "UNREGISTERED: $basename"
done

# Check for object safety violations
cargo check -p <crate-name> 2>&1 | grep "cannot be made into an object"

# Find contracts without implementations
grep -rn "^pub trait" crates/shared/src/*/contract_*.rs | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    trait=$(echo "$line" | grep -oP 'pub trait \K[a-zA-Z_]+')
    grep -q "impl.*for.*Adapter\|impl.*for.*Checker\|impl.*for.*Orchestrator" crates/<crate>/src/*.rs || echo "UNIMPLEMENTED: $trait in $file"
done
```

## Trait Structure Patterns (from trait-consolidation)

**Contract Trait Definition Rules:**

- Traits MUST include `Send + Sync` bounds for cross-thread safety
- Async methods must be `Sized` for object safety with async
- Generic methods need `where Self: Sized` bound
- Use associated error types (`type Error`) for flexible error handling

```rust
// contract_system_port.rs — complete trait structure example
pub trait IFileSystemPort: Send + Sync {
    // Associated type for error handling
    type Error;

    // Async methods (must be Sized for object safety)
    async fn read_file(&self, path: &str) -> Result<String, Self::Error>;
    async fn write_file(&self, path: &str, content: &str) -> Result<(), Self::Error>;

    // Generic methods (must be Sized for object safety)
    fn glob_files<F>(&self, pattern: &str, callback: F) -> usize
    where
        Self: Sized,
        F: FnMut(&str);

    // Regular methods (no extra bounds needed)
    fn list_files(&self, dir: &str) -> Vec<String>;
}
```

### Naming Convention (from fix-naming)

**Contract File Naming:**

| Concept | File Name | Trait Name | Implemented By |
|---|---|---|---|
| System operations | `contract_system_port.rs` | `IFileSystemPort` | Infrastructure adapters |
| Forbidden import checking | `contract_import_forbidden_protocol.rs` | `IImportForbiddenProtocol` | Capabilities checkers |
| Import runner orchestration | `contract_import_runner_aggregate.rs` | `IImportRunnerAggregate` | Agent orchestrators |

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY trait definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed. No other suffixes.
- ❌ **Forgetting to register new contract modules in mod.rs**: Every `contract_*.rs` file must have a corresponding `pub mod` in the domain's `mod.rs`.
- ❌ **Missing `Send + Sync` bounds on traits**: All contract traits MUST implement `Send + Sync` for cross-thread safety.
- ❌ **Forgetting `where Self: Sized` on generic methods**: This breaks `dyn Trait` usage for the rest of the trait.
- ❌ **Placing impl blocks in contract files**: Even thin wrapper impls belong in layer files, not contracts.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.

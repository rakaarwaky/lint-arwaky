
---
name: create-contract-rust
description: "Create and validate Rust contract layer files (contract_*.rs) — pure trait definitions that decouple layers without leaking implementation details."
version: 1.1.0
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
  - "fix god interface rust"
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

Create and validate Rust **contract layer** files in `crates/shared/src/<domain>/`. Contracts are **pure trait definitions** — they decouple layers by defining the "WHAT" (public promise) without implementing any "HOW" (logic) or leaking internal stepping stones. 

Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

## Rules

### The Fundamental Question (The Golden Rule)

> **"Is this a public promise needed by an outer layer, or just an internal stepping stone?"**
> 
> - **Public Promise (WHAT)**: Outer layers need to call this, or it requires polymorphism (multiple implementations). → **Put in Contract (`contract_*.rs`)**.
> - **Internal Stepping Stone (HOW)**: Helper methods, highly specific algorithms (e.g., specific regex), or logic that only serves other methods in the same struct. → **Keep as Private Helper in Implementation Struct**. **NEVER put this in the contract.**

### Contract Layer Structure

```text
crates/shared/src/<domain>/
├── mod.rs                          # Module exports for this domain
├── contract_*_port.rs              # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.rs          # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.rs         # Composition facades — implemented by Agents
```

**CRITICAL:** These suffixes are strict — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Three Suffix Types and Their Roles

| Suffix | Role | Implemented By | Example |
|--------|------|----------------|---------|
| `_port` | Outbound interface (needs I/O, external systems) | Infrastructure layer | `contract_system_port.rs`, `contract_import_parser_port.rs` |
| `_protocol` | Inbound interface (pure business logic, validation) | Capabilities layer | `contract_import_forbidden_protocol.rs`, `contract_naming_checker_protocol.rs` |
| `_aggregate` | Composition facade (orchestrates multiple ports/protocols) | Agent layer | `contract_import_runner_aggregate.rs`, `contract_tui_aggregate.rs` |

### Naming Convention

Pattern: `contract_<concept_word(s)>_<role_suffix>.rs`

| Concept | File Name | Trait Name | Implemented By |
|---------|-----------|------------|----------------|
| System operations | `contract_system_port.rs` | `IFileSystemPort` | Infrastructure adapters |
| Forbidden import checking | `contract_import_forbidden_protocol.rs` | `IImportForbiddenProtocol` | Capabilities checkers |
| Import runner orchestration | `contract_import_runner_aggregate.rs` | `IImportRunnerAggregate` | Agent orchestrators |

### Import Restrictions (AES201)

Contract files must remain completely pure. 

| Can Import From | Cannot Import From |
|-----------------|--------------------|
| `taxonomy_*` files (VOs, constants, utilities) | `capabilities_*`, `infrastructure_*`, `agent_*`, `surface_*` |
| Other `contract_*` files | Any layer files (`*.rs` without `contract_` or `taxonomy_` prefix) |

Contracts define interfaces only — **zero implementation logic**.

### Trait Structure Rules

Every contract trait must follow these structural rules for object safety and cross-thread compatibility:

1. **Bounds**: Traits MUST include `Send + Sync` bounds.
2. **Generics**: Generic methods MUST include `where Self: Sized` to preserve object safety for the rest of the trait.
3. **Errors**: Use associated types (`type Error;`) for flexible, implementation-specific error handling.
4. **No Helpers**: Do NOT include private helper signatures or highly specific algorithmic steps in the trait.
5. **No Primitives**: ALL primitive types are FORBIDDEN in contract trait method signatures:
   - `&str` → use `FilePath`, `SymbolName`, or domain-specific VO
   - `String` → use domain-specific VO
   - `bool` → use `BooleanVO`
   - `i32`, `i64`, `u32`, `u64`, `f32`, `f64`, `usize`, `isize` → use domain-specific VO (`LineNumber`, `Count`, `Score`, etc.)
   - `Vec<String>` → use `PatternList` or domain-specific list VO
   - `Option<String>` → use domain-specific optional VO

```rust
// contract_system_port.rs — Complete trait structure example
use shared::common::taxonomy_path_vo::FilePath;

pub trait IFileSystemPort: Send + Sync {
    // 1. Associated type for flexible error handling
    type Error;

    // 2. Async methods (implicitly Sized, but good to be explicit if mixing with generics)
    async fn read_file(&self, path: &FilePath) -> Result<ContentString, Self::Error>;
    async fn write_file(&self, path: &FilePath, content: &ContentString) -> Result<(), Self::Error>;

    // 3. Generic methods (MUST have `where Self: Sized` for object safety)
    fn glob_files<F>(&self, pattern: &PatternList, callback: F) -> Count
    where
        Self: Sized,
        F: FnMut(&FilePath);

    // 4. Regular methods (no extra bounds needed)
    fn list_files(&self, dir: &FilePath) -> FilePathList;
}
// NOTE: Implementation belongs in infrastructure_*.rs — NOT here.
```

## Detection Patterns

### BAD: Contract Contains Implementation

```rust
// BAD: Contract file contains impl block with logic
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &str) -> Result<String, Self::Error>;
}

impl IFileSystemPort for FileAdapter {  // ← IMPLEMENTATION belongs in infrastructure_*.rs
    async fn read_file(&self, path: &str) -> Result<String, Self::Error> {
        std::fs::read_to_string(path).map_err(Self::Error::from)  // ← I/O in contract file!
    }
}
```

### BAD: Contract Imports Non-Taxonomy Types

```rust
// BAD: Contract imports capability/infrastructure types
use crate::capabilities_my_checker::MyChecker;  // ← FORBIDDEN

pub trait IMyProtocol: Send + Sync {
    fn check(&self, checker: &MyChecker);  // ← Should use taxonomy VOs only
}
```

### BAD: Leaking Implementation Details (God Interface)

```rust
// BAD: Contract contains highly specific helper methods that force all implementors to write boilerplate
pub trait IFileParserPort: Send + Sync {
    type Error;
    
    // GOOD: Public promise
    fn parse_file(&self, path: &FilePath) -> Result<ParsedData, Self::Error>;
    
    // BAD: LEAKING IMPLEMENTATION DETAIL. 
    // A Python parser doesn't need Rust regex. This belongs in the Rust parser struct as a private helper.
    fn extract_rust_specific_regex(&self, content: &str) -> Vec<String>; 
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
```

## Workflow

### Step 1: Determine the Contract Role
Ask: **"Which layer will implement this interface?"**
- Infrastructure implements → `_port` (outbound)
- Capabilities implements → `_protocol` (inbound)
- Agent implements → `_aggregate` (composition facade)

### Step 2: Identify Public Methods (The Filter)
List all methods. Apply the Golden Rule:
- Does an outer layer call this? → **Keep in Contract**.
- Is it just a stepping stone / internal helper? → **Discard from Contract** (it will be a private method in the impl struct).

### Step 3: Create Contract File
Create `contract_<concept>_<suffix>.rs` in the appropriate domain under `crates/shared/src/<domain>/`.
- Ensure `Send + Sync` bounds.
- Add `where Self: Sized` to generic methods.
- Use `type Error;`.
- Import **only** `taxonomy_*` and other `contract_*` files.

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
use shared::common::taxonomy_path_vo::FilePath;

pub struct FileAdapter { /* ... */ }

impl IFileSystemPort for FileAdapter {
    type Error = std::io::Error;

    async fn read_file(&self, path: &FilePath) -> Result<String, Self::Error> {
        tokio::fs::read_to_string(path.value()).await
    }
    
    // Private helpers stay in `impl FileAdapter`, NOT in the trait above.
}
```

### Step 6: Verify
Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only** trait definitions — no `impl` blocks, no implementation logic.
- [ ] **No leaking implementation details**: Contract does not contain highly specific helper methods (e.g., specific regex, internal parsing steps) that belong in the impl struct.
- [ ] Trait includes `Send + Sync` bounds.
- [ ] Generic trait methods include `where Self: Sized`.
- [ ] Contract imports **only** `taxonomy_*` and other `contract_*` files.
- [ ] No `capabilities_*`, `infrastructure_*`, `agent_*`, or `surface_*` imports in contract files.
- [ ] Domain's `mod.rs` exports new contract module (`pub mod contract_<name>_<suffix>;`).
- [ ] Layer file correctly implements the trait.
- [ ] `cargo check -p shared` passes without warnings or errors.

## Quick Commands

```bash
# 1. Find contracts without implementations
grep -rn "^pub trait" crates/shared/src/*/contract_*.rs | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    trait=$(echo "$line" | grep -oP 'pub trait \K[a-zA-Z_]+')
    grep -q "impl.*for.*Adapter\|impl.*for.*Checker\|impl.*for.*Orchestrator" crates/*/src/*.rs || echo "UNIMPLEMENTED: $trait in $file"
done

# 2. Check for forbidden imports in contract files
grep -rn "use crate::capabilities_\|use crate::infrastructure_\|use crate::agent_\|use crate::surface_" crates/shared/src/*/contract_*.rs

# 3. Find contracts that don't have Send + Sync bounds
grep -rn "^pub trait" crates/shared/src/*/contract_*.rs | grep -v ": Send + Sync"

# 4. Detect "God Interfaces" (Traits with > 10 methods — likely leaking helpers)
awk '/^pub trait/ {trait=$0; count=0} /^    (async )?fn / {count++} /^}/ {if(count > 10) print "WARNING: God Interface?", trait, "has", count, "methods"}' crates/shared/src/*/contract_*.rs

# 5. Verify contract module exports are registered
ls crates/shared/src/<domain>/contract_*.rs | while read f; do
    basename=$(basename "$f" .rs)
    grep -q "pub mod $basename" crates/shared/src/<domain>/mod.rs || echo "UNREGISTERED: $basename in mod.rs"
done

# 6. Check for object safety violations
cargo check -p shared 2>&1 | grep "cannot be made into an object"
```

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY trait definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed.
- ❌ **Leaking implementation details (God Interface)**: Do not put private helpers, specific regex logic, or internal stepping stones in the contract. They belong in the implementation struct.
- ❌ **Forgetting to register new contract modules in `mod.rs`**: Every `contract_*.rs` file must have a corresponding `pub mod` in the domain's `mod.rs`.
- ❌ **Missing `Send + Sync` bounds on traits**: All contract traits MUST implement `Send + Sync` for cross-thread safety.
- ❌ **Forgetting `where Self: Sized` on generic methods**: This breaks `dyn Trait` usage for the rest of the trait.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.
```

---
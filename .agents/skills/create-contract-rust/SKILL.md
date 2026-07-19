---
name: create-contract-rust
description: "Create and validate Rust contract layer files in shared domain: pure trait definitions for ports, protocols, and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    contract,
    port,
    protocol,
    aggregate,
    trait,
    shared,
    aes201,
    object-safety,
    di,
    vo,
  ]
triggers:
  - "create contract rust"
  - "add contract rust"
  - "create port rust"
  - "create protocol rust"
  - "create aggregate rust"
  - "fix contract rust"
  - "check contract rust"
  - "audit contract rust"
dependencies: []
related:
  - create-taxonomy-rust
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-agent-rust
  - trait-consolidation-rust
  - fix-primitive-to-vo
---

# create-contract-rust

## Purpose

Create and validate Rust **contract layer** files inside:

```text
crates/shared/src/<domain>/
```

Contracts are pure trait definitions.

They define the **WHAT**:

- public promises,
- stable interfaces,
- polymorphism boundaries,
- DI boundaries.

They MUST NOT define the **HOW**:

- no implementation,
- no private helpers,
- no internal stepping stones,
- no I/O,
- no business logic,
- no layer imports.

Three contract suffixes serve different roles:

- `_port` → implemented by infrastructure
- `_protocol` → implemented by capabilities
- `_aggregate` → implemented by agents

---

## Definition of Done

A contract file is considered valid when:

1. It uses one of the allowed suffixes: `_port`, `_protocol`, `_aggregate`.
2. It contains only trait definitions.
3. It contains no implementation blocks.
4. It contains no default method bodies.
5. It contains no helper methods or internal stepping stones.
6. It imports only taxonomy types, other contract types, and necessary trait machinery.
7. It does not import from capabilities, infrastructure, agent, surface, or root layers.
8. Traits intended for DI are object-safe and `Send + Sync`.
9. Public contract signatures use shared VOs for domain data.
10. New contract modules are registered in `mod.rs`.
11. `cargo check -p shared` passes.

---

## The Fundamental Question

> **“Is this a public promise needed by an outer layer, or just an internal stepping stone?”**

### Public Promise

Put it in the contract when:

- outer layers need to call it,
- it defines a stable public interface,
- it requires polymorphism,
- it is injected via `Arc<dyn Trait>`.

Example:

```rust
fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
```

### Internal Stepping Stone

Keep it as a private helper in the implementation struct when:

- it only supports other methods in the same struct,
- it is algorithm-specific,
- it is implementation-specific,
- it is not part of the public promise.

Example:

```rust
fn extract_rust_trait_name(&self, line: &str) -> Option<SymbolName> {
    // internal helper, not contract material
}
```

Internal stepping stones MUST NOT appear in contract traits.

---

## Contract Layer Structure

```text
crates/shared/src/<domain>/
├── mod.rs
├── contract_*_port.rs
├── contract_*_protocol.rs
├── contract_*_aggregate.rs
├── taxonomy_*_vo.rs
├── taxonomy_*_entity.rs
├── taxonomy_*_error.rs
├── taxonomy_*_event.rs
├── taxonomy_*_constant.rs
└── taxonomy_*_utility.rs
```

Important:

- Contract files define traits only.
- Taxonomy files define data types only.
- Layer files define implementations only.

---

## Three Suffix Types and Their Roles

| Suffix         | Role                                               | Implemented By | Example                                   |
| -------------- | -------------------------------------------------- | -------------- | ----------------------------------------- |
| `_port`      | Outbound interface needing I/O or external systems | Infrastructure | `contract_file_system_port.rs`          |
| `_protocol`  | Inbound interface for pure domain behavior         | Capabilities   | `contract_import_forbidden_protocol.rs` |
| `_aggregate` | Composition facade for orchestration               | Agents         | `contract_import_runner_aggregate.rs`   |

---

## Naming Convention

Pattern:

```text
contract_<concept>_<role_suffix>.rs
```

Examples:

| Concept                     | File Name                                 | Trait Name                   | Implemented By |
| --------------------------- | ----------------------------------------- | ---------------------------- | -------------- |
| File system operations      | `contract_file_system_port.rs`          | `IFileSystemPort`          | Infrastructure |
| Forbidden import checking   | `contract_import_forbidden_protocol.rs` | `IImportForbiddenProtocol` | Capabilities   |
| Import runner orchestration | `contract_import_runner_aggregate.rs`   | `IImportRunnerAggregate`   | Agents         |

Trait names MUST use:

```text
I<Name>Port
I<Name>Protocol
I<Name>Aggregate
```

---

## Purity and Import Restrictions (AES201)

Contract files must remain pure.

### Allowed Imports

| Contract File               | May Import From                                                                |
| --------------------------- | ------------------------------------------------------------------------------ |
| `contract_*_port.rs`      | taxonomy types, other contract types, std marker traits, async_trait if needed |
| `contract_*_protocol.rs`  | taxonomy types, other contract types, std marker traits, async_trait if needed |
| `contract_*_aggregate.rs` | taxonomy types, other contract types, std marker traits, async_trait if needed |

### Forbidden Imports

Contract files MUST NOT import from:

- `capabilities_*`
- `infrastructure_*`
- `agent_*`
- `surface_*`
- root/container modules
- concrete implementation structs

Bad:

```rust
use crate::capabilities_my_checker::MyChecker; // BAD
```

Good:

```rust
use crate::code_analysis::taxonomy_source_vo::SourceContentVO;
use crate::code_analysis::taxonomy_lint_result_vo::LintResult;
```

---

## Trait Structure Rules

### 1. Contracts contain trait definitions only

Good:

```rust
pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
}
```

Bad:

```rust
pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
}

impl IImportForbiddenProtocol for MyChecker {
    // implementation belongs in capabilities layer
}
```

---

### 2. No default method bodies

Default methods are implementation logic.

Bad:

```rust
pub trait ICheckerProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;

    fn check_all(&self, sources: &[SourceContentVO]) -> Vec<LintResult> {
        // BAD: default implementation in contract
        Vec::new()
    }
}
```

If shared behavior is needed, put it in:

- capabilities helper,
- taxonomy utility if pure/domain-agnostic,
- or a separate shared service trait with explicit implementation.

---

### 3. No private helpers or internal stepping stones

Bad:

```rust
pub trait IFileParserPort: Send + Sync {
    fn parse_file(&self, path: &FilePath) -> Result<ParsedData, ParseError>;

    fn extract_rust_specific_regex(&self, content: &FileContent) -> Vec<SymbolName>; // BAD
}
```

The second method is implementation-specific.

It belongs in the implementor:

```rust
impl IFileParserPort for RustFileParser {
    fn parse_file(&self, path: &FilePath) -> Result<ParsedData, ParseError> {
        // ...
    }
}

impl RustFileParser {
    fn extract_rust_specific_regex(&self, content: &FileContent) -> Vec<SymbolName> {
        // private helper
    }
}
```

---

### 4. Traits intended for DI MUST be `Send + Sync`

Good:

```rust
pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
}
```

This is required for typical DI usage:

```rust
Arc<dyn IImportForbiddenProtocol>
```

---

### 5. Contracts MUST be object-safe when used as trait objects

Avoid patterns that break object safety.

Bad for `dyn Trait`:

```rust
pub trait IProcessorProtocol: Send + Sync {
    fn process<T>(&self, item: T);
}
```

If a generic method is truly required, bound it:

```rust
pub trait IProcessorProtocol: Send + Sync {
    fn process<T>(&self, item: T)
    where
        Self: Sized,
        T: Processable;
}
```

But note:

> Methods with `where Self: Sized` cannot be called through `dyn Trait`.

Prefer non-generic object-safe signatures when the trait is injected via DI.

Example:

```rust
pub trait IFileVisitorPort: Send + Sync {
    fn visit_files(
        &self,
        files: &FilePathList,
        visitor: &mut dyn FnMut(&FilePath),
    );
}
```

---

### 6. Async contracts MUST remain dyn-compatible

Async fn in traits is not automatically dyn-compatible.

If the trait will be used as:

```rust
Arc<dyn IFileSystemPort>
```

then use `async_trait` or explicit boxed futures.

Good:

```rust
use async_trait::async_trait;

use crate::file_system::taxonomy_file_content_vo::FileContent;
use crate::file_system::taxonomy_file_path_vo::FilePath;
use crate::file_system::taxonomy_file_read_error::FileReadError;
use crate::file_system::taxonomy_file_write_error::FileWriteError;

#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(
        &self,
        path: &FilePath,
    ) -> Result<FileContent, FileReadError>;

    async fn write_file(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError>;
}
```

If async is not required, do not use async in the contract.

---

### 7. Error strategy

Prefer shared taxonomy error types in contract signatures.

Good:

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

Associated error types may be used when implementation-specific errors are required:

```rust
pub trait IRepositoryPort: Send + Sync {
    type Error;

    fn get(&self, id: &EntityId) -> Result<EntityData, Self::Error>;
}
```

But associated types can make trait-object usage more verbose:

```rust
Arc<dyn IRepositoryPort<Error = RepositoryError>>
```

For most AES contracts, prefer concrete shared error VOs unless flexibility is truly needed.

---

## Primitive and VO Rules

Contract signatures should use shared taxonomy VOs for domain data.

This policy is consistent with capabilities, infrastructure, and taxonomy skills.

### Forbidden for domain values

| Primitive            | Rule                                                                    |
| -------------------- | ----------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract values. Use VO.         |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                    |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                    |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                    |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                    |
| `char`             | Forbidden for domain values. Use VO.                                    |
| `Vec<String>`      | Forbidden for domain collections. Use list VO.                          |
| `Option<String>`   | Forbidden for optional domain values. Use`Option<VO>` or optional VO. |

### Allowed with care

| Type     | Rule                                                                                |
| -------- | ----------------------------------------------------------------------------------- |
| `bool` | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str` | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- file paths,
- symbol names,
- messages,
- line numbers,
- counts,
- severity,
- requests,
- results,
- identifiers,
- policies.

---

## Examples

### GOOD: Port Contract

```rust
// contract_file_system_port.rs

use async_trait::async_trait;

use crate::file_system::taxonomy_file_content_vo::FileContent;
use crate::file_system::taxonomy_file_path_vo::FilePath;
use crate::file_system::taxonomy_file_read_error::FileReadError;
use crate::file_system::taxonomy_file_write_error::FileWriteError;

#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(
        &self,
        path: &FilePath,
    ) -> Result<FileContent, FileReadError>;

    async fn write_file(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError>;
}
```

Implemented by infrastructure.

---

### GOOD: Protocol Contract

```rust
// contract_import_forbidden_protocol.rs

use crate::code_analysis::taxonomy_lint_result_vo::LintResult;
use crate::code_analysis::taxonomy_source_vo::SourceContentVO;
use crate::import_rules::taxonomy_import_rule_list_vo::ImportRuleList;

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(
        &self,
        source: &SourceContentVO,
        rules: &ImportRuleList,
    ) -> Vec<LintResult>;
}
```

Implemented by capabilities.

---

### GOOD: Aggregate Contract

```rust
// contract_import_runner_aggregate.rs

use crate::code_analysis::taxonomy_lint_result_vo::LintResult;
use crate::import_rules::taxonomy_import_scan_request_vo::ImportScanRequest;

pub trait IImportRunnerAggregate: Send + Sync {
    fn run(&self, request: &ImportScanRequest) -> Vec<LintResult>;
}
```

Implemented by agents.

---

### GOOD: Object-Safe Callback Contract

Bad generic version:

```rust
pub trait IFileVisitorPort: Send + Sync {
    fn visit_files<F>(&self, files: &FilePathList, callback: F)
    where
        Self: Sized,
        F: FnMut(&FilePath);
}
```

This method cannot be used through `dyn IFileVisitorPort`.

Better object-safe version:

```rust
pub trait IFileVisitorPort: Send + Sync {
    fn visit_files(
        &self,
        files: &FilePathList,
        callback: &mut dyn FnMut(&FilePath),
    );
}
```

---

## Detection Patterns

### BAD: Contract Contains Implementation

```rust
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}

impl IFileSystemPort for FileAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // BAD: implementation belongs in infrastructure_*.rs
        todo!()
    }
}
```

Fix:

Move implementation to infrastructure layer.

---

### BAD: Contract Imports Non-Taxonomy Types

```rust
use crate::capabilities_my_checker::MyChecker; // BAD

pub trait IMyProtocol: Send + Sync {
    fn check(&self, checker: &MyChecker);
}
```

Fix:

Use taxonomy VOs and contract traits only.

```rust
use crate::code_analysis::taxonomy_source_vo::SourceContentVO;

pub trait IMyProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO);
}
```

---

### BAD: Leaking Implementation Details

```rust
pub trait IFileParserPort: Send + Sync {
    fn parse_file(&self, path: &FilePath) -> Result<ParsedData, ParseError>;

    fn extract_rust_specific_regex(&self, content: &FileContent) -> Vec<SymbolName>; // BAD
}
```

Fix:

Remove internal helper from contract.

```rust
pub trait IFileParserPort: Send + Sync {
    fn parse_file(&self, path: &FilePath) -> Result<ParsedData, ParseError>;
}
```

---

### BAD: Raw Primitives for Domain Values

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &str) -> Result<String, std::io::Error>;
}
```

Fix:

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

---

### BAD: Async Trait Without Dyn Compatibility

```rust
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

If used as `Arc<dyn IFileSystemPort>`, this may not be dyn-compatible.

Fix:

```rust
use async_trait::async_trait;

#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

---

## Workflow

### Step 1: Determine the Contract Role

Ask:

> “Which layer will implement this interface?”

| Implemented By | Suffix         |
| -------------- | -------------- |
| Infrastructure | `_port`      |
| Capabilities   | `_protocol`  |
| Agent          | `_aggregate` |

---

### Step 2: Identify Public Methods

Apply the Golden Rule:

```text
Is this method called by outer layers?
├─ YES → keep in contract
└─ NO → make it a private helper in implementation struct
```

Remove:

- internal parsing steps,
- helper methods,
- algorithm-specific methods,
- implementation stepping stones.

---

### Step 3: Create Contract File

Create:

```text
crates/shared/src/<domain>/contract_<concept>_<suffix>.rs
```

Ensure:

- trait name uses `I<Name>Port`, `I<Name>Protocol`, or `I<Name>Aggregate`,
- trait has `Send + Sync`,
- async methods are dyn-compatible,
- signatures use taxonomy VOs,
- no implementation exists.

---

### Step 4: Register Module

Update:

```text
crates/shared/src/<domain>/mod.rs
```

Example:

```rust
pub mod contract_file_system_port;
pub mod contract_import_forbidden_protocol;
pub mod contract_import_runner_aggregate;
```

---

### Step 5: Implement in Layer File

Infrastructure:

```rust
use shared::file_system::contract_file_system_port::IFileSystemPort;

pub struct FileSystemAdapter;

#[async_trait::async_trait]
impl IFileSystemPort for FileSystemAdapter {
    async fn read_file(
        &self,
        path: &FilePath,
    ) -> Result<FileContent, FileReadError> {
        // infrastructure implementation
    }

    async fn write_file(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError> {
        // infrastructure implementation
    }
}
```

Private helpers stay in the implementor:

```rust
impl FileSystemAdapter {
    fn normalize_path(&self, path: &FilePath) -> FilePath {
        // private helper
    }
}
```

---

### Step 6: Verify

```bash
cargo check -p shared
```

---

## Verification Checklist

- [ ] Contract file uses correct suffix: `_port`, `_protocol`, or `_aggregate`.
- [ ] Contract contains only trait definitions.
- [ ] Contract contains no `impl` blocks.
- [ ] Contract contains no default method bodies.
- [ ] Contract contains no private helper signatures.
- [ ] Contract contains no implementation-specific stepping stones.
- [ ] Trait includes `Send + Sync` bounds.
- [ ] Trait is object-safe when intended for `Arc<dyn Trait>`.
- [ ] Async trait methods are dyn-compatible.
- [ ] Generic methods include `where Self: Sized` when required.
- [ ] Contract imports only taxonomy and contract types.
- [ ] Contract does not import from capabilities.
- [ ] Contract does not import from infrastructure.
- [ ] Contract does not import from agents.
- [ ] Contract does not import from surface.
- [ ] Contract signatures use shared VOs for domain data.
- [ ] Owned primitives are not used for domain values.
- [ ] Numeric primitives are not used for domain values.
- [ ] `bool` is used only for semantic toggles.
- [ ] `&str` is used only for low-level borrowed input when VO is impractical.
- [ ] Error types come from shared taxonomy or explicit associated types.
- [ ] New contract module is registered in `mod.rs`.
- [ ] `cargo check -p shared` passes.

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# List contract traits
rg -n "^\s*pub trait" crates/shared/src/**/contract_*.rs

# Check forbidden imports in contract files
rg -n "^\s*use\s+.*(capabilities_|infrastructure_|agent_|surface_)" crates/shared/src/**/contract_*.rs

# Check possible raw primitive signatures
rg -n "fn .*\b(String|Vec<String>|Option<String>|usize|u32|i32|u64|i64|f32|f64)\b" crates/shared/src/**/contract_*.rs

# Check async fn without async_trait nearby
rg -n "^\s*async fn" crates/shared/src/**/contract_*.rs

# Check traits without Send + Sync on the same line
rg -n "^\s*pub trait I[A-Za-z0-9_]+(?!: Send \+ Sync)" crates/shared/src/**/contract_*.rs

# Check object safety issues
cargo check -p shared 2>&1 | rg "cannot be made into an object"
```

---

### Check Unregistered Contract Files

```bash
for file in crates/shared/src/<domain>/contract_*.rs; do
  basename=$(basename "$file" .rs)

  rg -q "^pub mod $basename;" crates/shared/src/<domain>/mod.rs \
    || echo "UNREGISTERED: $basename"
done
```

---

### Detect Possible God Interfaces

```bash
for file in crates/shared/src/**/contract_*.rs; do
  awk '
    /^\s*pub trait/ { trait_name = $0; count = 0 }
    /^\s*(async )?fn / { count++ }
    /^\s*}/ {
      if (count > 10) {
        print "WARNING: possible god interface in " FILENAME ": " trait_name " has " count " methods"
      }
    }
  ' "$file"
done
```

A trait with many methods is not automatically wrong, but it often indicates leaked helpers or mixed responsibilities.

---

## Common Mistakes

- ❌ Putting implementation logic in contract files.
- ❌ Adding default method bodies to contract traits.
- ❌ Importing concrete layer types into contracts.
- ❌ Importing capabilities, infrastructure, agents, or surface modules into contracts.
- ❌ Using wrong suffix for contract files.
- ❌ Leaking implementation details into contract traits.
- ❌ Putting internal stepping stones into contract traits.
- ❌ Creating god interfaces with too many unrelated methods.
- ❌ Forgetting `Send + Sync` bounds for DI traits.
- ❌ Forgetting object safety for `Arc<dyn Trait>` usage.
- ❌ Using async fn in traits without dyn-compatible handling.
- ❌ Using generic methods without `where Self: Sized`.
- ❌ Using raw `String` for domain values in contract signatures.
- ❌ Using numeric primitives for domain values in contract signatures.
- ❌ Using `Vec<String>` instead of domain list VOs.
- ❌ Using `Option<String>` instead of `Option<VO>` or optional VO.
- ❌ Forgetting to register contract modules in `mod.rs`.
- ❌ Duplicating contract definitions across domains instead of placing shared contracts in `common/`.

```
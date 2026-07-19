---
name: create-infrastructure-rust
description: "Create and validate Rust infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one impl struct per file, port trait contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    infrastructure,
    port,
    structure,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create infrastructure rust"
  - "add infrastructure rust"
  - "fix infrastructure structure rust"
  - "create port rust"
  - "infrastructure missing port rust"
  - "check infrastructure rust"
  - "audit infrastructure rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-infrastructure-structure-rust
  - create-missing-ports-rust
---

# create-infrastructure-rust

## Purpose

Create and validate Rust **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access,
- network calls,
- database access,
- external API calls,
- environment/system integration,
- technical mapping,
- serialization/deserialization,
- error mapping,
- adapter implementation for port traits.

Infrastructure MUST NOT contain business logic.

---

## Definition of Done

An infrastructure file is considered valid when:

1. It contains exactly **ONE implementation struct**.
2. The struct implements exactly **ONE domain port trait** in Block 2.
3. Block 2 contains **ONLY** the port trait implementation.
4. Constructors, std trait impls, and private helpers are placed in Block 3.
5. The file contains **zero business logic**.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via `Arc<dyn Trait>`.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
10. Adapter-specific helpers may remain inside the implementation file.
11. I/O errors are propagated explicitly.
12. `cargo check -p <crate-name>` passes.

---

## Rules

### Layer Boundaries (AES)

#### Infrastructure Layer (`infrastructure_*.rs`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| File I/O (`std::fs`, `File::open`, `read_dir`)      | Business rules                                        |
| Network calls (`reqwest`, `hyper`)                  | Domain logic                                          |
| Database operations (`sqlx`, `rusqlite`)            | Domain calculations                                   |
| External API calls                                  | Domain validation that decides business correctness   |
| Environment/system access via controlled adapter    | Direct import from concrete `agent_*` modules         |
| Serialization/deserialization                       | Direct import from concrete `capabilities_*` modules  |
| Technical mapping (DTO ↔ VO)                        | Locally defined domain data structures                |
| Error mapping from external libraries               | Raw primitives for domain values in public contracts  |
| Port trait implementation                           | Silent error swallowing                               |
| Private helpers supporting the adapter              |                                                       |

Infrastructure may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- port traits
- protocol traits defined in shared, when required by the adapter contract

Infrastructure must not depend on concrete capabilities or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation struct per file

Each infrastructure file contains exactly ONE main implementation struct.

```rust
pub struct FileSystemSourceReader {
    // ...
}
```

Do not define multiple service structs in the same file.

---

#### 2. Only the implementation struct may be defined in the layer file

An infrastructure file may define the implementation struct only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in infrastructure files:

```rust
pub struct CacheEntry {
    key: String,
    value: String,
}
```

Allowed:

```rust
use shared::cache::taxonomy_cache_entry_vo::CacheEntry;
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, adapters, clients, repositories, or ports MUST use trait objects.

```rust
pub struct OrphanFileCache {
    store: Arc<dyn IKeyValueStorePort>,
}
```

Do not use concrete service types:

```rust
pub struct OrphanFileCache {
    store: RedisKeyValueStore, // BAD: concrete dependency
}
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, timeouts, thresholds, etc. should use shared VOs.

```rust
pub struct HttpManifestClient {
    base_url: BaseUrl,
    timeout: TimeoutSeconds,
}
```

Avoid raw primitives for domain values:

```rust
pub struct HttpManifestClient {
    base_url: String,  // BAD
    timeout: u64,      // BAD
}
```

Borrowed primitives such as `&str` may be used internally for low-level boundary code, but public port contracts should expose shared VOs.

---

### Helper vs Utility Decision

The boundary is not only about `&self`.

The real question is:

> Does this function know about adapter-specific or domain-specific rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this struct, or by multiple modules?

---

### When to Keep as Private Helper (Block 3)

Keep the function inside the infrastructure file if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It accesses adapter-specific static/state.
3. It performs adapter-specific mapping.
4. It maps external errors into port-specific errors.
5. It knows infrastructure-specific configuration.
6. It is tightly coupled to this adapter only.
7. It is a factory method such as `new()` or builder method.
8. It is stateless but adapter-specific and only used by this struct.

Example:

```rust
impl FileSystemSourceReader {
    fn map_io_error(&self, path: &FilePath, err: std::io::Error) -> FileReadError {
        FileReadError::io(path.clone(), err)
    }
}
```

This helper is infrastructure-specific and may remain in Block 3.

---

### When to Extract to Utility (`*_utility.rs`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or adapter rules.
5. Reusable: useful for multiple infrastructure/capabilities/modules.

Example:

```rust
// shared/common/taxonomy_string_utility.rs
pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}
```

---

### I/O Blocker (CRITICAL)

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It belongs in infrastructure.

```rust
fn read_file_content(path: &FilePath) -> Result<FileContent, FileReadError> {
    let raw = std::fs::read_to_string(path.value())
        .map_err(|err| FileReadError::io(path.clone(), err))?;

    FileContent::new(raw)
        .map_err(FileReadError::validation)
}
```

Rule:

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT capabilities layer
```

---

## The 3-Block Structure

Every implementation file MUST follow this order:

1. **Block 1 — Struct Definition**
2. **Block 2 — Port Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

---

### Block 1 — Struct Definition

```rust
pub struct FileSystemSourceReader;
```

Or with dependencies:

```rust
pub struct OrphanFileCache {
    store: Arc<dyn IKeyValueStorePort>,
    policy: CachePolicy,
}
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain port trait ONLY.

```rust
impl IFileReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // port implementation
    }
}
```

Do NOT put these in Block 2:

```rust
impl Default for FileSystemSourceReader
impl Clone for FileSystemSourceReader
impl Debug for FileSystemSourceReader
impl Display for FileSystemSourceReader
impl From<...> for FileSystemSourceReader
```

Those belong in Block 3.

---

### Block 3 — Constructors, Std Traits, and Helpers

Block 3 contains:

- `new()`
- builders
- `Default`
- `Clone`
- `Debug`
- `Display`
- other std trait impls
- private helper methods
- adapter-specific associated functions used only by this struct

```rust
impl Default for FileSystemSourceReader {
    fn default() -> Self {
        Self
    }
}

impl FileSystemSourceReader {
    pub fn new() -> Self {
        Self
    }

    fn ensure_parent_dir(&self, path: &FilePath) -> Result<(), FileWriteError> {
        if let Some(parent) = path.parent_directory() {
            std::fs::create_dir_all(parent.value())
                .map_err(|err| FileWriteError::io(path.clone(), err))?;
        }

        Ok(())
    }
}
```

---

### Utility Functions Do Not Belong in Block 3

If a function is:

- stateless,
- pure,
- domain-agnostic,
- and reusable across multiple modules,

then extract it to shared utility.

```rust
use shared::common::taxonomy_path_utility::normalize_relative_path;
```

But if the function is adapter-specific or infrastructure-specific, it may remain in Block 3.

---

## Trait Placement Decision Rule

```text
Trait impl found in an infrastructure file?
  │
  ├─ Is it the domain port? (I<Name>Port)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```

---

## Example: Correct 3-Block Order

```rust
use std::sync::Arc;

use shared::file_system::taxonomy_file_content_vo::FileContent;
use shared::file_system::taxonomy_file_path_vo::FilePath;
use shared::file_system::taxonomy_file_read_error::FileReadError;
use shared::file_system::contract_file_reader_port::IFileReaderPort;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileSystemSourceReader;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(|err| FileReadError::io(path.clone(), err))?;

        FileContent::new(raw)
            .map_err(FileReadError::validation)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for FileSystemSourceReader {
    fn default() -> Self {
        Self
    }
}

impl FileSystemSourceReader {
    pub fn new() -> Self {
        Self
    }

    fn is_not_found(err: &std::io::Error) -> bool {
        err.kind() == std::io::ErrorKind::NotFound
    }
}
```

---

## Port Rules

### AES404 — Infrastructure Must Implement Port Trait

Every infrastructure struct MUST implement a port trait.

```rust
impl IFileReaderPort for FileSystemSourceReader {
    // public contract
}
```

---

### Port file naming

| Layer            | File Pattern          | Trait File                     | Trait Name          |
| ---------------- | --------------------- | ------------------------------ | ------------------- |
| Capabilities     | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`       |
| Agents           | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate`  |

---

### Port content rules

The port trait MUST contain only public contract methods.

Good:

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

Bad:

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;

    fn private_helper(&self); // BAD: helper in port
}
```

---

### Constructors are not port methods

`new()` and builders MUST stay in Block 3.

Bad:

```rust
pub trait IFileReaderPort {
    fn new() -> Self; // BAD
}
```

Good:

```rust
impl FileSystemSourceReader {
    pub fn new() -> Self {
        Self
    }
}
```

---

### Port methods should use shared VOs

Port contracts should avoid raw primitives for domain values.

Bad:

```rust
pub trait IFileReaderPort {
    fn read(&self, path: &str) -> Result<String, std::io::Error>;
}
```

Good:

```rust
pub trait IFileReaderPort {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

---

### Object safety

Port traits intended for `Arc<dyn Trait>` MUST be object-safe.

Avoid generic methods in dyn-compatible traits unless bounded properly.

If a generic method is required, add:

```rust
where
    Self: Sized,
```

or split the trait into:

- object-safe port trait
- generic extension trait

---

## The Fundamental Question

> **“Is this file pure I/O or external system integration?”**

If yes → **`infrastructure_*.rs` + implement port trait**

If no, and it contains business logic → **move to capabilities layer**

---

## Naming Convention

| Layer            | File Pattern          | Trait File                     | Trait Name          |
| ---------------- | --------------------- | ------------------------------ | ------------------- |
| Capabilities     | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`       |
| Agents           | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate`  |

---

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```rust
pub struct FileCache;

impl FileCache {
    pub fn read(&self) {
        // public behavior without port trait
    }
}
```

Fix:

```rust
pub struct FileCache;

impl IFileCachePort for FileCache {
    // contract implementation
}
```

---

### BAD: Business Logic in Infrastructure

```rust
impl OrphanFileCache {
    fn analyze(&self, content: &FileContent) -> bool {
        // BAD: domain logic
        content.value().contains("orphan")
    }
}
```

Fix:

Move analysis to capabilities.

```rust
// capabilities_orphan_analyzer.rs
impl IOrphanAnalyzerProtocol for OrphanAnalyzer {
    fn analyze(&self, content: &FileContent) -> OrphanAnalysisResult {
        // domain logic here
    }
}
```

Infrastructure should only load/save/cache data.

---

### BAD: Data Class Defined in Layer File

```rust
pub struct CacheEntry {
    key: String,
    value: String,
}
```

Fix:

Move to shared taxonomy:

```rust
// shared/cache/taxonomy_cache_entry_vo.rs
pub struct CacheEntry {
    key: CacheKey,
    value: CacheValue,
}
```

Then import it:

```rust
use shared::cache::taxonomy_cache_entry_vo::CacheEntry;
```

---

### BAD: Concrete Service Field

```rust
pub struct OrphanFileCache {
    store: RedisKeyValueStore, // BAD
}
```

Fix:

```rust
pub struct OrphanFileCache {
    store: Arc<dyn IKeyValueStorePort>,
}
```

---

### BAD: Std Trait in Block 2

```rust
pub struct FileCacheAdapter;

impl Default for FileCacheAdapter {
    fn default() -> Self {
        Self
    }
}

impl IFileReaderPort for FileCacheAdapter {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // ...
    }
}
```

Fix:

```rust
pub struct FileCacheAdapter;

impl IFileReaderPort for FileCacheAdapter {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // ...
    }
}

impl Default for FileCacheAdapter {
    fn default() -> Self {
        Self
    }
}
```

---

### GOOD: Implementor with Shared Data and DI

```rust
use std::sync::Arc;

use shared::cache::taxonomy_cache_policy_vo::CachePolicy;
use shared::cache::taxonomy_key_value_store_port::IKeyValueStorePort;
use shared::orphan_detector::taxonomy_orphan_file_cache_port::IOrphanFileCachePort;

pub struct OrphanFileCache {
    store: Arc<dyn IKeyValueStorePort>,
    policy: CachePolicy,
}

impl IOrphanFileCachePort for OrphanFileCache {
    // public port methods only
}
```

---

### GOOD: Correct 3-Block with Std Traits

```rust
pub struct FileCacheAdapter;

impl IFileReaderPort for FileCacheAdapter {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // port implementation
    }
}

impl Default for FileCacheAdapter {
    fn default() -> Self {
        Self
    }
}

impl FileCacheAdapter {
    pub fn new() -> Self {
        Self
    }
}
```

---

## Workflow

### Step 1: Analyze File

Read the file and ask:

> Is this code pure I/O or external system integration?

If yes → keep as infrastructure.

If it contains business logic → move to capabilities.

Examples of business logic that must move out of infrastructure:

- deciding whether a file is orphan
- calculating domain severity
- validating business rules
- computing domain metrics
- interpreting domain meaning from content

Technical mapping is still allowed:

- DTO to VO mapping
- serialization
- deserialization
- external error mapping
- connection handling
- retry mechanics
- transport-level normalization

---

### Step 2: Check for Missing Port

Does the infrastructure struct implement a port trait?

If no:

1. create `contract_<name>_port.rs`
2. define `I<Name>Port`
3. move public method signatures into the port
4. implement the port for the struct

---

### Step 3: Create Port File if Missing

Create port file in the appropriate shared domain folder.

Examples:

| Crate           | Port Path                                                |
| --------------- | --------------------------------------------------------- |
| import-rules    | `crates/shared/src/import_rules/contract_*_port.rs`        |
| code-analysis   | `crates/shared/src/code_analysis/contract_*_port.rs`       |
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_port.rs`     |

Register the module in the relevant `mod.rs`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. struct definition
2. port trait implementation
3. constructors, std traits, private helpers

---

### Step 5: Verify Struct Discipline

Check:

- exactly one implementation struct
- no local domain data structs
- no local enums/VOs/DTOs/constants
- service fields use `Arc<dyn Trait>`
- value fields use shared VOs

---

### Step 6: Verify Helper vs Utility Boundary

For each helper/function:

```text
Does it know adapter-specific or infrastructure-specific details?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.rs
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure:

- no forbidden imports from concrete capabilities
- no forbidden imports from concrete agents
- no business logic
- no domain calculations
- no local domain data definitions

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `unwrap_or_default()`
- fallible port methods return descriptive `Result`
- I/O errors are propagated
- public contracts use shared VOs
- no magic constants for domain values

---

### Step 9: Verify Compilation

Run:

```bash
cargo check -p <crate-name>
```

---

## Verification Checklist

- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation struct.
- [ ] Block 2 contains ONLY the port trait implementation.
- [ ] Block 3 contains constructors, std traits, and private helpers.
- [ ] Infrastructure struct implements a port trait (AES404).
- [ ] Port contains only public contract methods.
- [ ] Private helpers are not declared in the port.
- [ ] Constructors are not declared in the port.
- [ ] Std trait impls are in Block 3.
- [ ] Adapter-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] Generic port methods are object-safe or bounded with `where Self: Sized`.
- [ ] One file contains exactly one implementation struct.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use `Arc<dyn Trait>`.
- [ ] Value/configuration fields use shared VOs.
- [ ] Infrastructure contains zero business logic.
- [ ] No forbidden imports from concrete `capabilities_*`.
- [ ] No forbidden imports from concrete `agent_*`.
- [ ] Port module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

---

## Error Handling Rules

Infrastructure error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```rust
let content = std::fs::read_to_string(path.value()).unwrap_or_default();
```

Forbidden:

```rust
let value = result.ok().unwrap_or_default();
```

Unless the value is genuinely optional and the default is an explicit domain/technical decision.

---

### Rule 2: Fallible port methods should return `Result`

If a port method can fail due to I/O, network, database, parsing, or validation, return `Result<T, E>`.

```rust
fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
```

---

### Rule 3: Use descriptive error types

Prefer custom error types from shared taxonomy.

```rust
pub enum FileReadError {
    Io(FilePath, std::io::Error),
    Validation(ValidationError),
}
```

Avoid losing context:

```rust
.map_err(|e| format!("{:?}", e)) // BAD: context lost
```

---

### Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs.

Lint violations are usually domain/analysis outcomes and belong to capabilities.

Bad:

```rust
fn read(&self, path: &FilePath) -> Vec<LintResult> {
    // BAD: infrastructure deciding lint outcomes
}
```

Good:

```rust
fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
    // infrastructure returns data or error
}
```

Capabilities then decides whether an error becomes a lint violation.

---

### Proper Patterns

```rust
// OK: explicit I/O error propagation
fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
    let raw = std::fs::read_to_string(path.value())
        .map_err(|err| FileReadError::io(path.clone(), err))?;

    FileContent::new(raw)
        .map_err(FileReadError::validation)
}
```

```rust
// OK: optional config with explicit default constant
fn timeout(&self) -> TimeoutSeconds {
    self.config
        .timeout()
        .unwrap_or(DEFAULT_TIMEOUT_SECONDS)
}
```

---

## Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```rust
pub trait IFileWriterPort {
    fn write(&self, path: &str, content: &str) -> Result<(), std::io::Error>;
}
```

Good:

```rust
pub trait IFileWriterPort {
    fn write(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError>;
}
```

### Primitive Policy

| Primitive        | Rule |
| ---------------- | ---- |
| `String`         | Forbidden for domain fields and public contract values. Use VO. |
| `i32`, `i64`     | Forbidden for domain values. Use VO. |
| `u32`, `u64`     | Forbidden for domain values. Use VO. |
| `usize`, `isize` | Forbidden for domain values. Use VO. |
| `f32`, `f64`     | Forbidden for domain values. Use VO. |
| `char`           | Forbidden for domain values. Use VO. |
| `bool`           | Allowed for technical toggles when no richer VO is needed. |
| `&str`           | May be used internally for low-level boundary code, but public contracts should prefer VOs. |

Prefer VOs for:

- file paths
- URLs
- timeouts
- durations
- cache keys
- cache values
- query results
- identifiers
- messages

---

## Magic Constant Extraction Rules

No hardcoded domain literals in infrastructure.

Bad:

```rust
fn save(&self) -> Result<(), FileWriteError> {
    std::fs::File::create("manifest.json")
        .map_err(FileWriteError::io)?;

    Ok(())
}
```

Good:

```rust
use crate::taxonomy_manifest_constant::MANIFEST_FILENAME;

fn save(&self) -> Result<(), FileWriteError> {
    std::fs::File::create(MANIFEST_FILENAME.value())
        .map_err(FileWriteError::io)?;

    Ok(())
}
```

Constants MUST live in:

```text
taxonomy_*_constant.rs
```

Technical defaults should also be named constants or come from configuration VOs.

---

## Import Strategy

When fixing cross-import violations in infrastructure, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```rust
// shared/common/taxonomy_path_utility.rs
pub fn normalize_relative_path(path: &str) -> Option<String> {
    path.strip_prefix("/").map(|s| s.to_string())
}
```

Consumer:

```rust
use shared::common::taxonomy_path_utility::normalize_relative_path;
```

---

### Option B: Dependency Injection via Port Trait

Use when the code needs:

- state,
- collaborators,
- side effects,
- I/O,
- layer-specific implementation.

Example:

```rust
// contract_file_writer_port.rs
pub trait IFileWriterPort: Send + Sync {
    fn write(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError>;
}
```

```rust
// infrastructure_file_writer_adapter.rs
pub struct FileWriterAdapter;

impl IFileWriterPort for FileWriterAdapter {
    fn write(
        &self,
        path: &FilePath,
        content: &FileContent,
    ) -> Result<(), FileWriteError> {
        std::fs::write(path.value(), content.value())
            .map_err(|err| FileWriteError::io(path.clone(), err))
    }
}
```

```rust
// consumer
pub struct ReportPublisher {
    writer: Arc<dyn IFileWriterPort>,
}
```

The consumer depends only on the port trait, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in infrastructure?
  │
  ├─ Does it know adapter-specific or infrastructure-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → keep in infrastructure, not utility
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# List structs in infrastructure files
rg -n "^\s*pub struct" crates/<crate>/src/infrastructure_*.rs

# List port trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Port\s+for" crates/<crate>/src/infrastructure_*.rs

# Check possible business logic keywords
rg "is_orphan|analyze|validate|calculate|compute|business" crates/<crate>/src/infrastructure_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|agent_)" crates/<crate>/src/infrastructure_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/infrastructure_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/infrastructure_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
```

---

### Check Wrong Block Order

```bash
for file in crates/<crate>/src/infrastructure_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }

    /^impl (Default|Clone|Debug|Display)/ {
      if (!std) std = FNR
    }

    /^impl I[A-Z].*Port/ {
      if (!proto) proto = FNR
    }

    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before port (line " proto ")"
      }
    }
  ' "$file"
done
```

---

## Common Mistakes

- ❌ Putting business logic in infrastructure.
- ❌ Putting domain calculations in infrastructure.
- ❌ Putting domain validation in infrastructure.
- ❌ Defining domain data structs in infrastructure files.
- ❌ Using concrete service types as struct fields.
- ❌ Using raw primitives for domain value fields.
- ❌ Exposing raw primitives in public port contracts when a VO exists.
- ❌ Putting private helpers in the port trait.
- ❌ Putting constructors in the port trait.
- ❌ Placing std trait impls before the port trait.
- ❌ Mixing Block 2 and Block 3 responsibilities.
- ❌ Keeping reusable, domain-agnostic utility functions inside Block 3.
- ❌ Extracting adapter-specific helpers to shared utility too early.
- ❌ Creating god ports with too many unrelated methods.
- ❌ Forgetting object safety for `Arc<dyn Trait>` usage.
- ❌ Multiple implementation structs in one file.
- ❌ Direct dependency on concrete capabilities implementations.
- ❌ Direct dependency on concrete agent implementations.
- ❌ Silent error swallowing with `unwrap_or_default()`.
- ❌ Magic constants in infrastructure logic.
- ❌ Infrastructure returning lint results directly instead of returning data/errors to capabilities.
```

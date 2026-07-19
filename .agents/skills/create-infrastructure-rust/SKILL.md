---
name: create-infrastructure-rust
description: "Create and validate infrastructure layer files following AES rules: 3-block structure, one struct per file, port contracts, zero business logic."
version: 1.1.0
category: refactoring
tags:
  [rust, aes, infrastructure, port, structure, aes404, 3-block-structure, di]
triggers:
  - "create infrastructure rust"
  - "add infrastructure rust"
  - "fix infrastructure structure rust"
  - "create port rust"
  - "infrastructure missing port rust"
  - "verify infrastructure rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-capability-structure-rust
  - create-missing-protocols-rust
---

# create-infrastructure-rust

## Purpose

Create and validate Rust **infrastructure layer** files following clean architecture rules. Ensures infrastructure contains zero business logic, implements port traits, follows the 3-Block Structure, and uses DI for all fields.

## Rules

### Layer Boundaries (AES)

**Infrastructure Layer (`infrastructure_*.rs`)**

| Allowed                                  | Forbidden                                |
| ---------------------------------------- | ---------------------------------------- |
| File I/O (`std::fs`, `File::open`)       | Business rules                           |
| Network calls (`reqwest`, `hyper`)       | Domain logic                             |
| Database operations (`sqlx`, `rusqlite`) | Calculations (should be in capabilities) |
| External API calls                       | Direct import from `agent_*`             |
| Trait implementation                     | Direct import from `capabilities_*`      |

### Structural Rules (All Layers)

- **1 file = 1 impl struct** — each infrastructure file contains exactly ONE main impl struct
- **All data classes in shared** — no structs/enums with data may be defined outside shared/taxonomy
- **Fields must use DI** — impl struct fields should be `Arc<dyn Trait>` objects, not concrete types
- **Helper functions stay in layer** — helper methods that support the impl struct remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic free functions (no `&self`) should be extracted to `*_utility.rs` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. **Block 1 — `struct Definition`**
2. **Block 2 — `impl I<Name>Port for Struct`** (Public Contract)
   - Contains **ONLY** the domain port trait (e.g., `IFileReaderPort`, `INetworkClientPort`).
   - **NO** standard library trait impls here (`Default`, `Clone`, `Debug`, `Display`, `From`, etc.).
3. **Block 3 — `impl Struct`** (Constructors, Std Traits & Helpers)
   - `new()`, builders
   - `impl Default`, `impl Clone`, `impl Debug`, `impl Display`, and other std trait impls — these are **constructors/utilities**, not public contracts.
   - Private helper methods (`&self`)

**CRITICAL:** Block 2 is **RESERVED** for the domain port trait ONLY. Standard library trait impls (`Default`, `Clone`, `Debug`, `Display`, `From`) belong in **Block 3** because they serve as constructors or utility formatting, not as the public domain contract.

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic free functions (no `&self`) MUST be extracted OUT of the impl block into their own `*_utility.rs` modules in shared/taxonomy. They do NOT belong in Block 3.

#### Trait Placement Decision Rule

```
Trait impl found in an infrastructure file?
  │
  ├─ Is it the domain port? (I<Name>Port)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3 (alongside constructors)
```

#### Example: Correct 3-Block Order

```rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileCacheAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileReaderPort for FileCacheAdapter {
    fn read(
        &self,
        path: &FilePath,
    ) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path.value())
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for FileCacheAdapter {
    fn default() -> Self {
        Self
    }
}

impl FileCacheAdapter {
    pub fn new() -> Self {
        Self
    }

    fn ensure_dir(path: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap_or_default())
    }
}
```

### Port Rules

- **Every infrastructure struct MUST implement a port trait**
- **Port MUST define methods for all public methods**
- **Port contains ONLY public/contract methods** — no private helpers
- **Private helpers stay in Block 3** (`impl Struct`)
- **Constructors (`new`, builders) in Block 3**
- **Std trait impls (`Default`, `Clone`, etc.) in Block 3**
- **Generic port methods need `where Self: Sized`**

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.rs` + implement port trait**
If no (has business logic) → **split into capabilities layer instead**

## Naming Convention

| Layer              | File Pattern          | Trait File                     | Trait Name         |
| ------------------ | --------------------- | ------------------------------ | ------------------ |
| **Capabilities**   | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  |
| **Infrastructure** | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`      |
| **Agents**         | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` |

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```rust
// BAD: No port implementation
struct FileCache;
impl FileCache {
    fn read(&self) { ... }
}
```

### BAD: Business Logic in Infrastructure

```rust
// BAD: Business logic in infrastructure layer
impl OrphanFileCache {
    fn analyze(&self, content: &str) -> Result {
        // ← DOMAIN LOGIC — should be in capabilities
        let is_orphan = content.contains("orphan");
        return is_orphan;
    }
}
```

### BAD: Dataclass in Layer File

```rust
// BAD: Domain data defined in infrastructure layer
pub struct CacheEntry {  // ← DATA CLASS — should be in shared/taxonomy
    key: String,
    value: String,
    timestamp: u64,
}

pub struct OrphanFileCache {
    entry: CacheEntry,  // ← concrete type, not DI
}
```

### BAD: Std Trait in Block 2

```rust
// BAD: Default impl placed before port trait (wrong block order)
pub struct FileCacheAdapter;

impl Default for FileCacheAdapter {       // ← Block 2 position, but this is NOT the port
    fn default() -> Self { Self }
}

impl IFileReaderPort for FileCacheAdapter {  // ← pushed to Block 3 position
    fn read(&self, path: &FilePath) -> Result<String, std::io::Error> { ... }
}

impl FileCacheAdapter {                   // ← Block 3
    pub fn new() -> Self { Self }
}
```

### GOOD: Implementor with Shared Data

```rust
// GOOD: All data from shared, fields use DI
use shared::common::taxonomy_path_vo::FilePath;

pub struct OrphanFileCache {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,  // ← DI
}

impl IOrphanFileCachePort for OrphanFileCache { ... }
```

### GOOD: Correct 3-Block with Std Traits

```rust
// GOOD: Port in Block 2, Default + new() in Block 3
pub struct FileCacheAdapter;

impl IFileReaderPort for FileCacheAdapter {   // Block 2: domain port ONLY
    fn read(&self, path: &FilePath) -> Result<String, std::io::Error> { ... }
}

impl Default for FileCacheAdapter {           // Block 3: std trait = constructor
    fn default() -> Self { Self }
}

impl FileCacheAdapter {                       // Block 3: constructors & helpers
    pub fn new() -> Self { Self }
}
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has business logic → **MOVE to Capabilities** (AES404)
- If pure I/O/external integration → continue to Step 2

### Step 2: Check for Missing Port

Does the infrastructure struct implement a port trait? If no → create one.

```bash
# Find infrastructure without port implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Port for $struct" "$file" || echo "MISSING: $file has $struct without port"
done
```

### Step 3: Create Port File (if missing)

Create `contract_<name>_port.rs` in the shared crate with all public method signatures.

**Port location:**

| Crate           | Port Path                                              |
| --------------- | ------------------------------------------------------ |
| import-rules    | `crates/shared/src/import_rules/contract_*_port.rs`    |
| code-analysis   | `crates/shared/src/code_analysis/contract_*_port.rs`   |
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_port.rs` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `pub struct <Type>` (struct definition with DI fields)
2. `impl I<Name>Port for <Type>` (all public contract methods — **domain port ONLY**)
3. `impl <Type>` + std trait impls (constructors, `Default`/`Clone`/`Debug`, private helpers — utilities extracted to standalone modules)

### Step 5: Verify Struct Discipline

- **1 file = 1 impl struct** — no multiple structs in one file
- **All data classes in shared/taxonomy** — domain structs must be imported, not defined locally
- **Fields use DI** — `Arc<dyn Trait>`, never concrete types
- **No free functions (no `&self`) remain in Block 3** — extract to `*_utility.rs` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and business logic patterns:

```bash
# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate" crates/<crate>/src/infrastructure_*.rs

# Check for forbidden imports
grep -n "capabilities_\|agent_" crates/<crate>/src/infrastructure_*.rs
```

### Step 7: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Struct → Impl Port Trait → Impl Struct + Std Traits).
- [ ] **Block 2 contains ONLY the domain port trait** (`I<Name>Port`). No std traits (`Default`, `Clone`, `Debug`) in Block 2.
- [ ] **Std trait impls** (`Default`, `Clone`, `Debug`, `Display`) are in **Block 3**, alongside constructors.
- [ ] Infrastructure struct implements a port trait.
- [ ] Port contains **only** public/contract methods (no private helpers).
- [ ] Private helpers are in Block 3 (`impl Struct`).
- [ ] Constructors (`new`, builders) are in Block 3.
- [ ] No free functions (no `&self`) remain in Block 3 — extracted to `*_utility.rs` modules.
- [ ] Stateless utilities exist in their own `*_utility.rs` files in shared/taxonomy.
- [ ] Generic port methods include `where Self: Sized`.
- [ ] **1 file = 1 impl struct** — no multiple structs in one file.
- [ ] All data classes imported from shared/taxonomy (none defined locally).
- [ ] Impl struct fields use DI (`Arc<dyn Trait>`), not concrete types.
- [ ] **Zero business logic** in infrastructure layer (no domain rules, no calculations).
- [ ] No forbidden imports (no capabilities__, no agent__).
- [ ] Port module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes without warnings or errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^impl\|^pub struct" crates/<crate>/src/infrastructure_*.rs

# Find infrastructure without port implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Port for $struct" "$file" || echo "MISSING: $file has $struct without port"
done

# Ensure port does NOT contain private helper keywords
grep -E "fn (helper|util|private|internal)" crates/shared/src/contract_*_port.rs || echo "Clean: No helpers in port"

# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate\|business" crates/<crate>/src/infrastructure_*.rs

# Check for dataclasses defined in layer files
grep -rn "^pub struct" crates/<crate>/src/ | grep -v "shared/" | grep -v "impl\|trait\|fn " | grep infrastructure

# Check for concrete type fields (non-DI)
grep -rn "^\s*[a-z_]*:" crates/<crate>/src/infrastructure_*.rs | grep -v "Arc<dyn"

# Find free functions in Block 3 that should be extracted
grep -n "^    pub fn [a-z_]*(\s*[^&])" crates/<crate>/src/infrastructure_*.rs

# Check for object safety violations
cargo check -p <crate-name> 2>&1 | grep "cannot be made into an object"

# Find unwrap_or_default() calls (error handling)
rg "unwrap_or_default\(\)" crates/<crate>/src/infrastructure_*.rs

# Find magic constants (hardcoded literals)
rg "[0-9]+\.[0-9]+|#[0-9A-Fa-f]+" crates/<crate>/src/infrastructure_*.rs | grep -v "// " | head -20

# Detect std trait impls appearing BEFORE the port trait (wrong block order)
# If Default/Clone/Debug appears before I<Name>Port, the 3-block order is violated
awk '/^impl (Default|Clone|Debug|Display)/{std=NR} /^impl I[A-Z].*Port/{proto=NR} END{if(std && proto && std < proto) print "VIOLATION: std trait (line "std") before port (line "proto")"}' crates/<crate>/src/infrastructure_*.rs
```

## Error Handling (from fix-error-handling)

**Infrastructure Layer Error Rules:**

- Infrastructure may use `unwrap_or_default()` **ONLY when the error is genuinely unrecoverable** (e.g., missing optional config)
- All public methods MUST return `Result<T, E>` where `E` is descriptive
- IO failures → propagate with `Result` or return `LintResult::new_arch()`
- Logic errors → propagate with `Result` + custom error type

### Proper Patterns (Use)

```rust
// [OK] Optional config with sensible default
fn get_timeout(config: &Option<Config>) -> u32 {
    config.as_ref().and_then(|c| c.timeout).unwrap_or(30)
}

// [OK] Explicit error propagation for I/O
fn read_config(path: &FilePath) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path.value())  // Propagate IO error
}
```

## Magic Constant Extraction (from fix-magic-constant)

**Infrastructure Layer Constant Rules:**

- NO hardcoded literals in infrastructure layer
- All domain values MUST be named constants
- Constants MUST live in `taxonomy_*_constant.rs`

```rust
// [FORBIDDEN] BEFORE
fn save(&self) {
    let file = std::fs::File::create("manifest.json");  // magic path
}

// [OK] AFTER
use crate::taxonomy_animator_constant::MANIFEST_FILENAME;
fn save(&self) {
    let file = std::fs::File::create(MANIFEST_FILENAME);
}
```

## Import Strategy (from fix-imports)

When fixing cross-import violations in infrastructure, choose the right approach:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use when the code is **stateless, pure logic** with no side effects:

| Condition                                   | Example                                     |
| ------------------------------------------- | ------------------------------------------- |
| Pure function — no `&self`, no struct state | `parse_path()`, `normalize_name()`          |
| Stateless — all data via parameters         | `fn compute_distance(a: &Point, b: &Point)` |
| No side effects — deterministic output      | `fn sanitize_string(input: &str) -> String` |

```rust
// taxonomy_path_utility.rs (TAXONOMY LAYER)
pub fn parse_path(path: &str) -> Option<String> {
    path.strip_prefix("/").map(|s| s.to_string())
}

// infrastructure_adapter.rs (CONSUMER)
use crate::taxonomy_path_utility::{parse_path, normalize_name}; // ALLOWED: taxonomy import
```

### Option B: Dependency Injection via Traits (Port Pattern)

Use when the code requires **state, side effects, or layer-specific behavior**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Needs `&self` / struct state  | Struct with fields for data/mutation            |
| Has side effects / I/O        | File operations, network calls, DB queries      |
| Layer-specific implementation | Adapter that depends on concrete infrastructure |

```rust
// 1. Define port in CONTRACT layer
// contract_file_writer_port.rs
pub trait IFileWriterPort: Send + Sync {
    fn write(&self, path: &str, content: &str) -> Result<(), Error>;
}

// 2. Infrastructure implements the trait
// infrastructure_file_writer_adapter.rs
pub struct FileWriterAdapter;
impl IFileWriterPort for FileWriterAdapter {
    fn write(&self, path: &str, content: &str) -> Result<(), Error> {
        std::fs::write(path, content)
    }
}

// 3. Consumer receives via DI (knows only the trait)
// infrastructure_data_processor.rs
pub struct DataProcessor {
    writer: Arc<dyn IFileWriterPort>, // via DI, not direct import
}
```

## Common Mistakes (AVOID)

- ❌ **Putting business logic in infrastructure**: Domain rules, calculations, and validation MUST be in capabilities layer.
- ❌ **Defining data structs in layer files**: Domain data classes must be in shared/taxonomy. Only the impl struct belongs in layer files.
- ❌ **Using concrete types as fields**: Impl struct fields should always be `Arc<dyn Trait>` (DI), never concrete implementations.
- ❌ **Putting private helpers in the port**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave port methods and private helpers. Keep them in separate `impl` blocks.
- ❌ **Placing utilities in Block 3**: Stateless free functions (no `&self`) MUST be extracted to standalone `*_utility.rs` modules. They do NOT belong in the impl block.
- ❌ **Creating "God Ports"**: If a port has >10 methods or mixes unrelated concerns, split it into multiple ports.
- ❌ **Forgetting `where Self: Sized`**: This will break `dyn Trait` usage for the rest of the port.
- ❌ **Placing `new()` in the port impl**: Constructors must stay in the inherent `impl Struct` block (Block 3).
- ❌ **Multiple impl structs in one file**: Each file should have exactly ONE impl struct. Use `consolidate-files-rust` if merging multiple files.
- ❌ **Placing std trait impls (`Default`, `Clone`, `Debug`) in Block 2**: Block 2 is RESERVED for the domain port trait ONLY. Std traits are constructors/utilities and belong in Block 3.
- ❌ **Placing `impl Default` before `impl I<Name>Port`**: This breaks the 3-block order. Port trait MUST come first (Block 2), then `Default` + `new()` in Block 3.

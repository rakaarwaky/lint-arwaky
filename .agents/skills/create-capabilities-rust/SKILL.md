---
name: create-capabilities-rust
description: "Create and validate capabilities layer files following AES rules: 3-block structure, one struct per file, trait contracts, zero I/O."
version: 1.1.0
category: refactoring
tags:
  [
    rust,
    aes,
    capability,
    protocol,
    structure,
    aes403,
    aes404,
    3-block-structure,
    di,
  ]
triggers:
  - "create capability rust"
  - "add capability rust"
  - "fix capability structure rust"
  - "create trait rust"
  - "capability missing trait rust"
  - "check capabilities rust"
dependencies: []
related:
  - create-infrastructure-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-capability-structure-rust
  - create-missing-protocols-rust
---
# create-capabilities-rust

## Purpose

Create and validate Rust **capabilities layer** files following clean architecture rules. Ensures capabilities contain zero I/O, implement protocol traits, follow the 3-Block Structure, and use DI for all fields.

## Rules

### Layer Boundaries (AES)

**Capabilities Layer (`capabilities_*.rs`)**

| Allowed                               | Forbidden                                    |
| ------------------------------------- | -------------------------------------------- |
| Computation, validation, calculation  | File I/O (`std::fs`, `File::open`)       |
| Data transformation, business rules   | Network calls (`reqwest`, `hyper`)       |
| Domain logic, domain model definition | Database operations (`sqlx`, `rusqlite`) |
| Trait implementation                  | Direct import from`infrastructure_*`       |
|                                       | Direct import from`agent_*`                |
|                                       | Direct import from`capabilities_*` (self)  |

### Structural Rules (All Layers)

- **1 file = 1 impl struct** — each capabilities file contains exactly ONE main impl struct
- **All data classes in shared** — no structs/enums/cons with data may be defined outside shared/folder-name/taxonomy_
- **Fields must use DI** — impl struct fields should be `Arc<dyn Trait>` objects, not concrete types
- **Helper functions stay in layer** — helper methods that support the impl struct remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic free functions (no `&self`) should be extracted to `*file_name_utility.rs` modules in shared/folder-name/taxonomy_

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. **Block 1 — `struct Definition`**
2. **Block 2 — `impl I<Name>Protocol for Struct`** (Public Contract)
   - Contains **ONLY** the domain protocol trait (e.g., `ILineCheckerProtocol`, `IOrphanAnalyzerProtocol`).
   - **NO** standard library trait impls here (`Default`, `Clone`, `Debug`, `Display`, `From`, etc.).
3. **Block 3 — `impl Struct`** (Constructors, Std Traits & Helpers)
   - `new()`, builders
   - `impl Default`, `impl Clone`, `impl Debug`, `impl Display`, and other std trait impls — these are **constructors/utilities**, not public contracts.
   - Private helper methods (`&self`)

**CRITICAL:** Block 2 is **RESERVED** for the domain protocol trait ONLY. Standard library trait impls (`Default`, `Clone`, `Debug`, `Display`, `From`) belong in **Block 3** because they serve as constructors or utility formatting, not as the public domain contract.

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic free functions (no `&self`) MUST be extracted OUT of the impl block into their own `*_utility.rs` modules in shared/taxonomy. They do NOT belong in Block 3.

#### Trait Placement Decision Rule

```
Trait impl found in a capabilities file?
  │
  ├─ Is it the domain protocol? (I<Name>Protocol)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3 (alongside constructors)
```

#### Example: Correct 3-Block Order

```rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct ArchLineChecker;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        // ... domain logic ...
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self
    }

    fn is_barrel_file(basename: &str) -> bool {
        matches!(basename, "__init__.py" | "mod.rs")
    }
}
```

### Trait Rules

- **Every capability struct MUST implement a trait** (AES403)
- **Trait MUST define methods for all public methods**
- **Trait contains ONLY public/contract methods** — no private helpers
- **Private helpers stay in Block 3** (`impl Struct`)
- **Constructors (`new`, builders) in Block 3**
- **Std trait impls (`Default`, `Clone`, etc.) in Block 3**
- **Generic trait methods need `where Self: Sized`**

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes → **`capabilities_*.rs` + implement protocol trait**
If no (has I/O) → **split into infrastructure layer instead**

## Naming Convention

| Layer                    | File Pattern            | Trait File                       | Trait Name           |
| ------------------------ | ----------------------- | -------------------------------- | -------------------- |
| **Capabilities**   | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  |
| **Infrastructure** | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`      |
| **Agents**         | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` |

## Detection Patterns

### BAD: Capability Without Trait (AES403)

```rust
// BAD: No trait implementation
struct FrameComposer;
impl FrameComposer {
    fn compose_frame(&self) { ... }
}
```

### BAD: Mixed Logic in Capabilities

```rust
// BAD: I/O in capabilities layer
impl MyCapability {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt");  // ← FORBIDDEN
    }
}
```

### BAD: Dataclass in Layer File

```rust
// BAD: Domain data defined in capabilities layer
pub struct OrphanResult {  // ← DATA CLASS — should be in shared/taxonomy
    is_orphan: bool,
    reason: String,
    severity: Severity,
}

pub struct CapabilitiesOrphanAnalyzer {
    result: OrphanResult,  // ← concrete type, not DI
}
```

### BAD: Std Trait in Block 2

```rust
// BAD: Default impl placed before protocol trait (wrong block order)
pub struct ArchLineChecker;

impl Default for ArchLineChecker {       // ← Block 2 position, but this is NOT the protocol
    fn default() -> Self { Self }
}

impl ILineCheckerProtocol for ArchLineChecker {  // ← pushed to Block 3 position
    fn check_line_counts(&self, ...) { ... }
}

impl ArchLineChecker {                   // ← Block 3
    pub fn new() -> Self { Self }
}
```

### GOOD: Implementor with Shared Data

```rust
// GOOD: All data from shared, fields use DI
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,  // ← DI
    cache: Arc<dyn IOrphanFileCachePort>,                   // ← DI
}

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer { ... }
```

### GOOD: Correct 3-Block with Std Traits

```rust
// GOOD: Protocol in Block 2, Default + new() in Block 3
pub struct ArchLineChecker;

impl ILineCheckerProtocol for ArchLineChecker {   // Block 2: domain protocol ONLY
    fn check_line_counts(&self, ...) { ... }
}

impl Default for ArchLineChecker {                 // Block 3: std trait = constructor
    fn default() -> Self { Self }
}

impl ArchLineChecker {                             // Block 3: constructors & helpers
    pub fn new() -> Self { Self }
}
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has I/O → **MOVE to Infrastructure** (AES404)
- If pure business logic → continue to Step 2

### Step 2: Check for Missing Trait (AES403)

Does the capability struct implement a trait? If no → create one.

```bash
# Find capabilities without trait implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Protocol for $struct" "$file" || echo "MISSING: $file has $struct without trait"
done
```

### Step 3: Create Trait File (if missing)

Create `contract_<name>_protocol.rs` in the shared crate with all public method signatures.

**Trait location:**

| Crate           | Trait Path                                                   |
| --------------- | ------------------------------------------------------------ |
| import-rules    | `crates/shared/src/import_rules/contract_*_protocol.rs`    |
| code-analysis   | `crates/shared/src/code_analysis/contract_*_protocol.rs`   |
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_protocol.rs` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `pub struct <Type>` (struct definition with DI fields)
2. `impl I<Name>Protocol for <Type>` (all public contract methods — **domain protocol ONLY**)
3. `impl <Type>` + std trait impls (constructors, `Default`/`Clone`/`Debug`, private helpers — utilities extracted to standalone modules)

### Step 5: Verify Struct Discipline

- **1 file = 1 impl struct** — no multiple structs in one file
- **All data classes in shared/taxonomy** — domain structs must be imported, not defined locally
- **Fields use DI** — `Arc<dyn Trait>`, never concrete types
- **No free functions (no `&self`) remain in Block 3** — extract to `*_utility.rs` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and I/O patterns:

```bash
# Check for I/O in capabilities
grep -n "std::fs\|File::open\|reqwest\|sqlx" crates/<crate>/src/capabilities_*.rs

# Check for forbidden imports
grep -n "infrastructure_\|agent_" crates/<crate>/src/capabilities_*.rs
```

### Step 7: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Struct → Impl Protocol Trait → Impl Struct + Std Traits).
- [ ] **Block 2 contains ONLY the domain protocol trait** (`I<Name>Protocol`). No std traits (`Default`, `Clone`, `Debug`) in Block 2.
- [ ] **Std trait impls** (`Default`, `Clone`, `Debug`, `Display`) are in **Block 3**, alongside constructors.
- [ ] Capability struct implements a protocol trait (AES403 resolved).
- [ ] Trait contains **only** public/contract methods (no private helpers).
- [ ] Private helpers are in Block 3 (`impl Struct`).
- [ ] Constructors (`new`, builders) are in Block 3.
- [ ] No free functions (no `&self`) remain in Block 3 — extracted to `*_utility.rs` modules.
- [ ] Stateless utilities exist in their own `*_utility.rs` files in shared/taxonomy.
- [ ] Generic trait methods include `where Self: Sized`.
- [ ] **1 file = 1 impl struct** — no multiple structs in one file.
- [ ] All data classes imported from shared/taxonomy (none defined locally).
- [ ] Impl struct fields use DI (`Arc<dyn Trait>`), not concrete types.
- [ ] **Zero I/O** in capabilities layer (no std::fs, no network, no database).
- [ ] No forbidden imports (no infrastructure__, no agent__).
- [ ] Trait module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes without warnings or errors.

## Error Handling (from fix-error-handling)

**Capabilities Layer Error Rules:**

- **Never silently discard errors** with `unwrap_or_default()` in capabilities layer
- All public methods MUST return `Result<T, E>` where `E` is descriptive
- IO errors (file read, network) → propagate with `Result` or return `LintResult::new_arch()`
- Logic errors (validation, parsing) → propagate with `Result` + custom error type

### Silent Swallowing (Fix)

```rust
// [FORBIDDEN] Error silently discarded
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()  // Error thrown away
}

// [FORBIDDEN] Error detail lost
.cycle_check()
.map_err(|e| format!("{:?}", e))  // Debug formatting loses context
.unwrap_or_default()               // Silently discarded
```

### Proper Patterns (Use)

```rust
// [OK] Explicit error propagation
fn parse_file(path: &FilePath) -> Result<Content, ParseError> {
    std::fs::read_to_string(path).map_err(ParseError::Io)
}

// [OK] LintResult for check failures (not IO failures)
fn check_imports(...) -> Vec<LintResult> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return vec![LintResult::new_arch(
            "PARSE_ERROR", &format!("Cannot read: {}", e), path.clone()
        )],
    };
    // Import check failure -> LintResult (expected outcome)
}
```

## Primitive-to-VO Replacement (from fix-primitive-to-vo)

**Capabilities Layer VO Rules:**

- Entity fields MUST use VOs, not primitives (`String`, `i32`, `f64`, `bool`)
- Contract signatures MUST use VOs
- VOs MUST validate on construction

```rust
// BEFORE (primitive)
pub struct LintResult {
    pub file_path: String,
    pub line: u32,
    pub severity: String,
}

// AFTER (VO)
pub struct LintResult {
    pub file_path: FilePath,
    pub line: LineNumber,
    pub severity: Severity,
}
```

## Magic Constant Extraction (from fix-magic-constant)

**Capabilities Layer Constant Rules:**

- NO hardcoded literals in capabilities layer
- All domain values MUST be named constants
- Constants MUST live in `taxonomy_*_constant.rs`

```rust
// [FORBIDDEN] BEFORE
fn calculate_duration(&self) -> f64 {
    0.5  // magic
}

// [OK] AFTER
use crate::taxonomy_animator_constant::MIN_REVEAL_SECONDS;
fn calculate_duration(&self) -> f64 {
    MIN_REVEAL_SECONDS
}
```

## Import Strategy (from fix-imports)

When fixing cross-import violations in capabilities, choose the right approach:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use when the code is **stateless, pure logic** with no side effects:

| Condition                                     | Example                                       |
| --------------------------------------------- | --------------------------------------------- |
| Pure function — no`&self`, no struct state | `parse_path()`, `normalize_name()`        |
| Stateless — all data via parameters          | `fn compute_distance(a: &Point, b: &Point)` |
| No side effects — deterministic output       | `fn sanitize_string(input: &str) -> String` |

```rust
// taxonomy_path_utility.rs (TAXONOMY LAYER)
pub fn parse_path(path: &str) -> Option<String> {
    path.strip_prefix("/").map(|s| s.to_string())
}

// capabilities_timeline_processor.rs (CONSUMER)
use crate::taxonomy_path_utility::{parse_path, normalize_name}; // ALLOWED: taxonomy import
```

### Option B: Dependency Injection via Traits (Port/Protocol Pattern)

Use when the code requires **state, side effects, or layer-specific behavior**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Needs`&self` / struct state | Struct with fields for data/mutation            |
| Has side effects / I/O        | File operations, network calls, DB queries      |
| Layer-specific implementation | Adapter that depends on concrete infrastructure |

```rust
// 1. Define protocol in CONTRACT layer
// contract_frame_exporter_protocol.rs
pub trait IFrameExporterProtocol: Send + Sync {
    fn export(&self, frame: &Frame) -> PathBuf;
}

// 2. Capability implements the trait
// capabilities_frame_exporter.rs
pub struct FrameExporter { output_dir: PathBuf }
impl IFrameExporterProtocol for FrameExporter {
    fn export(&self, frame: &Frame) -> PathBuf {
        self.output_dir.join(format!("{}.png", frame.id))
    }
}

// 3. Consumer receives via DI (knows only the trait)
// capabilities_timeline_processor.rs
pub struct TimelineProcessor {
    exporter: Arc<dyn IFrameExporterProtocol>, // via DI, not direct import
}
```

## Decision Tree: Which Option to Choose?

```
Encountered cross-import violation in capabilities?
  │
  ├─ Does the code need &self / struct state?
  │   └─ YES → Option B: Dependency Injection
  │
  ├─ Does the code have side effects (I/O, file, network)?
  │   └─ YES → Option B: Dependency Injection
  │
  └─ Is it pure, stateless, no &self?
      └─ YES → Option A: Extract to Taxonomy Utility
```

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^impl\|^pub struct" crates/<crate>/src/capabilities_*.rs

# Find capabilities without trait implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Protocol for $struct" "$file" || echo "MISSING: $file has $struct without trait"
done

# Ensure trait does NOT contain private helper keywords
grep -E "fn (helper|util|private|internal)" crates/shared/src/contract_*_protocol.rs || echo "Clean: No helpers in trait"

# Check for I/O in capabilities (AES404)
grep -n "std::fs\|File::open\|reqwest\|sqlx" crates/<crate>/src/capabilities_*.rs

# Check for dataclasses defined in layer files
grep -rn "^pub struct" crates/<crate>/src/ | grep -v "shared/" | grep -v "impl\|trait\|fn " | grep capabilities

# Check for concrete type fields (non-DI)
grep -rn "^\s*[a-z_]*:" crates/<crate>/src/capabilities_*.rs | grep -v "Arc<dyn"

# Find free functions in Block 3 that should be extracted
grep -n "^    pub fn [a-z_]*(\s*[^&])" crates/<crate>/src/capabilities_*.rs

# Check for object safety violations
cargo check -p <crate-name> 2>&1 | grep "cannot be made into an object"

# Find unwrap_or_default() calls (error handling)
rg "unwrap_or_default\(\)" crates/<crate>/src/capabilities_*.rs

# Find magic constants (hardcoded literals)
rg "[0-9]+\.[0-9]+|#[0-9A-Fa-f]+" crates/<crate>/src/capabilities_*.rs | grep -v "// " | head -20

# Detect std trait impls appearing BEFORE the protocol trait (wrong block order)
# If Default/Clone/Debug appears before I<Name>Protocol, the 3-block order is violated
awk '/^impl (Default|Clone|Debug|Display)/{std=NR} /^impl I[A-Z].*Protocol/{proto=NR} END{if(std && proto && std < proto) print "VIOLATION: std trait (line "std") before protocol (line "proto")"}' crates/<crate>/src/capabilities_*.rs
```

## Common Mistakes (AVOID)

- ❌ **Putting I/O in capabilities**: File I/O, network calls, and database operations MUST be in infrastructure layer.
- ❌ **Defining data structs in layer files**: Domain data classes must be in shared/taxonomy. Only the impl struct belongs in layer files.
- ❌ **Using concrete types as fields**: Impl struct fields should always be `Arc<dyn Trait>` (DI), never concrete implementations.
- ❌ **Putting private helpers in the trait**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave trait methods and private helpers. Keep them in separate `impl` blocks.
- ❌ **Placing utilities in Block 3**: Stateless free functions (no `&self`) MUST be extracted to standalone `*_utility.rs` modules. They do NOT belong in the impl block.
- ❌ **Creating "God Traits"**: If a trait has >10 methods or mixes unrelated concerns, split it into multiple traits.
- ❌ **Forgetting `where Self: Sized`**: This will break `dyn Trait` usage for the rest of the trait.
- ❌ **Placing `new()` in the trait impl**: Constructors must stay in the inherent `impl Struct` block (Block 3).
- ❌ **Multiple impl structs in one file**: Each file should have exactly ONE impl struct. Use `consolidate-files-rust` if merging multiple files.
- ❌ **Placing std trait impls (`Default`, `Clone`, `Debug`) in Block 2**: Block 2 is RESERVED for the domain protocol trait ONLY. Std traits are constructors/utilities and belong in Block 3.
- ❌ **Placing `impl Default` before `impl I<Name>Protocol`**: This breaks the 3-block order. Protocol trait MUST come first (Block 2), then `Default` + `new()` in Block 3.

---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: pure domain behavior, zero I/O, 3-block structure, one impl struct per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    capability,
    protocol,
    structure,
    aes402,
    aes403,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create capability rust"
  - "add capability rust"
  - "fix capability structure rust"
  - "create trait rust"
  - "capability missing trait rust"
  - "check capabilities rust"
  - "audit capabilities rust"
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

Create and validate Rust **capabilities layer** files following clean architecture / AES rules.

A capabilities file must contain **pure domain behavior**:

- no I/O,
- no infrastructure detail,
- no agent detail,
- no locally defined domain data structures,
- one implementation struct per file,
- one domain protocol trait as the public contract,
- strict 3-block structure,
- dependency injection for service collaborators,
- shared VOs for domain data.

---

## Definition of Done

A capabilities file is considered valid when:

1. It contains exactly **ONE implementation struct**.
2. The struct implements exactly **ONE domain protocol trait** in Block 2.
3. Block 2 contains **ONLY** the domain protocol trait implementation.
4. Constructors, std trait impls, and private helpers are placed in Block 3.
5. The file contains **zero I/O** and zero side-effecting infrastructure calls.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via `Arc<dyn Trait>`.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
10. Domain-specific helpers may remain inside the implementation file.
11. `cargo check -p <crate-name>` passes.

---

## Rules

### Layer Boundaries (AES)

#### Capabilities Layer (`capabilities_*.rs`)

| Allowed                                    | Forbidden                                               |
| ------------------------------------------ | ------------------------------------------------------- |
| Computation, validation, calculation       | File I/O (`std::fs`, `File::open`, `read_dir`)    |
| Data transformation, business rules        | Network calls (`reqwest`, `hyper`)                  |
| Domain behavior using shared models        | Database operations (`sqlx`, `rusqlite`)            |
| Protocol trait implementation              | Direct stdout/stderr printing                           |
| Private helpers supporting the impl struct | Direct environment/system-clock/global-state mutation   |
| Calling injected port/protocol traits      | Direct import from`infrastructure_*`                  |
|                                            | Direct import from`agent_*`                           |
|                                            | Direct dependency on concrete`capabilities_*` modules |
|                                            | Locally defined domain data structures                  |

Capabilities may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol/port traits

Capabilities must not depend on concrete infrastructure or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation struct per file

Each capabilities file contains exactly ONE main implementation struct.

```rust
pub struct CapabilitiesOrphanAnalyzer {
    // ...
}
```

Do not define multiple service structs in the same file.

---

#### 2. Only the implementation struct may be defined in the layer file

A capabilities file may define the implementation struct only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in capabilities files:

```rust
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

Allowed:

```rust
use shared::orphan_detector::taxonomy_orphan_result_vo::OrphanResult;
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, services, adapters, or ports MUST use trait objects.

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}
```

Do not use concrete service types:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: FilenameExtractor, // BAD: concrete dependency
}
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, names, thresholds, etc. should use shared VOs.

```rust
pub struct FrameExporter {
    output_dir: OutputDirectory, // shared VO
}
```

Avoid raw primitives for domain values:

```rust
pub struct FrameExporter {
    output_dir: String, // BAD: primitive domain value
}
```

Borrowed primitives such as `&str` may be allowed in method parameters when VO wrapping is impractical, but domain identifiers should prefer VOs.

---

### Helper vs Utility Decision

The boundary is not only about `&self`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this struct, or by multiple modules?

---

#### Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It knows AES-specific patterns, layer names, suffixes, violation codes, or taxonomy conventions.
3. It accesses `self.field` or instance state.
4. It is tightly coupled to this capability only.
5. It is a factory method such as `new()` or builder method.
6. It is stateless but only used by this one struct and is domain-specific.

Example:

```rust
impl ContractRoleChecker {
    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        // Domain-specific parsing logic.
        // Even without `&self`, this can remain a private helper
        // if only this checker uses it.
    }
}
```

---

#### Extract to Utility (`*_utility.rs`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/infrastructures/modules.

Example:

```rust
// shared/code_analysis/taxonomy_string_utility.rs
pub fn match_whole_token(haystack: &str, needle: &str) -> bool {
    // generic token matching
}
```

---

#### I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in capabilities.

```rust
// BAD in capabilities layer
fn read_config(file_path: &str) -> Option<String> {
    std::fs::read_to_string(file_path).ok()
}
```

Correct placement:

```rust
// infrastructure_config_reader.rs
fn read_config(file_path: &FilePath) -> Result<ConfigContent, ConfigReadError> {
    let raw = std::fs::read_to_string(file_path.value())
        .map_err(ConfigReadError::Io)?;

    ConfigContent::new(raw)
        .map_err(ConfigReadError::Validation)
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
2. **Block 2 — Domain Protocol Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

---

### Block 1 — Struct Definition

```rust
pub struct ArchLineChecker;
```

Or with dependencies:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
    policy: OrphanAnalysisPolicy,
}
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol trait ONLY.

```rust
impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        // domain behavior
    }
}
```

Do NOT put these in Block 2:

```rust
impl Default for ArchLineChecker
impl Clone for ArchLineChecker
impl Debug for ArchLineChecker
impl Display for ArchLineChecker
impl From<...> for ArchLineChecker
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
- domain-specific associated functions used only by this struct

```rust
impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self
    }

    fn effective_threshold(&self, layer: &LayerDefinition) -> LineCountThreshold {
        // private helper
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
use shared::code_analysis::taxonomy_line_checker_utility::is_barrel_file;
```

But if the function is domain-specific and only used by this struct, it may remain in Block 3.

---

### Trait Placement Decision Rule

```text
Trait impl found in a capabilities file?
  │
  ├─ Is it the domain protocol? (I<Name>Protocol)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```

---

## Example: Correct 3-Block Structure

```rust
use std::sync::Arc;

use shared::code_analysis::taxonomy_file_path_vo::FilePath;
use shared::code_analysis::taxonomy_layer_definition_vo::LayerDefinition;
use shared::code_analysis::taxonomy_line_checker_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_line_checker_utility::is_barrel_file;
use shared::code_analysis::taxonomy_lint_result_vo::LintResult;
use shared::code_analysis::taxonomy_source_vo::SourceContentVO;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ArchLineChecker;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = file.basename();

        if is_barrel_file(basename) {
            return;
        }

        // Remaining domain logic...
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

    fn is_layer_relevant(&self, definition: &LayerDefinition) -> bool {
        // Private helper specific to this checker.
        true
    }
}
```

---

## Trait Rules

### AES403 — Capability Must Implement Protocol Trait

Every capability struct MUST implement a domain protocol trait.

```rust
impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    // public contract
}
```

---

### Trait file naming

| Layer          | File Pattern            | Trait File                       | Trait Name           |
| -------------- | ----------------------- | -------------------------------- | -------------------- |
| Capabilities   | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  |
| Infrastructure | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`      |
| Agents         | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` |

---

### Trait content rules

The protocol trait MUST contain only public domain contract methods.

Good:

```rust
pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
}
```

Bad:

```rust
pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(&self, ...);

    fn private_helper(&self); // BAD: helper in trait
}
```

---

### Constructors are not protocol methods

`new()` and builders MUST stay in Block 3.

Bad:

```rust
pub trait ILineCheckerProtocol {
    fn new() -> Self; // BAD
}
```

Good:

```rust
impl ArchLineChecker {
    pub fn new() -> Self {
        Self
    }
}
```

---

### Object safety

Protocol traits intended for `Arc<dyn Trait>` MUST be object-safe.

Avoid generic methods in dyn-compatible traits unless bounded properly.

If a generic method is required, add:

```rust
where
    Self: Sized,
```

or split the trait into:

- object-safe protocol trait
- generic extension trait

---

## Detection Patterns

### BAD: Capability Without Trait (AES403)

```rust
pub struct FrameComposer;

impl FrameComposer {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
```

Fix:

```rust
pub struct FrameComposer;

impl IFrameComposerProtocol for FrameComposer {
    fn compose_frame(&self) {
        // contract implementation
    }
}
```

---

### BAD: I/O in Capabilities (AES404)

```rust
impl MyCapability {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt"); // FORBIDDEN
    }
}
```

Fix:

Move I/O to infrastructure or port implementation.

```rust
// infrastructure_source_reader.rs
impl ISourceReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<SourceContentVO, SourceReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(SourceReadError::Io)?;

        SourceContentVO::new(path.clone(), raw)
            .map_err(SourceReadError::Validation)
    }
}
```

Capabilities receives already-loaded data:

```rust
impl IImportCheckerProtocol for ImportChecker {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult> {
        // pure analysis
        Vec::new()
    }
}
```

---

### BAD: Data Class Defined in Layer File

```rust
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

Fix:

Move to shared taxonomy:

```rust
// shared/orphan_detector/taxonomy_orphan_result_vo.rs
pub struct OrphanResult {
    is_orphan: OrphanFlag,
    reason: OrphanReason,
}
```

Then import it:

```rust
use shared::orphan_detector::taxonomy_orphan_result_vo::OrphanResult;
```

---

### BAD: Concrete Service Field

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: FilenameExtractor, // BAD
}
```

Fix:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}
```

---

### BAD: Std Trait in Block 2

```rust
pub struct ArchLineChecker;

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(&self, ...) {
        // ...
    }
}
```

Fix:

```rust
pub struct ArchLineChecker;

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(&self, ...) {
        // ...
    }
}

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}
```

---

### GOOD: Capability with DI and Shared VO

```rust
use std::sync::Arc;

use shared::orphan_detector::taxonomy_orphan_analysis_policy_vo::OrphanAnalysisPolicy;
use shared::orphan_detector::taxonomy_orphan_file_cache_port::IOrphanFileCachePort;
use shared::orphan_detector::taxonomy_orphan_filename_extractor_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_capabilities_orphan_protocol::ICapabilitiesOrphanProtocol;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
    policy: OrphanAnalysisPolicy,
}

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    // public contract methods only
}
```

---

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask:

> Is this pure domain behavior?

If yes → keep as capabilities.

If no → move I/O or side-effecting code to infrastructure.

Examples of code that must move out of capabilities:

- `std::fs`
- `File::open`
- `reqwest`
- `hyper`
- `sqlx`
- `rusqlite`
- direct printing
- environment mutation
- system clock access
- global state mutation

---

### Step 2: Check Missing Trait (AES403)

Does the capability struct implement a protocol trait?

If no:

1. create `contract_<name>_protocol.rs`
2. define `I<Name>Protocol`
3. move public domain method signatures into the trait
4. implement the trait for the struct

---

### Step 3: Create Trait File if Missing

Create trait file in the appropriate shared domain folder.

Examples:

| Crate           | Trait Path                                                   |
| --------------- | ------------------------------------------------------------ |
| import-rules    | `crates/shared/src/import_rules/contract_*_protocol.rs`    |
| code-analysis   | `crates/shared/src/code_analysis/contract_*_protocol.rs`   |
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_protocol.rs` |

Register the module in the relevant `mod.rs`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. struct definition
2. domain protocol trait implementation
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
Does it know domain rules?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.rs
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure no forbidden imports or I/O patterns.

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `unwrap_or_default()`
- fallible operations return descriptive `Result`
- check/analysis methods may return `Vec<LintResult>`
- domain data uses VOs
- no magic constants

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
- [ ] Block 2 contains ONLY the domain protocol trait implementation.
- [ ] Block 3 contains constructors, std traits, and private helpers.
- [ ] Capability struct implements a protocol trait (AES403).
- [ ] Trait contains only public domain contract methods.
- [ ] Private helpers are not declared in the trait.
- [ ] Constructors are not declared in the trait.
- [ ] Std trait impls are in Block 3.
- [ ] Domain-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] Generic trait methods are object-safe or bounded with `where Self: Sized`.
- [ ] One file contains exactly one implementation struct.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use `Arc<dyn Trait>`.
- [ ] Value/configuration fields use shared VOs.
- [ ] Zero I/O in capabilities layer (AES404).
- [ ] No forbidden imports from `infrastructure_*`.
- [ ] No forbidden imports from `agent_*`.
- [ ] No direct dependency on concrete `capabilities_*` implementations.
- [ ] Trait module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

---

## Error Handling Rules

Capabilities error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```rust
let value = result.unwrap_or_default();
```

Forbidden:

```rust
let value = result.ok().unwrap_or_default();
```

---

### Rule 2: Fallible operations should return `Result`

If a method represents an operation that can fail unexpectedly, return `Result<T, E>`.

```rust
fn parse_manifest(content: &ManifestContent) -> Result<Manifest, ManifestParseError> {
    // ...
}
```

---

### Rule 3: Check/analysis methods may return `Vec<LintResult>`

For linting/analysis use cases, violations are expected domain outcomes.

```rust
fn check_imports(source: &SourceContentVO) -> Vec<LintResult> {
    let mut violations = Vec::new();

    // analysis logic

    violations
}
```

This is allowed.

---

### Rule 4: I/O errors belong to infrastructure/port implementations

Bad in capabilities:

```rust
fn check_file(path: &FilePath) -> Vec<LintResult> {
    let content = std::fs::read_to_string(path.value()).unwrap_or_default(); // BAD
    Vec::new()
}
```

Good:

```rust
// infrastructure_source_reader.rs
impl ISourceReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<SourceContentVO, SourceReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(SourceReadError::Io)?;

        SourceContentVO::new(path.clone(), raw)
            .map_err(SourceReadError::Validation)
    }
}
```

```rust
// capabilities_import_checker.rs
impl IImportCheckerProtocol for ImportChecker {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult> {
        // pure analysis using already-read source
        Vec::new()
    }
}
```

---

## Primitive-to-VO Replacement Rules (AES402)

### General Rule

Domain data MUST use shared VOs, not raw primitives.

Bad:

```rust
pub struct LintResult {
    pub file_path: String,
    pub line: u32,
    pub severity: String,
}
```

Good:

```rust
pub struct LintResult {
    pub file_path: FilePath,
    pub line: LineNumber,
    pub severity: Severity,
}
```

---

### Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and contract return values. Use VO.                     |
| `i32`, `i64`     | Forbidden. Use domain VO.                                                           |
| `u32`, `u64`     | Forbidden. Use domain VO.                                                           |
| `usize`, `isize` | Forbidden for domain values. Use domain VO.                                         |
| `f32`, `f64`     | Forbidden. Use domain VO.                                                           |
| `char`             | Forbidden for domain values. Use domain VO.                                         |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- file paths
- symbol names
- messages
- line numbers
- column numbers
- severity
- durations
- counts
- thresholds
- identifiers

---

## Magic Constant Extraction Rules

No hardcoded domain literals in capabilities.

Bad:

```rust
fn calculate_duration(&self) -> f64 {
    0.5
}
```

Good:

```rust
use crate::taxonomy_animator_constant::MIN_REVEAL_SECONDS;

fn calculate_duration(&self) -> DurationSeconds {
    MIN_REVEAL_SECONDS
}
```

Constants MUST live in:

```text
taxonomy_*_constant.rs
```

---

## Import Strategy

When fixing cross-import violations in capabilities, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```rust
// shared/code_analysis/taxonomy_path_utility.rs
pub fn normalize_relative_path(path: &str) -> Option<String> {
    path.strip_prefix("/").map(|s| s.to_string())
}
```

Consumer:

```rust
use shared::code_analysis::taxonomy_path_utility::normalize_relative_path;
```

---

### Option B: Dependency Injection via Port/Protocol Trait

Use when the code needs:

- state,
- collaborators,
- side effects,
- infrastructure behavior,
- layer-specific implementation.

Example:

```rust
// contract_output_path_builder_protocol.rs
pub trait IOutputPathBuilderProtocol: Send + Sync {
    fn build_frame_path(&self, frame: &Frame) -> FrameOutputPath;
}
```

```rust
// capabilities_frame_exporter.rs
pub struct FrameExporter {
    path_builder: Arc<dyn IOutputPathBuilderProtocol>,
}

impl IFrameExporterProtocol for FrameExporter {
    fn export(&self, frame: &Frame) -> FrameOutputPath {
        self.path_builder.build_frame_path(frame)
    }
}
```

The capability depends only on the protocol, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# Check possible I/O in capabilities (AES404)
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/capabilities_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(infrastructure_|agent_)" crates/<crate>/src/capabilities_*.rs

# List structs in capabilities files
rg -n "^\s*pub struct" crates/<crate>/src/capabilities_*.rs

# List protocol trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Protocol\s+for" crates/<crate>/src/capabilities_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/capabilities_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/capabilities_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
```

---

### Check Wrong Block Order

```bash
for file in crates/<crate>/src/capabilities_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }

    /^impl (Default|Clone|Debug|Display)/ {
      if (!std) std = FNR
    }

    /^impl I[A-Z].*Protocol/ {
      if (!proto) proto = FNR
    }

    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before protocol (line " proto ")"
      }
    }
  ' "$file"
done
```

---

## Common Mistakes

- ❌ Putting I/O in capabilities.
- ❌ Defining domain data structs in capabilities files.
- ❌ Using concrete service types as struct fields.
- ❌ Using raw primitives for domain value fields.
- ❌ Putting private helpers in the protocol trait.
- ❌ Putting constructors in the protocol trait.
- ❌ Placing std trait impls before the domain protocol trait.
- ❌ Mixing Block 2 and Block 3 responsibilities.
- ❌ Keeping reusable, domain-agnostic utility functions inside Block 3.
- ❌ Extracting domain-specific single-consumer helpers to shared utility too early.
- ❌ Creating god traits with too many unrelated methods.
- ❌ Forgetting object safety for `Arc<dyn Trait>` usage.
- ❌ Multiple implementation structs in one file.
- ❌ Direct dependency on concrete capabilities implementations.
- ❌ Silent error swallowing with `unwrap_or_default()`.
- ❌ Magic constants in capabilities logic.

```

```

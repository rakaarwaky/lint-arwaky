---
name: create-taxonomy-rust
description: "Create and validate taxonomy layer files (shared/taxonomy) — all data classes, VOs, errors, and utilities must live here following strict naming conventions."
version: 1.0.0
category: refactoring
tags: [rust, aes, taxonomy, shared, dataclass, vo, entity, utility, structure]
triggers:
  - "create taxonomy rust"
  - "add taxonomy rust"
  - "move to taxonomy rust"
  - "dataclass in shared rust"
  - "create value object rust"
  - "create taxonomy entity rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - method_classifier-rust
---

# create-taxonomy-rust

## Purpose

Create and validate Rust **taxonomy layer** files in `crates/shared/src/<domain>/`. This is where ALL data classes, value objects, errors, constants, and stateless utility functions MUST live. No domain structs may be defined in capabilities, infrastructure, agents, or surface layers.

## Rules

### The Fundamental Question

> **"Is this struct a dataclass?"**

- **Dataclass** (struct with domain data, DTOs, results, VOs) → **MUST be in shared/taxonomy**. Never in capabilities/infrastructure/agents/surface.
- **Implementor** (struct that implements a trait, uses DI) → belongs in the layer file (`capabilities_*.rs`, `infrastructure_*.rs`, `agent_*.rs`).

### Taxonomy Layer Structure

```
crates/shared/src/
├── lib.rs                    # Top-level module declarations
├── common/                   # Cross-domain shared types
│   ├── mod.rs
│   └── taxonomy_*.rs
├── <domain>/                 # Domain-specific taxonomy
│   ├── mod.rs                # Module exports for this domain
│   ├── contract_*.rs         # Contract traits (port, protocol, aggregate)
│   ├── taxonomy_*_vo.rs      # Value Objects
│   ├── taxonomy_*_entity.rs  # Entity types
│   ├── taxonomy_*_error.rs   # Error types
│   ├── taxonomy_*_event.rs   # Event types
│   └── taxonomy_*_utility.rs # Stateless utility functions
```

### File Naming Convention

Taxonomy files follow strict naming patterns:

| Suffix      | Purpose                              | Allowed? | Example                              |
| ----------- | ------------------------------------ | -------- | ------------------------------------ |
| `_vo`       | Value Objects (wraps a single value) | ✅ YES   | `taxonomy_import_rule_vo.rs`         |
| `_entity`   | Domain entities with identity        | ✅ YES   | `taxonomy_analysis_entity.rs`        |
| `_error`    | Error types (`thiserror::Error`)     | ✅ YES   | `taxonomy_config_error.rs`           |
| `_event`    | Event/message types                  | ✅ YES   | `taxonomy_scan_event.rs`             |
| `_constant` | Static compile-time constants        | ✅ YES   | `taxonomy_layer_names_constant.rs`   |
| `_utility`  | Stateless free functions             | ✅ YES   | `taxonomy_symbol_renamer_utility.rs` |

**CRITICAL:** These suffixes are **strict** — only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed for `taxonomy_` prefixed files. No other suffixes.

### Import Restrictions (AES201)

Taxonomy files must remain **completely pure**:

| Taxonomy Type                                          | Can Import From              | Cannot Import From                                              |
| ------------------------------------------------------ | ---------------------------- | --------------------------------------------------------------- |
| **taxonomy(vo)**                                       | Other taxonomy types         | agents, infrastructure, surfaces, contracts, capabilities, root |
| **taxonomy(entity), taxonomy(error), taxonomy(event)** | taxonomy VOs/constants       | agents, infrastructure, surfaces, contracts, capabilities       |
| **taxonomy(constant)**                                 | Nothing (pure static values) | Any external imports                                            |
| **taxonomy(utility)**                                  | taxonomy types               | Non-taxonomy layers                                             |

### Dataclass Patterns

#### Value Objects (`_vo.rs`)

Wrap a single value with type safety:

```rust
// taxonomy_import_rule_vo.rs
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportRuleVO {
    pub pattern: String,
    pub message: String,
}

impl ImportRuleVO {
    pub fn new(pattern: String, message: String) -> Self {
        Self { pattern, message }
    }

    pub fn value(&self) -> &str {
        &self.pattern
    }
}

impl fmt::Display for ImportRuleVO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pattern)
    }
}
```

#### Macro-Generated Value Objects

For simple wrappers, use macros:

```rust
// taxonomy_common_vo.rs
string_value_object!(FieldName);      // wraps String
primitive_value_object!(BooleanVO);   // wraps bool
primitive_value_object!(SeverityVO);  // wraps u32
```

#### Error Types (`_error.rs`)

Use `thiserror::Error`:

```rust
// taxonomy_config_error.rs
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Error)]
pub struct ConfigError {
    pub key: String,
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config error: {} - {}", self.key, self.message)
    }
}
```

#### Utility Functions (`_utility.rs`)

Stateless free functions (no `&self`, no side effects):

```rust
// taxonomy_symbol_renamer_utility.rs

/// Stateless formatting utility — no &self needed
pub fn format_bytes(bytes: u64) -> String {
    // standalone, reusable across all layers
    ...
}

/// Stateless math utility — no &self needed
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    ...
}
```

## Detection Patterns

### BAD: Dataclass Defined in Layer File

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

### BAD: Dataclass Defined in Infrastructure

```rust
// BAD: Domain data defined in infrastructure layer
pub struct CacheEntry {  // ← DATA CLASS — should be in shared/taxonomy
    key: String,
    value: String,
    timestamp: u64,
}
```

### GOOD: Dataclass in Taxonomy + Implementor with DI

```rust
// GOOD: Dataclass in taxonomy
// crates/shared/src/orphan-detector/taxonomy_analysis_vo.rs
pub struct OrphanIndicatorResult {
    is_orphan: bool,
    reason: String,
    severity: Severity,
}

// GOOD: Implementor imports from taxonomy
// crates/orphan-detector/src/capabilities_orphan_analyzer.rs
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,  // ← DI
    cache: Arc<dyn IOrphanFileCachePort>,                   // ← DI
}
```

## Workflow

### Step 1: Identify the Dataclass

When you find a struct in a layer file (capabilities/infrastructure/agent/surface), ask: **"Is this a dataclass or an implementor?"**

- If it contains domain data, DTOs, results, or value wrappers → **dataclass → move to taxonomy**
- If it implements a trait and uses DI → **implementor → stays in layer file**

### Step 2: Determine Taxonomy Domain

Find the correct domain directory under `crates/shared/src/<domain>/`:

| Domain            | Directory                     | Example Types                              |
| ----------------- | ----------------------------- | ------------------------------------------ |
| `common`          | `shared/src/common/`          | Cross-domain types (PathVO, BooleanVO)     |
| `orphan-detector` | `shared/src/orphan-detector/` | Orphan results, severity, violations       |
| `code-analysis`   | `shared/src/code-analysis/`   | Analysis results, reachability, violations |
| `import-rules`    | `shared/src/import-rules/`    | Import rules, violations, language types   |
| `naming-rules`    | `shared/src/naming-rules/`    | Naming violations, patterns                |

### Step 3: Create or Update Taxonomy File

**Option A: New taxonomy domain** — Create `<domain>/` directory with `mod.rs`, then add taxonomy files.

**Option B: Existing domain** — Add new file to existing domain directory.

**Naming:** Use the correct suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).

```bash
# Example: Create orphan result dataclass in taxonomy
mkdir -p crates/shared/src/orphan-detector/
# Create taxonomy_orphan_vo.rs
```

### Step 4: Register Module

Update the domain's `mod.rs` to export the new taxonomy module:

```rust
// shared/src/orphan-detector/mod.rs
pub mod taxonomy_orphan_vo;  // ← Add this line
pub mod taxonomy_analysis_vo;
pub mod contract_orphan_protocol;
```

### Step 5: Update Imports in Layer Files

Replace local dataclass definitions with imports from taxonomy:

```rust
// BEFORE (BAD): Local dataclass
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}

// AFTER (GOOD): Import from taxonomy
use shared::orphan_detector::taxonomy_orphan_vo::OrphanResult;
```

### Step 6: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] **All dataclasses in shared/taxonomy** — no structs/enums with data defined in layer files.
- [ ] **Taxonomy file naming follows strict suffixes** — `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`.
- [ ] **Taxonomy files import only from taxonomy** — no imports from capabilities, infrastructure, agents, contracts, or surface.
- [ ] **Utility functions in `*_utility.rs`** — free functions (no `&self`) extracted to standalone modules.
- [ ] **Layer files import dataclasses from taxonomy** — not defined locally.
- [ ] **Domain's `mod.rs` exports new taxonomy modules** — `pub mod taxonomy_<name>`.
- [ ] **Value Objects have `new()`, `value()`, `Display`, `From<T>` implementations**.
- [ ] **Error types use `thiserror::Error`** — with proper `Display` implementation.
- [ ] **Constants are pure static values** — no external imports, no functions.
- [ ] `cargo check -p shared` passes without warnings or errors.

## Quick Commands

```bash
# Find dataclasses defined in layer files (not in shared/taxonomy)
grep -rn "^pub struct" crates/<crate>/src/ | grep -v "shared/" | grep -v "impl\|trait\|fn "

# Check for forbidden imports in taxonomy files
grep -n "use crate::capabilities_\|use crate::infrastructure_\|use crate::agent_" crates/shared/src/*/taxonomy_*.rs

# Find layer files with concrete type fields (non-DI) that might need taxonomy dataclasses
grep -rn "^\s*[a-z_]*:" crates/<crate>/src/ | grep -v "Arc<dyn" | grep -v "shared/"

# Verify taxonomy module exports are registered
grep -n "^pub mod taxonomy_" crates/shared/src/*/mod.rs

# Check for unregistered taxonomy files (exist on disk but not in mod.rs)
ls crates/shared/src/<domain>/taxonomy_*.rs | while read f; do
    basename=$(basename "$f" .rs)
    grep -q "pub mod $basename" crates/shared/src/<domain>/mod.rs || echo "UNREGISTERED: $basename"
done

# Check for dataclasses in layer files that should be moved to taxonomy
grep -rn "^pub struct" crates/<crate>/src/ | grep -v "shared/" | grep -v "impl\|trait" | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    echo "POSSIBLE DATACLASS: $file has $struct"
done

# Find magic constants in layer files (should be in taxonomy_constant)
rg "[0-9]+\.[0-9]+|#[0-9A-Fa-f]+" crates/<crate>/src/ | grep -v "shared/" | grep -v "// " | head -20
```

## Naming Convention (from fix-naming)

**All Layer File Naming:**

| Layer              | Pattern                  | Suffix                             |
| ------------------ | ------------------------ | ---------------------------------- |
| **root**           | `root_*_container.rs`    | `_container`                       |
| **taxonomy**       | `taxonomy_*_vo.rs`       | `_vo`, `_constant`                 |
| **contract**       | `contract_*_protocol.rs` | `_protocol`, `_port`, `_aggregate` |
| **capabilities**   | `capabilities_*.rs`      | flexible                           |
| **infrastructure** | `infrastructure_*.rs`    | flexible                           |
| **agent**          | `agent_*.rs`             | `_orchestrator`                    |
| **surface**        | `surface_*.rs`           | `_command`, `_controller`          |

## Primitive-to-VO Patterns (from fix-primitive-to-vo)

**Taxonomy Layer VO Creation Rules:**

- Entity fields MUST use VOs, not primitives (`String`, `i32`, `f64`, `bool`)
- Contract signatures MUST use VOs
- VOs MUST validate on construction
- Use macros for simple wrappers: `string_value_object!`, `primitive_value_object!`

```rust
// BEFORE (primitive in layer file)
pub struct LintResult {
    pub file_path: String,   // ← primitive
    pub line: u32,           // ← primitive
    pub severity: String,    // ← primitive
}

// AFTER (VO in taxonomy)
// crates/shared/src/import-rules/taxonomy_file_path_vo.rs
pub struct FilePath(String);

impl FilePath {
    pub fn new(path: String) -> Self { Self(path) }
    pub fn value(&self) -> &str { &self.0 }
}

// crates/shared/src/import-rules/taxonomy_line_number_vo.rs
string_value_object!(LineNumber);  // wraps String/number

// crates/shared/src/import-rules/taxonomy_severity_vo.rs
primitive_value_object!(Severity);  // wraps u32

pub struct LintResult {
    pub file_path: FilePath,   // ← VO
    pub line: LineNumber,      // ← VO
    pub severity: Severity,    // ← VO
}
```

## Magic Constant Definitions (from fix-magic-constant)

**Taxonomy Layer Constant Rules:**

- All domain values live in `taxonomy_*_constant.rs` files
- Constants are static compile-time values — no functions, no imports
- Used by agent, capabilities, and infrastructure layers

```rust
// crates/shared/src/animator/taxonomy_animator_constant.rs
/// Default frames per second for animation
pub const FPS_DEFAULT: f64 = 24.0;

/// Minimum reveal time in seconds
pub const MIN_REVEAL_SECONDS: f64 = 0.5;

/// Manifest filename constant
pub const MANIFEST_FILENAME: &str = "manifest.json";
```

**Layer consumption:**

```rust
// Agent layer
use crate::taxonomy_animator_constant::FPS_DEFAULT;
let result = self.process(fps: FPS_DEFAULT);

// Capabilities layer
use crate::taxonomy_animator_constant::MIN_REVEAL_SECONDS;
fn calculate_duration(&self) -> f64 { MIN_REVEAL_SECONDS }

// Infrastructure layer
use crate::taxonomy_animator_constant::MANIFEST_FILENAME;
let file = std::fs::File::create(MANIFEST_FILENAME);
```

## Common Mistakes (AVOID)

- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the impl struct belongs in layer files.
- ❌ **Importing non-taxonomy types into taxonomy files**: Taxonomy must remain completely pure — no imports from capabilities, infrastructure, agents, contracts, or surface.
- ❌ **Using wrong suffix for taxonomy files**: Only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed. No other suffixes.
- ❌ **Forgetting to register new taxonomy modules in mod.rs**: Every `taxonomy_*.rs` file must have a corresponding `pub mod` in the domain's `mod.rs`.
- ❌ **Placing utility functions in layer files**: Stateless free functions (no `&self`) MUST be extracted to `*_utility.rs` modules in shared/taxonomy.
- ❌ **Creating multiple dataclasses with different names for the same concept**: Consolidate into a single taxonomy file.
- ❌ **Duplicating taxonomy types across domains**: If a type belongs to multiple domains, put it in `common/` and import from there.

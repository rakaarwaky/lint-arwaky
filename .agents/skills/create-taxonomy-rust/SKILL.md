---
name: create-taxonomy-rust
description: "Create and validate Rust taxonomy layer files in shared taxonomy: VOs, entities, errors, events, constants, and pure reusable utilities. Ensures domain data lives only in shared taxonomy and remains pure."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    taxonomy,
    shared,
    vo,
    entity,
    error,
    event,
    constant,
    utility,
    aes201,
    primitive-to-vo,
  ]
triggers:
  - "create taxonomy rust"
  - "add taxonomy rust"
  - "move dataclass to taxonomy rust"
  - "create vo rust"
  - "create error taxonomy rust"
  - "create constant taxonomy rust"
  - "check taxonomy rust"
  - "audit taxonomy rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - fix-primitive-to-vo
  - fix-magic-constant
---

# create-taxonomy-rust

## Purpose

Create and validate Rust **taxonomy layer** files inside `crates/shared/src/<domain>/`.

Taxonomy is the single source of truth for:

- value objects,
- entities,
- domain errors,
- domain events,
- constants,
- pure reusable utility functions.

No domain data structures may be defined in:

- capabilities,
- infrastructure,
- agents,
- surface,
- root/container layers.

Those layers must import domain data from shared taxonomy.

---

## Definition of Done

A taxonomy change is considered valid when:

1. Domain data structures live in `shared/taxonomy`.
2. Taxonomy file naming uses the allowed strict suffixes.
3. Taxonomy files do not import from capability, infrastructure, agent, surface, or root layers.
4. Taxonomy files contain no I/O and no side effects.
5. Utility functions in `*_utility.rs` are stateless, pure, domain-agnostic, and reusable.
6. Value objects validate on construction.
7. Public domain contracts use VOs instead of raw owned primitives.
8. New taxonomy modules are registered in the relevant `mod.rs`.
9. `cargo check -p shared` passes.

---

## The Fundamental Question

> **“Is this struct/enum a dataclass or an implementor?”**

### Dataclass

A dataclass is a type that carries domain data.

Examples:

- value objects,
- DTOs,
- result objects,
- domain entities,
- domain errors,
- domain events,
- enums representing domain values.

These MUST live in shared taxonomy.

```rust
pub struct OrphanAnalysisResult {
    // domain data
}
```

### Implementor

An implementor is a struct that implements a trait and contains behavior, often with injected dependencies.

Examples:

- `capabilities_*.rs`
- `infrastructure_*.rs`
- `agent_*.rs`

These stay in their layer files.

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}
```

---

## Taxonomy Layer Structure

Use snake_case module directories.

```text
crates/shared/src/
├── lib.rs
├── common/
│   ├── mod.rs
│   ├── taxonomy_*_vo.rs
│   ├── taxonomy_*_error.rs
│   ├── taxonomy_*_constant.rs
│   └── taxonomy_*_utility.rs
│
├── <domain>/
│   ├── mod.rs
│   ├── contract_*_protocol.rs
│   ├── contract_*_port.rs
│   ├── contract_*_aggregate.rs
│   ├── taxonomy_*_vo.rs
│   ├── taxonomy_*_entity.rs
│   ├── taxonomy_*_error.rs
│   ├── taxonomy_*_event.rs
│   ├── taxonomy_*_constant.rs
│   └── taxonomy_*_utility.rs
```

Important:

- `contract_*.rs` files are NOT taxonomy files.
- Contract traits may import taxonomy types.
- Taxonomy files MUST NOT import contract traits.

---

## File Naming Convention

Taxonomy files MUST use strict suffixes.

| Suffix        | Purpose                            | Example                                |
| ------------- | ---------------------------------- | -------------------------------------- |
| `_vo`       | Value objects and value-like enums | `taxonomy_file_path_vo.rs`           |
| `_entity`   | Entities with identity             | `taxonomy_analysis_entity.rs`        |
| `_error`    | Error types                        | `taxonomy_config_error.rs`           |
| `_event`    | Event/message types                | `taxonomy_scan_event.rs`             |
| `_constant` | Static compile-time constants      | `taxonomy_layer_names_constant.rs`   |
| `_utility`  | Stateless pure reusable functions  | `taxonomy_symbol_renamer_utility.rs` |

Allowed taxonomy prefixes:

```text
taxonomy_*_vo.rs
taxonomy_*_entity.rs
taxonomy_*_error.rs
taxonomy_*_event.rs
taxonomy_*_constant.rs
taxonomy_*_utility.rs
```

No other taxonomy suffixes are allowed.

---

## Purity and Import Restrictions (AES201)

Taxonomy must remain pure and stable.

### Allowed Dependencies

| Taxonomy Type | May Import From                              | Must Not Import From                                                |
| ------------- | -------------------------------------------- | ------------------------------------------------------------------- |
| `_vo`       | other taxonomy types, std, serde when needed | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_entity`   | other taxonomy types, std, serde when needed | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_error`    | other taxonomy types, std, thiserror         | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_event`    | other taxonomy types, std, serde when needed | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values                      | external layer imports, I/O, functions                              |
| `_utility`  | taxonomy types, pure std helpers             | capabilities, infrastructure, agents, surface, root, contracts, I/O |

Taxonomy may contain:

- value validation,
- domain invariants inside constructors,
- pure transformations between taxonomy types.

Taxonomy must not contain:

- file I/O,
- network calls,
- database access,
- environment mutation,
- side effects,
- business orchestration,
- use-case logic,
- layer-specific behavior.

---

## Dataclass Patterns

### Value Objects (`_vo.rs`)

A value object should wrap domain values with type safety and validation.

Prefer private inner fields.

Bad:

```rust
pub struct FilePath {
    pub value: String,
}
```

Good:

```rust
use crate::common::taxonomy_validation_error::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ValidationError::empty("FilePath"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

For simple wrappers, macros may be used:

```rust
string_value_object!(SymbolName);
u32_value_object!(LineNumber);
bool_value_object!(OrphanFlag);
```

Macros should still support validation when the domain requires it.

---

### Composite Value Objects

Composite VOs should use other VOs as fields, not raw primitives.

Bad:

```rust
pub struct ImportRuleVO {
    pub pattern: String,
    pub message: String,
}
```

Good:

```rust
use crate::import_rules::taxonomy_rule_pattern_vo::RulePattern;
use crate::import_rules::taxonomy_rule_message_vo::RuleMessage;

#[derive(Debug, Clone, PartialEq)]
pub struct ImportRuleVO {
    pattern: RulePattern,
    message: RuleMessage,
}

impl ImportRuleVO {
    pub fn new(pattern: RulePattern, message: RuleMessage) -> Self {
        Self {
            pattern,
            message,
        }
    }

    pub fn pattern(&self) -> &RulePattern {
        &self.pattern
    }

    pub fn message(&self) -> &RuleMessage {
        &self.message
    }
}
```

---

### Entities (`_entity.rs`)

Entities represent domain objects with identity.

```rust
use crate::code_analysis::taxonomy_symbol_id_vo::SymbolId;
use crate::code_analysis::taxonomy_symbol_name_vo::SymbolName;

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolEntity {
    id: SymbolId,
    name: SymbolName,
}

impl SymbolEntity {
    pub fn new(id: SymbolId, name: SymbolName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &SymbolId {
        &self.id
    }

    pub fn name(&self) -> &SymbolName {
        &self.name
    }
}
```

---

### Error Types (`_error.rs`)

Use `thiserror::Error`.

Prefer VO fields instead of raw public strings.

Bad:

```rust
#[derive(Debug, thiserror::Error)]
#[error("Config error: {key} - {message}")]
pub struct ConfigError {
    pub key: String,
    pub message: String,
}
```

Good:

```rust
use thiserror::Error;

use crate::common::taxonomy_error_message_vo::ErrorMessage;
use crate::config::taxonomy_config_key_vo::ConfigKey;

#[derive(Debug, Error)]
#[error("Config error for {key}: {message}")]
pub struct ConfigError {
    key: ConfigKey,
    message: ErrorMessage,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self { key, message }
    }

    pub fn key(&self) -> &ConfigKey {
        &self.key
    }

    pub fn message(&self) -> &ErrorMessage {
        &self.message
    }
}
```

If an error wraps lower-level errors:

```rust
use thiserror::Error;

use crate::file_system::taxonomy_file_path_vo::FilePath;

#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("Failed to read file: {0}")]
    Io(FilePath, #[source] std::io::Error),

    #[error("File content is invalid: {0}")]
    Validation(FilePath),
}
```

---

### Event Types (`_event.rs`)

Events represent something that happened in the domain.

```rust
use crate::scan::taxonomy_scan_id_vo::ScanId;

#[derive(Debug, Clone, PartialEq)]
pub struct ScanCompletedEvent {
    scan_id: ScanId,
}

impl ScanCompletedEvent {
    pub fn new(scan_id: ScanId) -> Self {
        Self { scan_id }
    }

    pub fn scan_id(&self) -> &ScanId {
        &self.scan_id
    }
}
```

---

### Constants (`_constant.rs`)

Constants are pure static values.

```rust
/// Default frames per second for animation.
pub const FPS_DEFAULT: f64 = 24.0;

/// Minimum reveal time in seconds.
pub const MIN_REVEAL_SECONDS: f64 = 0.5;

/// Manifest filename.
pub const MANIFEST_FILENAME: &str = "manifest.json";
```

Rules:

- no functions,
- no I/O,
- no external layer imports,
- no mutable state.

Constants may be primitive scalars. Consumers should wrap domain-meaningful primitives into VOs when exposing them in public domain contracts.

---

## Utility Functions (`_utility.rs`)

Utility files contain pure reusable tools.

### The Ultimate Boundary

A function belongs in `*_utility.rs` ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no randomness, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Multi-consumer reusable: useful for multiple modules/layers.

---

### Good Utility Example

```rust
// taxonomy_token_utility.rs

pub fn match_whole_token(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }

    let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';

    haystack
        .match_indices(needle)
        .any(|(i, _)| {
            let before_ok = i == 0 || !is_ident_char(haystack.as_bytes()[i - 1]);
            let after_ok = i + needle.len() == haystack.len()
                || !is_ident_char(haystack.as_bytes()[i + needle.len()]);

            before_ok && after_ok
        })
}
```

This is a dumb reusable tool.

---

### Bad Utility: Domain Knowledge

```rust
// BAD: knows AES layer mapping rules
pub fn get_target_layer_from_suffix(suffix: &str) -> &'static str {
    match suffix {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        _ => "unknown",
    }
}
```

This belongs in capabilities as a private helper.

---

### Bad Utility: Single Consumer Only

```rust
// BAD: only used by one checker
pub fn format_import_violation(rule: &ImportRuleVO) -> String {
    format!("Import rule violation: {}", rule.pattern())
}
```

If only one capability uses it, keep it as a private helper in that capability.

---

## Primitive-to-VO Rules

Taxonomy is the layer that provides VO replacements for primitives.

### General Rule

Domain data MUST use VOs, not raw owned primitives.

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
use crate::code_analysis::taxonomy_file_path_vo::FilePath;
use crate::code_analysis::taxonomy_line_number_vo::LineNumber;
use crate::code_analysis::taxonomy_severity_vo::Severity;

pub struct LintResult {
    file_path: FilePath,
    line: LineNumber,
    severity: Severity,
}
```

---

### Primitive Policy

This policy must stay consistent with capabilities and infrastructure skills.

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract return values. Use VO.              |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                                |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                                |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                                |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                                |
| `char`             | Forbidden for domain values. Use VO.                                                |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- file paths,
- symbol names,
- messages,
- line numbers,
- column numbers,
- severity levels,
- durations,
- counts,
- thresholds,
- identifiers.

---

### VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

Good:

```rust
impl LineNumber {
    pub fn new(value: u32) -> Result<Self, ValidationError> {
        if value == 0 {
            return Err(ValidationError::positive("LineNumber"));
        }

        Ok(Self(value))
    }
}
```

If validation cannot fail, a simpler constructor may be used.

---

### Optional and Collection Primitives

Bad:

```rust
pub struct RuleSet {
    pub patterns: Vec<String>,
    pub description: Option<String>,
}
```

Good:

```rust
pub struct RuleSet {
    patterns: PatternList,
    description: Option<RuleDescription>,
}
```

Use:

- list VOs for collections,
- optional VOs or `Option<VO>` when semantically optional.

---

## Detection Patterns

### BAD: Dataclass Defined in Capabilities

```rust
// capabilities_orphan_analyzer.rs

pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

Fix:

Move to taxonomy.

```rust
// shared/orphan_detector/taxonomy_orphan_result_vo.rs
pub struct OrphanResult {
    is_orphan: OrphanFlag,
    reason: OrphanReason,
}
```

Then import:

```rust
use shared::orphan_detector::taxonomy_orphan_result_vo::OrphanResult;
```

---

### BAD: Dataclass Defined in Infrastructure

```rust
// infrastructure_file_cache.rs

pub struct CacheEntry {
    key: String,
    value: String,
}
```

Fix:

```rust
// shared/cache/taxonomy_cache_entry_vo.rs
pub struct CacheEntry {
    key: CacheKey,
    value: CacheValue,
}
```

---

### BAD: Raw Primitive Fields in Taxonomy VO

```rust
pub struct ImportRuleVO {
    pub pattern: String,
    pub message: String,
}
```

Fix:

```rust
pub struct ImportRuleVO {
    pattern: RulePattern,
    message: RuleMessage,
}
```

---

### BAD: Taxonomy Importing Layer Code

```rust
// taxonomy_orphan_vo.rs

use crate::capabilities_orphan_analyzer::OrphanAnalyzer; // BAD
```

Taxonomy must not import from layers.

---

### BAD: Domain Rule Inside Utility

```rust
// taxonomy_layer_utility.rs

pub fn is_port_trait_name(name: &str) -> bool {
    name.ends_with("Port")
}
```

If this knows AES naming conventions or layer rules, it is domain knowledge.

It belongs in capabilities as a helper, not taxonomy utility.

---

### GOOD: Dataclass in Taxonomy + Implementor with DI

```rust
// shared/orphan_detector/taxonomy_orphan_analysis_result_vo.rs
pub struct OrphanAnalysisResult {
    is_orphan: OrphanFlag,
    reason: OrphanReason,
}
```

```rust
// capabilities_orphan_analyzer.rs
use std::sync::Arc;

use shared::orphan_detector::taxonomy_orphan_analysis_result_vo::OrphanAnalysisResult;
use shared::orphan_detector::taxonomy_orphan_filename_extractor_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_orphan_file_cache_port::IOrphanFileCachePort;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}
```

Service dependencies use DI.

Value/result data comes from taxonomy.

---

## Workflow

### Step 1: Identify the Dataclass

When you find a struct or enum in a layer file, ask:

> Is this a dataclass or an implementor?

If it carries domain data:

- result object,
- DTO,
- VO,
- entity,
- error,
- event,
- enum,
- constant,

then move it to taxonomy.

If it implements behavior via trait and uses DI, keep it in the layer file.

---

### Step 2: Determine Taxonomy Domain

Choose the correct domain directory under:

```text
crates/shared/src/<domain>/
```

Examples:

| Domain          | Directory                       | Example Types                         |
| --------------- | ------------------------------- | ------------------------------------- |
| common          | `shared/src/common/`          | cross-domain VOs, errors, utilities   |
| orphan_detector | `shared/src/orphan_detector/` | orphan results, reasons, flags        |
| code_analysis   | `shared/src/code_analysis/`   | analysis results, symbols, violations |
| import_rules    | `shared/src/import_rules/`    | import rules, patterns, messages      |
| naming_rules    | `shared/src/naming_rules/`    | naming violations, patterns           |

If a type is used by multiple domains, put it in `common/`.

---

### Step 3: Create or Update Taxonomy File

Use the correct suffix:

```text
taxonomy_*_vo.rs
taxonomy_*_entity.rs
taxonomy_*_error.rs
taxonomy_*_event.rs
taxonomy_*_constant.rs
taxonomy_*_utility.rs
```

Example:

```bash
mkdir -p crates/shared/src/orphan_detector
touch crates/shared/src/orphan_detector/taxonomy_orphan_result_vo.rs
```

---

### Step 4: Register Module

Update the domain `mod.rs`.

```rust
// shared/src/orphan_detector/mod.rs

pub mod taxonomy_orphan_result_vo;
pub mod taxonomy_orphan_reason_vo;
pub mod contract_orphan_protocol;
pub mod contract_orphan_file_cache_port;
```

---

### Step 5: Update Imports in Layer Files

Before:

```rust
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

After:

```rust
use shared::orphan_detector::taxonomy_orphan_result_vo::OrphanResult;
```

---

### Step 6: Verify Purity

Check that taxonomy files do not import from:

- capabilities,
- infrastructure,
- agents,
- surface,
- root containers,
- contract traits.

Also check that taxonomy utilities do not perform I/O.

---

### Step 7: Verify Primitive-to-VO Compliance

Ensure:

- no public raw `String` domain fields,
- no numeric primitive domain fields,
- VOs validate on construction,
- contract traits use taxonomy VOs.

---

### Step 8: Verify Compilation

```bash
cargo check -p shared
```

---

## Verification Checklist

- [ ] All domain dataclasses live in shared/taxonomy.
- [ ] No domain structs/enums with data are defined in layer files.
- [ ] Taxonomy file naming uses allowed suffixes only.
- [ ] Taxonomy files do not import from capabilities.
- [ ] Taxonomy files do not import from infrastructure.
- [ ] Taxonomy files do not import from agents.
- [ ] Taxonomy files do not import from surface.
- [ ] Taxonomy files do not import from root containers.
- [ ] Taxonomy files do not import contract traits.
- [ ] Taxonomy files contain no I/O.
- [ ] Taxonomy utilities are stateless, pure, domain-agnostic, and multi-consumer.
- [ ] Domain-specific stateless helpers are NOT forced into taxonomy utility.
- [ ] Single-consumer helpers remain in their consuming layer.
- [ ] Value objects validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types use `thiserror::Error`.
- [ ] Constants are pure static values.
- [ ] New taxonomy modules are registered in `mod.rs`.
- [ ] `cargo check -p shared` passes.

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# Find possible dataclasses in layer files
rg -n "^\s*pub struct|^\s*pub enum" crates/<crate>/src --glob '!**/shared/**'

# Check forbidden imports in taxonomy files
rg -n "^\s*use\s+.*(capabilities_|infrastructure_|agent_|surface_)" crates/shared/src/**/taxonomy_*.rs

# Check possible I/O in taxonomy files
rg -n "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/shared/src/**/taxonomy_*.rs

# List registered taxonomy modules
rg -n "^pub mod taxonomy_" crates/shared/src/*/mod.rs

# Find magic constants in layer files
rg "[0-9]+\.[0-9]+" crates/<crate>/src --glob '!**/shared/**'
```

---

### Check Unregistered Taxonomy Files

```bash
for file in crates/shared/src/<domain>/taxonomy_*.rs; do
  basename=$(basename "$file" .rs)

  rg -q "^pub mod $basename;" crates/shared/src/<domain>/mod.rs \
    || echo "UNREGISTERED: $basename"
done
```

---

## Naming Convention

| Layer          | File Pattern                | Suffix                        |
| -------------- | --------------------------- | ----------------------------- |
| root           | `root_*_container.rs`     | `_container`                |
| taxonomy       | `taxonomy_*_vo.rs`        | `_vo`                       |
| taxonomy       | `taxonomy_*_entity.rs`    | `_entity`                   |
| taxonomy       | `taxonomy_*_error.rs`     | `_error`                    |
| taxonomy       | `taxonomy_*_event.rs`     | `_event`                    |
| taxonomy       | `taxonomy_*_constant.rs`  | `_constant`                 |
| taxonomy       | `taxonomy_*_utility.rs`   | `_utility`                  |
| contract       | `contract_*_protocol.rs`  | `_protocol`                 |
| contract       | `contract_*_port.rs`      | `_port`                     |
| contract       | `contract_*_aggregate.rs` | `_aggregate`                |
| capabilities   | `capabilities_*.rs`       | flexible                      |
| infrastructure | `infrastructure_*.rs`     | flexible                      |
| agent          | `agent_*.rs`              | `_orchestrator`             |
| surface        | `surface_*.rs`            | `_command`, `_controller` |

---

## Magic Constant Definitions

All domain constants MUST live in taxonomy constant files.

```rust
// crates/shared/src/animator/taxonomy_animator_constant.rs

/// Default frames per second for animation.
pub const FPS_DEFAULT: f64 = 24.0;

/// Minimum reveal time in seconds.
pub const MIN_REVEAL_SECONDS: f64 = 0.5;

/// Manifest filename.
pub const MANIFEST_FILENAME: &str = "manifest.json";
```

Layer consumption:

```rust
use shared::animator::taxonomy_animator_constant::FPS_DEFAULT;
```

```rust
use shared::animator::taxonomy_animator_constant::MIN_REVEAL_SECONDS;
```

```rust
use shared::animator::taxonomy_animator_constant::MANIFEST_FILENAME;
```

If a constant represents a domain value, wrap it in a VO at the consuming boundary when exposing it through public domain contracts.

---

## Common Mistakes

- ❌ Defining dataclasses in layer files.
- ❌ Defining domain enums in layer files.
- ❌ Importing non-taxonomy layer types into taxonomy files.
- ❌ Importing contract traits into taxonomy files.
- ❌ Using wrong suffix for taxonomy files.
- ❌ Forgetting to register taxonomy modules in `mod.rs`.
- ❌ Putting domain knowledge into `*_utility.rs`.
- ❌ Putting single-consumer helpers into `*_utility.rs`.
- ❌ Keeping reusable domain-agnostic utilities inside layer files.
- ❌ Exposing public raw `String` fields in VOs.
- ❌ Exposing public numeric primitive fields in domain types.
- ❌ Creating VOs without validation when domain invariants exist.
- ❌ Duplicating taxonomy types across domains.
- ❌ Putting cross-domain types in a specific domain instead of `common/`.
- ❌ Creating taxonomy utility functions with I/O.
- ❌ Treating every stateless function as utility.
- ❌ Treating every concrete field as DI violation.
- ❌ Forgetting that value fields may be shared VOs, while service fields must use DI.

```


# Finding: Source Parsing Decoupling — Complete Context for AI Agent

## Problem Statement

The `source-parsing` crate (`crates/source-parsing/`) contains infrastructure implementations (scanners, parsers, language detector) that are injected at runtime into other feature crates. This creates a runtime dependency: if `source-parsing` is deleted, 4 feature crates fail at runtime even though they compile fine (they only depend on contracts from `shared`).

**Goal:** Make each feature crate self-contained with its own infrastructure implementation, eliminating runtime dependency on `source-parsing` crate. After completion, `crates/source-parsing/` can be safely deleted.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  shared/src/source-parsing/                                  │
│  CONTRACTS (Ports) + VALUE OBJECTS                           │
│  • ISourceParserPort, IScannerProviderPort, ILanguageDetectorPort │
│  • FilePath, FilePathList, PrimitiveTypeList, etc.           │
│  (This module stays — it's in shared crate, not source-parsing)│
└─────────────────────────────────────────────────────────────┘
                           │
                           │ implements (NEEDS TO BE MOVED)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  crates/source-parsing/  ← TO BE DELETED                     │
│  INFRASTRUCTURE IMPLEMENTATIONS                              │
│  • ASTRustParserAdapter (782 lines)                         │
│  • ASTPythonParserAdapter (789 lines)                       │
│  • ASTJSParserAdapter (842 lines)                           │
│  • LanguageDetector (111 lines)                             │
│  • SourceParserOrchestrator (175 lines)                     │
│  • PathNormalizationProvider (268 lines)                    │
│  • FileCollectorProvider (in shared, NOT in source-parsing) │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ inject via DI (WILL BREAK)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  Feature Crates that NEED runtime implementations           │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  import-rules    → ISourceParserPort                │    │
│  │  code-analysis   → ISourceParserPort                │    │
│  │  cli-commands    → IScannerProviderPort, ILanguageDetectorPort │
│  │  git-hooks       → IScannerProviderPort             │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                             │
│  Feature Crates that are SAFE (only use Value Objects)      │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  auto-fix, config-system, external-lint,            │    │
│  │  maintenance, naming-rules, orphan-detector,        │    │
│  │  role-rules                                         │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## Current Dependency Chain (What Breaks)

### 1. Entry Points that use `source_parsing` directly

**`crates/root_cli_main_entry.rs`** (lines 20-21, 79-80, 128, 145, 207, 210):
```rust
let source_parsing_container =
    source_parsing::root_source_parsing_container::SourceParsingContainer::new();
let source_parser = source_parsing_container.source_parser();
// ...
scanner_provider: source_parsing_container.scanner_provider(),
language_detector: source_parsing_container.language_detector(),
```

**`crates/root_mcp_main_entry.rs`** (lines 15-17):
```rust
let source_parsing_container =
    source_parsing::root_source_parsing_container::SourceParsingContainer::new();
let source_parser = source_parsing_container.source_parser();
```

### 2. Feature Crates that use Ports (need implementations at runtime)

#### `import-rules` — needs `ISourceParserPort`

**`crates/import-rules/src/root_import_rules_container.rs`:**
```rust
use shared::source_parsing::contract_parser_port::ISourceParserPort;

pub struct ImportContainer {
    source_parser: Arc<dyn ISourceParserPort>,
}

impl ImportContainer {
    pub fn new(source_parser: Arc<dyn ISourceParserPort>) -> Self {
        Self { source_parser }
    }
    pub fn analyzer(&self) -> Arc<dyn IImportAnalyzerProtocol> { ... }
    pub fn orchestrator(&self) -> Arc<dyn IImportOrchestratorAggregate> { ... }
}
```

**`crates/import-rules/src/capabilities_layer_detection_analyzer.rs`:**
```rust
use shared::source_parsing::contract_parser_port::ISourceParserPort;

pub struct LayerDetectionAnalyzer {
    pub parser: Arc<dyn ISourceParserPort>,
}
```

**Files using `shared::source_parsing::*`:**
- `capabilities_dummy_import_checker.rs`
- `infrastructure_import_parser_adapter.rs`
- `capabilities_cycle_import_analyzer.rs`
- `infrastructure_filesystem_adapter.rs`
- `agent_import_orchestrator.rs`
- `capabilities_import_mandatory_checker.rs`
- `capabilities_import_forbidden_checker.rs`
- `capabilities_layer_detection_analyzer.rs`

---

#### `code-analysis` — needs `ISourceParserPort`

**`crates/code-analysis/src/root_code_analysis_container.rs`:**
```rust
impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    // Already has a null/stub implementation!
}

fn parser(&self) -> &dyn shared::source_parsing::contract_parser_port::ISourceParserPort { ... }
```

**`crates/code-analysis/src/capabilities_check_bypass_checker.rs`:**
```rust
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;
// Uses LanguageDetector helper directly (not via port)
```

**`crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`:**
```rust
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;
```

**`crates/code-analysis/src/agent_code_analysis_orchestrator.rs`:**
```rust
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};
shared::source_parsing::taxonomy_file_collector_helper::collect_source_files(...)
```

---

#### `cli-commands` — needs `IScannerProviderPort`, `ILanguageDetectorPort`

**`crates/cli-commands/src/surface_check_command.rs`:**
```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;

pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisLinterAggregate>,
    pub import_orchestrator: Arc<dyn IImportOrchestratorAggregate>,
    pub naming_orchestrator: Arc<dyn INamingOrchestratorAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleOrchestratorAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAnalyzerAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectorProtocol>,
    pub language_detector: Arc<dyn ILanguageDetectorPort>,
}
```

**`crates/cli-commands/src/surface_git_command.rs`:**
```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
// Uses is_lintable() to filter git diff files
```

---

#### `git-hooks` — needs `IScannerProviderPort`

**`crates/git-hooks/src/root_git_hooks_container.rs`:**
```rust
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;

pub struct GitContainer {
    scanner: Arc<dyn IScannerProviderPort>,
    // ...
}

impl GitContainer {
    pub fn new(scanner: Arc<dyn IScannerProviderPort>) -> Self { ... }
    pub fn new_default() -> Self {
        let scanner: Arc<dyn IScannerProviderPort> =
            Arc::new(shared::source_parsing::FileCollectorProvider::new());
        // ...
    }
}
```

**`crates/git-hooks/src/capabilities_diff_checker.rs`:**
```rust
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;

pub struct DiffChecker {
    _scanner: Arc<dyn IScannerProviderPort>,
}
```

---

## Source Code: Infrastructure to be Moved/Replicated

### A. `LanguageDetector` (111 lines) — needed by ALL 4 crates

**File:** `crates/source-parsing/src/infrastructure_language_detector.rs`

```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_language_detector_port::Language;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct LanguageDetector;

impl LanguageDetector {
    pub fn new() -> Self { Self }
}

impl Default for LanguageDetector {
    fn default() -> Self { Self::new() }
}

impl ILanguageDetectorPort for LanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        let ext = path.extension();
        match ext.as_str() {
            "py" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "rs" => Language::Rust,
            _ => Language::Unknown,
        }
    }
}
```

**Note:** This is simple enough to copy into each crate that needs it. Alternative: keep ONE copy in `shared` crate (already has `taxonomy_language_detector_helper`).

---

### B. `FileCollectorProvider` (122 lines) — already in shared!

**File:** `crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs`

This is ALREADY in the `shared` crate, not in `source-parsing`. It implements `IScannerProviderPort`.

**Key:** `git-hooks` already uses `shared::source_parsing::FileCollectorProvider::new()` in `new_default()`. No change needed for git-hooks.

---

### C. `SourceParserOrchestrator` (175 lines) — routes to language-specific parsers

**File:** `crates/source-parsing/src/infrastructure_parser_adapter.rs`

```rust
pub struct SourceParserOrchestrator {
    python_parser: Box<dyn ISourceParserPort>,
    rust_parser: Box<dyn ISourceParserPort>,
    js_parser: Box<dyn ISourceParserPort>,
    language_detector: Box<dyn ILanguageDetectorPort>,
}

impl SourceParserOrchestrator {
    pub fn new(
        python_parser: Box<dyn ISourceParserPort>,
        rust_parser: Box<dyn ISourceParserPort>,
        js_parser: Box<dyn ISourceParserPort>,
        language_detector: Box<dyn ILanguageDetectorPort>,
    ) -> Self { ... }

    fn select_parser(&self, path: &FilePath) -> &dyn ISourceParserPort {
        match self.language_detector.detect(path) {
            Language::Rust => &*self.rust_parser,
            Language::JavaScript | Language::TypeScript => &*self.js_parser,
            _ => &*self.python_parser,
        }
    }
}

impl ISourceParserPort for SourceParserOrchestrator {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        self.select_parser(path).extract_imports(path)
    }
    // ... delegates all methods to selected parser
}
```

---

### D. `ASTRustParserAdapter` (782 lines)

**File:** `crates/source-parsing/src/infrastructure_rust_scanner.rs`

Full Rust file parser using regex-based AST extraction. Implements `ISourceParserPort`.

---

### E. `ASTPythonParserAdapter` (789 lines)

**File:** `crates/source-parsing/src/infrastructure_py_scanner.rs`

Full Python file parser using regex-based AST extraction. Implements `ISourceParserPort`.

---

### F. `ASTJSParserAdapter` (842 lines)

**File:** `crates/source-parsing/src/infrastructure_js_scanner.rs`

Full JavaScript/TypeScript file parser using regex-based AST extraction. Implements `ISourceParserPort`.

---

### G. `PathNormalizationProvider` (268 lines)

**File:** `crates/source-parsing/src/infrastructure_path_provider.rs`

Path normalization with phantom root handling. Implements `IPathNormalizationPort`.

**Used by:** `cli-commands` (via `external_lint_container.path_normalization()`)

---

## Contracts (Ports) — DO NOT MODIFY, stay in shared

### `ISourceParserPort` (37 lines)
**File:** `crates/shared/src/source-parsing/contract_parser_port.rs`

```rust
pub trait ISourceParserPort: Send + Sync {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError>;
    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError>;
    fn get_class_attributes(&self, path: &FilePath) -> ResponseData;
    fn has_all_export(&self, path: &FilePath) -> SuccessStatus;
    fn find_primitive_violations(&self, path: &FilePath, primitive_types: &PrimitiveTypeList) -> PrimitiveViolationList;
    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList;
    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError>;
    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO;
    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus;
    fn get_class_methods(&self, path: &FilePath) -> MetadataVO;
    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO;
    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO;
    fn get_control_flow_count(&self, path: &FilePath) -> Count;
    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO;
    fn get_stem(&self, path: &FilePath) -> SymbolName;
    fn is_entry_point(&self, path: &FilePath) -> BooleanVO;
    fn get_supported_extensions(&self) -> PatternList;
}
```

### `IScannerProviderPort` (10 lines)
**File:** `crates/shared/src/source-parsing/contract_scanner_provider_port.rs`

```rust
pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
```

### `ILanguageDetectorPort` (47 lines)
**File:** `crates/shared/src/source_parsing/contract_language_detector_port.rs`

```rust
pub trait ILanguageDetectorPort: Send + Sync {
    fn detect(&self, path: &FilePath) -> Language;
    fn is_language(&self, path: &FilePath, lang: Language) -> bool { ... }
    fn is_lintable(&self, path: &FilePath) -> bool { ... }
}
```

---

## Workspace Cargo.toml References

**File:** `Cargo.toml` (root)

```toml
[workspace.dependencies]
source_parsing = { package = "source_parsing-lint-arwaky", path = "crates/source-parsing" }
# ... other deps

[dependencies]
source_parsing.workspace = true
# ... other deps
```

**Line 22:** workspace dependency definition
**Line 94:** root crate dependency

---

## Implementation Strategy

### Phase 1: Create Local Infrastructure Modules (per crate)

Each affected crate gets its own infrastructure files:

| Crate | New Files | Source to Copy From |
|-------|-----------|---------------------|
| import-rules | `infrastructure_language_detector.rs` | `source-parsing/src/infrastructure_language_detector.rs` |
| code-analysis | `infrastructure_language_detector.rs` | Same |
| cli-commands | `infrastructure_language_detector.rs`, `infrastructure_scanner_provider.rs` | LanguageDetector + FileCollectorProvider |
| git-hooks | (already has `new_default()` using `shared::FileCollectorProvider`) | No new files needed |

### Phase 2: Modify Containers to Provide Default Constructors

Each crate's container gets a `new_default()` method that creates its own infrastructure:

```rust
// Example: import-rules
impl ImportContainer {
    pub fn new_default() -> Self {
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
            // Option A: Create local Rust parser
            // Option B: Use NullSourceParser (like code-analysis does)
        );
        Self { source_parser }
    }
}
```

### Phase 3: Update Entry Points

**`root_cli_main_entry.rs`** — Remove `source_parsing::` references, use `new_default()` constructors.

**`root_mcp_main_entry.rs`** — Same.

### Phase 4: Remove `source-parsing` Crate

1. Delete `crates/source-parsing/` directory
2. Remove line 22 from `Cargo.toml`: `source_parsing = { ... }`
3. Remove line 94 from `Cargo.toml`: `source_parsing.workspace = true`

---

## Build & Test Commands

```bash
# Build everything
cargo build --release

# Self-lint
cargo run --bin lint-arwaky-cli -- check

# Scan
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Per-crate build/check/test
cargo build -p import_rules_lint_arwaky
cargo check -p naming_rules_lint_arwaky
cargo test -p code_analysis_lint_arwaky

# Tests
cargo test --workspace

# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Code duplication | Medium | Keep shared contracts in `shared`; only duplicate small infrastructure |
| Breaking DI pattern | Low | Maintain `new()` for DI, add `new_default()` for self-contained |
| Missing features | High | Copy FULL implementations, not simplified versions |
| Breaking existing tests | Medium | Run `cargo test --workspace` after each phase |
| Entry point compilation | High | Update all 3 entry points (CLI, MCP, TUI) |

---

## Files to Touch (Complete List)

### New Files to Create
- `crates/import-rules/src/infrastructure_language_detector.rs`
- `crates/code-analysis/src/infrastructure_language_detector.rs`
- `crates/cli-commands/src/infrastructure_language_detector.rs`
- `crates/cli-commands/src/infrastructure_scanner_provider.rs`

### Files to Modify
- `crates/import-rules/src/root_import_rules_container.rs` — add `new_default()`
- `crates/code-analysis/src/root_code_analysis_container.rs` — add `new_default()`
- `crates/cli-commands/src/surface_check_command.rs` — add `new_default()` to `CheckContext`
- `crates/cli-commands/src/surface_git_command.rs` — use local detector
- `crates/git-hooks/src/root_git_hooks_container.rs` — already has `new_default()`, verify
- `crates/root_cli_main_entry.rs` — remove `source_parsing::` usage
- `crates/root_mcp_main_entry.rs` — remove `source_parsing::` usage
- `Cargo.toml` (root) — remove lines 22 and 94

### Files to Delete (Phase 4)
- `crates/source-parsing/` (entire directory)

---

## Testing Checklist

- [ ] `cargo build --release` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `lint-arwaky-cli scan test-workspaces` produces same output as before
- [ ] `lint-arwaky-cli check` produces same output as before
- [ ] Each affected crate builds independently: `cargo build -p import_rules_lint-arwaky`
- [ ] Each affected crate tests independently: `cargo test -p import_rules_lint_arwaky`

---

## Notes

1. **`code-analysis` already has `NullSourceParser`** — a stub implementation of `ISourceParserPort`. This can be used as a fallback if full parsing is not needed.

2. **`git-hooks` already uses `shared::source_parsing::FileCollectorProvider::new()`** in its `new_default()` method. This is already self-contained.

3. **`shared/src/source-parsing/infrastructure_file_collector_provider.rs`** — The `FileCollectorProvider` is already in the `shared` crate, not in `source-parsing`. This implements `IScannerProviderPort` and can be used by `cli-commands` and `git-hooks`.

4. **The 3 scanner implementations (Rust, Python, JS) are ~2400 lines total.** These can be copied to `import-rules` (the only crate that needs full `ISourceParserPort` implementation). Other crates can use stubs.

5. **`cli-commands` needs `IPathNormalizationPort`** — currently provided by `source_parsing_container.path_normalization()`. This is used by `external_lint_container`. May need to create local `PathNormalizationProvider` or move to `shared`.

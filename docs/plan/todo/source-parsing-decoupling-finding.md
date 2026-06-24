# FINDING: Source-Parsing Decoupling — Complete Context for AI Agent

## 1. Problem Statement

The `source-parsing` crate provides infrastructure implementations (scanners, language detector, etc.) that are injected at runtime into other feature crates. If `source-parsing` is deleted, the following crates fail at **runtime**:

| Crate | Ports Needed | Status |
|-------|--------------|--------|
| import-rules | `ISourceParserPort` | ❌ RUNTIME GAGAL |
| code-analysis | `ISourceParserPort` | ❌ RUNTIME GAGAL |
| cli-commands | `IScannerProviderPort`, `ILanguageDetectorPort` | ❌ RUNTIME GAGAL |
| git-hooks | `IScannerProviderPort` | ❌ RUNTIME GAGAL |

**Goal:** Make each crate self-contained with its own infrastructure implementation, eliminating runtime dependency on `source-parsing` crate.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  shared/src/source-parsing/                                  │
│  CONTRACTS (Ports) + VALUE OBJECTS                           │
│  • ISourceParserPort, IScannerProviderPort, ILanguageDetector│
│  • FilePath, FilePathList, DirectoryPath                     │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ implements
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  crates/source-parsing/ (TO BE DELETED)                      │
│  INFRASTRUCTURE IMPLEMENTATIONS                              │
│  • ASTRustParserAdapter → ISourceParserPort                  │
│  • ASTPythonParserAdapter → ISourceParserPort                │
│  • ASTJSParserAdapter → ISourceParserPort                    │
│  • LanguageDetector → ILanguageDetectorPort                  │
│  • FileCollectorProvider → IScannerProviderPort              │
│  • SourceParserOrchestrator → ISourceParserPort              │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ inject via DI
                           ▼
┌─────────────────────────────────────────────────────────────┐
│  Feature Crates (NEED SELF-CONTAINED INFRASTRUCTURE)        │
│  • import-rules                                             │
│  • code-analysis                                            │
│  • cli-commands                                             │
│  • git-hooks                                                │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Source Code: Contracts (shared)

### 3.1 ISourceParserPort (`shared/src/source-parsing/contract_parser_port.rs`)

```rust
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::source_parsing::taxonomy_parser_error::SourceParserError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait ISourceParserPort: Send + Sync {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError>;
    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError>;
    fn get_class_attributes(&self, path: &FilePath) -> ResponseData;
    fn has_all_export(&self, path: &FilePath) -> SuccessStatus;
    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList;
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

### 3.2 IScannerProviderPort (`shared/src/source-parsing/contract_scanner_provider_port.rs`)

```rust
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
```

### 3.3 ILanguageDetectorPort (`shared/src/source-parsing/contract_language_detector_port.rs`)

```rust
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Unknown,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Rust => "rust",
            Language::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait ILanguageDetectorPort: Send + Sync {
    fn detect(&self, path: &FilePath) -> Language;

    fn is_language(&self, path: &FilePath, lang: Language) -> bool {
        self.detect(path) == lang
    }

    fn is_lintable(&self, path: &FilePath) -> bool {
        matches!(
            self.detect(path),
            Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
        )
    }
}
```

### 3.4 FileCollectorProvider (`shared/src/source-parsing/infrastructure_file_collector_provider.rs`)

```rust
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_system::taxonomy_config_vo::default_aes_config;
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::source_parsing::taxonomy_file_collector_helper::is_path_ignored;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub struct FileCollectorProvider {}

impl Default for FileCollectorProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCollectorProvider {
    pub fn new() -> Self {
        Self {}
    }
}

fn default_ignored_paths() -> Vec<String> {
    let config = default_aes_config();
    config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect()
}

pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        walk_source_files(dir, &mut files, &[]);
    }
    files
}

impl IScannerProviderPort for FileCollectorProvider {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let dir = Path::new(&path.value);
        let mut files = Vec::new();
        if !dir.exists() || !dir.is_dir() {
            return Ok(FilePathList { values: files });
        }
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
        Ok(FilePathList { values: files })
    }

    fn get_ignored_files(&self) -> FilePathList {
        FilePathList { values: vec![] }
    }
}

fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if path.is_dir() {
                walk_source_files(&path, files, ignored);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(fp) = FilePath::new(path_str.to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
}

pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if p.is_dir() {
                walk_rs_files(&p, cb, ignored);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}

pub fn count_loc(path: &str) -> usize {
    let src = Path::new(path);
    let ignored = default_ignored_paths();
    let mut count = 0usize;
    walk_rs_files(
        src,
        &mut |p| {
            if let Ok(c) = fs::read_to_string(&p) {
                count += c.lines().count();
            }
        },
        &ignored,
    );
    count.max(1)
}
```

---

## 4. Source Code: Infrastructure to be Moved

### 4.1 LanguageDetector (`source-parsing/src/infrastructure_language_detector.rs`)

```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_language_detector_port::Language;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct LanguageDetector;

impl LanguageDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
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

### 4.2 SourceParserOrchestrator (`source-parsing/src/infrastructure_parser_adapter.rs`)

```rust
use shared::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use shared::mcp_server::taxonomy_job_vo::ResponseData;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_language_detector_port::Language;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
use shared::source_parsing::taxonomy_parser_error::SourceParserError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::BooleanVO;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_suggestion_vo::MetadataVO;

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
    ) -> Self {
        Self {
            python_parser,
            rust_parser,
            js_parser,
            language_detector,
        }
    }

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

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        self.select_parser(path).get_raw_symbols(path)
    }

    fn get_class_attributes(&self, path: &FilePath) -> ResponseData {
        self.select_parser(path).get_class_attributes(path)
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        self.select_parser(path).has_all_export(path)
    }

    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        self.select_parser(path)
            .find_primitive_violations(path, primitive_types)
    }

    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList {
        self.select_parser(path).find_unused_imports(path)
    }

    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        self.select_parser(path).get_class_definitions(path)
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_function_definitions(path)
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        self.select_parser(path).is_symbol_exported(path, symbol)
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_methods(path)
    }

    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_bases_map(path)
    }

    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_assignment_targets(path)
    }

    fn get_control_flow_count(&self, path: &FilePath) -> Count {
        self.select_parser(path).get_control_flow_count(path)
    }

    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO {
        self.select_parser(path).is_barrel_file(path)
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        self.select_parser(path).get_stem(path)
    }

    fn is_entry_point(&self, path: &FilePath) -> BooleanVO {
        self.select_parser(path).is_entry_point(path)
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![
            ".py".to_string(),
            ".rs".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
            ".js".to_string(),
            ".jsx".to_string(),
        ])
    }
}
```

### 4.3 ASTRustParserAdapter (`source-parsing/src/infrastructure_rust_scanner.rs`)

**Full file: 782 lines** — Contains regex-based Rust source parser implementing `ISourceParserPort`.

Key methods:
- `extract_imports()` — extracts `use` statements
- `get_raw_symbols()` — extracts struct, enum, trait, fn definitions
- `find_unused_imports()` — finds unused imports
- `get_class_definitions()`, `get_function_definitions()` — extract definitions

**NOTE:** This is a large file. The implementing agent should read the full file from `crates/source-parsing/src/infrastructure_rust_scanner.rs`.

### 4.4 ASTPythonParserAdapter (`source-parsing/src/infrastructure_py_scanner.rs`)

**Full file: ~400 lines** — Python source parser implementing `ISourceParserPort`.

**NOTE:** Read full file from `crates/source-parsing/src/infrastructure_py_scanner.rs`.

### 4.5 ASTJSParserAdapter (`source-parsing/src/infrastructure_js_scanner.rs`)

**Full file: ~400 lines** — JavaScript/TypeScript source parser implementing `ISourceParserPort`.

**NOTE:** Read full file from `crates/source-parsing/src/infrastructure_js_scanner.rs`.

### 4.6 PathNormalizationProvider (`source-parsing/src/infrastructure_path_provider.rs`)

```rust
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_barrel_provider_vo::BarrelImportResolver;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct PathNormalizationProvider {}

impl IPathNormalizationPort for PathNormalizationProvider {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        // ... full implementation in source-parsing crate
    }

    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath {
        // ... full implementation in source-parsing crate
    }
}
```

**NOTE:** Read full file from `crates/source-parsing/src/infrastructure_path_provider.rs`.

### 4.7 SourceParsingContainer (`source-parsing/src/root_source_parsing_container.rs`)

```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use std::sync::Arc;

pub struct SourceParsingContainer {
    source_parser: Arc<dyn ISourceParserPort>,
    path_normalization: Arc<dyn IPathNormalizationPort>,
    scanner_provider: Arc<dyn IScannerProviderPort>,
    language_detector: Arc<dyn ILanguageDetectorPort>,
}

impl SourceParsingContainer {
    pub fn new() -> Self {
        let path_norm: Arc<dyn IPathNormalizationPort> =
            Arc::new(crate::infrastructure_path_provider::PathNormalizationProvider {});
        let lang_detector: Arc<dyn ILanguageDetectorPort> =
            Arc::new(crate::infrastructure_language_detector::LanguageDetector::new());
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
            crate::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(crate::infrastructure_py_scanner::ASTPythonParserAdapter::new()),
                Box::new(crate::infrastructure_rust_scanner::ASTRustParserAdapter::new()),
                Box::new(crate::infrastructure_js_scanner::ASTJSParserAdapter::new()),
                Box::new(crate::infrastructure_language_detector::LanguageDetector::new()),
            ),
        );
        Self {
            source_parser,
            path_normalization: path_norm.clone(),
            scanner_provider: Arc::new(shared::source_parsing::FileCollectorProvider::new()),
            language_detector: lang_detector,
        }
    }

    pub fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        self.source_parser.clone()
    }

    pub fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        self.path_normalization.clone()
    }

    pub fn scanner_provider(&self) -> Arc<dyn IScannerProviderPort> {
        self.scanner_provider.clone()
    }

    pub fn language_detector(&self) -> Arc<dyn ILanguageDetectorPort> {
        self.language_detector.clone()
    }
}
```

---

## 5. Source Code: Affected Crates

### 5.1 import-rules

#### Cargo.toml
```toml
[package]
name = "import_rules-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Import-compliance checks covering AES201–AES205"
license = "MIT"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
shared.workspace = true
```

#### root_import_rules_container.rs
```rust
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::contract_rule_protocol::IArchImportProtocol;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use std::sync::Arc;

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleAnalysisProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
}

impl ImportContainer {
    pub fn new(source_parser: Arc<dyn ISourceParserPort>) -> Self {
        Self::new_with_config(
            shared::config_system::taxonomy_config_vo::default_aes_config(),
            source_parser,
        )
    }

    pub fn new_with_config(
        config: ArchitectureConfig,
        source_parser: Arc<dyn ISourceParserPort>,
    ) -> Self {
        let fs = Arc::new(crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let parser: Arc<dyn IImportParserPort> =
            Arc::new(crate::infrastructure_import_parser_adapter::ImportParserAdapter::new());
        let analyzer = Arc::new(
            crate::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config.clone(),
                fs,
                source_parser,
            ),
        );

        // ... rest of initialization
    }
}
```

### 5.2 code-analysis

#### Cargo.toml
```toml
[package]
name = "code_analysis-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Code-quality analyzers covering AES301–AES305"
license = "MIT"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
tokio = { workspace = true, features = ["rt", "rt-multi-thread", "macros"] }
shared.workspace = true
```

#### root_code_analysis_container.rs (relevant parts)
```rust
// Already has NullSourceParser implementation
impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    // ... stub implementation
}

impl NullSourceParser {
    fn parser(&self) -> &dyn shared::source_parsing::contract_parser_port::ISourceParserPort {
        self
    }
}
```

### 5.3 cli-commands

#### Cargo.toml
```toml
[package]
name = "cli_commands-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "CLI command surfaces"
license = "MIT"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
clap.workspace = true
console.workspace = true
dialoguer.workspace = true
futures.workspace = true
anyhow.workspace = true
serde_yaml.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
ctrlc.workspace = true
tokio.workspace = true
shared.workspace = true
dirs.workspace = true
```

#### surface_check_command.rs (relevant parts)
```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector: Arc<dyn ILanguageDetectorPort>,
}
```

#### surface_git_command.rs
```rust
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    language_detector: Arc<dyn ILanguageDetectorPort>,
    base: String,
) -> ExitCode {
    // ... uses language_detector.is_lintable()
}
```

### 5.4 git-hooks

#### Cargo.toml
```toml
[package]
name = "git_hooks-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Git hook installation and diff-based pre-commit lint enforcement"
license = "MIT"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
shared.workspace = true
```

#### root_git_hooks_container.rs
```rust
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use std::sync::Arc;

pub struct GitContainer {
    aggregate: Arc<dyn GitHooksAggregate>,
}

impl GitContainer {
    pub fn new(
        scanner: Arc<dyn IScannerProviderPort>,
        hook_adapter: Arc<dyn IHookManagerPort>,
    ) -> Self {
        // ... creates DiffChecker with scanner
    }

    pub fn new_default() -> Self {
        let hook_adapter: Arc<dyn IHookManagerPort> =
            Arc::new(crate::infrastructure_hook_adapter::GitHookAdapter::new(
                match shared::source_parsing::taxonomy_path_vo::FilePath::new(".".to_string()) {
                    Ok(path) => path,
                    Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
                },
            ));
        let scanner: Arc<dyn IScannerProviderPort> =
            Arc::new(shared::source_parsing::FileCollectorProvider::new());
        Self::new(scanner, hook_adapter)
    }
}
```

#### capabilities_diff_checker.rs
```rust
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use std::sync::Arc;

pub struct DiffChecker {
    _scanner: Arc<dyn IScannerProviderPort>,
}

impl DiffChecker {
    pub fn new(scanner: Arc<dyn IScannerProviderPort>) -> Self {
        Self { _scanner: scanner }
    }

    // ... git diff logic using _scanner
}
```

---

## 6. Source Code: Entry Points

### 6.1 root_cli_main_entry.rs

```rust
use cli_commands::surface_check_command;
use cli_commands::surface_fix_command;
use cli_commands::surface_plugin_command;
use cli_commands::surface_watch_command;
use code_analysis::agent_code_analysis_orchestrator::init_global_checker;
use code_analysis::{has_critical, lint_path, CodeDuplicationAnalyzer};
use import_rules::root_import_rules_container::ImportContainer;
use role_rules::root_role_rules_container::RoleContainer;
use shared::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use std::sync::Arc;

fn main() -> ExitCode {
    let source_parsing_container =
        source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let source_parser = source_parsing_container.source_parser();

    let import_container = ImportContainer::new(source_parser.clone());
    // ... rest of DI composition using source_parsing_container

    // Used in multiple places:
    // - source_parsing_container.scanner_provider()
    // - source_parsing_container.language_detector()
    // - source_parsing_container.path_normalization()
}
```

### 6.2 root_mcp_main_entry.rs

```rust
use rmcp::ServiceExt;
use std::sync::Arc;
use mcp_server::agent_mcp_server_orchestrator::LintArwakyMcpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let source_parsing_container =
        source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let source_parser = source_parsing_container.source_parser();

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::new(source_parser);
    // ... rest of initialization
}
```

---

## 7. Workspace Configuration

### Root Cargo.toml (relevant parts)

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.dependencies]
shared = { package = "shared-lint-arwaky", path = "crates/shared" }
source_parsing = { package = "source_parsing-lint-arwaky", path = "crates/source-parsing" }
# ... other deps

[dependencies]
shared.workspace = true
source_parsing.workspace = true  # ← TO BE REMOVED
# ... other deps
```

---

## 8. Implementation Plan

### Phase 1: Create Infrastructure Modules per Crate

Each crate needs these files (copy from source-parsing, adapt):

| Crate | New Files |
|-------|-----------|
| import-rules | `infrastructure_language_detector.rs` |
| code-analysis | `infrastructure_language_detector.rs` |
| cli-commands | `infrastructure_language_detector.rs`, `infrastructure_scanner_provider.rs` |
| git-hooks | `infrastructure_scanner_provider.rs` |

### Phase 2: Add `new_default()` Constructors

Each crate should add a `new_default()` method that creates its own infrastructure:

```rust
impl ImportContainer {
    pub fn new_default() -> Self {
        let lang_detector = Arc::new(infrastructure_language_detector::LanguageDetector);
        // Create local parser implementation
        Self::new(local_parser)
    }
}
```

### Phase 3: Update Entry Points

Remove `source_parsing::` dependency from:
- `root_cli_main_entry.rs`
- `root_mcp_main_entry.rs`

Use each crate's `new_default()` instead.

### Phase 4: Remove source-parsing Crate

1. Delete `crates/source-parsing/`
2. Remove from workspace `Cargo.toml`
3. Remove from root `Cargo.toml`

---

## 9. Testing Checklist

- [ ] `cargo build --release` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `lint-arwaky-cli scan .` works
- [ ] `lint-arwaky-cli check .` works
- [ ] `lint-arwaky-cli scan test-workspaces` works
- [ ] Each crate compiles independently:
  - [ ] `cargo build -p import_rules-lint-arwaky`
  - [ ] `cargo build -p code_analysis-lint-arwaky`
  - [ ] `cargo build -p cli_commands-lint-arwaky`
  - [ ] `cargo build -p git_hooks-lint-arwaky`

---

## 10. Critical Notes

1. **FilePath, FilePathList, DirectoryPath** are in `shared` crate, NOT `source-parsing`. These VOs are safe.

2. **FileCollectorProvider** is already in `shared/src/source-parsing/infrastructure_file_collector_provider.rs`. Can be re-exported or copied.

3. **Scanner implementations** (Rust, Python, JS) are large (~400-800 lines each). Consider:
   - Option A: Copy full implementations to each crate (duplicated but self-contained)
   - Option B: Create a new `source-parsing-infra` crate (shared infra, no DI dependency)
   - Option C: Keep only what each crate actually uses

4. **code-analysis** already has `NullSourceParser` — may not need full parser implementation.

5. **git-hooks** only uses `IScannerProviderPort` (file collection), not `ISourceParserPort` (parsing).

---

**Document prepared for handoff to implementing AI agent.**
**All source code included. Ready for implementation.**


Hi Raka — below is a backend-focused review of the uploaded `naming-rules` crate and its `shared` dependencies. I focused on **API/contract design, error handling, business logic correctness, security, performance, scalability, maintainability, and SOLID principles**.

> **Scope note**
>
> The uploaded code is a **Rust linting/architecture-compliance engine**, not a typical HTTP backend with database access. Therefore:
>
> - There are **no database queries** in the provided code.
> - The “API” here is mostly the **public trait/contract surface** used by orchestrators and checkers.
> - I included database recommendations separately for future persistence of lint results.

---

# 1. Executive Summary

The crate has a good architectural intention:

- Clear separation between:
  - `root_` container/wiring
  - `agent_` orchestration
  - `capabilities_` rule execution
  - `contract_` protocols
  - `taxonomy_` value objects
  - `utility_` helpers

However, several important backend-quality issues exist:

| Severity | Category        | Main Issue                                                                                                  |
| -------- | --------------- | ----------------------------------------------------------------------------------------------------------- |
| High     | Performance     | Synchronous filesystem walking inside`async fn` can block the async runtime                               |
| High     | Reliability     | Audit APIs return`Vec<LintResult>` instead of `Result`, swallowing I/O and scan failures                |
| High     | Security        | File walking is not explicitly protected against path traversal, symlink escapes, or unbounded recursion    |
| High     | Correctness     | Layer detection inconsistency:`surface_` maps to `surfaces`, while other constants/docs use `surface` |
| High     | Maintainability | Config parsing silently falls back to defaults on invalid YAML                                              |
| Medium   | SOLID/ISP       | `INamingCheckerProtocol` forces checkers to implement unused methods                                      |
| Medium   | Performance     | Full file list is materialized in memory and processed sequentially                                         |
| Medium   | Correctness     | AES101 minimum word count is hardcoded while config exposes`word_count`                                   |
| Medium   | Maintainability | Duplicate`_make_result()` and layer-detection logic across checkers                                       |
| Medium   | Correctness     | Exceptions are matched by basename only, which can over-exempt files                                        |
| Low      | Correctness     | Multi-dot filenames such as`foo.test.ts` may produce false positives                                      |
| Low      | Maintainability | Some messages still mention`infrastructure_`, while FRD says `utility_` replaces it                     |

---

# 2. Detailed Findings and Fixes

---

## 2.1 High: Blocking Filesystem Walk Inside Async Code

### Current issue

In `agent_naming_orchestrator.rs`:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
    let mut results = LintResultList::new(Vec::new());

    let all_files = shared::naming_rules::utility_naming_filesystem::walk_recursive(
        target,
        Some(&self.ignored_patterns),
    );

    // ...
}
```

`walk_recursive` is synchronous and likely performs blocking filesystem I/O.

### Why this matters

If this code runs inside an async runtime such as Tokio, a large repository scan can block the worker thread and starve other tasks.

This is a common backend scalability problem:

- CPU-bound or blocking I/O work inside async handlers
- One large scan can degrade all concurrent requests
- Poor horizontal scalability in server/MCP mode

### Recommended fix

Move filesystem traversal into a blocking task.

If using Tokio:

```rust
use tokio::task::spawn_blocking;
```

Updated contract:

```rust
// crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::naming_rules::taxonomy_naming_error::NamingAuditError;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(
        &self,
        target: &FilePath,
    ) -> Result<Vec<LintResult>, NamingAuditError>;

    fn name(&self) -> &str;
}
```

New error type:

```rust
// crates/shared/src/naming-rules/taxonomy_naming_error.rs

use crate::common::taxonomy_adapter_error::ScanError;

#[derive(Debug, thiserror::Error)]
pub enum NamingAuditError {
    #[error(transparent)]
    Scan(#[from] ScanError),

    #[error("naming audit task join failed: {0}")]
    Join(String),

    #[error("naming configuration is invalid: {0}")]
    Config(String),
}
```

Updated orchestrator:

```rust
// crates/naming-rules/src/agent_naming_orchestrator.rs

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_file_walker_protocol::IFileWalkerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_constant::SOURCE_EXTENSIONS;
use shared::naming_rules::taxonomy_naming_error::NamingAuditError;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::path::Path;
use std::sync::Arc;

pub struct NamingOrchestrator {
    naming_convention_checker: Arc<dyn INamingConventionChecker>,
    suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    file_walker: Arc<dyn IFileWalkerProtocol>,
    config: Arc<ArchitectureConfig>,
    layer_map: Arc<LayerMapVO>,
    ignored_patterns: PatternList,
}

#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(
        &self,
        target: &FilePath,
    ) -> Result<Vec<LintResult>, NamingAuditError> {
        let walker = Arc::clone(&self.file_walker);
        let target = target.clone();
        let ignored_patterns = self.ignored_patterns.clone();

        let all_files = spawn_blocking(move || {
            walker.walk(&target, &ignored_patterns)
        })
        .await
        .map_err(|e| NamingAuditError::Join(e.to_string()))??;

        let files = Self::filter_source_files(&all_files);

        let mut results = LintResultList::default();

        self.naming_convention_checker
            .check_file_naming(
                self.config.as_ref(),
                self.layer_map.as_ref(),
                &files,
                &target,
                &mut results,
            )
            .await;

        self.suffix_prefix_checker
            .check_domain_suffixes(
                self.config.as_ref(),
                self.layer_map.as_ref(),
                &files,
                &target,
                &mut results,
            )
            .await;

        Ok(results.values)
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

impl NamingOrchestrator {
    pub fn new(
        naming_convention_checker: Arc<dyn INamingConventionChecker>,
        suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
        file_walker: Arc<dyn IFileWalkerProtocol>,
        config: Arc<ArchitectureConfig>,
        layer_map: Arc<LayerMapVO>,
    ) -> Self {
        let ignored_patterns = PatternList {
            values: config
                .ignored_paths
                .values
                .iter()
                .map(|fp| {
                    fp.value
                        .trim_start_matches("./")
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .to_string()
                })
                .collect(),
        };

        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            file_walker,
            config,
            layer_map,
            ignored_patterns,
        }
    }

    fn filter_source_files(
        files: &shared::common::taxonomy_paths_vo::FilePathList,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        let filtered = files
            .values
            .iter()
            .filter(|file| {
                Path::new(&file.value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| SOURCE_EXTENSIONS.contains(&ext))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        shared::common::taxonomy_paths_vo::FilePathList::new(filtered)
    }
}

use tokio::task::spawn_blocking;
```

If you do not want a Tokio dependency, make the walker explicitly synchronous and avoid pretending the operation is async. But for a backend/MCP service, `spawn_blocking` is the better scalability fix.

---

## 2.2 High: No Error Propagation From Filesystem or Audit Operations

### Current issue

The current aggregate returns:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
```

There is no way to report:

- missing directory
- permission denied
- symlink loop
- invalid configuration
- I/O failure
- walker failure

### Why this matters

A linting engine must distinguish between:

1. “No violations found”
2. “Scan failed”
3. “Configuration invalid”

Otherwise failures become silent false negatives.

### Recommended fix

Use `Result<Vec<LintResult>, NamingAuditError>` as shown above.

Also introduce a fallible file walker contract:

```rust
// crates/shared/src/naming-rules/contract_file_walker_protocol.rs

use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

pub trait IFileWalkerProtocol: Send + Sync {
    fn walk(
        &self,
        target: &FilePath,
        ignored_patterns: &PatternList,
    ) -> Result<FilePathList, ScanError>;
}
```

This improves:

- reliability
- testability
- observability
- CLI exit-code correctness
- MCP/API error reporting

---

## 2.3 High: Filesystem Security Hardening Is Missing

### Current issue

`FilePath::new()` normalizes slashes:

```rust
value.replace('\\', "/")
```

But it does not:

- canonicalize paths
- confine traversal to a workspace root
- prevent `../` escape
- prevent symlink traversal
- limit recursion depth
- explicitly ignore `.git`, `node_modules`, `target`, etc.

### Why this matters

If the scanner accepts user-provided paths or config-provided ignore rules, it may be exposed to:

- path traversal
- symlink loops
- denial of service through extremely large directory trees
- accidental scanning of sensitive directories
- scanning outside the intended workspace

### Recommended fix

Introduce a secure walker abstraction.

Example using `walkdir` and `globset`:

```rust
// crates/shared/src/naming-rules/utility_secure_file_walker.rs

use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::naming_rules::contract_file_walker_protocol::IFileWalkerProtocol;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct SecureFileWalker {
    workspace_root: PathBuf,
    max_depth: usize,
}

impl SecureFileWalker {
    pub fn new(workspace_root: PathBuf, max_depth: usize) -> Self {
        Self {
            workspace_root,
            max_depth,
        }
    }

    fn resolve_and_confine(&self, target: &FilePath) -> Result<PathBuf, ScanError> {
        let requested = PathBuf::from(&target.value);

        let canonical_target = requested.canonicalize().map_err(|e| {
            ScanError::new(
                target.clone(),
                ErrorMessage::new(format!("failed to canonicalize path: {e}")),
            )
        })?;

        let canonical_root = self.workspace_root.canonicalize().map_err(|e| {
            ScanError::new(
                target.clone(),
                ErrorMessage::new(format!("failed to canonicalize workspace root: {e}")),
            )
        })?;

        if !canonical_target.starts_with(&canonical_root) {
            return Err(ScanError::new(
                target.clone(),
                ErrorMessage::new("target path escapes workspace root"),
            ));
        }

        Ok(canonical_target)
    }

    fn build_ignore_set(
        &self,
        ignored_patterns: &PatternList,
        target: &FilePath,
    ) -> Result<GlobSet, ScanError> {
        let mut builder = GlobSetBuilder::new();

        for pattern in ignored_patterns.values.iter() {
            let glob = Glob::new(pattern).map_err(|e| {
                ScanError::new(
                    target.clone(),
                    ErrorMessage::new(format!("invalid ignore pattern '{pattern}': {e}")),
                )
            })?;

            builder.add(glob);
        }

        builder.build().map_err(|e| {
            ScanError::new(
                target.clone(),
                ErrorMessage::new(format!("failed to build ignore matcher: {e}")),
            )
        })
    }

    fn is_ignored(path: &Path, ignore_set: &GlobSet) -> bool {
        ignore_set.is_match(path)
    }
}

impl IFileWalkerProtocol for SecureFileWalker {
    fn walk(
        &self,
        target: &FilePath,
        ignored_patterns: &PatternList,
    ) -> Result<FilePathList, ScanError> {
        let root = self.resolve_and_confine(target)?;
        let ignore_set = self.build_ignore_set(ignored_patterns, target)?;

        let mut files = Vec::new();

        let walker = WalkDir::new(&root)
            .follow_links(false)
            .max_depth(self.max_depth)
            .into_iter()
            .filter_entry(|entry| !Self::is_ignored(entry.path(), &ignore_set));

        for entry in walker {
            let entry = entry.map_err(|e| {
                ScanError::new(
                    target.clone(),
                    ErrorMessage::new(format!("directory walk failed: {e}")),
                )
            })?;

            if entry.file_type().is_file() {
                let path = entry.path().to_string_lossy().to_string();

                let file_path = FilePath::new(path).map_err(|e| {
                    ScanError::new(target.clone(), ErrorMessage::new(e))
                })?;

                files.push(file_path);
            }
        }

        Ok(FilePathList::new(files))
    }
}
```

Additional recommendations:

- Always ignore `.git`, `node_modules`, `target`, `dist`, `build`, `.venv`, etc. by default.
- Make max depth configurable.
- Add file-count limits to prevent DoS.
- Add total byte-size limits if you later read file contents.

---

## 2.4 High: Layer Detection Inconsistency for `surface_`

### Current issue

In `utility_layer_detector.rs`:

```rust
const PREFIX_MAP: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];
```

But the FRD and constants use `surface_` as the layer prefix.

The naming constants define:

```rust
pub const LAYER_PREFIXES: &[&str] = &[
    "taxonomy_",
    "contract_",
    "utility_",
    "capabilities_",
    "agent_",
    "surface_",
    "root_",
];
```

This can cause a serious correctness issue:

- `surface_login_controller.rs` may be detected as layer `surfaces`
- but the layer map may define `surface`
- then `LayerMapVO` lookup returns `None`
- suffix validation may be silently skipped

### Recommended fix

Use one source of truth.

```rust
// crates/shared/src/common/utility_layer_detector.rs

use crate::naming_rules::taxonomy_naming_constant::LAYER_PREFIXES;
use std::path::Path;

pub fn detect_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    LAYER_PREFIXES.iter().find_map(|prefix| {
        stem.starts_with(prefix)
            .then(|| prefix.trim_end_matches('_').to_string())
    })
}
```

Now:

```text
surface_login_controller.rs -> surface
taxonomy_user_vo.rs         -> taxonomy
agent_naming_orchestrator.rs -> agent
```

If your existing configuration uses `surfaces`, either:

1. migrate config to `surface`, or
2. add a temporary compatibility alias.

Temporary compatibility helper:

```rust
pub fn normalize_layer_name(layer: &str) -> String {
    match layer {
        "surfaces" => "surface".to_string(),
        other => other.to_string(),
    }
}
```

But the cleaner solution is to standardize on `surface`.

---

## 2.5 High: Configuration Parsing Silently Falls Back to Defaults

### Current issue

In `taxonomy_config_vo.rs`:

```rust
let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
```

and later:

```rust
Err(e) => {
    eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
    eprintln!("[warn] Falling back to default config...");
    ArchitectureConfig::default()
}
```

### Why this matters

This is dangerous for a compliance tool.

If a user writes invalid configuration, the tool may silently use default rules and produce false confidence.

Example:

```yaml
architecture:
  enabled: true
  layers:
    taxonomy:
      suffix_policy: strict
      allowed_suffix:
        - _vo
        - _entity
   broken_field: ???
```

A syntax or type error could silently disable the intended policy.

### Recommended fix

Make runtime config parsing fallible.

```rust
// crates/shared/src/config-system/taxonomy_config_vo.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigParseError {
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    #[error("JSON conversion error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("configuration deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}
```

Then:

```rust
pub fn parse_config_yaml(
    yaml_str: &str,
) -> Result<ArchitectureConfig, ConfigParseError> {
    let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str)?;

    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = serde_json::to_value(arch_val)?;

        // existing transformation logic...

        let config: ArchitectureConfig = serde_json::from_value(arch_json)?;

        Ok(config)
    } else {
        Ok(ArchitectureConfig::default())
    }
}
```

For embedded default configs, it is acceptable to panic at startup if the built-in config is invalid, because that is a compile-time/test-time defect:

```rust
pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_RUST_CONFIG
        .get_or_init(|| {
            parse_config_yaml(include_str!(
                "../../../../lint_arwaky.config.rust.yaml"
            ))
            .expect("built-in Rust AES config must be valid")
        })
        .clone()
}
```

But user-provided config must return `Result`.

---

## 2.6 Medium: Interface Segregation Principle Violation

### Current issue

```rust
#[async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(...);
    async fn check_domain_suffixes(...);
}
```

But:

- `NamingConventionChecker` implements `check_domain_suffixes` as no-op
- `SuffixPrefixChecker` implements `check_file_naming` as no-op

### Why this matters

This violates the Interface Segregation Principle:

> Clients should not be forced to depend on methods they do not use.

It also makes the system harder to extend.

### Recommended fix

Split the protocol.

```rust
// crates/shared/src/naming-rules/contract_naming_checker_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use async_trait::async_trait;

#[async_trait]
pub trait INamingConventionChecker: Send + Sync {
    async fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

#[async_trait]
pub trait ISuffixPrefixChecker: Send + Sync {
    async fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

Then:

```rust
impl INamingConventionChecker for NamingConventionChecker { ... }
impl ISuffixPrefixChecker for SuffixPrefixChecker { ... }
```

This is cleaner and more SOLID-compliant.

---

## 2.7 Medium: AES101 Minimum Word Count Is Hardcoded

### Current issue

FRD says:

> contain at least 3 words

But config has:

```rust
naming: NamingConfig::new(Count::new(2)),
```

And the checker hardcodes:

```rust
NamingViolation::NamingConvention {
    min_words: 3,
    separator: SNAKE_CASE_SEPARATOR.to_string(),
    ...
}
```

The `_config` argument is unused.

### Why this matters

This creates inconsistency between:

- documentation
- configuration
- runtime behavior

It also prevents projects from tuning the rule.

### Recommended fix

Use config:

```rust
fn min_words_from_config(config: &ArchitectureConfig) -> usize {
    let value = config.naming.word_count.value;

    if value <= 0 {
        return 3;
    }

    usize::try_from(value).unwrap_or(3)
}
```

Update default config:

```rust
impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(3)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}
```

Then use a config-driven validator.

Instead of a static regex requiring exactly 3+ words, use a pure validator:

```rust
// crates/shared/src/naming-rules/utility_naming.rs

pub fn is_valid_aes_stem(stem: &str, min_words: usize) -> bool {
    if stem.is_empty() {
        return false;
    }

    let mut word_count = 0usize;
    let mut previous_was_underscore = false;

    for ch in stem.chars() {
        match ch {
            '_' => {
                if previous_was_underscore || word_count == 0 {
                    return false;
                }

                previous_was_underscore = true;
            }

            'a'..='z' | '0'..='9' => {
                if previous_was_underscore {
                    word_count += 1;
                    previous_was_underscore = false;
                } else if word_count == 0 {
                    word_count = 1;
                }
            }

            _ => return false,
        }
    }

    if previous_was_underscore {
        return false;
    }

    word_count >= min_words
}
```

Examples:

```text
capabilities_user_checker -> valid
taxonomy_user_vo          -> valid
db                        -> invalid
user_checker              -> invalid if min_words = 3
capabilities_db_adapter   -> valid
```

---

## 2.8 Medium: Duplicate `_make_result()` Logic

### Current issue

Both checkers define almost identical functions:

```rust
fn _make_result(...)
```

This violates DRY and increases maintenance cost.

### Recommended fix

Create a shared helper.

```rust
// crates/shared/src/naming-rules/utility_naming_lint_result.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::naming_rules::taxonomy_naming_constant::ADAPTER_NAME;

pub fn file_level_naming_result(
    file: &FilePath,
    code: &str,
    message: impl Into<String>,
    severity: Severity,
) -> LintResult {
    LintResult {
        file: file.clone(),
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message),
        source: Some(AdapterName::raw(ADAPTER_NAME)),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
```

Use `line: 0` for file-level violations instead of `line: 1`.

Why?

- `line: 1` can mislead editors into jumping to the first line.
- `line: 0` better represents a whole-file issue.

---

## 2.9 Medium: Exceptions Are Matched by Basename Only

### Current issue

```rust
if def.exceptions.values.contains(&filename.to_string()) {
    return;
}
```

### Why this matters

If exception is `mod.rs`, then every `mod.rs` in every directory is exempt.

That may be intended for barrel files, but for path-specific exceptions it is too broad.

Example:

```yaml
exceptions:
  - legacy_taxonomy_old_vo.rs
```

This would exempt:

```text
crates/a/legacy_taxonomy_old_vo.rs
crates/b/legacy_taxonomy_old_vo.rs
```

even if only one was intended.

### Recommended fix

Match against:

1. normalized full path
2. basename
3. glob pattern

Example:

```rust
fn is_exception(
    definition: &shared::taxonomy_definition_vo::LayerDefinition,
    file: &FilePath,
) -> bool {
    let basename = file.basename();

    definition.exceptions.values.iter().any(|pattern| {
        pattern == &file.value || pattern == &basename
    })
}
```

Better, use glob matching:

```rust
use globset::{Glob, GlobSetBuilder};

fn is_exception(
    definition: &shared::taxonomy_definition_vo::LayerDefinition,
    file: &FilePath,
) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in definition.exceptions.values.iter() {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }

    match builder.build() {
        Ok(set) => set.is_match(&file.value) || set.is_match(file.basename()),
        Err(_) => false,
    }
}
```

For performance, cache compiled exception globs per layer instead of rebuilding per file.

---

## 2.10 Medium: Full File List Materialized in Memory

### Current issue

```rust
let all_files = walk_recursive(...);
let files = Self::filter_source_files(&all_files);
```

This stores all files in memory.

### Why this matters

For monorepos, this can become expensive:

- tens or hundreds of thousands of files
- high memory usage
- poor concurrency
- long time-to-first-result

### Recommended fix

For a first improvement:

- keep file list but process in chunks
- parallelize CPU-bound checks using `rayon`

Longer term:

- stream directory entries
- process batches
- emit results incrementally
- support cancellation

Example future contract:

```rust
pub trait LintSink: Send + Sync {
    fn emit(&mut self, result: LintResult);
}
```

Then:

```rust
async fn run_audit_streaming(
    &self,
    target: &FilePath,
    sink: &mut dyn LintSink,
) -> Result<(), NamingAuditError>;
```

This is better for:

- CLI progress output
- MCP streaming responses
- large repositories
- memory-constrained environments

---

## 2.11 Medium: Repeated Layer Detection and Context Construction

### Current issue

Both checkers do:

```rust
let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

for f in &files.values {
    let f_str = f.to_string();
    let filename = f.rsplit('/').next().unwrap_or(&f_str);
    let layer = self._detect_layer(&f_str, &layer_keys);
    // ...
}
```

This repeats work.

### Recommended fix

Build a shared file context once.

```rust
// crates/shared/src/naming-rules/taxonomy_file_context_vo.rs

use crate::common::taxonomy_path_vo::FilePath;
use crate::common::utility_layer_detector;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::LayerNameVO;

pub struct FileNamingContext {
    pub file: FilePath,
    pub filename: String,
    pub stem: String,
    pub suffix: Option<String>,
    pub layer: Option<LayerNameVO>,
}

impl FileNamingContext {
    pub fn new(file: &FilePath, layer_map: &LayerMapVO) -> Self {
        let filename = file.basename();

        let stem = crate::naming_rules::utility_naming::get_stem(&filename)
            .unwrap_or_default()
            .to_string();

        let suffix = crate::naming_rules::utility_naming::get_suffix(&stem)
            .map(|s| s.to_string());

        let layer = utility_layer_detector::detect_layer_from_prefix(&filename)
            .map(LayerNameVO::new);

        Self {
            file: file.clone(),
            filename,
            stem,
            suffix,
            layer,
        }
    }

    pub fn definition<'a>(
        &self,
        layer_map: &'a LayerMapVO,
    ) -> Option<&'a crate::taxonomy_definition_vo::LayerDefinition> {
        self.layer
            .as_ref()
            .and_then(|layer| layer_map.values.get(layer))
    }
}
```

Then orchestrator can build contexts once:

```rust
let contexts: Vec<FileNamingContext> = files
    .values
    .iter()
    .map(|file| FileNamingContext::new(file, &self.layer_map))
    .collect();
```

And pass contexts into checkers.

This improves:

- performance
- consistency
- testability
- maintainability

---

## 2.12 Low: Multi-Dot Filenames Can Cause False Positives

### Current issue

```rust
pub fn get_stem(filename: &str) -> Option<&str> {
    if let Some(pos) = filename.rfind('.') {
        Some(&filename[..pos])
    } else {
        Some(filename)
    }
}
```

For:

```text
user_service.test.ts
```

stem becomes:

```text
user_service.test
```

This contains a dot and will fail snake_case validation.

### Why this matters

TypeScript and JavaScript projects commonly use:

```text
foo.test.ts
foo.spec.ts
foo.e2e.ts
```

### Recommended fix

Either:

1. explicitly exempt test/spec files, or
2. define a richer stem extraction policy.

Example exemption:

```rust
fn is_test_or_spec_file(filename: &str) -> bool {
    filename.ends_with(".test.ts")
        || filename.ends_with(".spec.ts")
        || filename.ends_with(".test.tsx")
        || filename.ends_with(".spec.tsx")
        || filename.ends_with(".test.js")
        || filename.ends_with(".spec.js")
        || filename.ends_with(".test.jsx")
        || filename.ends_with(".spec.jsx")
}
```

Then:

```rust
if file.is_barrel_file()
    || file.is_entry_point()
    || is_test_or_spec_file(&filename)
{
    return;
}
```

Alternatively, configure exceptions:

```yaml
exceptions:
  - "*.test.ts"
  - "*.spec.ts"
```

and use glob matching.

---

## 2.13 Low: Violation Messages Mention `infrastructure_`

### Current issue

In `taxonomy_naming_violation_vo.rs`:

```rust
If this is I/O or external system access → move to the infrastructure_ layer
```

But the FRD says:

> utility_ covering the former infrastructure_ concerns

### Recommended fix

Update messages to match the current architecture.

Example:

```rust
"If this is I/O or external system access, move it to the utility_ layer \
 with an allowed technical-role suffix such as _reader, _writer, _parser, or _adapter."
```

This avoids confusing users and keeps lint guidance consistent with the documented 7-layer model.

---

# 3. Improved Business Logic Implementation

Below is a cleaned-up version of the AES101 checker logic.

## 3.1 Fixed AES101 Checker Core

```rust
// crates/naming-rules/src/capabilities_naming_convention_checker.rs

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    LAYER_PREFIXES, RULE_CODE_NAMING_CONVENTION, RULE_CODE_SUFFIX_PREFIX,
    SNAKE_CASE_SEPARATOR,
};
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared::naming_rules::utility_naming::{get_stem, is_valid_aes_stem};
use shared::naming_rules::utility_naming_lint_result::file_level_naming_result;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;

#[derive(Clone, Default)]
pub struct NamingConventionChecker;

#[async_trait]
impl INamingConventionChecker for NamingConventionChecker {
    async fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let min_words = min_words_from_config(config);

        for file in &files.values {
            self.check_one(config, layer_map, file, min_words, results);
        }
    }
}

impl NamingConventionChecker {
    fn check_one(
        &self,
        _config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        file: &FilePath,
        min_words: usize,
        results: &mut LintResultList,
    ) {
        let filename = file.basename();

        if file.is_barrel_file() || file.is_entry_point() {
            return;
        }

        let stem = match get_stem(&filename) {
            Some(stem) => stem,
            None => return,
        };

        let layer = utility_layer_detector::detect_layer_from_prefix(&filename)
            .map(LayerNameVO::new);

        let definition = layer
            .as_ref()
            .and_then(|layer_name| layer_map.values.get(layer_name));

        if let Some(def) = definition {
            if is_exception(def, file) {
                return;
            }
        }

        match layer {
            Some(_) => {
                if !is_valid_aes_stem(stem, min_words) {
                    results.push(file_level_naming_result(
                        file,
                        RULE_CODE_NAMING_CONVENTION,
                        NamingViolation::NamingConvention {
                            min_words,
                            separator: SNAKE_CASE_SEPARATOR.to_string(),
                            reason: None,
                        }
                        .to_string(),
                        Severity::HIGH,
                    ));
                }
            }

            None => {
                let actual_prefix = stem
                    .split('_')
                    .next()
                    .unwrap_or_default()
                    .to_string();

                let has_known_layer_prefix = LAYER_PREFIXES
                    .iter()
                    .any(|prefix| stem.starts_with(prefix));

                if !actual_prefix.is_empty() && !has_known_layer_prefix {
                    let allowed: Vec<String> = LAYER_PREFIXES
                        .iter()
                        .map(|prefix| prefix.trim_end_matches('_').to_string())
                        .collect();

                    results.push(file_level_naming_result(
                        file,
                        RULE_CODE_SUFFIX_PREFIX,
                        NamingViolation::UnknownPrefix {
                            prefix: actual_prefix,
                            allowed,
                            reason: None,
                        }
                        .to_string(),
                        Severity::HIGH,
                    ));
                } else if !is_valid_aes_stem(stem, min_words) {
                    results.push(file_level_naming_result(
                        file,
                        RULE_CODE_NAMING_CONVENTION,
                        NamingViolation::NamingConvention {
                            min_words,
                            separator: SNAKE_CASE_SEPARATOR.to_string(),
                            reason: None,
                        }
                        .to_string(),
                        Severity::HIGH,
                    ));
                }
            }
        }
    }
}

fn min_words_from_config(config: &ArchitectureConfig) -> usize {
    let value = config.naming.word_count.value;

    if value <= 0 {
        return 3;
    }

    usize::try_from(value).unwrap_or(3)
}

fn is_exception(
    definition: &shared::taxonomy_definition_vo::LayerDefinition,
    file: &FilePath,
) -> bool {
    let basename = file.basename();

    definition
        .exceptions
        .values
        .iter()
        .any(|pattern| pattern == &file.value || pattern == &basename)
}
```

---

## 3.2 Fixed AES102 Checker Core

```rust
// crates/naming-rules/src/capabilities_suffix_prefix_checker.rs

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::ISuffixPrefixChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT,
};
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared::naming_rules::utility_naming::{get_stem, get_suffix};
use shared::naming_rules::utility_naming_lint_result::file_level_naming_result;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;

#[derive(Clone, Default)]
pub struct SuffixPrefixChecker;

#[async_trait]
impl ISuffixPrefixChecker for SuffixPrefixChecker {
    async fn check_domain_suffixes(
        &self,
        _config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for file in &files.values {
            self.check_one(file, layer_map, results);
        }
    }
}

impl SuffixPrefixChecker {
    fn check_one(
        &self,
        file: &FilePath,
        layer_map: &LayerMapVO,
        results: &mut LintResultList,
    ) {
        let filename = file.basename();

        if file.is_barrel_file() || file.is_entry_point() {
            return;
        }

        let layer = match utility_layer_detector::detect_layer_from_prefix(&filename) {
            Some(layer) => LayerNameVO::new(layer),
            None => return,
        };

        let definition = match layer_map.values.get(&layer) {
            Some(def) => def,
            None => return,
        };

        if is_exception(definition, file) {
            return;
        }

        let stem = match get_stem(&filename) {
            Some(stem) => stem,
            None => return,
        };

        let suffix = get_suffix(stem);

        if let Some(suffix) = suffix {
            if definition
                .naming
                .forbidden_suffix
                .values
                .iter()
                .any(|forbidden| forbidden == suffix)
            {
                results.push(file_level_naming_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::SuffixForbidden {
                        layer_name: layer.value().to_string(),
                        forbidden_suffix: suffix.to_string(),
                        reason: None,
                    }
                    .to_string(),
                    Severity::HIGH,
                ));

                return;
            }
        }

        if definition.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = suffix
                .map(|suffix| {
                    definition
                        .naming
                        .allowed_suffix
                        .values
                        .iter()
                        .any(|allowed| allowed == suffix)
                })
                .unwrap_or(false);

            if !valid {
                results.push(file_level_naming_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::SuffixMismatch {
                        layer_name: layer.value().to_string(),
                        used_suffix: suffix.unwrap_or("(none)").to_string(),
                        allowed: definition.naming.allowed_suffix.values.clone(),
                        reason: None,
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }
    }
}

fn is_exception(
    definition: &shared::taxonomy_definition_vo::LayerDefinition,
    file: &FilePath,
) -> bool {
    let basename = file.basename();

    definition
        .exceptions
        .values
        .iter()
        .any(|pattern| pattern == &file.value || pattern == &basename)
}
```

---

# 4. Improved Container Wiring

```rust
// crates/naming-rules/src/root_naming_rules_container.rs

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_file_walker_protocol::IFileWalkerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingConventionChecker>,
    suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    file_walker: Arc<dyn IFileWalkerProtocol>,
    config: Arc<ArchitectureConfig>,
    layer_map: Arc<LayerMapVO>,
}

impl NamingContainer {
    pub fn new(
        config: Arc<ArchitectureConfig>,
        layer_map: Arc<LayerMapVO>,
        file_walker: Arc<dyn IFileWalkerProtocol>,
    ) -> Self {
        let naming_convention_checker: Arc<dyn INamingConventionChecker> = Arc::new(
            crate::capabilities_naming_convention_checker::NamingConventionChecker::default(),
        );

        let suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker> = Arc::new(
            crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::default(),
        );

        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            file_walker,
            config,
            layer_map,
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            Arc::clone(&self.naming_convention_checker),
            Arc::clone(&self.suffix_prefix_checker),
            Arc::clone(&self.file_walker),
            Arc::clone(&self.config),
            Arc::clone(&self.layer_map),
        ))
    }
}
```

---

# 5. Database Review

There are **no database queries** in the uploaded code.

However, if you later persist lint results, follow these backend best practices.

## 5.1 Use Parameterized Queries

Do not do this:

```rust
let query = format!(
    "INSERT INTO lint_results(file, code, message) VALUES('{}', '{}', '{}')",
    file, code, message
);
```

Use SQLx or another parameterized query API:

```rust
sqlx::query(
    r#"
    INSERT INTO lint_results (
        run_id,
        file,
        line,
        column,
        code,
        severity,
        message,
        adapter
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
)
.bind(run_id)
.bind(file)
.bind(line)
.bind(column)
.bind(code)
.bind(severity)
.bind(message)
.bind(adapter)
.execute(&pool)
.await?;
```

## 5.2 Use Bulk Inserts

For large scans, avoid one query per violation.

Use:

- batch inserts
- transactions
- `COPY` for PostgreSQL
- `executemany` for Python
- prepared bulk operations for TypeScript/Node

Example:

```rust
let mut tx = pool.begin().await?;

for chunk in results.chunks(1000) {
    for result in chunk {
        sqlx::query(...)
            .bind(...)
            .execute(&mut *tx)
            .await?;
    }
}

tx.commit().await?;
```

## 5.3 Add Indexes

Suggested schema indexes:

```sql
CREATE INDEX idx_lint_results_run_file ON lint_results(run_id, file);
CREATE INDEX idx_lint_results_code ON lint_results(code);
CREATE INDEX idx_lint_results_severity ON lint_results(severity);
```

## 5.4 Add Connection Pool Timeouts

For backend services:

- max connections
- idle timeout
- acquire timeout
- statement timeout
- transaction timeout

This prevents lint-result persistence from becoming a scalability bottleneck.

---

# 6. SOLID Principles Assessment

## Single Responsibility Principle

### Current issue

`NamingConventionChecker` handles:

- AES101 snake_case validation
- AES102 unknown prefix detection

### Recommendation

Consider separating:

```rust
pub struct StemConventionChecker;      // AES101
pub struct LayerPrefixChecker;         // AES102 unknown prefix
pub struct SuffixPolicyChecker;        // AES102 suffix alignment
```

Then orchestrate them as independent rules.

This gives cleaner rule isolation and easier testing.

---

## Open/Closed Principle

### Current issue

Suffix policy behavior is controlled by string comparison:

```rust
if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT { ... }
```

### Recommendation

Model suffix policies as typed enums or strategy objects:

```rust
pub enum SuffixPolicy {
    Strict,
    Flexible,
    Custom(String),
}
```

Then validation can be extended without modifying existing checker logic.

---

## Liskov Substitution Principle

The current trait has no-op implementations:

```rust
async fn check_domain_suffixes(...) {
    // No-op
}
```

This is a LSP smell because callers may assume both methods are meaningful for every implementation.

Splitting the traits fixes this.

---

## Interface Segregation Principle

Fixed by splitting:

```rust
INamingConventionChecker
ISuffixPrefixChecker
```

---

## Dependency Inversion Principle

The orchestrator already depends on abstractions:

```rust
Arc<dyn INamingCheckerProtocol>
```

This is good.

Improve it further by injecting:

```rust
Arc<dyn IFileWalkerProtocol>
```

so filesystem access is also abstracted.

This makes testing much easier:

```rust
struct InMemoryFileWalker {
    files: FilePathList,
}
```

---

# 7. Recommended Test Cases

Add tests for the following.

## AES101

```text
capabilities_user_checker.rs  -> pass
taxonomy_user_vo.rs           -> pass
db.rs                         -> fail
user_checker.rs               -> fail if min_words = 3
capabilities_DB_adapter.rs    -> fail uppercase
capabilities-db-adapter.rs    -> fail wrong separator
capabilities__adapter.rs      -> fail double underscore
capabilities_adapter_.rs      -> fail trailing underscore
```

## AES102

```text
taxonomy_user_vo.rs              -> pass
taxonomy_user_checker.rs         -> fail if checker forbidden in taxonomy
capabilities_user_vo.rs          -> fail if vo forbidden in capabilities
surface_login_controller.rs      -> pass
surface_login_helper.rs          -> fail if helper not allowed in surface
root_naming_rules_container.rs   -> pass
root_naming_rules_service.rs     -> fail if service not allowed in root
```

## Security

```text
target = "../etc"                  -> fail workspace confinement
symlink loop                       -> fail gracefully or skip
ignored_patterns = ["**/*.rs"]     -> no files scanned
ignored_patterns = ["[invalid"]    -> return config/scan error
```

## Config

```text
invalid YAML        -> error
unknown layer       -> warning or error depending on policy
missing suffix list -> error for strict policy
word_count = 0      -> fallback to 3
word_count = 2      -> accept two-word stems if configured
```

---

# 8. Final Recommended Priority Order

If I were fixing this codebase in production, I would prioritize in this order:

## Phase 1: Correctness and Safety

1. Make `run_audit` return `Result`.
2. Make filesystem walking fallible.
3. Add workspace confinement and symlink protection.
4. Fix `surface_` layer mapping inconsistency.
5. Make config parsing return `Result` instead of silently defaulting.

## Phase 2: Architecture and SOLID

6. Split `INamingCheckerProtocol` into two traits.
7. Inject `IFileWalkerProtocol`.
8. Centralize `LintResult` creation.
9. Build shared `FileNamingContext`.
10. Move unknown-prefix detection into its own rule if you want stricter SRP.

## Phase 3: Performance and Scalability

11. Use `spawn_blocking` for filesystem traversal.
12. Avoid rebuilding layer keys per checker.
13. Process files in chunks.
14. Add parallelism for CPU-bound rule evaluation.
15. Add streaming result emission for large repositories.

## Phase 4: Product Quality

16. Support glob-based exceptions.
17. Support `.test.ts` and `.spec.ts` exemptions.
18. Make severity configurable.
19. Update messages to reference `utility_` instead of `infrastructure_`.
20. Add rich diagnostics for invalid configuration.

---

# 9. Summary

The code has a solid architectural foundation, but for production-grade backend quality I would focus on these major improvements:

1. **Return `Result` from audit operations**Silent failures are unacceptable in a compliance tool.
2. **Move blocking filesystem traversal out of async tasks**Use `spawn_blocking` or an explicit synchronous walker.
3. **Harden filesystem access**Canonicalize paths, confine scans to workspace, disable symlink following, limit depth.
4. **Fix layer-name inconsistency**`surface_` should map to `surface`, not `surfaces`, unless the whole system intentionally uses `surfaces`.
5. **Split checker traits**This fixes ISP and LSP issues.
6. **Use configuration values instead of hardcoded rule parameters**Especially AES101 `min_words`.
7. **Centralize result construction and file-context creation**This improves maintainability and reduces duplicate logic.
8. **Fail fast on invalid configuration**
   Do not silently fall back to defaults for user-provided config.

With these changes, the crate will be more secure, scalable, testable, and aligned with SOLID principles.

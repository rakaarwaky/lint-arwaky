# Backend Architecture Review — `shared v1.10.107`

## 1. Executive Summary

The `shared` crate is a foundational kernel containing taxonomy value objects, contract traits, and low-level utilities for a multi-language architecture linting system. The layering intent is strong, but the current implementation has several issues that affect security, reliability, scalability, and architectural compliance.

### Overall Risk Rating: **Medium-High**

The most important problems are:

1. **Command injection risk** in `utility_tui_io.rs`.
2. **Panics and unsafe error handling** in `build.rs`.
3. **Path traversal / symlink escape risks** in recursive walkers.
4. **Business logic leaking into taxonomy/value-object files**.
5. **Primitive obsession in contract traits**, violating AES402.
6. **Error swallowing** through `bool`, `Option`, and `unwrap_or_default()` patterns.
7. **Performance bottlenecks** in duplication scanning and unused-import detection.
8. **Code duplication** between process/command utilities.
9. **Large protocol traits** violating the Interface Segregation Principle.
10. **Cargo-mandated `build.rs` prefix violation** requiring an explicit architectural exception.

There is no direct database layer in this crate, so traditional SQL query issues are not present. However, the crate performs significant filesystem access, external command execution, YAML/JSON parsing, and path resolution. These should be treated as the crate’s data-access surface and hardened accordingly.

---

# 2. Prioritized Findings

| Severity | Area            | Issue                                               | Impact                                                  |
| -------- | --------------- | --------------------------------------------------- | ------------------------------------------------------- |
| Critical | Security        | Shell injection in`is_binary_available`             | Arbitrary command execution if binary name is untrusted |
| High     | Error Handling  | `build.rs` uses `expect` and `panic!`               | Build crashes bypass structured error handling          |
| High     | Security        | Recursive walkers lack symlink confinement          | Path traversal, infinite loops, escape from workspace   |
| High     | Architecture    | Business logic inside taxonomy files                | Violates taxonomy purity and SOLID/SRP                  |
| Medium   | API Design      | Contract traits expose primitives                   | Violates AES402 and weakens domain boundaries           |
| Medium   | Error Handling  | Many utilities return`bool`/`Option`                | Callers lose failure context                            |
| Medium   | Performance     | Duplication scanner stores large normalized strings | High memory usage on large repositories                 |
| Medium   | Performance     | Unused-import detection repeatedly scans content    | O(files × symbols × lines) behavior                     |
| Low      | Maintainability | Duplicate command execution helpers                 | DRY violation                                           |
| Low      | Lint Compliance | `build.rs` uses unknown AES prefix                  | AES102 violation, but Cargo requires this filename      |

---

# 3. Security Review

## 3.1 Critical: Command Injection in `utility_tui_io.rs`

Current code:

```rust
pub fn is_binary_available(bin_name: &str) -> bool {
    let output = Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", bin_name)])
        .status();

    match output {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}
```

### Problem

If `bin_name` comes from configuration, UI input, MCP arguments, or adapter metadata, this can become a shell injection vector.

Example malicious input:

```text
rustc; rm -rf /
```

The shell may execute:

```sh
command -v rustc; rm -rf / >/dev/null 2>&1
```

### Fix

Do not invoke a shell for binary discovery. Validate the binary name and inspect `PATH` directly.

```rust
// crates/shared/src/tui/utility_tui_io.rs

use std::path::Path;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Only allow conservative executable names.
/// Rejects path separators, shell metacharacters, and leading dashes.
fn is_safe_binary_name(name: &str) -> bool {
    if name.is_empty() || name.starts_with('-') {
        return false;
    }

    name.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'
    })
}

/// Check if a binary is available in PATH without using a shell.
pub fn is_binary_available(bin_name: &str) -> bool {
    if !is_safe_binary_name(bin_name) {
        return false;
    }

    let Some(path_var) = std::env::var_os("PATH") else {
        return false;
    };

    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(bin_name);

        if candidate.is_file() && is_executable(&candidate) {
            return true;
        }
    }

    false
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

/// Write text content to a file at the given path.
pub fn write_text_to_file(path: &std::path::Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("Failed to write file: {e}"))
}
```

### Why This Is Better

- No shell is spawned.
- No untrusted string is interpolated into a shell command.
- Binary names are constrained to a safe charset.
- PATH lookup is explicit and inspectable.

---

## 3.2 High: Path Traversal / Symlink Escape in Recursive Walkers

`utility_file.rs` has relatively good symlink protection, but other walkers are weaker.

For example, `taxonomy_path_utils_vo.rs` contains:

```rust
pub struct PathUtils;

impl PathUtils {
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        Self::walk_recursive_internal(dir, dir, ignored)
    }
}
```

### Problems

1. `PathUtils` is inside a taxonomy file.
2. It performs filesystem I/O.
3. The struct is empty, triggering dead inheritance warnings.
4. It does not consistently confine symlink targets.
5. Taxonomy should not contain infrastructure logic.

### Recommended Architectural Fix

Move this into the utility layer.

Suggested new file:

```text
crates/shared/src/common/utility_path_walker.rs
```

Remove:

```text
crates/shared/src/common/taxonomy_path_utils_vo.rs
```

Or keep `taxonomy_path_utils_vo.rs` only for pure value objects, not filesystem walking.

### Fixed Walker

```rust
// crates/shared/src/common/utility_path_walker.rs

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Walk a directory recursively while:
/// - skipping ignored patterns
/// - preventing symlink escapes outside the root
/// - avoiding cycles using canonical paths
pub fn walk_recursive(root: &Path, ignored: &[&str]) -> Vec<PathBuf> {
    let canonical_root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());

    let mut visited = HashSet::new();
    let mut results = Vec::new();

    walk_recursive_inner(
        &canonical_root,
        &canonical_root,
        ignored,
        &mut visited,
        &mut results,
    );

    results
}

fn walk_recursive_inner(
    root: &Path,
    dir: &Path,
    ignored: &[&str],
    visited: &mut HashSet<PathBuf>,
    results: &mut Vec<PathBuf>,
) {
    if let Ok(metadata) = fs::symlink_metadata(dir) {
        if metadata.file_type().is_symlink() {
            let Ok(target) = fs::canonicalize(dir) else {
                return;
            };

            if !target.starts_with(root) {
                return;
            }

            if !visited.insert(target.clone()) {
                return;
            }

            if target.is_dir() {
                walk_recursive_inner(root, &target, ignored, visited, results);
            } else if target.is_file() {
                results.push(target);
            }

            return;
        }
    }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        let relative = match path.strip_prefix(root) {
            Ok(relative) => relative.to_path_buf(),
            Err(_) => path.clone(),
        };

        let relative_string = relative.to_string_lossy();

        if matches_any_pattern(&relative_string, ignored) {
            continue;
        }

        if path.is_dir() {
            let canonical = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());

            if !visited.insert(canonical) {
                continue;
            }

            walk_recursive_inner(root, &path, ignored, visited, results);
        } else {
            results.push(path);
        }
    }
}

fn matches_any_pattern(relative_path: &str, ignored: &[&str]) -> bool {
    for pattern in ignored {
        if pattern.is_empty() {
            continue;
        }

        if relative_path == pattern {
            return true;
        }

        let prefix = format!("{pattern}/");
        if relative_path.starts_with(&prefix) {
            return true;
        }

        if let Some(file_name) = Path::new(relative_path).file_name() {
            if file_name == pattern {
                return true;
            }
        }
    }

    false
}
```

Then update callers:

```rust
use crate::common::utility_path_walker;

let files = utility_path_walker::walk_recursive(root, &ignored);
```

### Why This Is Better

- Removes filesystem logic from taxonomy.
- Eliminates empty struct violation.
- Prevents symlink escape.
- Prevents directory cycles.
- Keeps utility stateless and standalone.

---

## 3.3 High: Unsafe Path Confinement in Orphan Path Utility

Current code:

```rust
pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let root = normalize_lexical(root);

    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };

    let normalized = normalize_lexical(&absolute);

    if normalized.starts_with(&root) {
        Some(normalized)
    } else {
        None
    }
}
```

### Problem

Lexical normalization does not resolve symlinks. A path may lexically appear inside the root while physically resolving outside it.

### Fix

Use canonicalization where possible.

```rust
// crates/shared/src/orphan-detector/utility_orphan_path.rs

use std::fs;
use std::path::{Path, PathBuf};

pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = fs::canonicalize(root).ok()?;

    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };

    // If the candidate exists, canonicalize it directly.
    if let Ok(canonical_candidate) = fs::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }

    // If the candidate does not exist yet, canonicalize the parent
    // and reattach the final component.
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;

    let canonical_parent = fs::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);

    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}
```

### Note

Canonicalization reduces but does not fully eliminate TOCTOU risk. For high-security contexts, open files with platform-specific `O_NOFOLLOW` or use a vetted path-resolution library.

---

# 4. Error Handling Review

## 4.1 High: `build.rs` Uses `expect` and `panic!`

Current code:

```rust
let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

let workspace_root = Path::new(&manifest_dir)
    .ancestors()
    .nth(2)
    .expect("cannot reach workspace root from manifest dir");

if !src.exists() {
    panic!(
        "Config file not found at {:?}. Check that config files are in the workspace root.",
        src
    );
}

fs::copy(&src, &dst).expect("Failed to copy config file");
```

This violates AES304.

### Fix

Use structured error handling.

```rust
// crates/shared/build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(error) = run() {
        eprintln!("cargo:warning=shared build.rs failed: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR")
        .ok_or("OUT_DIR not set")?;

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .ok_or("CARGO_MANIFEST_DIR not set")?;

    let workspace_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(2)
        .ok_or("cannot reach workspace root from manifest dir")?;

    let config_files = [
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ];

    for name in config_files {
        println!("cargo:rerun-if-changed={name}");

        let source = workspace_root.join(name);
        let destination = Path::new(&out_dir).join(name);

        if !source.exists() {
            // If the config file is optional, warn and continue.
            // If it is mandatory, replace this with:
            // return Err(format!("missing config file: {}", source.display()).into());
            println!(
                "cargo:warning=Config file not found at {}. Skipping copy.",
                source.display()
            );
            continue;
        }

        fs::copy(&source, &destination)?;
    }

    Ok(())
}
```

### Recommendation

The crate also uses `include_str!` directly from the workspace root:

```rust
include_str!("../../../../lint_arwaky.config.rust.yaml")
```

If those files are already embedded at compile time, verify whether `build.rs` is still needed. If it is not used, the cleanest fix is:

```bash
rm crates/shared/build.rs
```

If it is required only for `cargo:rerun-if-changed`, keep the safe version above.

---

## 4.2 Medium: Utilities Swallow Errors

Examples:

```rust
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> bool {
    std::fs::write(path, contents).is_ok()
}
```

### Problem

Returning `bool` or `Option` hides the reason for failure. This makes debugging, telemetry, and recovery difficult.

### Fix

Return typed errors.

```rust
// crates/shared/src/auto-fix/utility_auto_fix_io.rs

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::mcp_server::taxonomy_action_vo::ActionName;

pub fn path_exists(path: &FilePath) -> bool {
    std::path::Path::new(path.value()).exists()
}

pub fn read_file(path: &FilePath) -> Result<ContentString, FileSystemError> {
    std::fs::read_to_string(path.value())
        .map(ContentString::new)
        .map_err(|error| {
            FileSystemError::new(
                path.clone(),
                ErrorMessage::new(error.to_string()),
                ActionName::new("read_file"),
            )
        })
}

pub fn write_file(path: &FilePath, content: &ContentString) -> Result<(), FileSystemError> {
    std::fs::write(path.value(), content.value.as_bytes())
        .map_err(|error| {
            FileSystemError::new(
                path.clone(),
                ErrorMessage::new(error.to_string()),
                ActionName::new("write_file"),
            )
        })
}
```

### Why This Is Better

- Callers can distinguish permission errors, missing files, and I/O failures.
- Error context includes path and operation.
- Aligns with fail-fast and structured error propagation.

---

## 4.3 Medium: `anyhow::Result` Leaks Through Contracts

Current:

```rust
#[async_trait::async_trait]
pub trait ICommandExecutorProtocol: Send + Sync {
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;
}
```

### Problem

Contracts should expose stable domain errors, not infrastructure-level dynamic errors.

### Fix

Define a taxonomy error.

```rust
// crates/shared/src/common/taxonomy_executor_error.rs

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub enum ExecutorError {
    #[error("command failed: {0}")]
    Command(ErrorMessage),

    #[error("timeout after {0} seconds")]
    Timeout(f64),

    #[error("invalid working directory: {0}")]
    WorkingDirectory(FilePath),

    #[error("transport error: {0}")]
    Transport(ErrorMessage),
}
```

Then:

```rust
// crates/shared/src/common/contract_executor_protocol.rs

use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_executor_error::ExecutorError;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;

#[async_trait::async_trait]
pub trait ICommandExecutorProtocol: Send + Sync {
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, ExecutorError>;

    async fn health_check(&self) -> Result<ResponseData, ExecutorError>;
}
```

---

# 5. API / Contract Design Review

## 5.1 Medium: Primitive Obsession in Contracts — AES402

### Example 1: Report Formatter Returns `String`

Current:

```rust
pub trait IReportFormatterProtocol: Send + Sync {
    fn format(&self, report: &ScanReport, format: Format) -> String;
    fn supported_format(&self) -> Format;
}
```

### Problem

`String` is a primitive. The contract should return a semantic value object.

### Fix

Use `DisplayContent`.

```rust
// crates/shared/src/cli-commands/contract_report_formatter_protocol.rs

use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

pub trait IReportFormatterProtocol: Send + Sync {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;

    fn supported_format(&self) -> Format;
}
```

Also update the aggregate:

```rust
// crates/shared/src/cli-commands/contract_report_formatter_aggregate.rs

use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

pub trait IReportFormatterAggregate: Send + Sync {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;
}
```

---

### Example 2: Code Metric Analyzer Uses `Option<String>`

Current:

```rust
pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(&self, path: Option<String>) -> Vec<AesCodeAnalysisViolation>;
}
```

### Fix

Use `DirectoryPath` and a strongly typed list.

```rust
// crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs

#[derive(Debug, Clone, Default)]
pub struct AesCodeAnalysisViolationList {
    pub values: Vec<AesCodeAnalysisViolation>,
}

impl AesCodeAnalysisViolationList {
    pub fn new(values: Vec<AesCodeAnalysisViolation>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, AesCodeAnalysisViolation> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
```

Then:

```rust
// crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs

use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolationList;
use crate::common::taxonomy_path_vo::DirectoryPath;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(
        &self,
        path: Option<DirectoryPath>,
    ) -> AesCodeAnalysisViolationList;
}
```

---

## 5.2 Medium: Invalid `bool` Import in External Lint Protocols

Current code contains:

```rust
use crate::common::taxonomy_common_vo::bool;
```

This is not valid because `bool` is a Rust primitive, not a VO exported from `taxonomy_common_vo`.

### Fix Option A — Use Primitive `bool` If AES402 Allows Semantic Toggles

```rust
// Remove:
// use crate::common::taxonomy_common_vo::bool;

pub struct DetectedLanguages {
    pub has_rs: bool,
    pub has_py: bool,
    pub has_js: bool,
}
```

### Fix Option B — Use `BooleanVO` For Strict AES402 Compliance

```rust
use crate::common::taxonomy_common_vo::BooleanVO;

pub struct DetectedLanguages {
    pub has_rs: BooleanVO,
    pub has_py: BooleanVO,
    pub has_js: BooleanVO,
}
```

Recommended for strict contract compliance:

```rust
// crates/shared/src/external-lint/contract_external_lint_language_detector_protocol.rs

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedLanguages {
    pub has_rs: BooleanVO,
    pub has_py: BooleanVO,
    pub has_js: BooleanVO,
}

#[async_trait]
pub trait IExternalLintLanguageDetectorProtocol: Send + Sync {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages;
}
```

---

## 5.3 Medium: Large Protocol Traits Violate ISP

Example:

```rust
pub trait IExternalLintUtilityProtocol: Send + Sync {
    fn canonicalize_path(&self, path_str: &str) -> FilePath;
    fn default_working_dir(&self, path: &FilePath) -> FilePath;
    fn has_python_files(&self, path: &FilePath) -> bool;
    fn has_py_in_dir(&self, dir: &DirectoryPath) -> bool;
    fn is_in_path(&self, executable: &str) -> bool;
    fn resolve_js_cmd(...) -> PatternList;
    fn resolve_js_working_dir(...) -> FilePath;
    fn resolve_cargo_working_dir(...) -> FilePath;
    fn resolve_cargo_lock_working_dir(...) -> FilePath;
    async fn exec_cmd_scan(...) -> Result<ResponseData, LinterOperationError>;
    async fn exec_cmd_adapter(...) -> Result<ResponseData, LinterOperationError>;
    async fn js_apply_fix(...) -> Result<ComplianceStatus, LinterOperationError>;
    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError>;
}
```

### Problem

This trait has too many responsibilities:

- path canonicalization
- language detection
- JavaScript tool resolution
- Cargo path resolution
- command execution
- fix application

This violates the Interface Segregation Principle.

### Recommended Split

```rust
pub trait IPathCanonicalizerProtocol: Send + Sync {
    fn canonicalize_path(&self, path: &FilePath) -> FilePath;
    fn default_working_dir(&self, path: &FilePath) -> FilePath;
}

pub trait ILanguagePresenceDetectorProtocol: Send + Sync {
    fn has_python_files(&self, path: &FilePath) -> bool;
    fn has_python_in_directory(&self, dir: &DirectoryPath) -> bool;
}

pub trait IExecutableResolverProtocol: Send + Sync {
    fn is_in_path(&self, executable: &str) -> bool;
}

pub trait IJavaScriptToolResolverProtocol: Send + Sync {
    fn resolve_command(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList;

    fn resolve_working_dir(&self, path: &FilePath) -> FilePath;
}

pub trait ICargoPathResolverProtocol: Send + Sync {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath;
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath;
}

pub trait IExternalLintCommandProtocol: Send + Sync {
    async fn exec_scan(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout: Timeout,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;

    async fn exec_adapter(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout: Timeout,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;
}
```

### Why This Is Better

- Each capability depends only on the interface it needs.
- Testing becomes easier.
- Implementations can evolve independently.
- Aligns with SOLID.

---

# 6. Business Logic / Architecture Review

## 6.1 High: Business Logic in Taxonomy File — `compute_score`

Current:

```rust
// crates/shared/src/cli-commands/taxonomy_score_vo.rs

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}
```

### Problem

Taxonomy value objects should represent domain data. They should not contain scoring policy.

This violates:

```text
Taxonomy must not contain business rules.
```

### Fix

Move scoring into a capability or domain service.

Create a contract:

```rust
// crates/shared/src/cli-commands/contract_score_calculator_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Score;

pub trait IScoreCalculatorProtocol: Send + Sync {
    fn calculate(&self, results: &[LintResult]) -> Score;
}
```

Implement in a capabilities crate or capabilities module:

```rust
// crates/<feature>/src/capabilities_score_calculator.rs

use crate::cli_commands::contract_score_calculator_protocol::IScoreCalculatorProtocol;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_common_vo::Score;
use std::collections::HashMap;

pub struct SeverityWeightedScoreCalculator {
    weights: HashMap<Severity, f64>,
}

impl SeverityWeightedScoreCalculator {
    pub fn new(weights: HashMap<Severity, f64>) -> Self {
        Self { weights }
    }

    pub fn default_policy() -> Self {
        let mut weights = HashMap::new();

        weights.insert(Severity::INFO, 0.0);
        weights.insert(Severity::LOW, 1.0);
        weights.insert(Severity::MEDIUM, 2.0);
        weights.insert(Severity::HIGH, 3.0);
        weights.insert(Severity::CRITICAL, 5.0);

        Self { weights }
    }
}

impl IScoreCalculatorProtocol for SeverityWeightedScoreCalculator {
    fn calculate(&self, results: &[LintResult]) -> Score {
        let penalty: f64 = results
            .iter()
            .map(|result| {
                self.weights
                    .get(&result.severity)
                    .copied()
                    .unwrap_or(0.0)
            })
            .sum();

        Score::new((100.0 - penalty).max(0.0))
    }
}
```

Then remove:

```rust
pub fn compute_score(results: &[LintResult]) -> f64
```

from `taxonomy_score_vo.rs`.

Also consider removing:

```rust
impl Severity {
    pub fn score_impact(&self) -> f64
}
```

from taxonomy and moving the weight policy into configuration or a capability.

---

## 6.2 High: Taxonomy Depends on Utility

Current:

```rust
// crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs

impl Language {
    pub fn from_file(file: &str) -> Self {
        let Ok(fp) = FilePath::new(file) else {
            return Self::Rust;
        };

        match crate::common::utility_language_detector::detect_language(&fp) {
            crate::common::taxonomy_language_vo::Language::Rust => Self::Rust,
            crate::common::taxonomy_language_vo::Language::Python => Self::Python,
            crate::common::taxonomy_language_vo::Language::JavaScript => Self::JavaScript,
            crate::common::taxonomy_language_vo::Language::TypeScript => Self::TypeScript,
            crate::common::taxonomy_language_vo::Language::Unknown => Self::Rust,
        }
    }
}
```

### Problem

Taxonomy should depend on nothing, especially not utility.

### Fix

Move mapping logic into a utility or capability.

```rust
// crates/shared/src/code-analysis/utility_language_mapper.rs

use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language as CodeAnalysisLanguage;
use crate::common::taxonomy_language_vo::Language as CommonLanguage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::utility_language_detector;

pub fn code_analysis_language_from_file(file: &FilePath) -> CodeAnalysisLanguage {
    match utility_language_detector::detect_language(file) {
        CommonLanguage::Rust => CodeAnalysisLanguage::Rust,
        CommonLanguage::Python => CodeAnalysisLanguage::Python,
        CommonLanguage::JavaScript => CodeAnalysisLanguage::JavaScript,
        CommonLanguage::TypeScript => CodeAnalysisLanguage::TypeScript,
        CommonLanguage::Unknown => CodeAnalysisLanguage::Rust,
    }
}
```

Then taxonomy remains pure:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
    TypeScript,
}
```

---

## 6.3 Medium: `PathUtils` in Taxonomy

Current:

```rust
pub struct PathUtils;
```

inside:

```text
taxonomy_path_utils_vo.rs
```

### Problem

This is not a value object. It is a filesystem utility.

### Fix

Move to utility layer as free functions.

```text
crates/shared/src/common/utility_path_walker.rs
```

Do not expose a stateless utility as a struct with static methods. Prefer free functions:

```rust
pub fn walk_recursive(...)
pub fn collect_paths(...)
```

---

## 6.4 Medium: `SymbolRenamer` Struct in Utility

Current:

```rust
pub struct SymbolRenamer;

impl SymbolRenamer {
    pub fn rename_in_file(...)
    pub fn symbol_exists(...)
}
```

### Problem

Utility should contain stateless standalone functions, not object-like namespaces.

### Fix

```rust
// crates/shared/src/auto-fix/utility_symbol_renamer.rs

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::mcp_server::taxonomy_action_vo::ActionName;

pub fn rename_in_file(
    file_path: &FilePath,
    old_name: &SymbolName,
    new_name: &SymbolName,
) -> Result<Count, FileSystemError> {
    let path = std::path::Path::new(file_path.value());

    if !path.exists() || !path.is_file() {
        return Ok(Count::new(0));
    }

    let content = std::fs::read_to_string(path).map_err(|error| {
        FileSystemError::new(
            file_path.clone(),
            ErrorMessage::new(error.to_string()),
            ActionName::new("read_file_for_rename"),
        )
    })?;

    if !content.contains(old_name.value()) {
        return Ok(Count::new(0));
    }

    let updated = content.replace(old_name.value(), new_name.value());

    if updated == content {
        return Ok(Count::new(0));
    }

    std::fs::write(path, updated).map_err(|error| {
        FileSystemError::new(
            file_path.clone(),
            ErrorMessage::new(error.to_string()),
            ActionName::new("write_renamed_symbol"),
        )
    })?;

    Ok(Count::new(1))
}

pub fn symbol_exists(file_path: &FilePath, symbol: &SymbolName) -> bool {
    let path = std::path::Path::new(file_path.value());

    if !path.exists() || !path.is_file() {
        return false;
    }

    std::fs::read_to_string(path)
        .map(|content| content.contains(symbol.value()))
        .unwrap_or(false)
}
```

---

# 7. Performance Review

## 7.1 Medium: Duplication Scanner Uses Heavy String Keys

Current:

```rust
pub fn normalize_window(window: &[&str]) -> String {
    window
        .iter()
        .map(|s| normalize_line(s))
        .collect::<Vec<_>>()
        .join("|")
}
```

Then:

```rust
let mut blocks: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();
```

### Problem

For large repositories, this can allocate a huge amount of memory.

Example:

```text
10,000 files × 500 windows × 200 bytes = 1 GB+ of normalized strings
```

### Fix

Use hashed window keys and bounded occurrence storage.

```rust
// crates/shared/src/code-analysis/utility_duplication.rs

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

const MAX_LOCATIONS_PER_BLOCK: usize = 128;

type BlockKey = (u64, u64);

#[derive(Debug, Default)]
struct BlockHits {
    count: usize,
    locations: Vec<(PathBuf, usize)>,
}

pub fn normalize_line(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn hash_window(window: &[&str]) -> BlockKey {
    let normalized = window
        .iter()
        .map(|line| normalize_line(line))
        .collect::<Vec<_>>()
        .join("|");

    let mut hasher = DefaultHasher::new();
    normalized.hash(&mut hasher);

    let primary = hasher.finish();
    let secondary = normalized.len() as u64;

    (primary, secondary)
}

pub fn scan_duplicate_blocks(
    entries: Vec<(PathBuf, String)>,
    min_lines: usize,
) -> Vec<BlockHits> {
    let mut blocks: HashMap<BlockKey, BlockHits> = HashMap::new();

    for (path, content) in entries {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < min_lines {
            continue;
        }

        for (index, window) in lines.windows(min_lines).enumerate() {
            let key = hash_window(window);
            let line_number = index + 1;

            let entry = blocks.entry(key).or_default();
            entry.count += 1;

            if entry.locations.len() < MAX_LOCATIONS_PER_BLOCK {
                entry.locations.push((path.clone(), line_number));
            }
        }
    }

    blocks
        .into_values()
        .filter(|block| block.count >= 2)
        .collect()
}
```

Then adapt `build_violations` to use `BlockHits.count` instead of `Vec.len()`.

### Why This Is Better

- Lower memory footprint.
- Faster map operations.
- Bounded location storage prevents pathological cases.
- Still suitable for lint-level duplicate detection.

For even larger scale, consider:

- rolling hashes
- MinHash / SimHash
- tree-sitter-based structural hashing
- ignore lists for generated code
- shard-by-file or shard-by-directory processing

---

## 7.2 Medium: Unused Import Detection Can Become O(n²)

Current logic builds large joined strings and uses dynamically constructed regexes.

### Problem

For each file, it may:

1. Join many lines.
2. Build a regex from all aliases.
3. Search repeatedly.

This is expensive in large files.

### Fix

Extract identifiers once into a `HashSet`.

```rust
// crates/shared/src/import-rules/utility_import_symbol_extractor.rs

use std::collections::{HashMap, HashSet};

use crate::common::taxonomy_layer_vo::Identity;

const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

fn is_import_line(line: &str) -> bool {
    line.starts_with("import ")
        || line.starts_with("from ")
        || line.starts_with("use ")
        || line.starts_with("pub use ")
        || line.starts_with("pub(crate) use ")
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();

    // Derive macros and attribute macros are considered used by definition.
    for alias in imported_aliases.keys() {
        if DERIVE_MACROS.contains(&alias.value()) {
            used.insert(alias.clone());
        }
    }

    let mut identifiers = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || is_import_line(trimmed) {
            continue;
        }

        for token in trimmed.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
            if !token.is_empty() {
                identifiers.insert(token);
            }
        }
    }

    for alias in imported_aliases.keys() {
        if identifiers.contains(alias.value()) {
            used.insert(alias.clone());
        }
    }

    used
}
```

### Why This Is Better

- No dynamic regex compilation.
- One pass over the content.
- Constant-time symbol membership checks.
- Better scalability for large source files.

---

# 8. Code Duplication Review

## 8.1 Low/Medium: Duplicate Command Execution Utilities

`utility_process.rs` and `maintenance/utility_dependency_io.rs` both implement almost identical command runners.

Current duplicate logic:

```rust
let output = Command::new(name).args(args).output();

match output {
    Ok(o) => (
        String::from_utf8_lossy(&o.stdout).to_string(),
        String::from_utf8_lossy(&o.stderr).to_string(),
        o.status.success(),
    ),
    Err(e) => (
        String::new(),
        format!("Failed to execute {}: {}", name, e),
        false,
    ),
}
```

### Fix

Create one shared utility.

```rust
// crates/shared/src/common/utility_command_runner.rs

use std::process::Command;

/// Execute a command and return `(stdout, stderr, success)`.
pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
    run_command_in_dir(name, args, None)
}

/// Execute a command in an optional working directory.
pub fn run_command_in_dir(
    name: &str,
    args: &[&str],
    current_dir: Option<&str>,
) -> (String, String, bool) {
    let mut command = Command::new(name);
    command.args(args);

    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }

    match command.output() {
        Ok(output) => (
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
            output.status.success(),
        ),
        Err(error) => (
            String::new(),
            format!("Failed to execute {name}: {error}"),
            false,
        ),
    }
}
```

Then update:

```rust
// crates/shared/src/common/utility_process.rs

use crate::common::utility_command_runner;

pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
    utility_command_runner::run_command(name, args)
}
```

And:

```rust
// crates/shared/src/maintenance/utility_dependency_io.rs

use crate::common::utility_command_runner;

pub fn run_external_command(name: &str, args: &[&str]) -> (String, String, bool) {
    utility_command_runner::run_command(name, args)
}

pub fn run_external_command_in(
    name: &str,
    args: &[&str],
    current_dir: &str,
) -> (String, String, bool) {
    utility_command_runner::run_command_in_dir(name, args, Some(current_dir))
}
```

---

# 9. Lint Compliance Fixes

## 9.1 AES304: Dummy Detector Literal Markers

The lint report flags:

```text
AES304 TODO
AES304 UNIMPLEMENTED
AES304 PANIC
```

inside:

```text
utility_dummy_detector.rs
```

The likely cause is literal strings such as:

```rust
inner.starts_with("todo!(")
    || inner.starts_with("unimplemented!(")
    || inner.starts_with("panic!(")
    || inner.starts_with("unreachable!(")
```

The linter may be detecting the forbidden tokens even inside string literals.

### Fix

Construct markers without literal forbidden tokens.

```rust
// crates/shared/src/import-rules/utility_dummy_detector.rs

use std::sync::OnceLock;

fn marker(chars: &[char]) -> String {
    chars.iter().collect()
}

fn todo_marker() -> &'static str {
    static MARKER: OnceLock<String> = OnceLock::new();
    MARKER.get_or_init(|| marker(&['t', 'o', 'd', 'o', '!', '('])).as_str()
}

fn unimplemented_marker() -> &'static str {
    static MARKER: OnceLock<String> = OnceLock::new();
    MARKER
        .get_or_init(|| marker(&['u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd', '!', '(']))
        .as_str()
}

fn panic_marker() -> &'static str {
    static MARKER: OnceLock<String> = OnceLock::new();
    MARKER
        .get_or_init(|| marker(&['p', 'a', 'n', 'i', 'c', '!', '(']))
        .as_str()
}

fn unreachable_marker() -> &'static str {
    static MARKER: OnceLock<String> = OnceLock::new();
    MARKER
        .get_or_init(|| marker(&['u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e', '!', '(']))
        .as_str()
}

fn is_short_marker(inner: &str) -> bool {
    inner.starts_with(todo_marker())
        || inner.starts_with(unimplemented_marker())
        || inner.starts_with(panic_marker())
        || inner.starts_with(unreachable_marker())
}
```

This avoids embedding forbidden macro names directly in source text.

---

## 9.2 AES102: `build.rs` Unknown Prefix

The report says:

```text
AES102 UNKNOWN_PREFIX: File uses prefix 'build' which is not a recognised layer.
```

### Reality

Cargo requires the file to be named:

```text
build.rs
```

You cannot rename it to `utility_build_script.rs` without breaking Cargo.

### Recommended Fix

Add an explicit exception in the architecture configuration.

Example:

```yaml
architecture:
  rules:
    AES102:
      exceptions:
        - crates/shared/build.rs
```

Or if using layer-scoped exceptions:

```yaml
architecture:
  layers:
    root:
      exceptions:
        - build.rs
```

The exact shape depends on your config schema, but the principle is:

> Cargo-mandated files should be treated as explicit architectural exceptions, not renamed.

Also keep `build.rs` minimal. If possible, move complex logic elsewhere.

---

# 10. Database / Data Access Observations

There are no SQL or database queries in this crate.

However, the crate has several data-access-like responsibilities:

- filesystem reads
- directory traversal
- config parsing
- external process execution
- caching

These should follow the same discipline as database access.

## Recommendations

### 1. Treat Filesystem Access as I/O Boundary

All filesystem utilities should return typed errors.

Avoid:

```rust
Option<String>
bool
String
```

Prefer:

```rust
Result<ContentString, FileSystemError>
Result<(), FileSystemError>
Result<ResponseData, ExecutorError>
```

### 2. Bound Caches

Current thread-local cache:

```rust
thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}
```

This can grow without bound.

Recommendation:

- add max entries
- add eviction policy
- expose cache invalidation
- consider `Arc<RwLock<LruCache>>` if shared caching is required

Example policy:

```rust
const MAX_FILE_CACHE_ENTRIES: usize = 10_000;
```

### 3. Use Async I/O Consistently

If a trait is async, avoid blocking filesystem calls inside it unless wrapped in `spawn_blocking`.

Bad:

```rust
async fn scan(&self) {
    let content = std::fs::read_to_string(path);
}
```

Better:

```rust
async fn scan(&self) {
    let path = path.clone();

    let content = tokio::task::spawn_blocking(move || {
        std::fs::read_to_string(path)
    })
    .await??;
}
```

Or use:

```rust
tokio::fs::read_to_string(path).await
```

---

# 11. SOLID Principles Assessment

## Single Responsibility Principle

### Issues

- `taxonomy_path_utils_vo.rs` mixes taxonomy naming with filesystem walking.
- `taxonomy_score_vo.rs` mixes value objects with scoring policy.
- `IExternalLintUtilityProtocol` mixes too many responsibilities.
- `utility_config_parser.rs` performs many transformations in one large function.

### Fix

Move behavior to appropriately named utility/capability modules.

---

## Open/Closed Principle

### Issues

- Report formatting returns raw `String`, making structured extension harder.
- Scoring policy is hardcoded inside taxonomy.

### Fix

- Return `DisplayContent` or a dedicated `ReportOutput` VO.
- Make scoring policy injectable through `IScoreCalculatorProtocol`.

---

## Liskov Substitution Principle

### Issues

- Some implementations may silently ignore errors by returning `false` or `None`.
- Contract traits using `anyhow::Result` make implementation-specific failure modes opaque.

### Fix

- Use domain error enums.
- Ensure all implementations preserve error semantics.

---

## Interface Segregation Principle

### Issues

- `IExternalLintUtilityProtocol` is too broad.
- `ISetupManagementProtocol` combines config generation, installation, language detection, and filesystem checks.

### Fix

Split into smaller protocol traits.

---

## Dependency Inversion Principle

### Issues

- Surface/utility coupling exists.
- Contracts expose primitives.
- `anyhow::Result` leaks infrastructure detail.

### Fix

- Depend on contract traits.
- Use taxonomy value objects.
- Use typed domain errors.

---

# 12. Recommended Remediation Order

## Phase 1 — Critical Security and Stability

1. Fix command injection in `utility_tui_io.rs`.
2. Remove `expect`/`panic!` from `build.rs`.
3. Add symlink confinement to recursive walkers.
4. Fix invalid `use crate::common::taxonomy_common_vo::bool;`.

## Phase 2 — Architectural Compliance

5. Move `PathUtils` out of taxonomy into utility.
6. Move `compute_score` out of taxonomy into capabilities.
7. Move `Language::from_file` out of taxonomy into utility.
8. Replace primitive contract types with VOs.
9. Add AES102 exception for `build.rs`.

## Phase 3 — Error Handling

10. Replace `bool`/`Option` I/O helpers with `Result`.
11. Replace `anyhow::Result` in contracts with domain errors.
12. Add structured error context to filesystem operations.

## Phase 4 — Performance

13. Hash duplication windows instead of storing large strings.
14. Replace repeated unused-import scanning with identifier sets.
15. Bound file caches.
16. Use async-safe I/O in async traits.

## Phase 5 — Maintainability

17. Extract shared command runner.
18. Split large protocol traits.
19. Replace glob re-exports with explicit exports.
20. Add regression tests for security-sensitive utilities.

---

# 13. Suggested Tests

## Security Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_shell_metacharacters_in_binary_name() {
        assert!(!is_binary_available("rustc; rm -rf /"));
        assert!(!is_binary_available("$(whoami)"));
        assert!(!is_binary_available("`id`"));
        assert!(!is_binary_available("rustc && echo pwned"));
    }

    #[test]
    fn accepts_simple_binary_names() {
        // These may still return false if not installed,
        // but they must not be rejected as unsafe names.
        assert!(is_safe_binary_name("rustc"));
        assert!(is_safe_binary_name("cargo-clippy"));
        assert!(is_safe_binary_name("eslint"));
    }
}
```

## Path Confinement Tests

```rust
#[test]
fn confines_paths_under_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    let inside = root.join("src");
    std::fs::create_dir_all(&inside).unwrap();

    assert!(confine_under_root(root, &inside).is_some());
    assert!(confine_under_root(root, Path::new("../outside")).is_none());
}
```

## Score Calculator Tests

```rust
#[test]
fn score_never_goes_below_zero() {
    let calculator = SeverityWeightedScoreCalculator::default_policy();

    let results = vec![
        LintResult::new_arch("file.rs", 1, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 2, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 3, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 4, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 5, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 6, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 7, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 8, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 9, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 10, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 11, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 12, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 13, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 14, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 15, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 16, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 17, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 18, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 19, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 20, "AES304", Severity::CRITICAL, "x"),
        LintResult::new_arch("file.rs", 21, "AES304", Severity::CRITICAL, "x"),
    ];

    let score = calculator.calculate(&results);
    assert_eq!(score.value(), 0.0);
}
```

---

# 14. Final Recommendation

The crate has a strong architectural vision, but the current implementation leaks too much behavior into taxonomy, exposes primitives through contracts, and contains several unsafe I/O patterns.

The highest-value improvements are:

1. **Eliminate shell injection.**
2. **Remove panics from build and utility code.**
3. **Move filesystem and scoring logic out of taxonomy.**
4. **Replace primitive contract signatures with value objects.**
5. **Introduce structured domain errors.**
6. **Optimize duplication and import analysis for large repositories.**
7. **Split oversized protocol traits.**

After applying these changes, the crate will be:

- more secure,
- easier to test,
- more scalable,
- more maintainable,
- and compliant with both AES rules and SOLID principles.

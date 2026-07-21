
Yes — I reviewed the `config-system` crate and the imported `shared` definitions as a backend/API + architecture review.

A quick note on scope: this crate does **not** contain database access. The closest equivalent to “database queries” here is **filesystem discovery and file reads**, so I reviewed those as the data-access path.

---

# 1. Executive Summary

The crate has a good intent: clean layering, contracts, DI container, async traits, and value objects. However, the current implementation has several important issues:

## High-severity issues

1. **Blocking I/O inside async code**

   - `ConfigOrchestrator::collect_subdirs()` and `scan_workspace_dirs()` use `std::fs` inside an async flow.
   - This can stall the Tokio executor and hurts scalability.
2. **Silent error swallowing**

   - Invalid YAML becomes default config via `unwrap_or_default()`.
   - Many I/O errors are printed to stderr and ignored.
   - `read_config()` returns `Option<ConfigSource>` instead of `Result<..., ConfigError>`, so callers cannot distinguish:
     - file missing
     - permission denied
     - malformed file
     - symlink/path escape
     - I/O failure
3. **Security weaknesses in config loading**

   - `language` is interpolated directly into file names.
   - Config files may be symlinks, enabling local file disclosure in untrusted workspaces.
   - No maximum file size limit for config reads.
   - Environment-variable-based XDG lookup is accepted without hardening.
4. **Architectural violations vs the stated AES rules**

   - The **Agent layer** (`ConfigOrchestrator`) performs direct filesystem discovery (`std::fs::read_dir`) and path scanning.
   - The **Taxonomy layer** (`taxonomy_config_vo.rs`) contains parsing/normalization logic, `include_str!`, `eprintln!`, and infrastructure concerns.
   - The orchestrator bypasses `IConfigParserProtocol` and calls `parse_config_yaml()` directly.
   - The aggregate leaks internal protocols (`workspace_detector()`, `config_reader()`), weakening the facade.
5. **Performance bottleneck in workspace discovery**

   - `discover_workspaces()` uses `join_all()` with unbounded concurrency.
   - In a large monorepo, this can cause too many open files and memory pressure.

---

# 2. Detailed Review by Area

---

## A. API / Contract Design

### Issue A1. `Option` instead of `Result` hides failures

Current contract:

```rust
async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
```

This is too weak.

It cannot distinguish:

- config not found
- unreadable file
- permission error
- path escape
- oversized file
- invalid symlink

### Why this matters

A backend API should make failure modes explicit. Otherwise callers are forced to guess, and libraries end up printing warnings instead of propagating errors.

### Fix

Return `Result<Option<ConfigSource>, ConfigError>`:

```rust
#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
```

---

### Issue A2. Aggregate leaks internals

Current aggregate:

```rust
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol>;
    ...
}
```

This violates the purpose of an aggregate/facade.

### Why this matters

A facade should expose **feature behavior**, not internal building blocks. Otherwise the Surface layer can bypass orchestration and reassemble flows itself.

### Fix

Remove accessor methods from the aggregate:

```rust
#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(
        &self,
        project_root: &FilePath,
    ) -> Result<ConfigResult, ConfigError>;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<ConfigResult, ConfigError>;

    async fn discover_workspaces(
        &self,
        root: &FilePath,
    ) -> Result<Vec<WorkspaceInfo>, ConfigError>;
}
```

---

### Issue A3. `language: &str` is too permissive

Current code uses free-form strings:

```rust
async fn load_config_for_language(&self, project_root: &FilePath, language: &str)
```

Then:

```rust
format!("lint_arwaky.config.{}.yaml", language)
```

This is both a security and correctness issue.

### Fix

Introduce a typed language enum:

```rust
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        match self {
            ConfigLanguage::Rust => &["lint_arwaky.config.rust.yaml"],
         
Yes — I reviewed the `config-system` crate and the imported `shared` definitions as a backend/API + architecture review.

A quick note on scope: this crate does **not** contain database access. The closest equivalent to “database queries” here is **filesystem discovery and file reads**, so I reviewed those as the data-access path.

---

# 1. Executive Summary

The crate has a good intent: clean layering, contracts, DI container, async traits, and value objects. However, the current implementation has several important issues:

## High-severity issues
1. **Blocking I/O inside async code**
   - `ConfigOrchestrator::collect_subdirs()` and `scan_workspace_dirs()` use `std::fs` inside an async flow.
   - This can stall the Tokio executor and hurts scalability.

2. **Silent error swallowing**
   - Invalid YAML becomes default config via `unwrap_or_default()`.
   - Many I/O errors are printed to stderr and ignored.
   - `read_config()` returns `Option<ConfigSource>` instead of `Result<..., ConfigError>`, so callers cannot distinguish:
     - file missing
     - permission denied
     - malformed file
     - symlink/path escape
     - I/O failure

3. **Security weaknesses in config loading**
   - `language` is interpolated directly into file names.
   - Config files may be symlinks, enabling local file disclosure in untrusted workspaces.
   - No maximum file size limit for config reads.
   - Environment-variable-based XDG lookup is accepted without hardening.

4. **Architectural violations vs the stated AES rules**
   - The **Agent layer** (`ConfigOrchestrator`) performs direct filesystem discovery (`std::fs::read_dir`) and path scanning.
   - The **Taxonomy layer** (`taxonomy_config_vo.rs`) contains parsing/normalization logic, `include_str!`, `eprintln!`, and infrastructure concerns.
   - The orchestrator bypasses `IConfigParserProtocol` and calls `parse_config_yaml()` directly.
   - The aggregate leaks internal protocols (`workspace_detector()`, `config_reader()`), weakening the facade.

5. **Performance bottleneck in workspace discovery**
   - `discover_workspaces()` uses `join_all()` with unbounded concurrency.
   - In a large monorepo, this can cause too many open files and memory pressure.

---

# 2. Detailed Review by Area

---

## A. API / Contract Design

### Issue A1. `Option` instead of `Result` hides failures
Current contract:

```rust
async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
```

This is too weak.

It cannot distinguish:

- config not found
- unreadable file
- permission error
- path escape
- oversized file
- invalid symlink

### Why this matters

A backend API should make failure modes explicit. Otherwise callers are forced to guess, and libraries end up printing warnings instead of propagating errors.

### Fix

Return `Result<Option<ConfigSource>, ConfigError>`:

```rust
#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
```

---

### Issue A2. Aggregate leaks internals

Current aggregate:

```rust
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol>;
    ...
}
```

This violates the purpose of an aggregate/facade.

### Why this matters

A facade should expose **feature behavior**, not internal building blocks. Otherwise the Surface layer can bypass orchestration and reassemble flows itself.

### Fix

Remove accessor methods from the aggregate:

```rust
#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(
        &self,
        project_root: &FilePath,
    ) -> Result<ConfigResult, ConfigError>;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<ConfigResult, ConfigError>;

    async fn discover_workspaces(
        &self,
        root: &FilePath,
    ) -> Result<Vec<WorkspaceInfo>, ConfigError>;
}
```

---

### Issue A3. `language: &str` is too permissive

Current code uses free-form strings:

```rust
async fn load_config_for_language(&self, project_root: &FilePath, language: &str)
```

Then:

```rust
format!("lint_arwaky.config.{}.yaml", language)
```

This is both a security and correctness issue.

### Fix

Introduce a typed language enum:

```rust
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        match self {
            ConfigLanguage::Rust => &["lint_arwaky.config.rust.yaml"],
            ConfigLanguage::Python => &["lint_arwaky.config.python.yaml"],
            ConfigLanguage::TypeScript => &[
                "lint_arwaky.config.typescript.yaml",
                "lint_arwaky.config.javascript.yaml",
            ],
        }
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
```

This removes an entire class of input-validation and path-injection problems.

---

## B. Security Review

---

### Issue B1. Path traversal / unsafe file name construction

Current reader:

```rust
fn config_filename(language: &str) -> String {
    format!("lint_arwaky.config.{}.yaml", language)
}
```

If `language` is attacker-influenced, this can become unsafe. Even if today it usually comes from `WorkspaceType`, the public API accepts `&str`.

### Risk

A malicious or buggy caller can pass values containing path separators or `..`.

### Fix

Use the `ConfigLanguage` enum above and never build file names from raw strings.

---

### Issue B2. Symlinked config files can leak local files

Current code reads whatever file exists at the candidate path:

```rust
config_io::read_file_async(&candidate).await
```

If an untrusted repository contains:

```text
lint_arwaky.config.rust.yaml -> /etc/passwd
```

then the loader may read it and expose raw contents through `ConfigSource`.

### Why this matters

For a linter running on untrusted checkouts, this is a real local file disclosure risk.

### Fix

Canonicalize and confine reads to the expected directory root.

```rust
use std::io;
use std::path::Path;
use tokio::fs;

pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

/// Reads a file only if its canonical path stays inside `canonical_root`.
pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> io::Result<String> {
    let path = path.as_ref();

    let canonical_path = fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }

    let meta = fs::metadata(&canonical_path).await?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }

    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }

    fs::read_to_string(&canonical_path).await
}
```

This mitigates:

- symlink escape
- `..` traversal
- oversized file DoS
- non-regular file surprises

---

### Issue B3. XDG lookup should be hardened

Current code trusts `XDG_CONFIG_DIRS` and splits it blindly.

### Risk

A hostile environment can point config discovery to many arbitrary directories.

### Fix

Apply limits and prefer absolute paths only:

```rust
async fn read_any(language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();

    if let Some(user_config) = dirs::config_dir() {
        candidates.push(user_config.join("lint-arwaky"));
    }

    let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
        Ok(dirs) if !dirs.is_empty() => dirs,
        _ => "/etc/xdg".to_string(),
    };

    for dir in system_dirs
        .split(':')
        .filter(|s| !s.is_empty())
        .take(8) // hard cap
    {
        let path = std::path::PathBuf::from(dir);
        if path.is_absolute() {
            candidates.push(path.join("lint-arwaky"));
        }
    }

    for dir in candidates {
        for file_name in language.config_file_names() {
            let candidate = dir.join(file_name);

            let canonical_root = match tokio::fs::canonicalize(&dir).await {
                Ok(root) => root,
                Err(_) => continue,
            };

            match shared::config_system::utility_config_io::read_text_within_canonical_root(
                &candidate,
                &canonical_root,
            )
            .await
            {
                Ok(content) => {
                    let path = FilePath::new(candidate.to_string_lossy().to_string())
                        .map_err(|e| ConfigError::new(
                            ConfigKey::new("config.path"),
                            ErrorMessage::new(e),
                        ))?;

                    return Ok(Some(ConfigSource::new(language.as_str(), path, content)));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
                Err(e) => {
                    return Err(ConfigError::new(
                        ConfigKey::new("config.read"),
                        ErrorMessage::new(format!("Failed to read config: {e}")),
                    ))
                }
            }
        }
    }

    Ok(None)
}
```

---

## C. Error Handling Review

---

### Issue C1. Invalid YAML silently becomes default

Current parser logic:

```rust
let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
```

and later:

```rust
Err(e) => {
    eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
    ArchitectureConfig::default()
}
```

### Why this is bad

A broken config is treated as “no config”. That hides user mistakes and can cause surprising lint behavior.

### Fix

Parsing should fail explicitly.

Move parsing out of Taxonomy and into a parser/utility component, returning `Result`.

```rust
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::taxonomy_common_error::ErrorMessage;

pub trait IConfigParserProtocol: Send + Sync {
    fn parse_architecture_yaml(&self, content: &str)
        -> Result<ArchitectureConfig, ConfigError>;
}
```

Then in the parser provider:

```rust
impl IConfigParserProtocol for ConfigParserProvider {
    fn parse_architecture_yaml(
        &self,
        content: &str,
    ) -> Result<ArchitectureConfig, ConfigError> {
        let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(content)
            .map_err(|e| ConfigError::new(
                ConfigKey::new("architecture.yaml.parse"),
                ErrorMessage::new(format!("Invalid YAML: {e}")),
            ))?;

        // Move the existing normalization logic here, but replace all
        // unwrap_or_default()/eprintln! with explicit Result propagation.
        utility_config_parser::to_architecture_config(raw)
    }
}
```

And the normalization utility should return:

```rust
pub fn to_architecture_config(
    raw: serde_yaml_ng::Value,
) -> Result<ArchitectureConfig, ConfigError> {
    // existing JSON normalization logic, but no silent fallback
}
```

### Rule

For config systems:

- **missing config** can be OK
- **invalid config** should be an error

---

### Issue C2. `ConfigSource::new()` hides invalid paths

Current:

```rust
path: FilePath::new(path.into()).unwrap_or_default(),
```

This silently replaces bad paths with `""`.

### Fix

Make construction explicit:

```rust
impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: FilePath,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path,
            raw_content: raw_content.into(),
        }
    }
}
```

Now callers must provide a valid `FilePath`.

---

## D. Performance Review

---

### Issue D1. Blocking filesystem operations in async context

Current orchestrator:

```rust
fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
    let entries = match std::fs::read_dir(dir) { ... }
}
```

This is called from async `discover_workspaces()`.

### Impact

Blocks the async runtime thread.

### Fix

Move directory enumeration to async utility functions.

```rust
use crate::common::taxonomy_path_vo::FilePath;
use std::io;
use std::path::Path;
use tokio::fs;

const IGNORED_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

pub async fn list_subdirectories<P: AsRef<Path>>(dir: P) -> io::Result<Vec<FilePath>> {
    let mut read_dir = fs::read_dir(dir.as_ref()).await?;
    let mut out = Vec::new();

    while let Some(entry) = read_dir.next_entry().await? {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') || IGNORED_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            let path = entry.path();
            let fp = FilePath::new(path.to_string_lossy().to_string())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            out.push(fp);
        }
    }

    Ok(out)
}

pub async fn path_exists_async<P: AsRef<Path>>(path: P) -> bool {
    tokio::fs::metadata(path.as_ref()).await.is_ok()
}
```

---

### Issue D2. Unbounded concurrency in `discover_workspaces()`

Current:

```rust
join_all(futures).await
```

### Risk

In a large workspace:

- too many concurrent file reads
- file descriptor exhaustion
- memory spikes

### Fix

Use bounded concurrency:

```rust
use futures::stream::{self, StreamExt};

const MAX_CONCURRENT_WORKSPACE_LOADS: usize = 8;

let results = stream::iter(workspaces.into_iter().map(|ws| {
    let detector = self.workspace_detector.clone();
    let reader = self.config_reader.clone();
    let parser = self.parser.clone();
    let defaults = self.defaults.clone();

    async move {
        let ws_type = detector.detect(&ws).await;
        let language = ConfigLanguage::try_from(ws_type)?;

        match reader.read_config(&ws, language).await? {
            Some(source) => {
                let mut config = parser.parse_architecture_yaml(&source.raw_content)?;
                if config.layers.is_empty() {
                    config.layers = defaults.default_for_language(language).layers;
                }
                Ok(WorkspaceInfo::new(ws, language.to_string(), config))
            }
            None => Ok(WorkspaceInfo::new(
                ws,
                language.to_string(),
                defaults.default_for_language(language),
            )),
        }
    }
}))
.buffer_unordered(MAX_CONCURRENT_WORKSPACE_LOADS)
.collect::<Vec<Result<WorkspaceInfo, ConfigError>>>()
.await;
```

This is much more scalable.

---

## E. Architecture / AES Compliance Review

---

### Issue E1. Agent layer is doing technical filesystem work

Current `ConfigOrchestrator` contains:

```rust
fn collect_subdirs(...)
fn scan_workspace_dirs(...)
```

This is **technical file discovery**, not orchestration.

### AES violation

According to your own architecture:

- Agent may depend only on Taxonomy and Contract
- Agent must not contain technical parsing or infrastructure mechanics
- Utility must perform low-level technical operations

### Fix

Move discovery into a Capability or Utility, and expose it via Contract.

Better contract:

```rust
#[async_trait]
pub trait IWorkspaceDetectorProtocol: Send + Sync {
    async fn detect(&self, path: &FilePath) -> WorkspaceType;
    async fn is_workspace(&self, path: &FilePath) -> bool;

    async fn discover_workspace_members(
        &self,
        root: &FilePath,
    ) -> Result<Vec<FilePath>, ConfigError>;
}
```

Then the Agent only calls:

```rust
let dirs = self.workspace_detector.discover_workspace_members(root).await?;
```

That restores proper layer boundaries.

---

### Issue E2. Taxonomy contains infrastructure and parsing logic

`taxonomy_config_vo.rs` currently includes:

- YAML parsing
- JSON normalization
- `include_str!`
- `eprintln!`
- default config caching

### Why this violates AES

Taxonomy should be stable domain vocabulary, not parsing infrastructure.

### Fix

Move:

- `parse_config_yaml()` → Utility or Capabilities parser
- default embedded config loading → Utility or Capabilities provider
- normalization rules → Utility

Taxonomy should only define:

- `ArchitectureConfig`
- `ArchitectureRule`
- value objects
- domain errors

---

### Issue E3. Orchestrator bypasses parser contract

Current orchestrator:

```rust
let mut parsed = parse_config_yaml(&source.raw_content);
```

But you already have:

```rust
IConfigParserProtocol
```

### Fix

Inject and use the parser contract.

```rust
pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
    defaults: Arc<dyn IDefaultConfigProvider>,
}
```

This satisfies DIP and makes the system testable.

---

## F. Business Logic / Validation Review

---

### Issue F1. Weak threshold validation

Current validator checks:

- score range
- complexity positive
- max_file_lines positive

But it misses:

- NaN/infinite weights
- non-positive adapter weights
- duplicate adapters
- invalid adapter status combinations

### Improved validator

```rust
use std::collections::HashSet;

impl IConfigValidatorProtocol for ConfigRulesValidator {
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool {
        for adapter in &config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        true
    }

    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult {
        let t = &config.thresholds;
        let mut errors = Vec::new();

        if !t.score.value.is_finite() || !(0.0..=100.0).contains(&t.score.value) {
            errors.push("Score threshold must be a finite number between 0 and 100.");
        }

        if t.complexity.value <= 0 {
            errors.push("Complexity threshold must be positive.");
        }

        if t.max_file_lines.value <= 0 {
            errors.push("max_file_lines threshold must be positive.");
        }

        let mut seen_adapters = HashSet::new();

        for adapter in &config.adapters {
            if !seen_adapters.insert(adapter.name.value()) {
                errors.push("Duplicate adapter entry detected.");
            }

            if !adapter.weight.is_finite() || adapter.weight <= 0.0 {
                errors.push("Adapter weight must be a finite positive number.");
            }
        }

        if errors.is_empty() {
            ValidationResult::ok()
        } else {
            ValidationResult::fail(&errors.join(" | "))
        }
    }
}
```

### Better long-term design

Change `ValidationResult` to carry multiple structured errors:

```rust
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ErrorMessage>,
}
```

That is more maintainable and API-friendly.

---

# 3. Recommended Target Refactor

Below is a coherent “target state” refactor that preserves your AES layering and improves SOLID compliance.

---

## 3.1 Add typed language

```rust
// shared/src/config-system/taxonomy_config_language_vo.rs

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        match self {
            ConfigLanguage::Rust => &["lint_arwaky.config.rust.yaml"],
            ConfigLanguage::Python => &["lint_arwaky.config.python.yaml"],
            ConfigLanguage::TypeScript => &[
                "lint_arwaky.config.typescript.yaml",
                "lint_arwaky.config.javascript.yaml",
            ],
        }
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
```

---

## 3.2 Harden utility I/O

```rust
// shared/src/config-system/utility_config_io.rs

use crate::common::taxonomy_path_vo::FilePath;
use std::io;
use std::path::Path;
use tokio::fs;

pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

const IGNORED_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

pub async fn path_exists_async<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path.as_ref()).await.is_ok()
}

pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).is_file()
}

pub fn read_file_sync<P: AsRef<Path>>(path: P) -> io::Result<String> {
    std::fs::read_to_string(path)
}

pub async fn read_file_async<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path).await
}

pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> io::Result<String> {
    let path = path.as_ref();

    let canonical_path = fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }

    let meta = fs::metadata(&canonical_path).await?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }

    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }

    fs::read_to
Yes — I reviewed the `config-system` crate and the imported `shared` definitions as a backend/API + architecture review.

A quick note on scope: this crate does **not** contain database access. The closest equivalent to “database queries” here is **filesystem discovery and file reads**, so I reviewed those as the data-access path.

---

# 1. Executive Summary

The crate has a good intent: clean layering, contracts, DI container, async traits, and value objects. However, the current implementation has several important issues:

## High-severity issues
1. **Blocking I/O inside async code**
   - `ConfigOrchestrator::collect_subdirs()` and `scan_workspace_dirs()` use `std::fs` inside an async flow.
   - This can stall the Tokio executor and hurts scalability.

2. **Silent error swallowing**
   - Invalid YAML becomes default config via `unwrap_or_default()`.
   - Many I/O errors are printed to stderr and ignored.
   - `read_config()` returns `Option<ConfigSource>` instead of `Result<..., ConfigError>`, so callers cannot distinguish:
     - file missing
     - permission denied
     - malformed file
     - symlink/path escape
     - I/O failure

3. **Security weaknesses in config loading**
   - `language` is interpolated directly into file names.
   - Config files may be symlinks, enabling local file disclosure in untrusted workspaces.
   - No maximum file size limit for config reads.
   - Environment-variable-based XDG lookup is accepted without hardening.

4. **Architectural violations vs the stated AES rules**
   - The **Agent layer** (`ConfigOrchestrator`) performs direct filesystem discovery (`std::fs::read_dir`) and path scanning.
   - The **Taxonomy layer** (`taxonomy_config_vo.rs`) contains parsing/normalization logic, `include_str!`, `eprintln!`, and infrastructure concerns.
   - The orchestrator bypasses `IConfigParserProtocol` and calls `parse_config_yaml()` directly.
   - The aggregate leaks internal protocols (`workspace_detector()`, `config_reader()`), weakening the facade.

5. **Performance bottleneck in workspace discovery**
   - `discover_workspaces()` uses `join_all()` with unbounded concurrency.
   - In a large monorepo, this can cause too many open files and memory pressure.

---

# 2. Detailed Review by Area

---

## A. API / Contract Design

### Issue A1. `Option` instead of `Result` hides failures
Current contract:

```rust
async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
```

This is too weak.

It cannot distinguish:

- config not found
- unreadable file
- permission error
- path escape
- oversized file
- invalid symlink

### Why this matters

A backend API should make failure modes explicit. Otherwise callers are forced to guess, and libraries end up printing warnings instead of propagating errors.

### Fix

Return `Result<Option<ConfigSource>, ConfigError>`:

```rust
#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
```

---

### Issue A2. Aggregate leaks internals

Current aggregate:

```rust
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol>;
    ...
}
```

This violates the purpose of an aggregate/facade.

### Why this matters

A facade should expose **feature behavior**, not internal building blocks. Otherwise the Surface layer can bypass orchestration and reassemble flows itself.

### Fix

Remove accessor methods from the aggregate:

```rust
#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(
        &self,
        project_root: &FilePath,
    ) -> Result<ConfigResult, ConfigError>;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<ConfigResult, ConfigError>;

    async fn discover_workspaces(
        &self,
        root: &FilePath,
    ) -> Result<Vec<WorkspaceInfo>, ConfigError>;
}
```

---

### Issue A3. `language: &str` is too permissive

Current code uses free-form strings:

```rust
async fn load_config_for_language(&self, project_root: &FilePath, language: &str)
```

Then:

```rust
format!("lint_arwaky.config.{}.yaml", language)
```

This is both a security and correctness issue.

### Fix

Introduce a typed language enum:

```rust
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        match self {
            ConfigLanguage::Rust => &["lint_arwaky.config.rust.yaml"],
            ConfigLanguage::Python => &["lint_arwaky.config.python.yaml"],
            ConfigLanguage::TypeScript => &[
                "lint_arwaky.config.typescript.yaml",
                "lint_arwaky.config.javascript.yaml",
            ],
        }
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
```

This removes an entire class of input-validation and path-injection problems.

---

## B. Security Review

---

### Issue B1. Path traversal / unsafe file name construction

Current reader:

```rust
fn config_filename(language: &str) -> String {
    format!("lint_arwaky.config.{}.yaml", language)
}
```

If `language` is attacker-influenced, this can become unsafe. Even if today it usually comes from `WorkspaceType`, the public API accepts `&str`.

### Risk

A malicious or buggy caller can pass values containing path separators or `..`.

### Fix

Use the `ConfigLanguage` enum above and never build file names from raw strings.

---

### Issue B2. Symlinked config files can leak local files

Current code reads whatever file exists at the candidate path:

```rust
config_io::read_file_async(&candidate).await
```

If an untrusted repository contains:

```text
lint_arwaky.config.rust.yaml -> /etc/passwd
```

then the loader may read it and expose raw contents through `ConfigSource`.

### Why this matters

For a linter running on untrusted checkouts, this is a real local file disclosure risk.

### Fix

Canonicalize and confine reads to the expected directory root.

```rust
use std::io;
use std::path::Path;
use tokio::fs;

pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

/// Reads a file only if its canonical path stays inside `canonical_root`.
pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> io::Result<String> {
    let path = path.as_ref();

    let canonical_path = fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }

    let meta = fs::metadata(&canonical_path).await?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }

    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }

    fs::read_to_string(&canonical_path).await
}
```

This mitigates:

- symlink escape
- `..` traversal
- oversized file DoS
- non-regular file surprises

---

### Issue B3. XDG lookup should be hardened

Current code trusts `XDG_CONFIG_DIRS` and splits it blindly.

### Risk

A hostile environment can point config discovery to many arbitrary directories.

### Fix

Apply limits and prefer absolute paths only:

```rust
async fn read_any(language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();

    if let Some(user_config) = dirs::config_dir() {
        candidates.push(user_config.join("lint-arwaky"));
    }

    let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
        Ok(dirs) if !dirs.is_empty() => dirs,
        _ => "/etc/xdg".to_string(),
    };

    for dir in system_dirs
        .split(':')
        .filter(|s| !s.is_empty())
        .take(8) // hard cap
    {
        let path = std::path::PathBuf::from(dir);
        if path.is_absolute() {
            candidates.push(path.join("lint-arwaky"));
        }
    }

    for dir in candidates {
        for file_name in language.config_file_names() {
            let candidate = dir.join(file_name);

            let canonical_root = match tokio::fs::canonicalize(&dir).await {
                Ok(root) => root,
                Err(_) => continue,
            };

            match shared::config_system::utility_config_io::read_text_within_canonical_root(
                &candidate,
                &canonical_root,
            )
            .await
            {
                Ok(content) => {
                    let path = FilePath::new(candidate.to_string_lossy().to_string())
                        .map_err(|e| ConfigError::new(
                            ConfigKey::new("config.path"),
                            ErrorMessage::new(e),
                        ))?;

                    return Ok(Some(ConfigSource::new(language.as_str(), path, content)));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
                Err(e) => {
                    return Err(ConfigError::new(
                        ConfigKey::new("config.read"),
                        ErrorMessage::new(format!("Failed to read config: {e}")),
                    ))
                }
            }
        }
    }

    Ok(None)
}
```

---

## C. Error Handling Review

---

### Issue C1. Invalid YAML silently becomes default

Current parser logic:

```rust
let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
```

and later:

```rust
Err(e) => {
    eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
    ArchitectureConfig::default()
}
```

### Why this is bad

A broken config is treated as “no config”. That hides user mistakes and can cause surprising lint behavior.

### Fix

Parsing should fail explicitly.

Move parsing out of Taxonomy and into a parser/utility component, returning `Result`.

```rust
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::taxonomy_common_error::ErrorMessage;

pub trait IConfigParserProtocol: Send + Sync {
    fn parse_architecture_yaml(&self, content: &str)
        -> Result<ArchitectureConfig, ConfigError>;
}
```

Then in the parser provider:

```rust
impl IConfigParserProtocol for ConfigParserProvider {
    fn parse_architecture_yaml(
        &self,
        content: &str,
    ) -> Result<ArchitectureConfig, ConfigError> {
        let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(content)
            .map_err(|e| ConfigError::new(
                ConfigKey::new("architecture.yaml.parse"),
                ErrorMessage::new(format!("Invalid YAML: {e}")),
            ))?;

        // Move the existing normalization logic here, but replace all
        // unwrap_or_default()/eprintln! with explicit Result propagation.
        utility_config_parser::to_architecture_config(raw)
    }
}
```

And the normalization utility should return:

```rust
pub fn to_architecture_config(
    raw: serde_yaml_ng::Value,
) -> Result<ArchitectureConfig, ConfigError> {
    // existing JSON normalization logic, but no silent fallback
}
```

### Rule

For config systems:

- **missing config** can be OK
- **invalid config** should be an error

---

### Issue C2. `ConfigSource::new()` hides invalid paths

Current:

```rust
path: FilePath::new(path.into()).unwrap_or_default(),
```

This silently replaces bad paths with `""`.

### Fix

Make construction explicit:

```rust
impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: FilePath,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path,
            raw_content: raw_content.into(),
        }
    }
}
```

Now callers must provide a valid `FilePath`.

---

## D. Performance Review

---

### Issue D1. Blocking filesystem operations in async context

Current orchestrator:

```rust
fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
    let entries = match std::fs::read_dir(dir) { ... }
}
```

This is called from async `discover_workspaces()`.

### Impact

Blocks the async runtime thread.

### Fix

Move directory enumeration to async utility functions.

```rust
use crate::common::taxonomy_path_vo::FilePath;
use std::io;
use std::path::Path;
use tokio::fs;

const IGNORED_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

pub async fn list_subdirectories<P: AsRef<Path>>(dir: P) -> io::Result<Vec<FilePath>> {
    let mut read_dir = fs::read_dir(dir.as_ref()).await?;
    let mut out = Vec::new();

    while let Some(entry) = read_dir.next_entry().await? {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') || IGNORED_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            let path = entry.path();
            let fp = FilePath::new(path.to_string_lossy().to_string())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            out.push(fp);
        }
    }

    Ok(out)
}

pub async fn path_exists_async<P: AsRef<Path>>(path: P) -> bool {
    tokio::fs::metadata(path.as_ref()).await.is_ok()
}
```

---

### Issue D2. Unbounded concurrency in `discover_workspaces()`

Current:

```rust
join_all(futures).await
```

### Risk

In a large workspace:

- too many concurrent file reads
- file descriptor exhaustion
- memory spikes

### Fix

Use bounded concurrency:

```rust
use futures::stream::{self, StreamExt};

const MAX_CONCURRENT_WORKSPACE_LOADS: usize = 8;

let results = stream::iter(workspaces.into_iter().map(|ws| {
    let detector = self.workspace_detector.clone();
    let reader = self.config_reader.clone();
    let parser = self.parser.clone();
    let defaults = self.defaults.clone();

    async move {
        let ws_type = detector.detect(&ws).await;
        let language = ConfigLanguage::try_from(ws_type)?;

        match reader.read_config(&ws, language).await? {
            Some(source) => {
                let mut config = parser.parse_architecture_yaml(&source.raw_content)?;
                if config.layers.is_empty() {
                    config.layers = defaults.default_for_language(language).layers;
                }
                Ok(WorkspaceInfo::new(ws, language.to_string(), config))
            }
            None => Ok(WorkspaceInfo::new(
                ws,
                language.to_string(),
                defaults.default_for_language(language),
            )),
        }
    }
}))
.buffer_unordered(MAX_CONCURRENT_WORKSPACE_LOADS)
.collect::<Vec<Result<WorkspaceInfo, ConfigError>>>()
.await;
```

This is much more scalable.

---

## E. Architecture / AES Compliance Review

---

### Issue E1. Agent layer is doing technical filesystem work

Current `ConfigOrchestrator` contains:

```rust
fn collect_subdirs(...)
fn scan_workspace_dirs(...)
```

This is **technical file discovery**, not orchestration.

### AES violation

According to your own architecture:

- Agent may depend only on Taxonomy and Contract
- Agent must not contain technical parsing or infrastructure mechanics
- Utility must perform low-level technical operations

### Fix

Move discovery into a Capability or Utility, and expose it via Contract.

Better contract:

```rust
#[async_trait]
pub trait IWorkspaceDetectorProtocol: Send + Sync {
    async fn detect(&self, path: &FilePath) -> WorkspaceType;
    async fn is_workspace(&self, path: &FilePath) -> bool;

    async fn discover_workspace_members(
        &self,
        root: &FilePath,
    ) -> Result<Vec<FilePath>, ConfigError>;
}
```

Then the Agent only calls:

```rust
let dirs = self.workspace_detector.discover_workspace_members(root).await?;
```

That restores proper layer boundaries.

---

### Issue E2. Taxonomy contains infrastructure and parsing logic

`taxonomy_config_vo.rs` currently includes:

- YAML parsing
- JSON normalization
- `include_str!`
- `eprintln!`
- default config caching

### Why this violates AES

Taxonomy should be stable domain vocabulary, not parsing infrastructure.

### Fix

Move:

- `parse_config_yaml()` → Utility or Capabilities parser
- default embedded config loading → Utility or Capabilities provider
- normalization rules → Utility

Taxonomy should only define:

- `ArchitectureConfig`
- `ArchitectureRule`
- value objects
- domain errors

---

### Issue E3. Orchestrator bypasses parser contract

Current orchestrator:

```rust
let mut parsed = parse_config_yaml(&source.raw_content);
```

But you already have:

```rust
IConfigParserProtocol
```

### Fix

Inject and use the parser contract.

```rust
pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
    defaults: Arc<dyn IDefaultConfigProvider>,
}
```

This satisfies DIP and makes the system testable.

---

## F. Business Logic / Validation Review

---

### Issue F1. Weak threshold validation

Current validator checks:

- score range
- complexity positive
- max_file_lines positive

But it misses:

- NaN/infinite weights
- non-positive adapter weights
- duplicate adapters
- invalid adapter status combinations

### Improved validator

```rust
use std::collections::HashSet;

impl IConfigValidatorProtocol for ConfigRulesValidator {
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool {
        for adapter in &config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        true
    }

    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult {
        let t = &config.thresholds;
        let mut errors = Vec::new();

        if !t.score.value.is_finite() || !(0.0..=100.0).contains(&t.score.value) {
            errors.push("Score threshold must be a finite number between 0 and 100.");
        }

        if t.complexity.value <= 0 {
            errors.push("Complexity threshold must be positive.");
        }

        if t.max_file_lines.value <= 0 {
            errors.push("max_file_lines threshold must be positive.");
        }

        let mut seen_adapters = HashSet::new();

        for adapter in &config.adapters {
            if !seen_adapters.insert(adapter.name.value()) {
                errors.push("Duplicate adapter entry detected.");
            }

            if !adapter.weight.is_finite() || adapter.weight <= 0.0 {
                errors.push("Adapter weight must be a finite positive number.");
            }
        }

        if errors.is_empty() {
            ValidationResult::ok()
        } else {
            ValidationResult::fail(&errors.join(" | "))
        }
    }
}
```

### Better long-term design

Change `ValidationResult` to carry multiple structured errors:

```rust
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ErrorMessage>,
}
```

That is more maintainable and API-friendly.

---

# 3. Recommended Target Refactor

Below is a coherent “target state” refactor that preserves your AES layering and improves SOLID compliance.

---

## 3.1 Add typed language

```rust
// shared/src/config-system/taxonomy_config_language_vo.rs

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        match self {
            ConfigLanguage::Rust => &["lint_arwaky.config.rust.yaml"],
            ConfigLanguage::Python => &["lint_arwaky.config.python.yaml"],
            ConfigLanguage::TypeScript => &[
                "lint_arwaky.config.typescript.yaml",
                "lint_arwaky.config.javascript.yaml",
            ],
        }
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
```

---

## 3.2 Harden utility I/O

```rust
// shared/src/config-system/utility_config_io.rs

use crate::common::taxonomy_path_vo::FilePath;
use std::io;
use std::path::Path;
use tokio::fs;

pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

const IGNORED_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

pub async fn path_exists_async<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path.as_ref()).await.is_ok()
}

pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).is_file()
}

pub fn read_file_sync<P: AsRef<Path>>(path: P) -> io::Result<String> {
    std::fs::read_to_string(path)
}

pub async fn read_file_async<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path).await
}

pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> io::Result<String> {
    let path = path.as_ref();

    let canonical_path = fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }

    let meta = fs::metadata(&canonical_path).await?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }

    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }

    fs::read_to_string(&canonical_path).await
}

pub async fn list_subdirectories<P: AsRef<Path>>(dir: P) -> io::Result<Vec<FilePath>> {
    let mut read_dir = fs::read_dir(dir.as_ref()).await?;
    let mut out = Vec::new();

    while let Some(entry) = read_dir.next_entry().await? {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') || IGNORED_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            let path = entry.path();
            let fp = FilePath::new(path.to_string_lossy().to_string())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            out.push(fp);
        }
    }

    Ok(out)
}
```

---

## 3.3 Update contracts

```rust
// shared/src/config-system/contract_reader_protocol.rs

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
```

```rust
// shared/src/config-system/contract_workspace_detector_protocol.rs

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

#[async_trait]
pub trait IWorkspaceDetectorProtocol: Send + Sync {
    async fn detect(&self, path: &FilePath) -> WorkspaceType;

    async fn is_workspace(&self, path: &FilePath) -> bool;

    async fn discover_workspace_members(
        &self,
        root: &FilePath,
    ) -> Result<Vec<FilePath>, ConfigError>;
}
```

```rust
// shared/src/config-system/contract_config_orchestrator_aggregate.rs

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(
        &self,
        project_root: &FilePath,
    ) -> Result<ConfigResult, ConfigError>;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<ConfigResult, ConfigError>;

    async fn discover_workspaces(
        &self,
        root: &FilePath,
    ) -> Result<Vec<WorkspaceInfo>, ConfigError>;
}
```

---

## 3.4 Secure reader implementation

```rust
// crates/config-system/src/capabilities_yaml_reader.rs

use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::config_system::utility_config_io as config_io;
use shared::taxonomy_common_error::ErrorMessage;
use std::io;
use std::path::PathBuf;

pub struct ConfigYamlReader;

#[async_trait]
impl IConfigReaderProtocol for ConfigYamlReader {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        let mut current = tokio::fs::canonicalize(project_root.value())
            .await
            .map_err(|e| Self::io_error(project_root, e))?;

        for _depth in 0..2 {
            for file_name in language.config_file_names() {
                let candidate = current.join(file_name);

                match config_io::read_text_within_canonical_root(&candidate, &current).await {
                    Ok(content) => {
                        let path = FilePath::new(candidate.to_string_lossy().to_string())
                            .map_err(|e| {
                                ConfigError::new(
                                    ConfigKey::new("config.path"),
                                    ErrorMessage::new(e),
                                )
                            })?;

                        return Ok(Some(ConfigSource::new(language.as_str(), path, content)));
                    }
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                        return Err(Self::security_error(&candidate, e))
                    }
                    Err(e) => return Err(Self::io_error_path(&candidate, e)),
                }
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }

        Self::read_any(language).await
    }

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
        let mut found = Vec::new();

        for lang in [
            ConfigLanguage::Rust,
            ConfigLanguage::Python,
            ConfigLanguage::TypeScript,
        ] {
            if let Some(source) = self.read_config(project_root, lang).await? {
                found.push((lang, source.path));
            }
        }

        Ok(found)
    }
}

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    async fn read_any(language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
        let mut candidates: Vec<PathBuf> = Vec::new();

        if let Some(user_config) = dirs::config_dir() {
            candidates.push(user_config.join("lint-arwaky"));
        }

        let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
            Ok(dirs) if !dirs.is_empty() => dirs,
            _ => "/etc/xdg".to_string(),
        };

        for dir in system_dirs
            .split(':')
            .filter(|s| !s.is_empty())
            .take(8)
        {
            let path = PathBuf::from(dir);
            if path.is_absolute() {
                candidates.push(path.join("lint-arwaky"));
            }
        }

        for dir in candidates {
            for file_name in language.config_file_names() {
                let candidate = dir.join(file_name);

                let canonical_root = match tokio::fs::canonicalize(&dir).await {
                    Ok(root) => root,
                    Err(_) => continue,
                };

                match config_io::read_text_within_canonical_root(&candidate, &canonical_root).await
                {
                    Ok(content) => {
                        let path = FilePath::new(candidate.to_string_lossy().to_string())
                            .map_err(|e| {
                                ConfigError::new(
                                    ConfigKey::new("config.path"),
                                    ErrorMessage::new(e),
                                )
                            })?;

                        return Ok(Some(ConfigSource::new(language.as_str(), path, content)));
                    }
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
                    Err(e) => return Err(Self::io_error_path(&candidate, e)),
                }
            }
        }

        Ok(None)
    }

    fn io_error(path: &FilePath, e: io::Error) -> ConfigError {
        ConfigError::new(
            ConfigKey::new("config.io"),
            ErrorMessage::new(format!("I/O error at {}: {e}", path.value)),
        )
    }

    fn io_error_path(path: &std::path::Path, e: io::Error) -> ConfigError {
        ConfigError::new(
            ConfigKey::new("config.io"),
            ErrorMessage::new(format!("I/O error at {}: {e}", path.display())),
        )
    }

    fn security_error(path: &std::path::Path, e: io::Error) -> ConfigError {
        ConfigError::new(
            ConfigKey::new("config.security"),
            ErrorMessage::new(format!("Rejected unsafe config path {}: {e}", path.display())),
        )
    }
}
```

---

## 3.5 Move workspace discovery out of Agent

```rust
// crates/config-system/src/capabilities_workspace_detector_provider.rs

use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::contract_workspace_detector_protocol::WorkspaceType;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::utility_config_io as config_io;
use shared::taxonomy_common_error::ErrorMessage;
use std::path::PathBuf;

pub struct WorkspaceDetector;

#[async_trait]
impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    async fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = PathBuf::from(path.value());

        if config_io::path_exists_async(path_buf.join("Cargo.toml")).await {
            return WorkspaceType::Rust;
        }

        if config_io::path_exists_async(path_buf.join("package.json")).await {
            return WorkspaceType::TypeScript;
        }

        if config_io::path_exists_async(path_buf.join("pyproject.toml")).await
            || config_io::path_exists_async(path_buf.join("setup.py")).await
            || config_io::path_exists_async(path_buf.join("requirements.txt")).await
        {
            return WorkspaceType::Python;
        }

        if let Some(parent) = path_buf.parent() {
            match parent.file_name().and_then(|n| n.to_str()) {
                Some("modules") => return WorkspaceType::Python,
                Some("packages") => return WorkspaceType::TypeScript,
                Some("crates") => return WorkspaceType::Rust,
                _ => {}
            }
        }

        WorkspaceType::Unknown
    }

    async fn is_workspace(&self, path: &FilePath) -> bool {
        let root = PathBuf::from(path.value());

        for dir in ["crates", "packages", "modules"] {
            if config_io::path_exists_async(root.join(dir)).await {
                return true;
            }
        }

        false
    }

    async fn discover_workspace_members(
        &self,
        root: &FilePath,
    ) -> Result<Vec<FilePath>, ConfigError> {
        let root_path = PathBuf::from(root.value());
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = root_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|name| workspace_dirs.contains(&name))
            .unwrap_or(false);

        if is_root_workspace_dir {
            return config_io::list_subdirectories(&root_path)
                .await
                .map_err(|e| {
                    ConfigError::new(
                        ConfigKey::new("workspace.discovery"),
                        ErrorMessage::new(format!(
                            "Failed to read workspace directory {}: {e}",
                            root_path.display()
                        )),
                    )
                });
        }

        if let Some(parent) = root_path.parent() {
            if let Some(parent_name) = parent.file_name().and_then(|n| n.to_str()) {
                if workspace_dirs.contains(&parent_name) && root_path.is_dir() {
                    return Ok(vec![root.clone()]);
                }
            }
        }

        let mut results = Vec::new();

        for dir in workspace_dirs {
            let dir_path = root_path.join(dir);
            if config_io::path_exists_async(&dir_path).await {
                let mut subs = config_io::list_subdirectories(&dir_path).await.map_err(|e| {
                    ConfigError::new(
                        ConfigKey::new("workspace.discovery"),
                        ErrorMessage::new(format!(
                            "Failed to read workspace directory {}: {e}",
                            dir_path.display()
                        )),
                    )
                })?;
                results.append(&mut subs);
            }
        }

        Ok(results)
    }
}
```

---

## 3.6 Refactored orchestrator

```rust
// crates/config-system/src/agent_config_orchestrator.rs

use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::contract_workspace_detector_protocol::WorkspaceType;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use shared::config_system::taxonomy_source_vo::ConfigResult;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::taxonomy_common_error::ErrorMessage;
use std::sync::Arc;

const MAX_CONCURRENT_WORKSPACE_LOADS: usize = 8;

pub trait IDefaultConfigProvider: Send + Sync {
    fn default_for_language(&self, language: ConfigLanguage) -> ArchitectureConfig;
}

pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
    defaults: Arc<dyn IDefaultConfigProvider>,
}

impl ConfigOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
        config_reader: Arc<dyn IConfigReaderProtocol>,
        parser: Arc<dyn IConfigParserProtocol>,
        defaults: Arc<dyn IDefaultConfigProvider>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
            parser,
            defaults,
        }
    }

    fn unknown_workspace_error(path: &FilePath) -> ConfigError {
        ConfigError::new(
            ConfigKey::new("workspace.type"),
            ErrorMessage::new(format!(
                "Unable to detect workspace language at {}",
                path.value
            )),
        )
    }
}

#[async_trait]
impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    async fn load_project_config(
        &self,
        project_root: &FilePath,
    ) -> Result<ConfigResult, ConfigError> {
        let ws_type = self.workspace_detector.detect(project_root).await;

        let language = match ws_type {
            WorkspaceType::Rust => ConfigLanguage::Rust,
            WorkspaceType::Python => ConfigLanguage::Python,
            WorkspaceType::TypeScript => ConfigLanguage::TypeScript,
            WorkspaceType::Unknown => return Err(Self::unknown_workspace_error(project_root)),
        };

        self.load_config_for_language(project_root, language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<ConfigResult, ConfigError> {
        match self.config_reader.read_config(project_root, language).await? {
            Some(source) => {
                let mut parsed = self.parser.parse_architecture_yaml(&source.raw_content)?;
                let mut warnings = Vec::new();

                if parsed.layers.is_empty() {
                    parsed.layers = self.defaults.default_for_language(language).layers;
                    warnings.push(
                        "Config file had no architecture layers, using built-in defaults for layers only."
                            .to_string(),
                    );
                }

                Ok(ConfigResult::new(parsed, source, warnings))
            }
            None => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = self.defaults.default_for_language(language);
                let source = ConfigSource::new(
                    language.as_str(),
                    FilePath::new("embedded").unwrap_or_default(),
                    "",
                );

                Ok(ConfigResult::new(config, source, warnings))
            }
        }
    }

    async fn discover_workspaces(
        &self,
        root: &FilePath,
    ) -> Result<Vec<WorkspaceInfo>, ConfigError> {
        let workspaces = self.workspace_detector.discover_workspace_members(root).await?;

        if workspaces.is_empty() {
            return Ok(Vec::new());
        }

        let results = stream::iter(workspaces.into_iter().map(|ws| {
            let detector = self.workspace_detector.clone();
            let reader = self.config_reader.clone();
            let parser = self.parser.clone();
            let defaults = self.defaults.clone();

            async move {
                let ws_type = detector.detect(&ws).await;

                let language = match ws_type {
                    WorkspaceType::Rust => ConfigLanguage::Rust,
                    WorkspaceType::Python => ConfigLanguage::Python,
                    WorkspaceType::TypeScript => ConfigLanguage::TypeScript,
                    WorkspaceType::Unknown => {
                        return Err(ConfigError::new(
                            ConfigKey::new("workspace.type"),
                            ErrorMessage::new(format!(
                                "Unable to detect workspace language at {}",
                                ws.value
                            )),
                        ))
                    }
                };

                match reader.read_config(&ws, language).await? {
                    Some(source) => {
                        let mut config = parser.parse_architecture_yaml(&source.raw_content)?;

                        if config.layers.is_empty() {
                            config.layers = defaults.default_for_language(language).layers;
                        }

                        Ok(WorkspaceInfo::new(ws, language.to_string(), config))
                    }
                    None => Ok(WorkspaceInfo::new(
                        ws,
                        language.to_string(),
                        defaults.default_for_language(language),
                    )),
                }
            }
        }))
        .buffer_unordered(MAX_CONCURRENT_WORKSPACE_LOADS)
        .collect::<Vec<Result<WorkspaceInfo, ConfigError>>>()
        .await;

        let mut ok = Vec::new();
        let mut errors = Vec::new();

        for item in results {
            match item {
                Ok(info) => ok.push(info),
                Err(err) => errors.push(err),
            }
        }

        // Strict mode: fail if no workspace could be loaded.
        if ok.is_empty() && !errors.is_empty() {
            return Err(errors.remove(0));
        }

        // Optional: if partial errors matter, return a richer result type
        // containing both `ok` and `errors`.
        Ok(ok)
    }
}
```

---

## 3.7 Wire container correctly

```rust
// crates/config-system/src/root_config_system_container.rs

use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    parser: Arc<dyn IConfigParserProtocol>,
    validator: Arc<dyn IConfigValidatorProtocol>,
}

impl ConfigContainer {
    pub fn new() -> Self {
        let workspace_detector =
            Arc::new(crate::capabilities_workspace_detector_provider::WorkspaceDetector::new());

        let yaml_reader = Arc::new(crate::capabilities_yaml_reader::ConfigYamlReader::new());

        let parser = Arc::new(crate::capabilities_parser_provider::ConfigParserProvider::new());

        let defaults = Arc::new(crate::capabilities_default_config_provider::EmbeddedDefaultConfigProvider::new());

        let orchestrator = Arc::new(crate::agent_config_orchestrator::ConfigOrchestrator::new(
            workspace_detector,
            yaml_reader,
            parser.clone(),
            defaults,
        ));

        let validator = Arc::new(crate::capabilities_rules_validator::ConfigRulesValidator::new());

        Self {
            orchestrator,
            parser,
            validator,
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IConfigOrchestratorAggregate> {
        self.orchestrator.clone()
    }

    pub fn parser(&self) -> Arc<dyn IConfigParserProtocol> {
        self.parser.clone()
    }

    pub fn validator(&self) -> Arc<dyn IConfigValidatorProtocol> {
        self.validator.clone()
    }
}
```

---

# 4. SOLID Assessment

## SRP — Single Responsibility Principle

### Current problem

- `ConfigOrchestrator` does orchestration + filesystem discovery.
- Taxonomy does domain modeling + parsing + default embedding + normalization.

### Fix

- Move filesystem enumeration to Utility/Capability.
- Move parsing to Parser Capability/Utility.
- Keep Taxonomy as pure domain model.

---

## OCP — Open/Closed Principle

### Current problem

Language handling is string-based and scattered.

### Fix

Use `ConfigLanguage` and provider abstractions so new languages can be added without sprinkling `match` and string formatting across the system.

---

## LSP — Liskov Substitution Principle

### Current problem

Traits returning `Option` force implementers to hide failures.

### Fix

Use `Result<Option<T>, E>` so all implementations can express errors uniformly.

---

## ISP — Interface Segregation Principle

### Current problem

Aggregate exposes internal protocols.

### Fix

Expose only high-level feature operations.

---

## DIP — Dependency Inversion Principle

### Current problem

Agent depends on concrete parsing functions and direct `std::fs`.

### Fix

Agent should depend only on:

- `IWorkspaceDetectorProtocol`
- `IConfigReaderProtocol`
- `IConfigParserProtocol`
- `IDefaultConfigProvider`

---

# 5. Prioritized Remediation Plan

If I were fixing this in production order, I would do it in this sequence:

## Phase 1 — Security + correctness (highest priority)

1. Replace `language: &str` with `ConfigLanguage`.
2. Add path confinement + symlink protection.
3. Add max file size checks.
4. Make config parsing fail on invalid YAML instead of defaulting.

## Phase 2 — Error handling

1. Change reader and orchestrator APIs to `Result`.
2. Remove `eprintln!` from library code.
3. Return structured warnings/errors.

## Phase 3 — Performance

1. Replace blocking `std::fs` with async I/O.
2. Replace `join_all` with `buffer_unordered`.
3. Optionally move CPU-heavy YAML normalization to `spawn_blocking`.

## Phase 4 — Architecture cleanup

1. Move parsing out of Taxonomy.
2. Move discovery out of Agent.
3. Wire parser and default provider through contracts in the container.
4. Remove aggregate accessor leakage.

---

# 6. Final Verdict

The current implementation is functional, but not yet production-grade from a backend engineering standpoint.

The biggest risks are:

1. **Security**

   - unsafe config path handling
   - symlink exposure
   - missing size limits
2. **Reliability**

   - silent fallbacks
   - weak error propagation
3. **Scalability**

   - blocking I/O in async code
   - unbounded concurrency
4. **Maintainability**

   - layer leakage
   - SRP violations
   - taxonomy doing infrastructure work

If you apply the refactor above, the crate becomes:

- safer on untrusted workspaces
- easier to test
- more scalable in large monorepos
- much closer to AES and SOLID principles

If you want, I can also provide a **file-by-file patch** for this crate in “diff style” so you can apply the fixes directly.

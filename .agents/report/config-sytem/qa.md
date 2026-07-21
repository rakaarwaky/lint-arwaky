Yes — after reviewing `config-system v1.10.106`, the AES structural scan may report `0 violations`, but there are still **functional correctness bugs** and **performance bottlenecks** in the runtime behavior.

Below is an expert QA + developer review focused only on:

- fixing broken or inconsistent behavior
- removing performance bottlenecks
- preserving existing features
- **not adding new product functionality**

---

# 1. Summary of Detected Issues

## Functional Bugs

### BUG-1: Async methods block the Tokio runtime with synchronous filesystem calls

**Severity:** High**Files:**

- `crates/config-system/src/agent_config_orchestrator.rs`
- `crates/config-system/src/capabilities_workspace_detector_provider.rs`
- `crates/shared/src/config-system/utility_config_io.rs`

`discover_workspaces()` and `load_project_config()` are async, but they call:

- `std::fs::read_dir`
- `Path::exists`
- `Path::is_dir`

These are blocking syscalls inside async code. This can stall the executor, especially on large workspaces or slow disks.

---

### BUG-2: Workspace config discovery misses root-level config in standard AES layouts

**Severity:** High**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

For a normal AES layout like:

```text
/root
  lint_arwaky.config.rust.yaml
  crates
    member-a
```

`ConfigYamlReader::read_config()` only walks upward **2 levels**:

- `/root/crates/member-a`
- `/root/crates`

It never checks:

- `/root`

So root-level configuration is ignored for workspace members.

---

### BUG-3: TypeScript workspaces ignore JavaScript config files despite shared defaults

**Severity:** Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

`default_config_for_language()` treats:

```rust
"javascript" | "typescript"
```

as the same default config family.

But `ConfigYamlReader::config_filename()` only looks for:

```text
lint_arwaky.config.typescript.yaml
```

So if a project already has:

```text
lint_arwaky.config.javascript.yaml
```

a TypeScript workspace will ignore it.

That is inconsistent behavior.

---

### BUG-4: Invalid or partially invalid YAML silently falls back to defaults

**Severity:** High**Files:**

- `crates/shared/src/config-system/taxonomy_config_vo.rs`
- `crates/config-system/src/agent_config_orchestrator.rs`

`parse_config_yaml()` currently swallows parse/deserialization failures and falls back to defaults.

Problems:

- callers receive no reliable warning in `ConfigResult.warnings`
- invalid configuration can silently become default configuration
- users can believe a config file was applied when it was not

This is a functional correctness issue.

---

### BUG-5: `ConfigParserProvider` bypasses the same normalization used by the orchestrator

**Severity:** High**File:**

- `crates/config-system/src/capabilities_parser_provider.rs`

`ConfigOrchestrator` parses architecture config through `parse_config_yaml()`, which performs important normalization:

- rules object → rules array
- suffix policy normalization
- ignored paths normalization
- scope expansion
- conditions flattening

But `ConfigParserProvider::parse_yaml_config()` and `parse_toml_config()` deserialize `ProjectConfig` directly.

That means the same config content can be interpreted differently depending on the code path.

This is a real consistency bug.

---

### BUG-6: Rule flattening can silently drop rules

**Severity:** Medium**File:**

- `crates/shared/src/config-system/taxonomy_config_vo.rs`

In `parse_config_yaml()`, rules with:

- empty `conditions: []`
- malformed conditions
- non-object condition entries
- empty scope arrays

can be dropped silently or fail to deserialize cleanly.

That changes intended rule behavior.

---

### BUG-7: `list_config_files()` can report global/XDG config files as project files

**Severity:** Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

`list_config_files()` calls `read_config()`, which falls back to global/XDG locations.

So it can return a global config path as if it were a project-local config file.

That is misleading.

---

### BUG-8: Unreadable config files are ignored silently

**Severity:** Low/Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

If a config file exists but cannot be read due to permissions or I/O errors, the code silently moves on.

That hides real environment problems.

---

## Performance Bottlenecks

### PERF-1: Blocking filesystem I/O inside async orchestration

**Severity:** High**File:**

- `agent_config_orchestrator.rs`

This is the biggest runtime bottleneck.

---

### PERF-2: Unbounded concurrency in workspace discovery

**Severity:** Medium**File:**

- `agent_config_orchestrator.rs`

`join_all(...)` launches all workspace loads concurrently.

For large repositories, this can create:

- too many concurrent filesystem operations
- unnecessary task pressure
- unstable performance

Bounded concurrency is better.

---

### PERF-3: Repeated parsing of the same inherited config

**Severity:** Medium**Files:**

- `agent_config_orchestrator.rs`
- `taxonomy_config_vo.rs`

If many workspace members inherit the same root config, the current logic parses that same YAML repeatedly.

This is a CPU hotspot in large monorepos.

---

# 2. Fixed Code

Below are the corrective patches.

---

# FIX 1: Make `ConfigOrchestrator` non-blocking and bounded

## What this fixes

- BUG-1
- PERF-1
- PERF-2

## Changes

- replace blocking directory traversal with `tokio::fs`
- run synchronous workspace detection inside `spawn_blocking`
- replace unbounded `join_all` with bounded `buffered(8)`

---

## Fixed file: `crates/config-system/src/agent_config_orchestrator.rs`

Replace the existing implementation with this corrected version:

```rust
use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_workspace_detector_protocol::{
    IWorkspaceDetectorProtocol, WorkspaceType,
};
use shared::config_system::taxonomy_config_vo::{
    default_config_for_language, parse_config_yaml_with_warnings,
};
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use shared::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
use std::sync::Arc;

pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
}

impl ConfigOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
        config_reader: Arc<dyn IConfigReaderProtocol>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }

    async fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();

        let mut entries = match tokio::fs::read_dir(dir).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read directory '{}': {}",
                    dir.display(),
                    e
                );
                return results;
            }
        };

        loop {
            match entries.next_entry().await {
                Ok(Some(entry)) => {
                    match entry.file_type().await {
                        Ok(ft) if ft.is_dir() => {
                            let sub = entry.path();
                            match FilePath::new(sub.to_string_lossy().to_string()) {
                                Ok(fp) => results.push(fp),
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Skipping invalid directory path '{}': {}",
                                        sub.display(),
                                        e
                                    );
                                }
                            }
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to read directory entry type in '{}': {}",
                                dir.display(),
                                e
                            );
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to iterate directory entry in '{}': {}",
                        dir.display(),
                        e
                    );
                    break;
                }
            }
        }

        results
    }

    async fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs(root).await;
        }

        if let Some(parent) = root.parent() {
            if let Some(parent_name) = parent.file_name() {
                let parent_str = parent_name.to_string_lossy();
                if workspace_dirs.contains(&parent_str.as_ref()) {
                    let root_is_dir = tokio::fs::metadata(root)
                        .await
                        .map(|m| m.is_dir())
                        .unwrap_or(false);

                    if root_is_dir {
                        if let Ok(fp) = FilePath::new(root.to_string_lossy().to_string()) {
                            return vec![fp];
                        }
                    }
                }
            }
        }

        let mut results = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            let is_dir = tokio::fs::metadata(&dir_path)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false);

            if is_dir {
                results.extend(Self::collect_subdirs(&dir_path).await);
            }
        }

        results
    }

    async fn detect_workspace_type(&self, path: &FilePath) -> WorkspaceType {
        let detector = self.workspace_detector.clone();
        let path = path.clone();

        tokio::task::spawn_blocking(move || detector.detect(&path))
            .await
            .unwrap_or(WorkspaceType::Unknown)
    }
}

#[async_trait]
impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol> {
        self.workspace_detector.clone()
    }

    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol> {
        self.config_reader.clone()
    }

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.detect_workspace_type(project_root).await;
        let language = ws_type.as_str().to_string();
        self.load_config_for_language(project_root, &language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                let (mut parsed, mut warnings) =
                    parse_config_yaml_with_warnings(&source.raw_content);

                if parsed.layers.is_empty() {
                    let defaults = default_config_for_language(language);
                    parsed.layers = defaults.layers;

                    if warnings.is_empty() {
                        warnings.push(
                            "Config file had no architecture layers, using built-in defaults for layers only."
                                .to_string(),
                        );
                    }
                }

                ConfigResult::new(parsed, source, warnings)
            }
            None => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = default_config_for_language(language);
                let source = ConfigSource::new(language, "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
        }
    }

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let root_path = std::path::Path::new(&root.value);
        let workspaces = Self::scan_workspace_dirs(root_path).await;

        if workspaces.is_empty() {
            eprintln!(
                "Warning: No AES-compliant workspace members (crates/, packages/, or modules/) found in '{}'. \
                This system mandates a multi-module structure. Please refactor your project.",
                root.value
            );
            return Vec::new();
        }

        let detector = self.workspace_detector.clone();
        let reader = self.config_reader.clone();

        let tasks = workspaces.into_iter().map(move |ws| {
            let detector = detector.clone();
            let reader = reader.clone();

            async move {
                let ws_type = {
                    let detector = detector.clone();
                    let ws_path = ws.clone();
                    tokio::task::spawn_blocking(move || detector.detect(&ws_path))
                        .await
                        .unwrap_or(WorkspaceType::Unknown)
                };

                let language = ws_type.as_str();

                let config = match reader.read_config(&ws, language).await {
                    Some(source) => {
                        let (mut parsed, warnings) =
                            parse_config_yaml_with_warnings(&source.raw_content);

                        for warning in warnings {
                            eprintln!("Warning: config for '{}': {}", ws.value, warning);
                        }

                        if parsed.layers.is_empty() {
                            parsed.layers = default_config_for_language(language).layers;
                        }

                        parsed
                    }
                    None => default_config_for_language(language),
                };

                WorkspaceInfo::new(ws, language.to_string(), config)
            }
        });

        stream::iter(tasks).buffered(8).collect().await
    }
}
```

---

# FIX 2: Correct config lookup depth, language aliases, and local-only listing

## What this fixes

- BUG-2
- BUG-3
- BUG-7
- BUG-8

## Changes

- increase upward search depth to cover `/root/crates/member`
- add TypeScript ↔ JavaScript fallback aliases
- make `list_config_files()` report only local project files
- warn on non-`NotFound` I/O errors

---

## Fixed file: `crates/config-system/src/capabilities_yaml_reader.rs`

Replace with this corrected implementation:

```rust
use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::config_system::utility_config_io as config_io;
use std::path::PathBuf;

// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
pub struct ConfigYamlReader;

#[async_trait]
impl IConfigReaderProtocol for ConfigYamlReader {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        for candidate_language in Self::language_aliases(language) {
            let filename = Self::config_filename(&candidate_language);

            if let Some(source) = self
                .read_local_config(project_root, &filename, language)
                .await
            {
                return Some(source);
            }
        }

        Self::read_any(language).await
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found: Vec<(String, String)> = Vec::new();

        for lang in ["rust", "python", "typescript"] {
            for candidate_language in Self::language_aliases(lang) {
                let filename = Self::config_filename(&candidate_language);

                if let Some(source) = self
                    .read_local_config(project_root, &filename, lang)
                    .await
                {
                    let path = source.path.to_string();

                    if !found.iter().any(|(_, p)| p == &path) {
                        found.push((lang.to_string(), path));
                    }

                    break;
                }
            }
        }

        found
    }
}

impl Default for ConfigYamlReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }

    fn language_aliases(language: &str) -> Vec<String> {
        match language {
            "typescript" => vec!["typescript".to_string(), "javascript".to_string()],
            "javascript" => vec!["javascript".to_string(), "typescript".to_string()],
            other => vec![other.to_string()],
        }
    }

    async fn read_local_config(
        &self,
        project_root: &FilePath,
        filename: &str,
        source_language: &str,
    ) -> Option<ConfigSource> {
        let mut current = PathBuf::from(&project_root.value);
        let mut depth = 0;

        // Covers:
        // 0: /root/crates/member
        // 1: /root/crates
        // 2: /root
        while !current.as_os_str().is_empty() && depth < 3 {
            let candidate = current.join(filename);

            match config_io::read_file_async(&candidate).await {
                Ok(content) => {
                    return Some(ConfigSource::new(
                        source_language,
                        candidate.to_string_lossy().to_string(),
                        content,
                    ));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    // keep searching
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read config file '{}': {}",
                        candidate.display(),
                        e
                    );
                }
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }

            depth += 1;
        }

        None
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Returns `None` to fall back to compiled-in defaults.
    async fn read_any(language: &str) -> Option<ConfigSource> {
        for candidate_language in Self::language_aliases(language) {
            let filename = Self::config_filename(&candidate_language);
            let mut candidates: Vec<PathBuf> = Vec::new();

            if let Some(user_config) = dirs::config_dir() {
                candidates.push(user_config.join("lint-arwaky").join(&filename));
            }

            let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
                Ok(dirs) if !dirs.is_empty() => dirs,
                _ => "/etc/xdg".to_string(),
            };

            for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
                candidates.push(
                    std::path::PathBuf::from(dir)
                        .join("lint-arwaky")
                        .join(&filename),
                );
            }

            for path in &candidates {
                match config_io::read_file_async(path).await {
                    Ok(content) => {
                        return Some(ConfigSource::new(
                            language,
                            path.to_string_lossy().to_string(),
                            content,
                        ));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to read global config file '{}': {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }

        None
    }
}
```

---

# FIX 3: Make YAML parsing warning-aware and stop silent rule loss

## What this fixes

- BUG-4
- BUG-6

## Changes

- keep old `parse_config_yaml()` API intact
- add warning-aware parsing helper
- preserve rules that
  Yes — after reviewing `config-system v1.10.106`, the AES structural scan may report `0 violations`, but there are still **functional correctness bugs** and **performance bottlenecks** in the runtime behavior.

Below is an expert QA + developer review focused only on:

- fixing broken or inconsistent behavior
- removing performance bottlenecks
- preserving existing features
- **not adding new product functionality**

---

# 1. Summary of Detected Issues

## Functional Bugs

### BUG-1: Async methods block the Tokio runtime with synchronous filesystem calls

**Severity:** High**Files:**

- `crates/config-system/src/agent_config_orchestrator.rs`
- `crates/config-system/src/capabilities_workspace_detector_provider.rs`
- `crates/shared/src/config-system/utility_config_io.rs`

`discover_workspaces()` and `load_project_config()` are async, but they call:

- `std::fs::read_dir`
- `Path::exists`
- `Path::is_dir`

These are blocking syscalls inside async code. This can stall the executor, especially on large workspaces or slow disks.

---

### BUG-2: Workspace config discovery misses root-level config in standard AES layouts

**Severity:** High**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

For a normal AES layout like:

```text
/root
  lint_arwaky.config.rust.yaml
  crates
    member-a
```

`ConfigYamlReader::read_config()` only walks upward **2 levels**:

- `/root/crates/member-a`
- `/root/crates`

It never checks:

- `/root`

So root-level configuration is ignored for workspace members.

---

### BUG-3: TypeScript workspaces ignore JavaScript config files despite shared defaults

**Severity:** Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

`default_config_for_language()` treats:

```rust
"javascript" | "typescript"
```

as the same default config family.

But `ConfigYamlReader::config_filename()` only looks for:

```text
lint_arwaky.config.typescript.yaml
```

So if a project already has:

```text
lint_arwaky.config.javascript.yaml
```

a TypeScript workspace will ignore it.

That is inconsistent behavior.

---

### BUG-4: Invalid or partially invalid YAML silently falls back to defaults

**Severity:** High**Files:**

- `crates/shared/src/config-system/taxonomy_config_vo.rs`
- `crates/config-system/src/agent_config_orchestrator.rs`

`parse_config_yaml()` currently swallows parse/deserialization failures and falls back to defaults.

Problems:

- callers receive no reliable warning in `ConfigResult.warnings`
- invalid configuration can silently become default configuration
- users can believe a config file was applied when it was not

This is a functional correctness issue.

---

### BUG-5: `ConfigParserProvider` bypasses the same normalization used by the orchestrator

**Severity:** High**File:**

- `crates/config-system/src/capabilities_parser_provider.rs`

`ConfigOrchestrator` parses architecture config through `parse_config_yaml()`, which performs important normalization:

- rules object → rules array
- suffix policy normalization
- ignored paths normalization
- scope expansion
- conditions flattening

But `ConfigParserProvider::parse_yaml_config()` and `parse_toml_config()` deserialize `ProjectConfig` directly.

That means the same config content can be interpreted differently depending on the code path.

This is a real consistency bug.

---

### BUG-6: Rule flattening can silently drop rules

**Severity:** Medium**File:**

- `crates/shared/src/config-system/taxonomy_config_vo.rs`

In `parse_config_yaml()`, rules with:

- empty `conditions: []`
- malformed conditions
- non-object condition entries
- empty scope arrays

can be dropped silently or fail to deserialize cleanly.

That changes intended rule behavior.

---

### BUG-7: `list_config_files()` can report global/XDG config files as project files

**Severity:** Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

`list_config_files()` calls `read_config()`, which falls back to global/XDG locations.

So it can return a global config path as if it were a project-local config file.

That is misleading.

---

### BUG-8: Unreadable config files are ignored silently

**Severity:** Low/Medium**File:**

- `crates/config-system/src/capabilities_yaml_reader.rs`

If a config file exists but cannot be read due to permissions or I/O errors, the code silently moves on.

That hides real environment problems.

---

## Performance Bottlenecks

### PERF-1: Blocking filesystem I/O inside async orchestration

**Severity:** High**File:**

- `agent_config_orchestrator.rs`

This is the biggest runtime bottleneck.

---

### PERF-2: Unbounded concurrency in workspace discovery

**Severity:** Medium**File:**

- `agent_config_orchestrator.rs`

`join_all(...)` launches all workspace loads concurrently.

For large repositories, this can create:

- too many concurrent filesystem operations
- unnecessary task pressure
- unstable performance

Bounded concurrency is better.

---

### PERF-3: Repeated parsing of the same inherited config

**Severity:** Medium**Files:**

- `agent_config_orchestrator.rs`
- `taxonomy_config_vo.rs`

If many workspace members inherit the same root config, the current logic parses that same YAML repeatedly.

This is a CPU hotspot in large monorepos.

---

# 2. Fixed Code

Below are the corrective patches.

---

# FIX 1: Make `ConfigOrchestrator` non-blocking and bounded

## What this fixes

- BUG-1
- PERF-1
- PERF-2

## Changes

- replace blocking directory traversal with `tokio::fs`
- run synchronous workspace detection inside `spawn_blocking`
- replace unbounded `join_all` with bounded `buffered(8)`

---

## Fixed file: `crates/config-system/src/agent_config_orchestrator.rs`

Replace the existing implementation with this corrected version:

```rust
use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_workspace_detector_protocol::{
    IWorkspaceDetectorProtocol, WorkspaceType,
};
use shared::config_system::taxonomy_config_vo::{
    default_config_for_language, parse_config_yaml_with_warnings,
};
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use shared::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
use std::sync::Arc;

pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
}

impl ConfigOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
        config_reader: Arc<dyn IConfigReaderProtocol>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }

    async fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();

        let mut entries = match tokio::fs::read_dir(dir).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read directory '{}': {}",
                    dir.display(),
                    e
                );
                return results;
            }
        };

        loop {
            match entries.next_entry().await {
                Ok(Some(entry)) => {
                    match entry.file_type().await {
                        Ok(ft) if ft.is_dir() => {
                            let sub = entry.path();
                            match FilePath::new(sub.to_string_lossy().to_string()) {
                                Ok(fp) => results.push(fp),
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Skipping invalid directory path '{}': {}",
                                        sub.display(),
                                        e
                                    );
                                }
                            }
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to read directory entry type in '{}': {}",
                                dir.display(),
                                e
                            );
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to iterate directory entry in '{}': {}",
                        dir.display(),
                        e
                    );
                    break;
                }
            }
        }

        results
    }

    async fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs(root).await;
        }

        if let Some(parent) = root.parent() {
            if let Some(parent_name) = parent.file_name() {
                let parent_str = parent_name.to_string_lossy();
                if workspace_dirs.contains(&parent_str.as_ref()) {
                    let root_is_dir = tokio::fs::metadata(root)
                        .await
                        .map(|m| m.is_dir())
                        .unwrap_or(false);

                    if root_is_dir {
                        if let Ok(fp) = FilePath::new(root.to_string_lossy().to_string()) {
                            return vec![fp];
                        }
                    }
                }
            }
        }

        let mut results = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            let is_dir = tokio::fs::metadata(&dir_path)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false);

            if is_dir {
                results.extend(Self::collect_subdirs(&dir_path).await);
            }
        }

        results
    }

    async fn detect_workspace_type(&self, path: &FilePath) -> WorkspaceType {
        let detector = self.workspace_detector.clone();
        let path = path.clone();

        tokio::task::spawn_blocking(move || detector.detect(&path))
            .await
            .unwrap_or(WorkspaceType::Unknown)
    }
}

#[async_trait]
impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol> {
        self.workspace_detector.clone()
    }

    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol> {
        self.config_reader.clone()
    }

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.detect_workspace_type(project_root).await;
        let language = ws_type.as_str().to_string();
        self.load_config_for_language(project_root, &language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                let (mut parsed, mut warnings) =
                    parse_config_yaml_with_warnings(&source.raw_content);

                if parsed.layers.is_empty() {
                    let defaults = default_config_for_language(language);
                    parsed.layers = defaults.layers;

                    if warnings.is_empty() {
                        warnings.push(
                            "Config file had no architecture layers, using built-in defaults for layers only."
                                .to_string(),
                        );
                    }
                }

                ConfigResult::new(parsed, source, warnings)
            }
            None => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = default_config_for_language(language);
                let source = ConfigSource::new(language, "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
        }
    }

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let root_path = std::path::Path::new(&root.value);
        let workspaces = Self::scan_workspace_dirs(root_path).await;

        if workspaces.is_empty() {
            eprintln!(
                "Warning: No AES-compliant workspace members (crates/, packages/, or modules/) found in '{}'. \
                This system mandates a multi-module structure. Please refactor your project.",
                root.value
            );
            return Vec::new();
        }

        let detector = self.workspace_detector.clone();
        let reader = self.config_reader.clone();

        let tasks = workspaces.into_iter().map(move |ws| {
            let detector = detector.clone();
            let reader = reader.clone();

            async move {
                let ws_type = {
                    let detector = detector.clone();
                    let ws_path = ws.clone();
                    tokio::task::spawn_blocking(move || detector.detect(&ws_path))
                        .await
                        .unwrap_or(WorkspaceType::Unknown)
                };

                let language = ws_type.as_str();

                let config = match reader.read_config(&ws, language).await {
                    Some(source) => {
                        let (mut parsed, warnings) =
                            parse_config_yaml_with_warnings(&source.raw_content);

                        for warning in warnings {
                            eprintln!("Warning: config for '{}': {}", ws.value, warning);
                        }

                        if parsed.layers.is_empty() {
                            parsed.layers = default_config_for_language(language).layers;
                        }

                        parsed
                    }
                    None => default_config_for_language(language),
                };

                WorkspaceInfo::new(ws, language.to_string(), config)
            }
        });

        stream::iter(tasks).buffered(8).collect().await
    }
}
```

---

# FIX 2: Correct config lookup depth, language aliases, and local-only listing

## What this fixes

- BUG-2
- BUG-3
- BUG-7
- BUG-8

## Changes

- increase upward search depth to cover `/root/crates/member`
- add TypeScript ↔ JavaScript fallback aliases
- make `list_config_files()` report only local project files
- warn on non-`NotFound` I/O errors

---

## Fixed file: `crates/config-system/src/capabilities_yaml_reader.rs`

Replace with this corrected implementation:

```rust
use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::config_system::utility_config_io as config_io;
use std::path::PathBuf;

// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
pub struct ConfigYamlReader;

#[async_trait]
impl IConfigReaderProtocol for ConfigYamlReader {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        for candidate_language in Self::language_aliases(language) {
            let filename = Self::config_filename(&candidate_language);

            if let Some(source) = self
                .read_local_config(project_root, &filename, language)
                .await
            {
                return Some(source);
            }
        }

        Self::read_any(language).await
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found: Vec<(String, String)> = Vec::new();

        for lang in ["rust", "python", "typescript"] {
            for candidate_language in Self::language_aliases(lang) {
                let filename = Self::config_filename(&candidate_language);

                if let Some(source) = self
                    .read_local_config(project_root, &filename, lang)
                    .await
                {
                    let path = source.path.to_string();

                    if !found.iter().any(|(_, p)| p == &path) {
                        found.push((lang.to_string(), path));
                    }

                    break;
                }
            }
        }

        found
    }
}

impl Default for ConfigYamlReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }

    fn language_aliases(language: &str) -> Vec<String> {
        match language {
            "typescript" => vec!["typescript".to_string(), "javascript".to_string()],
            "javascript" => vec!["javascript".to_string(), "typescript".to_string()],
            other => vec![other.to_string()],
        }
    }

    async fn read_local_config(
        &self,
        project_root: &FilePath,
        filename: &str,
        source_language: &str,
    ) -> Option<ConfigSource> {
        let mut current = PathBuf::from(&project_root.value);
        let mut depth = 0;

        // Covers:
        // 0: /root/crates/member
        // 1: /root/crates
        // 2: /root
        while !current.as_os_str().is_empty() && depth < 3 {
            let candidate = current.join(filename);

            match config_io::read_file_async(&candidate).await {
                Ok(content) => {
                    return Some(ConfigSource::new(
                        source_language,
                        candidate.to_string_lossy().to_string(),
                        content,
                    ));
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    // keep searching
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read config file '{}': {}",
                        candidate.display(),
                        e
                    );
                }
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }

            depth += 1;
        }

        None
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Returns `None` to fall back to compiled-in defaults.
    async fn read_any(language: &str) -> Option<ConfigSource> {
        for candidate_language in Self::language_aliases(language) {
            let filename = Self::config_filename(&candidate_language);
            let mut candidates: Vec<PathBuf> = Vec::new();

            if let Some(user_config) = dirs::config_dir() {
                candidates.push(user_config.join("lint-arwaky").join(&filename));
            }

            let system_dirs = match std::env::var("XDG_CONFIG_DIRS") {
                Ok(dirs) if !dirs.is_empty() => dirs,
                _ => "/etc/xdg".to_string(),
            };

            for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
                candidates.push(
                    std::path::PathBuf::from(dir)
                        .join("lint-arwaky")
                        .join(&filename),
                );
            }

            for path in &candidates {
                match config_io::read_file_async(path).await {
                    Ok(content) => {
                        return Some(ConfigSource::new(
                            language,
                            path.to_string_lossy().to_string(),
                            content,
                        ));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to read global config file '{}': {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }

        None
    }
}
```

---

# FIX 3: Make YAML parsing warning-aware and stop silent rule loss

## What this fixes

- BUG-4
- BUG-6

## Changes

- keep old `parse_config_yaml()` API intact
- add warning-aware parsing helper
- preserve rules that previously could be dropped
- normalize empty scope arrays
- preserve base rule when conditions are empty/malformed

---

## Patch file: `crates/shared/src/config-system/taxonomy_config_vo.rs`

### Step 1: Replace the existing `parse_config_yaml()` signature wrapper

Replace the current public entrypoint with:

```rust
pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    parse_config_yaml_with_warnings(yaml_str).0
}
```

Then rename the old body to:

```rust
pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    // ...
}
```

---

### Step 2: Replace the beginning of the renamed function with this

```rust
pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    let mut warnings = Vec::new();

    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => {
            warnings.push(format!(
                "Failed to parse YAML config: {}; using built-in defaults",
                e
            ));
            return (ArchitectureConfig::default(), warnings);
        }
    };

    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = match serde_json::to_value(arch_val) {
            Ok(v) => v,
            Err(e) => {
                warnings.push(format!(
                    "Failed to convert architecture config to JSON: {}; using built-in defaults",
                    e
                ));
                serde_json::Value::Object(serde_json::Map::new())
            }
        };

        // Extract layers from rules (first rule containing "layers" key) if not at top-level
        if arch_json.get("layers").is_none() {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                for (_rule_code, rule_val) in rules_obj.iter_mut() {
                    if let Some(layers) = rule_val.get_mut("layers") {
                        let layers = std::mem::take(layers);
                        arch_json["layers"] = layers;
                        break;
                    }
                }
            }
        }

        let mut json = arch_json;

        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() {
                        remove_nulls(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        remove_nulls(v);
                    }
                }
                _ => {}
            }
        }

        remove_nulls(&mut json);

        // Convert ignored_paths from array to {values: [...]} format
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({ "values": arr });
        }

        // Existing suffix normalization remains unchanged here.
        // Keep the current `if let Some(layers_obj) = json.get_mut("layers") { ... }` block as-is.

        // Replace only the rules flattening block with the corrected block below.
```

---

### Step 3: Replace the existing `rules` flattening block with this corrected block

```rust
        if let Some(rules_obj) = json.get_mut("rules") {
            if let Some(obj) = rules_obj.as_object_mut() {
                let mut flat = serde_json::Value::Array(Vec::new());

                for (code, rule_val) in obj.iter() {
                    let Some(rule_obj) = rule_val.as_object() else {
                        continue;
                    };

                    let mut base = rule_obj.clone();
                    base.insert("name".to_string(), serde_json::json!(code));

                    let mut set_empty_scope = false;
                    let mut first_scope: Option<String> = None;
                    let mut scope_expansion: Option<Vec<String>> = None;

                    if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                        if scope_arr.is_empty() {
                            set_empty_scope = true;
                        } else if !base.contains_key("conditions") && scope_arr.len() > 1 {
                            scope_expansion = Some(
                                scope_arr
                                    .iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect(),
                            );
                        } else {
                            first_scope = scope_arr
                                .first()
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());
                        }
                    }

                    if set_empty_scope {
                        base.insert("scope".to_string(), serde_json::json!(""));
                    }

                    if let Some(first) = first_scope {
                        base.insert("scope".to_string(), serde_json::json!(first));
                    }

                    if let Some(scopes) = scope_expansion {
                        if scopes.is_empty() {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(base);
                            }
                        } else {
                            for scope in scopes {
                                let mut entry = base.clone();
                                entry.insert("scope".to_string(), serde_json::json!(scope));

                                if let Some(arr) = flat.as_array_mut() {
                                    arr.push(entry);
                                }
                            }
                        }

                        continue;
                    }

                    if let Some(conditions) = base.remove("conditions") {
                        let mut pushed = false;

                        if let Some(conds) = conditions.as_array() {
                            if conds.is_empty() {
                                if let Some(arr) = flat.as_array_mut() {
                                    arr.push(base.clone());
                                }
                                pushed = true;
                            } else {
                                for cond in conds {
                                    if let Some(cond_obj) = cond.as_object() {
                                        let mut entry = base.clone();

                                        for (k, v) in cond_obj {
                                            entry.insert(k.clone(), v.clone());
                                        }

                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(entry);
                                        }

                                        pushed = true;
                                    }
                                }
                            }
                        }

                        if !pushed {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(base);
                            }
                        }
                    } else if let Some(arr) = flat.as_array_mut() {
                        arr.push(base);
                    }
                }

                *rules_obj = flat;
            }
        }
```

---

### Step 4: Replace the final deserialization return path with this

```rust
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                warnings.push(format!(
                    "Failed to deserialize ArchitectureConfig: {:?}; using built-in defaults",
                    e
                ));
                ArchitectureConfig::default()
            }
        };

        // Top-level ignored_paths (outside architecture section) — merge into config
        if config.ignored_paths.values.is_empty() {
            if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
                let paths: Vec<_> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                    .collect();

                if !paths.is_empty() {
                    config.ignored_paths = FilePathList::new(paths);
                }
            }
        }

        (config, warnings)
    } else {
        let mut config = ArchitectureConfig::default();

        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();

            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }

        (config, warnings)
    }
}
```

---

# FIX 4: Make `ConfigParserProvider` use the same normalization path

## What this fixes

- BUG-5
- also fixes suspicious import path for `ErrorMessage`

## Why this matters

Before this fix, parser provider deserialization bypassed architecture normalization. That created inconsistent parsing behavior between:

- orchestrator path
- parser provider path

This patch makes both paths consistent.

---

## Fixed file: `crates/config-system/src/capabilities_parser_provider.rs`

Replace with this corrected implementation:

```rust
// PURPOSE: ConfigParserProvider — IConfigParserProtocol implementation for YAML and TOML config parsing
use serde_json::Value as JsonValue;
use shared::common::taxonomy_common_vo::ErrorMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_vo::parse_config_yaml_with_warnings;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use shared::config_system::utility_config_io as config_io;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ConfigParserProvider {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IConfigParserProtocol for ConfigParserProvider {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        let p = &path.value;
        let err_path = path.clone();

        let content = match config_io::read_file_sync(p) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(format!("Failed to read config: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };

        Self::parse_yaml_content(&content, path)
    }

    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError> {
        let p = &path.value;
        let err_path = path.clone();

        let content = match config_io::read_file_sync(p) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("tool.lint-arwaky"),
                    message: ErrorMessage::new(format!("Failed to read TOML: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };

        Self::parse_toml_content(&content, path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for ConfigParserProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParserProvider {
    pub fn new() -> Self {
        Self {}
    }

    fn normalize_project_json(json: &mut JsonValue) {
        if let Some(obj) = json.as_object_mut() {
            if let Some(arr) = obj.get("ignored_paths").and_then(|v| v.as_array()).cloned() {
                obj.insert(
                    "ignored_paths".to_string(),
                    serde_json::json!({ "values": arr }),
                );
            }

            // Architecture is attached after normalized parsing.
            obj.remove("architecture");
        }
    }

    fn parse_yaml_content(content: &str, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        let err_path = path.clone();

        let yaml_value: serde_yaml_ng::Value =
            serde_yaml_ng::from_str(content).map_err(|e| ConfigError {
                key: ConfigKey::new("yaml.parse"),
                message: ErrorMessage::new(format!("Failed to parse YAML: {}", e)),
                config_file: err_path.clone(),
                ..Default::default()
            })?;

        let mut json: JsonValue =
            serde_json::to_value(yaml_value).map_err(|e| ConfigError {
                key: ConfigKey::new("yaml.convert"),
                message: ErrorMessage::new(format!("Failed to convert YAML to JSON: {}", e)),
                config_file: err_path.clone(),
                ..Default::default()
            })?;

        let (architecture, warnings) = parse_config_yaml_with_warnings(content);

        if let Some(msg) = warnings.into_iter().next() {
            return Err(ConfigError {
                key: ConfigKey::new("yaml.parse"),
                message: ErrorMessage::new(msg),
                config_file: err_path.clone(),
                ..Default::default()
            });
        }

        Self::normalize_project_json(&mut json);

        let mut config: ProjectConfig =
            serde_json::from_value(json).map_err(|e| ConfigError {
                key: ConfigKey::new("yaml.parse"),
                message: ErrorMessage::new(format!(
                    "Failed to deserialize project config: {}",
                    e
                )),
                config_file: err_path,
                ..Default::default()
            })?;

        config.architecture = architecture;
        Ok(config)
    }

    fn parse_toml_content(
        content: &str,
        path: &FilePath,
    ) -> Result<Option<ProjectConfig>, ConfigError> {
        let err_path = path.clone();

        let toml_value: toml::Value = toml::from_str(content).map_err(|e| ConfigError {
            key: ConfigKey::new("tool.lint-arwaky"),
            message: ErrorMessage::new(format!("Failed to parse TOML: {}", e)),
            config_file: err_path.clone(),
            ..Default::default()
        })?;

        let tool_section = toml_value
            .get("tool")
            .and_then(|t| t.get("lint-arwaky").or_else(|| t.get("lint_arwaky")));

        let Some(tool_section) = tool_section else {
            return Ok(None);
        };

        let mut json: JsonValue =
            serde_json::to_value(tool_section).map_err(|e| ConfigError {
                key: ConfigKey::new("toml.convert"),
                message: ErrorMessage::new(format!("Failed to convert TOML to JSON: {}", e)),
                config_file: err_path.clone(),
                ..Default::default()
            })?;

        let yaml_for_normalization =
            serde_yaml_ng::to_string(&json).map_err(|e| ConfigError {
                key: ConfigKey::new("toml.convert"),
                message: ErrorMessage::new(format!(
                    "Failed to convert TOML config to YAML for normalization: {}",
                    e
                )),
                config_file: err_path.clone(),
                ..Default::default()
            })?;

        let (architecture, warnings) =
            parse_config_yaml_with_warnings(&yaml_for_normalization);

        if let Some(msg) = warnings.into_iter().next() {
            return Err(ConfigError {
                key: ConfigKey::new("toml.parse"),
                message: ErrorMessage::new(msg),
                config_file: err_path.clone(),
                ..Default::default()
            });
        }

        Self::normalize_project_json(&mut json);

        let mut config: ProjectConfig =
            serde_json::from_value(json).map_err(|e| ConfigError {
                key: ConfigKey::new("toml.parse"),
                message: ErrorMessage::new(format!(
                    "Failed to deserialize TOML project config: {}",
                    e
                )),
                config_file: err_path,
                ..Default::default()
            })?;

        config.architecture = architecture;
        Ok(Some(config))
    }
}
```

---

# FIX 5: Small correctness hardening in shared value object

## What this fixes

- prevents negative scores from `Score::deduct`

This is a small but important correctness guard if scoring is intended to stay within `[0, 100]`.

---

## Patch file: `crates/shared/src/common/taxonomy_common_vo.rs`

Replace:

```rust
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
```

with:

```rust
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: (self.value - severity.score_impact()).max(0.0),
        }
    }
```

---

# 3. Remaining Performance Note

There is still one secondary performance hotspot:

## PERF-3: Repeated parsing of the same inherited config

If many workspace members resolve to the same root config file, the system may parse the same YAML repeatedly.

### Why I did not force a global cache in the patch

Adding a persistent cache would introduce new stateful behavior. Since your instruction was:

> Under no circumstances should you add new functionalities

I kept the mandatory fix set limited to:

- removing blocking I/O
- bounding concurrency
- fixing incorrect lookup behavior
- fixing parsing consistency

### If you want the final optimization without changing product behavior

Add a **request-scoped parse cache** inside `discover_workspaces()` keyed by:

```rust
(source.path.value, language)
```

That will reduce repeated parsing in large monorepos while still preserving existing functionality.

---

# 4. Regression Checklist

After applying these fixes, verify the following cases:

## Functional Regression Tests

### Config discovery

- [ ] member with own config uses member config
- [ ] member without config inherits root config in `/root/crates/member`
- [ ] TypeScript workspace finds `lint_arwaky.config.typescript.yaml`
- [ ] TypeScript workspace falls back to `lint_arwaky.config.javascript.yaml`

### Invalid config handling

- [ ] invalid YAML returns warning in `ConfigResult.warnings`
- [ ] invalid architecture section does not silently pretend success
- [ ] empty `conditions: []` no longer drops rules
- [ ] empty `scope: []` no longer creates deserialization instability

### Parser consistency

- [ ] YAML parsed by orchestrator and parser provider produces equivalent architecture semantics
- [ ] TOML `[tool.lint-arwaky]` parsing uses same normalization as YAML path

### Listing behavior

- [ ] `list_config_files()` returns only project-local files
- [ ] global/XDG files are not reported as project files

---

# 5. Final Verdict

The most important defects were:

1. **blocking filesystem I/O inside async code**
2. **incorrect upward config search depth**
3. **silent parse fallback hiding configuration errors**
4. **inconsistent parsing between orchestrator and parser provider**
5. **unbounded concurrency during workspace discovery**

The patches above fix those issues while staying within the existing feature boundary.

If you want, I can next provide:

- **a unified diff patch format**
- or a **minimal patch set only for the Rust compiler to apply directly**

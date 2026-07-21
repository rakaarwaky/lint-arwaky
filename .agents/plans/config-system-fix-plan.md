# Config-System Fix Plan

> Generated from BA + QA + Backend report validation against actual codebase.
> 17 validated issues across 7 phases.

## Decisions

| Decision | Choice |
|----------|--------|
| AES Layer Violations | Fix violations (move parsing out of taxonomy, filesystem out of agent) |
| Security Hardening | Full hardening (symlink, path confinement, XDG, max file size) |
| Error Handling | Change to `Result<Option<ConfigSource>, ConfigError>` |
| ConfigLanguage | Introduce typed enum |
| Aggregate Facade | Remove accessor methods |
| Config Lookup Depth | Increase from 2 to 3 |

## Severity Legend

- **CRITICAL**: Security vulnerability or data-correctness bug
- **HIGH**: False positive/negative or architectural violation
- **MEDIUM**: Improvement, missing check, or performance fix
- **LOW**: Documentation or minor optimization

---

## Phase 1: Taxonomy Cleanup — Move Parsing Out (HIGH)

### P1.1 — Move `parse_config_yaml()` from Taxonomy to Utility

**Skill**: `create-utility-rust` — parsing logic is stateless, domain-agnostic, reusable. Must be free functions only, no struct.
**File**: `crates/shared/src/config-system/taxonomy_config_vo.rs` → `crates/shared/src/config-system/utility_config_parser.rs` (NEW)
**Severity**: HIGH
**AES Code**: AES violation (Taxonomy contains infrastructure)
**Problem**: `taxonomy_config_vo.rs` contains YAML parsing, JSON normalization, `include_str!`, `eprintln!`, and default config caching. Taxonomy should only define value objects.

**Action**:
1. Create `utility_config_parser.rs` in shared/config-system
2. Move `parse_config_yaml()` and `parse_config_yaml_with_warnings()` to the new utility
3. Move `default_config_for_language()` and `default_aes_config()` to a new `utility_config_defaults.rs`
4. Keep `ArchitectureConfig`, `ArchitectureRule`, and VOs in taxonomy
5. Update all imports

**Before** (`taxonomy_config_vo.rs`):
```rust
pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig { ... }
pub fn default_config_for_language(language: &str) -> ArchitectureConfig { ... }
pub fn default_aes_config() -> ArchitectureConfig { ... }
```

**After**:
```rust
// taxonomy_config_vo.rs — only VOs remain
pub struct ArchitectureConfig { ... }
pub struct ArchitectureRule { ... }

// utility_config_parser.rs — parsing logic
pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig { ... }
pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) { ... }

// utility_config_defaults.rs — default configs
pub fn default_config_for_language(language: &str) -> ArchitectureConfig { ... }
pub fn default_aes_config() -> ArchitectureConfig { ... }
```

**Module registration:** Add `pub mod utility_config_parser;` and `pub mod utility_config_defaults;` to `crates/shared/src/config-system/mod.rs`.

---

### P1.2 — Move filesystem discovery out of Agent layer

**Skill**: `create-capabilities-rust` — filesystem operations belong in capabilities (external adaptation). Must implement `IWorkspaceDetectorProtocol` contract.
**File**: `crates/config-system/src/agent_config_orchestrator.rs` → `crates/config-system/src/capabilities_workspace_detector_provider.rs` (NEW)
**Severity**: HIGH
**AES Code**: AES violation (Agent does technical work)
**Problem**: `collect_subdirs()` and `scan_workspace_dirs()` are in the Agent layer but perform technical filesystem operations.

**Action**:
1. Add `discover_workspace_members()` to `IWorkspaceDetectorProtocol` contract
2. Move `collect_subdirs()` and `scan_workspace_dirs()` into `WorkspaceDetector` (capabilities layer)
3. Agent only calls `self.workspace_detector.discover_workspace_members(root)`

**Contract change** (`contract_workspace_detector_protocol.rs`):
```rust
#[async_trait]
pub trait IWorkspaceDetectorProtocol: Send + Sync {
    fn detect(&self, path: &FilePath) -> WorkspaceType;
    fn is_workspace(&self, path: &FilePath) -> bool;
    async fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}
```

**Agent simplification** (`agent_config_orchestrator.rs`):
```rust
async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
    let workspaces = self.workspace_detector.discover_workspace_members(root).await;
    // ... rest of logic without filesystem operations
}
```

**Module registration:** Add `pub mod capabilities_workspace_detector_provider;` to `crates/config-system/src/mod.rs`.

---

### P1.3 — Orchestrator must use parser contract

**Skill**: `create-agent-rust` — agent must depend only on Taxonomy and Contract, not call utility functions directly.
**File**: `crates/config-system/src/agent_config_orchestrator.rs`
**Severity**: HIGH
**AES Code**: AES violation (Agent bypasses Contract)
**Problem**: Line 127: `parse_config_yaml(&source.raw_content)` — direct call to utility function, bypassing `IConfigParserProtocol`.

**Before** (line 127):
```rust
let mut parsed = parse_config_yaml(&source.raw_content);
```

**After**: Inject parser and use contract:
```rust
pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
}

// In load_config_for_language:
let mut parsed = self.parser.parse_architecture_yaml(&source.raw_content)?;
```

---

## Phase 2: Security Hardening (CRITICAL)

### P2.1 — Add path confinement for config reads

**Skill**: `create-utility-rust` — stateless I/O function, domain-agnostic, reusable.
**File**: `crates/shared/src/config-system/utility_config_io.rs`
**Severity**: CRITICAL
**AES Code**: Security
**Problem**: Config files may be symlinks pointing outside the project root. No canonicalization or confinement.

**Add to `utility_config_io.rs`**:
```rust
pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> io::Result<String> {
    let path = path.as_ref();
    let canonical_path = tokio::fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }
    let meta = tokio::fs::metadata(&canonical_path).await?;
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
    tokio::fs::read_to_string(&canonical_path).await
}
```

---

### P2.2 — Introduce `ConfigLanguage` enum

**Skill**: `create-taxonomy-rust` — pure value object, no behavior, no dependencies.
**File**: `crates/shared/src/config-system/taxonomy_config_language_vo.rs` (NEW)
**Severity**: CRITICAL
**AES Code**: Security (path injection via `language: &str`)
**Problem**: `language` is interpolated directly into file names. Free-form strings allow path injection.

**New file**:
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

**Module registration:** Add `pub mod taxonomy_config_language_vo;` to `crates/shared/src/config-system/mod.rs`.

---

### P2.3 — Harden XDG lookup

**Skill**: `create-capabilities-rust` — adding bounds to environment parsing, allowed.
**File**: `crates/config-system/src/capabilities_yaml_reader.rs`
**Severity**: HIGH
**AES Code**: Security
**Problem**: `XDG_CONFIG_DIRS` is split blindly with no limits. Hostile environment can point to many directories.

**Before** (line 86):
```rust
for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
```

**After**:
```rust
for dir in system_dirs.split(':').filter(|s| !s.is_empty()).take(8) {
    let path = std::path::PathBuf::from(dir);
    if !path.is_absolute() { continue; }
    candidates.push(path.join("lint-arwaky"));
}
```

---

## Phase 3: Error Handling (HIGH)

### P3.1 — Change `read_config` to return `Result`

**Skill**: `create-contract-rust` — changing protocol signature to return `Result`, must remain object-safe.
**File**: `crates/shared/src/config-system/contract_reader_protocol.rs`
**Severity**: HIGH
**AES Code**: Error handling
**Problem**: `Option<ConfigSource>` hides failures (permission denied, malformed file, etc).

**Before**:
```rust
async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)>;
```

**After**:
```rust
async fn read_config(
    &self,
    project_root: &FilePath,
    language: ConfigLanguage,
) -> Result<Option<ConfigSource>, ConfigError>;

async fn list_config_files(
    &self,
    project_root: &FilePath,
) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
```

Update `ConfigYamlReader` implementation to return `Result` with proper error propagation.

---

### P3.2 — Make YAML parsing warning-aware

**Skill**: `create-utility-rust` — adding warning-aware parsing, stateless standalone functions.
**File**: `crates/shared/src/config-system/utility_config_parser.rs` (after P1.1)
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `parse_config_yaml()` swallows parse failures via `unwrap_or_default()`. Invalid config silently becomes default.

**Before** (`taxonomy_config_vo.rs:87`):
```rust
let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
```

**After**:
```rust
pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    let mut warnings = Vec::new();

    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => {
            warnings.push(format!("Failed to parse YAML: {}; using defaults", e));
            return (ArchitectureConfig::default(), warnings);
        }
    };
    // ... rest of normalization with warnings
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    parse_config_yaml_with_warnings(yaml_str).0
}
```

---

### P3.3 — Fix rule flattening dropping empty-condition rules

**Skill**: `create-utility-rust` — fixing utility function logic, stateless standalone.
**File**: `crates/shared/src/config-system/utility_config_parser.rs` (after P1.1)
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: Rules with empty `conditions: []` are dropped silently (lines 205-226).

**Before** (line 205-226):
```rust
if let Some(conditions) = base.remove("conditions") {
    if let Some(conds) = conditions.as_array() {
        if !conds.is_empty() {
            for cond in conds { ... }
        }
    }
    // Empty conditions → rule is DROPPED (no push to flat)
} else {
    flat.push(base);
}
```

**After**:
```rust
if let Some(conditions) = base.remove("conditions") {
    let mut pushed = false;
    if let Some(conds) = conditions.as_array() {
        if conds.is_empty() {
            flat.push(base.clone());
            pushed = true;
        } else {
            for cond in conds {
                if let Some(cond_obj) = cond.as_object() {
                    let mut entry = base.clone();
                    for (k, v) in cond_obj { entry.insert(k.clone(), v.clone()); }
                    flat.push(entry);
                    pushed = true;
                }
            }
        }
    }
    if !pushed { flat.push(base); }
} else {
    flat.push(base);
}
```

---

## Phase 4: Bug Fixes (HIGH)

### P4.1 — Fix config lookup depth (2 → 3)

**Skill**: `create-capabilities-rust` — fixing protocol implementation logic, allowed.
**File**: `crates/config-system/src/capabilities_yaml_reader.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `depth < 2` misses root-level config in standard AES layouts.

**Before** (line 23):
```rust
while !current.as_os_str().is_empty() && depth < 2 {
```

**After**:
```rust
while !current.as_os_str().is_empty() && depth < 3 {
```

---

### P4.2 — Add TypeScript ↔ JavaScript fallback aliases

**Skill**: `create-capabilities-rust` — adding helper method to capabilities, allowed.
**File**: `crates/config-system/src/capabilities_yaml_reader.rs`
**Severity**: MEDIUM
**AES Code**: Correctness
**Problem**: TypeScript workspace only looks for `lint_arwaky.config.typescript.yaml`, ignoring `lint_arwaky.config.javascript.yaml`.

**Add helper**:
```rust
fn language_aliases(language: &str) -> Vec<String> {
    match language {
        "typescript" => vec!["typescript".to_string(), "javascript".to_string()],
        "javascript" => vec!["javascript".to_string(), "typescript".to_string()],
        other => vec![other.to_string()],
    }
}
```

Update `read_config()` to try aliases:
```rust
async fn read_config(&self, project_root: &FilePath, language: ConfigLanguage) -> Result<Option<ConfigSource>, ConfigError> {
    for candidate_language in language.config_file_names() {
        let filename = candidate_language;
        if let Some(source) = self.read_local_config(project_root, filename, language).await? {
            return Ok(Some(source));
        }
    }
    Self::read_any(language).await
}
```

---

### P4.3 — Make `list_config_files` report only local project files

**Skill**: `create-capabilities-rust` — fixing protocol implementation, allowed.
**File**: `crates/config-system/src/capabilities_yaml_reader.rs`
**Severity**: MEDIUM
**AES Code**: Correctness
**Problem**: `list_config_files()` calls `read_config()` which falls back to XDG/global locations.

**Before** (line 44-52):
```rust
async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for lang in &["rust", "python", "typescript"] {
        if let Some(config) = self.read_config(project_root, lang).await {
            found.push((lang.to_string(), config.path.to_string()));
        }
    }
    found
}
```

**After**: Use `read_local_config` only (no XDG fallback):
```rust
async fn list_config_files(&self, project_root: &FilePath) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
    let mut found = Vec::new();
    for lang in &[ConfigLanguage::Rust, ConfigLanguage::Python, ConfigLanguage::TypeScript] {
        for filename in lang.config_file_names() {
            if let Some(source) = self.read_local_config(project_root, filename, *lang).await? {
                let path = FilePath::new(source.path.to_string())
                    .map_err(|e| ConfigError::new(...))?;
                if !found.iter().any(|(_, p)| *p == path) {
                    found.push((*lang, path));
                }
                break;
            }
        }
    }
    Ok(found)
}
```

---

### P4.4 — Warn on non-NotFound I/O errors

**Skill**: `create-capabilities-rust` — adding error handling to capabilities, allowed.
**File**: `crates/config-system/src/capabilities_yaml_reader.rs`
**Severity**: MEDIUM
**AES Code**: Error handling
**Problem**: All I/O errors are silently ignored, hiding real environment problems.

**Before** (line 25):
```rust
if let Ok(content) = config_io::read_file_async(&candidate).await {
```

**After**:
```rust
match config_io::read_file_async(&candidate).await {
    Ok(content) => {
        return Ok(Some(ConfigSource::new(...)));
    }
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        // keep searching
    }
    Err(e) => {
        eprintln!("Warning: Failed to read config '{}': {}", candidate.display(), e);
    }
}
```

---

## Phase 5: Aggregate Facade Cleanup (MEDIUM)

### P5.1 — Remove accessor methods from aggregate

**Skill**: `create-contract-rust` — removing methods from aggregate trait, must remain object-safe.
**File**: `crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs`
**Severity**: MEDIUM
**AES Code**: Architecture (aggregate facade)
**Problem**: `workspace_detector()` and `config_reader()` expose internal protocols.

**Before**:
```rust
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol>;
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(&self, project_root: &FilePath, language: &str) -> ConfigResult;
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
```

**After**:
```rust
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(&self, project_root: &FilePath, language: ConfigLanguage) -> ConfigResult;
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
```

---

## Phase 6: Performance Fixes (MEDIUM)

### P6.1 — Replace blocking I/O with async

**Skill**: `create-capabilities-rust` — async I/O in capabilities is allowed (external adaptation).
**File**: `crates/config-system/src/capabilities_workspace_detector_provider.rs` (after P1.2)
**Severity**: MEDIUM
**AES Code**: Performance
**Problem**: `std::fs::read_dir` and `Path::exists` block the async runtime.

**Action**: Use `tokio::fs` for all filesystem operations in `discover_workspace_members()`.

---

### P6.2 — Bound concurrency in workspace discovery

**Skill**: `create-agent-rust` — concurrency bounding is orchestration flow control, allowed. Agent controls sequence and movement, not business calculation.
**File**: `crates/config-system/src/agent_config_orchestrator.rs`
**Severity**: MEDIUM
**AES Code**: Performance
**Problem**: `join_all(futures)` launches all workspace loads concurrently with no limit.

**Before** (line 182):
```rust
join_all(futures).await
```

**After**:
```rust
use futures::stream::{self, StreamExt};

stream::iter(futures).buffered(8).collect().await
```

---

### P6.3 — Cache repeated config parsing

**Skill**: `create-agent-rust` — caching is state ownership within execution scope. However, agent skill says "zero business logic." Caching parsed configs is arguably business state. Consider moving cache to a capabilities layer (`CapabilitiesConfigCache`) if this violates agent purity. For now, acceptable as orchestration-level memoization.
**File**: `crates/config-system/src/agent_config_orchestrator.rs`
**Severity**: MEDIUM
**AES Code**: Performance
**Problem**: Multiple workspace members inheriting the same root config parse the same YAML repeatedly.

**Action**: Add a `HashMap<String, ArchitectureConfig>` cache keyed by config file path. Before parsing, check cache.

---

## Phase 7: Documentation (LOW)

### P7.1 — Rewrite FRD with structured requirements

**Skill**: `add-docs-rust` — rewriting FRD with structured requirements.
**File**: FRD document
**Severity**: LOW
**Problem**: FRD is a component list, not a requirements document.

**Action**: Rewrite with:
- User stories (As a [role], I need [capability] so that [value])
- Given/When/Then acceptance criteria
- Config resolution algorithm as numbered priority chain
- Error/warning taxonomy
- NFR targets

---

### P7.2 — Document config resolution algorithm

**Skill**: `add-docs-rust` — documenting config resolution algorithm in FRD.
**File**: FRD document
**Severity**: LOW
**Problem**: Config resolution is entirely implicit.

**Action**: Document priority chain:
1. Project-root YAML (`lint_arwaky.config.{lang}.yaml`)
2. Parent dir (depth ≤ 3)
3. XDG user config (`~/.config/lint-arwaky/`)
4. XDG system dirs (`/etc/xdg/lint-arwaky/`)
5. Embedded defaults

---

### P7.3 — Document merge strategy

**Skill**: `add-docs-rust` — documenting merge semantics in FRD.
**File**: FRD document
**Severity**: LOW
**Problem**: `utility_config_merger.rs` merge semantics undocumented.

**Action**: Document field-level merge rules, conflict resolution, array concatenation vs replacement.

---

## Execution Order

1. **Phase 1** (P1.1-P1.3): Move parsing out of taxonomy, filesystem out of agent, use parser contract.
   - **Verify:** `cargo check -p shared && cargo check -p config-system`
2. **Phase 2** (P2.1-P2.3): Security hardening — path confinement, ConfigLanguage enum, XDG hardening.
   - **Verify:** `cargo check -p shared && cargo check -p config-system`
3. **Phase 3** (P3.1-P3.3): Error handling — Result returns, warning-aware parsing, rule flattening.
   - **Verify:** `cargo check -p shared && cargo check -p config-system`
4. **Phase 4** (P4.1-P4.4): Bug fixes — lookup depth, language aliases, local-only listing, I/O warnings.
   - **Verify:** `cargo check -p config-system`
5. **Phase 5** (P5.1): Aggregate facade cleanup.
   - **Verify:** `cargo check -p shared && cargo check -p config-system`
6. **Phase 6** (P6.1-P6.3): Performance — async I/O, bounded concurrency, config caching.
   - **Verify:** `cargo check -p config-system`
7. **Phase 7** (P7.1-P7.3): Documentation. Independent.

**Final verification (all phases complete):**
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
cargo run --bin lint-arwaky-cli -- check .
```

---

## Files Summary

### New files (4)
- `crates/shared/src/config-system/utility_config_parser.rs` — YAML parsing logic (P1.1)
- `crates/shared/src/config-system/utility_config_defaults.rs` — default config functions (P1.1)
- `crates/shared/src/config-system/taxonomy_config_language_vo.rs` — ConfigLanguage enum VO (P2.2)
- `crates/config-system/src/capabilities_workspace_detector_provider.rs` — workspace detection capabilities (P1.2)

### Modified files (6)
- `crates/shared/src/config-system/taxonomy_config_vo.rs` — remove parsing logic, keep VOs only (P1.1)
- `crates/shared/src/config-system/contract_reader_protocol.rs` — change read_config to return Result (P3.1)
- `crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs` — remove accessor methods (P5.1)
- `crates/shared/src/config-system/mod.rs` — register new modules (P1.1, P2.2)
- `crates/config-system/src/agent_config_orchestrator.rs` — use parser contract, bound concurrency, add cache (P1.3, P6.2, P6.3)
- `crates/config-system/src/capabilities_yaml_reader.rs` — XDG hardening, lookup depth, aliases, error handling (P2.3, P4.1-P4.4)
- `crates/config-system/src/mod.rs` — register new capabilities module (P1.2)

---

## Summary

| Phase | Items | Severity | Description |
|-------|-------|----------|-------------|
| 1 | P1.1-P1.3 | HIGH | Move parsing out of taxonomy, filesystem out of agent, use parser contract |
| 2 | P2.1-P2.3 | CRITICAL | Path confinement, ConfigLanguage enum, XDG hardening |
| 3 | P3.1-P3.3 | HIGH | Result error handling, warning-aware parsing, rule flattening fix |
| 4 | P4.1-P4.4 | HIGH/MEDIUM | Lookup depth, language aliases, local-only listing, I/O warnings |
| 5 | P5.1 | MEDIUM | Remove aggregate accessor methods |
| 6 | P6.1-P6.3 | MEDIUM | Async I/O, bounded concurrency, config caching |
| 7 | P7.1-P7.3 | LOW | FRD documentation updates |

**Total**: 17 items across 7 phases.

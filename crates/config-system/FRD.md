# FRD — config-system

## System Overview

```
┌─────────────────────────────────────────┐
│           Surface Layer                 │
│  surface_config_command.rs              │
├─────────────────────────────────────────┤
│           Agent Layer                   │
│  agent_config_orchestrator.rs           │
├─────────────────────────────────────────┤
│        Capabilities Layer               │
│  capabilities_yaml_reader.rs            │
│  capabilities_workspace_detector.rs     │
│  capabilities_rules_validator.rs        │
│  capabilities_parser_provider.rs        │
├─────────────────────────────────────────┤
│         Contract Layer                  │
│  contract_*.rs (protocols + aggregate)  │
├─────────────────────────────────────────┤
│         Taxonomy Layer                  │
│  taxonomy_*.rs (VOs, errors)            │
├─────────────────────────────────────────┤
│         Utility Layer                   │
│  utility_config_*.rs                    │
└─────────────────────────────────────────┘
```

The config-system crate manages lint-arwaky configuration: loading, parsing, validation, and workspace detection. It reads `lint_arwaky.config.*.yaml` files from multiple priority sources, merges them with embedded defaults, and provides a unified configuration facade for all other lint crates via `IConfigOrchestratorAggregate`.

## Functional Requirements

### FR-001: Config File Discovery and Loading
- **Description**: Locate and load the first matching YAML config file for a given project root and language, following a 5-level priority chain.
- **Input**: `project_root: FilePath`, `language: ConfigLanguage`
- **Output**: `Result<Option<ConfigSource>, ConfigError>` — the loaded config source with raw content, path, and language, or `None` if no config found.
- **Business Rules**:
  - Priority order: (1) project-root YAML, (2) parent directory YAML (up to depth 3), (3) XDG user config `~/.config/lint-arwaky/`, (4) XDG system dirs `/etc/xdg/lint-arwaky/` (limited to 8 dirs, absolute paths only), (5) embedded defaults.
  - First match wins — deeper/more specific configs take priority over shallower ones.
  - Config files exceeding 1 MiB (`MAX_CONFIG_FILE_SIZE`) are rejected.
  - Symlinks pointing outside the project root are rejected via canonical path resolution.
- **Edge Cases**:
  - No config file exists at any level → returns `None`, caller falls back to embedded defaults.
  - YAML parse failure → logs warning via `eprintln!`, continues searching next priority level.
  - Non-NotFound I/O error (e.g., permission denied) → logs warning, continues searching.
  - Rules with empty `conditions: []` are preserved (not dropped).
- **Error Handling**:
  - `InvalidData` error when config file exceeds 1 MiB.
  - `PermissionDenied` error when symlink points outside project root.
  - IO error on invalid path canonicalization.
  - ConfigError propagated from YAML parse or file read failures.

### FR-002: Language-Aware Config File Resolution
- **Description**: Map a `ConfigLanguage` enum to the correct set of config filenames to search for.
- **Input**: `ConfigLanguage` (Rust, Python, TypeScript)
- **Output**: `Vec<String>` of config filenames to search in priority order.
- **Business Rules**:
  - Rust → `lint_arwaky.config.rust.yaml`
  - Python → `lint_arwaky.config.python.yaml`
  - TypeScript → `lint_arwaky.config.typescript.yaml`, fallback to `lint_arwaky.config.javascript.yaml`
  - `ConfigLanguage` is an enum, not a string — prevents path injection (`language = "../../etc/passwd"`).
- **Edge Cases**:
  - Unknown language → no config files returned, embedded defaults used.
  - TypeScript config not found but JavaScript config exists → uses JavaScript config.
- **Error Handling**: None — pure mapping function.

### FR-003: Workspace Type Detection
- **Description**: Detect the language/type of a project by scanning for marker files (Cargo.toml, pyproject.toml, package.json, etc.) and parent directory conventions.
- **Input**: `path: FilePath`
- **Output**: `WorkspaceType` (Rust, Python, TypeScript, Unknown)
- **Business Rules**:
  - Single-pass directory scan for config files (single syscall instead of up to 10).
  - Parent directory name matching: `crates/` → Rust, `packages/` → TypeScript, `modules/` → Python.
  - Walks up to 2 parent directories if no marker found at target path.
- **Edge Cases**:
  - No marker files found at any level → returns `WorkspaceType::Unknown`.
  - Multiple marker files present (e.g., both Cargo.toml and package.json) → first match in scan order wins.
- **Error Handling**: `read_dir` failures are silently ignored (returns `None`), fallback to `Unknown`.

### FR-004: Multi-Workspace Member Discovery
- **Description**: Discover all workspace member directories under `crates/`, `packages/`, and `modules/` subdirectories.
- **Input**: `root: FilePath`
- **Output**: `Vec<FilePath>` of workspace member paths.
- **Business Rules**:
  - Scans for subdirectories under `crates/`, `packages/`, `modules/`.
  - Uses async I/O (`tokio::fs`) for non-blocking filesystem operations.
  - Concurrency bounded to 8 concurrent workspace loads via `buffered(8)`.
  - If root is itself a workspace directory (e.g., `crates/`), returns its direct subdirectories.
  - If root's parent is a workspace directory, returns root as a single-member workspace.
- **Edge Cases**:
  - No workspace directories found → returns empty vec, prints warning to stderr.
  - Symlink targets outside workspace root → pruned during file collection.
  - I/O error reading a workspace directory → warning logged, skipped.
- **Error Handling**: `eprintln!` warnings for directory read failures, graceful degradation.

### FR-005: Config Merging and Default Injection
- **Description**: Merge loaded config with embedded defaults using field-level merge rules.
- **Input**: `parsed: ArchitectureConfig`, `language: ConfigLanguage`
- **Output**: `ConfigResult` (merged config + source info + warnings)
- **Business Rules**:
  - **Layers** — concatenated; later definitions override earlier ones for the same layer name.
  - **Rules** — concatenated; rules are deduplicated by `name` field.
  - **Naming** — merged recursively; non-empty values override defaults.
  - **Ignored paths** — concatenated and deduplicated.
  - Empty arrays/objects in a child config do NOT override parent values.
  - When config has no layers, injects defaults for layers only and adds warning.
  - When no config file found, returns embedded defaults with warning.
- **Edge Cases**:
  - Config with empty `layers` array → defaults injected, warning emitted.
  - Duplicate rule names across configs → first occurrence wins.
  - Config error during load → falls back to embedded defaults with error warning.
- **Error Handling**: ConfigError logged as warning string, defaults used as fallback.

### FR-006: Config Validation
- **Description**: Validate loaded project config thresholds and adapter settings against schema constraints.
- **Input**: `config: ProjectConfig`, `adapter_name: AdapterName`
- **Output**: `ValidationResult` (ok or fail with error messages), `bool` (adapter enabled status)
- **Business Rules**:
  - Score threshold must be between 0.0 and 100.0 (inclusive).
  - Complexity threshold must be positive (> 0).
  - `max_file_lines` threshold must be positive (> 0).
  - Adapter enabled check: defaults to `true` if adapter not found in config.
- **Edge Cases**:
  - Score threshold at exactly 0 or 100 → valid.
  - Score threshold at 0.1 → valid.
  - Unknown adapter name → returns `true` (enabled by default).
- **Error Handling**: Multiple validation errors joined with ` | ` separator.

### FR-007: Config Caching
- **Description**: Cache parsed config by file path to avoid repeated YAML parsing.
- **Input**: `cache_key: String` (file path), `source: ConfigSource`
- **Output**: `ArchitectureConfig` (cached or freshly parsed)
- **Business Rules**:
  - Cache is `Mutex<HashMap<String, Arc<ArchitectureConfig>>>` with pre-allocated capacity of 32.
  - Uses `or_insert_with` to parse only on cache miss.
  - Thread-safe via `Mutex` with `unwrap_or_else(|e| e.into_inner())` for poisoned lock recovery.
- **Edge Cases**:
  - Poisoned mutex lock → recovers via `into_inner()`.
  - Same file path requested concurrently → only one parse occurs.
- **Error Handling**: Mutex poisoning handled gracefully.

### FR-008: Ignored Paths Assembly
- **Description**: Build the complete list of ignored paths from config + hardcoded defaults.
- **Input**: `config: ArchitectureConfig`
- **Output**: `Vec<String>` of ignored path patterns
- **Business Rules**:
  - Default ignored paths (hardcoded): `target`, `.mimocode`, `.agents`, `node_modules`, `build.rs`, `.git`, `dist`, `build`, `coverage`, `.venv`.
  - Config-specified ignored paths appended with deduplication via `HashSet`.
  - Path separators normalized to platform-specific `MAIN_SEPARATOR`.
  - Pre-allocated capacity: 10 defaults + config count.
- **Edge Cases**:
  - Config specifies a path already in defaults → deduplicated, not added twice.
  - Config specifies empty string path → filtered out.
- **Error Handling**: None — pure function.

### FR-009: TOML Config Parsing
- **Description**: Parse TOML config files (e.g., `Cargo.toml` `[tool.lint-arwaky]` section) into `ProjectConfig`.
- **Input**: `path: FilePath`
- **Output**: `Result<Option<ProjectConfig>, ConfigError>`
- **Business Rules**:
  - Reads the `[tool.lint-arwaky]` or `[tool.lint_arwaky]` section from TOML.
  - Converts TOML value to JSON, then deserializes to `ProjectConfig`.
  - Returns `Ok(None)` if no `[tool]` section exists (not an error).
- **Edge Cases**:
  - TOML file exists but has no `[tool]` section → returns `Ok(None)`.
  - TOML file is not valid TOML → returns `ConfigError`.
- **Error Handling**: `ConfigError` with specific keys (`tool.lint-arwaky`, `toml.convert`, `toml.parse`).

### FR-010: Config File Listing
- **Description**: List all config files found at the project root for all supported languages.
- **Input**: `project_root: FilePath`
- **Output**: `Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>`
- **Business Rules**:
  - Iterates all three languages (Rust, Python, TypeScript).
  - For each language, checks all config filenames at project root.
  - Deduplicates by path (same file not listed twice).
  - Breaks after first config found per language.
- **Edge Cases**:
  - Multiple languages have config files → all returned.
  - No config files for any language → returns empty vec.
  - I/O error reading a config file → warning logged, continues.
- **Error Handling**: `ConfigError` propagated for `FilePath` creation failures.

## Data Model / Entity Relationship

```
ArchitectureConfig
├── layers: Vec<ArchitectureLayer>
│   └── ArchitectureLayer
│       ├── name: String
│       ├── rules: Vec<ArchitectureRule>
│       └── conditions: Vec<Condition>
├── naming: NamingConfig
├── ignored_paths: IgnoredPaths
└── thresholds: Thresholds

ConfigSource
├── language: String
├── path: String
└── raw_content: String

ConfigResult
├── config: ArchitectureConfig
├── source: ConfigSource
└── warnings: Vec<String>

WorkspaceInfo
├── path: FilePath
├── workspace_type: String
└── config: ArchitectureConfig

ConfigLanguage (enum)
├── Rust
├── Python
└── TypeScript

WorkspaceType (enum)
├── Rust
├── Python
├── TypeScript
└── Unknown

ConfigError
├── key: ConfigKey
├── message: ErrorMessage
├── config_file: FilePath
```

## API Contract

| Function | Input | Output | Description |
|---|---|---|---|
| `load_project_config(project_root)` | `FilePath` | `ConfigResult` | Auto-detect language and load config |
| `load_config_for_language(project_root, language)` | `FilePath, ConfigLanguage` | `ConfigResult` | Load config for specific language |
| `discover_workspaces(root)` | `FilePath` | `Vec<WorkspaceInfo>` | Discover and load configs for all workspace members |
| `load_config_sync(project_root)` | `&str` | `ArchitectureConfig` | Synchronous config load (no async runtime) |
| `ignored_paths(project_root)` | `&str` | `Vec<String>` | Get merged ignored paths list |
| `ignored_paths_for_language(project_root, language)` | `&str, ConfigLanguage` | `Vec<String>` | Get ignored paths for specific language |
| `read_config(project_root, language)` | `FilePath, ConfigLanguage` | `Result<Option<ConfigSource>, ConfigError>` | Read raw config from filesystem |
| `list_config_files(project_root)` | `FilePath` | `Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>` | List all config files at project root |
| `detect(path)` | `FilePath` | `WorkspaceType` | Detect workspace type from marker files |
| `is_workspace(path)` | `FilePath` | `bool` | Check if path is a workspace root |
| `discover_workspace_members(root)` | `FilePath` | `Vec<FilePath>` | Find workspace member directories |
| `is_adapter_enabled(config, name)` | `ProjectConfig, AdapterName` | `bool` | Check if adapter is enabled in config |
| `validate_thresholds(config)` | `ProjectConfig` | `ValidationResult` | Validate config thresholds |
| `parse_yaml_config(path)` | `FilePath` | `Result<ProjectConfig, ConfigError>` | Parse YAML config file |
| `parse_toml_config(path)` | `FilePath` | `Result<Option<ProjectConfig>, ConfigError>` | Parse TOML config section |

## Integration Points

- **Internal**:
  - `shared` crate — taxonomy VOs (`ConfigLanguage`, `ArchitectureConfig`, `ConfigSource`, `ConfigResult`, `WorkspaceInfo`, `ConfigError`), contracts (`IConfigOrchestratorAggregate`, `IConfigReaderProtocol`, `IConfigValidatorProtocol`, `IWorkspaceDetectorProtocol`, `IConfigParserProtocol`), utility functions (`parse_config_yaml`, `default_config_for_language`, `merge_config`).
  - `config-system` root container (`root_config_system_container.rs`) — wires `ConfigOrchestrator`, `ConfigYamlReader`, `ConfigRulesValidator`, `ConfigParserProvider` via DI.
- **External**:
  - `dirs` crate — XDG config directory resolution.
  - `tokio::fs` — async filesystem I/O for workspace discovery.
  - `serde_yaml_ng` — YAML deserialization.
  - `toml` — TOML parsing for `[tool.lint-arwaky]` sections.

## Non-functional Requirements (Detailed)

- **Performance**: Config read from project root < 50ms; config read from XDG paths < 100ms; workspace discovery for 10 members < 500ms (concurrency bound of 8).
- **Memory**: Memory overhead per parsed config < 10 KB (cached); cache pre-allocated with capacity 32.
- **Concurrency**: Workspace discovery bounded to 8 concurrent loads via `buffered(8)`; config cache thread-safe via `Mutex`.
- **Security**: Symlink attack detection via O(1) canonical path check; config file size capped at 1 MiB; `ConfigLanguage` enum prevents path injection; XDG_CONFIG_DIRS limited to 8 entries, absolute paths only.
- **Reliability**: Poisoned mutex locks recovered via `into_inner()`; YAML parse failures produce warnings not silent defaults.

## Test Scenarios / QA Checklist

- [ ] FR-001: Config loaded from project root when `lint_arwaky.config.rust.yaml` exists
- [ ] FR-001: Config loaded from parent directory (depth 1-3) when not at root
- [ ] FR-001: XDG user config used when no project-root config exists
- [ ] FR-001: XDG system dirs searched in order when user config missing
- [ ] FR-001: Embedded defaults used when no config file found anywhere
- [ ] FR-001: Config file > 1 MiB rejected with error
- [ ] FR-001: Symlink outside project root rejected
- [ ] FR-002: TypeScript falls back to JavaScript config when `.typescript.yaml` not found
- [ ] FR-003: Rust detected from `Cargo.toml` presence
- [ ] FR-003: Python detected from `pyproject.toml`, `setup.py`, or `requirements.txt`
- [ ] FR-003: TypeScript detected from `package.json` or `tsconfig.json`
- [ ] FR-003: Parent directory `crates/` → Rust, `packages/` → TypeScript, `modules/` → Python
- [ ] FR-004: Workspace members discovered under `crates/`, `packages/`, `modules/`
- [ ] FR-004: Empty workspace list produces warning
- [ ] FR-005: Empty layers in config triggers default injection with warning
- [ ] FR-005: Duplicate rules deduplicated by name
- [ ] FR-005: Config error falls back to defaults with warning
- [ ] FR-006: Score threshold 0-100 accepted, values outside rejected
- [ ] FR-006: Unknown adapter defaults to enabled
- [ ] FR-007: Same config file parsed only once (cache hit)
- [ ] FR-008: Default ignored paths always present
- [ ] FR-008: Config ignored paths deduplicated with defaults
- [ ] FR-009: TOML `[tool.lint-arwaky]` section parsed correctly
- [ ] FR-009: TOML without `[tool]` returns `None`

## Assumptions & Constraints

- `ConfigLanguage` enum restricts input to exactly Rust, Python, TypeScript — no arbitrary strings allowed.
- Config file naming follows the `lint_arwaky.config.{lang}.yaml` convention strictly.
- Workspace structure must follow `crates/`, `packages/`, `modules/` convention.
- Maximum 8 XDG_CONFIG_DIRS entries; only absolute paths accepted.
- Maximum config file size: 1 MiB.
- Workspace discovery concurrency: 8 concurrent loads maximum.
- YAML parsing uses `serde_yaml_ng` (YAML 1.2).
- TOML parsing reads only `[tool]` section, not full TOML config.

## Glossary

| Term | Definition |
|---|---|
| AES | Architecture Enforcement Specification — the coding standard enforced by lint-arwaky |
| ConfigLanguage | Typed enum restricting language input to Rust, Python, TypeScript |
| WorkspaceType | Enum identifying project language from marker files |
| ArchitectureConfig | Parsed configuration containing layers, rules, naming, and thresholds |
| ConfigSource | Metadata about a loaded config file (language, path, raw content) |
| ConfigResult | Merged config + source info + warnings from the loading process |
| XDG | XDG Base Directory Specification — standard for user/system config paths |

## Reference

- PRD: [PRD.md](../../PRD.md)

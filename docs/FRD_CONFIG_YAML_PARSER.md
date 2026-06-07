# FRD — Config YAML Parser

> **Feature**: Replace hardcoded config paths with dynamic YAML file reader for 3 languages

## 1. Problem Statement

Currently, Lint Arwaky **hardcodes config paths** in multiple places:

| Location                                        | Problem                                                                                          |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `capabilities/architecture_lint_handler.rs`   | Walks up dirs looking for hardcoded `lint_arwaky.config.rust.yaml`                             |
| `taxonomy/architecture_config_vo.rs`          | Compile-time `include_str!("../../lint_arwaky.config.rust.yaml")` — only Rust config embedded |
| `infrastructure/javascript_linter_adapter.rs` | Checks for `lint_arwaky.config.python.yaml` (wrong name for JS)                                |
| `infrastructure/config_discovery_provider.rs` | Missing `lint_arwaky.config.javascript.yaml` and `lint_arwaky.config.rust.yaml`              |

Also, **config loading lives in the wrong layer**: `ArchitectureLintHandler` (capability) does YAML reading and parsing directly — it should read via infrastructure adapters.

## 2. Solution Overview

| Layer                    | New/Changed                  | Responsibility                                                                                                          |
| ------------------------ | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| **Taxonomy**       | New VOs                      | `ProjectLanguage`, `ConfigResult`, `LanguageSource` language detection result                                     |
| **Contract**       | New port + enhanced protocol | `IConfigReaderPort` for file reading, `ILanguageDetectorPort` for language detection                                |
| **Infrastructure** | New adapter                  | `ConfigYamlReader` reads 3 config files by language; `LanguageDetectorProvider` detects language from dir structure |
| **Capabilities**   | New processor                | `ConfigOrchestrationProcessor` — decides WHICH config to load based on detected language                             |
| **Agent**          | New orchestrator             | `ConfigLoaderOrchestrator` — wires config loading flow: detect language → read config → parse → return            |
| **Surfaces**       | Cleanup                      | Remove hardcoded path detection from `cli_check_command.rs`                                                           |

### Flow

```
User runs: lint-arwaky-cli scan /some/project

Surface (cli_check_command):
  └─► Agent (ConfigLoaderOrchestrator):
        ├─► detect_language(/some/project) → LanguageSource(Rust | Python | JavaScript)
        │     └─► Infrastructure (LanguageDetectorProvider):
        │           ├─ Check for src-rust/ → Rust
        │           ├─ Check for src-python/ → Python
        │           ├─ Check for src-javascript/ → JavaScript
        │           ├─ Check for Cargo.toml → Rust
        │           ├─ Check for pyproject.toml → Python
        │           └─ Check for package.json → JavaScript
        │
        ├─► read_config(/some/project, language) → ConfigResult
        │     └─► Infrastructure (ConfigYamlReader):
        │           ├─ If Rust        → read lint_arwaky.config.rust.yaml
        │           ├─ If Python      → read lint_arwaky.config.python.yaml
        │           └─ If JavaScript  → read lint_arwaky.config.javascript.yaml
        │
        └─► parse_config(raw_yaml) → ArchitectureConfig
              └─► Infrastructure (ConfigParserProvider):
                    ├─ serde_yaml::from_str → serde_json::Value
                    ├─ Extract "architecture" key
                    ├─ Convert suffix format
                    ├─ Flatten rules (global/internal/external)
                    └─ Deserialize → ArchitectureConfig

Capability (ConfigOrchestrationProcessor):
  └─► Receives ArchitectureConfig, passes to checkers/analyzers
```

## 3. Taxonomy Changes

### New Value Objects

| VO                  | File                       | Fields                                                                                        | Description                          |
| ------------------- | -------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------ |
| `ProjectLanguage` | `project_language_vo.rs` | `language: String` (rust \| python \| javascript)                                           | Detected language of scanned project |
| `LanguageSource`  | `project_language_vo.rs` | `language: ProjectLanguage`, `confidence: u8`, `source: String` (dir \| file \| config) | Detection result with confidence     |
| `ConfigSource`    | `config_source_vo.rs`    | `language: ProjectLanguage`, `path: FilePath`, `raw_content: ContentString`             | Raw YAML content read from disk      |
| `ConfigResult`    | `config_source_vo.rs`    | `config: ArchitectureConfig`, `source: ConfigSource`, `warnings: Vec<ErrorMessage>`     | Final parsed result                  |

### Existing VOs to Keep

- `ArchitectureConfig` — unchanged
- `LayerDefinition` — unchanged
- `ArchitectureRule` — unchanged
- `ProjectConfig` — unchanged
- `LayerNameVO` — unchanged

## 4. Contract Changes

### New Port: `IConfigReaderPort`

File: `contract/config_reader_port.rs`

```rust
#[async_trait]
pub trait IConfigReaderPort: Send + Sync {
    /// Read the correct config file based on detected language
    async fn read_config(
        &self,
        project_root: &DirectoryPath,
        language: &ProjectLanguage,
    ) -> Result<ConfigSource, ConfigError>;

    /// List available config files in project root
    async fn list_config_files(
        &self,
        project_root: &DirectoryPath,
    ) -> Vec<(ProjectLanguage, FilePath)>;
}
```

### New Port: `ILanguageDetectorPort`

File: `contract/language_detector_port.rs`

```rust
#[async_trait]
pub trait ILanguageDetectorPort: Send + Sync {
    /// Detect the primary language of a project
    async fn detect_language(
        &self,
        project_root: &DirectoryPath,
    ) -> LanguageSource;
}
```

### Enhanced Protocol: `IConfigOrchestrationProtocol`

File: `contract/config_orchestration_protocol.rs`

```rust
#[async_trait]
pub trait IConfigOrchestrationProtocol: Send + Sync {
    /// Full flow: detect language → read config → parse → return ArchitectureConfig
    async fn load_project_config(
        &self,
        project_root: &DirectoryPath,
    ) -> Result<ConfigResult, ConfigError>;

    /// Load config for a specific language (override detection)
    async fn load_config_for_language(
        &self,
        project_root: &DirectoryPath,
        language: &ProjectLanguage,
    ) -> Result<ConfigResult, ConfigError>;
}
```

### Updated ServiceContainerAggregate

Add config-related accessor methods:

```rust
pub trait ServiceContainerAggregate: Send + Sync {
    // existing methods...
  
    /// NEW: Get config orchestrator
    fn get_config_orchestrator(&self) -> Arc<dyn IConfigOrchestrationProtocol>;
  
    /// NEW: Get language detector
    fn get_language_detector(&self) -> Arc<dyn ILanguageDetectorPort>;
}
```

## 5. Infrastructure Changes

### NEW: `ConfigYamlReader`

File: `infrastructure/config_yaml_reader.rs`

```rust
pub struct ConfigYamlReader {
    file_system: Arc<dyn IFileSystemPort>,
}

impl IConfigReaderPort for ConfigYamlReader { ... }
```

**Logic**:

- Given `ProjectLanguage`, construct the correct filename:
  - Rust → `lint_arwaky.config.rust.yaml`
  - Python → `lint_arwaky.config.python.yaml`
  - JavaScript → `lint_arwaky.config.javascript.yaml`
- Look in `project_root`, walk up if not found
- Return raw YAML content as `ConfigSource`

### NEW: `LanguageDetectorProvider`

File: `infrastructure/language_detector_provider.rs`

```rust
pub struct LanguageDetectorProvider {
    file_system: Arc<dyn IFileSystemPort>,
}

impl ILanguageDetectorPort for LanguageDetectorProvider { ... }
```

**Detection priority** (highest confidence first):

| Check                                                             | Language   | Confidence |
| ----------------------------------------------------------------- | ---------- | ---------- |
| `src-rust/` exists                                              | Rust       | 100        |
| `Cargo.toml` exists                                             | Rust       | 90         |
| `src-python/` exists                                            | Python     | 100        |
| `pyproject.toml` or `setup.py` or `requirements.txt` exists | Python     | 90         |
| `src-javascript/` exists                                        | JavaScript | 100        |
| `package.json` exists                                           | JavaScript | 90         |
| `src/` exists + `.rs` files                                   | Rust       | 70         |
| `src/` exists + `.py` files                                   | Python     | 70         |
| `src/` exists + `.js`/`.ts` files                           | JavaScript | 70         |
| Fallback: check file extensions across project                    | Best match | 50         |

### UPDATED: `ConfigParserProvider`

File: `infrastructure/config_parser_provider.rs`

Enhance existing `ConfigParserProvider` to:

- Accept raw YAML content string instead of file path
- Handle all 3 config files (the YAML structure is the same, only filename differs)
- Return `ArchitectureConfig` (not `ProjectConfig`)

### Files to DELETE

| File                                            | Reason                                                                |
| ----------------------------------------------- | --------------------------------------------------------------------- |
| `infrastructure/config_discovery_provider.rs` | Logic replaced by `ConfigYamlReader` + `LanguageDetectorProvider` |

## 6. Capability Changes

### NEW: `ConfigOrchestrationProcessor`

File: `capabilities/config_orchestration_processor.rs`

```rust
pub struct ConfigOrchestrationProcessor {
    config_reader: Arc<dyn IConfigReaderPort>,
    language_detector: Arc<dyn ILanguageDetectorPort>,
    config_parser: Arc<dyn IConfigParserPort>,
}

impl IConfigOrchestrationProtocol for ConfigOrchestrationProcessor { ... }
```

**Logic for `load_project_config()`**:

1. Call `language_detector.detect_language(project_root)` → get `ProjectLanguage`
2. Call `config_reader.read_config(project_root, language)` → get raw YAML as `ConfigSource`
3. Call `config_parser.parse_raw_yaml(config_source.raw_content)` → get `ArchitectureConfig`
4. Return `ConfigResult { config, source, warnings }`

**Logic for fallback** (no config found):

1. Return built-in default config (`default_aes_config()`)
2. Add warning: "No config file found, using built-in defaults"

### Files to DELETE

| File                                                     | Reason                                                                                                                                                                                                 |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `capabilities/architecture_lint_handler.rs` — REPLACE | Config loading logic moved to `ConfigOrchestrationProcessor`. Keep file but remove `try_load_yaml_config()`, `load_config()`, `default_aes_config()` — these move to infrastructure/processor |

## 7. Agent Changes

### NEW: `ConfigLoaderOrchestrator` (simplified)

File: Not needed as separate file — logic can live in `DependencyInjectionContainer` directly.

### Updated: `DependencyInjectionContainer`

File: `agent/dependency_injection_container.rs`

**Wiring**:

```rust
// NEW config adapters
let config_reader = Arc::new(ConfigYamlReader::new(file_system.clone()));
let language_detector = Arc::new(LanguageDetectorProvider::new(file_system.clone()));
let config_parser = Arc::new(ConfigParserProvider::new());

// NEW config orchestrator processor
let config_orchestrator = Arc::new(ConfigOrchestrationProcessor::new(
    config_reader,
    language_detector,
    config_parser,
));
```

**Expose**:

```rust
impl ServiceContainerAggregate for DependencyInjectionContainer {
    // NEW methods:
    fn get_config_orchestrator(&self) -> Arc<dyn IConfigOrchestrationProtocol> {
        self.config_orchestrator.clone()
    }
    fn get_language_detector(&self) -> Arc<dyn ILanguageDetectorPort> {
        self.language_detector.clone()
    }
}
```

### Updated: `ArchitectureLintHandler`

**Simplify**: Remove `try_load_yaml_config()`, `load_config()`, `default_aes_config()` methods. Instead, call `container.get_config_orchestrator().load_project_config(root)`.

## 8. Surface Changes

### Updated: `cli_check_command.rs`

Remove hardcoded directory checks for `src-rust`, `src-python`, `src-javascript`, `src`. Instead, delegate language detection to `ILanguageDetectorPort` via config orchestrator.

### Updated: `cli_main_entry.rs`

Remove hardcoded config path from `SetupCommands::Init`.

## 9. Files Summary

### New Files (6)

| File                                               | Layer          |
| -------------------------------------------------- | -------------- |
| `taxonomy/project_language_vo.rs`                | Taxonomy       |
| `taxonomy/config_source_vo.rs`                   | Taxonomy       |
| `contract/config_reader_port.rs`                 | Contract       |
| `contract/language_detector_port.rs`             | Contract       |
| `contract/config_orchestration_protocol.rs`      | Contract       |
| `infrastructure/config_yaml_reader.rs`           | Infrastructure |
| `infrastructure/language_detector_provider.rs`   | Infrastructure |
| `capabilities/config_orchestration_processor.rs` | Capabilities   |

### Modified Files (6)

| File                                          | Change                                                         |
| --------------------------------------------- | -------------------------------------------------------------- |
| `contract/service_container_aggregate.rs`   | Add `get_config_orchestrator()`, `get_language_detector()` |
| `infrastructure/config_parser_provider.rs`  | Add `parse_raw_yaml(content)` method                         |
| `capabilities/architecture_lint_handler.rs` | Remove config loading logic, delegate to orchestrator          |
| `agent/dependency_injection_container.rs`   | Wire config adapters + orchestrator                            |
| `surfaces/cli_check_command.rs`             | Remove hardcoded dir checks                                    |
| `taxonomy/architecture_config_vo.rs`        | Remove `include_str!` embedded default                       |

### Deleted Files (1)

| File                                            | Reason                                                          |
| ----------------------------------------------- | --------------------------------------------------------------- |
| `infrastructure/config_discovery_provider.rs` | Replaced by `ConfigYamlReader` + `LanguageDetectorProvider` |

## 10. AES Compliance

| Rule   | Compliance                                                                                                                        |
| ------ | --------------------------------------------------------------------------------------------------------------------------------- |
| AES001 | Capability → Infrastructure through `IConfigReaderPort`/`ILanguageDetectorPort` (ports), not direct imports ✅               |
| AES002 | All new files implement mandatory contract traits ✅                                                                              |
| AES011 | Suffixes:`_vo`, `_port`, `_protocol`, `_reader`, `_provider`, `_detector`, `_processor` — all allowed per layer ✅ |
| AES003 | Filenames: 3-word pattern ✅                                                                                                      |
| AES012 | Barrel re-exports required ✅                                                                                                     |
| AES027 | Every logic file implements a contract trait ✅                                                                                   |

## 11. Implementation Order

1. **Taxonomy**: Create `ProjectLanguage`, `LanguageSource`, `ConfigSource`, `ConfigResult` VOs
2. **Contract**: Create `IConfigReaderPort`, `ILanguageDetectorPort`, `IConfigOrchestrationProtocol`; update `ServiceContainerAggregate`
3. **Infrastructure**: Create `ConfigYamlReader`, `LanguageDetectorProvider`; update `ConfigParserProvider`
4. **Capabilities**: Create `ConfigOrchestrationProcessor`; simplify `ArchitectureLintHandler`
5. **Agent**: Wire in `DependencyInjectionContainer`
6. **Surfaces**: Clean up `cli_check_command.rs`, `cli_main_entry.rs`
7. **Cleanup**: Delete old files, update barrel exports, test

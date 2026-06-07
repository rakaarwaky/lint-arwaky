# FRD — Config YAML Parser

> **PRD Reference**: [FR-002](PRD.md) — Config system: multi-config support, YAML reader, language detection, config-driven rules
> **Dependency**: FR-001 (6-layer AES architecture)
> **Status**: ✅ **PRODUCTION-READY** — All files implemented with real code. Language detection (4-step), config YAML reader, config orchestration processor all working. `config_discovery_provider.rs` removed (replaced by `ConfigYamlReader` + `LanguageDetectorProvider`). Moved to agent layer as `ConfigLoadingOrchestrator`. All 3 YAML configs (Rust/Python/JS) baked into binary via `include_str!` at compile time.

## 1. Problem Statement

Config loading was previously hardcoded in many places:

| Location | Problem |
|----------|---------|
| `capabilities/architecture_lint_handler.rs` | Searches for `lint_arwaky.config.rust.yaml` with hardcoded path — traverses parent directory manually |
| `taxonomy/architecture_config_vo.rs` | `include_str!("../../lint_arwaky.config.rust.yaml")` — only embeds Rust config |
| `infrastructure/javascript_linter_adapter.rs` | Actually looks for `lint_arwaky.config.python.yaml` for JS — wrong filename |
| `infrastructure/config_discovery_provider.rs` | Config for JS and Rust not registered |

Also, config loading was in the wrong layer: `ArchitectureLintHandler` (capabilities) reads files directly. Infrastructure should handle I/O.

## 2. Basic Concept

Each project has a dominant programming language (Rust, Python, JavaScript). Each has its own config file:

```
Project Rust:     ./lint_arwaky.config.rust.yaml
Project Python:   ./lint_arwaky.config.python.yaml
Project JS/TS:    ./lint_arwaky.config.javascript.yaml
```

When the user runs `lint-arwaky-cli scan /some/project`, the system must:
1. **Detect language** of the project (from directory structure)
2. **Read config** matching that language
3. **Parse YAML** → `ArchitectureConfig`
4. **Use config** for all rule checking

## 3. Working Mechanism — Step by Step

### 3.1 Language Detection (`LanguageDetectorProvider`)

```
Input: /some/project

Step 1: Check source directory (confidence 100)
  ├── /some/project/src-rust/      → Rust ✅
  ├── /some/project/src-python/    → Python
  └── /some/project/src-javascript/ → JavaScript

Step 2: Check manifest file (confidence 90)
  ├── Cargo.toml → Rust
  ├── pyproject.toml / setup.py / requirements.txt → Python
  └── package.json → JavaScript

Step 3: Check src/ + file extension (confidence 70)
  ├── src/ + *.rs → Rust
  ├── src/ + *.py → Python
  └── src/ + *.js/.ts → JavaScript

Step 4: Fallback — scan all files in project (confidence 50)
  └── Count most frequent extension → guess language

Output: LanguageSource { language: "rust", confidence: 100, source: "directory" }
```

### 3.2 Config File Reading (`ConfigYamlReader`)

```
Input: /some/project, language = "rust"

Step 1: Determine config filename
  └── LanguageConfig:
        rust       → "lint_arwaky.config.rust.yaml"
        python     → "lint_arwaky.config.python.yaml"
        javascript → "lint_arwaky.config.javascript.yaml"

Step 2: Find file
  ├── /some/project/lint_arwaky.config.rust.yaml  ← Search in project root first
  ├── If not found → traverse up to parent directory
  └── If not found at all → fallback to embedded default

Step 3: Read file
  └── fs::read_to_string() → raw YAML string

Output: ConfigSource { language: "rust", path: "/some/project/...", raw_content: "layers:\n  ..." }
```

### 3.3 YAML Parsing (`ConfigParserProvider`)

```
Input: Raw YAML string

Step 1: serde_yaml::from_str(raw) → serde_json::Value
Step 2: Extract key "architecture"
Step 3: Flatten rules: global + internal + external
Step 4: Deserialize → ArchitectureConfig

ArchitectureConfig {
    layers: Vec<LayerDefinition>,      // Definitions of each layer
    rules: Vec<ArchitectureRule>,       // All AES rules
    naming: NamingConfig,               // Naming rules
    scoring: ScoringConfig,             // Score threshold
    bypass: BypassConfig,               // Bypass patterns
}
```

### 3.4 Config Orchestration (`ConfigOrchestrationProcessor`)

```
User: lint-arwaky-cli scan /some/project
    │
    ▼
Surface (cli_check_command)
    │
    ▼
ConfigOrchestrationProcessor.load_project_config("/some/project")
    │
    ├─► ILanguageDetectorPort.detect_language("/some/project")
    │     └─→ LanguageSource { "rust", 100 }
    │
    ├─► IConfigReaderPort.read_config("/some/project", "rust")
    │     └─→ ConfigSource (raw YAML)
    │
    ├─► IConfigParserPort.parse_raw_yaml(raw_yaml)
    │     └─→ ArchitectureConfig
    │
    └─→ ConfigResult { config: ArchitectureConfig, source: ConfigSource, warnings: [] }

If no config file found:
    └─→ default_aes_config()  ← embedded include_str!("../../lint_arwaky.config.rust.yaml")
         └─→ ConfigResult { warnings: ["No config file found, using built-in defaults"] }
```

## 4. Config YAML Structure

```yaml
# lint_arwaky.config.rust.yaml
architecture:
  layers:
    - name: taxonomy
      path: src-rust/taxonomy/
      suffixes: [vo, entity, event, error, constant]
      allowed_imports: [taxonomy]
      forbidden_imports: [agent, infrastructure, surfaces, contract, capabilities]
      mandatory_imports: []
      recursive: true
      min_lines: 10
      max_lines: 500
      no_primitives: true
      
    - name: contract
      path: src-rust/contract/
      suffixes: [port, protocol, aggregate]
      allowed_imports: [taxonomy, contract]
      forbidden_imports: [agent, infrastructure, surfaces, capabilities]
      mandatory_imports: [taxonomy]
      recursive: true
      min_lines: 5
      max_lines: 300
      no_primitives: true

    # ... other layers with the same structure

  rules:
    - code: AES001
      name: import-layer-violation
      severity: HIGH
      enabled: true
    - code: AES002
      name: mandatory-import-missing
      severity: HIGH
      enabled: true
    # ... all 31 rules

  naming:
    pattern: "^[a-z]+_[a-z]+_[a-z]+\\.rs$"
    exceptions: ["main.rs", "lib.rs", "mod.rs"]

  scoring:
    initial_score: 100.0
    penalties:
      LOW: 1.0
      MEDIUM: 2.0
      HIGH: 3.0
      CRITICAL: 5.0
    auto_fail_on_critical: true

  bypass:
    patterns:
      - "#[allow\\("]
      - "unwrap\\(\\)"
      - "panic!"
      - "noqa"
      - "type: ignore"
      - "# type:"
```

## 5. Key Files

### Taxonomy (2 new VOs)
| File | Struct | Field |
|------|--------|-------|
| `taxonomy/project_language_vo.rs` | `ProjectLanguage` | `language: String` |
| | `LanguageSource` | `language`, `confidence: u8`, `source: String` |
| `taxonomy/config_source_vo.rs` | `ConfigSource` | `language`, `path`, `raw_content` |
| | `ConfigResult` | `config`, `source`, `warnings` |

### Contract (3 new ports)
| File | Trait | Method |
|------|-------|--------|
| `contract/config_reader_port.rs` | `IConfigReaderPort` | `read_config(project_root, language) → ConfigSource`, `list_config_files(project_root) → Vec<(Language, Path)>` |
| `contract/language_detector_port.rs` | `ILanguageDetectorPort` | `detect_language(project_root) → LanguageSource` |
| `contract/config_orchestration_protocol.rs` | `IConfigOrchestrationProtocol` | `load_project_config(project_root) → ConfigResult`, `load_config_for_language(project_root, language) → ConfigResult` |

### Infrastructure (2 new, 1 updated)
| File | Class | Implements |
|------|-------|------------|
| `infrastructure/config_yaml_reader.rs` | `ConfigYamlReader` | `IConfigReaderPort` |
| `infrastructure/language_detector_provider.rs` | `LanguageDetectorProvider` | `ILanguageDetectorPort` |
| `infrastructure/config_parser_provider.rs` | `ConfigParserProvider` | (updated) add `parse_raw_yaml(content)` |

### Agent (1 new)
| File | Class | Implements |
|------|-------|------------|
| `agent/config_loading_orchestrator.rs` | `ConfigLoadingOrchestrator` | `IConfigOrchestrationProtocol` |

### Removed
| File | Reason |
|------|--------|
| `infrastructure/config_discovery_provider.rs` | Replaced by `ConfigYamlReader` + `LanguageDetectorProvider`. Removed from barrel (`infrastructure/mod.rs`). |

## 6. Complete Data Flow

```
YAML File (disk)
    │
    ▼
ConfigYamlReader.read_config()      ← Infrastructure
    │  fs::read_to_string()
    ▼
Raw YAML String (ConfigSource)
    │
    ▼
ConfigParserProvider.parse_raw_yaml()  ← Infrastructure
    │  serde_yaml::from_str()
    ▼
ArchitectureConfig (struct)
    │
    ▼
ConfigOrchestrationProcessor  ← Capabilities
    │  Combine source + config + warnings
    ▼
ConfigResult
    │
    ▼
DI Container (agent) → Surface (CLI)
    │
    ▼
LintCheckingCoordinator.run_all_checks()
    │  Use ArchitectureConfig.rules for each check
    ▼
Each checker reads from config:
    ├── naming_checker → config.naming.pattern
    ├── import_checker → config.layers[].allowed_imports / forbidden_imports
    ├── metric_checker → config.layers[].min_lines / max_lines
    ├── bypass_checker → config.bypass.patterns
    └── scoring → config.scoring.penalties
```

## 7. AES Compliance

| Rule | Compliance |
|------|------------|
| AES001 | `ConfigOrchestrationProcessor` (capabilities) calls `IConfigReaderPort` (contract) — not directly importing infrastructure |
| AES002 | All classes must implement contract trait |
| AES003 | All filenames are 3-word: `config_yaml_reader`, `language_detector_provider`, `config_orchestration_processor` |
| AES011 | Suffix: `_reader`, `_provider`, `_detector`, `_processor` — all allowed per layer |
| AES027 | Each logic file implements a trait (`IConfigReaderPort`, `ILanguageDetectorPort`, `IConfigOrchestrationProtocol`) |

## 8. Acceptance Criteria

| # | Criteria | Mechanism | Status |
|---|----------|-----------|--------|
| AC001 | Config loading moved from capability to infrastructure | `ArchitectureLintHandler` previously: `fs::read()` directly + `serde_yaml::from_str()`. Now: delegates to `ConfigYamlReader` + `ConfigParserProvider` | ✅ |
| AC002 | Read 3 configs based on language | `IConfigReaderPort` selects filename from `ProjectLanguage` | ✅ |
| AC003 | Detect language from directory structure | `LanguageDetectorProvider` checks `src-{lang}/` → Cargo.toml → pyproject.toml → package.json → extensions | ✅ |
| AC004 | Fallback to default if config is missing | `default_aes_config()` called with warning | ✅ |
| AC005 | `include_str!` default as built-in fallback | All 3 configs embedded: `architecture_config_vo.rs` has `include_str!("../../lint_arwaky.config.{rust,python,javascript}.yaml")` | ✅ |
| AC006 | `config_discovery_provider.rs` removed from barrel | File removed from `infrastructure/mod.rs` | ✅ |
| AC007 | Blocking I/O uses `spawn_blocking` | `tokio::task::spawn_blocking(|| fs::read_to_string())` | ✅ |
| AC008 | Runtime reuse via `Handle::try_current()` | Does not create a new Runtime on every call | ✅ |
| AC009 | `has_src` guard still works | Before running AES lint, check `src-rust/` exists first | ✅ |
| AC010 | Barrel exports updated in all layers | Every `mod.rs` re-exports new files | ✅ |
| AC011 | `cargo check --bin lint-arwaky-cli` passes | Compiles without errors | ✅ |
| AC012 | `cargo test` — 46 passed, 0 failed | Unit tests pass | ✅ |

(End of file - total 293 lines)

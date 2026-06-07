# FRD — Config YAML Parser

> **PRD Reference**: [FR-002](PRD.md) — Config system: multi-config support, YAML reader, language detection, config-driven rules
> **Dependency**: FR-001 (6-layer AES architecture)
> **Status**: ✅ COMPLETE — Multi-config YAML reader, language detector, config orchestrator implemented

## 1. Problem Statement

Currently, Lint Arwaky **hardcodes config paths** in multiple places:

| Location | Problem |
|----------|---------|
| `capabilities/architecture_lint_handler.rs` | Walks up dirs looking for hardcoded `lint_arwaky.config.rust.yaml` |
| `taxonomy/architecture_config_vo.rs` | Compile-time `include_str!("../../lint_arwaky.config.rust.yaml")` — only Rust config embedded |
| `infrastructure/javascript_linter_adapter.rs` | Checks for `lint_arwaky.config.python.yaml` (wrong name for JS) |
| `infrastructure/config_discovery_provider.rs` | Missing `lint_arwaky.config.javascript.yaml` and `lint_arwaky.config.rust.yaml` |

Also, **config loading lives in the wrong layer**: `ArchitectureLintHandler` (capability) does YAML reading and parsing directly — it should read via infrastructure adapters.

## 2. Solution Overview

| Layer | New/Changed | Responsibility |
|-------|-------------|----------------|
| **Taxonomy** | New VOs | `ProjectLanguage`, `ConfigResult`, `LanguageSource` |
| **Contract** | New port + protocol | `IConfigReaderPort`, `ILanguageDetectorPort`, `IConfigOrchestrationProtocol` |
| **Infrastructure** | New adapters | `ConfigYamlReader`, `LanguageDetectorProvider`, updated `ConfigParserProvider` |
| **Capabilities** | New processor | `ConfigOrchestrationProcessor` — decides which config to load |
| **Agent** | Updated DI | Wire config orchestrator |
| **Surfaces** | Cleanup | Remove hardcoded dir checks |

### Flow

```
User runs: lint-arwaky-cli scan /some/project
  └─► Surface (cli_check_command)
        └─► Agent (ConfigLoaderOrchestrator)
              ├─► LanguageDetectorProvider.detect_language()
              │     ├─► src-rust/ OR Cargo.toml → Rust
              │     ├─► src-python/ OR pyproject.toml → Python
              │     └─► src-javascript/ OR package.json → JavaScript
              ├─► ConfigYamlReader.read_config(language)
              │     ├─► Rust → lint_arwaky.config.rust.yaml
              │     ├─► Python → lint_arwaky.config.python.yaml
              │     └─► JavaScript → lint_arwaky.config.javascript.yaml
              └─► ConfigParserProvider.parse_raw_yaml() → ArchitectureConfig
```

## 3. Taxonomy Changes

### New Value Objects

| VO | File | Fields |
|----|------|--------|
| `ProjectLanguage` | `project_language_vo.rs` | `language: String` |
| `LanguageSource` | `project_language_vo.rs` | `language`, `confidence: u8`, `source: String` |
| `ConfigSource` | `config_source_vo.rs` | `language`, `path`, `raw_content` |
| `ConfigResult` | `config_source_vo.rs` | `config`, `source`, `warnings` |

## 4. Contract Changes

### New Ports

| File | Trait | Methods |
|------|-------|---------|
| `contract/config_reader_port.rs` | `IConfigReaderPort` | `read_config()`, `list_config_files()` |
| `contract/language_detector_port.rs` | `ILanguageDetectorPort` | `detect_language()` |
| `contract/config_orchestration_protocol.rs` | `IConfigOrchestrationProtocol` | `load_project_config()`, `load_config_for_language()` |

## 5. Infrastructure Changes

### NEW: `ConfigYamlReader`

File: `infrastructure/config_yaml_reader.rs`

Given `ProjectLanguage`, reads correct file: `lint_arwaky.config.{rust,python,javascript}.yaml`.

### NEW: `LanguageDetectorProvider`

File: `infrastructure/language_detector_provider.rs`

Detection priority: `src-{lang}/` (100) > manifest file (90) > `src/` + extensions (70) > fallback (50).

### UPDATED: `ConfigParserProvider`

Accept raw YAML string, handle all 3 config files.

## 6. Capability Changes

### NEW: `ConfigOrchestrationProcessor`

File: `capabilities/config_orchestration_processor.rs`

Flow: detect language → read config → parse → return `ConfigResult`. Falls back to `default_aes_config()` if no config found.

## 7. Agent Changes

Update `DependencyInjectionContainer` to wire config reader + detector + parser + orchestrator.

## 8. Surface Changes

Remove hardcoded directory checks from `cli_check_command.rs`.

## 9. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | Config loading moved from `ArchitectureLintHandler` to `ConfigOrchestrationProcessor` | ✅ |
| AC002 | `IConfigReaderPort` reads 3 config files by `ProjectLanguage` | ✅ |
| AC003 | `LanguageDetectorProvider` detects language from dir structure | ✅ |
| AC004 | Fallback to `default_aes_config()` if config not found | ✅ |
| AC005 | `include_str!` default restored as built-in fallback | ✅ |
| AC006 | `config_discovery_provider.rs` deleted | ✅ |
| AC007 | Blocking I/O wrapped in `spawn_blocking` | ✅ |
| AC008 | Runtime reuse via `Handle::try_current()` | ✅ |
| AC009 | `cli_check_command.rs` keeps `has_src` guard | ✅ |
| AC010 | Barrel exports updated in all layers | ✅ |
| AC011 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC012 | `cargo test` — 46 passed, 0 failed | ✅ |

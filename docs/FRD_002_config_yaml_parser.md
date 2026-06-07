# 📄 Feature Requirements Document (FRD)
**Feature Name:** Config YAML Parser  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the configuration system that reads, parses, and serves architecture rules from YAML config files. It covers multi-language config support (`lint_arwaky.config.rust.yaml`, `.python.yaml`, `.javascript.yaml`), language detection from project structure, and the config orchestration pipeline.

### 2.2 Scope
**In-Scope:**
- Reading config files by detected project language
- Language detection from directory structure and manifest files
- YAML parsing and deserialization into `ArchitectureConfig`
- Config orchestration: detect → read → parse → serve
- Built-in default config fallback

**Out-of-Scope:**
- CLI command parsing (handled by clap in surfaces layer)
- Writing/modifying config files
- Runtime config reload

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **ProjectLanguage** | Enum: Rust, Python, JavaScript |
| **LanguageSource** | Detection result with confidence score |
| **ConfigSource** | Raw YAML content read from disk |
| **ConfigResult** | Final parsed config with metadata |
| **ArchitectureConfig** | Deserialized struct with all layer definitions and rules |
| **LanguageDetectorProvider** | Infrastructure adapter that detects project language |
| **ConfigYamlReader** | Infrastructure adapter that reads config files |
| **ConfigOrchestrationProcessor** | Capability that orchestrates the full flow |

## 3. Feature Overview
### 3.1 Background & Problem
Config loading was previously hardcoded in many places: `architecture_lint_handler.rs` traversed parent directories looking for a hardcoded filename, `architecture_config_vo.rs` only embedded the Rust config at compile time, and `javascript_linter_adapter.rs` actually looked for the wrong config name (`.python.yaml` for JS projects). Config loading also lived in the wrong layer — capabilities should not read files directly.

### 3.2 Business Goals
- Centralize config loading in infrastructure layer
- Support 3 languages with correct config files
- Detect project language automatically
- Provide fallback defaults when no config file exists
- Make all architecture rules configurable via YAML without code changes

### 3.3 Target Users
- **Developers**: Configure architecture rules per project
- **DevOps**: Set up CI with custom thresholds
- **AI Agents**: Read config to understand project architecture

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer working on a Rust project, I want the linter to automatically detect my project language and load the correct config, so I don't have to specify it manually.
- **US-002:** As an architect, I want to customize layer suffix rules in YAML, so I can adapt the architecture to my project's needs without changing the linter code.
- **US-003:** As a CI pipeline, I want config loading to fall back to built-in defaults when no config file exists, so the linter never crashes due to missing configuration.

### 4.2 Use Cases & Workflow
**Language Detection Flow:**
```
Input: /some/project

Step 1: Check source directory (confidence 100)
  ├── /some/project/src-rust/ → Rust ✅
  ├── /some/project/src-python/ → Python
  └── /some/project/src-javascript/ → JavaScript

Step 2: Check manifest file (confidence 90)
  ├── Cargo.toml → Rust
  ├── pyproject.toml → Python
  └── package.json → JavaScript

Step 3: Check src/ + file extensions (confidence 70)
  ├── src/ + *.rs → Rust
  ├── src/ + *.py → Python
  └── src/ + *.js/.ts → JavaScript

Step 4: Fallback — scan all extensions (confidence 50)
  └── Most common extension → best guess
```

**Config Reading Flow:**
```
Input: project_root, language = "rust"

1. Determine filename: lint_arwaky.config.rust.yaml
2. Search: project_root → parent directories
3. If found → read with fs::read_to_string()
4. If not found → return default_aes_config() with warning
```

**Config Orchestration Flow:**
```
User: lint-arwaky-cli scan /project
  │
  ▼
ConfigOrchestrationProcessor.load_project_config()
  │
  ├─► detect_language("/project") → Rust
  ├─► read_config("/project", Rust) → raw YAML
  ├─► parse_raw_yaml(raw) → ArchitectureConfig
  └─► return ConfigResult { config, source, warnings }
```

### 4.3 Business Rules
- Config file naming: `lint_arwaky.config.{language}.yaml`
- Language detection priority: `src-{lang}/` (100) > manifest file (90) > `src/` + extensions (70) > fallback (50)
- If no config found → `default_aes_config()` built-in fallback
- If config parsing fails → return error with clear message

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Config loading latency | < 100ms |
| NFR-002 | Language detection accuracy | > 95% (Rust/Python/JS) |
| NFR-003 | Memory: embedded default config | < 50KB |

## 6. UI/UX Requirements
No direct UI. Feedback via:
- CLI output when config is loaded or falls back to defaults
- Warning message when no config file found

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Project has `src-rust/` directory | `detect_language()` runs | Language = Rust (confidence 100) | ❌ — `language_detector_provider.rs` empty, `language_detector_port.rs` empty |
| AC-002 | Project has `Cargo.toml` (no `src-rust/`) | `detect_language()` runs | Language = Rust (confidence 90) | ❌ — same as AC-001 |
| AC-003 | Project has `lint_arwaky.config.rust.yaml` | `read_config()` runs | ConfigSource with raw content | ❌ — `config_yaml_reader.rs` empty, `config_reader_port.rs` empty |
| AC-004 | Project has NO config file | `read_config()` runs | Fallback to `default_aes_config()` with warning | ✅ — only `architecture_config_vo.rs` and `config_parser_provider.rs` work |
| AC-005 | Malformed YAML in config file | `parse_raw_yaml()` runs | Error returned, not crash | ⚠️ — `config_parser_provider.rs` real but uses `unwrap()` (crash risk) |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (6-Layer Arch) | Config structure depends on layer definitions | If architecture changes, YAML schema must update | Architecture config uses generic schema |
| serde_yaml | External crate for YAML parsing | Version compatibility | Pinned in Cargo.toml: 0.9.34 |
| File system access | Config reading requires file I/O | IO error crashes | Wrapped in `spawn_blocking`, errors propagated |

## 9. Appendices
- `lint_arwaky.config.rust.yaml` — Example Rust config
- `lint_arwaky.config.python.yaml` — Example Python config
- `lint_arwaky.config.javascript.yaml` — Example JS/TS config
- `src-rust/infrastructure/language_detector_provider.rs` — Language detection implementation
- `src-rust/infrastructure/config_yaml_reader.rs` — Config reader implementation
- `src-rust/capabilities/config_orchestration_processor.rs` — Config orchestration
- `src-rust/taxonomy/architecture_config_vo.rs` — `default_aes_config()` embedded fallback

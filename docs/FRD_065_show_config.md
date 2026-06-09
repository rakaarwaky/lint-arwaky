# 📄 Feature Requirements Document (FRD)
**Feature Name:** Show Config (`config show`)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the show config CLI command `config show`. It displays the current active configuration loaded by Lint Arwaky, including all AES rule settings, layer definitions, CI threshold, and source file patterns. Shows both the file source and resolved (effective) configuration.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli config show` — display active configuration
- Resolved rules: all 31 AES rules with enabled/disabled/severity
- Layer definitions: prefixes, suffixes, policies
- CI settings: threshold, reporting options
- Source patterns: include/exclude globs
- Config source: file path or "built-in defaults"

**Out-of-Scope:**
- Editing configuration (handled by editing the YAML file directly)
- Config file creation (handled by FR-061)
- Environment diagnostics (handled by FR-060)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Resolved config** | The effective configuration after merging file + defaults |
| **Config source** | Path to the loaded config file, or "built-in defaults" |
| **RuleEntry** | Individual AES rule configuration: code, enabled, severity, options |

## 3. Feature Overview
### 3.1 Background & Problem
Users often didn't know which configuration was active — whether their YAML file was being loaded correctly, what rule severities were set, or whether the config they edited was taking effect. There was no way to inspect the resolved configuration.

### 3.2 Business Goals
- Provide transparency into active configuration
- Show which config file was loaded (or if defaults are used)
- Display all 31 AES rules with their resolved severity levels
- Enable debugging of configuration issues

### 3.3 Target Users
- **Developers**: Verify config edits are loading correctly
- **Architecture Engineers**: Review active rule set and severity levels
- **AI Agents**: Read config state to understand project rules

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `config show` to see which config file is active and what rules it contains.
- **US-002:** As an architect, I want to see the resolved severity for each AES rule, so I know which violations will fail the build.
- **US-003:** As a developer, I want to verify that my YAML edits are being picked up after I modify the config file.

### 4.2 Use Cases & Workflow
**Config Show Pipeline:**
```
lint-arwaky-cli config show
  │
  ├─► 1. Detect project language
  ├─► 2. Load config (same flow as FR-002)
  ├─► 3. Resolve: merge file config + built-in defaults
  └─► 4. Display:
        ├── Source info
        ├── Layer definitions
        ├── Rules table
        ├── CI settings
        └── Source patterns
```

**Resolved Config Data:**
```yaml
config_source:
  file: /project/lint_arwaky.config.rust.yaml
  status: loaded
  language: rust

layers:
  taxonomy:     prefixes=[taxonomy_]     suffixes=[_vo, _entity, _event, _error, _constant]
  contract:     prefixes=[contract_]     suffixes=[_port, _protocol, _aggregate]
  capabilities: prefixes=[capabilities_] suffixes=[_checker, _analyzer, _processor]
  infrastructure: prefixes=[infrastructure_] suffixes=[_adapter, _provider, _scanner]
  agent:        prefixes=[agent_]        suffixes=[_container, _orchestrator, _coordinator, _registry, _manager]
  surface:      prefixes=[surface_]      suffixes=[_command, _handler, _controller]

rules:
  AES001: enabled=true  severity=CRITICAL
  AES002: enabled=true  severity=HIGH
  AES003: enabled=true  severity=LOW
  # ... 28 more ...

ci:
  threshold: 70

source:
  include: ["src-rust/**/*.rs"]
  exclude: ["**/target/**"]
```

### 4.3 Business Rules
- Config is loaded using the same pipeline as FR-002 (detect → read → parse)
- Display includes both the raw file content AND the resolved (merged) config
- If no config file found, display "built-in defaults" as source
- Rules are always displayed in code order (AES001 → AES033)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Config load + display | < 200ms |
| NFR-002 | Display accuracy vs. actual runtime config | 100% identical |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli config show
⚙️  Lint Arwaky Configuration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📁 Config Source
  File:   /project/lint_arwaky.config.rust.yaml
  Status: ✅ Loaded
  Language: Rust

📐 Layer Architecture (6 layers)
  taxonomy:       prefixes=[taxonomy_]       suffixes=[_vo, _entity, ...]
  contract:       prefixes=[contract_]       suffixes=[_port, _protocol, ...]
  capabilities:   prefixes=[capabilities_]   suffixes=[_checker, _analyzer, ...]
  infrastructure: prefixes=[infrastructure_] suffixes=[_adapter, _provider, ...]
  agent:          prefixes=[agent_]          suffixes=[_container, _orchestrator, ...]
  surface:        prefixes=[surface_]        suffixes=[_command, _handler, ...]

📋 Rules (31 total)
  AES001  🔴 CRITICAL  enabled  Forbidden import
  AES002  🟠 HIGH      enabled  Mandatory import
  AES003  🟢 LOW       enabled  Naming convention
  AES004  🟡 MEDIUM    enabled  File size limit
  ...
  AES033  🟢 LOW       enabled  MCP schema

🎯 CI Settings
  Threshold: 70

📂 Source Patterns
  Include: src-rust/**/*.rs
  Exclude: **/target/**, **/vendor/**, **/node_modules/**
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Config file exists and is valid | `config show` runs | File path + all rules displayed | Pending Review |
| AC-002 | No config file exists | `config show` runs | "Built-in defaults" shown as source | Pending Review |
| AC-003 | Config file has custom severity | `config show` runs | Custom severity reflected in displayed rules | Pending Review |
| AC-004 | Config file has layer overrides | `config show` runs | Custom layer definitions shown | Pending Review |
| AC-005 | Malformed config file | `config show` runs | Error message with parse error details | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI config show command | `config-system/surface_config_command.rs` | — | **FULLY IMPLEMENTED** |
| Config loader (reused) | `config-system/agent_loading_orchestrator.rs` | 82 | **PARTIALLY IMPLEMENTED** — config-discard bug |
| Config display formatter | `config-system/capabilities_config_formatter.rs` | — | **FULLY IMPLEMENTED** — human-readable display |
| Config VO | `config-system/taxonomy_config_vo.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Config-discard bug affects `config show`** — same bug as FR-002 section 8.2: parsed config is discarded, defaults are shown instead of actual file config
   - **Impact**: `config show` displays defaults, not the user's actual configuration
   - **Fix**: Same fix as FR-002 — wire parsed config through instead of underscore-discarding

2. **`config show` re-loads config on every invocation** — no caching
   - **Impact**: 200ms overhead per call, but acceptable for a diagnostic command
   - **Note**: This is intentional for accuracy (always shows current state)

3. **Layer definitions not displayed when config is malformed** — if the config fails to parse, `config show` errors out instead of showing partial config + error context
   - **Impact**: Users can't see what part of their config is valid
   - **Fix**: Show error context with partial config display

### 8.3 What Needs to Be Added

- **Config-discard fix**: Same as FR-002
- **Partial display on error**: Show valid config sections alongside parse errors
- **Config diff**: Future: `config diff` to compare file config vs. resolved config

### 8.4 What to Keep

- **Comprehensive display** ✅ — rules, layers, CI, source patterns
- **Config source info** ✅ — file path + language
- **Rule count** ✅ — "31 total" summary
- **User-friendly colors** ✅ — severity-colored output

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli config show` displays config for all test projects
- Config source correctly shows files vs. defaults
- Pending Review: Config-discard bug fix, partial display on error

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-002 (Config) | Config loading pipeline | Config-discard bug affects accuracy | Fix in progress |
| Language detection | Language must be detected to load config | Wrong language → wrong config displayed | --language flag override |

## 10. Appendices
- `src-rust/config-system/surface_config_command.rs` — CLI config command
- `src-rust/config-system/agent_loading_orchestrator.rs` — Config loader
- `src-rust/config-system/capabilities_config_formatter.rs` — Display formatter
- `src-rust/config-system/taxonomy_config_vo.rs` — Config value objects

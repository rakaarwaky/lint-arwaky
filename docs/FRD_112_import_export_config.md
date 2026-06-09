# 📄 Feature Requirements Document (FRD)
**Feature Name:** Import/Export Configuration — `import`/`export` Subcommands
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
Defines the `import` and `export` subcommands for serializing and deserializing Lint Arwaky configuration, enabling config sharing and backup.

### 2.2 Scope
**In-Scope:** `lint-arwaky-cli export [--format yaml|json]`, `lint-arwaky-cli import <file>`, config validation on import.
**Out-of-Scope:** Config migration across versions, remote config fetching.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **export** | Serialize current config to file/stdout |
| **import** | Load and validate config from file, replace current |

## 3. Feature Overview
### 3.1 Background & Problem
Configuration was only editable via manual YAML editing. No backup or sharing mechanism existed.

### 3.2 Business Goals
- Enable config backup and restoration
- Support config sharing between team members
- Validate config on import to prevent errors

### 3.3 Target Users
- Developers configuring lint rules
- Teams standardizing lint config across projects

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to export my config to share with my team.
- **US-002:** As a developer, I want to import a config file and validate it before applying.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli export --format yaml > my-config.yaml
lint-arwaky-cli import my-config.yaml
```

### 4.3 Business Rules
- Export reads current config from `lint_arwaky.config.*.yaml`
- Import validates all required fields before replacing
- Supported formats: YAML, JSON
- Import creates backup of existing config automatically

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Export latency | < 100ms |
| NFR-002 | Import validation | < 200ms |
| NFR-003 | Config file size limit | < 1MB |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli export --format json
{ "layers": [...], "rules": {...} }

$ lint-arwaky-cli import ./team-config.yaml
 Validating configuration... ✅
 Configuration imported successfully.
 Backup saved to: lint_arwaky.config.rust.yaml.bak
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Current config exists | `export --format yaml` | YAML config written to stdout | Pending Review |
| AC-002 | Valid config file | `import valid.yaml` | Config applied, backup created | Pending Review |
| AC-003 | Invalid config file | `import invalid.yaml` | Error with validation details | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Export command | `config-system/surface_export_command.rs` | Pending Review |
| Import command | `config-system/surface_import_command.rs` | Pending Review |
| Import validator | `config-system/capabilities_config_validator.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-002 (Config Parser) | Config YAML parsing | Malformed import breaks config | Validate before apply; auto-backup |

## 10. Appendices
- `src-rust/config-system/surface_export_command.rs`
- `src-rust/config-system/surface_import_command.rs`

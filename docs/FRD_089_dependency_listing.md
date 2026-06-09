# 📄 Feature Requirements Document (FRD)
**Feature Name:** Dependency Listing (`dependencies` subcommand)
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
This document defines the dependency listing feature that enumerates all project dependencies with their versions, license information, and dependency type (direct vs transitive). It supports both Python (requirements.txt/Pipfile.lock) and Rust (Cargo.toml/Cargo.lock) projects.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli dependencies [path]` — list all dependencies with versions
- Python: parse `requirements.txt`, `Pipfile.lock`, `pyproject.toml`
- Rust: parse `Cargo.toml` and `Cargo.lock`
- Display: package name, version, dependency type (direct/transitive), license (if available)
- Filter options: `--direct`, `--transitive`, `--outdated`

**Out-of-Scope:**
- Dependency vulnerability scanning (covered in FR-085)
- License compliance auditing
- Dependency tree visualization beyond flat list

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Direct dependency** | Package explicitly listed in project manifest |
| **Transitive dependency** | Package pulled in as a dependency of a direct dependency |
| **Manifest** | Project file listing direct dependencies (Cargo.toml, pyproject.toml) |
| **Lock file** | Pinned version file (Cargo.lock, Pipfile.lock) |
| **Outdated** | Dependency with a newer version available (per registry) |

## 3. Feature Overview
### 3.1 Background & Problem
Developers often lose track of what dependencies their project uses, especially transitive ones. Without a simple "list all deps" command, teams resort to manually reading manifest files or using package-specific commands (`pip list`, `cargo tree`). A unified command simplifies dependency auditing.

### 3.2 Business Goals
- Provide a single command to view all dependencies across languages
- Distinguish direct vs transitive dependencies
- Support compliance workflows (license visibility)
- Integrate with dependency update workflows (outdated detection)

### 3.3 Target Users
- **Developers**: See what dependencies the project uses
- **Security Engineers**: Audit dependency supply chain
- **Tech Leads**: Review dependency footprint before approval

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli dependencies .` to see all project dependencies, so I can review what packages my project uses.
- **US-002:** As a developer, I want to see which dependencies are direct vs transitive, so I know which ones I explicitly chose.
- **US-003:** As a tech lead, I want to check for outdated dependencies with `--outdated`, so I can schedule upgrades.

### 4.2 Use Cases & Workflow
**Dependency Listing Pipeline:**
```
lint-arwaky-cli dependencies .
  │
  ├─► 1. Detect project language and manifest files
  │     ├── Rust: Cargo.toml + Cargo.lock
  │     └── Python: pyproject.toml / requirements.txt / Pipfile.lock
  │
  ├─► 2. Parse manifest for direct dependencies
  │
  ├─► 3. Parse lock file for all dependencies (direct + transitive)
  │
  ├─► 4. Resolve license info (from registry metadata if available)
  │
  ├─► 5. Optionally check for outdated versions (--outdated)
  │
  └─► 6. Display formatted table
```

**Example Output:**
```
$ lint-arwaky-cli dependencies .
📦 Dependency Report — /home/user/project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Language: Rust
  Dependencies: 42 total (12 direct, 30 transitive)
  ─────────────────────────────────────────────────
  Package           Version    Type        License
  ─────────────────────────────────────────────────
  serde             1.0.200    direct      MIT/Apache-2.0
  tokio             1.35.0     direct      MIT
  clap              4.5.0      direct      MIT/Apache-2.0
  syn               2.0.50     transitive  MIT/Apache-2.0
  quote             1.0.35     transitive  MIT/Apache-2.0
  ... (37 more)
  ─────────────────────────────────────────────────
  2 outdated dependencies:
    serde   1.0.200  → 1.0.210  (1 month behind)
    clap    4.5.0    → 4.6.0    (2 weeks behind)
```

### 4.3 Business Rules
- Direct dependencies from manifest; total from lock file
- `--direct` flag shows only direct dependencies
- `--transitive` flag shows only transitive dependencies
- `--outdated` checks latest version from registry (crates.io/PyPI)
- License column shows "Unknown" if not resolvable from metadata

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Parse time for 500 dependencies | < 200ms |
| NFR-002 | Manifest file detection | < 10ms |
| NFR-003 | Outdated check latency per package | < 100ms (network) |
| NFR-004 | Report generation for 500 deps | < 50ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli dependencies . --outdated
📦 Dependency Report — /home/user/project
  Total: 42 (12 direct, 30 transitive)
  ─────────────────────────────────────────────────
   serde         1.0.200  direct  🔴 outdated → 1.0.210
   tokio         1.35.0   direct  🟢 up-to-date
   clap          4.5.0    direct  🔴 outdated → 4.6.0
   syn           2.0.50   trans   🟢 up-to-date
  ─────────────────────────────────────────────────
  2 of 12 direct dependencies are outdated
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust project with Cargo.toml + Cargo.lock | `dependencies .` runs | All deps listed with version, type, license | Pending Review |
| AC-002 | Python project with pyproject.toml | `dependencies .` runs | All deps listed | Pending Review |
| AC-003 | `--direct` flag used | `dependencies . --direct` runs | Only direct deps shown | Pending Review |
| AC-004 | `--outdated` flag used | `dependencies . --outdated` runs | Outdated deps flagged with newer version | Pending Review |
| AC-005 | Project with no dependencies | `dependencies .` runs | "No dependencies found" message | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Dependencies subcommand | `cli-commands/surface_analysis_command.rs` | — | **STUB** — handler exists, body pending |
| Cargo.lock parser | `language-adapters/` | — | **NOT IMPLEMENTED** |
| Python manifest parser | `language-adapters/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- Cargo.lock parser (TOML format, extract `[package]` entries)
- Python manifest parser (pyproject.toml, requirements.txt, Pipfile.lock)
- License resolution from registry (crates.io API / PyPI JSON API)
- Outdated version check (network request to registry)
- Formatted table output

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-072 (Language Adapters) | Python/Rust file detection | Language detection may fail | Fallback to manifest-based detection |
| Cargo.lock TOML format | Format may change between Rust editions | Parse errors | Pin parser to stable TOML spec |
| PyPI/crates.io API | Network-dependent license/outdated checks | Offline environments | Cache results, offline flag |

## 10. Appendices
- `cli-commands/surface_analysis_command.rs` — `dependencies()` handler
- `language-adapters/` — Python/Rust manifest parsers
- Config keys: `dependencies.outdated_check`, `dependencies.registry_timeout`

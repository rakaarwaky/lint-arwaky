# 📄 Feature Requirements Document (FRD)
**Feature Name:** Naming Convention Checker (AES003)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026  
**Version:** v1.1

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths for 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES003 rule that enforces the `[layer]_[concept]_[suffix]` naming pattern. Every file must follow `[layer]_[concept]_[suffix].rs` pattern where the layer prefix communicates architectural role, concept communicates domain purpose, and suffix communicates file type.

### 2.2 Scope
**In-Scope:**
- `[layer]_[concept]_[suffix]` prefix-based naming pattern
- Stem extraction and regex validation
- Barrel file and entry point exemption
- Layer-specific exception list
- HIGH severity violations

**Out-of-Scope:**
- File suffix rules (AES008/AES010/AES011 — separate FRDs)
- Symbol naming inside files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES003** | Rule code for naming convention violation |
| **check_file_naming()** | Main detection method |
| **get_stem()** | Removes file extension |
| **Prefix-based pattern** | `^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$` |

## 3. Feature Overview
### 3.1 Background & Problem
Files had arbitrary names with no structural meaning — `helpers.rs`, `utils.rs`, `types.rs`. Without enforced naming, a file's domain and architectural role was not visible from its path.

### 3.2 Business Goals
- Every filename communicates domain + concept + role
- Enforce consistent prefix-based pattern across codebase
- Provide clear rename guidance

### 3.3 Target Users
- **Developers**: Guided to use correct naming pattern
- **Architects**: Standardize naming across the codebase

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a file, I want to be told if my filename doesn't follow the `[layer]_[concept]_[suffix]` pattern.

### 4.2 Use Cases & Workflow
**Validation:**
```
"capabilities_import_checker.rs" → stem "capabilities_import_checker"
  → regex ^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$ → MATCH Pending Review

"helpers_utils.rs" → stem "helpers_utils"
  → regex ^(taxonomy_|contract_|capabilities_|infrastructure_|agent_|surface_)[a-z]+_[a-z]+$ → NO MATCH (no layer prefix) Pending Review
```

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`

### 4.3 Business Rules
- Severity: HIGH
- Default pattern: `[layer]_[concept]_[suffix]` (configurable via YAML)
- Barrel files and entry points are skipped
- Layer-specific exception list honored

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | < 1% |

## 6. UI/UX Requirements
```
AES003 HIGH - src-rust/shared-common/helpers_utils.rs
  AES003 NAMING_CONVENTION: Filename does not follow the [layer]_[concept]_[suffix] pattern.
  WHY? Prefix-based naming ensures architectural consistency across 26 feature folders.
  FIX: Rename to something like taxonomy_helpers_utils.rs.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File without layer prefix | `check_file_naming()` runs | AES003 HIGH flagged | Pending Review |
| AC-002 | File with valid [layer]_[concept]_[suffix] name | `check_file_naming()` runs | No violation | Pending Review |
| AC-003 | Barrel file (mod.rs) | `check_file_naming()` runs | Skipped | Pending Review |
| AC-004 | Entry point (main.rs) | `check_file_naming()` runs | Skipped | Pending Review |

## 8. Empirical Findings (Code Audit)

N/A — Pending review after vertical slicing refactoring.

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | Pattern from config | Missing config = default [layer]_[concept]_[suffix] | Built-in default |
| regex crate | Naming regex validation | Complex patterns may fail | Simple prefix pattern only |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:65` — `check_file_naming()`
- `lint_arwaky.config.rust.yaml` — Global `Naming Convention` rule

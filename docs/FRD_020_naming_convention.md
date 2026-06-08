# 📄 Feature Requirements Document (FRD)
**Feature Name:** Naming Convention Checker (AES003)
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
This document defines the AES003 rule that enforces strict 3-word snake_case naming convention. Every file must follow `word1_word2_word3.ext` pattern to communicate domain, concept, and architectural role through its filename.

### 2.2 Scope
**In-Scope:**
- 3-word underscore-separated naming pattern
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
| **3-word pattern** | `^[a-z0-9]+(_[a-z0-9]+){2}$` |

## 3. Feature Overview
### 3.1 Background & Problem
Files had arbitrary names with no structural meaning — `helpers.rs`, `utils.rs`, `types.rs`. Without enforced naming, a file's domain and architectural role was not visible from its path.

### 3.2 Business Goals
- Every filename communicates domain + concept + role
- Enforce consistent 3-word pattern across codebase
- Provide clear rename guidance

### 3.3 Target Users
- **Developers**: Guided to use correct naming pattern
- **Architects**: Standardize naming across the codebase

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a file, I want to be told if my filename doesn't follow the 3-word pattern.

### 4.2 Use Cases & Workflow
**Validation:**
```
"architecture_import_checker.rs" → stem "architecture_import_checker"
  → regex ^[a-z0-9]+(_[a-z0-9]+){2}$ → MATCH ✅

"project_helpers.rs" → stem "project_helpers"
  → regex ^[a-z0-9]+(_[a-z0-9]+){2}$ → NO MATCH (2 words) ❌
```

**Exceptions:** `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`

### 4.3 Business Rules
- Severity: HIGH
- Default word count: 3 (configurable via YAML)
- Barrel files and entry points are skipped
- Layer-specific exception list honored

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | < 1% |

## 6. UI/UX Requirements
```
AES003 HIGH - src-rust/capabilities/project_helpers.rs
  AES003 NAMING_CONVENTION: Filename does not follow the 3-word underscore pattern.
  WHY? Strict three-word names ensure architectural consistency.
  FIX: Rename to something like project_helper_utils.rs.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with 2-word name | `check_file_naming()` runs | AES003 HIGH flagged | ✅ |
| AC-002 | File with 3-word name | `check_file_naming()` runs | No violation | ✅ |
| AC-003 | Barrel file (mod.rs) | `check_file_naming()` runs | Skipped | ✅ |
| AC-004 | Entry point (main.rs) | `check_file_naming()` runs | Skipped | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | Word count from config | Missing config = default 3 | Built-in default |
| regex crate | Naming regex validation | Complex patterns may fail | Simple 3-word only |

## 9. Appendices
- `src-rust/capabilities/architecture_naming_checker.rs:65` — `check_file_naming()`
- `lint_arwaky.config.rust.yaml` — Global `Naming Convention` rule

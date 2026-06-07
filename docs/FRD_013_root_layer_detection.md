# 📄 Feature Requirements Document (FRD)
**Feature Name:** Root Layer Detection / Strict Suffix Policy (AES010)  
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
This document defines the AES010 rule that enforces strict suffix policy. Layers with `suffix_policy: "strict"` require every file to use an allowed suffix from the layer's definition. Files without a valid suffix are flagged.

### 2.2 Scope
**In-Scope:**
- Extracting suffix from filename stem
- Validating against `allowed_suffix.values` per layer
- Skipping barrel files and entry points
- HIGH severity violations

**Out-of-Scope:**
- Forbidden suffix rules (AES011 — separate FRD)
- Contract-specific suffix rules (AES008 — separate FRD)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES010** | Rule code for root layer suffix policy violation |
| **check_domain_suffixes()** | Main detection method |
| **get_stem()** | Removes file extension |
| **get_suffix()** | Extracts last underscore-delimited word |
| **suffix_policy** | Config field: "strict" or "flexible" |

## 3. Feature Overview
### 3.1 Background & Problem
Files in capabilities, infrastructure, and other layers could have arbitrary names — `helpers.rs`, `utils.rs`, `types.rs` — with no indication of their architectural role. Without enforced suffixes, a file's layer identity was not visible from its name.

### 3.2 Business Goals
- Every filename communicates its architectural role via suffix
- Enforce layer-specific allowed suffix lists
- Provide clear guidance on which suffix to use

### 3.3 Target Users
- **Developers**: Guided to use correct suffix for their layer
- **Architects**: Standardize naming across the codebase

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a file to capabilities/, I want to be told which suffix to use, so my file follows the naming convention.

### 4.2 Use Cases & Workflow
**Suffix Extraction:**
```
"architecture_import_checker.rs"
  → stem: "architecture_import_checker"
  → suffix: "checker"

"project_helpers.rs"
  → stem: "project_helpers"
  → suffix: "helpers"
```

**Validation:**
```
File in capabilities/project_helpers.rs
  suffix = "helpers"

Look up capabilities layer definition:
  allowed_suffix = ["analyzer", "checker", "processor", ...]
  suffix_policy = "strict"

"helpers" NOT in allowed list
  → AES010 HIGH violation
```

### 4.3 Business Rules
- Severity: HIGH
- Skip barrel files (`mod.rs`), entry points, and exception files
- If layer is "contract" → emit AES008 instead of AES010
- Configurable via YAML `suffix_policy` and `allowed_suffix`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES010 HIGH - src-rust/capabilities/project_helpers.rs
  AES010 SUFFIX_MISMATCH: File is missing a required strict suffix.
  WHY? Strict suffixes ensure every component has a clear role.
  FIX: Add one of: analyzer, checker, processor, evaluator, ...
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File in capabilities/ has no valid suffix | `check_domain_suffixes()` runs | AES010 HIGH flagged | ✅ |
| AC-002 | Barrel file (mod.rs) | `check_domain_suffixes()` runs | Skipped | ✅ |
| AC-003 | Entry point file (main.rs) | `check_domain_suffixes()` runs | Skipped | ✅ |
| AC-004 | File has valid suffix for its layer | `check_domain_suffixes()` runs | No violation | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `suffix_policy` and `allowed_suffix` | Missing config = no rules | Built-in defaults |

## 9. Appendices
- `src-rust/capabilities/architecture_naming_checker.rs:124` — `check_domain_suffixes()`
- `src-rust/taxonomy/layer_definition_vo.rs` — `suffix_policy`, `allowed_suffix`
- `docs/RULES_AES.md` — Allowed suffix lists per layer
- `docs/ARCHITECTURE.md` — Layer specifications

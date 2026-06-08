# 📄 Feature Requirements Document (FRD)
**Feature Name:** Mandatory Struct/Trait Definition Checker (AES009)
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
This document defines the AES009 rule that requires every file to define at least one struct, enum, or trait. Files without a type definition violate the principle that each file encapsulates a distinct data type.

### 2.2 Scope
**In-Scope:**
- Detecting struct/enum/trait/class definitions per file
- Rust: `struct`, `enum`, `trait`, `pub struct`, `pub enum`, `pub trait`
- Python: `class`
- JS/TS: `class`, `export class`, `export default class`
- Skipping barrel files, entry points, and `_constant` files
- HIGH severity violations

**Out-of-Scope:**
- Naming rules (AES003 — separate FRD)
- Content validation beyond type detection

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES009** | Rule code for missing struct/trait definition |
| **check_mandatory_class_definition()** | Main detection method |
| **file_has_class_definition()** | Content scan for struct/enum/trait/class keywords |

## 3. Feature Overview
### 3.1 Background & Problem
Files could exist without defining any struct, enum, or trait — containing only loose functions, constants, or side effects. This violated the principle that each file should encapsulate a coherent type.

### 3.2 Business Goals
- Every file must define at least one type
- Prevent loose functions without struct/trait encapsulation
- Skip barrel files and constant-only files

### 3.3 Target Users
- **Developers**: Reminded to wrap functions in structs/traits
- **Architects**: Enforce type-oriented encapsulation

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned if my file has no struct or trait definition, so I encapsulate my logic properly.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: capabilities/loose_functions.rs
  Content: "fn do_something() { ... }"
  → No "struct", "enum", "trait", or "class" found
  → AES009 HIGH violation

File: capabilities/my_checker.rs
  Content: "pub struct MyChecker; impl Checker for MyChecker { ... }"
  → "pub struct" found
  → No violation
```

**Exceptions:** `__init__.py`, `main.py`, `mod.rs`, `lib.rs`, `_constant` files

### 4.3 Business Rules
- Severity: HIGH
- Skip barrel files, entry points, and `_constant` files
- Configurable via YAML `mandatory_class_definition` flag
- Root layer has `mandatory_class_definition: false`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES009 HIGH - src-rust/capabilities/loose_functions.rs
  AES009 MANDATORY_CLASS_DEFINITION: File is missing a struct, enum, or trait definition.
  WHY? Encapsulation in structs/traits is required for proper modularization.
  FIX: Group functions into a struct or implement a Trait.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with only functions (no struct/trait) | `check_mandatory_class_definition()` runs | AES009 HIGH flagged | ✅ |
| AC-002 | File with struct definition | `check_mandatory_class_definition()` runs | No violation | ✅ |
| AC-003 | Barrel file (mod.rs) | `check_mandatory_class_definition()` runs | Skipped | ✅ |
| AC-004 | _constant file | `check_mandatory_class_definition()` runs | Skipped (AES033 precedence) | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `mandatory_class_definition` flag | Missing flag = no enforcement | Built-in default |
| Content regex | Rust/Python/JS keyword detection | False negative on complex generics | Conservative regex |

## 9. Appendices
- `src-rust/capabilities/architecture_metric_checker.rs:188` — `check_mandatory_class_definition()`
- `lint_arwaky.config.rust.yaml` — Global `Mandatory Struct or Trait Definition` rule

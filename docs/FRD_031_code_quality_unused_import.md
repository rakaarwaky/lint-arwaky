# Feature Requirements Document (FRD)
**Feature Name:** Unused Import Detector (AES015)
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
This document defines the AES015 rule that detects **unused imports** in source files. An import is considered unused when its name never appears in the file outside the import statement itself. The rule is implemented in `check_unused_imports()`.

### 2.2 Scope
**In-Scope:**
- Rust `use` statements where the imported name is never referenced
- Python `import` / `from … import` statements
- MEDIUM severity (informational, not blocking)
- Any file in any layer

**Out-of-Scope:**
- Auto-fixing violations
- Standard library imports (std::, core::, alloc::)
- Imports used only in `use` statements themselves

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES015** | Rule code for unused import violation |
| **check_unused_imports()** | Detection method in `LintCheckingCoordinator` |

## 3. Feature Overview
### 3.1 Background & Problem
Unused imports clutter the codebase, create false dependencies, and confuse developers reading the code. They also increase compilation time and make dependency analysis harder.

### 3.2 Business Goals
- Keep import lists minimal and accurate
- Reduce noise in code reviews
- Maintain clean dependency graphs

### 3.3 Target Users
- **Developers**: Get notified when an import is unnecessary
- **Reviewers**: Less noise from dead imports in pull requests

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when a Rust `use` statement references a symbol never used in the file.
- **US-002:** As a developer, I want to be warned when a Python `import` brings in a module never referenced.

### 4.2 Detection Pipeline
```
File: src-rust/layer-rules/capabilities_import_checker.rs

1. Find all `use X;` lines
2. For each, extract the symbol name
3. Skip std::, core::, alloc:: imports
4. Check if the name appears anywhere else in the file
5. If not → AES015 MEDIUM
```

### 4.3 Business Rules
- Severity: MEDIUM
- Only Rust `use` statements currently detected
- Standard library imports (std::, core::, alloc::) are exempt
- Multi-import `use foo::{Bar, Baz};` skipped (not supported)
- Imports with `as _` (underscore) are skipped

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 10ms |
| NFR-002 | False positive rate | 0% for actually unused imports |
| NFR-003 | False negative rate | < 10% for multi-import patterns |

## 6. UI/UX Requirements
```
AES015 MEDIUM - src-rust/layer-rules/capabilities_import_checker.rs:10
  AES015 UNUSED_IMPORT: 'some_module::SomeStruct' imported but never used.
  WHY? Unused imports create false dependencies and confuse readers.
  FIX: Remove the unused import statement.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust file with unused `use foo::Bar;` | `check_unused_imports()` runs | AES015 MEDIUM flagged | Pending Review |
| AC-002 | Rust file with std import | Checker runs | Skipped (stdlib exempt) | Pending Review |
| AC-003 | Rust file with used import | Checker runs | No AES015 | Pending Review |
| AC-004 | Python file with unused `import os` | Checker runs | AES015 flagged | Pending Review Python not supported |
| AC-005 | Rust file with `use foo::{A, B};` | Checker runs | AES015 flagged for unused | Pending Review Multi-import skipped |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/pipeline-jobs/agent_checking_coordinator.rs:254-300`
- **Status**: **PARTIALLY IMPLEMENTED** — Rust `use` only
- Invoked from `run_all_checks()` line 58

### 8.2 Bugs/Gaps Found

1. **Multi-import not supported** — `use foo::{Bar, Baz};` is skipped because `name.starts_with('{')` causes `continue`
2. **Python imports not detected** — `import X`, `from X import Y` never checked
3. **Fragile name extraction** — `split("as ").last()` in the name extraction may break if `as` appears in the module path

### 8.3 What Needs to Be Added
- Multi-import parsing for `use foo::{Bar, Baz};`
- Python `import` / `from … import` detection
- Robust name extraction

### 8.4 What to Keep
- Single-import Rust `use` detection Pending Review
- Standard library exemption Pending Review
- Case-insensitive name matching Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Line scanning) | Regex-free name extraction | Fragile parsing | Improve with import-specific parsing |

## 10. Appendices
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:254` — `check_unused_imports()`

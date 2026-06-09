# 📄 Feature Requirements Document (FRD)
**Feature Name:** Project-Wide Symbol Rename — Scope-Aware Refactoring
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
Defines the project-wide symbol rename feature that renames symbols across all files with scope awareness, ensuring correct renaming even when the same name appears in different contexts.

### 2.2 Scope
**In-Scope:** Symbol rename across all project files, scope filtering (only rename in matching scope context), dry-run preview, rename logging.
**Out-of-Scope:** Type-aware rename (rename only vars of specific type), semantic rename (detect overrides).

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Symbol** | Function name, variable name, struct name, class name |
| **Scope-aware** | Rename only when symbol is within a specific function/class scope |

## 3. Feature Overview
### 3.1 Background & Problem
The existing `NamingRenamerProcessor` renames all occurrences of a symbol project-wide, potentially renaming unrelated symbols with the same name in different scopes.

### 3.2 Business Goals
- Provide scope-filtered renaming to prevent false positives
- Support dry-run preview before applying
- Maintain rename audit log

### 3.3 Target Users
- Developers refactoring symbol names
- AI agents performing automated fixes

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to rename `old_name` to `new_name` only within `check_imports` scope, so other uses of `old_name` are unaffected.

### 4.2 Use Cases & Workflow
```
Input: rename "handler" → "request_handler" in scope "process_request"
Output:
  File: src/handler.rs:25 → renamed in process_request()
  File: src/router.rs:42  → NOT renamed (different scope)
  3 occurrences renamed, 2 skipped (different scopes)
```

### 4.3 Business Rules
- If no scope specified, renames project-wide (current behavior)
- Scope matching is case-sensitive
- Skips comments, string literals, template literals
- Reports count: total, renamed, skipped-per-scope

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Rename (1000 files) | < 2s |
| NFR-002 | Scope detection per file | < 5ms |

## 6. UI/UX Requirements
```
 Renaming "handler" → "request_handler" in scope "process_request":
 ┌────────────┬────────┬──────────┬──────────┐
 │ File        │ Total  │ Renamed │ Skipped  │
 ├────────────┼────────┼──────────┼──────────┤
 │ handler.rs  │      3 │        3 │        0 │
 │ router.rs   │      2 │        0 │        2 │
 └────────────┴────────┴──────────┴──────────┘
 Summary: 3 renamed, 2 skipped (different scope)
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Symbol in scope | Rename with scope filter | Only in-scope occurrences renamed | Pending Review |
| AC-002 | Symbol in comments | Rename project-wide | Comments not modified | Pending Review |
| AC-003 | Dry-run mode | Rename --dry-run | Count shown, files not modified | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Scope-aware renamer | `semantic-analysis/capabilities_scope_renamer.rs` | Pending Review |
| Rename CLI command | `semantic-analysis/surface_rename_command.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Symbol detection in source | Regex may match false symbols | Word-boundary matching |
| FR-120 (Scope Detection) | Scope boundary identification | Incorrect scope boundaries cause wrong renames | Dry-run preview before apply |

## 10. Appendices
- `src-rust/semantic-analysis/capabilities_scope_renamer.rs`

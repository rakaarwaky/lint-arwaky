# 📄 Feature Requirements Document (FRD)
**Feature Name:** Compare Violation Diff — `diff` Subcommand
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
Defines the `diff` subcommand that compares violation differences between two project paths, enabling delta analysis of code quality changes.

### 2.2 Scope
**In-Scope:** `lint-arwaky-cli diff <path-a> <path-b>`, violation comparison by AES code, severity, file count.
**Out-of-Scope:** Line-level diff, git-aware diff, auto-fix generation.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Diff result** | Struct with added/removed/changed violations between two paths |

## 3. Feature Overview
### 3.1 Background & Problem
Teams had no tooling to compare code quality across branches or releases, making it hard to track quality regression.

### 3.2 Business Goals
- Enable before/after quality comparison
- Support CI gate decisions based on violation delta
- Highlight new violations introduced between versions

### 3.3 Target Users
- Developers comparing branches
- CI pipelines enforcing quality gates

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to compare violations between `main` and `feature-branch`, so I know if I introduced new issues.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli diff ./release-v1.0/ ./release-v1.1/
  │
  ├─► Scan path-a → violation set A
  ├─► Scan path-b → violation set B
  ├─► Compare: added (B\A), removed (A\B), unchanged
  └─► Report: "+5 violations, -3 violations, 42 unchanged"
```

### 4.3 Business Rules
- Comparison keyed by (file_path, aes_code, line_number)
- Same violation in both paths = unchanged
- CI mode: exit code 1 if added violations > threshold

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Diff for 500 violation files | < 5s |
| NFR-002 | Output formatting | Plain text + JSON |

## 6. UI/UX Requirements
```
 Comparing violations between ./v1.0 and ./v1.1...
 ┌──────────────────────┬───────┬───────┬──────────┐
 │ AES Code             │ Added │ Removed │ Changed │
 ├──────────────────────┼───────┼───────┼──────────┤
 │ AES001 (Import)      │     2 │     1 │        0 │
 │ AES003 (Naming)      │     3 │     2 │        1 │
 └──────────────────────┴───────┴───────┴──────────┘
 Summary: +5 violations, -3 violations
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Two project paths | `diff path-a path-b` | Violation delta computed | Pending Review |
| AC-002 | Identical projects | `diff a a` | Zero added/removed | Pending Review |
| AC-003 | One path invalid | `diff valid bogus` | Error with path hint | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Diff CLI command | `cli-commands/surface_diff_command.rs` | Pending Review |
| Diff processor | `cli-commands/capabilities_diff_processor.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Pipeline) | Both paths scanned through pipeline | Large projects slow | Add progress indicators |

## 10. Appendices
- `src-rust/cli-commands/surface_diff_command.rs`

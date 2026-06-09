# 📄 Feature Requirements Document (FRD)
**Feature Name:** Cyclomatic Complexity Analysis (`complexity` subcommand)
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
This document defines the cyclomatic complexity analysis feature that computes per-function complexity scores using Radon (Python) or a Rust AST-based analyzer. Results are graded A–F and highlighted in the lint report.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli complexity [path]` — compute cyclomatic complexity
- Python: Radon integration (`radon cc` command)
- Rust: Built-in AST-based complexity calculation (match arms, if/else, loops, etc.)
- Complexity grading: A (1–5), B (6–10), C (11–20), D (21–30), E (31–40), F (41+)
- Per-function reporting with file:line references

**Out-of-Scope:**
- Cognitive complexity scoring
- Halstead metrics
- Maintainability index (separate feature)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Cyclomatic complexity** | M = E − N + 2P — measures number of linearly independent paths through source code |
| **Radon** | Python tool for computing code metrics (radon.readthedocs.io) |
| **Grade** | Letter grade A–F assigned based on complexity score range |
| **Match arm** | Rust `match` arm — counts as one branching path |

## 3. Feature Overview
### 3.1 Background & Problem
High cyclomatic complexity indicates code that is difficult to test, maintain, and understand. Developers have no built-in way to measure complexity per function. Manual review is subjective and inconsistent across teams.

### 3.2 Business Goals
- Identify overly complex functions that need refactoring
- Enforce complexity thresholds in CI/CD
- Provide objective, automated complexity grading
- Track complexity trends over time

### 3.3 Target Users
- **Developers**: Identify complex functions for refactoring
- **Tech Leads**: Enforce code quality standards
- **Code Reviewers**: Get automated complexity data in PRs

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli complexity .` to see which functions have high complexity, so I can prioritize refactoring.
- **US-002:** As a tech lead, I want to flag any function with grade E or F (complexity > 30) as a lint violation, so the team maintains readable code.
- **US-003:** As a developer, I want to see the line number and file for each complex function, so I can navigate directly to the code.

### 4.2 Use Cases & Workflow
**Complexity Analysis Pipeline:**
```
lint-arwaky-cli complexity .
  │
  ├─► 1. Detect project language
  │     ├── Python → run radon cc <path> -s --json
  │     └── Rust   → walk AST, compute M = E − N + 2P per function
  │
  ├─► 2. Parse results
  │     ├── Radon: extract function name, complexity score, line number
  │     └── Rust AST: extract fn name, match/if/loop branches, line range
  │
  ├─► 3. Assign grade
  │     ├── A (1–5), B (6–10), C (11–20), D (21–30), E (31–40), F (41+)
  │     └── Flag E and F as violations
  │
  └─► 4. Report
        "src/parser.rs:42  parse_config()    C=14 (Grade C)"
        "src/main.rs:105   handle_request()  C=38 (Grade E) ⚠️"
```

**Example Output:**
```
$ lint-arwaky-cli complexity .
📊 Cyclomatic Complexity Analysis — /home/user/project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Function                              File:Line    Score  Grade
  ─────────────────────────────────────────────────────────────────
  handle_request()                      src/main.rs:105  38   E ⚠️
  parse_config()                        src/config.rs:42  14   C
  validate_input()                      src/validate.rs:8  3    A
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Summary: 3 functions analyzed
  Average complexity: 18.3 (Grade C)
  Functions flagged:  1 (Grade E)
```

### 4.3 Business Rules
- Thresholds: Grade E or F → lint violation (configurable)
- Radon flags: `--show-complexity` and `--average` used for summary
- Rust analyzer: match arms, if/else, while/for loops, and logical operators count as branches
- Results cached per file for incremental analysis

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Analysis time for 1000 functions | < 5s |
| NFR-002 | Per-function parse time (Rust AST) | < 2ms |
| NFR-003 | Radon output parsing | < 50ms |
| NFR-004 | Grade classification accuracy | 100% |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli complexity src/
📊 Complexity Report — src/
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  src/parser.rs
    ├── parse_config()          line 42    C=14   Grade C
    ├── tokenize()              line 88    C=6    Grade B
    └── build_ast()             line 120   C=22   Grade D

  src/analyzer.rs
    └── analyze_node()          line 55    C=19   Grade C
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Complexity grade distribution:
    A (1–5):    0 functions
    B (6–10):   1 function
    C (11–20):  2 functions
    D (21–30):  1 function
    E (31–40):  0 functions
    F (41+):    0 functions
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Python project with `radon cc` results | `complexity .` runs | Per-function complexity scores displayed with grade | Pending Review |
| AC-002 | Rust project with complex match blocks | `complexity .` runs | AST-based complexity computed, match arms counted as branches | Pending Review |
| AC-003 | Function with complexity 38 (Grade E) | Analysis runs | Violation flagged, warning shown | Pending Review |
| AC-004 | All functions are Grade A or B | Analysis runs | "No complex functions found" message | Pending Review |
| AC-005 | Radon not installed | `complexity .` on Python project | Friendly error with install hint | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Complexity subcommand | `code-analysis/` | — | **NOT IMPLEMENTED** |
| Rust AST complexity analyzer | `code-analysis/` | — | **NOT IMPLEMENTED** |
| Radon adapter | `language-adapters/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `handle_complexity()` handler in `surface_analysis_command.rs`
- Rust AST complexity visitor (count branches per function)
- Radon JSON output parser
- Grade classification logic with configurable thresholds
- Complexity report formatter

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-076 (Python Adapter for metrics) | Radon execution via language adapter | Radon may not be installed | Graceful fallback + install hint |
| Radon CLI | External Python tool | API changes between Radon versions | Pin version in documentation |
| Rust syn crate | AST parsing for Rust complexity | Crate version updates | Use stable syn API subset |

## 10. Appendices
- `docs/RULES_RADON.md` — Radon complexity grade definitions
- Radon documentation: https://radon.readthedocs.io/
- Cyclomatic complexity: https://en.wikipedia.org/wiki/Cyclomatic_complexity

# 📄 Feature Requirements Document (FRD)
**Feature Name:** Text Report Format (Human-Readable)
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
This document defines the text (human-readable) report format for lint violations. This is the default output format, rendering violations to stdout with color-coded severity, file:line references, and violation descriptions.

### 2.2 Scope
**In-Scope:**
- Default `--format text` output (implied when no format flag given)
- Color-coded severity: CRITICAL (red), HIGH (yellow), MEDIUM (blue), LOW (gray)
- File:line prefix for each violation
- Violation code, description, and suggested fix
- Summary section with total counts and score
- Header with project path and lint timestamp

**Out-of-Scope:**
- Machine-readable formats (FR-096, FR-097, FR-098)
- HTML report generation
- Custom color schemes via config

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **ANSI color** | Terminal escape codes for colored output |
| **Severity** | CRITICAL (score 0), HIGH (score -5), MEDIUM (score -3), LOW (score -1) |
| **Violation code** | Rule identifier (e.g., AES001, AES014) |
| **Suggested fix** | Brief remediation advice shown below each violation |

## 3. Feature Overview
### 3.1 Background & Problem
The current report output is functional but lacks formatting, colors, and structure. Developers need a clear, scannable report that highlights the most severe issues first and provides enough context to fix them quickly.

### 3.2 Business Goals
- Provide clear, scannable violation output
- Use color coding to draw attention to severity
- Group violations by file for easier navigation
- Include summary statistics (total, score, distribution)

### 3.3 Target Users
- **Developers**: Read and act on lint output in terminal
- **CI/CD Pipelines**: stdout/stderr capture for log inspection

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want violations grouped by file with line numbers, so I can navigate to each issue quickly.
- **US-002:** As a developer, I want severity color-coded (red for CRITICAL), so I can prioritize fixing the most important issues first.
- **US-003:** As a developer, I want to see a violation code and description, so I understand what rule was broken and how to fix it.

### 4.2 Use Cases & Workflow
**Text Report Generation:**
```
ReportFormatterProcessor::format_text(violations, score)
  │
  ├─► 1. Print header: "Lint Arwaky Report — <project> — <date>"
  │
  ├─► 2. Group violations by file path
  │
  ├─► 3. For each file:
  │     ├── File header: "src/main.rs"
  │     ├── Each violation:
  │     │   ├── Line number + column
  │     │   ├── Severity badge (colored)
  │     │   ├── Violation code
  │     │   ├── Description
  │     │   └── Suggested fix (indented, gray)
  │     └── Empty line between files
  │
  ├─► 4. Print summary section
  │     ├── Total violations by severity
  │     └── Score
  │
  └─► 5. Print final score line with color
```

**Example Output:**
```
$ lint-arwaky-cli check .
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Lint Arwaky Report
  Project:  /home/user/project
  Date:     2026-06-09T10:00:00Z
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  src/main.rs:42:5
    🔴 CRITICAL  AES001  Layer violation
    Statement: Layer 'capabilities' cannot import from 'infrastructure'.
    FIX: Use ports/protocols from the contract layer instead.

  src/main.rs:88:12
    🔴 CRITICAL  AES014  Bypass comment violation
    Statement: Unwrap detected: line 88.
    FIX: Use safe error propagation (?, expect, match).

  src/utils.rs:15:3
    🟡 HIGH      AES003  Naming convention
    Statement: Filename does not follow [layer]_[concept]_[suffix].rs pattern.
    FIX: Rename the file to match the 3-word pattern.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Summary
  ─────────────────────────────────────────────────
  CRITICAL:  2
  HIGH:      1
  MEDIUM:    0
  LOW:       0
  ─────────────────────────────────────────────────
  Score: 72.0 / 100
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 4.3 Business Rules
- Default output format (no `--format` flag needed)
- ANSI colors: CRITICAL=red (31), HIGH=yellow (33), MEDIUM=blue (34), LOW=gray (90)
- Violations sorted by severity (CRITICAL first), then by file path
- Score displayed as "N / 100"
- Non-zero exit code if CRITICAL violations exist

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Report generation for 1000 violations | < 50ms |
| NFR-002 | Color rendering in all major terminals | Compatible with xterm, gnome-terminal, iTerm2, Windows Terminal |
| NFR-003 | No color if stdout is not a TTY | Auto-detection |
| NFR-004 | Line length per violation | < 120 chars (wrapped if needed) |

## 6. UI/UX Requirements
See example output in Section 4.2. The report must be scannable within 3 seconds — severity colors and file grouping enable fast visual parsing.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | 5 violations across 2 files | Text report generated | Violations grouped by file, severity colored | Pending Review |
| AC-002 | CRITICAL violation in output | Report rendered | Red color applied, first in list | Pending Review |
| AC-003 | 0 violations (perfect score) | `check .` runs | "No violations found" message, score 100 | Pending Review |
| AC-004 | stdout piped to file | Report rendered | No ANSI color codes in file | Pending Review |
| AC-005 | 1000+ violations | Report generated | All violations listed, summary accurate | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Report formatter | `output-report/capabilities_reporting_formatter.rs` | — | **FULLY IMPLEMENTED** — `format_text()` |
| Score computation | `output-report/taxonomy_score_constant.rs` | — | **FULLY IMPLEMENTED** |
| Color support | `output-report/` | — | **PARTIALLY IMPLEMENTED** — basic color support |

### 8.2 What Needs to Be Added
- Enhanced color coding with severity-appropriate colors
- File grouping logic
- Summary section with severity distribution
- TTY detection for color suppression
- Violation wrapping for long lines

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Violation list from lint engine | Engine must return structured violations | Use `LintResult` taxonomy type |
| ANSI terminal | Color codes | Some terminals don't support 256 colors | Fallback to basic 16 colors |

## 10. Appendices
- `output-report/capabilities_reporting_formatter.rs` — Text formatter implementation
- `output-report/taxonomy_result_vo.rs` — `LintResult` value object
- `output-report/taxonomy_score_constant.rs` — `compute_score()` function

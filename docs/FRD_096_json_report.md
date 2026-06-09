# 📄 Feature Requirements Document (FRD)
**Feature Name:** JSON Report Format (Machine-Readable)
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
This document defines the JSON report format for lint violations. This machine-readable format enables integration with IDEs, CI systems, and custom tooling that consume structured JSON data.

### 2.2 Scope
**In-Scope:**
- `--format json` flag for lint commands
- Valid JSON output (parseable with any JSON library)
- Structured violation objects with severity, code, file, line, column, message, and fix
- Summary object with totals and score
- Schema version field for forward compatibility

**Out-of-Scope:**
- Streaming JSON output (single JSON object)
- JSON pretty-print vs compact (configurable via `--pretty`)
- JSON Schema file generation

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **JSON** | JavaScript Object Notation — RFC 7159 |
| **Violation object** | Single JSON object representing one lint finding |
| **Schema version** | Integer field for format versioning (current: 1) |
| **Pretty-print** | Formatted JSON with indentation |

## 3. Feature Overview
### 3.1 Background & Problem
The text report is human-readable but difficult for tools to parse. IDEs, CI dashboards, and custom automation need structured data. Without a JSON output, every tool must write fragile text scrapers.

### 3.2 Business Goals
- Enable IDE integration (VS Code, JetBrains, Vim/Neovim)
- Enable CI dashboard ingestion
- Provide stable, versioned JSON schema
- Support automation and scripting

### 3.3 Target Users
- **IDE Plugin Developers**: Consume lint results for inline annotations
- **CI/CD Engineers**: Parse results for dashboards and quality gates
- **Scripting/Automation**: Process results in shell or Python scripts

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an IDE plugin developer, I want `--format json` to produce parseable violation data, so I can display inline annotations in the editor.
- **US-002:** As a CI engineer, I want a structured summary with score and totals, so I can implement quality gates.
- **US-003:** As a script writer, I want `jq`-compatible JSON output, so I can filter and transform results easily.

### 4.2 Use Cases & Workflow
**JSON Report Generation:**
```
lint-arwaky-cli check . --format json
  │
  ├─► 1. Collect all violations from lint engine
  │
  ├─► 2. Serialize to JSON structure
  │     ├── schema_version: 1
  │     ├── summary: { total, critical, high, medium, low, score }
  │     └── violations: [ { file, line, column, severity, code, message, fix } ]
  │
  └─► 3. Print JSON to stdout
```

**Example Output (JSON):**
```json
{
  "schema_version": 1,
  "project": "/home/user/project",
  "timestamp": "2026-06-09T10:00:00Z",
  "summary": {
    "total": 3,
    "by_severity": {
      "CRITICAL": 2,
      "HIGH": 1,
      "MEDIUM": 0,
      "LOW": 0
    },
    "score": 72.0
  },
  "violations": [
    {
      "file": "src/main.rs",
      "line": 42,
      "column": 5,
      "severity": "CRITICAL",
      "code": "AES001",
      "message": "Layer 'capabilities' cannot import from 'infrastructure'.",
      "fix": "Use ports/protocols from the contract layer instead."
    },
    {
      "file": "src/main.rs",
      "line": 88,
      "column": 12,
      "severity": "CRITICAL",
      "code": "AES014",
      "message": "Unwrap detected: line 88.",
      "fix": "Use safe error propagation (?, expect, match)."
    },
    {
      "file": "src/utils.rs",
      "line": 15,
      "column": 3,
      "severity": "HIGH",
      "code": "AES003",
      "message": "Filename does not follow [layer]_[concept]_[suffix].rs pattern.",
      "fix": "Rename the file to match the 3-word pattern."
    }
  ]
}
```

### 4.3 Business Rules
- JSON output is a single valid JSON object (not JSON-lines)
- `schema_version` increments on breaking changes
- All string fields are UTF-8
- `score` is a float (0.0–100.0)
- Empty violations array when no findings
- `--pretty` flag enables 2-space indentation (default: compact)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Serialization time for 1000 violations | < 50ms |
| NFR-002 | Output size for 1000 violations | < 500 KB |
| NFR-003 | Encoding | UTF-8 only |
| NFR-004 | Parseable by any JSON library | Guaranteed |

## 6. UI/UX Requirements
The JSON format is consumed by machines, not humans. However, the `--pretty` flag provides formatted output for debugging:

```
$ lint-arwaky-cli check . --format json --pretty | jq '.summary'
{
  "total": 3,
  "by_severity": {
    "CRITICAL": 2,
    "HIGH": 1,
    "MEDIUM": 0,
    "LOW": 0
  },
  "score": 72.0
}
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `--format json` flag used | Lint command runs | Valid JSON output to stdout | Pending Review |
| AC-002 | 3 violations found | JSON generated | All 3 violations in `violations` array | Pending Review |
| AC-003 | `jq .summary` on output | JSON parsed | Summary object with totals and score | Pending Review |
| AC-004 | No violations found | JSON generated | Empty `violations` array, summary.score = 100 | Pending Review |
| AC-005 | `--pretty` flag | JSON generated | 2-space indented JSON | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| JSON formatter | `output-report/` | — | **NOT IMPLEMENTED** |
| Serialization model | `output-report/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `format_json()` in report formatter
- `LintReportJson` struct for serialization
- `--format json` CLI flag parsing
- `--pretty` flag support
- Serde-based serialization

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Violation list from lint engine | Engine must return structured violations | Use `LintResult` taxonomy type |
| serde/serde_json | JSON serialization crate | Crate version updates | Pin to stable version |

## 10. Appendices
- `output-report/capabilities_reporting_formatter.rs` — `format_json()` method
- `output-report/taxonomy_result_vo.rs` — `LintResult` value object
- `output-report/taxonomy_report_json_vo.rs` — JSON report model

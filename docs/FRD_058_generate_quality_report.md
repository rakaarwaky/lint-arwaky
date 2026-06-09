# 📄 Feature Requirements Document (FRD)
**Feature Name:** Generate Quality Report (`report [path]`)  
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
This document defines the quality report generation CLI command `report [path] --output-format <format>`. It generates structured quality reports from the last check/scan results in text, JSON, SARIF 2.1.0, and JUnit XML formats. Enables integration with CI/CD platforms, code review tools, and GitHub Code Scanning.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli report <path> --output-format text` — human-readable text report
- `lint-arwaky-cli report <path> --output-format json` — JSON report
- `lint-arwaky-cli report <path> --output-format sarif` — SARIF 2.1.0 report
- `lint-arwaky-cli report <path> --output-format junit` — JUnit XML report
- Report structure: score, violation counts per severity, per-rule breakdown, file list

**Out-of-Scope:**
- Running checks (report consumes existing results from FR-055)
- Auto-fixing violations (handled by FR-057)
- Quality trends over time (handled by FR-006)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **SARIF** | Static Analysis Results Interchange Format (OASIS standard) |
| **JUnit XML** | XML format originally for test results, repurposed for lint violations |
| **QualityReport** | Structured output with score, violations, metadata |
| **ReportFormatter** | Trait with implementations for each output format |

## 3. Feature Overview
### 3.1 Background & Problem
The `check` and `scan` commands only output text to stdout. Teams needed machine-readable formats for CI integration (SARIF for GitHub Code Scanning, JUnit for Jenkins/GitLab), JSON for custom tooling, and human-readable text for local development.

### 3.2 Business Goals
- Support 4 output formats: text, JSON, SARIF 2.1.0, JUnit XML
- Enable GitHub Code Scanning via SARIF upload
- Enable CI pipeline parsing via JUnit XML
- Enable custom tooling via JSON
- Provide clear human-readable output by default

### 3.3 Target Users
- **Developers**: Use text format for local feedback
- **DevOps**: Use JUnit XML for CI pipeline integration
- **Security Teams**: Use SARIF for GitHub Advanced Security / CodeQL integration
- **Custom Tooling**: Use JSON for programmatic consumption

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want `report . --output-format text` to see a readable table of violations.
- **US-002:** As a DevOps engineer, I want `report . --output-format sarif` to upload results to GitHub Code Scanning.
- **US-003:** As a CI pipeline maintainer, I want `report . --output-format junit` so Jenkins/GitLab can parse the results.
- **US-004:** As a tool builder, I want `report . --output-format json` to build custom dashboards.

### 4.2 Use Cases & Workflow
**Report Pipeline:**
```
lint-arwaky-cli report /project --output-format json
  │
  ├─► 1. Load last check results from cache OR re-run check
  │
  ├─► 2. Select formatter:
  │     ├── text → TextReportFormatter
  │     ├── json → JSONReportFormatter
  │     ├── sarif → SARIFReportFormatter (SARIF 2.1.0)
  │     └── junit → JUnitReportFormatter
  │
  └─► 3. Output formatted report to stdout
```

**SARIF Output Structure:**
```json
{
  "version": "2.1.0",
  "$schema": "https://docs.oasis-open.org/sarif/sarif/v2.1.0/schemas/sarif-schema-2.1.0.json",
  "runs": [{
    "tool": {
      "driver": {
        "name": "Lint Arwaky",
        "version": "1.10.2",
        "rules": [
          {"id": "AES001", "name": "forbidden-import", "shortDescription": {"text": "Forbidden import from upper layer"}}
        ]
      }
    },
    "results": [
      {
        "ruleId": "AES001",
        "level": "error",
        "message": {"text": "Forbidden import: surface imports from infrastructure"},
        "locations": [{
          "physicalLocation": {
            "artifactLocation": {"uri": "src-rust/cli-commands/surface_check.rs"},
            "region": {"startLine": 12}
          }
        }]
      }
    ]
  }]
}
```

**JUnit XML Output Structure:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="Lint Arwaky" tests="31" failures="3" errors="0">
  <testcase name="AES003: naming-convention" classname="architecture">
    <failure message="Naming violation" type="HIGH">
      File: src-rust/layer-rules/capabilities_import_checker.rs:12
    </failure>
  </testcase>
  <testcase name="AES005: file-size" classname="architecture"/>
</testsuite>
```

### 4.3 Business Rules
- Default format is `text` when `--output-format` is not specified
- Report is generated from last check/scan results (cached in memory)
- If no cached results, `report` re-runs `check` first
- SARIF output conforms to SARIF 2.1.0 (OASIS standard)
- JUnit XML maps each AES rule to a testcase, violations to failures

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Report generation (text, 100 violations) | < 100ms |
| NFR-002 | SARIF compliance | 100% valid per SARIF 2.1.0 schema |
| NFR-003 | JUnit XML parseable by Jenkins/GitLab | Verified |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli report /project --output-format text
📊 Quality Report for /project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 87.5 / 100
Language: Rust
Rules Checked: 31

Violations:
  CRITICAL: 0  HIGH: 3  MEDIUM: 5  LOW: 2

Format: --output-format json | sarif | junit

$ lint-arwaky-cli report /project --output-format json | jq '.score'
87.5

$ lint-arwaky-cli report /project --output-format sarif > lint-results.sarif
# Upload to GitHub: gh codeql upload-sarif --sarif=lint-results.sarif
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Check completed with 10 violations | `report --output-format text` | Readable table with all violations | Pending Review |
| AC-002 | Check completed with 10 violations | `report --output-format json` | Valid JSON with score + violations | Pending Review |
| AC-003 | Check completed with 10 violations | `report --output-format sarif` | Valid SARIF 2.1.0 output | Pending Review |
| AC-004 | Check completed with 10 violations | `report --output-format junit` | Valid JUnit XML, parseable by Jenkins | Pending Review |
| AC-005 | No cached results | `report --output-format json` | Check re-run, then report generated | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI report command | `cli-commands/surface_report_command.rs` | — | **FULLY IMPLEMENTED** — CLI dispatch with --output-format |
| Report formatter trait | `output-report/contract_report_formatter.rs` | — | **FULLY IMPLEMENTED** — trait with 4 implementations |
| Text formatter | `output-report/capabilities_text_formatter.rs` | — | **FULLY IMPLEMENTED** |
| JSON formatter | `output-report/capabilities_json_formatter.rs` | — | **FULLY IMPLEMENTED** — serde_json serialization |
| SARIF formatter | `output-report/capabilities_sarif_formatter.rs` | — | **FULLY IMPLEMENTED** — SARIF 2.1.0 |
| JUnit formatter | `output-report/capabilities_junit_formatter.rs` | — | **FULLY IMPLEMENTED** — JUnit XML |
| Score constants | `output-report/taxonomy_score_constant.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **SARIF formatter uses hardcoded `artifactLocation.uri` without base directory** — paths are relative to scan root, but SARIF spec expects absolute URIs or `artifactLocation.uriBaseId`
   - **Impact**: GitHub Code Scanning may not link results to correct files
   - **Fix**: Add `originalUriBaseIds` with project root, use relative URIs

2. **JUnit formatter maps all severities to `<failure>`** — CRITICAL, HIGH, MEDIUM, LOW are all `<failure>`; should use `<error>` for CRITICAL
   - **Impact**: CI pipelines cannot differentiate severity levels from JUnit
   - **Fix**: Map CRITICAL → `<error>`, HIGH/MEDIUM → `<failure>`, LOW → `<skipped>`

3. **No report caching** — `report` always re-runs `check` even if results haven't changed
   - **Impact**: 2x runtime for report generation
   - **Fix**: Cache last check results in memory, invalidate on file change

### 8.3 What Needs to Be Added

- **SARIF URI base**: Add `originalUriBaseIds` for correct file linking
- **JUnit severity mapping**: Different XML elements per severity level
- **Result caching**: Skip re-check if results are unchanged
- **File output**: Support `--output <file>` flag to write report to file

### 8.4 What to Keep

- **4 format implementations** ✅ — text, JSON, SARIF, JUnit all working
- **JSON correctness** ✅ — serde_json produces valid, parseable JSON
- **SARIF structure** ✅ — correct schema, rule metadata, locations
- **CLI dispatch** ✅ — clean `--output-format` flag routing

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli report test-project-rust/ --output-format json` produces valid JSON
- `lint-arwaky-cli report test-project-rust/ --output-format sarif` passes SARIF schema validation
- `lint-arwaky-cli report test-project-rust/ --output-format junit` parses in Jenkins
- Pending Review: SARIF URI base fix, JUnit severity mapping

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Check) | Check results are the data source | Check fails = no report | Report re-runs check if needed |
| serde_json | JSON serialization | None stable | Pinned in Cargo.toml |
| SARIF 2.1.0 schema | External standard | Schema changes | Pin to 2.1.0 |

## 10. Appendices
- `src-rust/cli-commands/surface_report_command.rs` — CLI report command
- `src-rust/output-report/contract_report_formatter.rs` — Formatter trait
- `src-rust/output-report/capabilities_sarif_formatter.rs` — SARIF formatter
- `src-rust/output-report/capabilities_junit_formatter.rs` — JUnit formatter
- `src-rust/output-report/capabilities_json_formatter.rs` — JSON formatter
- `src-rust/output-report/capabilities_text_formatter.rs` — Text formatter

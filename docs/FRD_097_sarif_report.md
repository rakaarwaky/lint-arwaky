# 📄 Feature Requirements Document (FRD)
**Feature Name:** SARIF 2.1.0 Report Format (GitHub Code Scanning)
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
This document defines the SARIF (Static Analysis Results Interchange Format) 2.1.0 report format for lint violations. This enables integration with GitHub Code Scanning, Azure DevOps, and other SARIF-compatible platforms.

### 2.2 Scope
**In-Scope:**
- `--format sarif` flag for lint commands
- SARIF 2.1.0 specification compliance (OASIS Standard)
- Tool information: name, version, rules (AES codes)
- Results: file, line, column, severity, rule ID, message
- GitHub Code Scanning upload compatibility

**Out-of-Scope:**
- SARIF 2.0 support (legacy)
- Embedded artifact content (snippets)
- Conversion from other formats to SARIF

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **SARIF** | Static Analysis Results Interchange Format — OASIS Standard |
| **Run** | A single execution of the analysis tool |
| **Result** | A single finding or violation |
| **Rule** | A lint rule (e.g., AES001) |
| **Artifact** | A file that was analyzed |
| **GitHub Code Scanning** | GitHub feature that consumes SARIF for PR annotations |

## 3. Feature Overview
### 3.1 Background & Problem
GitHub Code Scanning requires SARIF 2.1.0 format for uploading analysis results. Without SARIF support, lint-arwaky cannot integrate with GitHub's code scanning UI, which shows inline annotations on PRs and security overview dashboards.

### 3.2 Business Goals
- Enable GitHub Code Scanning integration
- Enable Azure DevOps SARIF integration
- Provide standardized interchange format for any SARIF consumer
- Align with industry standard for static analysis reporting

### 3.3 Target Users
- **GitHub Users**: View lint violations as Code Scanning alerts
- **CI/CD Engineers**: Upload SARIF to GitHub via `github/codeql-action/upload-sarif`
- **Enterprise Customers**: Integrate with Azure DevOps or other SARIF platforms

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a GitHub user, I want to run lint with `--format sarif` and upload the result to GitHub Code Scanning, so I can see violations inline in PRs.
- **US-002:** As a CI engineer, I want SARIF output to include tool metadata and all rule definitions, so the Code Scanning UI has full context.
- **US-003:** As a platform engineer, I want SARIF-compliant output for my enterprise dashboards.

### 4.2 Use Cases & Workflow
**SARIF Report Generation:**
```
lint-arwaky-cli check . --format sarif
  │
  ├─► 1. Collect all violations from lint engine
  │
  ├─► 2. Build SARIF document
  │     ├── version: "2.1.0"
  │     ├── runs[0].tool.driver: { name, version, rules[] }
  │     ├── runs[0].artifacts: [{ location.uri: file path }]
  │     └── runs[0].results: [{ ruleId, level, message, locations }]
  │
  └─► 3. Serialize to JSON and print to stdout
```

**Example Output (SARIF JSON):**
```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "lint-arwaky",
          "version": "1.10.2",
          "rules": [
            {
              "id": "AES001",
              "shortDescription": { "text": "Import layer violation" },
              "fullDescription": { "text": "Layer '{layer}' cannot import from '{target}'." },
              "defaultConfiguration": { "level": "error" },
              "properties": { "category": "Architecture" }
            }
          ]
        }
      },
      "artifacts": [
        {
          "location": { "uri": "src/main.rs" },
          "length": -1
        }
      ],
      "results": [
        {
          "ruleId": "AES001",
          "level": "error",
          "message": { "text": "Layer 'capabilities' cannot import from 'infrastructure'." },
          "locations": [
            {
              "physicalLocation": {
                "artifactLocation": { "uri": "src/main.rs" },
                "region": {
                  "startLine": 42,
                  "startColumn": 5
                }
              }
            }
          ]
        }
      ]
    }
  ]
}
```

### 4.3 Business Rules
- SARIF version: 2.1.0 (OASIS Standard)
- Severity mapping: CRITICAL → "error", HIGH → "warning", MEDIUM → "note", LOW → "note"
- Rule objects defined once per run, referenced by results via `ruleId`
- File URIs are relative to project root
- All required SARIF fields must be populated (spec validation)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | SARIF generation time for 1000 violations | < 100ms |
| NFR-002 | Output size for 1000 violations | < 1 MB |
| NFR-003 | Schema compliance | Validated against SARIF 2.1.0 JSON schema |
| NFR-004 | Encoding | UTF-8 only |

## 6. UI/UX Requirements
SARIF is machine-readable; the user experience is through GitHub Code Scanning:

```
# In CI workflow:
- name: Run lint
  run: lint-arwaky-cli check . --format sarif > results.sarif

- name: Upload SARIF to GitHub
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `--format sarif` flag used | Lint command runs | Valid SARIF 2.1.0 JSON output | Pending Review |
| AC-002 | 5 violations found | SARIF generated | All 5 violations in results array with rule references | Pending Review |
| AC-003 | SARIF validated against schema | Validation runs | Passes sarif-schema-2.1.0.json validation | Pending Review |
| AC-004 | No violations found | SARIF generated | Empty results array, valid SARIF document | Pending Review |
| AC-005 | CRITICAL violation | SARIF generated | `level: "error"` in result | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| SARIF formatter | `output-report/` | — | **NOT IMPLEMENTED** |
| SARIF model types | `output-report/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `format_sarif()` in report formatter
- SARIF model structs (SarifLog, Run, Tool, Result, etc.)
- Severity-to-SARIF-level mapping
- Rule deduplication (one entry per unique rule)
- Artifact collection (file paths)
- `--format sarif` CLI flag

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Violation list from lint engine | Engine must return structured violations | Use `LintResult` taxonomy type |
| SARIF 2.1.0 schema | OASIS standard | Schema changes | Pin to 2.1.0, follow semver for format |
| serde/serde_json | JSON serialization | Crate version updates | Pin to stable version |

## 10. Appendices
- SARIF 2.1.0 specification: https://docs.oasis-open.org/sarif/sarif/v2.1.0/
- SARIF JSON schema: https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json
- GitHub Code Scanning SARIF upload: https://docs.github.com/en/code-security/code-scanning
- `output-report/capabilities_sarif_formatter.rs` — SARIF formatter (to be created)

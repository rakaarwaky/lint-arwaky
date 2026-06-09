# 📄 Feature Requirements Document (FRD)
**Feature Name:** JUnit XML Report Format (CI Integration)
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
This document defines the JUnit XML report format for lint violations. This format enables integration with Jenkins, GitLab CI, CircleCI, and other CI systems that natively consume JUnit XML for test result visualization.

### 2.2 Scope
**In-Scope:**
- `--format junit` flag for lint commands
- JUnit XML schema compliance
- Violations mapped to test cases: file as test suite, violations as test cases
- Pass/fail summary with counts
- stdout/stderr-style output in test case details

**Out-of-Scope:**
- Ant-style JUnit XML (different schema namespace)
- JUnit 5 XML format (uses different structure)
- Test-suite-level properties

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **JUnit XML** | XML format originally from JUnit, widely adopted by CI systems for test results |
| **Test suite** | Represents a single file being linted |
| **Test case** | Represents a single violation or a "no violation" marker |
| **Failure** | CRITICAL or HIGH severity violation |
| **Error** | Lint engine runtime error (as opposed to violation) |

## 3. Feature Overview
### 3.1 Background & Problem
CI systems like Jenkins and GitLab CI have built-in JUnit XML parsers for displaying test results in pipelines. Without JUnit XML output, lint violations cannot be visualized as test results in CI dashboards, forcing teams to parse text output manually.

### 3.2 Business Goals
- Enable Jenkins JUnit plugin integration
- Enable GitLab CI test report visualization
- Enable CircleCI test summary integration
- Provide CI-native violation reporting

### 3.3 Target Users
- **CI/CD Engineers**: Configure CI pipelines to display lint results as test reports
- **Developers**: View lint violations in CI pipeline test summary pages
- **Platform Teams**: Standardize on JUnit XML for all quality tools

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a CI engineer, I want `--format junit` to produce JUnit XML, so Jenkins can display lint results in the test report.
- **US-002:** As a developer, I want to see lint failures in GitLab's pipeline test tab, so I don't have to scroll through raw logs.
- **US-003:** As a CI engineer, I want failed lint results to show the violation code and description in the test case failure message.

### 4.2 Use Cases & Workflow
**JUnit XML Report Generation:**
```
lint-arwaky-cli check . --format junit
  │
  ├─► 1. Group violations by file
  │
  ├─► 2. Build JUnit XML document
  │     ├── <testsuite name="lint-arwaky" tests="N" failures="M" errors="0">
  │     ├── For each file:
  │     │   ├── <testsuite name="src/main.rs" tests="X" failures="Y">
  │     │   ├── For each CRITICAL/HIGH violation:
  │     │   │   └── <testcase name="AES001:42" classname="src/main.rs">
  │     │   │       <failure message="Layer violation" type="CRITICAL"/>
  │     │   │   </testcase>
  │     │   ├── For MEDIUM/LOW violations:
  │     │   │   └── <testcase name="AES003:15" classname="src/utils.rs">
  │     │   │   </testcase> <!-- non-failure -->
  │     │   └── If file has no violations:
  │     │       └── <testcase name="passed" .../> <!-- success -->
  │     └── </testsuite>
  │
  └─► 3. Serialize to XML and print to stdout
```

**Example Output (JUnit XML):**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="lint-arwaky" tests="7" failures="2" errors="0" time="0.5">
  <testsuite name="src/main.rs" tests="2" failures="2">
    <testcase name="AES001:42" classname="src/main.rs" file="src/main.rs" line="42">
      <failure message="Layer 'capabilities' cannot import from 'infrastructure'." type="CRITICAL">
        FIX: Use ports/protocols from the contract layer instead.
      </failure>
    </testcase>
    <testcase name="AES014:88" classname="src/main.rs" file="src/main.rs" line="88">
      <failure message="Unwrap detected: line 88." type="CRITICAL">
        FIX: Use safe error propagation (?, expect, match).
      </failure>
    </testcase>
  </testsuite>
  <testsuite name="src/utils.rs" tests="3" failures="0">
    <testcase name="AES003:15" classname="src/utils.rs" file="src/utils.rs" line="15" />
    <testcase name="AES010:22" classname="src/utils.rs" file="src/utils.rs" line="22" />
    <testcase name="passed" classname="src/utils.rs" />
  </testsuite>
  <testsuite name="src/config.rs" tests="1" failures="0">
    <testcase name="passed" classname="src/config.rs" />
  </testsuite>
</testsuite>
```

### 4.3 Business Rules
- Severity mapping: CRITICAL and HIGH → `<failure>` element, MEDIUM and LOW → passing `<testcase>`
- Each file becomes a nested `<testsuite>` with `tests`, `failures` counts
- Files with no violations → one passing test case named "passed"
- `<failure>` message contains violation description, body contains suggested fix
- `file` and `line` attributes for IDE integration

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | XML generation time for 1000 violations | < 100ms |
| NFR-002 | Output size for 1000 violations | < 500 KB |
| NFR-003 | Encoding | UTF-8 with XML declaration |
| NFR-004 | XML well-formedness | Guaranteed (valid XML) |

## 6. UI/UX Requirements
JUnit XML is machine-readable; the user experience is through CI dashboards:

```
# Jenkins pipeline:
stage('Lint') {
    steps {
        sh 'lint-arwaky-cli check . --format junit > lint-results.xml'
    }
    post {
        always {
            junit 'lint-results.xml'
        }
    }
}
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | `--format junit` flag used | Lint command runs | Valid JUnit XML output | Pending Review |
| AC-002 | 2 CRITICAL + 1 MEDIUM violations | JUnit XML generated | 2 failures, 1 passing, correct structure | Pending Review |
| AC-003 | No violations (perfect score) | JUnit XML generated | All suites have 1 test case: "passed" | Pending Review |
| AC-004 | XML validated against JUnit schema | Validation runs | Well-formed XML | Pending Review |
| AC-005 | Jenkins consumes the output | Pipeline runs | Test results visible in Jenkins dashboard | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| JUnit formatter | `output-report/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `format_junit()` in report formatter
- JUnit XML writer (or use quick-xml crate)
- Severity-to-JUnit mapping
- File grouping into nested `<testsuite>` elements
- `--format junit` CLI flag

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Violation list from lint engine | Engine must return structured violations | Use `LintResult` taxonomy type |
| quick-xml or similar | XML serialization crate | Crate version updates | Pin to stable version |
| JUnit XML consumer | Jenkins/GitLab must recognize the output | Various CI parsers may interpret differently | Follow industry de facto standard |

## 10. Appendices
- JUnit XML format reference: https://github.com/testmoapp/junitxml
- Jenkins JUnit plugin: https://plugins.jenkins.io/junit/
- GitLab test reports: https://docs.gitlab.com/ee/ci/testing/
- `output-report/capabilities_junit_formatter.rs` — JUnit formatter (to be created)

# FRD — report-formatter

## System Overview

```
┌───────────────────────────────────────────────────────┐
│                    Surface Layer                       │
│  cli-commands calls report formatter aggregate         │
├───────────────────────────────────────────────────────┤
│                    Agent Layer                         │
│  report formatter orchestrator                         │
│  (routes formatting by format type)                    │
├───────────────────────────────────────────────────────┤
│                  Capabilities Layer                    │
│  text formatter (Text)                                │
│  JSON formatter (JSON)                                │
│  SARIF 2.1.0 formatter (SARIF)                        │
│  JUnit XML formatter (JUnit)                          │
├───────────────────────────────────────────────────────┤
│                   Utility Layer                        │
│  default text report fallback                          │
└───────────────────────────────────────────────────────┘
```

The report-formatter crate provides formatting capabilities for scan report output. It implements the report formatter protocol for each output format (text, JSON, SARIF, JUnit) and exposes the report formatter aggregate via the orchestrator for the surface layer to consume. The surface layer never formats output directly — it always delegates through the aggregate trait.

## Functional Requirements

### FR-001: Text Format Output
- **Description**: Produce human-readable text output with severity badges and violation details.
- **Input**: `report: ScanReport`, `format: Format::Text`
- **Output**: `DisplayContent` containing formatted text string
- **Business Rules**:
  - Delegates to the code analysis aggregate for detailed output formatting.
  - Reconstructs the lint result list from report results and passes default file path.
  - Falls back to the default text summary if format doesn't match text format.
  - Output includes violation counts by code, severity breakdown, and file locations.
- **Edge Cases**:
  - Empty results list → produces clean report with 0 violations.
  - Format mismatch → returns default text summary.
  - Report with diagnostics → includes diagnostic information in output.
- **Error Handling**: None — formatting is infallible.

### FR-002: JSON Format Output
- **Description**: Produce pretty-printed JSON output for CI/CD integration.
- **Input**: `report: ScanReport`, `format: Format::Json`
- **Output**: `DisplayContent` containing pretty-printed JSON string
- **Business Rules**:
  - Serializes report results via the JSON serialization library.
  - Falls back to empty array string on serialization failure.
  - Falls back to the default text summary if format doesn't match JSON format.
  - Each lint result includes: file, line, code, severity, message.
- **Edge Cases**:
  - Empty results → produces empty array string.
  - Serialization failure → returns empty array string.
  - Format mismatch → returns default text summary.
- **Error Handling**: Serialization error caught gracefully.

### FR-003: SARIF 2.1.0 Format Output
- **Description**: Produce SARIF 2.1.0 JSON format for IDE integration and GitHub Code Scanning.
- **Input**: `report: ScanReport`, `format: Format::Sarif` (also `results: &[LintResult]` for direct call)
- **Output**: `DisplayContent` containing SARIF 2.1.0 JSON string
- **Business Rules**:
  - Includes tool metadata: name, version, information URI.
  - Maps severity levels: CRITICAL/HIGH → error, MEDIUM → warning, LOW/INFO → note.
  - Each result includes: rule ID, level, message text, physical location.
  - Physical location includes artifact URI and start line.
  - Schema URI points to OASIS SARIF 2.1.0 schema.
  - Line numbers clamped to minimum 1.
- **Edge Cases**:
  - Empty results → valid SARIF with empty results array.
  - Line number 0 or negative → clamped to 1.
  - Format mismatch → returns default text summary.
  - Serialization failure → returns empty object string.
- **Error Handling**: Serialization error caught gracefully.

### FR-004: JUnit XML Format Output
- **Description**: Produce JUnit XML format for CI/CD test report integration.
- **Input**: `report: ScanReport`, `format: Format::Junit` (also `results: &[LintResult]` for direct call)
- **Output**: `DisplayContent` containing JUnit XML string
- **Business Rules**:
  - Each violation becomes a test case with classname (rule code) and name (file:line).
  - Non-INFO violations include failure element with message and type attributes.
  - INFO severity violations produce clean test case without failure element.
  - XML is properly escaped: `&`, `<`, `>`, `"`, `'` → named entities.
  - Root element: testsuites with tests and failure counts.
  - Pre-allocated string capacity based on result count.
- **Edge Cases**:
  - Empty results → valid XML with 0 tests, 0 failures.
  - All violations INFO severity → no failure elements.
  - Special characters in messages → properly XML-escaped.
  - Format mismatch → returns default text summary.
- **Error Handling**: None — XML generation is infallible.

### FR-005: Format Delegation (Orchestrator)
- **Description**: Route formatting request to the appropriate capabilities formatter based on `Format` enum.
- **Input**: `report: ScanReport`, `format: Format`
- **Output**: `DisplayContent`
- **Business Rules**:
  - Text format → text formatter.
  - JSON format → JSON formatter.
  - SARIF format → SARIF formatter.
  - JUnit format → JUnit formatter.
  - Each formatter implements the report formatter protocol.
  - Orchestrator holds a reference to each format's formatter implementation.
- **Edge Cases**:
  - Unknown format variant → exhaustive match ensures compile-time safety.
  - Formatter panicked → not caught (trait method is infallible).
- **Error Handling**: None — dispatch is infallible.

### FR-006: Default Report Fallback
- **Description**: Produce a simple text summary when the requested format doesn't match.
- **Input**: `report: ScanReport`
- **Output**: `String` containing summary text
- **Business Rules**:
  - Shows violation count, diagnostic count, and score (if available).
  - Groups violations by code, sorted by count (descending).
  - Shows diagnostics with source, severity, and message.
  - Pre-allocated capacity based on result count.
- **Edge Cases**:
  - Empty results → "Violations: 0".
  - No score in report → score line omitted.
  - No diagnostics → diagnostics section omitted.
- **Error Handling**: None — pure function.

### FR-007: XML Escape Utility
- **Description**: Escape special XML characters for safe inclusion in JUnit XML output.
- **Input**: `s: &str`
- **Output**: `String` with escaped characters
- **Business Rules**:
  - `&` → `&amp;`
  - `<` → `&lt;`
  - `>` → `&gt;`
  - `"` → `&quot;`
  - `'` → `&apos;`
  - All other characters passed through unchanged.
- **Edge Cases**:
  - Empty string → empty output.
  - No special characters → string unchanged.
  - Multiple special characters → all escaped.
- **Error Handling**: None — pure function.

## Data Model / Entity Relationship

```
ScanReport
├── results: Vec<LintResult>
├── diagnostics: Vec<PipelineDiagnostic>
└── score: Option<Score>

LintResult
├── file: FilePath
├── line: LineNumber
├── code: LintCode
├── severity: Severity (CRITICAL, HIGH, MEDIUM, LOW, INFO)
├── message: ErrorMessage
└── fixable: bool

Format (enum)
├── Text
├── Json
├── Sarif
└── Junit

DisplayContent (VO)
└── value: String

Report Formatter Protocol (interface)
├── format(report, format) -> DisplayContent
└── supported_format() -> Format

Report Formatter Aggregate (interface)
└── format(report, format) -> DisplayContent

SARIF Output Structure
├── $schema: SARIF 2.1.0 schema URI
├── version: "2.1.0"
└── runs[0]
    ├── tool.driver
    │   ├── name: "lint-arwaky"
    │   ├── version: CARGO_PKG_VERSION
    │   └── information_uri: GitHub URL
    └── results[]
        ├── rule_id: LintCode
        ├── level: "error" | "warning" | "note"
        ├── message.text: String
        └── locations[].physical_location
            ├── artifact_location.uri: String
            └── region.start_line: i64

JUnit Output Structure
├── testsuites (name="lint-arwaky", tests, failures)
└── testsuite (name="lint-arwaky", tests, failures)
    └── testcase (classname, name)
        └── failure? (message, type)
```

## API Contract

| Function | Input | Output | Description |
|---|---|---|---|
| `orchestrator::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Route to appropriate formatter |
| `text_formatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Human-readable text output |
| `text_formatter::format_text(report)` | `ScanReport` | `DisplayContent` | Direct text formatting |
| `json_formatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Pretty-printed JSON output |
| `sarif_formatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | SARIF 2.1.0 JSON output |
| `sarif_formatter::format_sarif(results)` | `&[LintResult]` | `DisplayContent` | Direct SARIF formatting |
| `junit_formatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | JUnit XML output |
| `junit_formatter::format_junit(results)` | `&[LintResult]` | `DisplayContent` | Direct JUnit formatting |
| `format_report_default(report)` | `&ScanReport` | `String` | Default text summary fallback |
| `xml_escape(s)` | `&str` | `String` | XML entity escaping |

## Integration Points

- **Internal**:
  - `shared` — taxonomy VOs, contract traits (report formatter protocol, report formatter aggregate).
  - `code-analysis` — code analysis aggregate for text formatter delegation.
  - `cli-commands` — consumed via report formatter aggregate from CLI container wiring.
- **External**:
  - JSON serialization library for JSON and SARIF formatters.
  - No other external dependencies — formatters are self-contained.

## Non-functional Requirements (Detailed)

- **Performance**: Pre-allocated string capacity based on result count to minimize reallocation.
- **Memory**: No heap allocation beyond output string — formatters are stateless except text formatter which holds a reference to the code analysis aggregate.
- **Correctness**: SARIF output matches OASIS SARIF 2.1.0 schema. JUnit XML is valid XML with proper escaping. JSON output is valid and pretty-printed.
- **Thread Safety**: All formatters implement Send + Sync via trait bounds.
- **Extensibility**: New formats added by implementing the report formatter protocol and adding variant to the Format enum.

## Test Scenarios / QA Checklist

- [ ] FR-001: Text formatter produces human-readable output with violation details
- [ ] FR-001: Text formatter falls back to default when format doesn't match
- [ ] FR-002: JSON formatter produces valid pretty-printed JSON
- [ ] FR-002: JSON formatter produces `"[]"` for empty results
- [ ] FR-003: SARIF output includes correct tool metadata (name, version, URI)
- [ ] FR-003: SARIF severity mapping: CRITICAL/HIGH→error, MEDIUM→warning, LOW/INFO→note
- [ ] FR-003: SARIF line numbers clamped to minimum 1
- [ ] FR-003: SARIF output matches 2.1.0 schema structure
- [ ] FR-004: JUnit XML is valid XML with proper declaration
- [ ] FR-004: JUnit violations produce `<failure>` elements
- [ ] FR-004: JUnit INFO violations produce clean `<testcase>` without `<failure>`
- [ ] FR-004: XML special characters properly escaped in all fields
- [ ] FR-004: JUnit test/failure counts match actual results
- [ ] FR-005: Orchestrator routes to correct formatter for each Format variant
- [ ] FR-006: Default fallback shows violation counts by code
- [ ] FR-006: Default fallback includes score when available
- [ ] FR-007: `xml_escape` escapes all 5 special characters
- [ ] FR-007: `xml_escape` passes through normal text unchanged

## Assumptions & Constraints

- All formatters are infallible — they cannot return errors (only display content).
- Scan report is the single input type for all formatters.
- Format routing is determined at compile time via exhaustive match on Format enum.
- Text formatter depends on code analysis aggregate for detailed formatting — other formatters are stateless.
- SARIF output uses the OASIS SARIF 2.1.0 schema — not earlier versions.
- JUnit XML follows the standard JUnit schema compatible with CI/CD parsers.

## Glossary

| Term | Definition |
|---|---|
| SARIF | Static Analysis Results Interchange Format — OASIS standard for tool output |
| JUnit XML | XML format originally from JUnit, widely used for CI/CD test reporting |
| DisplayContent | Semantic VO wrapping formatted string output |
| LintResult | Individual violation finding with file, line, code, severity, message |
| ScanReport | Aggregated results + diagnostics from a full pipeline run |
| Report Formatter Protocol | Interface for individual format implementations (text, json, sarif, junit) |
| Report Formatter Aggregate | Interface for the orchestrator that routes to the correct formatter |

## Reference

- PRD: [PRD.md](../../PRD.md)

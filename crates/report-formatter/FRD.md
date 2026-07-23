# FRD — report-formatter

## System Overview

```
┌───────────────────────────────────────────────────────┐
│                    Surface Layer                       │
│  cli-commands calls IReportFormatterAggregate.format() │
├───────────────────────────────────────────────────────┤
│                    Agent Layer                         │
│  agent_report_formatter_orchestrator.rs                │
│  ReportFormatterOrchestrator (delegates by Format)     │
├───────────────────────────────────────────────────────┤
│                  Capabilities Layer                    │
│  capabilities_text_formatter.rs   (Text)               │
│  capabilities_json_formatter.rs   (JSON)               │
│  capabilities_sarif_formatter.rs  (SARIF 2.1.0)       │
│  capabilities_junit_formatter.rs  (JUnit XML)          │
├───────────────────────────────────────────────────────┤
│                   Utility Layer                        │
│  utility_report_format.rs  (default text fallback)     │
└───────────────────────────────────────────────────────┘
```

The report-formatter crate provides formatting capabilities for `ScanReport` output. It implements `IReportFormatterProtocol` for each output format (text, JSON, SARIF, JUnit) and exposes `IReportFormatterAggregate` via `ReportFormatterOrchestrator` for the surface layer to consume. The surface layer never formats output directly — it always delegates through the aggregate trait.

## Functional Requirements

### FR-001: Text Format Output
- **Description**: Produce human-readable text output with severity badges and violation details.
- **Input**: `report: ScanReport`, `format: Format::Text`
- **Output**: `DisplayContent` containing formatted text string
- **Business Rules**:
  - Delegates to `ICodeAnalysisAggregate::format_report()` for detailed output.
  - Reconstructs `LintResultList` from report results and passes default `FilePath`.
  - Falls back to `format_report_default()` if format doesn't match `Format::Text`.
  - Output includes violation counts by code, severity breakdown, and file locations.
- **Edge Cases**:
  - Empty results list → produces clean report with 0 violations.
  - Format mismatch → returns default text summary via `format_report_default()`.
  - Report with diagnostics → includes diagnostic information in output.
- **Error Handling**: None — formatting is infallible.

### FR-002: JSON Format Output
- **Description**: Produce pretty-printed JSON output for CI/CD integration.
- **Input**: `report: ScanReport`, `format: Format::Json`
- **Output**: `DisplayContent` containing pretty-printed JSON string
- **Business Rules**:
  - Serializes `report.results` (Vec<LintResult>) via `serde_json::to_string_pretty`.
  - Falls back to `"[]"` string on serialization failure.
  - Falls back to `format_report_default()` if format doesn't match `Format::Json`.
  - Each `LintResult` includes: file, line, code, severity, message.
- **Edge Cases**:
  - Empty results → produces `"[]"`.
  - Serialization failure → returns `"[]"`.
  - Format mismatch → returns default text summary.
- **Error Handling**: Serialization error caught via `unwrap_or_else`.

### FR-003: SARIF 2.1.0 Format Output
- **Description**: Produce SARIF 2.1.0 JSON format for IDE integration and GitHub Code Scanning.
- **Input**: `report: ScanReport`, `format: Format::Sarif` (also `results: &[LintResult]` for direct call)
- **Output**: `DisplayContent` containing SARIF 2.1.0 JSON string
- **Business Rules**:
  - Includes tool metadata: name (`lint-arwaky`), version (`CARGO_PKG_VERSION`), information URI.
  - Maps severity levels: CRITICAL/HIGH → `error`, MEDIUM → `warning`, LOW/INFO → `note`.
  - Each result includes: `rule_id`, `level`, `message.text`, `locations[].physical_location`.
  - Physical location includes `artifact_location.uri` and `region.start_line`.
  - Schema URI points to OASIS SARIF 2.1.0 schema.
  - Line numbers clamped to minimum 1 via `std::cmp::max(1, r.line.value())`.
- **Edge Cases**:
  - Empty results → valid SARIF with empty `results` array.
  - Line number 0 or negative → clamped to 1.
  - Format mismatch → returns default text summary.
  - Serialization failure → returns `"{}"`.
- **Error Handling**: Serialization error caught via `unwrap_or_else`.

### FR-004: JUnit XML Format Output
- **Description**: Produce JUnit XML format for CI/CD test report integration.
- **Input**: `report: ScanReport`, `format: Format::Junit` (also `results: &[LintResult]` for direct call)
- **Output**: `DisplayContent` containing JUnit XML string
- **Business Rules**:
  - Each violation becomes a `<testcase>` with `classname` (rule code) and `name` (file:line).
  - Non-INFO violations include `<failure>` element with `message` and `type` attributes.
  - INFO severity violations produce clean `<testcase>` without `<failure>`.
  - XML is properly escaped: `&`, `<`, `>`, `"`, `'` → named entities.
  - Root element: `<testsuites>` with `tests` and `failure` counts.
  - Pre-allocated string capacity: `total * 256` bytes.
- **Edge Cases**:
  - Empty results → valid XML with 0 tests, 0 failures.
  - All violations INFO severity → no `<failure>` elements.
  - Special characters in messages → properly XML-escaped.
  - Format mismatch → returns default text summary.
- **Error Handling**: None — XML generation is infallible.

### FR-005: Format Delegation (Orchestrator)
- **Description**: Route formatting request to the appropriate capabilities formatter based on `Format` enum.
- **Input**: `report: ScanReport`, `format: Format`
- **Output**: `DisplayContent`
- **Business Rules**:
  - `Format::Text` → `TextFormatter`
  - `Format::Json` → `JsonFormatter`
  - `Format::Sarif` → `SarifFormatter`
  - `Format::Junit` → `JunitFormatter`
  - Each formatter implements `IReportFormatterProtocol`.
  - Orchestrator holds `Arc<dyn IReportFormatterProtocol>` for each format.
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
  - Pre-allocated capacity: `256 + results.len() * 32` bytes.
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

IReportFormatterProtocol (trait)
├── format(&self, report: &ScanReport, format: Format) -> DisplayContent
└── supported_format(&self) -> Format

IReportFormatterAggregate (trait)
└── format(&self, report: &ScanReport, format: Format) -> DisplayContent

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
| `ReportFormatterOrchestrator::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Route to appropriate formatter |
| `TextFormatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Human-readable text output |
| `TextFormatter::format_text(report)` | `ScanReport` | `DisplayContent` | Direct text formatting |
| `JsonFormatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | Pretty-printed JSON output |
| `SarifFormatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | SARIF 2.1.0 JSON output |
| `SarifFormatter::format_sarif(results)` | `&[LintResult]` | `DisplayContent` | Direct SARIF formatting |
| `JunitFormatter::format(report, format)` | `ScanReport, Format` | `DisplayContent` | JUnit XML output |
| `JunitFormatter::format_junit(results)` | `&[LintResult]` | `DisplayContent` | Direct JUnit formatting |
| `format_report_default(report)` | `&ScanReport` | `String` | Default text summary fallback |
| `xml_escape(s)` | `&str` | `String` | XML entity escaping |

## Integration Points

- **Internal**:
  - `shared` — taxonomy VOs (`ScanReport`, `LintResult`, `Format`, `DisplayContent`), contract traits (`IReportFormatterProtocol`, `IReportFormatterAggregate`).
  - `code-analysis` — `ICodeAnalysisAggregate` for `TextFormatter` delegation to `format_report()`.
  - `cli-commands` — consumed via `IReportFormatterAggregate` from `CliContainer` wiring.
- **External**:
  - `serde_json` — JSON serialization for JSON and SARIF formatters.
  - No other external dependencies — formatters are self-contained.

## Non-functional Requirements (Detailed)

- **Performance**: Pre-allocated string capacity based on result count to minimize reallocation. Text formatter: `256 + results.len() * 32`. JUnit formatter: `total * 256`.
- **Memory**: No heap allocation beyond output string — formatters are stateless except `TextFormatter` which holds an `Arc<dyn ICodeAnalysisAggregate>`.
- **Correctness**: SARIF output matches OASIS SARIF 2.1.0 schema. JUnit XML is valid XML with proper escaping. JSON output is valid and pretty-printed.
- **Thread Safety**: All formatters implement `Send + Sync` via trait bounds. `Arc<dyn IReportFormatterProtocol>` allows concurrent access.
- **Extensibility**: New formats added by implementing `IReportFormatterProtocol` and adding variant to `Format` enum.

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

- All formatters are infallible — they cannot return errors (only `DisplayContent`).
- `ScanReport` is the single input type for all formatters.
- Format routing is determined at compile time via exhaustive match on `Format` enum.
- Text formatter depends on `ICodeAnalysisAggregate` for `format_report()` — other formatters are stateless.
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
| IReportFormatterProtocol | Trait for individual format implementations (text, json, sarif, junit) |
| IReportFormatterAggregate | Trait for the orchestrator that routes to the correct formatter |

## Reference

- PRD: [PRD.md](../../PRD.md)

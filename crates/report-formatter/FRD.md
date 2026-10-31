# FRD — Report Formatter

## Feature Overview

The **Report Formatter** feature sits in the agent/aggregate layer and is called by the scan surface through the `IReportFormatterAggregate` front door. The surface passes a `ScanReport` plus the requested `Format` variant; the aggregate routes the request to exactly one of four formatters and returns the rendered `DisplayContent`.

`[Surface] --> [IReportFormatterAggregate] --> [TextFormatter | JsonFormatter | SarifFormatter | JunitFormatter] --> [DisplayContent] --> [Surface]`

All formatters are self-contained: each operates solely on `ScanReport` data and does not depend on other rule crates. Findings arrive in three shapes — AES violation codes, external-linter rule ids, and `PARSE_WARN` entries — and every formatter renders all three consistently. A parse failure recorded once per `ScanReport` must surface exactly once in every rendered output.

## Functional Requirements

### FR-ReportFormatter-001: Render Text Format Output

**Description:** The feature shall render a `ScanReport` as human-readable console text. The text output shall group findings per file, end with a summary line, and be infallible: every `ScanReport`, including an empty one, shall produce a valid text body without raising an error.

#### Business Logic
- The renderer shall group findings by source file and append a summary line carrying total counts.
- The renderer shall preserve AES violation codes, external-linter rule ids, and `PARSE_WARN` entries in the rendered text.
- The renderer shall never fail or panic on any `ScanReport` input.
- An empty `ScanReport` shall render as a valid text body with a zero-finding summary.

#### Edge Cases
- Empty report: the renderer shall produce a valid text body with zero findings, and the body shall round-trip to disk unchanged.
- Width/failure fields inside findings: rendering shall not fail on long or invalid-width data.
- Findings are rendered once each; the renderer shall not duplicate a finding across files.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A `ScanReport` delivered to the aggregate with `Format::Text` selected. |
| **Output** | A `DisplayContent` carrying the rendered text, returned to the caller for stdout or file destination. |
| **Protocol** | `ITextFormatProtocol` |
| **Method** | `format_text` |
| **Event** | None; rendering is a pure transformation. |
| **Error** | No error path; the formatter shall not raise an error for any `ScanReport`. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| The report carries findings across multiple files | The aggregate formats it with `Format::Text` | The returned `DisplayContent` groups findings per file and ends with a summary line. |
| The report is empty | The aggregate formats it with `Format::Text` | The returned `DisplayContent` renders a valid text body with a zero-finding summary. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Formatting failures on any seeded `ScanReport` | 0 |
| Findings duplicated in rendered text | 0 |

### FR-ReportFormatter-002: Render JSON Format Output

**Description:** The feature shall render a `ScanReport` as canonical JSON. The JSON document shall serialize every finding and the summary counters, and shall parse back to the same logical report; an empty `ScanReport` shall serialize as valid JSON with a zero summary.

#### Business Logic
- The renderer shall emit JSON that serializes all findings and the summary counters.
- The emitted JSON shall parse without error when consumed by a JSON parser.
- Findings sourced from external linters shall retain their tool-native rule ids in the JSON.
- An empty `ScanReport` shall serialize as valid JSON with a zero summary.

#### Edge Cases
- Empty report: the JSON shall parse with a zero summary block.
- A full report shall round-trip to disk and parse back with identical summary counters.
- Non-UTF-8 corner values in findings shall serialize as escaped JSON strings.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A `ScanReport` delivered to the aggregate with `Format::Json` selected. |
| **Output** | A `DisplayContent` carrying JSON text, consumed by downstream parsers and CI gates. |
| **Protocol** | `IJsonFormatProtocol` |
| **Method** | `format_json` |
| **Event** | None. |
| **Error** | No error path; malformed JSON would surface as a consumer-side parse failure, never as a formatter error. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| The report carries external-lint findings with tool-native rule ids | The aggregate formats it with `Format::Json` | The JSON parses and the rule ids are preserved byte-for-byte. |
| The report is empty | The aggregate formats it with `Format::Json` | The JSON parses with a zero summary block. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Formatted outputs that fail to parse as JSON | 0 |
| Summary counters diverging from the input report | 0 |

### FR-ReportFormatter-003: Render SARIF 2.1.0 Format Output

**Description:** The feature shall render a `ScanReport` as a SARIF 2.1.0 document for code-scanning uploaders. The document shall map findings to SARIF results with a severity level and rule id, retain tool-native rule ids for external findings, and validate as SARIF 2.1.0 even when the report is empty.

#### Business Logic
- The renderer shall emit a SARIF 2.1.0 document carrying `runs`, a tool driver block, and result entries.
- Each finding shall map to a SARIF result with a severity level and a rule id.
- Findings sourced from external linters shall retain their tool-native rule ids.
- An empty `ScanReport` shall emit a valid SARIF 2.1.0 document with zero results.

#### Edge Cases
- Empty report: the document shall validate as SARIF 2.1.0 with zero results.
- A full report shall round-trip to disk and re-validate against the SARIF 2.1.0 schema.
- A finding with an unknown severity shall map to a default SARIF level rather than failing validation.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A `ScanReport` delivered to the aggregate with `Format::Sarif` selected. |
| **Output** | A `DisplayContent` carrying a SARIF 2.1.0 document for upload tooling. |
| **Protocol** | `ISarifFormatProtocol` |
| **Method** | `format_sarif` |
| **Event** | None. |
| **Error** | No error path; schema violations would surface as consumer-side validation failures. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| The report carries findings with mixed severities | The aggregate formats it with `Format::Sarif` | The document validates as SARIF 2.1.0 and each result carries a severity level and rule id. |
| The report is empty | The aggregate formats it with `Format::Sarif` | The document validates as SARIF 2.1.0 with zero results. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Seeded formatted reports that fail SARIF 2.1.0 validation | 0 |

### FR-ReportFormatter-004: Render JUnit XML Format Output

**Description:** The feature shall render a `ScanReport` as JUnit XML for CI test-report ingestion. The XML shall wrap test cases in a test suite, represent each finding as one failing test case named with the rule id, and parse as valid XML even when the report is empty.

#### Business Logic
- The renderer shall emit JUnit XML with a test suite wrapping test cases.
- Each finding shall appear as exactly one failing test case.
- Each test case name shall carry the rule id of its finding.
- An empty `ScanReport` shall emit a valid test suite with zero failing test cases.

#### Edge Cases
- Empty report: the XML shall parse as valid XML with zero failing test cases.
- A full report shall round-trip to disk and parse with one test case per finding.
- A formatter invoked through the aggregate routing shall return the same content as a direct call.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A `ScanReport` delivered to the aggregate with `Format::JUnit` selected. |
| **Output** | A `DisplayContent` carrying JUnit XML consumed by CI reporters. |
| **Protocol** | `IJUnitFormatProtocol` |
| **Method** | `format_junit` |
| **Event** | None. |
| **Error** | No error path; malformed XML would surface on the CI parse side. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| The report carries multiple findings | The aggregate formats it with `Format::JUnit` | The XML parses and each finding appears as one failing test case named with its rule id. |
| The report is empty | The aggregate formats it with `Format::JUnit` | The XML parses as a valid test suite with zero failing test cases. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Seeded formatted reports that fail JUnit XML parsing | 0 |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `ReportFormatterOrchestrator` routes each `Format` variant to its dedicated formatter. |
| Aggregate | `IReportFormatterAggregate`, the feature's single front door. |
| Method | `format(report, format)` returns the rendered `DisplayContent`. |
| Request | A `ScanReport` plus the requested `Format` variant. |
| Response | A `DisplayContent` rendered in the requested format. |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

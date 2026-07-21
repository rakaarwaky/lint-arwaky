# FRD — report-formatter

## Feature Goal
The report-formatter crate provides formatting capabilities for ScanReport output. It implements `IReportFormatterProtocol` for each output format (text, JSON, SARIF, JUnit) and exposes `IReportFormatterAggregate` via `ReportFormatterOrchestrator` for the surface layer to consume.

## Architecture

### Layer Role
- **Capabilities** (`capabilities_*.rs`) — Single-responsibility formatters implementing `IReportFormatterProtocol`
- **Agent** (`agent_report_formatter_orchestrator.rs`) — Delegates to the appropriate formatter based on requested format

### Dependency Flow
```
Surface (cli-commands) → IReportFormatterAggregate (shared) → ReportFormatterOrchestrator (agent) → IReportFormatterProtocol (capabilities)
```

## Formatters

### TextFormatter
- Produces human-readable text output with severity badges
- Delegates to `ICodeAnalysisAggregate::format_report()` for detailed output
- Fallback: simple text summary with violation counts by code

### JsonFormatter
- Produces pretty-printed JSON output via `serde_json::to_string_pretty`
- Machine-readable format for CI/CD integration

### SarifFormatter
- Produces SARIF 2.1.0 JSON format
- Includes tool metadata (name, version, information URI)
- Maps severity levels: CRITICAL/HIGH → error, MEDIUM → warning, LOW/INFO → note

### JunitFormatter
- Produces JUnit XML format
- Each violation becomes a `<testcase>` with `<failure>` element
- Proper XML escaping for special characters (`&`, `<`, `>`, `"`, `'`)

## Contracts

### IReportFormatterProtocol (shared)
```rust
pub trait IReportFormatterProtocol: Send + Sync {
    fn format(&self, report: &ScanReport, format: Format) -> String;
    fn supported_format(&self) -> Format;
}
```

### IReportFormatterAggregate (shared)
```rust
pub trait IReportFormatterAggregate: Send + Sync {
    fn format(&self, report: &ScanReport, format: Format) -> String;
}
```

## Dependencies
- `shared` — Taxonomy types (ScanReport, Format, LintResult) and contract traits
- `code-analysis` — ICodeAnalysisAggregate for TextFormatter delegation

## Success Indicators
- [ ] AES compliance — capabilities implement protocol, agent orchestrates
- [ ] No inline formatting — surface delegates to aggregate, never formats directly
- [ ] All formats covered — text, JSON, SARIF, JUnit
- [ ] Proper XML escaping — JUnit output is valid XML
- [ ] SARIF compliance — output matches SARIF 2.1.0 schema

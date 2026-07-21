# report-formatter

Report formatting capabilities for lint-arwaky. Provides text, JSON, SARIF, and JUnit formatters implementing `IReportFormatterProtocol`.

## Usage

```rust
use report_formatter::{TextFormatter, JsonFormatter, SarifFormatter, JunitFormatter};
use report_formatter::ReportFormatterOrchestrator;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;

// Create formatters
let text = Arc::new(TextFormatter::new(code_analysis_linter));
let json = Arc::new(JsonFormatter::new());
let sarif = Arc::new(SarifFormatter::new());
let junit = Arc::new(JunitFormatter::new());

// Create orchestrator
let formatter = ReportFormatterOrchestrator::new(text, json, sarif, junit);

// Format output
let output = formatter.format(&report, Format::Sarif);
```

## Architecture

This crate follows AES rules:
- **Capabilities** (`capabilities_*.rs`) — Implement `IReportFormatterProtocol`
- **Agent** (`agent_report_formatter_orchestrator.rs`) — Implements `IReportFormatterAggregate`
- **No Surface** — Surface layer lives in `cli-commands`

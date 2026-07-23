// PURPOSE: report-formatter — formatting capabilities for ScanReport output
//
// Provides text, JSON, SARIF, and JUnit formatters implementing
// IReportFormatterProtocol. Consumed by cli-commands via
// IReportFormatterAggregate (agent layer).
pub mod agent_report_formatter_orchestrator;
pub mod capabilities_json_formatter;
pub mod capabilities_junit_formatter;
pub mod capabilities_sarif_formatter;
pub mod capabilities_text_formatter;
pub mod taxonomy_sarif_vo;
pub mod utility_report_format;

pub use agent_report_formatter_orchestrator::{ReportFormatterDeps, ReportFormatterOrchestrator};
pub use capabilities_json_formatter::JsonFormatter;
pub use capabilities_junit_formatter::JunitFormatter;
pub use capabilities_sarif_formatter::SarifFormatter;
pub use capabilities_text_formatter::TextFormatter;
pub use utility_report_format::format_report_default;

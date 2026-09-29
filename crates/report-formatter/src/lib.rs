// PURPOSE: report-formatter — formatting capabilities for ScanReport output
//
// Provides text, JSON, SARIF, and JUnit formatters, each implementing its
// FR-ReportFormatter-NNN format seam. Consumed by cli-commands via
// IReportFormatterAggregate (agent layer). Exactly 4 FRs = 4 protocols = 4 capabilities.
pub mod agent_report_formatter_orchestrator;
pub mod capabilities_json_formatter;
pub mod capabilities_junit_formatter;
pub mod capabilities_sarif_formatter;
pub mod capabilities_text_formatter;

pub use agent_report_formatter_orchestrator::{ReportFormatterDeps, ReportFormatterOrchestrator};
pub use capabilities_json_formatter::JsonFormatter;
pub use capabilities_junit_formatter::{JunitFormatter, xml_escape};
pub use capabilities_sarif_formatter::SarifFormatter;
pub use capabilities_text_formatter::TextFormatter;

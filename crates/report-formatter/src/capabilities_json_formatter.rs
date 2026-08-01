// PURPOSE: JsonFormatter — implements IReportFormatterProtocol for JSON output
//
// Formats ScanReport into pretty-printed JSON output.
use shared::report_formatter::format_report_default;
use shared::cli_commands::{Format, ScanReport};

use shared::common::DisplayContent;
use shared::report_formatter::IReportFormatterProtocol;

// ─── Block 1: Struct Definition ───────────────────────────
/// JsonFormatter — produces JSON output from ScanReport.
pub struct JsonFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Json {
            DisplayContent::new(
                serde_json::to_string_pretty(&report.results).unwrap_or_else(|_| "[]".to_string()),
            )
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self
    }
}

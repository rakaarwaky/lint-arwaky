// PURPOSE: JsonFormatter — implements IReportFormatterProtocol for JSON output
//
// Formats ScanReport into pretty-printed JSON output.
use super::utility_report_format::format_report_default;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

use std::marker::PhantomData;

// ─── Block 1: Struct Definition ───────────────────────────
/// JsonFormatter — produces JSON output from ScanReport.
pub struct JsonFormatter {
    _p: PhantomData<()>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> String {
        if format == Format::Json {
            serde_json::to_string_pretty(&report.results).unwrap_or_else(|_| "[]".to_string())
        } else {
            format_report_default(report)
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
        Self { _p: PhantomData }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

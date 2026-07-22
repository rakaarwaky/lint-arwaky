// PURPOSE: TextFormatter — implements IReportFormatterProtocol for text output
//
// Formats ScanReport into human-readable text output, matching the existing
// code_analysis_linter.format_report() output style.
use super::utility_report_format::format_report_default;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// TextFormatter — produces human-readable text output from ScanReport.
pub struct TextFormatter {
    code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for TextFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Text {
            self.format_text(report)
        } else {
            // Fallback to default formatting
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Text
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl TextFormatter {
    /// Create a new text formatter.
    pub fn new(
        code_analysis_linter: Arc<
            dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
        }
    }

    /// Format the scan report into human-readable text.
    pub fn format_text(&self, report: &ScanReport) -> DisplayContent {
        // Reconstruct a FilePath for the code analysis formatter
        let results_list =
            shared::cli_commands::taxonomy_result_vo::LintResultList::new(report.results.clone());
        let report_path = shared::common::taxonomy_path_vo::FilePath::default();
        DisplayContent::new(
            self.code_analysis_linter
                .format_report(&results_list, &report_path),
        )
    }
}

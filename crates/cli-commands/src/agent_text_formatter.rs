// PURPOSE: TextFormatter — implements IReportFormatterProtocol for text output
//
// Formats ScanReport into human-readable text output, matching the existing
// code_analysis_linter.format_report() output style.
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use std::sync::Arc;

/// TextFormatter — produces human-readable text output from ScanReport.
pub struct TextFormatter {
    code_analysis_linter: Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
}

impl TextFormatter {
    /// Create a new text formatter.
    pub fn new(code_analysis_linter: Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>) -> Self {
        Self { code_analysis_linter }
    }

    /// Format the scan report into human-readable text.
    pub fn format_text(&self, report: &ScanReport) -> String {
        // Reconstruct a FilePath for the code analysis formatter
        let results_list = shared::cli_commands::taxonomy_result_vo::LintResultList::new(report.results.clone());
        let report_path = shared::common::taxonomy_path_vo::FilePath::default();
        self.code_analysis_linter.format_report(&results_list, &report_path)
    }
}

#[async_trait::async_trait]
impl IReportFormatterProtocol for TextFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> String {
        if format == Format::Text {
            self.format_text(report)
        } else {
            // Fallback to default formatting
            format_report_default(report)
        }
    }

    fn supported_format(&self) -> Format {
        Format::Text
    }
}

/// Default report formatter — produces a simple text summary.
pub fn format_report_default(report: &ScanReport) -> String {
    let mut output = String::new();
    output.push_str("Lint Arwaky Report\n");
    output.push_str(&format!("Violations: {}\n", report.results.len()));
    output.push_str(&format!("Diagnostics: {}\n", report.diagnostics.len()));
    if let Some(score) = &report.score {
        output.push_str(&format!("Score: {:.1}/100\n", score.value()));
    }

    // Group violations by code
    let mut code_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for r in &report.results {
        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
    }
    if !code_counts.is_empty() {
        output.push_str("\nViolations by code:\n");
        let mut sorted: Vec<_> = code_counts.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        for (code, count) in &sorted {
            output.push_str(&format!("  {code}: {count}\n"));
        }
    }

    // Show diagnostics
    if !report.diagnostics.is_empty() {
        output.push_str("\nDiagnostics:\n");
        for d in &report.diagnostics {
            output.push_str(&format!("  [{}/:{:?}] {}\n", d.source, d.severity, d.message));
        }
    }

    output
}

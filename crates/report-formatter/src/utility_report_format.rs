// PURPOSE: Stateless utility functions for report formatting (AES406)
// Pure functions only — no domain types, structs, or enums belong here

use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

/// Default report formatter — produces a simple text summary.
///
/// Used by all formatters as the fallback when the requested format doesn't match.
pub fn format_report_default(report: &ScanReport) -> String {
    let mut output = String::with_capacity(256 + report.results.len() * 32);
    output.push_str("Lint Arwaky Report\n");
    output.push_str(&format!("Violations: {}\n", report.results.len()));
    output.push_str(&format!("Diagnostics: {}\n", report.diagnostics.len()));

    // Show score if available
    if let Some(score) = &report.score {
        output.push_str(&format!("Score: {:.1}/100\n", score.value()));
    }

    // Group violations by code
    let mut code_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::with_capacity(20);
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
            output.push_str(&format!(
                "  [{}/:{:?}] {}\n",
                d.source, d.severity, d.message
            ));
        }
    }

    output
}

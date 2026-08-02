// PURPOSE: TextFormatter — implements IReportFormatterProtocol for text output
//
// Self-contained: operates solely on ScanReport data without delegating
// to other crates. Produces human-readable output with severity badges,
// violation counts grouped by rule code (descending), severity breakdown,
// external lint results section, diagnostics section, and compliance score.
use shared::cli_commands::{Format, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::IReportFormatterProtocol;
use crate::utility_report_format::format_report_default;
use std::collections::BTreeMap;

// ─── Block 1: Struct Definition ───────────────────────────
/// TextFormatter — produces self-contained human-readable text output from ScanReport.
pub struct TextFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for TextFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Text {
            self.format_text(report)
        } else {
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
    pub fn new() -> Self {
        Self
    }

    /// Format the scan report into human-readable text.
    pub fn format_text(&self, report: &ScanReport) -> DisplayContent {
        let mut out = String::with_capacity(1024 + report.results.len() * 80);

        // ── Header ──
        out.push_str("Lint Arwaky Report\n");
        out.push_str("===========================================\n\n");

        // ── Separate AES violations from external lint results ──
        let mut aes_violations = Vec::new();
        let mut external_results = Vec::new();
        for r in &report.results {
            if r.code.code().starts_with("AES") || r.code.code().starts_with("PARSE_") {
                aes_violations.push(r);
            } else {
                external_results.push(r);
            }
        }

        // ── AES Violations Section ──
        out.push_str(&format!("AES Violations: {}\n", aes_violations.len()));
        out.push_str("-------------------------------------------\n");
        if aes_violations.is_empty() {
            out.push_str("  None\n");
        } else {
            for r in &aes_violations {
                let badge = severity_badge(&r.severity);
                out.push_str(&format!(
                    "  {} {} {}:{}  {}\n",
                    badge,
                    r.code.code(),
                    r.file.value,
                    r.line.value(),
                    r.message.value,
                ));
            }
        }
        out.push('\n');

        // ── Violation counts grouped by rule code (descending) ──
        if !aes_violations.is_empty() {
            let counts = group_by_code(&aes_violations);
            out.push_str("Violations by rule code:\n");
            for (code, count) in &counts {
                out.push_str(&format!("  {code}: {count}\n"));
            }
            out.push('\n');
        }

        // ── Severity breakdown ──
        if !aes_violations.is_empty() {
            let sev = count_by_severity(&aes_violations);
            out.push_str("Severity breakdown:\n");
            out.push_str(&format!("  CRITICAL: {}\n", sev.critical));
            out.push_str(&format!("  HIGH:     {}\n", sev.high));
            out.push_str(&format!("  MEDIUM:   {}\n", sev.medium));
            out.push_str(&format!("  LOW:      {}\n", sev.low));
            out.push_str(&format!("  INFO:     {}\n", sev.info));
            out.push('\n');
        }

        // ── External Lint Results Section ──
        out.push_str(&format!(
            "External Lint Results: {}\n",
            external_results.len()
        ));
        out.push_str("-------------------------------------------\n");
        if external_results.is_empty() {
            out.push_str("  None\n");
        } else {
            // Group by tool (source)
            let mut by_tool: BTreeMap<
                String,
                Vec<&shared::cli_commands::taxonomy_result_vo::LintResult>,
            > = BTreeMap::new();
            for r in &external_results {
                let tool = r
                    .source
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                by_tool.entry(tool).or_default().push(r);
            }
            for (tool, results) in &by_tool {
                out.push_str(&format!("  [{tool}]\n"));
                for r in results {
                    out.push_str(&format!(
                        "    {} {}:{}  {}\n",
                        r.code.code(),
                        r.file.value,
                        r.line.value(),
                        r.message.value,
                    ));
                }
            }
        }
        out.push('\n');

        // ── Diagnostics Section ──
        if !report.diagnostics.is_empty() {
            out.push_str(&format!("Diagnostics: {}\n", report.diagnostics.len()));
            out.push_str("-------------------------------------------\n");
            for d in &report.diagnostics {
                let sev_label = match d.severity {
                    shared::cli_commands::taxonomy_scan_report_vo::DiagnosticSeverity::Warning => {
                        "WARNING"
                    }
                    shared::cli_commands::taxonomy_scan_report_vo::DiagnosticSeverity::Error => {
                        "ERROR"
                    }
                    shared::cli_commands::taxonomy_scan_report_vo::DiagnosticSeverity::Info => {
                        "INFO"
                    }
                };
use crate::utility_report_format::format_report_default;
                out.push_str(&format!("  [{}] [{}] {}\n", sev_label, d.source, d.message));
            }
            out.push('\n');
        }

        // ── Summary ──
        out.push_str("===========================================\n");
        out.push_str(&format!("Total violations: {}\n", report.results.len()));
        if let Some(score) = &report.score {
            out.push_str(&format!("Compliance score: {:.1}/100\n", score.value()));
        }

        DisplayContent::new(out)
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Helper Functions ─────────────────────────────────────

fn severity_badge(sev: &shared::common::taxonomy_severity_vo::Severity) -> &'static str {
    match sev {
        shared::common::taxonomy_severity_vo::Severity::CRITICAL => "[!!!]",
        shared::common::taxonomy_severity_vo::Severity::HIGH => "[!! ]",
        shared::common::taxonomy_severity_vo::Severity::MEDIUM => "[!  ]",
        shared::common::taxonomy_severity_vo::Severity::LOW => "[.  ]",
        shared::common::taxonomy_severity_vo::Severity::INFO => "[   ]",
    }
}

fn group_by_code(
    results: &[&shared::cli_commands::taxonomy_result_vo::LintResult],
) -> Vec<(String, usize)> {
    let mut counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::with_capacity(20);
    for r in results {
        *counts.entry(r.code.code().to_string()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
    sorted
}

struct SeverityCounts {
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    info: usize,
}

fn count_by_severity(
    results: &[&shared::cli_commands::taxonomy_result_vo::LintResult],
) -> SeverityCounts {
    let mut counts = SeverityCounts {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        info: 0,
    };
    for r in results {
        match r.severity {
            shared::common::taxonomy_severity_vo::Severity::CRITICAL => counts.critical += 1,
            shared::common::taxonomy_severity_vo::Severity::HIGH => counts.high += 1,
            shared::common::taxonomy_severity_vo::Severity::MEDIUM => counts.medium += 1,
            shared::common::taxonomy_severity_vo::Severity::LOW => counts.low += 1,
            shared::common::taxonomy_severity_vo::Severity::INFO => counts.info += 1,
        }
    }
    counts
}

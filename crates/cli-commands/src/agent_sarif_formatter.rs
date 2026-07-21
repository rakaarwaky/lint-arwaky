// PURPOSE: SarifFormatter — implements IReportFormatterProtocol for SARIF output
//
// Formats ScanReport into SARIF 2.1.0 JSON format.
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

/// SarifFormatter — produces SARIF 2.1.0 JSON output from ScanReport.
pub struct SarifFormatter;

impl SarifFormatter {
    /// Create a new SARIF formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SarifFormatter {
    fn default() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl IReportFormatterProtocol for SarifFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> String {
        if format == Format::Sarif {
            self.format_sarif(&report.results)
        } else {
            format_report_default(report)
        }
    }

    fn supported_format(&self) -> Format {
        Format::Sarif
    }
}

impl SarifFormatter {
    /// Format results as a SARIF 2.1.0 JSON string.
    pub fn format_sarif(&self, results: &[LintResult]) -> String {
        #[derive(serde::Serialize)]
        struct SarifLog {
            #[serde(rename = "$schema")]
            schema: &'static str,
            version: &'static str,
            runs: Vec<SarifRun>,
        }

        #[derive(serde::Serialize)]
        struct SarifRun {
            tool: SarifTool,
            results: Vec<SarifResult>,
        }

        #[derive(serde::Serialize)]
        struct SarifTool {
            driver: SarifDriver,
        }

        #[derive(serde::Serialize)]
        struct SarifDriver {
            name: &'static str,
            version: &'static str,
            information_uri: &'static str,
        }

        #[derive(serde::Serialize)]
        struct SarifResult {
            rule_id: String,
            level: String,
            message: SarifMessage,
            locations: Vec<SarifLocation>,
        }

        #[derive(serde::Serialize)]
        struct SarifMessage {
            text: String,
        }

        #[derive(serde::Serialize)]
        struct SarifLocation {
            physical_location: SarifPhysicalLocation,
        }

        #[derive(serde::Serialize)]
        struct SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation,
            region: SarifRegion,
        }

        #[derive(serde::Serialize)]
        struct SarifArtifactLocation {
            uri: String,
        }

        #[derive(serde::Serialize)]
        struct SarifRegion {
            start_line: i64,
        }

        // Map Severity → SARIF level
        fn severity_to_sarif_level(sev: &shared::cli_commands::taxonomy_severity_vo::Severity) -> &'static str {
            match sev {
                shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL |
                shared::cli_commands::taxonomy_severity_vo::Severity::HIGH => "error",
                shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM => "warning",
                shared::cli_commands::taxonomy_severity_vo::Severity::LOW |
                shared::cli_commands::taxonomy_severity_vo::Severity::INFO => "note",
            }
        }

        let sarif_results: Vec<SarifResult> = results
            .iter()
            .map(|r| SarifResult {
                rule_id: r.code.to_string(),
                level: severity_to_sarif_level(&r.severity).to_string(),
                message: SarifMessage {
                    text: r.message.value.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: r.file.value.clone(),
                        },
                        region: SarifRegion {
                            start_line: std::cmp::max(1, r.line.value()),
                        },
                    },
                }],
            })
            .collect();

        let log = SarifLog {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            version: "2.1.0",
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "lint-arwaky",
                        version: env!("CARGO_PKG_VERSION"),
                        information_uri: "https://github.com/rakaarwaky/lint-arwaky",
                    },
                },
                results: sarif_results,
            }],
        };

        serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Default report formatter — produces a simple text summary.
pub fn format_report_default(report: &ScanReport) -> String {
    let mut output = String::new();
    output.push_str("Lint Arwaky Report\n");
    output.push_str(&format!("Violations: {}\n", report.results.len()));
    output.push_str(&format!("Diagnostics: {}\n", report.diagnostics.len()));

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

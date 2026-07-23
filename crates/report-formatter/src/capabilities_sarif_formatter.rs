// PURPOSE: SarifFormatter — implements IReportFormatterProtocol for SARIF output
//
// Formats ScanReport into SARIF 2.1.0 JSON format.
use crate::utility_report_format::format_report_default;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_display_content_vo::DisplayContent;

// ─── Block 1: Struct Definition ───────────────────────────
/// SarifFormatter — produces SARIF 2.1.0 JSON output from ScanReport.
pub struct SarifFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for SarifFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Sarif {
            self.format_sarif(&report.results)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Sarif
    }
}

impl SarifFormatter {
    /// Format results as a SARIF 2.1.0 JSON string wrapped in DisplayContent.
    pub fn format_sarif(&self, results: &[LintResult]) -> DisplayContent {
        use crate::taxonomy_sarif_vo::{
            SarifArtifactLocation, SarifDriver, SarifLocation, SarifLog, SarifMessage,
            SarifPhysicalLocation, SarifRegion, SarifResult, SarifRun, SarifTool,
        };

        // Map Severity → SARIF level
        fn severity_to_sarif_level(
            sev: &shared::common::taxonomy_severity_vo::Severity,
        ) -> &'static str {
            match sev {
                shared::common::taxonomy_severity_vo::Severity::CRITICAL
                | shared::common::taxonomy_severity_vo::Severity::HIGH => "error",
                shared::common::taxonomy_severity_vo::Severity::MEDIUM => "warning",
                shared::common::taxonomy_severity_vo::Severity::LOW
                | shared::common::taxonomy_severity_vo::Severity::INFO => "note",
            }
        }

        let mut sarif_results = Vec::with_capacity(results.len());
        for r in results {
            sarif_results.push(SarifResult {
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
            });
        }

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

        DisplayContent::new(serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string()))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
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

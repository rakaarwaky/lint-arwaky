// PURPOSE: SarifFormatter — implements IReportFormatterProtocol for SARIF 2.1.0 output
use std::collections::BTreeSet;

use crate::utility_report_format::format_report_default;
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::{
    IReportFormatterProtocol, SarifArtifactLocation, SarifDriver, SarifLocation, SarifLog,
    SarifMessage, SarifPhysicalLocation, SarifRegion, SarifResult, SarifRule, SarifRun, SarifTool,
};

// ─── Block 1: Struct Definition ───────────────────────────
/// SarifFormatter — produces SARIF 2.1.0 JSON output from ScanReport.
pub struct SarifFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for SarifFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Sarif {
            self.format_sarif_report(report)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Sarif
    }
}

impl SarifFormatter {
    /// Format full ScanReport as SARIF 2.1.0 JSON string wrapped in DisplayContent.
    pub fn format_sarif_report(&self, report: &ScanReport) -> DisplayContent {
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

        let mut sarif_results = Vec::new();
        let mut rule_ids = BTreeSet::new();

        // 1. Violations
        for r in &report.results {
            let rule_id = r.code.to_string();
            rule_ids.insert(rule_id.clone());
            sarif_results.push(SarifResult {
                rule_id,
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

        // 2. Diagnostics
        for d in &report.diagnostics {
            let rule_id = "PARSE_WARN".to_string();
            rule_ids.insert(rule_id.clone());
            sarif_results.push(SarifResult {
                rule_id,
                level: "note".to_string(),
                message: SarifMessage {
                    text: format!("{} (source: {})", d.message, d.source),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: "workspace".to_string(),
                        },
                        region: SarifRegion { start_line: 1 },
                    },
                }],
            });
        }

        // Rules metadata array
        let rules: Vec<SarifRule> = rule_ids
            .into_iter()
            .map(|id| SarifRule {
                id,
                default_configuration_level: "error".to_string(),
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
                rules,
            }],
        };

        DisplayContent::new(serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string()))
    }

    /// Direct call for &[LintResult] for backward compatibility.
    pub fn format_sarif(&self, results: &[LintResult]) -> DisplayContent {
        let dummy_report = ScanReport {
            results: results.to_vec(),
            diagnostics: vec![],
            score: None,
        };
        self.format_sarif_report(&dummy_report)
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

// PURPOSE: SarifFormatter — implements ISarifFormatProtocol for SARIF output (FR-003).
use std::collections::{BTreeMap, BTreeSet};

use shared_cli_commands::{Format, LintResult, ScanReport};
use shared_common::taxonomy_display_content_vo::DisplayContent;
use shared_report_formatter::contract_report_formatter_protocol::ISarifFormatProtocol;
use shared_report_formatter::taxonomy_report_formatter_vo::{
    SarifArtifactLocation, SarifLocation, SarifPhysicalLocation, SarifRegion,
};
use shared_report_formatter::taxonomy_report_formatter_vo::{SarifDriver, SarifRule};
use shared_report_formatter::taxonomy_report_formatter_vo::{SarifLog, SarifRun, SarifTool};
use shared_report_formatter::taxonomy_report_formatter_vo::{SarifMessage, SarifResult};

// ─── Block 1: Struct Definition ───────────────────────────
/// SarifFormatter — produces SARIF 2.1.0 JSON output from ScanReport.
pub struct SarifFormatter;

// ─── Block 2: Protocol Trait Implementations ──────────────

impl ISarifFormatProtocol for SarifFormatter {
    fn format_sarif(&self, report: &ScanReport) -> DisplayContent {
        format_sarif_report(report)
    }

    fn supported_format(&self) -> Format {
        Format::Sarif
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl SarifFormatter {
    /// Create a new SARIF formatter.
    pub fn new() -> Self {
        Self
    }

    /// Direct call for &[LintResult] for backward compatibility.
    pub fn format_sarif_slice(&self, results: &[LintResult]) -> DisplayContent {
        let dummy_report = ScanReport {
            results: results.to_vec(),
            diagnostics: vec![],
            score: None,
        };
        format_sarif_report(&dummy_report)
    }
}

impl Default for SarifFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Free Functions (stateless renderers) ─────────────────

/// FR-003: render a full ScanReport as a SARIF 2.1.0 document.
fn format_sarif_report(report: &ScanReport) -> DisplayContent {
    fn severity_to_sarif_level(sev: &shared_common::Severity) -> &'static str {
        match sev {
            shared_common::Severity::CRITICAL | shared_common::Severity::HIGH => "error",
            shared_common::Severity::MEDIUM => "warning",
            shared_common::Severity::LOW | shared_common::Severity::INFO => "note",
        }
    }

    let mut sarif_results = Vec::new();
    let mut rule_ids = BTreeSet::new();

    // Track max severity level per rule for default_configuration_level
    let mut rule_levels: BTreeMap<String, &str> = BTreeMap::new();
    let level_priority = |l: &str| match l {
        "error" => 3,
        "warning" => 2,
        "note" => 1,
        _ => 0,
    };

    // 1. Violations
    for r in &report.results {
        let rule_id = r.code.to_string();
        rule_ids.insert(rule_id.clone());

        // PARSE_WARN diagnostics always map to "note" regardless of severity
        let level = if r.code.code().starts_with("PARSE_") {
            "note"
        } else {
            severity_to_sarif_level(&r.severity)
        };

        rule_levels
            .entry(rule_id.clone())
            .and_modify(|existing| {
                if level_priority(level) > level_priority(existing) {
                    *existing = level;
                }
            })
            .or_insert(level);

        sarif_results.push(SarifResult {
            rule_id,
            level: level.to_string(),
            message: SarifMessage {
                text: r.message.value().to_string(),
            },
            locations: vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation {
                        uri: r.file.value().to_string(),
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

    // Rules metadata array — uses max severity level per rule
    let rules: Vec<SarifRule> = rule_ids
        .into_iter()
        .map(|id| SarifRule {
            id: id.clone(),
            default_configuration_level: rule_levels
                .get(&id)
                .copied()
                .unwrap_or("error")
                .to_string(),
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

    // A serialization failure must never render as a bare `{}`: a lenient SARIF
    // consumer can read an object with no `runs` as a clean scan. Emit an
    // explicitly marked error document that fails schema validation instead.
    DisplayContent::new(serde_json::to_string_pretty(&log).unwrap_or_else(|e| {
        serde_json::json!({
            "formatter_error": true,
            "format": "sarif",
            "reason": e.to_string(),
        })
        .to_string()
    }))
}

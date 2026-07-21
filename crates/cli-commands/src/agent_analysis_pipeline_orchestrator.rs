// PURPOSE: AnalysisPipelineOrchestrator — implements IAnalysisPipelineAggregate
//
// This is the agent layer orchestrator that wires together all 6 linter groups
// and produces a unified ScanReport. It depends only on contracts (traits),
// never on concrete implementations.
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

/// AnalysisPipelineOrchestrator — agent layer that coordinates the full lint pipeline.
///
/// Implements IAnalysisPipelineAggregate by running all 6 linter groups in sequence:
///   1. Code analysis (AES301-305)
///   2. Naming rules (AES101-102)
///   3. Import rules (AES201-205)
///   4. External linters (Clippy, Ruff, ESLint, etc.)
///   5. Role rules (AES401-406)
///   6. Orphan detection (AES501-506)
// ─── Block 1: Struct Definition ───────────────────────────
pub struct AnalysisPipelineOrchestrator {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub format: Format,
    pub filter: Option<String>,
    pub member: Option<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IAnalysisPipelineAggregate for AnalysisPipelineOrchestrator {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        self.run_pipeline(request).await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl AnalysisPipelineOrchestrator {
    /// Run the full analysis pipeline on a target path.
    ///
    /// This is the core scan pipeline. It runs all 6 linter groups in the
    /// same order every time and collects results into a ScanReport.
    pub async fn run_pipeline(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        let target = &request.target.value;
        let path_obj = FilePath::new(target.to_string()).map_err(PipelineError::InvalidPath)?;

        let mut all_results = Vec::new();
        let mut diagnostics = Vec::new();

        // 1. Run AES analysis (AES301-305) — file lines, bypass, mandatory defs
        let aes_results = self.code_analysis_linter.run_code_analysis(&path_obj);
        let aes_count = aes_results.len();
        all_results.extend(aes_results.values);
        diagnostics.push(PipelineDiagnostic::new(
            "code-analysis".to_string(),
            format!("AES analysis complete: {aes_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 2-5. Run async linter groups concurrently
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PipelineError::Analysis(format!("failed to create runtime: {e}")))?;

        let (naming_results, import_results, external_results, role_results) = rt.block_on(async {
            tokio::join!(
                self.naming_orchestrator.run_audit(&path_obj),
                self.import_orchestrator.run_audit(&path_obj),
                self.external_lint.scan_all(&path_obj),
                self.role_orchestrator.run_audit(&path_obj),
            )
        });

        // Report audit failures instead of silently discarding them
        match naming_results {
            Ok(values) => {
                let naming_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit complete: {naming_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] naming audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        match import_results {
            Ok(values) => {
                let import_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit complete: {import_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] import audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        let external_count = external_results.len();
        all_results.extend(external_results.values);
        let role_count = role_results.len();
        all_results.extend(role_results);
        diagnostics.push(PipelineDiagnostic::new(
            "external".to_string(),
            format!("External lint complete: {external_count} violations"),
            DiagnosticSeverity::Info,
        ));
        diagnostics.push(PipelineDiagnostic::new(
            "roles".to_string(),
            format!("Role audit complete: {role_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 6. Run orphan detection (AES501-506) — dead code via import graph
        let orphan_results = self.run_orphan_detection(target).await;
        let orphan_count = orphan_results.len();
        all_results.extend(orphan_results);
        diagnostics.push(PipelineDiagnostic::new(
            "orphan".to_string(),
            format!("Orphan detection complete: {orphan_count} violations"),
            DiagnosticSeverity::Info,
        ));

        Ok(ScanReport::new(all_results, diagnostics))
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    async fn run_orphan_detection(&self, path: &str) -> Vec<LintResult> {
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let ignored = self.config_orchestrator.ignored_paths(orphan_scan_root);
        let source_files = match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.orphan_orchestrator
            .check_orphans(&file_strs, orphan_scan_root)
    }

    /// Filter results to the target path and return formatted output string.
    pub fn format_results(&self, results: Vec<LintResult>, path: &str) -> String {
        let canonical_scan_path = std::path::PathBuf::from(path);
        let canonical_scan_path = canonical_scan_path
            .canonicalize()
            .unwrap_or(canonical_scan_path);
        let cwd = crate::surface_common_command::current_dir();

        // Filter results to the target path (P2.3: use Path::starts_with)
        let filtered_results: Vec<_> = results
            .into_iter()
            .filter(|r| {
                let abs_path = cwd.join(&r.file.value);
                abs_path.starts_with(&canonical_scan_path)
            })
            .collect();

        match self.format {
            Format::Text => {
                let results_list =
                    shared::cli_commands::taxonomy_result_vo::LintResultList::new(filtered_results);
                let report_path = FilePath::new(path.to_string()).unwrap_or_default();
                self.code_analysis_linter
                    .format_report(&results_list, &report_path)
            }
            Format::Json => {
                serde_json::to_string_pretty(&filtered_results).unwrap_or_else(|_| "[]".to_string())
            }
            Format::Sarif => self.format_sarif_output(&filtered_results),
            Format::Junit => self.format_junit_output(&filtered_results),
        }
    }

    /// Format results as a SARIF 2.1.0 JSON string.
    fn format_sarif_output(&self, results: &[LintResult]) -> String {
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
        fn severity_to_sarif_level(
            sev: &shared::cli_commands::taxonomy_severity_vo::Severity,
        ) -> &'static str {
            match sev {
                shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
                | shared::cli_commands::taxonomy_severity_vo::Severity::HIGH => "error",
                shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM => "warning",
                shared::cli_commands::taxonomy_severity_vo::Severity::LOW
                | shared::cli_commands::taxonomy_severity_vo::Severity::INFO => "note",
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

    /// Format results as JUnit XML.
    fn format_junit_output(&self, results: &[LintResult]) -> String {
        let total = results.len();
        let failures: Vec<_> = results
            .iter()
            .filter(|r| {
                matches!(
                    r.severity,
                    shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
                        | shared::cli_commands::taxonomy_severity_vo::Severity::HIGH
                        | shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM
                        | shared::cli_commands::taxonomy_severity_vo::Severity::LOW
                )
            })
            .collect();
        let failure_count = failures.len();

        let mut xml = String::with_capacity(total.saturating_mul(256));
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));
        xml.push_str(&format!(
            "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));

        for r in results {
            let classname = xml_escape(&r.code.to_string());
            let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
            let message = xml_escape(&r.message.value);
            let sev = r.severity.to_string();
            let is_info = r.severity == shared::cli_commands::taxonomy_severity_vo::Severity::INFO;

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            if !is_info {
                xml.push_str(&format!(
                    "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
                ));
                xml.push_str(&format!("        {message}\n"));
                xml.push_str("      </failure>\n");
            }
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        xml
    }
}

/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}

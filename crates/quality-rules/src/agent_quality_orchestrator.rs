// PURPOSE: CodeAnalysisOrchestrator — agent that orchestrates Code Quality (AES301–AES305) checks, file collection, and reporting
// ALGORITHM (run_lint_at):
//   1. Load config; build ignored-patterns list
//   2. Recursively collect all lintable source files from src_dir (via detect_source_dir + collect_source_files)
//   3. Fail early if no files found
//   4. Run all checks directly (no async/Tokio overhead)
// ALGORITHM (run_all_checks):
//   1. If config.enabled = false, return empty
//   2. Pre-read files into (path, content) entries; skip unreadable files
//   3. For each file:
//      a. Run bypass_checker.check_bypass_comments (AES304 — layer-independent)
//      b. Run dead_inheritance_checker.check_dead_inheritance (AES303 sub-check 2)
//      c. Skip barrel files (mod.rs, __init__.py, index.ts)
//      d. Detect layer from filename prefix; skip if unknown or in exception list
//      e. Run line_checker.check_line_counts (AES301–302)
//      f. Run class_checker.check_mandatory_class_definition (AES303 sub-check 1)
//   4. Run duplication check using pre-read entries (AES305)
//   5. Return aggregated LintResult list

use rayon::prelude::*;
use shared_cli_commands::{LintResult, LintResultList};

use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared_quality_rules::contract_quality_protocol::IBypassCheckerProtocol;
use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::contract_quality_protocol::IDeadInheritanceProtocol;
use shared_quality_rules::contract_quality_protocol::ILineCheckerProtocol;
use shared_quality_rules::contract_quality_protocol::IMandatoryClassProtocol;
use shared_quality_rules::taxonomy_quality_rules_request::CodeAnalysisRequest;
use shared_quality_rules::taxonomy_quality_rules_response::CodeAnalysisResponse;

use shared_common::taxonomy_display_content_vo::DisplayContent;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::utility_compliance_score::compute_score;
use shared_common::utility_layer_detector::{
    collect_layer_keys, detect_layer_from_prefix, extract_filename, get_layer_def,
    resolve_specialized_layer,
};
use shared_common::{BooleanVO, Score};
use shared_common::{LayerMapVO, LayerNameVO};
use shared_config_system::ArchitectureConfig;
use shared_quality_rules::CodeAnalysisRuleVO;

use shared_quality_rules::utility_violation_formatter::format_code_analysis_violation;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CodeAnalysisDeps {
    pub bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    pub dead_inheritance_checker: Arc<dyn IDeadInheritanceProtocol>,
    pub line_checker: Arc<dyn ILineCheckerProtocol>,
    pub class_checker: Arc<dyn IMandatoryClassProtocol>,
    pub duplication_checker: Arc<dyn ICodeMetricAnalyzerProtocol>,
}

pub struct CodeAnalysisOrchestrator {
    deps: CodeAnalysisDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl ICodeAnalysisAggregate for CodeAnalysisOrchestrator {
    fn execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        match request {
            CodeAnalysisRequest::RunAnalysis { files } => CodeAnalysisResponse::Analysis {
                violations: self.run_analysis_with_entries(&files),
            },
            CodeAnalysisRequest::CalcScore { results } => CodeAnalysisResponse::Score {
                score: self.calc_score(&results),
            },
            CodeAnalysisRequest::FormatReport {
                results,
                project_root,
            } => CodeAnalysisResponse::Report {
                content: self.format_report(&LintResultList::new(results), &project_root),
            },
            CodeAnalysisRequest::CheckCritical { results } => CodeAnalysisResponse::Critical {
                is_critical: self.check_critical(&results),
            },
            CodeAnalysisRequest::ActiveRules => CodeAnalysisResponse::Rules {
                rules: self.active_rules(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CodeAnalysisOrchestrator {
    pub fn new(deps: CodeAnalysisDeps, config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        Self {
            deps,
            config,
            layer_map,
        }
    }

    /// Render a compliance report from results.
    pub fn render_report(&self, results: &[LintResult], project_root: &str) -> String {
        // Pre-allocated header (static string, no repeat allocation)
        let header = "============================================================";
        let mut output = String::with_capacity(results.len() * 80 + 120);
        output.push_str(header);
        output.push_str("\n  AES Architecture Compliance Report \n");
        output.push_str(header);
        output.push_str(&format!("\n  Project: {}\n", project_root));
        output.push_str(&format!("  Violations: {}\n", results.len()));
        output.push('\n');
        for r in results {
            output.push_str(&format!(
                "  [{}] {} - {}\n",
                r.code, r.file.value, r.message.value
            ));
        }
        output
    }

    /// Run analysis on pre-parsed file entries from the filesystem crate.
    pub fn run_analysis_with_entries(
        &self,
        files: &[shared_filesystem::taxonomy_filesystem_vo::FileEntry],
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let mut violations: Vec<LintResult> = Vec::new();

        // Parallel per-file processing using FileEntry content directly
        let file_violations: Vec<Vec<LintResult>> = files
            .par_iter()
            .filter(|f| f.parse_ok && !f.content.is_empty())
            .map(|entry| {
                let mut v = Vec::new();
                let file = entry.path.to_string_lossy();
                let filename = std::path::Path::new(file.as_ref())
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                let c = &entry.content;

                // Layer-independent checks
                self.deps
                    .bypass_checker
                    .check_bypass_comments(&file, c, &mut v);
                self.deps
                    .dead_inheritance_checker
                    .check_dead_inheritance(&file, c, &mut v);

                // AES304: Cargo.toml bypass detection
                if filename == "Cargo.toml" || filename == "cargo.toml" {
                    self.deps.bypass_checker.check_cargo_toml(c, &mut v);
                    return v;
                }

                // Skip barrel files (single source: shared_common::DEFAULT_RULE_EXCEPTIONS)
                if shared_common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) {
                    return v;
                }

                // Layer detection
                let fname = extract_filename(&file);
                let layer = match detect_layer_from_prefix(fname) {
                    Some(l) => l,
                    None => return v,
                };
                let keys = collect_layer_keys(&self.layer_map);
                let layer = LayerNameVO::new(resolve_specialized_layer(&layer, &file, &keys));
                let def = match get_layer_def(&layer.value, &self.config.layers) {
                    Some(d) => d,
                    None => return v,
                };
                if def.exceptions.values.contains(&fname.to_string()) {
                    return v;
                }

                // Layer-dependent checks
                self.deps
                    .line_checker
                    .check_line_counts(&file, Some(def), c, &mut v);
                self.deps.class_checker.check_mandatory_class_definition(
                    &file,
                    Some(def),
                    c,
                    &mut v,
                );

                v
            })
            .collect();

        for file_v in file_violations {
            violations.extend(file_v);
        }

        // AES305: duplication analysis on pre-fetched entries
        let entries: Vec<(std::path::PathBuf, String)> = files
            .iter()
            .filter(|f| f.parse_ok && !f.content.is_empty())
            .map(|f| (f.path.clone(), f.content.clone()))
            .collect();
        if !entries.is_empty() {
            // Hoist AES305 rule lookup outside per-violation loop (was O(violations) per scan)
            let aes305_rule = self.config.rules.iter().find(|r| r.name.value == "AES305");
            for (file_path, aes_violation) in self
                .deps
                .duplication_checker
                .handle_duplicates_entries(&entries)
            {
                // Check AES305 exceptions (match against file stem or full filename)
                let file_name = std::path::Path::new(&file_path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let file_stem = std::path::Path::new(&file_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let excluded = if let Some(rule) = aes305_rule
                    && (rule.exceptions.values.contains(&file_name.to_string())
                        || rule.exceptions.values.contains(&file_stem.to_string()))
                {
                    true
                } else {
                    false
                };
                if excluded {
                    continue;
                }
                let msg = format_code_analysis_violation(&aes_violation);
                violations.push(LintResult::new_arch(
                    &file_path,
                    1,
                    "AES305",
                    Severity::MEDIUM,
                    msg,
                ));
            }
        }

        violations
    }

    pub fn calc_score(&self, results: &[LintResult]) -> Score {
        let cs: fn(&[LintResult]) -> f64 = compute_score;
        Score::new(cs(results))
    }

    pub fn check_critical(&self, results: &[LintResult]) -> BooleanVO {
        BooleanVO::new(
            shared_quality_rules::utility_compliance_checker::contains_critical_severity(results),
        )
    }

    pub fn format_report(
        &self,
        results: &LintResultList,
        project_root: &FilePath,
    ) -> DisplayContent {
        DisplayContent::new(self.render_report(&results.values, project_root.value()))
    }

    pub fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        self.config
            .rules
            .iter()
            .map(|r| r.code_analysis.clone())
            .collect()
    }
}

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
use shared::cli_commands::{LintResult, LintResultList};

use shared::quality_rules::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::quality_rules::contract_class_protocol::IMandatoryClassProtocol;
use shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::quality_rules::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::quality_rules::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::quality_rules::contract_line_protocol::ILineCheckerProtocol;

use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_compliance_score::compute_score;
use shared::common::utility_layer_detector::{
    collect_layer_keys, detect_layer_from_prefix, extract_filename, get_layer_def,
    resolve_specialized_layer,
};
use shared::common::{BooleanVO, Score};
use shared::common::{LayerMapVO, LayerNameVO};
use shared::config_system::ArchitectureConfig;
use shared::quality_rules::CodeAnalysisRuleVO;

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
    fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList {
        LintResultList::new(self.legacy_entry_disabled("run_code_analysis", project_root.value()))
    }

    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList {
        LintResultList::new(self.legacy_entry_disabled("run_code_analysis_dir", src_dir.value()))
    }

    fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult> {
        self.legacy_entry_disabled("run_code_analysis_path", path.value())
    }

    fn calc_score(&self, results: &[LintResult]) -> Score {
        let cs: fn(&[LintResult]) -> f64 = compute_score;
        Score::new(cs(results))
    }

    fn check_critical(&self, results: &[LintResult]) -> BooleanVO {
        let hc: fn(&[LintResult]) -> bool = has_critical;
        BooleanVO::new(hc(results))
    }

    fn format_report(&self, results: &LintResultList, project_root: &FilePath) -> DisplayContent {
        DisplayContent::new(self.render_report(&results.values, project_root.value()))
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        self.config
            .rules
            .iter()
            .map(|r| r.code_analysis.clone())
            .collect()
    }

    /// Run analysis on pre-parsed file entries from the filesystem crate.
    fn run_analysis_with_entries(
        &self,
        files: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
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

                // Skip barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
                if shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) {
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
                if let Some(rule) = aes305_rule
                    && rule.exceptions.values.contains(&file_name.to_string())
                {
                    continue;
                }
                let msg = match &aes_violation {
                    shared::quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason } => {
                        format!(
                            "AES305 CODE_DUPLICATION: Duplicate code block detected.\nWHY? {}\nFIX: Extract the duplicated logic into a shared function.",
                            reason.as_ref().map(|r| r.to_string()).unwrap_or_default()
                        )
                    }
                    other => format!("{:?}", other),
                };
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
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

/// Check if any CRITICAL severity violations exist in results.
#[rustfmt::skip]
pub fn has_critical
    (results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

impl CodeAnalysisOrchestrator {
    pub fn new(deps: CodeAnalysisDeps, config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        Self {
            deps,
            config,
            layer_map,
        }
    }

    /// Legacy entry points cannot run without pre-fetched FileEntry data (crate is zero-I/O).
    /// Fail loudly so callers cannot silently skip AES301–AES305.
    fn legacy_entry_disabled(&self, method: &str, target: &str) -> Vec<LintResult> {
        vec![LintResult::new_arch(
            target,
            0,
            "AES301",
            Severity::CRITICAL,
            format!(
                "AES301 API_MISUSE: quality-rules entry point `{}` requires pre-fetched file entries.\nWHY? This method returns no results by design (zero-I/O aggregate).\nFIX: Call filesystem.build_file_index() then run_analysis_with_entries(file_list()).",
                method
            ),
        )]
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
}

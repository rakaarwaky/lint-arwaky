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

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;

use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_compliance_score::compute_score;
use shared::common::utility_layer_detector::{
    collect_layer_keys, detect_layer_from_prefix, extract_filename, get_layer_def,
    resolve_specialized_layer,
};
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;
use std::path::Path;
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
        LintResultList::new(self.run_self_lint(project_root.value()))
    }

    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList {
        LintResultList::new(self.run_scan(src_dir.value()))
    }

    fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult> {
        self.run_self_lint(path.value())
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
        DisplayContent::new(self.format_report(&results.values, project_root.value()))
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        self.config
            .rules
            .iter()
            .map(|r| r.code_analysis.clone())
            .collect()
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

    /// Run AES analysis on the current project (self-lint).
    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = shared::code_analysis::utility_target_resolver::detect_source_dir(root);
        self.run_lint_at(&src_dir)
    }

    /// Run AES analysis on a specific directory.
    pub fn run_scan(&self, target_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(target_dir))
    }

    /// Core method: collect files and run all checks.
    fn run_lint_at(&self, src_dir: &Path) -> Vec<LintResult> {
        let config = &self.config;
        let dir_path = match DirectoryPath::new(src_dir.to_string_lossy().to_string()) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let ignored: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        let files = shared::code_analysis::utility_target_resolver::collect_source_files(
            src_dir, &dir_path, &ignored,
        );
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        self.run_all_checks(&files_str, &root_dir)
    }

    /// Run code-analysis AES checks on the given files.
    /// Only handles checks belonging to the code-analysis crate.
    /// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
    /// have their own orchestrators called by the surface via contract aggregates.
    pub fn run_all_checks(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let mut violations: Vec<LintResult> = Vec::new();
        let mut entries: Vec<(String, String)> = Vec::new();

        // Scan Cargo.toml for workspace clippy allow bypass (AES304)
        let root_path = Path::new(root_dir);
        let mut cargo_candidates: Vec<std::path::PathBuf> = Vec::new();
        cargo_candidates.push(root_path.join("Cargo.toml"));
        if let Some(parent) = root_path.parent() {
            cargo_candidates.push(parent.join("Cargo.toml"));
        }
        for cargo_path in &cargo_candidates {
            if cargo_path.exists() {
                match shared::code_analysis::utility_file_reader::read_lintable_file(
                    &cargo_path.to_string_lossy(),
                ) {
                    Ok(Some(cargo_content)) => {
                        self.deps
                            .bypass_checker
                            .check_cargo_toml(&cargo_content, &mut violations);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        violations.push(LintResult::new_arch(
                            &cargo_path.to_string_lossy(),
                            0,
                            "AES000",
                            Severity::LOW,
                            format!("Cargo.toml skipped: {}", e),
                        ));
                    }
                }
            }
        }

        for file in files {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let c = match shared::code_analysis::utility_file_reader::read_lintable_file(file) {
                Ok(Some(content)) => content,
                Ok(None) => {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES301",
                        Severity::LOW,
                        "File skipped: exceeds maximum lintable size (2 MiB)".to_string(),
                    ));
                    continue;
                }
                Err(e) => {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES000",
                        Severity::LOW,
                        format!("File skipped: {}", e),
                    ));
                    continue;
                }
            };
            entries.push((file.clone(), c.clone()));

            // Layer-independent checks (run on ALL files)
            self.deps
                .bypass_checker
                .check_bypass_comments(file, &c, &mut violations);
            self.deps
                .dead_inheritance_checker
                .check_dead_inheritance(file, &c, &mut violations);

            if matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js") {
                continue;
            }

            // Layer detection
            let filename = extract_filename(file);
            let layer = match detect_layer_from_prefix(filename) {
                Some(l) => l,
                None => continue,
            };
            let keys = collect_layer_keys(&self.layer_map);
            let layer = LayerNameVO::new(resolve_specialized_layer(&layer, file, &keys));
            let def = match get_layer_def(&layer.value, &self.config.layers) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent checks (code-analysis only)
            self.deps
                .line_checker
                .check_line_counts(file, Some(def), &c, &mut violations);

            // Mandatory class definition check (AES303)
            self.deps.class_checker.check_mandatory_class_definition(
                file,
                Some(def),
                &c,
                &mut violations,
            );
        }

        // AES305: File-level similarity check
        let src_dir = shared::code_analysis::utility_target_resolver::detect_source_dir(
            std::path::Path::new(root_dir),
        );
        if let Ok(dp) = shared::common::taxonomy_path_vo::DirectoryPath::new(
            src_dir.to_string_lossy().to_string(),
        ) {
            for (file_path, aes_violation) in
                self.deps.duplication_checker.handle_duplicates(Some(dp))
            {
                let msg = aes_violation.to_string();
                violations.push(LintResult::new_arch(
                    &file_path,
                    1,
                    "AES305",
                    Severity::LOW,
                    msg,
                ));
            }
        }

        violations
    }

    /// Format a compliance report from results.
    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
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

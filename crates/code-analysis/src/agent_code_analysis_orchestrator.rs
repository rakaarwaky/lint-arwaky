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

use crate::CodeAnalysisCheckerContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_compliance_score::compute_score;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// Code-analysis orchestrator — collects files, runs Code Quality checks (AES301–AES305), formats reports.
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
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

    fn check_critical(&self, results: &[LintResult]) -> bool {
        let hc: fn(&[LintResult]) -> bool = has_critical;
        hc(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &FilePath) -> String {
        self.format_report(&results.values, project_root.value())
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        self.container
            .config()
            .rules
            .iter()
            .map(|r| r.code_analysis.clone())
            .collect()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for CodeAnalysisOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Run a full AES self-lint on a path.
#[rustfmt::skip]
pub fn lint_path
    (path: &str) -> Vec<LintResult> {
    let root = match FilePath::new(path.to_string()) {
        Ok(fp) => fp,
        Err(_) => match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => return Vec::new(),
        },
    };
    let orchestrator = CodeAnalysisOrchestrator::new();
    orchestrator.run_self_lint(&root.value)
}

/// Check if any CRITICAL severity violations exist in results.
#[rustfmt::skip]
pub fn has_critical
    (results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

impl CodeAnalysisOrchestrator {
    pub fn new() -> Self {
        Self {
            container: Arc::new(CodeAnalysisCheckerContainer::default()),
        }
    }

    pub fn new_with_container(container: Arc<CodeAnalysisCheckerContainer>) -> Self {
        Self { container }
    }

    /// Run AES analysis on the current project (self-lint).
    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = shared::code_analysis::utility_target::detect_source_dir(root);
        self.run_lint_at(&src_dir)
    }

    /// Run AES analysis on a specific directory.
    pub fn run_scan(&self, target_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(target_dir))
    }

    /// Core method: collect files and run all checks.
    fn run_lint_at(&self, src_dir: &Path) -> Vec<LintResult> {
        let config = self.container.config();
        let ignored: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let dir_path = match DirectoryPath::new(src_dir.to_string_lossy().to_string()) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let files = shared::code_analysis::utility_target::collect_source_files(
            src_dir, &dir_path, &ignored,
        );
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        self.run_all_checks(config, &files_str, &root_dir)
    }

    /// Run code-analysis AES checks on the given files.
    /// Only handles checks belonging to the code-analysis crate.
    /// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
    /// have their own orchestrators called by the surface via contract aggregates.
    pub fn run_all_checks(
        &self,
        config: &ArchitectureConfig,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
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
                        self.container
                            .bypass_checker()
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
            self.container
                .bypass_checker()
                .check_bypass_comments(file, &c, &mut violations);
            self.container
                .dead_inheritance_checker()
                .check_dead_inheritance(file, &c, &mut violations);

            if matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js") {
                continue;
            }

            // Layer detection
            let layer = match self.container.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };
            let def = match self.container.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent checks (code-analysis only)
            self.container
                .line_checker()
                .check_line_counts(file, Some(def), &c, &mut violations);

            // Mandatory class definition check (AES303)
            self.container
                .class_checker()
                .check_mandatory_class_definition(file, Some(def), &c, &mut violations);
        }

        // AES305: File-level similarity check (run once across all files, using pre-read entries)
        // P1.5 fix: read thresholds from config instead of hardcoding
        let min_dup_lines = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(10);
        let threshold_pct = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .and_then(|r| r.code_analysis.duplication_threshold)
            .unwrap_or(50.0);
        let dup_violations = self
            .container
            .duplication_checker()
            .check_file_similarity_entries(&entries, min_dup_lines, threshold_pct);
        for (file_path, dv) in dup_violations {
            violations.push(LintResult::new_arch(
                &file_path,
                0,
                "AES305",
                Severity::HIGH,
                dv.to_string(),
            ));
        }

        violations
    }

    /// Format a compliance report from results.
    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
        let mut output = String::new();
        output.push_str(&"=".repeat(60));
        output.push_str("\n  AES Architecture Compliance Report \n");
        output.push_str(&"=".repeat(60));
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

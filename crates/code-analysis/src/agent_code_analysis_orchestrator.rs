// PURPOSE: CodeAnalysisOrchestrator — agent that orchestrates Code Quality (AES301–AES305) checks, file collection, and reporting
// ALGORITHM (run_lint_at):
//   1. Load config; build ignored-patterns list
//   2. Recursively collect all lintable source files from src_dir (via detect_source_dir + collect_source_files)
//   3. Fail early if no files found
//   4. Create tokio runtime; run_all_checks inside block_on
// ALGORITHM (run_all_checks):
//   1. If config.enabled = false, return empty
//   2. For each file:
//      a. Read file content
//      b. Run bypass_checker.check_bypass_comments (AES304 — layer-independent)
//      c. Run dead_inheritance_checker.check_dead_inheritance (AES303 sub-check 2)
//      d. Skip barrel files (mod.rs, __init__.py, index.ts)
//      e. Detect layer from filename prefix; skip if unknown or in exception list
//      f. Run line_checker.check_line_counts (AES301–302)
//      g. Run class_checker.check_mandatory_class_definition (AES303 sub-check 1)
//   3. Return aggregated LintResult list

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::CodeAnalysisCheckerContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_score_vo::compute_score;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

static GLOBAL_CONTAINER: OnceLock<Arc<CodeAnalysisCheckerContainer>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using CodeAnalysisOrchestrator.
pub fn init_global_checker(container: Arc<CodeAnalysisCheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}

/// Detect source directory from project root (packages/, crates/, modules/).
pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    shared::source_parsing::taxonomy_file_collector_helper::collect_source_files(
        root_dir, dir_path, ignored,
    )
}

/// Code-analysis orchestrator — collects files, runs Code Quality checks (AES301–AES305), formats reports.
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
}

impl Default for CodeAnalysisOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve target path: normalize "crates" → parent, keep "." as-is, etc.
pub fn resolve_target(path: Option<String>) -> String {
    match path {
        Some(p) => p,
        None => ".".to_string(),
    }
}

/// Run a full AES self-lint on a path.
pub fn lint_path(path: &str) -> Vec<LintResult> {
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
pub fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

impl CodeAnalysisOrchestrator {
    /// Create a new orchestrator. Falls back to a default container if init_global_checker not called.
    pub fn new() -> Self {
        Self {
            container: match GLOBAL_CONTAINER.get().cloned() {
                Some(c) => c,
                None => Arc::new(CodeAnalysisCheckerContainer::default()),
            },
        }
    }

    pub fn new_with_container(container: Arc<CodeAnalysisCheckerContainer>) -> Self {
        Self { container }
    }

    /// Run AES analysis on the current project (self-lint).
    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = detect_source_dir(root);
        self.run_lint_at(&src_dir)
    }

    /// Run AES analysis on a specific directory.
    pub fn run_scan(&self, target_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(target_dir))
    }

    /// Core method: collect files and run all checks.
    fn run_lint_at(&self, src_dir: &Path) -> Vec<LintResult> {
        let config = self.container.analyzer().config();
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
        let files = collect_source_files(src_dir, &dir_path, &ignored);
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(_) => return Vec::new(),
        };
        rt.block_on(async { self.run_all_checks(config, &files_str, &root_dir).await })
    }

    /// Run code-analysis AES checks on the given files.
    /// Only handles checks belonging to the code-analysis crate.
    /// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
    /// have their own orchestrators called by the surface via contract aggregates.
    pub async fn run_all_checks(
        &self,
        config: &ArchitectureConfig,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return Vec::new();
        }
        let mut violations: Vec<LintResult> = Vec::new();

        for file in files {
            let filename = match Path::new(file).file_name().and_then(|n| n.to_str()) {
                Some(n) => n,
                None => "",
            };
            let c = match std::fs::read_to_string(file) {
                Ok(content) => content,
                Err(_) => String::new(),
            };

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

        // AES305: File-level similarity check (run once across all files)
        let min_dup_lines: usize = 5;
        let threshold_pct: f64 = 50.0;
        let dup_violations = self.container.duplication_checker().check_file_similarity(
            files,
            min_dup_lines,
            threshold_pct,
        );
        for dv in dup_violations {
            violations.push(LintResult::new_arch(
                "",
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

impl ICodeAnalysisAggregate for CodeAnalysisOrchestrator {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList {
        LintResultList::new(self.run_self_lint(project_root))
    }

    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList {
        LintResultList::new(self.run_scan(src_dir))
    }

    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult> {
        self.run_self_lint(path)
    }

    fn calc_score(&self, results: &[LintResult]) -> f64 {
        compute_score(results)
    }

    fn check_critical(&self, results: &[LintResult]) -> bool {
        has_critical(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        self.format_report(&results.values, project_root)
    }
}

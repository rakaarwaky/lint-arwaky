// PURPOSE: CodeAnalysisOrchestrator — single agent that orchestrates ALL AES checks, file collection, and reporting

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::CodeAnalysisCheckerContainer;
use async_trait::async_trait;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::config_system::taxonomy_config_vo::{default_aes_config, ArchitectureConfig};
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

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
    project_root.join("crates")
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(dir_path: &DirectoryPath) -> Vec<FilePath> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if dir_name == "target"
                    || dir_name == ".git"
                    || dir_name == ".opencode"
                    || dir_name == "node_modules"
                {
                    continue;
                }
                let sub_dir =
                    DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                files.extend(collect_source_files(&sub_dir));
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(fp) = FilePath::new(path_str.to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
    files
}

/// Unified code-analysis orchestrator — collects files, runs ALL AES checks, formats reports.
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
}

impl Default for CodeAnalysisOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeAnalysisOrchestrator {
    /// Create a new orchestrator. Panics if init_global_checker has not been called.
    pub fn new() -> Self {
        Self {
            container: GLOBAL_CONTAINER.get().cloned().unwrap_or_else(|| {
                unreachable!(
                    "init_global_checker must be called before CodeAnalysisOrchestrator::new()"
                )
            }),
        }
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
        let config = default_aes_config();
        let dir_path =
            DirectoryPath::new(src_dir.to_string_lossy().to_string()).unwrap_or_default();
        let files = collect_source_files(&dir_path);
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(_) => return Vec::new(),
        };
        rt.block_on(async { self.run_all_checks(&config, &files_str, &root_dir).await })
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
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let c = std::fs::read_to_string(file).unwrap_or_default();

            // Layer-independent checks (run on ALL files)
            self.container
                .bypass_checker()
                .check_bypass_comments(file, &c, &mut violations);
            self.container
                .inline_unused_checker()
                .check_unused_imports(file, &c, &mut violations);
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
                .mandatory_inheritance_checker()
                .check_mandatory_inheritance(file, &c, layer.value(), config, &mut violations);
            self.container
                .mandatory_inheritance_checker()
                .check_contract_implementation(file, &c, files, &mut violations);

            let fp = FilePath::new(file.to_string()).unwrap_or_default();
            let content_vo = ContentString::new(c.clone());
            let language = if file.ends_with(".rs") {
                "rust"
            } else if file.ends_with(".py") {
                "python"
            } else if file.ends_with(".js")
                || file.ends_with(".ts")
                || file.ends_with(".jsx")
                || file.ends_with(".tsx")
            {
                "javascript"
            } else {
                "unknown"
            };
            let source_vo = SourceContentVO::new(fp, content_vo, language);

            // Layer-rule checks
            self.container
                .capabilities_role_checker()
                .check_capability_routing(&source_vo, &layer, &mut violations);
            self.container
                .line_checker()
                .check_line_counts(file, Some(def), &mut violations);

            // Mandatory class definition check (AES024)
            self.container
                .class_checker()
                .check_mandatory_class_definition(file, Some(def), &mut violations);
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

#[async_trait]
impl ICodeAnalysisAggregate for CodeAnalysisOrchestrator {
    async fn run_analysis(
        &self,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let config = default_aes_config();
        self.run_all_checks(&config, files, root_dir).await
    }

    fn name(&self) -> &str {
        "code-analysis"
    }
}

impl IArchLintProtocol for CodeAnalysisOrchestrator {
    fn run_self_lint(&self, project_root: &str) -> LintResultList {
        LintResultList::new(CodeAnalysisOrchestrator::run_self_lint(self, project_root))
    }

    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList {
        LintResultList::new(self.run_scan(src_dir))
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        CodeAnalysisOrchestrator::format_report(self, &results.values, project_root)
    }
}

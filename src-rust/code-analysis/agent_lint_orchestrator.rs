// PURPOSE: Orchestrator: Orchestrates Lint
use std::path::Path;

use crate::config_system::taxonomy_config_vo::default_aes_config;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    for name in &["src-rust", "src-python", "src-javascript", "src"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.join("src-rust")
}

pub struct ArchitectureLintOrchestrator {}

impl Default for ArchitectureLintOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureLintOrchestrator {
    pub fn new() -> Self {
        let _: Option<&dyn ServiceContainerAggregate> = None;
        Self {}
    }

    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = detect_source_dir(root);
        self.run_lint_at(&src_dir, Some(root))
    }

    pub fn run_self_lint_dir(&self, src_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(src_dir), None)
    }

    fn run_lint_at(&self, src_dir: &Path, _project_root: Option<&Path>) -> Vec<LintResult> {
        let config = default_aes_config();
        let dir_path =
            DirectoryPath::new(src_dir.to_string_lossy().to_string()).unwrap_or_default();
        let files = collect_source_files(&dir_path);
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        let orchestrator =
            crate::code_analysis::agent_checking_orchestrator::LintCheckingOrchestrator::new();
        orchestrator.run_all_checks(&config, &files_str, &root_dir)
    }

    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
        let mut output = String::new();
        output.push_str(&"=".repeat(60));
        output.push_str("\n  AES Architecture Compliance Report (Self-Lint)\n");
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

impl IArchLintProtocol for ArchitectureLintOrchestrator {
    fn run_self_lint(&self, project_root: &str) -> LintResultList {
        LintResultList::new(self.run_self_lint(project_root))
    }

    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList {
        LintResultList::new(self.run_self_lint_dir(src_dir))
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        self.format_report(&results.values, project_root)
    }
}

pub struct ArchLintPipelineOrchestrator {
    inner: ArchitectureLintOrchestrator,
}

impl Default for ArchLintPipelineOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLintPipelineOrchestrator {
    pub fn new() -> Self {
        Self {
            inner: ArchitectureLintOrchestrator::new(),
        }
    }

    pub fn execute_pipeline(&self, project_root: &str) -> LintResultList {
        let results = self.inner.run_self_lint(project_root);
        LintResultList::new(results)
    }

    pub fn execute_pipeline_dir(&self, src_dir: &str) -> LintResultList {
        let results = self.inner.run_self_lint_dir(src_dir);
        LintResultList::new(results)
    }
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
fn collect_source_files(dir_path: &DirectoryPath) -> Vec<FilePath> {
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

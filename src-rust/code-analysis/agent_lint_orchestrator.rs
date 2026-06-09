use std::path::Path;

use crate::config_system::infrastructure_discovery_provider::ConfigDiscoveryProvider;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::output_report::capabilities_reporting_formatter::ReportFormatterProcessor;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::source_parsing::infrastructure_file_collector::FileCollectorProvider;

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

    fn run_lint_at(&self, src_dir: &Path, project_root: Option<&Path>) -> Vec<LintResult> {
        let config = ConfigDiscoveryProvider::load_architecture_config(project_root, src_dir);
        let collector = FileCollectorProvider::new();
        let dir_path = crate::source_parsing::taxonomy_path_vo::DirectoryPath::new(
            src_dir.to_string_lossy().to_string(),
        )
        .unwrap_or_default();
        let files = match collector.scan_directory(&dir_path) {
            Ok(list) => list.values.iter().map(|f| f.value.clone()).collect(),
            Err(_) => Vec::new(),
        };
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let coordinator =
            crate::code_analysis::agent_checking_coordinator::LintCheckingCoordinator::new();
        coordinator.run_all_checks(&config, &files, &root_dir)
    }

    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
        let formatter = ReportFormatterProcessor::new();
        formatter.format_text(results, project_root)
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

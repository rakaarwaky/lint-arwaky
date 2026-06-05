use std::path::Path;
use std::sync::Arc;

use crate::contract::source_system_port::IFileSystemPort;
use crate::contract::source_parser_port::ISourceParserPort;
use crate::taxonomy::{LintResult, LintResultList};
use crate::capabilities::architecture_lint_handler::{
collect_rs_files, load_config, format_report,
};
use crate::capabilities::architecture_compliance_analyzer::ArchComplianceAnalyzer;

pub struct ArchitectureLintOrchestrator {
fs: Arc<dyn IFileSystemPort>,
parser: Arc<dyn ISourceParserPort>,
}

impl ArchitectureLintOrchestrator {
pub fn new(fs: Arc<dyn IFileSystemPort>, parser: Arc<dyn ISourceParserPort>) -> Self {
Self { fs, parser }
}

pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
let src_dir = Path::new(project_root).join("src-rust");
self.run_lint_at(&src_dir, Some(Path::new(project_root)))
}

pub fn run_self_lint_dir(&self, src_dir: &str) -> Vec<LintResult> {
self.run_lint_at(Path::new(src_dir), None)
}

fn run_lint_at(&self, src_dir: &Path, project_root: Option<&Path>) -> Vec<LintResult> {
let config = load_config(project_root, src_dir);
let analyzer = ArchComplianceAnalyzer::new(config, self.fs.clone(), self.parser.clone());
let files = collect_rs_files(src_dir);
if files.is_empty() {
return Vec::new();
}
let root_dir = src_dir.to_string_lossy().to_string();

let mut violations = analyzer.run_analysis(&files, &root_dir);
analyzer.run_project_wide_checks(&files, &root_dir, &mut violations);
violations
}

pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
format_report(results, project_root)
}
}

pub struct ArchLintPipelineOrchestrator {
inner: ArchitectureLintOrchestrator,
}

impl ArchLintPipelineOrchestrator {
pub fn new(fs: Arc<dyn IFileSystemPort>, parser: Arc<dyn ISourceParserPort>) -> Self {
Self {
inner: ArchitectureLintOrchestrator::new(fs, parser),
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

pub fn generate_report(&self, results: &LintResultList, project_root: &str) -> String {
self.inner.format_report(&results.results, project_root)
}
}


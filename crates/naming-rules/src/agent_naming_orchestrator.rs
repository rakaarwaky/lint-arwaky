// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use crate::ArchNamingChecker;
use async_trait::async_trait;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::output_report::taxonomy_result_vo::{LintResult, LintResultList};
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use std::path::Path;
use std::sync::Arc;

pub struct NamingOrchestrator {
    checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
}

impl NamingOrchestrator {
    pub fn new(checker: ArchNamingChecker, analyzer: Arc<dyn IAnalyzer>) -> Self {
        Self {
            checker: Arc::new(checker),
            analyzer,
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files);
        } else if path.is_file() {
            files.push(FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default());
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.walk_dir(&path, files);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            files.push(
                                FilePath::new(path.to_string_lossy().to_string())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let root_dir = FilePath::new(target.value().split('/').next().unwrap_or(".").to_string())
            .unwrap_or_default();

        self.checker
            .check_file_naming(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.checker
            .check_domain_suffixes(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

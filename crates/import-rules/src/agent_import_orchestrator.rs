// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
use import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use import_rules::contract_rule_protocol::{IAnalyzer, IArchImportProtocol};
use output_report::taxonomy_result_vo::{LintResult, LintResultList};
use source_parsing::taxonomy_path_vo::FilePath;
use source_parsing::FilePathList;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
}

impl ImportOrchestrator {
    pub fn new(
        mandatory: Arc<dyn IArchImportProtocol>,
        forbidden: Arc<dyn IArchImportProtocol>,
        intent: Arc<dyn IArchImportProtocol>,
        analyzer: Arc<dyn IAnalyzer>,
    ) -> Self {
        Self {
            mandatory,
            forbidden,
            intent,
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
                        if matches!(ext.to_str(), Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")) {
                            files.push(FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default());
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let root_dir = FilePath::new(
            target
                .value()
                .split('/')
                .next()
                .unwrap_or(".")
                .to_string(),
        )
        .unwrap_or_default();

        self.mandatory
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.forbidden
            .check_forbidden_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.forbidden
            .check_legacy_import_rules(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.intent
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}
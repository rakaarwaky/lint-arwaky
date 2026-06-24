// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, IArchImportProtocol};
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use std::path::Path;
use std::sync::Arc;

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleAnalysisProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
    ignored_paths: Vec<String>,
}

impl ImportOrchestrator {
    pub fn new(
        mandatory: Arc<dyn IArchImportProtocol>,
        forbidden: Arc<dyn IArchImportProtocol>,
        intent: Arc<dyn IArchImportProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleAnalysisProtocol>,
        analyzer: Arc<dyn IAnalyzer>,
    ) -> Self {
        let config = analyzer.config();
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            mandatory,
            forbidden,
            intent,
            unused,
            cycle,
            analyzer,
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = match p.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => String::new(),
        };
        shared::source_parsing::taxonomy_file_collector_helper::is_path_ignored(
            &s,
            &self.ignored_paths,
        ) || match dir_name.strip_prefix('.') {
            Some(n) => self.ignored_paths.iter().any(|i| i.contains(n)),
            None => false,
        }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
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
        let first_component = target.value().split('/').next().unwrap_or(".");
        let root_dir = FilePath::new(first_component.to_string()).unwrap_or_default();

        self.mandatory
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.forbidden
            .check_forbidden_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.intent
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        // AES203: unused import check - read file content once and check all languages
        for file in files.iter() {
            let file_path = file.value();
            if let Ok(content) = std::fs::read_to_string(file_path) {
                self.unused
                    .check_unused_imports(file_path, &content, &mut results.values);
            }
        }

        // AES205: circular dependency / cycle detection
        self.cycle
            .check_cycles(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}

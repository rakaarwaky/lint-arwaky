// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks via contract ports
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_common_vo::PatternList;
use std::sync::Arc;

pub struct NamingOrchestrator {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn INamingAnalyzerProtocol>,
    fs: Arc<dyn INamingFileSystemPort>,
    ignored_patterns: PatternList,
}

impl NamingOrchestrator {
    pub fn new(
        naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
        suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
        analyzer: Arc<dyn INamingAnalyzerProtocol>,
        fs: Arc<dyn INamingFileSystemPort>,
    ) -> Self {
        let config = analyzer.config();
        let ignored_patterns = PatternList {
            values: config
                .ignored_paths
                .values
                .iter()
                .map(|fp| {
                    fp.value
                        .trim_start_matches("./")
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .to_string()
                })
                .collect(),
        };
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
            fs,
            ignored_patterns,
        }
    }

    fn filter_source_files(files: &FilePathList) -> FilePathList {
        let source_exts = ["rs", "py", "js", "ts", "jsx", "tsx"];
        let filtered: Vec<FilePath> = files
            .values
            .iter()
            .filter(|f| match f.value.rsplit('.').next() {
                Some(ext) => source_exts.contains(&ext),
                None => false,
            })
            .cloned()
            .collect();
        FilePathList::new(filtered)
    }
}

#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let all_files = self.fs.walk(target, Some(&self.ignored_patterns)).await;
        let files = Self::filter_source_files(&all_files);
        let root_dir = target.clone();

        self.naming_convention_checker
            .check_file_naming(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.suffix_prefix_checker
            .check_domain_suffixes(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

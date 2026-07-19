// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks via contract ports
//
// Orchestrates 2 naming-related AES rules:
//   1. AES101: file suffix convention (e.g., `*_vo.rs` for value objects)
//   2. AES102: filename pattern matching (e.g., snake_case for Rust files)
//
// The orchestrator walks the target directory, filters to source files,
// then delegates to two checkers. Both checkers implement the same
// INamingCheckerProtocol trait but are configured with different rules.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_common_vo::PatternList;
use std::path::Path;
use std::sync::Arc;

/// Naming orchestrator — the agent layer for naming convention enforcement.
///
/// Dependencies:
///   - naming_convention_checker: AES102 — filename pattern matching
///   - suffix_prefix_checker: AES101 — file suffix convention checks
///   - analyzer: provides layer configuration and detection
///   - fs: filesystem access for walking directories
///   - ignored_patterns: paths to skip during file collection
// ─── Block 1: Struct Definition ───────────────────────────
pub struct NamingOrchestrator {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn ILayerDetectionProtocol>,
    fs: Arc<dyn INamingFileSystemPort>,
    ignored_patterns: PatternList,
}

// ─── Block 2: Public Contract ─────────────────────────────
#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    /// Run both naming convention checks (AES101 + AES102) on the target.
    ///
    /// Orchestration flow:
    ///   1. Walk target directory (via fs port, skipping ignored paths)
    ///   2. Filter to source files only
    ///   3. Run naming_convention_checker.check_file_naming (AES102)
    ///   4. Run suffix_prefix_checker.check_domain_suffixes (AES101)
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let config = self.analyzer.config();
        if !config.enabled.value {
            return Vec::new();
        }

        let mut results = LintResultList::new(Vec::new());
        let all_files = self.fs.walk(target, Some(&self.ignored_patterns)).await;
        let files = Self::filter_source_files(&all_files);
        let root_dir = target;

        if config.is_rule_enabled("AES101") || config.is_rule_enabled("AES102") {
            self.naming_convention_checker
                .check_file_naming(self.analyzer.as_ref(), &files, root_dir, &mut results)
                .await;
        }
        if config.is_rule_enabled("AES102") {
            self.suffix_prefix_checker
                .check_domain_suffixes(self.analyzer.as_ref(), &files, root_dir, &mut results)
                .await;
        }

        results.values
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl NamingOrchestrator {
    /// Constructor: builds the orchestrator with injected dependencies.
    /// Pre-processes ignored patterns from config (normalize paths).
    pub fn new(
        naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
        suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
        analyzer: Arc<dyn ILayerDetectionProtocol>,
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

    /// Filter to source code files only (.rs, .py, .js, .ts, .jsx, .tsx).
    pub fn filter_source_files(files: &FilePathList) -> FilePathList {
        let filtered: Vec<FilePath> = files
            .values
            .iter()
            .filter(|f| {
                let path = Path::new(&f.value);
                matches!(
                    path.extension().and_then(|e| e.to_str()),
                    Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                )
            })
            .cloned()
            .collect();
        FilePathList::new(filtered)
    }
}

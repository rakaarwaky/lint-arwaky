// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_naming_rules::contract_naming_checker_protocol::{
    INamingConventionProtocol, ISuffixPolicyProtocol,
};
use shared_naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared_naming_rules::contract_test_file_prefix_protocol::ITestFilePrefixProtocol;
use shared_naming_rules::taxonomy_naming_audit_stats_vo::NamingAuditStats;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use std::sync::Arc;

// ─── Block 1: Module Declaration ──────────────────────────────

pub struct NamingOrchestratorDeps {
    pub naming_convention: Arc<dyn INamingConventionProtocol>,
    pub suffix_policy: Arc<dyn ISuffixPolicyProtocol>,
    pub test_file_prefix: Arc<dyn ITestFilePrefixProtocol>,
    pub config: Arc<ArchitectureConfig>,
    pub layer_map: Arc<LayerMapVO>,
}

pub struct NamingOrchestrator {
    deps: NamingOrchestratorDeps,
}

impl NamingOrchestrator {
    pub fn new(deps: NamingOrchestratorDeps) -> Self {
        Self { deps }
    }

    pub fn name(&self) -> &str {
        "naming-rules"
    }

    /// Run audit on pre-parsed file entries from the filesystem crate.
    fn run_audit_with_entries(
        &self,
        files: &[FileEntry],
    ) -> (Vec<ViolationItem>, NamingAuditStats) {
        self.run_audit_over(
            &FilePathList::new(Self::paths_of(files)),
            &FilePathList::default(),
        )
    }

    /// AES101/AES102 read *source*; AES103 reads *tests*. Both file sets come
    /// from the caller because only the filesystem walk knows which directories
    /// the default skip list prunes.
    fn run_audit_over(
        &self,
        source: &FilePathList,
        tests: &FilePathList,
    ) -> (Vec<ViolationItem>, NamingAuditStats) {
        // Naming checks are path-only: the rules read the file name, never the
        // body. A file with no content still has a name, so it is judged like
        // any other — dropping it here would let a zero-byte file with a
        // broken name pass as clean, which is the one state a naming audit
        // must never report as clean.
        let root = FilePath::new(".".to_string()).unwrap_or_default();
        self.run_checks(source, tests, &root)
    }

    /// The path of every entry, as the rule checkers read them.
    ///
    /// Every entry counts, whatever its content: a file whose body is empty
    /// still carries a name, and the name is what these rules judge. Only a
    /// path the path type rejects is dropped, because a rule cannot read it.
    /// The function lives on the orchestrator because AES405 forbids a free
    /// function in an agent file.
    fn paths_of(files: &[FileEntry]) -> Vec<FilePath> {
        files
            .iter()
            .filter_map(|f| FilePath::new(f.path.to_string_lossy().to_string()).ok())
            .collect()
    }

    /// Check if a specific AES rule is enabled in the configuration.
    /// Returns true if the rule is found and enabled, or if not found (default enabled).
    fn is_rule_enabled(config: &ArchitectureConfig, rule_code: &str) -> bool {
        config
            .rules
            .iter()
            .find(|r| r.rule_type.code() == rule_code)
            .is_none_or(|r| r.enabled.value)
    }

    fn run_checks(
        &self,
        files: &FilePathList,
        test_files: &FilePathList,
        root_dir: &FilePath,
    ) -> (Vec<ViolationItem>, NamingAuditStats) {
        let mut results: Vec<ViolationItem> = Vec::new();
        let mut stats = NamingAuditStats::default();

        if Self::is_rule_enabled(&self.deps.config, "AES101") {
            let mut naming_results = Vec::<ViolationItem>::new();
            self.deps.naming_convention.check_file_naming(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut naming_results,
            );
            stats.aes101_inspected = files.values.len();
            results.extend(naming_results);
        }

        if Self::is_rule_enabled(&self.deps.config, "AES102") {
            let mut suffix_results = Vec::<ViolationItem>::new();
            self.deps.suffix_policy.check_domain_suffixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut suffix_results,
            );
            stats.aes102_inspected = files.values.len();
            results.extend(suffix_results);
        }

        if Self::is_rule_enabled(&self.deps.config, "AES103") {
            let mut prefix_results = Vec::<ViolationItem>::new();
            self.deps.test_file_prefix.check_test_file_prefixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                test_files,
                root_dir,
                &mut prefix_results,
            );
            stats.aes103_inspected = test_files.values.len();
            results.extend(prefix_results);
        }

        (results, stats)
    }
}
// ─── Block 2: Module Contract ──────────────

impl INamingRunnerAggregate for NamingOrchestrator {
    fn execute(&self, request: NamingRequest) -> NamingResponse {
        match request {
            NamingRequest::RunAuditWithEntries { files } => {
                let (violations, stats) = self.run_audit_with_entries(&files);
                NamingResponse::Audit { violations, stats }
            }
            NamingRequest::RunAuditWithTestEntries {
                source_files,
                test_files,
            } => {
                let (violations, stats) = self.run_audit_over(
                    &FilePathList::new(Self::paths_of(&source_files)),
                    &FilePathList::new(Self::paths_of(&test_files)),
                );
                NamingResponse::Audit { violations, stats }
            }
            NamingRequest::Name => NamingResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Module Helpers ──────────────

// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_lint_vo::{LintResult, LintResultList};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_naming_rules::contract_naming_checker_protocol::{
    INamingConventionProtocol, ISuffixPolicyProtocol,
};
use shared_naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared_naming_rules::contract_test_file_prefix_protocol::ITestFilePrefixProtocol;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

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

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl INamingRunnerAggregate for NamingOrchestrator {
    fn execute(&self, request: NamingRequest) -> NamingResponse {
        match request {
            NamingRequest::RunAuditWithEntries { files } => NamingResponse::Audit {
                violations: self.run_audit_with_entries(&files),
            },
            NamingRequest::RunAuditWithTestEntries {
                source_files,
                test_files,
            } => NamingResponse::Audit {
                violations: self.run_audit_over(
                    &FilePathList::new(Self::paths_of(&source_files)),
                    &FilePathList::new(Self::paths_of(&test_files)),
                ),
            },
            NamingRequest::Name => NamingResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl NamingOrchestrator {
    pub fn new(deps: NamingOrchestratorDeps) -> Self {
        Self { deps }
    }

    pub fn name(&self) -> &str {
        "naming-rules"
    }

    /// Run audit on pre-parsed file entries from the filesystem crate.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        self.run_audit_over(
            &FilePathList::new(Self::paths_of(files)),
            &FilePathList::default(),
        )
    }

    /// AES101/AES102 read *source*; AES103 reads *tests*. Both file sets come
    /// from the caller because only the filesystem walk knows which directories
    /// the default skip list prunes.
    fn run_audit_over(&self, source: &FilePathList, tests: &FilePathList) -> Vec<LintResult> {
        // Naming checks are path-only — do NOT skip parse failures.
        // `content.is_empty()` is used as a proxy for "unreadable" per the FRD glossary.
        // If the filesystem crate adds a separate error field in the future,
        // this filter should also check `parse_ok == false`.
        let root = FilePath::new(".".to_string()).unwrap_or_default();
        self.run_checks(source, tests, &root)
    }

    /// The paths of every entry carrying content, as the rule checkers read them.
    ///
    /// The `content.is_empty()` guard is the FRD's "unreadable" proxy; a file
    /// with no body cannot be judged, and reporting on it would only produce
    /// noise. It lives on the orchestrator because AES405 forbids a free
    /// function in an agent file.
    fn paths_of(files: &[FileEntry]) -> Vec<FilePath> {
        files
            .iter()
            .filter(|f| !f.content.is_empty())
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
    ) -> Vec<LintResult> {
        let mut results: Vec<LintResult> = Vec::new();

        if Self::is_rule_enabled(&self.deps.config, "AES101") {
            let mut naming_results = LintResultList::new(Vec::new());
            self.deps.naming_convention.check_file_naming(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut naming_results,
            );
            results.extend(naming_results.values);
        }

        if Self::is_rule_enabled(&self.deps.config, "AES102") {
            let mut suffix_results = LintResultList::new(Vec::new());
            self.deps.suffix_policy.check_domain_suffixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                files,
                root_dir,
                &mut suffix_results,
            );
            results.extend(suffix_results.values);
        }

        if Self::is_rule_enabled(&self.deps.config, "AES103") {
            let mut prefix_results = LintResultList::new(Vec::new());
            self.deps.test_file_prefix.check_test_file_prefixes(
                self.deps.config.as_ref(),
                self.deps.layer_map.as_ref(),
                test_files,
                root_dir,
                &mut prefix_results,
            );
            results.extend(prefix_results.values);
        }

        results
    }
}

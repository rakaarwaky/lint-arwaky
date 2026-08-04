// E2E tests — full pipeline: detect languages → select adapters → verify adapter names.
//
// These tests simulate the complete flow that ExternalLintOrchestrator follows:
// language detection → adapter selection → adapter name verification.
// We don't actually run the linters (that would require installed tools),
// but we verify the end-to-end wiring from detection to selection.

use std::collections::HashMap;
use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    ExternalLintDeps, ExternalLintOrchestrator,
};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

// ─── Mocks ────────────────────────────────────────────────

struct MockFilesystem {
    has_rs: bool,
    has_py: bool,
    has_js: bool,
}

impl shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate for MockFilesystem {
    fn file_list(&self) -> &[shared::filesystem::taxonomy_filesystem_vo::FileEntry] {
        &[]
    }
    fn read_cached(&self, _: &FilePath) -> shared::common::taxonomy_source_vo::ContentString {
        shared::common::taxonomy_source_vo::ContentString::new("")
    }
    fn get_file_content(&self, _: &std::path::Path) -> Option<String> {
        None
    }
    fn has_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn collect_file_entries(&self, _: &[String]) -> Vec<(std::path::PathBuf, String)> {
        vec![]
    }
    fn discover_source_files(&self, _: &std::path::Path, _: &[String]) -> Vec<String> {
        vec![]
    }
    fn read_file(&self, _: &std::path::Path) -> Option<String> {
        None
    }
    fn scan_directory(&self, _: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn discover_files(&self, _: &std::path::Path) -> Vec<String> {
        let mut files = Vec::new();
        if self.has_rs {
            files.push("main.rs".to_string());
        }
        if self.has_py {
            files.push("app.py".to_string());
        }
        if self.has_js {
            files.push("index.ts".to_string());
        }
        files
    }
    fn collect_source_files(&self, _: &std::path::Path, _: &[String]) -> Vec<FilePath> {
        vec![]
    }
    fn read_lintable_file(&self, _: &str) -> Option<String> {
        None
    }
    fn used_identifiers_for(&self, _: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn build_file_index(&self, _: &std::path::Path) {}
    fn build_file_index_with_ignored(&self, _: &std::path::Path, _: &[String]) {}
    fn build_orphan_graph_context(
        &self,
        _: &std::path::Path,
        _: &[String],
    ) -> shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext {
        shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext::new(
            shared::filesystem::taxonomy_filesystem_vo::ImportGraph::new(
                std::collections::HashMap::new(),
            ),
            shared::filesystem::taxonomy_filesystem_vo::InboundLinkMap::new(
                std::collections::HashMap::new(),
            ),
            shared::filesystem::taxonomy_filesystem_vo::InheritanceMap::new(
                std::collections::HashMap::new(),
            ),
            vec![],
        )
    }
}

use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_language_vo::Language;
use shared::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry as FE, FileExtension, ImplEntry, ImportEntry, ParseWarning,
    ScanTiming, ToolName,
};
impl shared::filesystem::contract_parser_protocol::IParserProtocol for MockFilesystem {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &[]
    }
    fn import_list(&self) -> Vec<ImportEntry> {
        Vec::new()
    }
    fn parse_all(&self, _: &mut [FE]) {}
    fn imports_for(&self, _: &std::path::Path) -> Vec<ImportEntry> {
        vec![]
    }
    fn extract(&self, _: &std::path::Path, _: &str, _: Language) -> Vec<ImportEntry> {
        vec![]
    }
    fn resolve_barrel_imports(&self, _: &std::path::Path) {}
}
impl shared::filesystem::contract_graph_protocol::IGraphProtocol for MockFilesystem {
    fn build_graph(&self, _: &[ImportEntry], _: &[FE], _: &[DefinitionEntry], _: &[ImplEntry]) {}
    fn symbol_definitions(&self) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
        todo!()
    }
    fn implementations(&self) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
        todo!()
    }
    fn dependents(&self, _: &std::path::Path) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn dependencies(&self, _: &std::path::Path) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn reachable(&self, _: &std::path::Path, _: &std::path::Path) -> bool {
        false
    }
    fn reverse_links(
        &self,
    ) -> &std::collections::HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> {
        todo!()
    }
}
impl shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol for MockFilesystem {
    fn workspace_root(&self, _: &FilePath) -> Option<std::path::PathBuf> {
        None
    }
    fn find_workspace_root_from_path(
        &self,
        _: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn is_member_path(&self, _: &FilePath) -> bool {
        false
    }
    fn is_leaf_member_path(&self, _: &FilePath) -> bool {
        false
    }
    fn detect_source_dir(&self, _: &std::path::Path) -> std::path::PathBuf {
        todo!()
    }
    fn detect_language_from_path(&self, _: &str) -> ConfigLanguage {
        todo!()
    }
    fn check_wired_in_container(&self, _: &std::path::Path, _: &[String]) -> bool {
        false
    }
    fn resolve_orphan_module_path(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
        _: &str,
    ) -> Option<std::path::PathBuf> {
        None
    }
}
impl shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol
    for MockFilesystem
{
    fn is_executable_in_path(&self, _: &ToolName) -> bool {
        false
    }
    fn is_binary_available(&self, _: &ToolName) -> bool {
        false
    }
    fn has_local_bin(&self, _: &std::path::Path, _: &ToolName) -> bool {
        false
    }
    fn resolve_js_cmd(&self, _: &ToolName, _: Vec<String>, _: &FilePath) -> Option<Vec<String>> {
        None
    }
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn has_config_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn has_cargo_toml(&self, _: &FilePath) -> Option<FilePath> {
        None
    }
    fn has_cargo_lock(&self, _: &FilePath) -> Option<FilePath> {
        None
    }
    fn is_python_file_recursive(&self, _: &FilePath) -> bool {
        false
    }
    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
}
impl shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol for MockFilesystem {
    fn path_exists(&self, _: &std::path::Path) -> bool {
        false
    }
    fn is_dir(&self, _: &std::path::Path) -> bool {
        false
    }
    fn is_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn should_ignore(&self, _: &FilePath, _: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, _: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn canonicalize_path_str(&self, _: &FilePath) -> String {
        String::new()
    }
    fn is_symlink(&self, _: &std::path::Path) -> bool {
        false
    }
    fn metadata(&self, _: &std::path::Path) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn symlink_metadata(&self, _: &std::path::Path) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_source_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn is_source_ext(&self, _: &FileExtension) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_python_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _: &std::path::Path,
        _: &[String],
    ) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _: &std::path::Path, _: &[String]) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(
        &self,
        _: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(&self, _: &std::path::Path) -> Result<String, std::io::Error> {
        Ok(String::new())
    }
    fn write_string(&self, _: &std::path::Path, _: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn copy_file(&self, _: &std::path::Path, _: &std::path::Path) -> Result<u64, std::io::Error> {
        Ok(0)
    }
    fn create_dir_all(&self, _: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_permissions(&self, _: &std::path::Path, _: u32) -> std::io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _: &std::path::Path) -> std::io::Result<()> {
        Ok(())
    }
    fn run_git_command(&self, _: &[&str], _: &str) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn parse_output_lines(&self, _: &str) -> Vec<String> {
        vec![]
    }
    fn run_external_command_in(&self, _: &str, _: &[&str], _: &str) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn timing(&self) -> &ScanTiming {
        todo!()
    }
}

struct MockLintExecutor;
impl IExternalLintExecutorProtocol for MockLintExecutor {
    fn exec_cmd_scan(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &str,
        _: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

struct MockConfigParser;
impl shared::config_system::contract_parser_protocol::IConfigParserProtocol for MockConfigParser {
    fn parse_yaml_config(
        &self,
        _: &FilePath,
    ) -> Result<
        shared::config_system::taxonomy_setting_vo::ProjectConfig,
        shared::config_system::taxonomy_config_error::ConfigError,
    > {
        Err(shared::config_system::taxonomy_config_error::ConfigError::default())
    }
    fn parse_toml_config(
        &self,
        _: &FilePath,
    ) -> Result<
        Option<shared::config_system::taxonomy_setting_vo::ProjectConfig>,
        shared::config_system::taxonomy_config_error::ConfigError,
    > {
        Ok(None)
    }
    fn parse_config_yaml_with_warnings(
        &self,
        _: &str,
    ) -> (
        shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        Vec<String>,
    ) {
        (
            shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
            vec![],
        )
    }
    fn parse_adapter_entries_from_yaml(
        &self,
        _: &str,
    ) -> Vec<shared::config_system::taxonomy_setting_vo::AdapterEntry> {
        vec![]
    }
}

// ─── E2E: Rust-only project ───────────────────────────────

#[test]
fn e2e_rust_only_project_selects_clippy_rustfmt_audit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

// ─── E2E: Python-only project ─────────────────────────────

#[test]
fn e2e_python_only_project_selects_ruff_mypy_bandit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["ruff", "mypy", "bandit"]);
}

// ─── E2E: JS-only project ─────────────────────────────────

#[test]
fn e2e_js_only_project_selects_eslint_prettier_tsc() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["eslint", "prettier", "tsc"]);
}

// ─── E2E: Mixed project ───────────────────────────────────

#[test]
fn e2e_mixed_project_selects_all_nine() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
}

// ─── E2E: No languages detected ───────────────────────────

#[test]
fn e2e_no_languages_detected_selects_nothing() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false);
    assert!(selected.is_empty());
}

// ─── E2E: Full pipeline — Rust+Python project ─────────────

#[test]
fn e2e_full_pipeline_rust_python() {
    // Step 1: Select adapters (simulating language detection)
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, false);
    let selected_names: Vec<String> = selected.iter().map(|a| a.value().to_string()).collect();

    // Step 2: Build orchestrator with matching adapters
    let lint_exec: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(MockLintExecutor);
    let fs_arc: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem {
            has_rs: true,
            has_py: true,
            has_js: false,
        });

    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    for name in &selected_names {
        match name.as_str() {
            "ruff" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::RuffAdapter::new(
                        lint_exec.clone(),
                        None,
                        fs_arc.clone(),
                    )),
                );
            }
            "mypy" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::MyPyAdapter::new(
                        lint_exec.clone(),
                        None,
                        fs_arc.clone(),
                    )),
                );
            }
            "bandit" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::BanditAdapter::new(
                        lint_exec.clone(),
                        None,
                        fs_arc.clone(),
                    )),
                );
            }
            // Skip Rust adapters (need ICommandExecutorProtocol mock)
            _ => {}
        }
    }

    // Step 3: Verify adapter_names matches
    let deps = ExternalLintDeps {
        adapters,
        filesystem: Arc::new(MockFilesystem {
            has_rs: true,
            has_py: true,
            has_js: false,
        }),
        config_parser: Arc::new(MockConfigParser),
        selector: Arc::new(
            external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults(),
        ),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let adapter_names = orchestrator.adapter_names();

    let registered: Vec<&str> = adapter_names.iter().map(|a| a.value()).collect();
    assert!(registered.contains(&"ruff"));
    assert!(registered.contains(&"mypy"));
    assert!(registered.contains(&"bandit"));

    // Step 4: scan_all with empty adapters (no Rust ones registered)
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path);
    assert!(results.values.is_empty()); // adapters scan empty dirs → no findings
}

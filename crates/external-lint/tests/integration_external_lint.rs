// Integration tests — verify individual adapter creation and selector wiring.
//
// These tests construct real adapters with mock executors/filesystems and verify
// that the subsystem wires together correctly. ExternalLintContainer::default()
// panics, so we test individual component creation instead.

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// ─── Mocks ────────────────────────────────────────────────

struct MockFilesystem;
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
        vec![]
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
    fn import_list(&self) -> &[ImportEntry] {
        &[]
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

struct MockCmdExecutor;
impl shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol
    for MockCmdExecutor
{
    fn execute_command(
        &self,
        _: shared::common::taxonomy_common_vo::PatternList,
        _: FilePath,
        _: Option<shared::common::taxonomy_duration_vo::Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
}

// ─── Integration: Individual adapter creation ─────────────

#[test]
fn create_ruff_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Python files in /tmp with mock filesystem
}

#[test]
fn create_bandit_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_mypy_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_clippy_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RustLinterAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Cargo.toml at /tmp
}

#[test]
fn create_rustfmt_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RustFmtAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_cargo_audit_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::CargoAuditAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Cargo.lock at /tmp
}

#[test]
fn create_eslint_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::ESLintAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_prettier_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_tsc_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

// ─── Integration: Selector with defaults ──────────────────

#[test]
fn selector_with_defaults_selects_all_nine_for_mixed_project() {
    use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9);
}

#[test]
fn selector_with_custom_lists() {
    use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

    let selector = CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("clippy")],
        vec![AdapterName::raw("ruff")],
        vec![],
    );
    let selected = selector.select_adapters(true, true, false);
    assert_eq!(selected.len(), 2); // only rust + python, no js
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"ruff"));
}

// ─── Integration: apply_fix on adapters ───────────────────

#[test]
fn bandit_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // bandit doesn't fix
}

#[test]
fn mypy_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // mypy doesn't fix
}

#[test]
fn tsc_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // tsc doesn't fix
}

#[test]
fn ruff_apply_fix_returns_true() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(status.value); // ruff --fix exits 0 → true
}

#[test]
fn cargo_audit_apply_fix_returns_true() {
    // cargo-audit has no fix, but returns true (manual update needed)
    let adapter = external_lint_lint_arwaky::CargoAuditAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(status.value);
}

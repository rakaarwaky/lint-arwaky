// Unit tests — collect_scan with nonexistent path returns error.
use dispatcher_lint_arwaky::surface_check_action::{ScanOptions, collect_scan};
use shared::common::FilePath;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use std::sync::Arc;

/// Minimal mock filesystem that does canonicalize by returning the path as-is.
struct MinimalFilesystem;

impl shared::filesystem::contract_parser_protocol::IParserProtocol for MinimalFilesystem {
    fn parse_warnings(&self) -> &[shared::filesystem::taxonomy_filesystem_vo::ParseWarning] {
        &[]
    }
    fn import_list(&self) -> Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry> {
        vec![]
    }
    fn parse_all(&self, _: &mut [shared::filesystem::taxonomy_filesystem_vo::FileEntry]) {}
    fn imports_for(
        &self,
        _: &std::path::Path,
    ) -> Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry> {
        vec![]
    }
    fn extract(
        &self,
        _: &std::path::Path,
        _: &str,
        _: shared::common::taxonomy_language_vo::Language,
    ) -> Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry> {
        vec![]
    }
    fn resolve_barrel_imports(&self, _: &std::path::Path) {}
}

impl shared::filesystem::contract_graph_protocol::IGraphProtocol for MinimalFilesystem {
    fn build_graph(
        &self,
        _: &[shared::filesystem::taxonomy_filesystem_vo::ImportEntry],
        _: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
        _: &[shared::filesystem::taxonomy_filesystem_vo::DefinitionEntry],
        _: &[shared::filesystem::taxonomy_filesystem_vo::ImplEntry],
    ) {
    }
    fn symbol_definitions(&self) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
        use std::collections::HashMap;
        static EMPTY: std::sync::OnceLock<HashMap<String, Vec<std::path::PathBuf>>> =
            std::sync::OnceLock::new();
        EMPTY.get_or_init(HashMap::new)
    }
    fn implementations(&self) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
        use std::collections::HashMap;
        static EMPTY: std::sync::OnceLock<HashMap<String, Vec<std::path::PathBuf>>> =
            std::sync::OnceLock::new();
        EMPTY.get_or_init(HashMap::new)
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
        use std::collections::HashMap;
        static EMPTY: std::sync::OnceLock<HashMap<std::path::PathBuf, Vec<std::path::PathBuf>>> =
            std::sync::OnceLock::new();
        EMPTY.get_or_init(HashMap::new)
    }
}

impl shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol for MinimalFilesystem {
    fn workspace_root(&self, start: &FilePath) -> Option<std::path::PathBuf> {
        std::path::Path::new(start.value())
            .parent()
            .map(|p| p.to_path_buf())
    }
    fn find_workspace_root_from_path(
        &self,
        start: &std::path::Path,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        Ok(start.to_path_buf())
    }
    fn is_member_path(&self, _: &FilePath) -> bool {
        false
    }
    fn is_leaf_member_path(&self, _: &FilePath) -> bool {
        false
    }
    fn detect_source_dir(&self, project_root: &std::path::Path) -> std::path::PathBuf {
        project_root.join("src")
    }
    fn detect_language_from_path(
        &self,
        _: &str,
    ) -> shared::common::taxonomy_config_language_vo::ConfigLanguage {
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust
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
    for MinimalFilesystem
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

impl shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol
    for MinimalFilesystem
{
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
    fn canonicalize(&self, path: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(&self, path: &FilePath) -> String {
        path.value.clone()
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
        path.rsplit('/').next().unwrap_or(path)
    }
    fn is_source_file(&self, _: &std::path::Path) -> bool {
        false
    }
    fn is_source_ext(&self, _: &shared::filesystem::taxonomy_filesystem_vo::FileExtension) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit('/').next().unwrap_or(path)
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit('/').nth(1).unwrap_or(path)
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
    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        output.lines().map(String::from).collect()
    }
    fn run_external_command_in(&self, _: &str, _: &[&str], _: &str) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn timing(&self) -> &shared::filesystem::taxonomy_filesystem_vo::ScanTiming {
        static TIMING: shared::filesystem::taxonomy_filesystem_vo::ScanTiming =
            shared::filesystem::taxonomy_filesystem_vo::ScanTiming {
                walk_ms: 0,
                cache_ms: 0,
                parse_ms: 0,
                extract_ms: 0,
                graph_ms: 0,
                total_ms: 0,
            };
        &TIMING
    }
}

impl shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate for MinimalFilesystem {
    fn file_list(&self) -> &[shared::filesystem::taxonomy_filesystem_vo::FileEntry] {
        &[]
    }
    fn read_cached(&self, _: &FilePath) -> shared::common::taxonomy_source_vo::ContentString {
        Default::default()
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
    fn implemented_traits_map(&self) -> std::collections::HashMap<String, Vec<String>> {
        std::collections::HashMap::new()
    }
    fn build_file_index(&self, _: &std::path::Path) {}
    fn build_file_index_with_ignored(&self, _: &std::path::Path, _: &[String]) {}
    fn build_orphan_graph_context(
        &self,
        _: &std::path::Path,
        _: &[String],
    ) -> shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext {
        use std::collections::HashMap;
        shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext::new(
            shared::filesystem::taxonomy_filesystem_vo::ImportGraph::new(HashMap::new()),
            shared::filesystem::taxonomy_filesystem_vo::InboundLinkMap::new(HashMap::new()),
            shared::filesystem::taxonomy_filesystem_vo::InheritanceMap::new(HashMap::new()),
            vec![],
        )
    }
    fn find_workspace_root(&self, _: &std::path::Path) -> Option<std::path::PathBuf> {
        None
    }
}

fn mock_fs() -> Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> {
    Arc::new(MinimalFilesystem)
}

#[test]
fn collect_scan_nonexistent_path_returns_error() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: mock_fs(),
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("does not exist"));
}

#[test]
fn collect_scan_with_filter_returns_error_for_nonexistent() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: Some("AES".to_string()),
        member: None,
        filesystem: mock_fs(),
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
}

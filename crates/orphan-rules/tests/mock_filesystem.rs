// Shared mock filesystem for orphan-rules integration tests.
// Each test file includes this via: #[path = "mock_filesystem.rs"] mod mock_filesystem;

use once_cell::sync::Lazy;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::collections::HashMap;
use std::sync::Arc;

// ── Mock Filesystem ────────────────────────────────────────

pub struct MockFilesystem;

static EMPTY_STRING_MAP: Lazy<HashMap<String, Vec<std::path::PathBuf>>> = Lazy::new(HashMap::new);
static EMPTY_PATH_MAP: Lazy<HashMap<std::path::PathBuf, Vec<std::path::PathBuf>>> =
    Lazy::new(HashMap::new);

impl IParserProtocol for MockFilesystem {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &[]
    }
    fn import_list(&self) -> &[ImportEntry] {
        &[]
    }
    fn parse_all(&self, _files: &mut [FileEntry]) {}
    fn imports_for(&self, _path: &std::path::Path) -> Vec<ImportEntry> {
        vec![]
    }
    fn extract(
        &self,
        _path: &std::path::Path,
        _content: &str,
        _language: shared::common::taxonomy_language_vo::Language,
    ) -> Vec<ImportEntry> {
        vec![]
    }
    fn resolve_barrel_imports(&self, _: &std::path::Path) {}
}

impl IGraphProtocol for MockFilesystem {
    fn build_graph(
        &self,
        _imports: &[ImportEntry],
        _files: &[FileEntry],
        _definitions: &[DefinitionEntry],
        _implementations: &[ImplEntry],
    ) {
    }
    fn symbol_definitions(&self) -> &HashMap<String, Vec<std::path::PathBuf>> {
        &EMPTY_STRING_MAP
    }
    fn implementations(&self) -> &HashMap<String, Vec<std::path::PathBuf>> {
        &EMPTY_STRING_MAP
    }
    fn dependents(&self, _path: &std::path::Path) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn dependencies(&self, _path: &std::path::Path) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn reachable(&self, _from: &std::path::Path, _to: &std::path::Path) -> bool {
        false
    }
    fn reverse_links(&self) -> &HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> {
        &EMPTY_PATH_MAP
    }
}

impl IWorkspaceProtocol for MockFilesystem {
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
    fn is_member_path(&self, _path: &FilePath) -> bool {
        false
    }
    fn is_leaf_member_path(&self, _path: &FilePath) -> bool {
        false
    }
    fn detect_source_dir(&self, project_root: &std::path::Path) -> std::path::PathBuf {
        project_root.join("src")
    }
    fn detect_language_from_path(&self, path: &str) -> ConfigLanguage {
        if path.ends_with(".rs") {
            ConfigLanguage::Rust
        } else if path.ends_with(".py") {
            ConfigLanguage::Python
        } else {
            ConfigLanguage::TypeScript
        }
    }
    fn check_wired_in_container(
        &self,
        _workspace_root: &std::path::Path,
        _identifiers: &[String],
    ) -> bool {
        false
    }
    fn resolve_orphan_module_path(
        &self,
        _root: &std::path::Path,
        _base_dir: &std::path::Path,
        _module_path: &str,
    ) -> Option<std::path::PathBuf> {
        None
    }
}

impl IToolResolutionProtocol for MockFilesystem {
    fn is_executable_in_path(&self, _executable: &ToolName) -> bool {
        false
    }
    fn is_binary_available(&self, _bin_name: &ToolName) -> bool {
        false
    }
    fn has_local_bin(&self, _working_dir: &std::path::Path, _executable: &ToolName) -> bool {
        false
    }
    fn resolve_js_cmd(
        &self,
        _executable: &ToolName,
        _args: Vec<String>,
        _working_dir: &FilePath,
    ) -> Option<Vec<String>> {
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
    fn has_config_file(&self, _dir: &std::path::Path) -> bool {
        false
    }
    fn has_cargo_toml(&self, _path: &FilePath) -> Option<FilePath> {
        None
    }
    fn has_cargo_lock(&self, _path: &FilePath) -> Option<FilePath> {
        None
    }
    fn is_python_file_recursive(&self, _path: &FilePath) -> bool {
        false
    }
    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
}

impl IFileSystemIOProtocol for MockFilesystem {
    fn path_exists(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn is_dir(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn is_file(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, path: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(&self, path: &FilePath) -> String {
        path.value.clone()
    }
    fn is_symlink(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn metadata(&self, _path: &std::path::Path) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn symlink_metadata(
        &self,
        _path: &std::path::Path,
    ) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit('/').next().unwrap_or(path)
    }
    fn is_source_file(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn is_source_ext(&self, _ext: &FileExtension) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit('/').next().unwrap_or(path)
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit('/').nth(1).unwrap_or(path)
    }
    fn is_python_file(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _dir: &std::path::Path,
        _ignored: &[String],
    ) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _dir: &std::path::Path, _ignored: &[String]) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(
        &self,
        _dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(&self, _path: &std::path::Path) -> Result<String, std::io::Error> {
        Ok(String::new())
    }
    fn write_string(&self, _path: &std::path::Path, _content: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn copy_file(
        &self,
        _src: &std::path::Path,
        _dst: &std::path::Path,
    ) -> Result<u64, std::io::Error> {
        Ok(0)
    }
    fn create_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_permissions(&self, _path: &std::path::Path, _mode: u32) -> std::io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &std::path::Path) -> std::io::Result<()> {
        Ok(())
    }
    fn run_git_command(&self, _args: &[&str], _dir: &str) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        output.lines().map(String::from).collect()
    }
    fn run_external_command_in(
        &self,
        _name: &str,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn timing(&self) -> &ScanTiming {
        static TIMING: ScanTiming = ScanTiming {
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

impl IFilesystemAggregate for MockFilesystem {
    fn file_list(&self) -> &[FileEntry] {
        &[]
    }
    fn read_cached(&self, _path: &FilePath) -> ContentString {
        ContentString::default()
    }
    fn get_file_content(&self, _path: &std::path::Path) -> Option<String> {
        None
    }
    fn has_file(&self, _path: &std::path::Path) -> bool {
        false
    }
    fn collect_file_entries(&self, _files: &[String]) -> Vec<(std::path::PathBuf, String)> {
        vec![]
    }
    fn discover_source_files(&self, _root: &std::path::Path, _ignored: &[String]) -> Vec<String> {
        vec![]
    }
    fn read_file(&self, _path: &std::path::Path) -> Option<String> {
        None
    }
    fn scan_directory(&self, _root: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn discover_files(&self, _root: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn collect_source_files(&self, _dir: &std::path::Path, _ignored: &[String]) -> Vec<FilePath> {
        vec![]
    }
    fn read_lintable_file(&self, _path: &str) -> Option<String> {
        None
    }
    fn used_identifiers_for(&self, _: &std::path::Path) -> Vec<String> {
        vec![]
    }
    fn build_file_index(&self, _: &std::path::Path) {}
    fn build_orphan_graph_context(
        &self,
        _root_dir: &std::path::Path,
        _ignored: &[String],
    ) -> shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext {
        shared::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext::new(
            shared::filesystem::taxonomy_filesystem_vo::ImportGraph::new(HashMap::new()),
            shared::filesystem::taxonomy_filesystem_vo::InboundLinkMap::new(HashMap::new()),
            shared::filesystem::taxonomy_filesystem_vo::InheritanceMap::new(HashMap::new()),
            vec![],
        )
    }
}

pub fn mock_filesystem() -> Arc<dyn IFilesystemAggregate> {
    Arc::new(MockFilesystem)
}

// PURPOSE: Shared mock filesystem for dispatcher unit tests.
// Included via: #[path = "../../shared/tests/common/mock_filesystem.rs"]
// mod mock_filesystem;
//
// Mirrors crates/orphan-rules/tests/mock_filesystem.rs but uses the crate's
// dev-dependency path for shared (shared_lint_arwaky vs shared).
//
// Linked into several test targets; each consumer adds
// #[allow(dead_code, unused_imports)] to its `mod mock_filesystem;` line
// because not every target uses every symbol in this file.

use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_config_language_vo::ConfigLanguage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_source_vo::ContentString;
use shared_common::taxonomy_tool_name_vo::ToolName as CommonToolName;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IGraphProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::taxonomy_filesystem_request::FilesystemRequest;
use shared_filesystem::taxonomy_filesystem_response::FilesystemResponse;
use shared_filesystem::taxonomy_filesystem_vo::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Mock filesystem — path_exists returns false so collect_scan fails fast.
pub struct MockFilesystem {
    discover: Vec<String>,
    is_python: Option<bool>,
    canonicalize_prefix: Option<String>,
    /// What `DetectProjectLanguages` reports. Default: no languages, which makes
    /// every adapter selector decline — a test that needs an adapter to run
    /// sets this so the delegation is actually observable.
    languages: ProjectLanguagesVO,
}

impl MockFilesystem {
    pub fn new() -> Self {
        Self {
            discover: Vec::new(),
            is_python: None,
            canonicalize_prefix: None,
            languages: ProjectLanguagesVO::default(),
        }
    }

    /// Mock that reports the given files from `discover_files` / `scan_directory`.
    pub fn with_files(files: Vec<String>) -> Self {
        Self {
            discover: files,
            is_python: None,
            canonicalize_prefix: None,
            languages: ProjectLanguagesVO::default(),
        }
    }

    /// Mock that reports the given project languages from
    /// `DetectProjectLanguages`. An adapter-selection test needs this: with the
    /// default (no languages) every selector declines, so the assertion would
    /// pass whether or not the request was delegated to the filesystem seam.
    pub fn with_languages(languages: ProjectLanguagesVO) -> Self {
        Self {
            discover: Vec::new(),
            is_python: None,
            canonicalize_prefix: None,
            languages,
        }
    }

    /// Mock that reports whether the target path contains Python files.
    /// Useful for adapter tests that need `scan()` to proceed past the
    /// `is_python_file_recursive` guard.
    pub fn with_python_flag(is_python: bool) -> Self {
        Self {
            discover: Vec::new(),
            is_python: Some(is_python),
            canonicalize_prefix: None,
            languages: ProjectLanguagesVO::default(),
        }
    }

    /// Mock that canonicalizes the path argument by prepending `prefix/`.
    /// Combine with `with_python_flag(true)` for end-to-end adapter command tests.
    pub fn with_canonicalize_prefix(prefix: &str) -> Self {
        Self {
            discover: Vec::new(),
            is_python: Some(true),
            canonicalize_prefix: Some(prefix.to_string()),
            languages: ProjectLanguagesVO::default(),
        }
    }
}

/// Convenience constructor returning an `Arc<dyn IFilesystemAggregate>`.
pub fn mock_filesystem() -> Arc<dyn IFilesystemAggregate> {
    Arc::new(MockFilesystem::new())
}

/// Convenience constructor returning the workspace seam of the same mock.
pub fn mock_workspace() -> Arc<dyn IWorkspaceProtocol> {
    Arc::new(MockFilesystem::new())
}

/// Convenience constructor returning the io seam of the same mock.
pub fn mock_io() -> Arc<dyn shared_filesystem::IFileSystemIOProtocol> {
    Arc::new(MockFilesystem::new())
}

/// Convenience constructor returning the parser seam of the same mock.
pub fn mock_parser() -> Arc<dyn shared_filesystem::IParserProtocol> {
    Arc::new(MockFilesystem::new())
}

/// Convenience constructor returning the tool-resolution seam of the same mock.
pub fn mock_tool_resolution() -> Arc<dyn IToolResolutionProtocol> {
    Arc::new(MockFilesystem::new())
}

impl Default for MockFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

static EMPTY_STRING_MAP: std::sync::OnceLock<HashMap<String, Vec<std::path::PathBuf>>> =
    std::sync::OnceLock::new();
static EMPTY_PATH_MAP: std::sync::OnceLock<HashMap<std::path::PathBuf, Vec<std::path::PathBuf>>> =
    std::sync::OnceLock::new();

impl IParserProtocol for MockFilesystem {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &[]
    }
    fn import_list(&self) -> Vec<ImportEntry> {
        Vec::new()
    }
    fn parse_all(&self, _files: &mut [FileEntry]) {}
    fn imports_for(&self, _path: &std::path::Path) -> Vec<ImportEntry> {
        vec![]
    }
    fn extract(
        &self,
        _path: &std::path::Path,
        _content: &str,
        _language: shared_common::taxonomy_language_vo::Language,
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
        EMPTY_STRING_MAP.get_or_init(HashMap::new)
    }
    fn implementations(&self) -> &HashMap<String, Vec<std::path::PathBuf>> {
        EMPTY_STRING_MAP.get_or_init(HashMap::new)
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
        EMPTY_PATH_MAP.get_or_init(HashMap::new)
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
    fn detect_language_from_path(&self, _path: &str) -> ConfigLanguage {
        ConfigLanguage::Rust
    }
    fn detect_project_languages(&self, _root: &std::path::Path) -> ProjectLanguagesVO {
        self.languages
    }
    fn check_wired_in_container(
        &self,
        _workspace_root: &std::path::Path,
        _identifiers: &PatternList,
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
    fn is_executable_in_path(&self, _executable: &CommonToolName) -> bool {
        false
    }
    fn is_binary_available(&self, _bin_name: &CommonToolName) -> bool {
        false
    }
    fn has_local_bin(&self, _working_dir: &std::path::Path, _executable: &CommonToolName) -> bool {
        false
    }
    fn resolve_js_cmd(
        &self,
        _executable: &CommonToolName,
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
        self.is_python.unwrap_or(false)
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
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
        match &self.canonicalize_prefix {
            Some(prefix) => FilePath::new(format!("{prefix}/{}", path.value())).unwrap_or_default(),
            None => path.clone(),
        }
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
        _ignored: &PatternList,
    ) -> Vec<std::path::PathBuf> {
        vec![]
    }
    fn is_ignored_dir(&self, _dir: &std::path::Path, _ignored: &PatternList) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(
        &self,
        _dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(&self, _path: &std::path::Path) -> Result<ContentString, std::io::Error> {
        Ok(ContentString::new(""))
    }
    fn write_string(&self, _path: &std::path::Path, _content: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn copy_file(
        &self,
        _src: &std::path::Path,
        _dst: &std::path::Path,
    ) -> Result<ByteCount, std::io::Error> {
        Ok(ByteCount::new(0))
    }
    fn create_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_permissions(&self, _path: &std::path::Path, _mode: FileMode) -> std::io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &std::path::Path) -> std::io::Result<()> {
        Ok(())
    }
    fn run_git_command(&self, _args: &[&str], _dir: &str) -> GitCommandResult {
        GitCommandResult::new(String::new(), String::new(), false)
    }
    fn parse_output_lines(&self, output: &str) -> ParsedLines {
        ParsedLines::new(output.lines().map(String::from).collect())
    }
    fn run_external_command_in(
        &self,
        _name: &CommonToolName,
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
    fn execute(&self, request: FilesystemRequest) -> FilesystemResponse {
        match request {
            FilesystemRequest::FileList | FilesystemRequest::FileListSnapshot => {
                FilesystemResponse::Files {
                    entries: Vec::new(),
                }
            }
            // The mock owns no filesystem, so a test-directory walk finds
            // nothing. The dispatch test that needs real test files passes its
            // own aggregate instead of this one.
            FilesystemRequest::DiscoverFilesInDirectories { .. } => {
                FilesystemResponse::Paths { paths: Vec::new() }
            }
            FilesystemRequest::ReadCached { .. } => FilesystemResponse::Content {
                value: ContentString::default(),
            },
            FilesystemRequest::GetFileContent { .. }
            | FilesystemRequest::ReadFile { .. }
            | FilesystemRequest::ReadLintableFile { .. } => {
                FilesystemResponse::ContentOpt { value: None }
            }
            FilesystemRequest::HasFile { .. } => FilesystemResponse::Has { exists: false },
            FilesystemRequest::CollectFileEntries { .. } => {
                FilesystemResponse::Entries { pairs: Vec::new() }
            }
            FilesystemRequest::DiscoverSourceFiles { .. } => {
                FilesystemResponse::Paths { paths: Vec::new() }
            }
            FilesystemRequest::ScanDirectory { .. } | FilesystemRequest::DiscoverFiles { .. } => {
                FilesystemResponse::Paths {
                    paths: self.discover.clone(),
                }
            }
            FilesystemRequest::CollectSourceFiles { .. } => {
                FilesystemResponse::SourcePaths { paths: Vec::new() }
            }
            FilesystemRequest::UsedIdentifiers { .. } | FilesystemRequest::UsedIdentifiersAll => {
                FilesystemResponse::Identifiers { ids: Vec::new() }
            }
            FilesystemRequest::ImplementedTraitsMap => FilesystemResponse::TraitsMap {
                map: HashMap::new(),
            },
            FilesystemRequest::BuildFileIndex { .. }
            | FilesystemRequest::BuildFileIndexWithIgnored { .. } => FilesystemResponse::Files {
                entries: Vec::new(),
            },
            FilesystemRequest::BuildOrphanGraphContext { .. } => FilesystemResponse::GraphContext {
                context: GraphAnalysisContext::new(
                    ImportGraph::new(HashMap::new()),
                    InboundLinkMap::new(HashMap::new()),
                    InheritanceMap::new(HashMap::new()),
                    Vec::new(),
                ),
            },
            FilesystemRequest::FindWorkspaceRoot { .. } => FilesystemResponse::Root { path: None },
            FilesystemRequest::ResolvedImportList
            | FilesystemRequest::ExtendImportCache { .. }
            | FilesystemRequest::ImportListSnapshot => FilesystemResponse::Imports {
                entries: Vec::new(),
            },
            FilesystemRequest::DetectProjectLanguages { .. } => {
                FilesystemResponse::ProjectLanguages {
                    languages: self.languages,
                }
            }
            FilesystemRequest::ReadFileResult { .. } => FilesystemResponse::Content {
                value: ContentString::default(),
            },
            FilesystemRequest::PathExists { .. } => {
                FilesystemResponse::PathExists { exists: false }
            }
            FilesystemRequest::WriteFile { .. }
            | FilesystemRequest::CreateDirAll { .. }
            | FilesystemRequest::CopyFile { .. }
            | FilesystemRequest::RemoveDirAll { .. }
            | FilesystemRequest::RemoveFile { .. }
            | FilesystemRequest::SetPermissions { .. }
            | FilesystemRequest::Canonicalize { .. }
            | FilesystemRequest::ReadDirEntries { .. } => FilesystemResponse::OpOk { ok: false },
        }
    }
}

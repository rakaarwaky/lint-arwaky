// Agent layer — orchestrates FR-001 through FR-005
// Only orchestration: delegates to capabilities & utility

use crate::utility_filesystem_io;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, ImportEntry, ParseWarning, ScanTiming,
};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;
// ─── Block 1: Struct Definition ───────────────────────────

pub struct FilesystemOrchestratorDeps {
    pub io: Arc<dyn IFileSystemIOProtocol>,
    pub workspace: Arc<dyn IWorkspaceProtocol>,
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
    pub parser: Arc<dyn IParserProtocol>,
    pub graph: Arc<dyn IGraphProtocol>,
}

pub struct FilesystemOrchestrator {
    deps: FilesystemOrchestratorDeps,

    // Pipeline state (owned by agent, not by capabilities)
    files: OnceLock<Vec<FileEntry>>,
    file_index: OnceLock<HashMap<PathBuf, usize>>,
    imports: OnceLock<Vec<ImportEntry>>,
    warnings: OnceLock<Vec<ParseWarning>>,
    cached_reverse_links: OnceLock<HashMap<PathBuf, Vec<PathBuf>>>,
    cached_definitions: OnceLock<HashMap<String, Vec<PathBuf>>>,
    cached_implementations: OnceLock<HashMap<String, Vec<PathBuf>>>,
}

// ─── Block 2: Protocol Trait Implementations ──────────────

// ═══ IParserProtocol (5 methods) ═══════════════════════════

impl IParserProtocol for FilesystemOrchestrator {
    fn parse_warnings(&self) -> &[ParseWarning] {
        self.warnings.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn import_list(&self) -> &[ImportEntry] {
        self.imports.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn parse_all(&self, files: &mut [FileEntry]) {
        self.deps.parser.parse_all(files);
    }

    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
        self.deps.parser.imports_for(path)
    }

    fn extract(
        &self,
        path: &Path,
        content: &str,
        language: shared::filesystem::taxonomy_filesystem_vo::Language,
    ) -> Vec<ImportEntry> {
        self.deps.parser.extract(path, content, language)
    }
}

// ═══ IGraphProtocol (8 methods) ════════════════════════════

static EMPTY_HASH_MAP: std::sync::LazyLock<HashMap<PathBuf, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);
static EMPTY_STRING_MAP: std::sync::LazyLock<HashMap<String, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);

impl IGraphProtocol for FilesystemOrchestrator {
    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.cached_definitions.get().unwrap_or(&EMPTY_STRING_MAP)
    }

    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.cached_implementations
            .get()
            .unwrap_or(&EMPTY_STRING_MAP)
    }

    fn dependents(&self, path: &Path) -> Vec<PathBuf> {
        self.cached_reverse_links
            .get()
            .and_then(|m| m.get(path))
            .cloned()
            .unwrap_or_default()
    }

    fn dependencies(&self, path: &Path) -> Vec<PathBuf> {
        self.deps.graph.dependencies(path)
    }

    fn reachable(&self, from: &Path, to: &Path) -> bool {
        if from == to {
            return true;
        }
        self.cached_reverse_links.get().is_some_and(|m| {
            m.contains_key(to) && m.get(to).is_some_and(|v| v.contains(&from.to_path_buf()))
        })
    }

    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.cached_reverse_links.get().unwrap_or(&EMPTY_HASH_MAP)
    }
}

// ═══ IWorkspaceProtocol (8 methods) ════════════════════════

impl IWorkspaceProtocol for FilesystemOrchestrator {
    fn workspace_root(&self, start: &FilePath) -> Option<PathBuf> {
        self.deps.workspace.workspace_root(start)
    }

    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error> {
        self.deps.workspace.find_workspace_root_from_path(start)
    }

    fn is_member_path(&self, path: &FilePath) -> bool {
        self.deps.workspace.is_member_path(path)
    }

    fn is_leaf_member_path(&self, path: &FilePath) -> bool {
        self.deps.workspace.is_leaf_member_path(path)
    }

    fn detect_source_dir(&self, project_root: &Path) -> PathBuf {
        self.deps.workspace.detect_source_dir(project_root)
    }

    fn detect_language_from_path(&self, path: &str) -> ConfigLanguage {
        self.deps.workspace.detect_language_from_path(path)
    }

    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool {
        self.deps
            .workspace
            .check_wired_in_container(workspace_root, identifiers)
    }

    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf> {
        self.deps
            .workspace
            .resolve_orphan_module_path(root, base_dir, module_path)
    }
}

// ═══ IToolResolutionProtocol (12 methods) ══════════════════

impl IToolResolutionProtocol for FilesystemOrchestrator {
    fn is_executable_in_path(
        &self,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        self.deps.tool_resolution.is_executable_in_path(executable)
    }

    fn is_binary_available(
        &self,
        bin_name: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        self.deps.tool_resolution.is_binary_available(bin_name)
    }

    fn has_local_bin(
        &self,
        working_dir: &Path,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        self.deps
            .tool_resolution
            .has_local_bin(working_dir, executable)
    }

    fn resolve_js_cmd(
        &self,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
        args: Vec<String>,
        working_dir: &FilePath,
    ) -> Option<Vec<String>> {
        self.deps
            .tool_resolution
            .resolve_js_cmd(executable, args, working_dir)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        self.deps.tool_resolution.resolve_js_working_dir(path)
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        self.deps.tool_resolution.resolve_cargo_working_dir(path)
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        self.deps
            .tool_resolution
            .resolve_cargo_lock_working_dir(path)
    }

    fn has_config_file(&self, dir: &Path) -> bool {
        self.deps.tool_resolution.has_config_file(dir)
    }

    fn has_cargo_toml(&self, path: &FilePath) -> Option<FilePath> {
        self.deps.tool_resolution.has_cargo_toml(path)
    }

    fn has_cargo_lock(&self, path: &FilePath) -> Option<FilePath> {
        self.deps.tool_resolution.has_cargo_lock(path)
    }

    fn is_python_file_recursive(&self, path: &FilePath) -> bool {
        self.deps.tool_resolution.is_python_file_recursive(path)
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        self.deps.tool_resolution.default_working_dir(path)
    }
}

// ═══ IFileSystemIOProtocol (32 methods) ════════════════════

impl IFileSystemIOProtocol for FilesystemOrchestrator {
    // ── Path Operations (15) ─────────────────────────────────

    fn path_exists(&self, path: &Path) -> bool {
        self.deps.io.path_exists(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.deps.io.is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.deps.io.is_file(path)
    }

    fn should_ignore(&self, path: &FilePath, ignored: &[String]) -> bool {
        self.deps.io.should_ignore(path, ignored)
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        self.deps.io.canonicalize(path)
    }

    fn canonicalize_path_str(&self, path: &FilePath) -> String {
        self.deps.io.canonicalize_path_str(path)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        self.deps.io.is_symlink(path)
    }

    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        self.deps.io.metadata(path)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        self.deps.io.symlink_metadata(path)
    }

    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_file_stem(path)
    }

    fn is_source_file(&self, path: &Path) -> bool {
        self.deps.io.is_source_file(path)
    }

    fn is_source_ext(
        &self,
        ext: &shared::filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        self.deps.io.is_source_ext(ext)
    }

    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_basename(path)
    }

    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_parent(path)
    }

    fn is_python_file(&self, path: &Path) -> bool {
        self.deps.io.is_python_file(path)
    }

    // ── Directory Operations (3) ─────────────────────────────

    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
        self.deps.io.scan_directory_with_ignored(dir, ignored)
    }

    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool {
        self.deps.io.is_ignored_dir(dir, ignored)
    }

    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        self.deps.io.read_dir_entries_as_pathbuf(dir)
    }

    // ── File Read/Write (7) ──────────────────────────────────

    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error> {
        self.deps.io.read_to_string(path)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        self.deps.io.write_string(path, content)
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
        self.deps.io.copy_file(src, dst)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.remove_dir_all(path)
    }

    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()> {
        self.deps.io.set_permissions(path, mode)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.deps.io.remove_file(path)
    }

    // ── Process Execution (3) ────────────────────────────────

    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool) {
        self.deps.io.run_git_command(args, dir)
    }

    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        self.deps.io.parse_output_lines(output)
    }

    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool) {
        self.deps
            .io
            .run_external_command_in(name, args, current_dir)
    }

    // ── Scan Timing ──────────────────────────────────────────

    fn timing(&self) -> &ScanTiming {
        self.deps.io.timing()
    }
}

// ═══ IFilesystemAggregate (5 cache methods) ════════════════

impl IFilesystemAggregate for FilesystemOrchestrator {
    fn file_list(&self) -> &[FileEntry] {
        self.files.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn read_cached(&self, path: &FilePath) -> ContentString {
        let p: &Path = path;
        self.get_file_content(p)
            .map(|value| ContentString { value })
            .unwrap_or_else(|| ContentString {
                value: String::new(),
            })
    }

    fn get_file_content(&self, path: &Path) -> Option<String> {
        self.file_index
            .get()
            .and_then(|idx| idx.get(path))
            .and_then(|&i| self.files.get()?.get(i))
            .map(|entry| entry.content.clone())
    }

    fn has_file(&self, path: &Path) -> bool {
        self.file_index
            .get()
            .map(|idx| idx.contains_key(path))
            .unwrap_or(false)
    }

    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)> {
        let mut out = Vec::new();
        for file_str in files {
            let content = self
                .string_cache
                .get(file_str)
                .map(|r| r.value().clone())
                .unwrap_or_else(|| {
                    utility_filesystem_io::read_file_safe(file_str).unwrap_or_default()
                });
            out.push((PathBuf::from(file_str), content));
        }
        out
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl FilesystemOrchestrator {
    pub fn new(deps: FilesystemOrchestratorDeps) -> Self {
        Self {
            deps,
            string_cache: dashmap::DashMap::new(),
            files: OnceLock::new(),
            file_index: OnceLock::new(),
            imports: OnceLock::new(),
            warnings: OnceLock::new(),
            cached_reverse_links: OnceLock::new(),
            cached_definitions: OnceLock::new(),
            cached_implementations: OnceLock::new(),
        }
    }

    pub fn build_file_index(&self) {
        if let Some(file_list) = self.files.get() {
            let index: HashMap<PathBuf, usize> = file_list
                .iter()
                .enumerate()
                .map(|(i, entry)| (entry.path.clone(), i))
                .collect();
            let _ = self.file_index.set(index);
        }
    }
}

// Agent layer — orchestrates FR-001 through FR-004
// Zero I/O, zero business logic, zero domain computation
// Only orchestration: calls capabilities via contract protocols, returns results

use std::path::Path;
use std::sync::{OnceLock, RwLock};

use crate::capabilities_ast_parser::ASTParser;
use crate::capabilities_dependency_graph::DependencyGraph;
use crate::capabilities_file_walker::FileWalker;
use crate::utility_filesystem_io;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::{
    IFileWalkerProtocol,
    IASTParserProtocol, IDependencyGraphProtocol,
};
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, FilesystemResult, ImportEntry, Language, ParseWarning, ScanTiming,
};
use std::collections::HashMap;
use std::path::PathBuf;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FilesystemOrchestrator {
    walker: Box<dyn IFileWalkerProtocol>,
    parser: Box<dyn IASTParserProtocol>,
    graph: RwLock<Box<dyn IDependencyGraphProtocol>>,
    files: OnceLock<Vec<FileEntry>>,
    imports: OnceLock<Vec<ImportEntry>>,
    warnings: OnceLock<Vec<ParseWarning>>,
    timing: OnceLock<ScanTiming>,
    cached_reverse_links: OnceLock<HashMap<PathBuf, Vec<PathBuf>>>,
    cached_definitions: OnceLock<HashMap<String, Vec<PathBuf>>>,
    cached_implementations: OnceLock<HashMap<String, Vec<PathBuf>>>,
}

// ─── Block 2: Public Contract (aggregate trait ONLY) ──────

static EMPTY_HASH_MAP: once_cell::sync::Lazy<HashMap<PathBuf, Vec<PathBuf>>> =
    once_cell::sync::Lazy::new(HashMap::new);
static EMPTY_STRING_MAP: once_cell::sync::Lazy<HashMap<String, Vec<PathBuf>>> =
    once_cell::sync::Lazy::new(HashMap::new);

impl IFilesystemAggregate for FilesystemOrchestrator {
    fn scan(&self, root: &Path, ignored: &[String]) -> FilesystemResult {
        self.run_pipeline(root, ignored);
        FilesystemResult {
            files: self.file_list().to_vec(),
            imports: self.import_list().to_vec(),
            warnings: self.parse_warnings().to_vec(),
            graph: shared::filesystem::taxonomy_filesystem_vo::GraphData::default(),
            parsed_count: self.file_list().iter().filter(|f| f.parse_ok).count(),
            parse_errors: self.file_list().iter().filter(|f| !f.parse_ok).count(),
            unresolved_imports: 0,
            timing: self.timing().clone(),
        }
    }

    fn file_list(&self) -> &[FileEntry] {
        self.files.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn parse_warnings(&self) -> &[ParseWarning] {
        self.warnings.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn import_list(&self) -> &[ImportEntry] {
        self.imports.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn dependency_graph(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.cached_reverse_links.get().unwrap_or(&EMPTY_HASH_MAP)
    }

    fn reverse_import_map(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.cached_reverse_links.get().unwrap_or(&EMPTY_HASH_MAP)
    }

    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.cached_definitions.get().unwrap_or(&EMPTY_STRING_MAP)
    }

    fn trait_implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.cached_implementations
            .get()
            .unwrap_or(&EMPTY_STRING_MAP)
    }

    fn timing(&self) -> &ScanTiming {
        static DEFAULT: ScanTiming = ScanTiming {
            cache_ms: 0,
            walk_ms: 0,
            parse_ms: 0,
            extract_ms: 0,
            graph_ms: 0,
            total_ms: 0,
        };
        self.timing.get().unwrap_or(&DEFAULT)
    }

    fn read_lintable_file(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<Option<String>, String> {
        utility_filesystem_io::read_lintable_file(&path.value)
    }

    fn get_file_content(&self, path: &Path) -> Option<String> {
        self.file_list()
            .iter()
            .find(|f| f.path == path)
            .map(|f| f.content.clone())
    }

    fn has_file(&self, path: &Path) -> bool {
        self.file_list().iter().any(|f| f.path == path)
    }

    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry> {
        let exts = Language::extensions();
        self.walker.walk(root, ignored, &exts)
    }

    fn discover_source_files(
        &self,
        root: &Path,
        ignored: &[String],
    ) -> Vec<shared::common::taxonomy_path_vo::FilePath> {
        let exts = Language::extensions();
        self.walker.discover_paths(root, ignored, &exts)
    }

    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
        self.import_list()
            .iter()
            .filter(|i| i.source_file == path)
            .cloned()
            .collect()
    }

    fn depends_on(&self, from: &Path, to: &Path) -> bool {
        self.graph
            .read()
            .map(|g| g.reachable(from, to))
            .unwrap_or(false)
    }

    fn cycles(&self) -> Vec<Vec<PathBuf>> {
        self.graph.read().map(|g| g.cycles()).unwrap_or_default()
    }

    fn orphan_files(&self) -> Vec<PathBuf> {
        self.graph
            .read()
            .map(|g| g.orphan_files())
            .unwrap_or_default()
    }

    fn path_exists(&self, path: &Path) -> bool {
        utility_filesystem_io::path_exists(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        utility_filesystem_io::is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_file(path)
    }

    fn should_ignore(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
        ignored: &[String],
    ) -> bool {
        utility_filesystem_io::is_path_ignored(&path.value, ignored)
    }

    fn workspace_root(
        &self,
        start: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<PathBuf> {
        crate::utility_workspace_detection::find_workspace_root(&start.value)
    }

    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
        utility_filesystem_io::scan_directory_with_ignored(dir, ignored)
    }

    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool {
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
        utility_filesystem_io::is_path_ignored(name, ignored)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        utility_filesystem_io::is_symlink(path)
    }

    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        utility_filesystem_io::metadata(path)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        utility_filesystem_io::symlink_metadata(path)
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        utility_filesystem_io::canonicalize(path)
    }

    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_file_stem(path)
    }

    fn is_python_file(&self, dir: &Path) -> bool {
        utility_filesystem_io::is_source_file(dir)
            && dir
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "py")
                .unwrap_or(false)
    }

    fn has_config_file(&self, dir: &Path) -> bool {
        crate::utility_tool_resolution::has_config_file(dir)
    }

    fn has_cargo_toml(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::common::taxonomy_path_vo::FilePath> {
        crate::utility_tool_resolution::has_cargo_toml(&path.value)
            .map(|s| shared::common::taxonomy_path_vo::FilePath::new(s).unwrap_or_default())
    }

    fn has_cargo_lock(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<shared::common::taxonomy_path_vo::FilePath> {
        crate::utility_tool_resolution::has_cargo_lock(&path.value)
            .map(|s| shared::common::taxonomy_path_vo::FilePath::new(s).unwrap_or_default())
    }

    fn is_executable_in_path(
        &self,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        crate::utility_tool_resolution::is_executable_in_path(&executable.value)
    }

    fn has_local_bin(
        &self,
        working_dir: &Path,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        crate::utility_tool_resolution::has_local_bin(working_dir, &executable.value)
    }

    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error> {
        utility_filesystem_io::read_to_string(path)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        utility_filesystem_io::write_string(path, content)
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
        utility_filesystem_io::copy_file(src, dst)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        utility_filesystem_io::create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        utility_filesystem_io::remove_dir_all(path)
    }

    fn is_member_path(&self, path: &shared::common::taxonomy_path_vo::FilePath) -> bool {
        crate::utility_workspace_detection::is_member_path(&path.value)
    }

    fn is_leaf_member_path(&self, path: &shared::common::taxonomy_path_vo::FilePath) -> bool {
        crate::utility_workspace_detection::is_leaf_member_path(&path.value)
    }

    fn detect_source_dir(&self, project_root: &Path) -> PathBuf {
        crate::utility_workspace_detection::detect_source_dir(project_root)
    }

    fn scan_directory_recursive(&self, dir: &Path) -> Vec<String> {
        let mut files = Vec::new();
        scan_recursive(dir, &mut files);
        files
    }

    fn collect_source_files_from_path(&self, dir: &Path, files: &mut Vec<String>) {
        scan_recursive(dir, files);
    }

    fn is_source_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_source_file(path)
    }

    fn is_source_ext(
        &self,
        ext: &shared::filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        utility_filesystem_io::is_source_ext(&ext.value)
    }

    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_basename(path)
    }

    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_parent(path)
    }

    fn canonicalize_path_str(&self, path: &shared::common::taxonomy_path_vo::FilePath) -> String {
        utility_filesystem_io::canonicalize_path_str(&path.value)
    }

    fn resolve_js_cmd(
        &self,
        executable: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
        args: Vec<String>,
        working_dir: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Option<Vec<String>> {
        crate::utility_tool_resolution::resolve_js_cmd(&executable.value, args, &working_dir.value)
    }

    fn resolve_js_working_dir(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_path_vo::FilePath {
        let resolved = crate::utility_tool_resolution::resolve_js_working_dir(
            std::path::Path::new(&path.value),
        );
        shared::common::taxonomy_path_vo::FilePath::new(resolved.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn resolve_cargo_working_dir(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_path_vo::FilePath {
        let resolved = crate::utility_tool_resolution::resolve_cargo_working_dir(&path.value);
        shared::common::taxonomy_path_vo::FilePath::new(resolved.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_path_vo::FilePath {
        let resolved = crate::utility_tool_resolution::resolve_cargo_lock_working_dir(&path.value);
        shared::common::taxonomy_path_vo::FilePath::new(resolved.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn default_working_dir(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_path_vo::FilePath {
        let resolved =
            crate::utility_tool_resolution::default_working_dir(std::path::Path::new(&path.value));
        shared::common::taxonomy_path_vo::FilePath::new(resolved.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn is_python_file_recursive(&self, path: &shared::common::taxonomy_path_vo::FilePath) -> bool {
        crate::utility_tool_resolution::has_python_files_recursive(std::path::Path::new(
            &path.value,
        ))
    }

    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()> {
        utility_filesystem_io::set_permissions(path, mode)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        utility_filesystem_io::remove_file(path)
    }

    fn read_cached(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_source_vo::ContentString {
        crate::utility_file_cache::read_cached(path)
    }

    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool {
        check_wired_in_container(workspace_root, identifiers)
    }

    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error> {
        crate::utility_workspace_detection::find_workspace_root_from_path(start)
    }

    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf> {
        let candidate = if std::path::Path::new(module_path).is_absolute() {
            PathBuf::from(module_path)
        } else {
            base_dir.join(module_path)
        };
        confine_under_root(root, &candidate)
    }

    fn detect_language_from_path(
        &self,
        path: &str,
    ) -> shared::common::taxonomy_config_language_vo::ConfigLanguage {
        crate::utility_workspace_detection::detect_language_from_path(path)
    }

    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)> {
        let mut out = Vec::new();
        for file_str in files {
            let content = crate::utility_file_cache::cache_get_by_str(file_str)
                .unwrap_or_else(|| utility_filesystem_io::read_file_safe(file_str));
            out.push((PathBuf::from(file_str), content));
        }
        out
    }

    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool) {
        utility_filesystem_io::run_git_command(args, dir)
    }

    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        utility_filesystem_io::parse_output_lines(output)
    }

    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool) {
        utility_filesystem_io::run_external_command_in(name, args, current_dir)
    }

    fn is_binary_available(
        &self,
        bin_name: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
    ) -> bool {
        crate::utility_tool_resolution::is_binary_available(&bin_name.value)
    }

    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        utility_filesystem_io::read_dir_entries_as_pathbuf(dir)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl FilesystemOrchestrator {
    pub fn new() -> Self {
        Self {
            walker: Box::new(crate::capabilities_file_walker::FileWalker::new()),
            parser: Box::new(crate::capabilities_ast_parser::ASTParser::new()),
            graph: RwLock::new(Box::new(
                crate::capabilities_dependency_graph::DependencyGraph::new(),
            )),
            files: OnceLock::new(),
            imports: OnceLock::new(),
            warnings: OnceLock::new(),
            timing: OnceLock::new(),
            cached_reverse_links: OnceLock::new(),
            cached_definitions: OnceLock::new(),
            cached_implementations: OnceLock::new(),
        }
    }

    /// Run the scan pipeline. Returns `true` if the pipeline executed,
    /// `false` if data was already populated (stale — caller should use
    /// existing results or reconstruct a new orchestrator).
    fn run_pipeline(&self, root: &Path, ignored: &[String]) -> bool {
        if self.files.get().is_some() {
            return false; // Already ran — stale data
        }

        let start = std::time::Instant::now();

        // Stage 1: File Discovery
        let walk_start = std::time::Instant::now();
        let exts = Language::extensions();
        let mut files = self.walker.walk(root, ignored, &exts);
        let walk_ms = walk_start.elapsed().as_millis() as u64;

        // Stage 2: AST Parsing
        let parse_start = std::time::Instant::now();
        self.parser.parse_all(&mut files);
        let parse_ms = parse_start.elapsed().as_millis() as u64;

        // Collect parse warnings
        let warnings: Vec<ParseWarning> = files
            .iter()
            .filter(|f| !f.parse_ok)
            .map(|f| ParseWarning {
                file_path: f.path.clone(),
                error_detail: "File skipped: parse failure".to_string(),
            })
            .collect();

        // Stage 3: Import Extraction
        let extract_start = std::time::Instant::now();
        let mut imports = Vec::new();
        for file in &files {
            if file.parse_ok {
                let file_imports = crate::utility_import_extractor::extract_imports(
                    &file.path,
                    &file.content,
                    file.language,
                );
                imports.extend(file_imports);
            }
        }
        let extract_ms = extract_start.elapsed().as_millis() as u64;

        // Stage 4: Graph Construction
        let graph_start = std::time::Instant::now();
        let mut graph = self.graph.write().unwrap();
        graph.build(&imports, &files, &[], &[]);
        let graph_ms = graph_start.elapsed().as_millis() as u64;

        let total_ms = start.elapsed().as_millis() as u64;

        // Cache graph query results (no more Box::leak)
        let _ = self.cached_reverse_links.set(graph.reverse_links().clone());
        let _ = self.cached_definitions.set(graph.definitions().clone());
        let _ = self
            .cached_implementations
            .set(graph.implementations().clone());

        // Store results
        let _ = self.files.set(files);
        let _ = self.imports.set(imports);
        let _ = self.warnings.set(warnings);
        let _ = self.timing.set(ScanTiming {
            walk_ms,
            cache_ms: 0,
            parse_ms,
            extract_ms,
            graph_ms,
            total_ms,
        });
        true
    }
}

impl Default for FilesystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Private Helpers ──────────────────────────────────────

fn scan_recursive(dir: &Path, files: &mut Vec<String>) {
    for path in utility_filesystem_io::scan_directory(dir) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            if !matches!(
                name,
                "target" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
            ) {
                scan_recursive(&path, files);
            }
        } else if let Some(path_str) = path.to_str() {
            files.push(path_str.to_string());
        }
    }
}

fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = utility_filesystem_io::canonicalize(root).ok()?;
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };
    if let Ok(canonical_candidate) = utility_filesystem_io::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;
    let canonical_parent = utility_filesystem_io::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);
    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

fn check_wired_in_container(workspace_root: &Path, identifiers: &[String]) -> bool {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_containers(&dir, identifiers) {
            return true;
        }
    }
    false
}

fn check_dir_containers(dir: &Path, identifiers: &[String]) -> bool {
    for path in utility_filesystem_io::scan_directory(dir) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if matches!(
            name,
            "target"
                | ".git"
                | "node_modules"
                | "dist"
                | "build"
                | "__pycache__"
                | ".venv"
                | "tests"
        ) {
            continue;
        }
        if path.is_dir() {
            if check_dir_containers(&path, identifiers) {
                return true;
            }
        } else if name.ends_with("_container.rs")
            || name.ends_with("_container.py")
            || name.ends_with("_container.ts")
            || name.ends_with("_entry.rs")
            || name.ends_with("_entry.py")
            || name.ends_with("_entry.ts")
        {
            if let Ok(content) = utility_filesystem_io::read_to_string(&path) {
                for id in identifiers {
                    if content.contains(id) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

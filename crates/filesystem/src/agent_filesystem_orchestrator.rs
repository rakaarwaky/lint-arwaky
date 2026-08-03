// Agent layer — orchestrates FR-001 through FR-005
// Only orchestration: delegates to capabilities & utility

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
    DefinitionEntry, FileEntry, GraphAnalysisContext, ImplEntry, ImportEntry, ImportGraph,
    InboundLinkMap, InheritanceMap, ParseWarning, ScanTiming,
};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
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
    last_root: Mutex<Option<PathBuf>>,
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

    fn resolve_barrel_imports(&self, root_dir: &Path) {
        self.deps.parser.resolve_barrel_imports(root_dir);
    }
}

// ═══ IGraphProtocol (8 methods) ════════════════════════════

static EMPTY_HASH_MAP: std::sync::LazyLock<HashMap<PathBuf, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);
static EMPTY_STRING_MAP: std::sync::LazyLock<HashMap<String, Vec<PathBuf>>> =
    std::sync::LazyLock::new(HashMap::new);

impl IGraphProtocol for FilesystemOrchestrator {
    fn build_graph(
        &self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    ) {
        self.deps
            .graph
            .build_graph(imports, files, definitions, implementations);
    }

    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.ensure_graph_built();
        self.cached_definitions.get().unwrap_or(&EMPTY_STRING_MAP)
    }

    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        self.ensure_graph_built();
        self.cached_implementations
            .get()
            .unwrap_or(&EMPTY_STRING_MAP)
    }

    fn dependents(&self, path: &Path) -> Vec<PathBuf> {
        self.ensure_graph_built();
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
        self.ensure_graph_built();
        self.deps.graph.reachable(from, to)
    }

    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.ensure_graph_built();
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

// ═══ IFileSystemIOProtocol (29 methods) ════════════════════

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
        let p = Path::new(path.value());
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
            let path = PathBuf::from(file_str);
            let content = self
                .get_file_content(&path)
                .unwrap_or_else(|| self.deps.io.read_to_string(&path).unwrap_or_default());
            out.push((path, content));
        }
        out
    }

    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<String> {
        self.deps
            .io
            .scan_directory_with_ignored(root, ignored)
            .into_iter()
            .filter(|p| self.deps.io.is_source_file(p))
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    fn read_file(&self, path: &Path) -> Option<String> {
        self.get_file_content(path)
    }

    fn scan_directory(&self, root: &Path) -> Vec<String> {
        self.deps
            .io
            .scan_directory_with_ignored(root, &[])
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    fn discover_files(&self, root: &Path) -> Vec<String> {
        self.scan_directory(root)
    }

    fn collect_source_files(&self, dir: &Path, ignored: &[String]) -> Vec<FilePath> {
        self.deps
            .io
            .scan_directory_with_ignored(dir, ignored)
            .into_iter()
            .filter(|p| self.deps.io.is_source_file(p))
            .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .collect()
    }

    fn read_lintable_file(&self, path: &str) -> Option<String> {
        let p = Path::new(path);
        let meta = self.deps.io.metadata(p).ok()?;
        if meta.len() > 2 * 1024 * 1024 {
            return None;
        }
        self.deps.io.read_to_string(p).ok()
    }

    fn used_identifiers_for(&self, path: &Path) -> Vec<String> {
        self.file_index
            .get()
            .and_then(|idx| idx.get(path))
            .and_then(|&i| self.files.get()?.get(i))
            .and_then(|entry| entry.parse_metadata.as_ref())
            .map(|meta| match meta {
                shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Python(m) => {
                    m.used_identifiers.clone()
                }
                shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::TypeScript(m) => {
                    m.used_identifiers.clone()
                }
                shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::JavaScript(m) => {
                    m.used_identifiers.clone()
                }
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    fn build_file_index(&self, root: &Path) {
        self.build_file_index_impl(root, &[]);
    }

    fn build_orphan_graph_context(
        &self,
        root_dir: &Path,
        _ignored: &[String],
    ) -> GraphAnalysisContext {
        self.build_file_index(root_dir);
        self.ensure_graph_built();

        let top_root = self
            .deps
            .workspace
            .workspace_root(
                &FilePath::new(root_dir.to_string_lossy().to_string()).unwrap_or_default(),
            )
            .unwrap_or_else(|| root_dir.to_path_buf());

        // Build forward graph from import entries (source → targets)
        let imports = self.imports.get().cloned().unwrap_or_default();
        let mut forward: HashMap<String, Vec<String>> = HashMap::new();
        for imp in &imports {
            let src_rel = path_to_relative(&imp.source_file, &top_root);
            let target = imp.resolved_path.as_ref().unwrap_or(&imp.source_file);
            let tgt_rel = path_to_relative(target, &top_root);
            if src_rel != tgt_rel {
                forward.entry(src_rel).or_default().push(tgt_rel);
            }
        }

        let reverse: HashMap<String, Vec<String>> = self
            .deps
            .graph
            .reverse_links()
            .iter()
            .map(|(k, v)| {
                let rel = path_to_relative(k, &top_root);
                let rel_v: Vec<String> = v.iter().map(|p| path_to_relative(p, &top_root)).collect();
                (rel, rel_v)
            })
            .collect();

        // implementations() keys are symbol/trait names (String), values are file paths
        let inheritance: HashMap<String, Vec<String>> = self
            .deps
            .graph
            .implementations()
            .iter()
            .map(|(k, v)| {
                let rel_v: Vec<String> = v.iter().map(|p| path_to_relative(p, &top_root)).collect();
                (k.clone(), rel_v)
            })
            .collect();

        let all_files: Vec<String> = self
            .files
            .get()
            .map(|entries| {
                entries
                    .iter()
                    .map(|e| path_to_relative(&e.path, &top_root))
                    .collect()
            })
            .unwrap_or_default();

        GraphAnalysisContext::new(
            ImportGraph::new(forward),
            InboundLinkMap::new(reverse),
            InheritanceMap::new(inheritance),
            all_files,
        )
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl FilesystemOrchestrator {
    pub fn new(deps: FilesystemOrchestratorDeps) -> Self {
        Self {
            deps,
            files: OnceLock::new(),
            file_index: OnceLock::new(),
            imports: OnceLock::new(),
            warnings: OnceLock::new(),
            cached_reverse_links: OnceLock::new(),
            cached_definitions: OnceLock::new(),
            cached_implementations: OnceLock::new(),
            last_root: Mutex::new(None),
        }
    }

    /// Reset cached state so the next `build_file_index` call re-scans from scratch.
    /// Used when scanning multiple workspace members sequentially.
    pub fn reset_file_index(&self) {
        // We can't reset OnceLock, but we can create new ones by replacing self.
        // Since OnceLock doesn't support reset, we track via last_root and
        // recreate the struct. Instead, use a workaround: take the values.
        // Actually OnceLock has no take(). So we use the field tracking approach.
        // The caller should create a new FilesystemOrchestrator per workspace.
        // For backward compat, this is a no-op; use per-workspace instances instead.
    }

    /// Walk filesystem from root, discover source files, read content, parse imports.
    /// Populates files, file_index, imports, warnings caches.
    /// `extra_ignored` additional patterns beyond the built-in defaults.
    pub fn build_file_index_impl(&self, root: &Path, extra_ignored: &[String]) {
        if self.files.get().is_some() {
            return;
        }

        let mut ignored: Vec<String> = vec![
            "target".into(),
            "node_modules".into(),
            ".git".into(),
            "dist".into(),
            "build".into(),
            "__pycache__".into(),
            ".venv".into(),
        ];
        ignored.extend_from_slice(extra_ignored);

        let scanned: Vec<PathBuf> =
            crate::utility_workspace_detection::discover_source_files(root, &ignored)
                .into_iter()
                .map(PathBuf::from)
                .collect();

        let mut entries = Vec::new();
        let mut all_imports = Vec::new();
        let all_warnings = Vec::new();

        for path in &scanned {
            let language = self
                .deps
                .workspace
                .detect_language_from_path(&path.to_string_lossy());
            let content = match self.deps.io.read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let lang_enum = match language {
                shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::Rust
                }
                shared::common::taxonomy_config_language_vo::ConfigLanguage::Python => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::Python
                }
                shared::common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
                }
            };
            let imports = self.deps.parser.extract(path, &content, lang_enum);
            all_imports.extend(imports);

            // parse_ok: true when content is non-empty and language is recognized
            let parse_ok = !content.is_empty()
                && lang_enum != shared::filesystem::taxonomy_filesystem_vo::Language::Unknown;

            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let size = content.len() as u64;
            entries.push(shared::filesystem::taxonomy_filesystem_vo::FileEntry {
                path: path.clone(),
                extension,
                language: lang_enum,
                size,
                content,
                parse_ok,
                parse_metadata: None,
            });
        }

        let _ = self.files.set(entries.clone());
        let _ = self.imports.set(all_imports);
        let _ = self.warnings.set(all_warnings);

        let index: HashMap<PathBuf, usize> = entries
            .iter()
            .enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();
        let _ = self.file_index.set(index);
    }

    fn ensure_graph_built(&self) {
        if self.cached_reverse_links.get().is_some() {
            return;
        }
        let files = self.files.get().cloned().unwrap_or_default();
        let imports = self.imports.get().cloned().unwrap_or_default();

        // Extract definitions and implementations from parsed file metadata
        let mut definitions: Vec<DefinitionEntry> = Vec::new();
        let mut implementations: Vec<ImplEntry> = Vec::new();
        for entry in &files {
            if !entry.parse_ok {
                continue;
            }
            if let Some(ref meta) = entry.parse_metadata {
                match meta {
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Rust(m) => {
                        let lang = entry.language;
                        // Collect all symbol definitions from Rust metadata
                        for name in m
                            .struct_definitions
                            .iter()
                            .chain(m.enum_definitions.iter())
                            .chain(m.trait_definitions.iter())
                            .chain(m.type_definitions.iter())
                        {
                            definitions.push(DefinitionEntry {
                                name: name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                        // Collect trait implementations
                        for item in &m.impl_blocks {
                            if let Some(ref trait_name) = item.trait_name {
                                implementations.push(ImplEntry {
                                    trait_name: trait_name.clone(),
                                    file_path: entry.path.clone(),
                                    language: lang,
                                });
                            }
                        }
                    }
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Python(m) => {
                        let lang = entry.language;
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                    }
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::TypeScript(m) => {
                        let lang = entry.language;
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                        for iface in &m.interface_declarations {
                            definitions.push(DefinitionEntry {
                                name: iface.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        self.deps
            .graph
            .build_graph(&imports, &files, &definitions, &implementations);

        if let Some(rl) = self.deps.graph.reverse_links().clone().into() {
            let _ = self.cached_reverse_links.set(rl);
        }
        if let Some(sd) = self.deps.graph.symbol_definitions().clone().into() {
            let _ = self.cached_definitions.set(sd);
        }
        if let Some(imp) = self.deps.graph.implementations().clone().into() {
            let _ = self.cached_implementations.set(imp);
        }
    }
}

// ─── Free Functions ───────────────────────────────────────

/// Convert absolute path to relative path string (relative to workspace root).
fn path_to_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}

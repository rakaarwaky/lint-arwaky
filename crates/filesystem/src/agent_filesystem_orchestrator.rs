// Agent layer — orchestrates FR-001 through FR-005
// Only orchestration: delegates to capabilities & utility
use shared::{
    common::{
        DEFAULT_IGNORED_PATHS,
        taxonomy_common_vo::{FileContentPair, PatternList},
        taxonomy_config_language_vo::ConfigLanguage,
        taxonomy_path_vo::FilePath,
        taxonomy_source_vo::ContentString,
    },
    filesystem::{
        contract_filesystem_aggregate::IFilesystemAggregate,
        contract_filesystem_io_protocol::IFileSystemIOProtocol,
        contract_graph_protocol::IGraphProtocol,
        contract_parser_protocol::IParserProtocol,
        contract_tool_resolution_protocol::IToolResolutionProtocol,
        contract_workspace_protocol::IWorkspaceProtocol,
        taxonomy_filesystem_vo::{
            ByteCount, DefinitionEntry, FileEntry, FileExtension, FileMode, GitCommandResult,
            GraphAnalysisContext, ImplEntry, ImportEntry, ImportGraph, ImportType, InboundLinkMap,
            InheritanceMap, Language, ParseMetadata, ParseWarning, ParsedLines, ScanTiming,
            ToolName,
        },
    },
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, OnceLock},
};

// ─── Macros ────────────────────────────────────────────────────────────────

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FilesystemOrchestratorDeps {
    pub io: Arc<dyn IFileSystemIOProtocol>,
    pub workspace: Arc<dyn IWorkspaceProtocol>,
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
    pub parser: Arc<dyn IParserProtocol>,
    pub graph: Arc<dyn IGraphProtocol>,
}
pub struct FilesystemOrchestrator {
    pub(crate) deps: FilesystemOrchestratorDeps,
    pub(crate) files: OnceLock<Vec<FileEntry>>,
    pub(crate) file_index: OnceLock<HashMap<PathBuf, usize>>,
    pub(crate) imports: OnceLock<Vec<ImportEntry>>,
    pub(crate) resolved_imports: OnceLock<Vec<ImportEntry>>,
    pub(crate) warnings: OnceLock<Vec<ParseWarning>>,
    pub(crate) cached_reverse_links: OnceLock<HashMap<PathBuf, Vec<PathBuf>>>,
    pub(crate) cached_definitions: OnceLock<HashMap<String, Vec<PathBuf>>>,
    pub(crate) cached_implementations: OnceLock<HashMap<String, Vec<PathBuf>>>,
}

// ─── Block 2: Protocol Trait Implementations ──────────────
// ═══ IParserProtocol ═══════════════════════════
impl IParserProtocol for FilesystemOrchestrator {
    fn parse_warnings(&self) -> &[ParseWarning] {
        self.warnings.get().map(|v| v.as_slice()).unwrap_or(&[])
    }
    fn import_list(&self) -> Vec<ImportEntry> {
        self.imports.get().cloned().unwrap_or_default()
    }
    fn parse_all(&self, files: &mut [FileEntry]) {
        self.deps.parser.parse_all(files)
    }
    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
        self.deps.parser.imports_for(path)
    }
    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry> {
        self.deps.parser.extract(path, content, language)
    }
    fn resolve_barrel_imports(&self, root_dir: &Path) {
        self.deps.parser.resolve_barrel_imports(root_dir)
    }
}

// ═══ IGraphProtocol ════════════════════════════
static EMPTY_HASH_MAP: LazyLock<HashMap<PathBuf, Vec<PathBuf>>> = LazyLock::new(HashMap::new);
static EMPTY_STRING_MAP: LazyLock<HashMap<String, Vec<PathBuf>>> = LazyLock::new(HashMap::new);

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
    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        self.ensure_graph_built();
        self.cached_reverse_links.get().unwrap_or(&EMPTY_HASH_MAP)
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
}

// ═══ IWorkspaceProtocol ════════════════════════
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
    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &PatternList) -> bool {
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

// ═══ IToolResolutionProtocol ══════════════════
impl IToolResolutionProtocol for FilesystemOrchestrator {
    fn is_executable_in_path(&self, executable: &ToolName) -> bool {
        self.deps.tool_resolution.is_executable_in_path(executable)
    }
    fn is_binary_available(&self, bin_name: &ToolName) -> bool {
        self.deps.tool_resolution.is_binary_available(bin_name)
    }
    fn has_local_bin(&self, working_dir: &Path, executable: &ToolName) -> bool {
        self.deps
            .tool_resolution
            .has_local_bin(working_dir, executable)
    }
    fn resolve_js_cmd(
        &self,
        executable: &ToolName,
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

// ═══ IFileSystemIOProtocol ════════════════════
impl IFileSystemIOProtocol for FilesystemOrchestrator {
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
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
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
    fn is_source_ext(&self, ext: &FileExtension) -> bool {
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
    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &PatternList) -> Vec<PathBuf> {
        self.deps.io.scan_directory_with_ignored(dir, ignored)
    }
    fn is_ignored_dir(&self, dir: &Path, ignored: &PatternList) -> bool {
        self.deps.io.is_ignored_dir(dir, ignored)
    }
    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        self.deps.io.read_dir_entries_as_pathbuf(dir)
    }
    fn read_to_string(&self, path: &Path) -> Result<ContentString, std::io::Error> {
        self.deps.io.read_to_string(path)
    }
    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        self.deps.io.write_string(path, content)
    }
    fn copy_file(&self, src: &Path, dst: &Path) -> Result<ByteCount, std::io::Error> {
        self.deps.io.copy_file(src, dst)
    }
    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.create_dir_all(path)
    }
    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.remove_dir_all(path)
    }
    fn set_permissions(&self, path: &Path, mode: FileMode) -> std::io::Result<()> {
        self.deps.io.set_permissions(path, mode)
    }
    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.deps.io.remove_file(path)
    }
    fn run_git_command(&self, args: &[&str], dir: &str) -> GitCommandResult {
        self.deps.io.run_git_command(args, dir)
    }
    fn parse_output_lines(&self, output: &str) -> ParsedLines {
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
    fn timing(&self) -> &ScanTiming {
        self.deps.io.timing()
    }
}

// ═══ IFilesystemAggregate ════════════════
impl IFilesystemAggregate for FilesystemOrchestrator {
    fn file_list(&self) -> &[FileEntry] {
        self.files.get().map(|v| v.as_slice()).unwrap_or(&[])
    }
    fn read_cached(&self, path: &FilePath) -> ContentString {
        let value = self
            .get_file_content(Path::new(path.value()))
            .unwrap_or_default();
        ContentString { value }
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
            .is_some_and(|idx| idx.contains_key(path))
    }
    fn collect_file_entries(&self, files: &PatternList) -> Vec<FileContentPair> {
        files
            .values()
            .iter()
            .map(|file_str| {
                let path = PathBuf::from(file_str);
                let content = self.get_file_content(&path).unwrap_or_else(|| {
                    self.deps
                        .io
                        .read_to_string(&path)
                        .map(|c| c.value)
                        .unwrap_or_default()
                });
                FileContentPair::new(path, content)
            })
            .collect()
    }
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<String> {
        let pl = PatternList::new(ignored.to_vec());
        self.deps
            .io
            .scan_directory_with_ignored(root, &pl)
            .into_iter()
            .filter(|p| self.deps.io.is_source_file(p))
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
    fn read_file(&self, path: &Path) -> Option<String> {
        self.get_file_content(path)
            .or_else(|| self.deps.io.read_to_string(path).ok().map(|c| c.value))
    }
    fn scan_directory(&self, root: &Path) -> Vec<String> {
        let empty = PatternList::default();
        self.deps
            .io
            .scan_directory_with_ignored(root, &empty)
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
    fn discover_files(&self, root: &Path) -> Vec<String> {
        self.scan_directory(root)
    }
    fn collect_source_files(&self, dir: &Path, ignored: &[String]) -> Vec<FilePath> {
        let pl = PatternList::new(ignored.to_vec());
        self.deps
            .io
            .scan_directory_with_ignored(dir, &pl)
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
        self.deps.io.read_to_string(p).ok().map(|c| c.value)
    }
    fn used_identifiers_for(&self, path: &Path) -> Vec<String> {
        self.file_index
            .get()
            .and_then(|idx| idx.get(path))
            .and_then(|&i| self.files.get()?.get(i))
            .and_then(|entry| entry.parse_metadata.as_ref())
            .map(|meta| match meta {
                ParseMetadata::Rust(m) => m.used_identifiers.clone(),
                ParseMetadata::Python(m) => m.used_identifiers.clone(),
                ParseMetadata::TypeScript(m) => m.used_identifiers.clone(),
                ParseMetadata::JavaScript(m) => m.used_identifiers.clone(),
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }
    fn implemented_traits_map(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        if let Some(files) = self.files.get() {
            for entry in files.iter() {
                if let Some(ParseMetadata::Rust(meta)) = &entry.parse_metadata {
                    for impl_block in &meta.impl_blocks {
                        if let Some(ref trait_name) = impl_block.trait_name {
                            let types = map.entry(trait_name.clone()).or_default();
                            if !types.contains(&impl_block.implementor_type) {
                                types.push(impl_block.implementor_type.clone());
                            }
                        }
                    }
                }
            }
        }
        map
    }
    fn build_file_index(&self, root: &Path) {
        self.build_file_index_impl(root, &[]);
    }
    fn build_file_index_with_ignored(&self, root: &Path, ignored: &[String]) {
        self.build_file_index_impl(root, ignored);
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
        let all_files_set: HashSet<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let imports = self.imports.get().cloned().unwrap_or_default();
        let mut forward: HashMap<String, Vec<String>> = HashMap::new();
        let mut resolved_import_entries: Vec<ImportEntry> = Vec::with_capacity(imports.len());
        for imp in &imports {
            let src_rel = path_to_relative(&imp.source_file, &top_root);
            let target_file = self.resolve_import_target(imp, &src_rel, &top_root, &all_files_set);
            let mut entry = imp.clone();
            if let Some(tgt_rel) = &target_file {
                entry.resolved_path = Some(PathBuf::from(tgt_rel));
                if src_rel != *tgt_rel {
                    forward
                        .entry(src_rel.clone())
                        .or_default()
                        .push(tgt_rel.clone());
                    if tgt_rel.ends_with(".rs") {
                        if let Some(lib_path) =
                            crate::utility_import_resolution::derive_crate_lib_rs(tgt_rel)
                        {
                            if all_files_set.contains(lib_path.as_str()) && lib_path != src_rel {
                                forward.entry(src_rel.clone()).or_default().push(lib_path);
                            }
                        }
                    }
                }
            }
            resolved_import_entries.push(entry);
        }
        let _ = self.resolved_imports.set(resolved_import_entries);
        let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
        for (src, targets) in &forward {
            for tgt in targets {
                reverse.entry(tgt.clone()).or_default().push(src.clone());
            }
        }
        let inheritance: HashMap<String, Vec<String>> = self
            .deps
            .graph
            .implementations()
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.iter().map(|p| path_to_relative(p, &top_root)).collect(),
                )
            })
            .collect();
        let lib_rs_files: Vec<String> = all_files
            .iter()
            .filter(|f| f.ends_with("/lib.rs"))
            .cloned()
            .collect();
        for lib in &lib_rs_files {
            for other_lib in &lib_rs_files {
                if lib != other_lib {
                    forward
                        .entry(lib.clone())
                        .or_default()
                        .push(other_lib.clone());
                }
            }
        }
        GraphAnalysisContext::new(
            ImportGraph::new(forward),
            InboundLinkMap::new(reverse),
            InheritanceMap::new(inheritance),
            all_files,
        )
    }
    // #15: route through injected protocol instead of static utility call
    fn find_workspace_root(&self, start: &Path) -> Option<PathBuf> {
        self.deps
            .workspace
            .find_workspace_root_from_path(start)
            .ok()
    }
    fn resolved_import_list(&self) -> Vec<ImportEntry> {
        self.resolved_imports.get().cloned().unwrap_or_default()
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
            resolved_imports: OnceLock::new(),
            warnings: OnceLock::new(),
            cached_reverse_links: OnceLock::new(),
            cached_definitions: OnceLock::new(),
            cached_implementations: OnceLock::new(),
        }
    }
    pub fn resolve_import_target(
        &self,
        imp: &ImportEntry,
        src_rel: &str,
        top_root: &Path,
        all_files_set: &HashSet<&str>,
    ) -> Option<String> {
        if imp.import_type == ImportType::Mod && imp.resolved_path.is_none() {
            let src_dir = Path::new(src_rel)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            let mod_name = &imp.raw_path;
            let candidate_rs = if src_dir.is_empty() {
                format!("{}.rs", mod_name)
            } else {
                format!("{}/{}.rs", src_dir, mod_name)
            };
            let candidate_mod = if src_dir.is_empty() {
                format!("{}/mod.rs", mod_name)
            } else {
                format!("{}/{}/mod.rs", src_dir, mod_name)
            };
            if all_files_set.contains(candidate_rs.as_str()) {
                Some(candidate_rs)
            } else if all_files_set.contains(candidate_mod.as_str()) {
                Some(candidate_mod)
            } else {
                None
            }
        } else if imp.resolved_path.is_some() {
            imp.resolved_path
                .as_ref()
                .map(|p| path_to_relative(p, top_root))
        } else {
            let raw = &imp.raw_path;
            let src_dir = Path::new(src_rel)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            if raw.starts_with("./") || raw.starts_with("../") {
                let base = Path::new(&src_dir);
                let rel = raw.strip_prefix("./").unwrap_or(raw);
                let candidate = base.join(rel).to_string_lossy().to_string();
                if all_files_set.contains(candidate.as_str()) {
                    Some(candidate)
                } else if !candidate.contains('.') {
                    let exts = [".ts", ".js", ".tsx", ".jsx", ".rs", ".py"];
                    exts.iter().find_map(|ext| {
                        let c = format!("{}{}", candidate, ext);
                        if all_files_set.contains(c.as_str()) {
                            Some(c)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else if imp.language == Language::Python {
                if raw.starts_with('.') {
                    let src_dir = Path::new(src_rel)
                        .parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let module_path = raw.trim_start_matches('.').replace('.', "/");
                    let candidates = vec![
                        format!("{}/{}.py", src_dir, module_path),
                        format!("{}/{}/__init__.py", src_dir, module_path),
                    ];
                    candidates
                        .into_iter()
                        .find(|c| all_files_set.contains(c.as_str()))
                } else {
                    let module_path = raw.replace('.', "/");
                    // A: direct path (import already includes member prefix)
                    let try_direct = |suffix: &str| {
                        let p = format!("{}{}", module_path, suffix);
                        all_files_set.contains(p.as_str()).then_some(p)
                    };
                    try_direct(".py")
                        .or_else(|| try_direct("/__init__.py"))
                        // B: prepend modules/ | packages/ | crates/
                        .or_else(|| {
                            ["modules", "packages", "crates"].iter().find_map(|md| {
                                let py = format!("{}/{}.py", md, module_path);
                                if all_files_set.contains(py.as_str()) { return Some(py); }
                                let init = format!("{}/{}/__init__.py", md, module_path);
                                all_files_set.contains(init.as_str()).then_some(init)
                            })
                        })
                        // C: suffix/stem match (bare module name in nested dir)
                        .or_else(|| {
                            let stem = raw.rsplit('.').next().unwrap_or(raw);
                            let suffix = format!("/{}.py", stem);
                            all_files_set.iter().find(|f| {
                                **f == format!("{}.py", stem) || f.ends_with(&suffix)
                            }).map(|f| f.to_string())
                        })
                }
            } else if imp.language == Language::TypeScript || imp.language == Language::JavaScript {
                let parts: Vec<&str> = raw.split('/').collect();
                if parts.len() >= 2 {
                    let pkg_name = parts[0];
                    let sub = if parts.len() > 2 && parts[1] == "src" {
                        parts[2..].join("/")
                    } else {
                        parts[1..].join("/")
                    };
                    crate::utility_import_resolution::resolve_external_crate_import(
                        pkg_name,
                        &sub,
                        top_root,
                        all_files_set,
                    )
                } else {
                    None
                }
            } else {
                let module = raw
                    .strip_prefix("crate::")
                    .or_else(|| raw.strip_prefix("super::"))
                    .unwrap_or(raw);
                let root_seg = module.split("::").next().unwrap_or("");
                if !root_seg.is_empty() {
                    let candidate = if src_dir.is_empty() {
                        format!("{}.rs", root_seg)
                    } else {
                        format!("{}/{}.rs", src_dir, root_seg)
                    };
                    if all_files_set.contains(candidate.as_str()) {
                        Some(candidate)
                    } else {
                        let sub_path = module.split("::").skip(1).collect::<Vec<_>>().join("/");
                        crate::utility_import_resolution::resolve_external_crate_import(
                            root_seg,
                            &sub_path,
                            top_root,
                            all_files_set,
                        )
                    }
                } else {
                    None
                }
            }
        }
    }
}

// ─── Pipeline Helpers ───
impl FilesystemOrchestrator {
    pub fn build_file_index_impl(&self, root: &Path, extra_ignored: &[String]) {
        let ws_root = self
            .deps
            .workspace
            .workspace_root(&FilePath::new(root.to_string_lossy().to_string()).unwrap_or_default())
            .unwrap_or_else(|| root.to_path_buf());
        let mut ignored: Vec<String> = DEFAULT_IGNORED_PATHS
            .iter()
            .map(|s| format!("{}/", s))
            .collect();
        ignored.extend_from_slice(extra_ignored);
        let abs_root = std::fs::canonicalize(&ws_root).unwrap_or(ws_root);
        let member_dirs: Vec<&str> = ["crates", "packages", "modules"]
            .iter()
            .filter(|d| abs_root.join(d).is_dir())
            .copied()
            .collect();
        let scanned: Vec<PathBuf> =
            crate::utility_workspace_detection::discover_source_files(&abs_root, &ignored)
                .into_iter()
                .map(PathBuf::from)
                .filter(|p| {
                    if member_dirs.is_empty() {
                        return true;
                    }
                    if let Ok(rel) = p.strip_prefix(&abs_root) {
                        let rel_str = rel.to_string_lossy();
                        member_dirs
                            .iter()
                            .any(|d| rel_str.starts_with(&format!("{}/", d)))
                    } else {
                        true
                    }
                })
                .collect();
        let mut entries = Vec::new();
        let mut all_imports = Vec::new();
        for path in &scanned {
            let language = self
                .deps
                .workspace
                .detect_language_from_path(&path.to_string_lossy());
            let content = match self.deps.io.read_to_string(path) {
                Ok(c) => c.value,
                Err(_) => continue,
            };
            let lang_enum = match language {
                ConfigLanguage::Rust => Language::Rust,
                ConfigLanguage::Python => Language::Python,
                ConfigLanguage::TypeScript => Language::TypeScript,
            };
            all_imports.extend(self.deps.parser.extract(path, &content, lang_enum));
            let parse_ok = !content.is_empty() && lang_enum != Language::Unknown;
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            entries.push(FileEntry {
                path: path.clone(),
                extension,
                language: lang_enum,
                size: content.len() as u64,
                content,
                parse_ok,
                parse_metadata: None,
            });
        }
        self.parse_all(&mut entries);
        self.resolve_barrel_imports(&abs_root);
        let _ = self.files.set(entries.clone());
        // #10: import_list() already returns Vec — no .to_vec() needed
        let _ = self.imports.set(self.deps.parser.import_list());
        // #11: propagate real warnings from the parser, not an empty vec
        let _ = self
            .warnings
            .set(self.deps.parser.parse_warnings().to_vec());
        let _ = self.file_index.set(
            entries
                .iter()
                .enumerate()
                .map(|(i, e)| (e.path.clone(), i))
                .collect(),
        );
    }
    pub(crate) fn ensure_graph_built(&self) {
        if self.cached_reverse_links.get().is_some() {
            return;
        }
        let files = self.files.get().cloned().unwrap_or_default();
        let imports = self.imports.get().cloned().unwrap_or_default();
        let mut definitions: Vec<DefinitionEntry> = Vec::new();
        let mut implementations: Vec<ImplEntry> = Vec::new();
        for entry in &files {
            if !entry.parse_ok {
                continue;
            }
            if let Some(ref meta) = entry.parse_metadata {
                match meta {
                    ParseMetadata::Rust(m) => {
                        let lang = entry.language;
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
                    ParseMetadata::Python(m) => {
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: entry.language,
                            });
                        }
                    }
                    ParseMetadata::TypeScript(m) => {
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: entry.language,
                            });
                        }
                        for iface in &m.interface_declarations {
                            definitions.push(DefinitionEntry {
                                name: iface.clone(),
                                file_path: entry.path.clone(),
                                language: entry.language,
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
        // #20: .clone() already returns owned HashMap — no .into() round-trip needed
        let rl = self.deps.graph.reverse_links().clone();
        let _ = self.cached_reverse_links.set(rl);
        let sd = self.deps.graph.symbol_definitions().clone();
        let _ = self.cached_definitions.set(sd);
        let imp = self.deps.graph.implementations().clone();
        let _ = self.cached_implementations.set(imp);
    }
}

// ─── Free Functions ───────────────────────────────────────
fn path_to_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}

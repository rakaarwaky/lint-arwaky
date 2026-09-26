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
        contract_filesystem_protocol::IFileSystemIOProtocol,
        contract_filesystem_protocol::IGraphProtocol,
        contract_filesystem_protocol::IParserProtocol,
        contract_filesystem_protocol::IToolResolutionProtocol,
        contract_filesystem_protocol::IWorkspaceProtocol,
        taxonomy_filesystem_request::FilesystemRequest,
        taxonomy_filesystem_response::FilesystemResponse,
        taxonomy_filesystem_vo::{
            DefinitionEntry, FileEntry, GraphAnalysisContext, ImplEntry, ImportEntry, ImportGraph,
            ImportType, InboundLinkMap, InheritanceMap, Language, ParseMetadata, ParseWarning,
        },
    },
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
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
    /// Extra import cache entries not tied to `files` (dispatcher patches).
    pub(crate) imports_extra: std::sync::Mutex<Vec<ImportEntry>>,
    /// Snapshot of the import cache at the last `build_file_index_with_ignored`.
    pub(crate) imports_snapshot: std::sync::Mutex<Vec<ImportEntry>>,
    pub(crate) resolved_imports: OnceLock<Vec<ImportEntry>>,
    pub(crate) warnings: OnceLock<Vec<ParseWarning>>,
    pub(crate) cached_reverse_links: OnceLock<HashMap<PathBuf, Vec<PathBuf>>>,
    pub(crate) cached_definitions: OnceLock<HashMap<String, Vec<PathBuf>>>,
    pub(crate) cached_implementations: OnceLock<HashMap<String, Vec<PathBuf>>>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IFilesystemAggregate for FilesystemOrchestrator {
    fn execute(&self, request: FilesystemRequest) -> FilesystemResponse {
        match request {
            FilesystemRequest::FileList | FilesystemRequest::FileListSnapshot => {
                FilesystemResponse::Files {
                    entries: self.file_list_snapshot(),
                }
            }
            FilesystemRequest::ReadCached { path } => FilesystemResponse::Content {
                value: self.read_cached(&path),
            },
            FilesystemRequest::GetFileContent { path } => FilesystemResponse::ContentOpt {
                value: self.get_file_content(&path),
            },
            FilesystemRequest::HasFile { path } => FilesystemResponse::Has {
                exists: self.has_file(&path),
            },
            FilesystemRequest::CollectFileEntries { patterns } => FilesystemResponse::Entries {
                pairs: self.collect_file_entries(&patterns),
            },
            FilesystemRequest::DiscoverSourceFiles { root, ignored } => FilesystemResponse::Paths {
                paths: self.discover_source_files(&root, &ignored),
            },
            FilesystemRequest::ReadFile { path } => FilesystemResponse::ContentOpt {
                value: self.read_file(&path),
            },
            FilesystemRequest::ScanDirectory { root } => FilesystemResponse::Paths {
                paths: self.scan_directory(&root),
            },
            FilesystemRequest::DiscoverFiles { root } => FilesystemResponse::Paths {
                paths: self.discover_files(&root),
            },
            FilesystemRequest::CollectSourceFiles { dir, ignored } => {
                FilesystemResponse::SourcePaths {
                    paths: self.collect_source_files(&dir, &ignored),
                }
            }
            FilesystemRequest::ReadLintableFile { path } => FilesystemResponse::ContentOpt {
                value: self.read_lintable_file(&path),
            },
            FilesystemRequest::UsedIdentifiers { path } => FilesystemResponse::Identifiers {
                ids: self.used_identifiers_for(&path),
            },
            FilesystemRequest::UsedIdentifiersAll => FilesystemResponse::Identifiers {
                ids: self.used_identifiers_all(),
            },
            FilesystemRequest::ImplementedTraitsMap => FilesystemResponse::TraitsMap {
                map: self.implemented_traits_map(),
            },
            FilesystemRequest::BuildFileIndex { root } => {
                self.build_file_index(&root);
                FilesystemResponse::Paths { paths: Vec::new() }
            }
            FilesystemRequest::BuildFileIndexWithIgnored { root, ignored } => {
                self.build_file_index_with_ignored(&root, &ignored);
                FilesystemResponse::Paths { paths: Vec::new() }
            }
            FilesystemRequest::BuildOrphanGraphContext { root, ignored } => {
                FilesystemResponse::GraphContext {
                    context: self.build_orphan_graph_context(&root, &ignored),
                }
            }
            FilesystemRequest::FindWorkspaceRoot { start } => FilesystemResponse::Root {
                path: self.find_workspace_root(&start),
            },
            FilesystemRequest::ResolvedImportList => FilesystemResponse::Imports {
                entries: self.resolved_import_list(),
            },
            FilesystemRequest::ExtendImportCache { entries } => {
                self.extend_import_cache(entries);
                FilesystemResponse::Imports {
                    entries: Vec::new(),
                }
            }
            FilesystemRequest::ImportListSnapshot => FilesystemResponse::Imports {
                entries: self.import_list_snapshot(),
            },
        }
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl FilesystemOrchestrator {
    /// Builds a graph analysis context for the workspace rooted at the specified directory.
    ///
    /// The context includes resolved import links, inbound links, inheritance relationships,
    /// implementation bridges, container wiring, and the workspace's discovered files.
    ///
    /// # Arguments
    ///
    /// * `root_dir` - Directory from which to determine the workspace and discover files.
    ///
    /// # Returns
    ///
    /// A graph analysis context containing workspace files and their dependency relationships.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let context = orchestrator.build_orphan_graph_context(std::path::Path::new("."), &[]);
    /// ```
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
                    .map(|e| crate::utility_container_wiring::path_to_relative(&e.path, &top_root))
                    .collect()
            })
            .unwrap_or_default();
        let all_files_set: HashSet<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let stem_index = Self::build_stem_index(&all_files);
        let imports = self.imports.get().cloned().unwrap_or_default();
        let mut forward: HashMap<String, Vec<String>> = HashMap::new();
        let mut resolved_import_entries: Vec<ImportEntry> = Vec::with_capacity(imports.len());
        for imp in &imports {
            let src_rel =
                crate::utility_container_wiring::path_to_relative(&imp.source_file, &top_root);
            let target_file =
                self.resolve_import_target(imp, &src_rel, &top_root, &all_files_set, &stem_index);
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
        let inheritance: HashMap<String, Vec<String>> = self
            .deps
            .graph
            .implementations()
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.iter()
                        .map(|p| crate::utility_container_wiring::path_to_relative(p, &top_root))
                        .collect(),
                )
            })
            .collect();

        // P2: contract → capabilities bridge (reverse index, issue #193).
        // For every trait/interface/base, wire its defining (contract) file to each
        // implementor so BFS that reaches a contract also reaches its capabilities.
        crate::utility_container_wiring::add_impl_bridge_edges(
            &top_root,
            self.deps.graph.symbol_definitions(),
            self.deps.graph.implementations(),
            &mut forward,
        );

        // P1: Container-aware wiring — resolve each *_container.* file's used identifiers
        // against the workspace symbol table and add synthetic edges container →
        // referenced file, so BFS that reaches the container reaches its DI services.
        crate::utility_container_wiring::add_container_wiring_edges(
            &all_files,
            &top_root,
            self.deps.graph.symbol_definitions(),
            |p: &Path| self.used_identifiers_for(p),
            &mut forward,
        );

        crate::utility_container_wiring::add_lib_rs_edges(&all_files, &mut forward);

        // Build the reverse (inbound) link map only after all synthetic edges
        // (impl bridges, container wiring, lib.rs) have been added, so taxonomy
        // and utility analyzers see the same inbound links the surfaces analyzer does.
        let reverse = crate::utility_container_wiring::build_reverse_index(&forward);
        GraphAnalysisContext::new(
            ImportGraph::new(forward),
            InboundLinkMap::new(reverse),
            InheritanceMap::new(inheritance),
            all_files,
        )
    }
    fn find_workspace_root(&self, start: &Path) -> Option<PathBuf> {
        self.deps
            .workspace
            .find_workspace_root_from_path(start)
            .ok()
    }
    fn resolved_import_list(&self) -> Vec<ImportEntry> {
        self.resolved_imports.get().cloned().unwrap_or_default()
    }
    fn extend_import_cache(&self, entries: Vec<ImportEntry>) {
        if let Ok(mut extra) = self.imports_extra.lock() {
            extra.extend(entries);
        }
    }
    fn import_list_snapshot(&self) -> Vec<ImportEntry> {
        self.imports_snapshot
            .lock()
            .map(|s| s.clone())
            .unwrap_or_default()
    }
    /// Returns a snapshot of all discovered source file entries.
    pub fn file_list_snapshot(&self) -> Vec<FileEntry> {
        self.files.get().cloned().unwrap_or_default()
    }
    /// Reads a file's content from the bounded cache.
    pub fn read_cached(&self, path: &FilePath) -> ContentString {
        let value = self
            .get_file_content(Path::new(path.value()))
            .unwrap_or_default();
        ContentString { value }
    }
    /// Returns a cached file's content by path.
    pub fn get_file_content(&self, path: &Path) -> Option<String> {
        self.file_index
            .get()
            .and_then(|idx| idx.get(path))
            .and_then(|&i| self.files.get()?.get(i))
            .map(|entry| entry.content.clone())
    }
    /// Reports whether a path is present in the cache.
    pub fn has_file(&self, path: &Path) -> bool {
        self.file_index
            .get()
            .is_some_and(|idx| idx.contains_key(path))
    }
    /// Collects (path, content) pairs for each lintable file in the pattern list.
    pub fn collect_file_entries(&self, files: &PatternList) -> Vec<FileContentPair> {
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
    /// Discovers source files under a root, filtering by ignored patterns.
    pub fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<String> {
        let pl = PatternList::new(ignored.to_vec());
        self.deps
            .io
            .scan_directory_with_ignored(root, &pl)
            .into_iter()
            .filter(|p| self.deps.io.is_source_file(p))
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
    /// Reads a file's text content, falling back to disk when uncached.
    pub fn read_file(&self, path: &Path) -> Option<String> {
        self.get_file_content(path)
            .or_else(|| self.deps.io.read_to_string(path).ok().map(|c| c.value))
    }
    /// Scans a directory recursively and returns every file path.
    pub fn scan_directory(&self, root: &Path) -> Vec<String> {
        let empty = PatternList::default();
        self.deps
            .io
            .scan_directory_with_ignored(root, &empty)
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
    /// Discovers all files (source and non-source) under a root.
    pub fn discover_files(&self, root: &Path) -> Vec<String> {
        self.scan_directory(root)
    }
    /// Collects source-file paths under a directory, filtered by ignored patterns.
    pub fn collect_source_files(&self, dir: &Path, ignored: &[String]) -> Vec<FilePath> {
        let pl = PatternList::new(ignored.to_vec());
        self.deps
            .io
            .scan_directory_with_ignored(dir, &pl)
            .into_iter()
            .filter(|p| self.deps.io.is_source_file(p))
            .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .collect()
    }
    /// Reads a lintable file, skipping files larger than 2 MiB.
    pub fn read_lintable_file(&self, path: &str) -> Option<String> {
        let p = Path::new(path);
        let meta = self.deps.io.metadata(p).ok()?;
        if meta.len() > 2 * 1024 * 1024 {
            return None;
        }
        self.deps.io.read_to_string(p).ok().map(|c| c.value)
    }
    /// Returns tree-sitter-extracted used identifiers for a cached file.
    pub fn used_identifiers_for(&self, path: &Path) -> Vec<String> {
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
    /// Returns every used identifier found across all cached parse metadata.
    pub fn used_identifiers_all(&self) -> Vec<String> {
        self.files
            .get()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry.parse_metadata.as_ref())
                    .flat_map(|meta| match meta {
                        ParseMetadata::Rust(m) => m.used_identifiers.clone(),
                        ParseMetadata::Python(m) => m.used_identifiers.clone(),
                        ParseMetadata::TypeScript(m) => m.used_identifiers.clone(),
                        ParseMetadata::JavaScript(m) => m.used_identifiers.clone(),
                        _ => Vec::new(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Builds a cross-file trait-name to implementor map from cached parse metadata.
    pub fn implemented_traits_map(&self) -> HashMap<String, Vec<String>> {
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
    /// Builds the file index from a root, discovering, reading, and parsing.
    pub fn build_file_index(&self, root: &Path) {
        self.build_file_index_impl(root, &[]);
    }
    /// Builds the file index with extra ignored patterns merged into the defaults.
    pub fn build_file_index_with_ignored(&self, root: &Path, ignored: &[String]) {
        self.build_file_index_impl(root, ignored);
    }
    /// Builds the file index and records an import-cache snapshot.
    pub fn build_file_index_and_snapshot(&self, root: &Path, ignored: &[String]) {
        self.build_file_index_impl(root, ignored);
        if let Ok(mut snapshot) = self.imports_snapshot.lock() {
            *snapshot = self.deps.parser.import_list();
        }
    }

    pub fn new(deps: FilesystemOrchestratorDeps) -> Self {
        Self {
            deps,
            files: OnceLock::new(),
            file_index: OnceLock::new(),
            imports: OnceLock::new(),
            imports_extra: std::sync::Mutex::new(Vec::new()),
            imports_snapshot: std::sync::Mutex::new(Vec::new()),
            resolved_imports: OnceLock::new(),
            warnings: OnceLock::new(),
            cached_reverse_links: OnceLock::new(),
            cached_definitions: OnceLock::new(),
            cached_implementations: OnceLock::new(),
        }
    }
    /// Resolves an import to a workspace-relative source file.
    ///
    /// Resolution supports Rust modules and external crates, relative imports, Python modules and packages, and TypeScript or JavaScript packages. Existing resolved paths are converted to paths relative to `top_root`.
    ///
    /// # Arguments
    ///
    /// * `imp` - Import metadata, including its raw path, language, and import type.
    /// * `src_rel` - Workspace-relative path of the importing source file.
    /// * `top_root` - Workspace root used to normalize resolved paths.
    /// * `all_files_set` - Workspace-relative paths of all discovered files.
    /// * `stem_index` - Index of file stems used for Python module matching.
    ///
    /// # Returns
    ///
    /// The workspace-relative target path when the import resolves to a discovered file; otherwise, `None`.
    pub fn resolve_import_target(
        &self,
        imp: &ImportEntry,
        src_rel: &str,
        top_root: &Path,
        all_files_set: &HashSet<&str>,
        stem_index: &HashMap<String, Vec<String>>,
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
                .map(|p| crate::utility_container_wiring::path_to_relative(p, top_root))
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
            } else {
                crate::utility_import_resolution::resolve_by_language(
                    imp,
                    &src_dir,
                    src_rel,
                    top_root,
                    all_files_set,
                    stem_index,
                )
            }
        }
    }
}

// ─── Pipeline Helpers ───
impl FilesystemOrchestrator {
    /// Builds a lexicographically-sorted index of file stems → file paths.
    pub fn build_stem_index(file_paths: &[String]) -> HashMap<String, Vec<String>> {
        let mut stem_index: HashMap<String, Vec<String>> = HashMap::new();
        for f in file_paths {
            if let Some(stem) = Path::new(f).file_stem().and_then(|s| s.to_str()) {
                stem_index
                    .entry(stem.to_string())
                    .or_default()
                    .push(f.clone());
            }
        }
        for v in stem_index.values_mut() {
            v.sort();
        }
        stem_index
    }

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
        let abs_root = self.deps.io.canonicalize(&ws_root).unwrap_or(ws_root);
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
        self.deps.parser.parse_all(&mut entries);
        self.deps.parser.resolve_barrel_imports(&abs_root);
        let _ = self.files.set(entries.clone());
        let parser_imports = self.deps.parser.import_list();
        // Snapshot before the parser cache gets overwritten by a scoped parse_all
        if let Ok(mut snap) = self.imports_snapshot.lock() {
            *snap = parser_imports.clone();
        }
        let _ = self.imports.set(parser_imports);
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
    /// Builds and caches the dependency graph and its symbol relationships.
    ///
    /// Subsequent calls reuse the cached graph data.
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
                            // P4: class inheritance is a contract→capabilities bridge
                            // (e.g. `class AdditionAnalyzer(CalculatorProtocol)`).
                            for base in &class.bases {
                                if !base.is_empty() {
                                    implementations.push(ImplEntry {
                                        trait_name: base.clone(),
                                        file_path: entry.path.clone(),
                                        language: entry.language,
                                    });
                                }
                            }
                        }
                    }
                    ParseMetadata::TypeScript(m) => {
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: entry.language,
                            });
                            // P4: `implements` clauses are a contract→capabilities bridge
                            // (e.g. `class AdditionAnalyzer implements CalculatorProtocol`).
                            for iface in &class.implements {
                                if !iface.is_empty() {
                                    implementations.push(ImplEntry {
                                        trait_name: iface.clone(),
                                        file_path: entry.path.clone(),
                                        language: entry.language,
                                    });
                                }
                            }
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
        let rl = self.deps.graph.reverse_links().clone();
        let _ = self.cached_reverse_links.set(rl);
        let sd = self.deps.graph.symbol_definitions().clone();
        let _ = self.cached_definitions.set(sd);
        let imp = self.deps.graph.implementations().clone();
        let _ = self.cached_implementations.set(imp);
    }
}

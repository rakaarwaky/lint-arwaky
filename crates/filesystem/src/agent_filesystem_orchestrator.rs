// Agent layer — orchestrates FR-001 through FR-005
// Only orchestration: delegates to capabilities & utility
use shared_common::{
    taxonomy_common_vo::{FileContentPair, PatternList},
    taxonomy_default_constant::DEFAULT_IGNORED_PATHS,
    taxonomy_language_vo::ConfigLanguage,
    taxonomy_lint_vo::ContentString,
    taxonomy_path_vo::FilePath,
};
use shared_filesystem::utility_agent_filesystem::{build_stem_index, read_guard, write_guard};
use shared_filesystem::{
    contract_filesystem_aggregate::IFilesystemAggregate,
    contract_filesystem_protocol::IFileSystemIOProtocol,
    contract_filesystem_protocol::IGraphProtocol,
    contract_filesystem_protocol::IParserProtocol,
    contract_filesystem_protocol::IToolResolutionProtocol,
    contract_filesystem_protocol::IWorkspaceProtocol,
    taxonomy_filesystem_request::FilesystemRequest,
    taxonomy_filesystem_response::FilesystemResponse,
    taxonomy_filesystem_vo::FileMode,
    taxonomy_filesystem_vo::{
        DefinitionEntry, FileEntry, GraphAnalysisContext, ImplEntry, ImportEntry, ImportGraph,
        InboundLinkMap, InheritanceMap, Language, ParseMetadata, ParseWarning,
    },
    utility_path_filter, utility_test_file_discovery, utility_workspace_detection,
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, RwLock},
};

// ─── Block 1: Module Declaration ───────────────────────────
pub struct FilesystemOrchestratorDeps {
    pub io: Arc<dyn IFileSystemIOProtocol>,
    pub workspace: Arc<dyn IWorkspaceProtocol>,
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
    pub parser: Arc<dyn IParserProtocol>,
    pub graph: Arc<dyn IGraphProtocol>,
}
pub struct FilesystemOrchestrator {
    pub(crate) deps: FilesystemOrchestratorDeps,
    pub(crate) files: RwLock<Option<Vec<FileEntry>>>,
    pub(crate) file_index: RwLock<Option<HashMap<PathBuf, usize>>>,
    pub(crate) imports: RwLock<Option<Vec<ImportEntry>>>,
    /// Extra import cache entries not tied to `files` (dispatcher patches).
    pub(crate) imports_extra: std::sync::Mutex<Vec<ImportEntry>>,
    /// Snapshot of the import cache at the last `build_file_index_with_ignored`.
    pub(crate) imports_snapshot: std::sync::Mutex<Vec<ImportEntry>>,
    pub(crate) resolved_imports: OnceLock<Vec<ImportEntry>>,
    pub(crate) warnings: RwLock<Option<Vec<ParseWarning>>>,
    pub(crate) cached_reverse_links: RwLock<Option<HashMap<PathBuf, Vec<PathBuf>>>>,
    pub(crate) cached_definitions: RwLock<Option<HashMap<String, Vec<PathBuf>>>>,
    pub(crate) cached_implementations: RwLock<Option<HashMap<String, Vec<PathBuf>>>>,
    /// The effective ignore set (defaults + extras) used for the current warm
    /// index. A subsequent build with a *different* effective set triggers a
    /// rebuild so the new patterns take effect; an identical set is a no-op.
    pub(crate) index_ignored: Mutex<Option<Vec<String>>>,
    /// Serializes concurrent rebuilds of the file index.
    pub(crate) rebuild_lock: Mutex<()>,
}

impl FilesystemOrchestrator {
    pub fn new(deps: FilesystemOrchestratorDeps) -> Self {
        Self {
            deps,
            files: RwLock::new(None),
            file_index: RwLock::new(None),
            imports: RwLock::new(None),
            imports_extra: std::sync::Mutex::new(Vec::new()),
            imports_snapshot: std::sync::Mutex::new(Vec::new()),
            resolved_imports: OnceLock::new(),
            warnings: RwLock::new(None),
            cached_reverse_links: RwLock::new(None),
            cached_definitions: RwLock::new(None),
            cached_implementations: RwLock::new(None),
            index_ignored: Mutex::new(None),
            rebuild_lock: Mutex::new(()),
        }
    }
}

// ─── Block 2: Module Contract ──────────────
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
            FilesystemRequest::DiscoverFilesInDirectories {
                root,
                directories,
                ignored,
            } => FilesystemResponse::Paths {
                paths: self.discover_files_in_directories(&root, &directories, &ignored),
            },
            FilesystemRequest::ReadFile { path } => FilesystemResponse::ContentOpt {
                value: self.read_file(&path),
            },
            FilesystemRequest::ScanDirectory { root } => FilesystemResponse::Paths {
                paths: self.scan_directory(&root),
            },
            FilesystemRequest::DiscoverFiles { root } => FilesystemResponse::Paths {
                paths: utility_workspace_detection::discover_files(root),
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
            FilesystemRequest::DetectProjectLanguages { root } => {
                // Through the workspace seam, not straight to the shared utility:
                // every sibling arm in this match goes through an injected
                // capability, and a substituted implementation must stay in
                // control of what the agent observes.
                FilesystemResponse::ProjectLanguages {
                    languages: self.deps.workspace.detect_project_languages(&root),
                }
            }
            FilesystemRequest::ReadFileResult { path } => FilesystemResponse::ContentOpt {
                // A missing or unreadable file is `None`, not "". Returning an
                // empty string here made every caller that probes for a file
                // (config discovery walking up the tree) believe it found an
                // empty file and stop looking.
                value: self.deps.io.read_to_string(&path).ok().map(|c| c.value),
            },
            FilesystemRequest::WriteFile { path, content } => FilesystemResponse::OpOk {
                ok: self.deps.io.write_string(&path, &content).is_ok(),
            },
            FilesystemRequest::CreateDirAll { path } => FilesystemResponse::OpOk {
                ok: self.deps.io.create_dir_all(&path).is_ok(),
            },
            FilesystemRequest::CopyFile { src, dst } => FilesystemResponse::OpOk {
                ok: self.deps.io.copy_file(&src, &dst).is_ok(),
            },
            FilesystemRequest::RemoveDirAll { path } => FilesystemResponse::OpOk {
                ok: self.deps.io.remove_dir_all(&path).is_ok(),
            },
            FilesystemRequest::RemoveFile { path } => FilesystemResponse::OpOk {
                ok: self.deps.io.remove_file(&path).is_ok(),
            },
            FilesystemRequest::SetPermissions { path, mode } => FilesystemResponse::OpOk {
                ok: self
                    .deps
                    .io
                    .set_permissions(&path, FileMode::new(mode))
                    .is_ok(),
            },
            FilesystemRequest::Canonicalize { path } => FilesystemResponse::Paths {
                paths: match self.deps.io.canonicalize(&path) {
                    Ok(p) => vec![p.to_string_lossy().to_string()],
                    Err(_) => Vec::new(),
                },
            },
            FilesystemRequest::PathExists { path } => FilesystemResponse::PathExists {
                exists: self.deps.io.path_exists(&path),
            },
            FilesystemRequest::BinaryAvailable { bin_name } => {
                FilesystemResponse::BinaryAvailable {
                    available: self
                        .deps
                        .tool_resolution
                        .is_binary_available(&shared_common::ToolName::new(bin_name.clone())),
                }
            }
            FilesystemRequest::IsMemberPath { path } => FilesystemResponse::IsMember {
                is_member: self.deps.workspace.is_member_path(&path),
            },
            FilesystemRequest::ParseAll { mut entries } => {
                self.deps.parser.parse_all(&mut entries);
                FilesystemResponse::ParsedEntries { entries }
            }
            FilesystemRequest::ImportsFor { path } => FilesystemResponse::Imports {
                entries: self.deps.parser.imports_for(&path),
            },
            FilesystemRequest::ReadDirEntries { dir } => FilesystemResponse::Paths {
                paths: match self.deps.io.read_dir_entries_as_pathbuf(&dir) {
                    Ok(v) => v
                        .into_iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect(),
                    Err(_) => Vec::new(),
                },
            },
        }
    }
}

// ─── Block 3: Module Helpers ─────────

impl FilesystemOrchestrator {
    // Cache & content accessors — each backs a dispatcher arm in Block 2; served from the warm index, falling back to the io seam where noted.
    /// Returns a snapshot of all discovered source file entries.
    pub fn file_list_snapshot(&self) -> Vec<FileEntry> {
        let guard = read_guard(&self.files);
        guard.as_ref().cloned().unwrap_or_default()
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
        let idx = read_guard(&self.file_index);
        let files_guard = read_guard(&self.files);
        idx.as_ref()
            .and_then(|i| i.get(path))
            .and_then(|&i| files_guard.as_ref()?.get(i))
            .map(|entry| entry.content.clone())
    }

    /// Reports whether a path is present in the cache.
    pub fn has_file(&self, path: &Path) -> bool {
        let idx = read_guard(&self.file_index);
        idx.as_ref().is_some_and(|idx| idx.contains_key(path))
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

    /// Reads a file's text content, falling back to disk when uncached.
    pub fn read_file(&self, path: &Path) -> Option<String> {
        self.get_file_content(path)
            .or_else(|| self.deps.io.read_to_string(path).ok().map(|c| c.value))
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
        let idx = read_guard(&self.file_index);
        let files_guard = read_guard(&self.files);
        idx.as_ref()
            .and_then(|i| i.get(path))
            .and_then(|&i| files_guard.as_ref()?.get(i))
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
        let guard = read_guard(&self.files);
        guard
            .as_ref()
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
        let guard = read_guard(&self.files);
        if let Some(files) = guard.as_ref() {
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

    // Discovery & scanning — recursive walks, pattern filtering, and directory listing.
    /// Discovers source files under a root, filtering by ignored patterns.
    /// Recursive walk so nested source trees (e.g. `src/lib.rs`) are found.
    pub fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<String> {
        let mut found = Vec::new();
        self.discover_recursive(root, root, ignored, &mut found);
        found.sort();
        found.dedup();
        found
    }

    /// Recursive directory walk used by `discover_source_files`. Skips
    /// default-ignored dirs by base name and config-ignored patterns by their
    /// full relative path from `root` (so multi-segment patterns such as
    /// `internal/lint-arwaky/.worktrees` are honoured).
    fn discover_recursive(
        &self,
        dir: &Path,
        root: &Path,
        ignored: &[String],
        found: &mut Vec<String>,
    ) {
        let Ok(entries) = dir.read_dir() else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if shared_common::DEFAULT_IGNORED_PATHS.contains(&name) {
                continue;
            }
            let rel_str = path
                .strip_prefix(root)
                .map(|r| r.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string_lossy().to_string());
            if utility_path_filter::is_path_ignored(&rel_str, ignored) {
                continue;
            }
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => {
                    self.discover_recursive(&path, root, ignored, found);
                }
                Ok(kind) if kind.is_file() && self.deps.io.is_source_file(&path) => {
                    found.push(path.to_string_lossy().to_string());
                }
                _ => {}
            }
        }
    }

    /// Discovers source files inside *directories* anywhere under *root*, with
    /// those directories exempted from the default skip list (keeps `tests/`
    /// and `benches/`). Recursive walk, depth-capped. AES103 needs these.
    pub fn discover_files_in_directories(
        &self,
        root: &Path,
        directories: &[String],
        ignored: &[String],
    ) -> Vec<String> {
        let names: Vec<&str> = directories.iter().map(String::as_str).collect();
        // The configured list names `tests` and `benches` — they are in
        // `DEFAULT_IGNORED_PATHS` precisely so the production walk prunes them.
        // Drop them from the effective pattern list before the discovery walk,
        // otherwise the very suite roots we're meant to find get pruned.
        let effective: Vec<String> = ignored
            .iter()
            .filter(|pattern| !names.contains(&pattern.as_str()))
            .cloned()
            .collect();
        let keep = utility_test_file_discovery::ignore_patterns_keeping(&names, &effective);
        let mut found: Vec<String> = Vec::new();
        for dir in utility_test_file_discovery::find_test_directories(root, &names, &effective) {
            // Start at depth 0 so the full cap is available for the recursive walk.
            // The cap on MAX_TEST_WALK_DEPTH keeps a symlinked directory from
            // making the walk unbounded.
            self.collect_source_files_recursive(&dir, &keep, 0, &mut found);
        }
        found.sort();
        found.dedup();
        found
    }

    /// How deep the `tests/`/`benches/` walk descends (AES103 flat-layout rule).
    const MAX_TEST_WALK_DEPTH: usize = 4;

    /// Recursive source-file collection under *dir*, depth-first. Symlinked
    /// directories are never descended into; depth is caller-capped.
    fn collect_source_files_recursive(
        &self,
        dir: &Path,
        ignored: &[String],
        depth: usize,
        found: &mut Vec<String>,
    ) {
        if depth > Self::MAX_TEST_WALK_DEPTH {
            return;
        }
        let Ok(entries) = dir.read_dir() else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            // An ignored name prunes the whole subtree: that is what a config
            // pattern like `slow_tests` means.
            if shared_common::DEFAULT_IGNORED_PATHS.contains(&name)
                || utility_path_filter::is_path_ignored(name, ignored)
            {
                continue;
            }
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => {
                    self.collect_source_files_recursive(&path, ignored, depth + 1, found);
                }
                Ok(kind) if kind.is_file() && self.deps.io.is_source_file(&path) => {
                    found.push(path.to_string_lossy().to_string());
                }
                _ => {}
            }
        }
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
    /// Recursive, file-only: language-detection input for the external-lint
    /// orchestrator (keeps nested `src/*.py` visible to extension checks).
    pub fn discover_files(&self, root: &Path) -> Vec<String> {
        utility_workspace_detection::discover_files(root)
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

    // Import-cache & workspace mechanics — small state accessors over the pub(crate) fields.
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

    // Import resolution — thin delegation; the mechanism lives in `utility_import_resolution`.
    /// Resolves an import to a workspace-relative source file.
    /// Delegates to `utility_import_resolution::resolve_import_target`.
    pub fn resolve_import_target(
        &self,
        imp: &ImportEntry,
        src_rel: &str,
        top_root: &Path,
        all_files_set: &HashSet<&str>,
        stem_index: &HashMap<String, Vec<String>>,
    ) -> Option<String> {
        shared_filesystem::utility_import_resolution::resolve_import_target(
            imp,
            src_rel,
            top_root,
            all_files_set,
            stem_index,
        )
    }

    // Warm-index building — scans, parses, and populates the cache fields; rebuilds only when the effective ignore set changes.
    /// Builds the file index from a root. Warm-index semantics apply: a
    /// rebuild happens only when the effective ignore set changes.
    pub fn build_file_index(&self, root: &Path) {
        self.build_file_index_impl(root, &[]);
    }

    /// Builds the file index with extra ignored patterns merged into the
    /// defaults. Rebuilds when the effective ignore set differs from the
    /// current warm index; no-op when it matches.
    pub fn build_file_index_with_ignored(&self, root: &Path, ignored: &[String]) {
        self.build_file_index_impl(root, ignored);
    }

    pub fn build_file_index_impl(&self, root: &Path, extra_ignored: &[String]) {
        let effective_ignored: Vec<String> = {
            let mut ig: Vec<String> = DEFAULT_IGNORED_PATHS
                .iter()
                .map(|s| format!("{}/", s))
                .collect();
            ig.extend_from_slice(extra_ignored);
            ig
        };
        // Fast path: same ignore set as the current warm index — skip rebuild.
        {
            let tracked = match self.index_ignored.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            if tracked.as_ref() == Some(&effective_ignored) {
                return;
            }
        }
        let _rebuild_guard = match self.rebuild_lock.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        // Re-check under the rebuild lock in case another thread already
        // rebuilt with the same ignore set while we were waiting.
        {
            let tracked = match self.index_ignored.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            if tracked.as_ref() == Some(&effective_ignored) {
                return;
            }
        }
        let ws_root = self
            .deps
            .workspace
            .workspace_root(&FilePath::new(root.to_string_lossy().to_string()).unwrap_or_default())
            .unwrap_or_else(|| root.to_path_buf());
        let ignored = effective_ignored.clone();
        let abs_root = self.deps.io.canonicalize(&ws_root).unwrap_or(ws_root);
        let member_dirs: Vec<&str> = ["crates", "packages", "modules"]
            .iter()
            .filter(|d| abs_root.join(d).is_dir())
            .copied()
            .collect();
        let scanned: Vec<PathBuf> =
            shared_filesystem::utility_workspace_detection::discover_source_files(
                &abs_root, &ignored,
            )
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
        let parser_imports = self.deps.parser.import_list();
        // Snapshot before the parser cache gets overwritten by a scoped parse_all
        if let Ok(mut snap) = self.imports_snapshot.lock() {
            *snap = parser_imports.clone();
        }
        {
            let mut w = write_guard(&self.files);
            *w = Some(entries.clone());
        }
        {
            let mut w = write_guard(&self.imports);
            *w = Some(parser_imports);
        }
        {
            let mut w = write_guard(&self.file_index);
            *w = Some(
                entries
                    .iter()
                    .enumerate()
                    .map(|(i, e)| (e.path.clone(), i))
                    .collect(),
            );
        }
        {
            let mut w = write_guard(&self.warnings);
            *w = Some(self.deps.parser.parse_warnings().to_vec());
        }
        // Also clear the cached graph data so ensure_graph_built will rebuild it.
        {
            let mut w = write_guard(&self.cached_reverse_links);
            *w = None;
        }
        {
            let mut w = write_guard(&self.cached_definitions);
            *w = None;
        }
        {
            let mut w = write_guard(&self.cached_implementations);
            *w = None;
        }
        {
            let mut w = match self.index_ignored.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            *w = Some(effective_ignored);
        }
    }

    // Graph building & context assembly — symbol relationships plus synthetic wiring edges for orphan analysis.
    /// Builds and caches the dependency graph and its symbol relationships.
    pub fn ensure_graph_built(&self) {
        {
            let guard = read_guard(&self.cached_reverse_links);
            if guard.is_some() {
                return;
            }
        }
        let files = {
            let guard = read_guard(&self.files);
            guard.as_ref().cloned().unwrap_or_default()
        };
        let imports = {
            let guard = read_guard(&self.imports);
            guard.as_ref().cloned().unwrap_or_default()
        };
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
        {
            let mut w = write_guard(&self.cached_reverse_links);
            *w = Some(rl);
        }
        let sd = self.deps.graph.symbol_definitions().clone();
        {
            let mut w = write_guard(&self.cached_definitions);
            *w = Some(sd);
        }
        let imp = self.deps.graph.implementations().clone();
        {
            let mut w = write_guard(&self.cached_implementations);
            *w = Some(imp);
        }
    }

    /// Builds a graph analysis context for the workspace rooted at `root_dir`,
    /// including resolved import links, inbound links, inheritance relationships,
    /// implementation bridges, container wiring, and discovered files.
    fn build_orphan_graph_context(
        &self,
        root_dir: &Path,
        ignored: &[String],
    ) -> GraphAnalysisContext {
        self.build_file_index_with_ignored(root_dir, ignored);
        self.ensure_graph_built();
        let top_root = self
            .deps
            .workspace
            .workspace_root(
                &FilePath::new(root_dir.to_string_lossy().to_string()).unwrap_or_default(),
            )
            .unwrap_or_else(|| root_dir.to_path_buf());
        let all_files: Vec<String> = {
            let guard = read_guard(&self.files);
            guard
                .as_ref()
                .map(|entries| {
                    entries
                        .iter()
                        .map(|e| {
                            shared_filesystem::utility_container_wiring::path_to_relative(
                                &e.path, &top_root,
                            )
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        let all_files_set: HashSet<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let stem_index = build_stem_index(&all_files);
        let imports: Vec<ImportEntry> = {
            let guard = read_guard(&self.imports);
            guard.as_ref().cloned().unwrap_or_default()
        };
        let mut forward: HashMap<String, Vec<String>> = HashMap::new();
        let mut resolved_import_entries: Vec<ImportEntry> = Vec::with_capacity(imports.len());
        for imp in &imports {
            let src_rel = shared_filesystem::utility_container_wiring::path_to_relative(
                &imp.source_file,
                &top_root,
            );
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
                            shared_filesystem::utility_import_resolution::derive_crate_lib_rs(
                                tgt_rel,
                                &all_files_set,
                            )
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
                        .map(|p| {
                            shared_filesystem::utility_container_wiring::path_to_relative(
                                p, &top_root,
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        // P2: contract → capabilities bridge (reverse index, issue #193).
        // For every trait/interface/base, wire its defining (contract) file to each
        // implementor so BFS that reaches a contract also reaches its capabilities.
        shared_filesystem::utility_container_wiring::add_impl_bridge_edges(
            &top_root,
            self.deps.graph.symbol_definitions(),
            self.deps.graph.implementations(),
            &mut forward,
        );

        // P1: Container-aware wiring — resolve each *_container.* file's used identifiers
        // against the workspace symbol table and add synthetic edges container →
        // referenced file, so BFS that reaches the container reaches its DI services.
        shared_filesystem::utility_container_wiring::add_container_wiring_edges(
            &all_files,
            &top_root,
            self.deps.graph.symbol_definitions(),
            |p: &Path| self.used_identifiers_for(p),
            &mut forward,
        );

        shared_filesystem::utility_container_wiring::add_lib_rs_edges(&all_files, &mut forward);

        // Build the reverse (inbound) link map only after all synthetic edges
        // (impl bridges, container wiring, lib.rs) have been added, so taxonomy
        // and utility analyzers see the same inbound links the surfaces analyzer does.
        let reverse = shared_filesystem::utility_container_wiring::build_reverse_index(&forward);
        GraphAnalysisContext::new(
            ImportGraph::new(forward),
            InboundLinkMap::new(reverse),
            InheritanceMap::new(inheritance),
            all_files,
        )
    }
}

// PURPOSE: FilesystemOrchestrator pipeline helpers — file index building,
// stem indexing, and graph construction.
//
// These are the `pub` pipeline entry points called by other subsystems;
// they live in a second agent file so the primary agent file stays under the size limit.
use crate::agent_filesystem_orchestrator::FilesystemOrchestrator;
use shared_common::{
    DEFAULT_IGNORED_PATHS,
    taxonomy_language_vo::ConfigLanguage,
};
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImplEntry, Language, ParseMetadata,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// Coordinates the filesystem aggregate and the primary orchestrator to
/// run the index/graph pipeline.
pub struct FilesystemPipelineOrchestrator {
    orchestrator: Arc<FilesystemOrchestrator>,
    fs_aggregate: Arc<dyn IFilesystemAggregate>,
    parser: Arc<dyn shared_filesystem::contract_filesystem_protocol::IParserProtocol>,
    workspace: Arc<dyn shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol>,
}

impl IFilesystemAggregate for FilesystemPipelineOrchestrator {
    fn execute(
        &self,
        request: shared_filesystem::FilesystemRequest,
    ) -> shared_filesystem::FilesystemResponse {
        self.orchestrator.execute(request)
    }
}

impl FilesystemPipelineOrchestrator {
    pub fn new(
        orchestrator: Arc<FilesystemOrchestrator>,
        fs_aggregate: Arc<dyn IFilesystemAggregate>,
        parser: Arc<dyn shared_filesystem::contract_filesystem_protocol::IParserProtocol>,
        workspace: Arc<dyn shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol>,
    ) -> Self {
        Self {
            orchestrator,
            fs_aggregate,
            parser,
            workspace,
        }
    }

    /// Runs the full pipeline: rebuild the index, then ensure the graph is
    /// cached and up to date.
    pub fn run_pipeline(&self, root: &Path, ignored: &[String]) {
        self.orchestrator.build_file_index_impl(root, ignored);
        self.orchestrator.ensure_graph_built();
    }

    /// Returns the workspace root for `start`, delegating to the workspace
    /// protocol so the pipeline respects the same discovery logic.
    pub fn workspace_root(&self, start: &Path) -> Option<PathBuf> {
        self.workspace
            .workspace_root(&FilePath::new(start.to_string_lossy().to_string()).unwrap_or_default())
    }

    /// Executes a filesystem request through the aggregate, so callers that
    /// only hold the pipeline orchestrator can still issue I/O.
    pub fn fs_execute(
        &self,
        request: shared_filesystem::FilesystemRequest,
    ) -> shared_filesystem::FilesystemResponse {
        self.fs_aggregate.execute(request)
    }

    /// Returns the parsed imports for `path`, used by the graph builder to
    /// fill missing import edges without re-parsing the file.
    pub fn imports_for(&self, path: &Path) -> Vec<shared_filesystem::ImportEntry> {
        self.parser.imports_for(path)
    }
}

// ─── Block 2: Pipeline Methods on FilesystemOrchestrator ──────
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
            let mut w = Self::write_guard(&self.files);
            *w = Some(entries.clone());
        }
        {
            let mut w = Self::write_guard(&self.imports);
            *w = Some(parser_imports);
        }
        {
            let mut w = Self::write_guard(&self.file_index);
            *w = Some(
                entries
                    .iter()
                    .enumerate()
                    .map(|(i, e)| (e.path.clone(), i))
                    .collect(),
            );
        }
        {
            let mut w = Self::write_guard(&self.warnings);
            *w = Some(self.deps.parser.parse_warnings().to_vec());
        }
        // Also clear the cached graph data so ensure_graph_built will rebuild it.
        {
            let mut w = Self::write_guard(&self.cached_reverse_links);
            *w = None;
        }
        {
            let mut w = Self::write_guard(&self.cached_definitions);
            *w = None;
        }
        {
            let mut w = Self::write_guard(&self.cached_implementations);
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
    /// Builds and caches the dependency graph and its symbol relationships.
    pub fn ensure_graph_built(&self) {
        {
            let guard = Self::read_guard(&self.cached_reverse_links);
            if guard.is_some() {
                return;
            }
        }
        let files = {
            let guard = Self::read_guard(&self.files);
            guard.as_ref().cloned().unwrap_or_default()
        };
        let imports = {
            let guard = Self::read_guard(&self.imports);
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
            let mut w = Self::write_guard(&self.cached_reverse_links);
            *w = Some(rl);
        }
        let sd = self.deps.graph.symbol_definitions().clone();
        {
            let mut w = Self::write_guard(&self.cached_definitions);
            *w = Some(sd);
        }
        let imp = self.deps.graph.implementations().clone();
        {
            let mut w = Self::write_guard(&self.cached_implementations);
            *w = Some(imp);
        }
    }
}

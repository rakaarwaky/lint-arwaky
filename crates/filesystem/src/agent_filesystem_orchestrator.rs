// PURPOSE: Agent layer — filesystem orchestrator (FR-005)
// Single entry point that orchestrates FR-001 through FR-004.
// Pipeline runs once (lazy: triggered on first accessor call).
// Results cached internally, served to all consumers via reference.
// Implements IFilesystemAggregate trait.

use crate::capabilities_ast_parser::ASTParser;
use crate::capabilities_dependency_graph::DependencyGraph;
use crate::capabilities_file_walker::FileWalker;
use crate::capabilities_import_extractor;
use crate::utility_filesystem_io;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock, LazyLock};
use std::time::Instant;

// ═══════════════════════════════════════════════════════════════
// FR-005: Filesystem Orchestrator — Cached Pipeline Results
// ═══════════════════════════════════════════════════════════════

/// Cached results from a completed pipeline run.
struct CachedResults {
    files: Vec<FileEntry>,
    imports: Vec<ImportEntry>,
    warnings: Vec<ParseWarning>,
    reverse_links: HashMap<PathBuf, Vec<PathBuf>>,
    definitions: HashMap<String, Vec<PathBuf>>,
    implementations: HashMap<String, Vec<PathBuf>>,
    timing: ScanTiming,
}

/// Filesystem orchestrator — single entry point that runs the full pipeline.
/// Delegates all I/O and computation to capabilities layer.
/// FR-005: Pipeline runs once, results cached and served via references.
pub struct FilesystemOrchestrator {
    walker: FileWalker,
    parser: ASTParser,
    graph: RwLock<DependencyGraph>,
    cached: OnceLock<CachedResults>,
}

impl FilesystemOrchestrator {
    pub fn new() -> Self {
        Self {
            walker: FileWalker::new(),
            parser: ASTParser::new(),
            graph: RwLock::new(DependencyGraph::new()),
            cached: OnceLock::new(),
        }
    }

    /// Get the dependency graph (for queries outside the aggregate trait).
    pub fn graph(&self) -> &RwLock<DependencyGraph> {
        &self.graph
    }

    /// Run the full pipeline: walk -> parse -> extract -> graph.
    /// FR-005: Pipeline runs once, results cached internally.
    fn run_pipeline_internal(&self, root: &PathBuf, ignored: &[String]) {
        let extensions = Language::extensions();
        let mut timing = ScanTiming::default();

        // Stage 1: Walk (FR-001)
        let t = Instant::now();
        let mut files = self.walker.walk(root, ignored, extensions);
        timing.walk_ms = t.elapsed().as_millis() as u64;

        // Stage 2: Parse ASTs (FR-002)
        let t = Instant::now();
        self.parser.parse_all(&mut files);
        timing.parse_ms = t.elapsed().as_millis() as u64;

        // Count parse results
        let parsed_count = files.iter().filter(|f| f.parse_ok).count();
        let _parse_errors = files.len() - parsed_count;

        // Emit parse warnings
        let warnings: Vec<ParseWarning> = files
            .iter()
            .filter(|f| !f.parse_ok)
            .map(|f| ParseWarning {
                file_path: f.path.clone(),
                error_detail: "parse failure".to_string(),
            })
            .collect();

        // Stage 3: Extract imports (FR-003)
        let t = Instant::now();
        let mut all_imports = Vec::new();
        for file in &files {
            if !file.parse_ok || file.content.is_empty() {
                continue;
            }
            let imports =
                capabilities_import_extractor::extract_imports(&file.path, &file.content, file.language);
            all_imports.extend(imports);
        }
        timing.extract_ms = t.elapsed().as_millis() as u64;

        // Extract definitions and implementations from parse metadata
        let (definitions, implementations) = extract_definitions_and_impls(&files);

        let _unresolved_imports = all_imports.iter().filter(|i| !i.is_resolved).count();

        // Stage 4: Build graph (FR-004)
        let t = Instant::now();
        {
            let mut graph = self.graph.write().unwrap();
            graph.build(&all_imports, &files, &definitions, &implementations);
        }
        timing.graph_ms = t.elapsed().as_millis() as u64;

        timing.total_ms = timing.walk_ms + timing.parse_ms + timing.extract_ms + timing.graph_ms;

        // Cache all results
        let graph = self.graph.read().unwrap();
        let _ = self.cached.set(CachedResults {
            files,
            imports: all_imports,
            warnings,
            reverse_links: graph.reverse_links().clone(),
            definitions: graph.definitions().clone(),
            implementations: graph.implementations().clone(),
            timing,
        });
    }
}

impl Default for FilesystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract definitions and implementations from parse metadata across all files.
fn extract_definitions_and_impls(
    files: &[FileEntry],
) -> (Vec<DefinitionEntry>, Vec<ImplEntry>) {
    let mut definitions = Vec::new();
    let mut implementations = Vec::new();

    for file in files {
        if !file.parse_ok {
            continue;
        }
        if let Some(meta) = &file.parse_metadata {
            match meta {
                ParseMetadata::Rust(rust_meta) => {
                    // Struct/Enum/Trait/Type definitions
                    for name in &rust_meta.struct_definitions {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::Rust,
                        });
                    }
                    for name in &rust_meta.enum_definitions {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::Rust,
                        });
                    }
                    for name in &rust_meta.trait_definitions {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::Rust,
                        });
                    }
                    for name in &rust_meta.type_definitions {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::Rust,
                        });
                    }
                    // Impl blocks
                    for imp in &rust_meta.impl_blocks {
                        if let Some(trait_name) = &imp.trait_name {
                            implementations.push(ImplEntry {
                                trait_name: trait_name.clone(),
                                file_path: file.path.clone(),
                                language: Language::Rust,
                            });
                        }
                    }
                }
                ParseMetadata::Python(py_meta) => {
                    // Class definitions (with base classes for impl map)
                    for class in &py_meta.class_declarations {
                        definitions.push(DefinitionEntry {
                            name: class.name.clone(),
                            file_path: file.path.clone(),
                            language: Language::Python,
                        });
                        for base in &class.bases {
                            implementations.push(ImplEntry {
                                trait_name: base.clone(),
                                file_path: file.path.clone(),
                                language: Language::Python,
                            });
                        }
                    }
                }
                ParseMetadata::TypeScript(ts_meta)
                | ParseMetadata::JavaScript(ts_meta) => {
                    // Class definitions
                    for class in &ts_meta.class_declarations {
                        definitions.push(DefinitionEntry {
                            name: class.name.clone(),
                            file_path: file.path.clone(),
                            language: Language::TypeScript,
                        });
                        for iface in &class.implements {
                            implementations.push(ImplEntry {
                                trait_name: iface.clone(),
                                file_path: file.path.clone(),
                                language: Language::TypeScript,
                            });
                        }
                    }
                    // Interface definitions
                    for name in &ts_meta.interface_declarations {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::TypeScript,
                        });
                    }
                    // Type alias definitions
                    for name in &ts_meta.type_alias_declarations {
                        definitions.push(DefinitionEntry {
                            name: name.clone(),
                            file_path: file.path.clone(),
                            language: Language::TypeScript,
                        });
                    }
                }
            }
        }
    }

    (definitions, implementations)
}

// ═══════════════════════════════════════════════════════════════
// FR-005: IFilesystemAggregate Implementation
// ═══════════════════════════════════════════════════════════════

impl IFilesystemAggregate for FilesystemOrchestrator {
    // ── Pipeline Trigger (FR-005) ─────────────────────────────

    fn run_pipeline(&self, root: &PathBuf, ignored: &[String]) {
        if self.cached.get().is_none() {
            self.run_pipeline_internal(root, ignored);
        }
    }

    // ── File Access (FR-001) ─────────────────────────────────

    fn file_list(&self) -> &[FileEntry] {
        self.cached
            .get()
            .map(|c| c.files.as_slice())
            .unwrap_or(&[])
    }

    // ── Parsed File Access (FR-002) ──────────────────────────

    fn parsed_file_list(&self) -> &[FileEntry] {
        // Same as file_list — files include parse_ok and parse_metadata
        self.file_list()
    }

    // ── Parse Warnings (FR-002) ──────────────────────────────

    fn parse_warnings(&self) -> &[ParseWarning] {
        self.cached
            .get()
            .map(|c| c.warnings.as_slice())
            .unwrap_or(&[])
    }

    // ── Import Access (FR-003) ───────────────────────────────

    fn import_list(&self) -> &[ImportEntry] {
        self.cached
            .get()
            .map(|c| c.imports.as_slice())
            .unwrap_or(&[])
    }

    // ── Graph Access (FR-004) ────────────────────────────────

    fn dependency_graph(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        // Return reverse_links as the "dependency graph" for backward compat
        static EMPTY: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        self.cached
            .get()
            .map(|c| &c.reverse_links)
            .unwrap_or(&EMPTY)
    }

    fn reverse_import_map(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        static EMPTY: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        self.cached
            .get()
            .map(|c| &c.reverse_links)
            .unwrap_or(&EMPTY)
    }

    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        static EMPTY: HashMap<String, Vec<PathBuf>> = HashMap::new();
        self.cached
            .get()
            .map(|c| &c.definitions)
            .unwrap_or(&EMPTY)
    }

    fn trait_implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        static EMPTY: HashMap<String, Vec<PathBuf>> = HashMap::new();
        self.cached
            .get()
            .map(|c| &c.implementations)
            .unwrap_or(&EMPTY)
    }

    // ── Timing ───────────────────────────────────────────────

    fn timing(&self) -> &ScanTiming {
        static DEFAULT_TIMING: ScanTiming = ScanTiming {
            walk_ms: 0,
            cache_ms: 0,
            parse_ms: 0,
            extract_ms: 0,
            graph_ms: 0,
            total_ms: 0,
        };
        self.cached
            .get()
            .map(|c| &c.timing)
            .unwrap_or(&DEFAULT_TIMING)
    }

    // ── File Reading (backward compat) ───────────────────────

    fn read_file(&self, path: &Path) -> Option<String> {
        // Check cached files first
        if let Some(cached) = self.cached.get() {
            if let Some(entry) = cached.files.iter().find(|f| f.path == path) {
                return Some(entry.content.clone());
            }
        }
        utility_filesystem_io::cache_get(&path.to_path_buf())
            .or_else(|| utility_filesystem_io::read_file(path).ok())
    }

    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String> {
        utility_filesystem_io::read_lintable_file(path)
    }

    fn get_file_content(&self, path: &PathBuf) -> Option<String> {
        if let Some(cached) = self.cached.get() {
            if let Some(entry) = cached.files.iter().find(|f| &f.path == path) {
                return Some(entry.content.clone());
            }
        }
        utility_filesystem_io::cache_get(path)
            .or_else(|| utility_filesystem_io::read_file(path).ok())
    }

    fn has_file(&self, path: &PathBuf) -> bool {
        if let Some(cached) = self.cached.get() {
            return cached.files.iter().any(|f| &f.path == path);
        }
        utility_filesystem_io::cache_contains(path)
    }

    // ── File Discovery (backward compat) ─────────────────────

    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry> {
        let extensions = Language::extensions();
        self.walker
            .walk(&root.to_path_buf(), ignored, extensions)
    }

    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath> {
        let mut files = Vec::new();
        utility_filesystem_io::walk_source_files(root, &mut files, ignored);
        files
    }

    // ── Import/Dependency (backward compat) ──────────────────

    fn imports_for(&self, path: &PathBuf) -> Vec<ImportEntry> {
        if let Some(cached) = self.cached.get() {
            return cached
                .imports
                .iter()
                .filter(|i| i.source_file == *path)
                .cloned()
                .collect();
        }
        Vec::new()
    }

    fn all_imports(&self) -> &[ImportEntry] {
        self.import_list()
    }

    fn depends_on(&self, from: &PathBuf, to: &PathBuf) -> bool {
        self.graph.read().unwrap().reachable(from, to)
    }

    fn cycles(&self) -> Vec<Vec<PathBuf>> {
        self.graph.read().unwrap().cycles()
    }

    fn orphan_files(&self) -> Vec<PathBuf> {
        self.graph.read().unwrap().orphan_files()
    }

    // ── Path Queries ─────────────────────────────────────────

    fn path_exists(&self, path: &Path) -> bool {
        utility_filesystem_io::path_exists(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        utility_filesystem_io::is_dir(path)
    }

    fn should_ignore(&self, path: &str, ignored: &[String]) -> bool {
        utility_filesystem_io::is_path_ignored(path, ignored)
    }

    // ── Workspace ────────────────────────────────────────────

    fn workspace_root(&self, start: &str) -> Option<PathBuf> {
        utility_filesystem_io::find_workspace_root(start)
    }
}

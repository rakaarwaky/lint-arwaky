// PURPOSE: Agent layer — filesystem orchestrator (FR-006)
// Orchestrates: walk → cache → parse → extract → graph
// Zero I/O, zero business logic — delegates to capabilities.

use crate::capabilities_ast_parser::ASTParser;
use crate::capabilities_dependency_graph::DependencyGraph;
use crate::capabilities_file_cache::FileCache;
use crate::capabilities_file_walker::FileWalker;
use crate::capabilities_import_extractor;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::*;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::Instant;

// ─── Block 1: Struct Definition ────────────────────────────

/// Filesystem orchestrator — single entry point that runs the full pipeline.
/// Delegates all I/O and computation to capabilities layer.
pub struct FilesystemOrchestrator {
    walker: FileWalker,
    cache: FileCache,
    parser: ASTParser,
    graph: RwLock<DependencyGraph>,
}

// ─── Block 2: Aggregate Trait Implementation ───────────────

impl IFilesystemAggregate for FilesystemOrchestrator {
    fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult {
        let extensions = Language::extensions();
        let mut timing = ScanTiming::default();

        // Stage 1: Walk (FR-001)
        let t = Instant::now();
        let files = self.walker.walk(root, ignored, extensions);
        timing.walk_ms = t.elapsed().as_millis() as u64;

        // Stage 2: Cache (FR-002)
        let t = Instant::now();
        self.cache.populate(&files);
        timing.cache_ms = t.elapsed().as_millis() as u64;

        // Stage 3: Parse ASTs (FR-003)
        let t = Instant::now();
        let cache_ref = &self.cache;
        self.parser.parse_all(&files, &|path| cache_ref.get(path));
        timing.parse_ms = t.elapsed().as_millis() as u64;

        // Stage 4: Extract imports (FR-004)
        let t = Instant::now();
        let mut all_imports = Vec::new();
        let mut parsed_count = 0;
        let mut parse_errors = 0;

        for file in &files {
            if let Some(content) = self.cache.get(&file.path) {
                let imports = capabilities_import_extractor::extract_imports(
                    &file.path,
                    &content,
                    file.language,
                );
                if !imports.is_empty() || !content.is_empty() {
                    parsed_count += 1;
                }
                all_imports.extend(imports);
            } else {
                parse_errors += 1;
            }
        }
        timing.extract_ms = t.elapsed().as_millis() as u64;

        // Stage 5: Build graph (FR-005)
        let t = Instant::now();
        let mut graph = self.graph.write().unwrap();
        graph.build(&all_imports, &files);
        timing.graph_ms = t.elapsed().as_millis() as u64;

        let unresolved_imports = all_imports.iter().filter(|i| !i.is_resolved).count();
        timing.total_ms = timing.walk_ms
            + timing.cache_ms
            + timing.parse_ms
            + timing.extract_ms
            + timing.graph_ms;

        FilesystemResult {
            files,
            imports: all_imports,
            parsed_count,
            parse_errors,
            unresolved_imports,
            timing,
        }
    }

    fn get_file_content(&self, path: &PathBuf) -> Option<String> {
        self.cache.get(path)
    }

    fn has_file(&self, path: &PathBuf) -> bool {
        self.cache.contains(path)
    }

    fn all_files(&self) -> &[FileEntry] {
        &[]
    }

    fn imports_for(&self, _path: &PathBuf) -> Vec<ImportEntry> {
        Vec::new()
    }

    fn all_imports(&self) -> &[ImportEntry] {
        &[]
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

    fn timing(&self) -> &ScanTiming {
        static DEFAULT_TIMING: ScanTiming = ScanTiming {
            walk_ms: 0,
            cache_ms: 0,
            parse_ms: 0,
            extract_ms: 0,
            graph_ms: 0,
            total_ms: 0,
        };
        &DEFAULT_TIMING
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl FilesystemOrchestrator {
    pub fn new() -> Self {
        Self {
            walker: FileWalker::new(),
            cache: FileCache::new(),
            parser: ASTParser::new(),
            graph: RwLock::new(DependencyGraph::new()),
        }
    }

    /// Get the dependency graph (for queries outside the aggregate trait).
    pub fn graph(&self) -> &RwLock<DependencyGraph> {
        &self.graph
    }
}

impl Default for FilesystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// PURPOSE: Agent layer — filesystem orchestrator
// Wires capabilities together: walk → cache → parse → extract → graph
// Other crates import IFilesystemAggregate from shared and receive this via DI.

use shared::filesystem::taxonomy_filesystem_vo::*;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use crate::capabilities_file_walker::FileWalker;
use crate::capabilities_file_cache::FileCache;
use crate::capabilities_ast_parser::ASTParser;
use crate::capabilities_import_extractor::ImportExtractor;
use crate::capabilities_dependency_graph::DependencyGraph;
use std::path::PathBuf;
use std::collections::HashSet;

pub struct FilesystemOrchestrator {
    walker: FileWalker,
    cache: FileCache,
    parser: ASTParser,
    extractor: ImportExtractor,
    graph: DependencyGraph,
    files: Vec<FileEntry>,
    imports: Vec<ImportEntry>,
    timing: ScanTiming,
}

impl FilesystemOrchestrator {
    pub fn new() -> Self {
        Self {
            walker: FileWalker::new(),
            cache: FileCache::new(),
            parser: ASTParser::new(),
            extractor: ImportExtractor::new(),
            graph: DependencyGraph::new(),
            files: Vec::new(),
            imports: Vec::new(),
            timing: ScanTiming::default(),
        }
    }
}

impl Default for FilesystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl IFilesystemAggregate for FilesystemOrchestrator {
    fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult {
        let start = std::time::Instant::now();

        // Stage 1: Walk filesystem
        let walk_start = std::time::Instant::now();
        let files = self.walker.walk(root, ignored, Language::extensions());
        let walk_ms = walk_start.elapsed().as_millis() as u64;

        // Stage 2: Cache file contents
        let cache_start = std::time::Instant::now();
        self.cache.populate(&files);
        let cache_ms = cache_start.elapsed().as_millis() as u64;

        // Stage 3: Parse ASTs
        let parse_start = std::time::Instant::now();
        let cache_ref = &self.cache;
        self.parser.parse_all(&files, &|p| cache_ref.get(p));
        let parse_ms = parse_start.elapsed().as_millis() as u64;

        // Stage 4: Extract imports
        let extract_start = std::time::Instant::now();
        let mut all_imports = Vec::new();
        for entry in &files {
            if let Some(content) = self.cache.get(&entry.path) {
                let imps = self.extractor.extract(&entry.path, &content, entry.language);
                all_imports.extend(imps);
            }
        }
        let extract_ms = extract_start.elapsed().as_millis() as u64;

        // Stage 5: Build dependency graph
        let graph_start = std::time::Instant::now();
        self.graph.build(&all_imports, &files);
        let graph_ms = graph_start.elapsed().as_millis() as u64;

        let total_ms = start.elapsed().as_millis() as u64;

        FilesystemResult {
            files,
            imports: all_imports,
            parsed_count: 0, // TODO: track from parser
            parse_errors: 0,
            unresolved_imports: 0,
            timing: ScanTiming {
                walk_ms,
                cache_ms,
                parse_ms,
                extract_ms,
                graph_ms,
                total_ms,
            },
        }
    }

    fn get_file_content(&self, path: &PathBuf) -> Option<String> {
        self.cache.get(path)
    }

    fn has_file(&self, path: &PathBuf) -> bool {
        self.cache.contains(path)
    }

    fn all_files(&self) -> &[FileEntry] {
        &self.files
    }

    fn imports_for(&self, path: &PathBuf) -> Vec<ImportEntry> {
        self.imports.iter().filter(|i| i.source_file == *path).cloned().collect()
    }

    fn all_imports(&self) -> &[ImportEntry] {
        &self.imports
    }

    fn depends_on(&self, from: &PathBuf, to: &PathBuf) -> bool {
        self.graph.reachable(from, to)
    }

    fn cycles(&self) -> Vec<Vec<PathBuf>> {
        self.graph.cycles()
    }

    fn orphan_files(&self) -> Vec<PathBuf> {
        self.graph.orphan_files()
    }

    fn timing(&self) -> &ScanTiming {
        &self.timing
    }
}

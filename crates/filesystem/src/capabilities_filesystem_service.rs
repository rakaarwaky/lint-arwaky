// PURPOSE: Capabilities layer — filesystem service facade
// Orchestrates: walk → cache → parse → extract → graph
// Single entry point for all rule crates.

use crate::utility_ast_parser::ASTParser;
use crate::utility_dependency_graph::DependencyGraph;
use crate::utility_file_cache::FileCache;
use crate::utility_file_walker::FileWalker;
use crate::utility_import_extractor;
use shared::filesystem::*;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::Instant;

pub struct FilesystemService {
    walker: FileWalker,
    cache: FileCache,
    parser: ASTParser,
    graph: RwLock<DependencyGraph>,
}

impl FilesystemService {
    pub fn new() -> Self {
        Self {
            walker: FileWalker::new(),
            cache: FileCache::new(),
            parser: ASTParser::new(),
            graph: RwLock::new(DependencyGraph::new()),
        }
    }

    pub fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult {
        let extensions = Language::extensions();
        let mut timing = ScanTiming::default();

        // Stage 1: Walk
        let t = Instant::now();
        let files = self.walker.walk(root, ignored, extensions);
        timing.walk_ms = t.elapsed().as_millis() as u64;

        // Stage 2: Cache
        let t = Instant::now();
        self.cache.populate(&files);
        timing.cache_ms = t.elapsed().as_millis() as u64;

        // Stage 3: Parse ASTs
        let t = Instant::now();
        let cache_ref = &self.cache;
        self.parser.parse_all(&files, &|path| cache_ref.get(path));
        timing.parse_ms = t.elapsed().as_millis() as u64;

        // Stage 4: Extract imports
        let t = Instant::now();
        let mut all_imports = Vec::new();
        let mut parsed_count = 0;
        let mut parse_errors = 0;

        for file in &files {
            if let Some(content) = self.cache.get(&file.path) {
                let imports =
                    utility_import_extractor::extract_imports(&file.path, &content, file.language);
                if !imports.is_empty() || !content.is_empty() {
                    parsed_count += 1;
                }
                all_imports.extend(imports);
            } else {
                parse_errors += 1;
            }
        }
        timing.extract_ms = t.elapsed().as_millis() as u64;

        // Stage 5: Build graph
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

    pub fn graph(&self) -> &RwLock<DependencyGraph> {
        &self.graph
    }

    pub fn get_file_content(&self, path: &PathBuf) -> Option<String> {
        self.cache.get(path)
    }

    pub fn has_file(&self, path: &PathBuf) -> bool {
        self.cache.contains(path)
    }
}

impl Default for FilesystemService {
    fn default() -> Self {
        Self::new()
    }
}

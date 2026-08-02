// PURPOSE: Agent layer — filesystem orchestrator (FR-005)
// Single entry point that orchestrates FR-001 through FR-004.
// Pipeline runs once (lazy: triggered on first accessor call).
// Results cached internally, served to all consumers via reference.
// Implements IFilesystemAggregate trait.

use crate::utility_filesystem_io;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_source_vo::ContentString;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IImportExtractorProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, FilesystemResult, GraphData, ImplEntry, ImportEntry, Language,
    ParseMetadata, ParseWarning, ScanTiming,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, OnceLock, RwLock};
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
    walker: crate::capabilities_file_walker::FileWalker,
    parser: crate::capabilities_ast_parser::ASTParser,
    graph: RwLock<crate::capabilities_dependency_graph::DependencyGraph>,
    cached: OnceLock<CachedResults>,
}

impl FilesystemOrchestrator {
    pub fn new() -> Self {
        Self {
            walker: crate::capabilities_file_walker::FileWalker::new(),
            parser: crate::capabilities_ast_parser::ASTParser::new(),
            graph: RwLock::new(crate::capabilities_dependency_graph::DependencyGraph::new()),
            cached: OnceLock::new(),
        }
    }

    /// Get the dependency graph (for queries outside the aggregate trait).
    pub fn graph(&self) -> &RwLock<crate::capabilities_dependency_graph::DependencyGraph> {
        &self.graph
    }

    /// Run the full pipeline: walk -> parse -> extract -> graph.
    /// FR-005: Pipeline runs once, results cached internally.
    fn run_pipeline_internal(&self, root: &Path, ignored: &[String]) {
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
            let extractor = crate::capabilities_import_extractor::ImportExtractor;
            let imports = extractor.extract(&file.path, &file.content, file.language);
            all_imports.extend(imports);
        }
        timing.extract_ms = t.elapsed().as_millis() as u64;

        // Extract definitions and implementations from parse metadata
        let (definitions, implementations) = extract_definitions_and_impls(&files);

        let _unresolved_imports = all_imports.iter().filter(|i| !i.is_resolved).count();

        // Stage 4: Build graph (FR-004)
        let t = Instant::now();
        {
            match self.graph.write() {
                Ok(mut graph) => {
                    graph.build(&all_imports, &files, &definitions, &implementations);
                }
                Err(_) => return,
            }
        }
        timing.graph_ms = t.elapsed().as_millis() as u64;

        timing.total_ms = timing.walk_ms + timing.parse_ms + timing.extract_ms + timing.graph_ms;

        // Cache all results
        if let Ok(graph) = self.graph.read() {
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

    // ── Async File I/O ─────────────────────────────────────

    pub async fn read_file_async(&self, path: &Path) -> std::io::Result<String> {
        tokio::fs::read_to_string(path).await
    }

    pub async fn read_text_within_canonical_root(
        &self,
        path: &Path,
        canonical_root: &Path,
    ) -> std::io::Result<String> {
        utility_filesystem_io::read_text_within_canonical_root(path, canonical_root).await
    }
}

impl Default for FilesystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract definitions and implementations from parse metadata across all files.
fn extract_definitions_and_impls(files: &[FileEntry]) -> (Vec<DefinitionEntry>, Vec<ImplEntry>) {
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
                ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
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
                _ => {} // ParseMetadata::Unknown — skip
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

    fn run_pipeline(&self, root: &Path, ignored: &[String]) {
        if self.cached.get().is_none() {
            self.run_pipeline_internal(root, ignored);
        }
    }

    fn scan(&self, root: &Path, ignored: &[String]) -> FilesystemResult {
        self.run_pipeline(root, ignored);
        match self.cached.get() {
            Some(cached) => FilesystemResult {
                files: cached.files.clone(),
                imports: cached.imports.clone(),
                warnings: cached.warnings.clone(),
                graph: GraphData::default(),
                parsed_count: cached.files.iter().filter(|f| f.parse_ok).count(),
                parse_errors: cached.files.iter().filter(|f| !f.parse_ok).count(),
                unresolved_imports: cached.imports.iter().filter(|i| !i.is_resolved).count(),
                timing: cached.timing.clone(),
            },
            None => FilesystemResult {
                files: Vec::new(),
                imports: Vec::new(),
                warnings: Vec::new(),
                graph: GraphData::default(),
                parsed_count: 0,
                parse_errors: 0,
                unresolved_imports: 0,
                timing: ScanTiming::default(),
            },
        }
    }

    // ── File Access (FR-001) ─────────────────────────────────

    fn file_list(&self) -> &[FileEntry] {
        self.cached.get().map(|c| c.files.as_slice()).unwrap_or(&[])
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
        static EMPTY: LazyLock<HashMap<PathBuf, Vec<PathBuf>>> = LazyLock::new(HashMap::new);
        self.cached
            .get()
            .map(|c| &c.reverse_links)
            .unwrap_or(&EMPTY)
    }

    fn reverse_import_map(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        static EMPTY: LazyLock<HashMap<PathBuf, Vec<PathBuf>>> = LazyLock::new(HashMap::new);
        self.cached
            .get()
            .map(|c| &c.reverse_links)
            .unwrap_or(&EMPTY)
    }

    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>> {
        static EMPTY: LazyLock<HashMap<String, Vec<PathBuf>>> = LazyLock::new(HashMap::new);
        self.cached.get().map(|c| &c.definitions).unwrap_or(&EMPTY)
    }

    fn trait_implementations(&self) -> &HashMap<String, Vec<PathBuf>> {
        static EMPTY: LazyLock<HashMap<String, Vec<PathBuf>>> = LazyLock::new(HashMap::new);
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
        if let Some(cached) = self.cached.get()
            && let Some(entry) = cached.files.iter().find(|f| f.path == path)
        {
            return Some(entry.content.clone());
        }
        utility_filesystem_io::cache_get(&path.to_path_buf())
            .or_else(|| utility_filesystem_io::read_file(path).ok())
    }

    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String> {
        utility_filesystem_io::read_lintable_file(path)
    }

    fn get_file_content(&self, path: &Path) -> Option<String> {
        if let Some(cached) = self.cached.get()
            && let Some(entry) = cached.files.iter().find(|f| f.path == path)
        {
            return Some(entry.content.clone());
        }
        utility_filesystem_io::cache_get(&path.to_path_buf())
            .or_else(|| utility_filesystem_io::read_file(path).ok())
    }

    fn has_file(&self, path: &Path) -> bool {
        if let Some(cached) = self.cached.get() {
            return cached.files.iter().any(|f| f.path == path);
        }
        utility_filesystem_io::cache_contains(&path.to_path_buf())
    }

    // ── File Discovery (backward compat) ─────────────────────

    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry> {
        let extensions = Language::extensions();
        self.walker.walk(root, ignored, extensions)
    }

    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath> {
        let mut files = Vec::new();
        utility_filesystem_io::walk_source_files(root, &mut files, ignored);
        files
    }

    // ── Import/Dependency (backward compat) ──────────────────

    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
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

    fn depends_on(&self, from: &Path, to: &Path) -> bool {
        match self.graph.read() {
            Ok(graph) => graph.reachable(from, to),
            Err(_) => false,
        }
    }

    fn cycles(&self) -> Vec<Vec<PathBuf>> {
        match self.graph.read() {
            Ok(graph) => graph.cycles(),
            Err(_) => Vec::new(),
        }
    }

    fn orphan_files(&self) -> Vec<PathBuf> {
        match self.graph.read() {
            Ok(graph) => graph.orphan_files(),
            Err(_) => Vec::new(),
        }
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

    // ── Directory Operations ─────────────────────────────────

    fn scan_directory(&self, dir: &Path) -> Vec<PathBuf> {
        let entries = utility_filesystem_io::scan_directory(dir);
        entries
            .into_iter()
            .map(|(_, path_str, _)| PathBuf::from(path_str))
            .collect()
    }

    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
        let dir_path = DirectoryPath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        match utility_filesystem_io::scan_directory_with_ignored(&dir_path, ignored) {
            Ok(entries) => entries
                .values
                .into_iter()
                .map(|fp| PathBuf::from(&fp.value))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool {
        utility_filesystem_io::is_ignored_dir(dir, ignored)
    }

    // ── Path Metadata ────────────────────────────────────────

    fn is_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_file(path)
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        std::fs::canonicalize(path)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        std::fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
    }

    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        std::fs::metadata(path)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        std::fs::symlink_metadata(path)
    }

    // ── Path Utilities ─────────────────────────────────────────

    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_file_stem(path)
    }

    // ── Path Discovery Helpers ────────────────────────────────

    fn has_python_files(&self, dir: &Path) -> bool {
        utility_filesystem_io::has_python_files(dir)
    }

    fn has_config_file(&self, dir: &Path) -> bool {
        utility_filesystem_io::has_config_file(dir)
    }

    fn has_cargo_toml(&self, path_str: &str) -> Option<String> {
        utility_filesystem_io::has_cargo_toml(path_str)
    }

    fn has_cargo_lock(&self, path_str: &str) -> Option<String> {
        utility_filesystem_io::has_cargo_lock(path_str)
    }

    fn is_executable_in_path(&self, executable: &str) -> bool {
        utility_filesystem_io::is_executable_in_path(executable)
    }

    fn has_local_bin(&self, working_dir: &Path, executable: &str) -> bool {
        utility_filesystem_io::has_local_bin(working_dir, executable)
    }

    // ── Write Operations (setup/hooks) ───────────────────────

    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, content)
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
        std::fs::copy(src, dst)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::remove_dir_all(path)
    }

    // ── Workspace Member Detection ─────────────────────────

    // ── Workspace Member Detection ─────────────────────────

    fn is_member_path(&self, path: &str) -> bool {
        utility_filesystem_io::is_member_path(path)
    }

    fn is_leaf_member_path(&self, path: &str) -> bool {
        utility_filesystem_io::is_leaf_member_path(path)
    }

    // ── Source Detection ───────────────────────────────────

    fn detect_source_dir(&self, project_root: &Path) -> PathBuf {
        utility_filesystem_io::detect_source_dir(project_root)
    }

    fn collect_source_files(&self, root_dir: &Path, ignored: &[String]) -> Vec<FilePath> {
        utility_filesystem_io::collect_source_files(
            root_dir,
            &DirectoryPath::new(root_dir.to_string_lossy().to_string()).unwrap_or_default(),
            ignored,
        )
    }

    fn scan_directory_recursive(&self, dir: &Path) -> Vec<String> {
        utility_filesystem_io::scan_directory_recursive(dir)
    }

    fn collect_source_files_from_path(&self, dir: &Path, files: &mut Vec<String>) {
        utility_filesystem_io::collect_source_files_from_path(dir, files)
    }

    // ── Path Metadata Helpers ─────────────────────────────

    fn is_source_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_source_file(path)
    }

    fn is_source_ext(&self, ext: &str) -> bool {
        utility_filesystem_io::is_source_ext(ext)
    }

    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_basename(path)
    }

    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_parent(path)
    }

    // ── Canonicalize (String variant) ─────────────────────────

    fn canonicalize_path_str(&self, path_str: &str) -> String {
        utility_filesystem_io::canonicalize_path_str(path_str)
    }

    // ── Path Resolution (external-lint) ───────────────────────

    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: Vec<String>,
        working_dir: &str,
    ) -> Option<Vec<String>> {
        utility_filesystem_io::resolve_js_cmd(executable, args, working_dir)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        utility_filesystem_io::resolve_js_working_dir(path)
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        utility_filesystem_io::resolve_cargo_working_dir(path)
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        utility_filesystem_io::resolve_cargo_lock_working_dir(path)
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        utility_filesystem_io::default_working_dir(path)
    }

    // ── Python Detection (recursive) ──────────────────────────

    fn has_python_files_recursive(&self, path: &FilePath) -> bool {
        utility_filesystem_io::has_python_files_recursive(path)
    }

    // ── File Mutations ────────────────────────────────────────

    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()> {
        utility_filesystem_io::set_permissions(path, mode)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        utility_filesystem_io::remove_file(path)
    }

    // ── Cache ─────────────────────────────────────────────────

    fn read_cached(&self, path: &FilePath) -> ContentString {
        utility_filesystem_io::read_cached(path)
    }

    // ── Workspace Detection ───────────────────────────────────

    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool {
        utility_filesystem_io::check_wired_in_container(workspace_root, identifiers)
    }

    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error> {
        utility_filesystem_io::find_workspace_root_from_path(start)
    }

    // ── Orphan Detection ──────────────────────────────────────

    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf> {
        utility_filesystem_io::resolve_orphan_module_path(root, base_dir, module_path)
    }

    // ── Language Detection ────────────────────────────────────

    fn detect_language_from_path(
        &self,
        path: &str,
    ) -> shared::config_system::taxonomy_config_language_vo::ConfigLanguage {
        utility_filesystem_io::detect_language_from_path(path)
    }

    // ── File Entry Collection ─────────────────────────────────

    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)> {
        utility_filesystem_io::collect_file_entries(files)
    }

    // ── Process Execution (git) ───────────────────────────────

    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool) {
        utility_filesystem_io::run_git_command(args, dir)
    }

    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        utility_filesystem_io::parse_output_lines(output)
    }

    // ── Process Execution (external) ──────────────────────────

    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool) {
        utility_filesystem_io::run_external_command_in(name, args, current_dir)
    }

    // ── TUI I/O ───────────────────────────────────────────────

    fn write_text_to_file(&self, path: &Path, text: &str) -> Result<(), String> {
        utility_filesystem_io::write_text_to_file(path, text)
    }

    fn is_binary_available(&self, bin_name: &str) -> bool {
        utility_filesystem_io::is_binary_available(bin_name)
    }

    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        utility_filesystem_io::read_dir_entries_as_pathbuf(dir)
    }

    // ── Noop (linter compatibility) ───────────────────────────

    fn noop_apply_fix(
        &self,
    ) -> Result<
        shared::common::taxonomy_message_vo::ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
            false,
        ))
    }
}

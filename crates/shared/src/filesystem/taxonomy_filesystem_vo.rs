// PURPOSE: Taxonomy layer — filesystem domain value objects
// Shared across all crates that need file I/O, parsing, or dependency graph types.

pub use crate::common::taxonomy_language_vo::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

// ═══════════════════════════════════════════════════════════════
// Tool & Extension VOs
// ═══════════════════════════════════════════════════════════════

/// Tool or executable name (e.g. "eslint", "cargo").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolName {
    pub value: String,
}

impl ToolName {
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Tool name cannot be empty".to_string());
        }
        Ok(Self { value })
    }
}

/// File extension string (e.g. "rs", "py", "ts").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileExtension {
    pub value: String,
}

impl FileExtension {
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("File extension cannot be empty".to_string());
        }
        Ok(Self { value })
    }
}

// ═══════════════════════════════════════════════════════════════
// FR-001: File Discovery — Language & FileEntry
// ═══════════════════════════════════════════════════════════════

/// Supported programming languages for AST parsing.
/// A discovered source file with metadata and optional parse results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// File extension (without dot).
    pub extension: String,
    /// Detected language.
    pub language: Language,
    /// File size in bytes.
    pub size: u64,
    /// File content (UTF-8). Empty for skipped/unreadable files.
    pub content: String,
    /// Whether AST parsing succeeded.
    pub parse_ok: bool,
    /// Language-specific parse metadata (None if parse_ok = false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_metadata: Option<ParseMetadata>,
}

// ═══════════════════════════════════════════════════════════════
// FR-002: AST Parsing — ParseMetadata & Language-Specific Types
// ═══════════════════════════════════════════════════════════════

/// Language-specific parse metadata extracted from AST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseMetadata {
    Rust(RustMetadata),
    Python(PythonMetadata),
    TypeScript(TypeScriptMetadata),
    JavaScript(JavaScriptMetadata),
    Unknown,
}

/// Rust-specific parse metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RustMetadata {
    /// `use` statements.
    pub use_statements: Vec<RustUseItem>,
    /// `mod` declarations with optional `#[path]` attribute.
    pub mod_declarations: Vec<RustModItem>,
    /// Struct definitions.
    pub struct_definitions: Vec<String>,
    /// Enum definitions.
    pub enum_definitions: Vec<String>,
    /// Trait definitions.
    pub trait_definitions: Vec<String>,
    /// Type alias definitions.
    pub type_definitions: Vec<String>,
    /// `impl` blocks (trait name, implementor type, has generic).
    pub impl_blocks: Vec<RustImplItem>,
    /// Function definitions (name, has_body).
    pub function_definitions: Vec<RustFnItem>,
    /// All identifiers used in the file body (excluding use declarations).
    pub used_identifiers: Vec<String>,
}

/// A Rust `use` statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustUseItem {
    /// Full use path (e.g., `std::collections::HashMap`).
    pub path: String,
    /// Whether the use has `pub` visibility.
    pub is_pub: bool,
    /// Whether this is a glob import (`use foo::*`).
    pub is_glob: bool,
    /// Imported names (for grouped imports like `use foo::{A, B}`).
    pub names: Vec<String>,
}

/// A Rust `mod` declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustModItem {
    /// Module name.
    pub name: String,
    /// Optional `#[path = "..."]` attribute value.
    pub path_attribute: Option<String>,
}

/// A Rust `impl` block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustImplItem {
    /// Trait name (if implementing a trait, e.g., `Display`).
    pub trait_name: Option<String>,
    /// Qualified trait path (e.g., `std::fmt::Display`).
    pub trait_path: Option<String>,
    /// Implementor type name (e.g., `MyStruct`).
    pub implementor_type: String,
    /// Whether the impl has generic parameters (`impl<T>`).
    pub has_generics: bool,
}

/// A Rust function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFnItem {
    /// Function name.
    pub name: String,
    /// Whether the function has a body (not just a signature).
    pub has_body: bool,
}

/// Python-specific parse metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PythonMetadata {
    /// `import X` statements.
    pub import_statements: Vec<String>,
    /// `from X import Y` statements (module path).
    pub import_from_statements: Vec<String>,
    /// Class declarations (name, base classes).
    pub class_declarations: Vec<PythonClassItem>,
    /// Function definitions (name, has_body).
    pub function_definitions: Vec<PythonFnItem>,
    /// All identifiers used in the file body (tree-sitter extracted).
    pub used_identifiers: Vec<String>,
}

/// A Python class declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonClassItem {
    /// Class name.
    pub name: String,
    /// Base class names (from `class Foo(Bar, Baz)`).
    pub bases: Vec<String>,
}

/// A Python function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonFnItem {
    /// Function name.
    pub name: String,
    /// Whether the function has a body.
    pub has_body: bool,
}

/// TypeScript/JavaScript-specific parse metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeScriptMetadata {
    /// Import statements (source path).
    pub import_statements: Vec<String>,
    /// Export statements with `from` (source path).
    pub export_from_statements: Vec<String>,
    /// Class declarations (name, implements interfaces).
    pub class_declarations: Vec<TSClassItem>,
    /// Interface declarations.
    pub interface_declarations: Vec<String>,
    /// Type alias declarations.
    pub type_alias_declarations: Vec<String>,
    /// Function definitions (name, has_body).
    pub function_definitions: Vec<TSFnItem>,
    /// All identifiers used in the file body (tree-sitter extracted).
    pub used_identifiers: Vec<String>,
}

/// A TypeScript/JavaScript class declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSClassItem {
    /// Class name.
    pub name: String,
    /// Implemented interface names (from `implements IFoo, IBar`).
    pub implements: Vec<String>,
}

/// A TypeScript/JavaScript function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSFnItem {
    /// Function name.
    pub name: String,
    /// Whether the function has a body.
    pub has_body: bool,
}

/// JavaScript metadata is identical to TypeScript for our purposes.
pub type JavaScriptMetadata = TypeScriptMetadata;

// ═══════════════════════════════════════════════════════════════
// FR-002: Parse Warning Diagnostic
// ═══════════════════════════════════════════════════════════════

/// Diagnostic warning for files that failed to parse.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseWarning {
    /// File path that failed to parse.
    pub file_path: PathBuf,
    /// Error detail message.
    pub error_detail: String,
}

impl ParseWarning {
    /// Create a PARSE_WARN diagnostic message per FR-002.
    pub fn message(&self) -> String {
        format!("File skipped: parse failure — {}", self.error_detail)
    }
}

// ═══════════════════════════════════════════════════════════════
// FR-003: Import/Dependency Extraction — ImportEntry
// ═══════════════════════════════════════════════════════════════

/// Import type extracted from AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImportType {
    /// Rust: `use foo::bar`
    Use,
    /// Rust: `mod foo`
    Mod,
    /// Python: `import foo`
    Import,
    /// TS/JS/Python: `from foo import bar` / `import foo from 'bar'`
    ImportFrom,
    /// TypeScript/JS: `require('bar')`
    Require,
    /// Re-export (`pub use`, `export ... from`)
    ReExport,
}

/// A single import/dependency relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEntry {
    /// File that contains the import.
    pub source_file: PathBuf,
    /// Raw import path (as written in source).
    pub raw_path: String,
    /// Resolved absolute module path (if resolvable).
    pub resolved_path: Option<PathBuf>,
    /// Type of import.
    pub import_type: ImportType,
    /// Language of the source file.
    pub language: Language,
    /// Is this a dynamic import? (e.g., `import()`, conditional)
    pub is_dynamic: bool,
    /// Was the import resolved to a file in the workspace?
    pub is_resolved: bool,
    /// Imported symbols (for grouped imports: `use foo::{A, B}`).
    pub symbols: Vec<String>,
    /// Is this a re-export? (`pub use`, `export { X } from`)
    pub is_reexport: bool,
    /// Is this a wildcard import? (`use foo::*`, `export * from`)
    pub is_wildcard: bool,
}

// ═══════════════════════════════════════════════════════════════
// FR-004: Dependency Graph — Graph VOs
// ═══════════════════════════════════════════════════════════════

/// Graph node representing a source file in the dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNodeVO {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// Detected language.
    pub language: Language,
    /// Whether the file is external to the workspace.
    pub is_external: bool,
}

/// Graph edge representing an import relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEdgeVO {
    /// Type of import.
    pub import_type: ImportType,
    /// Raw import path as written in source.
    pub raw_path: String,
    /// Whether the import was resolved to a workspace file.
    pub resolved: bool,
    /// Is this a re-export edge?
    pub is_reexport: bool,
    /// Is this a wildcard import edge?
    pub is_wildcard: bool,
}

/// A symbol definition entry (symbol name -> defining file).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionEntry {
    /// Symbol name (trait/class/struct/interface/type alias).
    pub name: String,
    /// File path where the symbol is defined.
    pub file_path: PathBuf,
    /// Language of the defining file.
    pub language: Language,
}

/// A trait/interface implementation entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplEntry {
    /// Trait/interface name being implemented.
    pub trait_name: String,
    /// File containing the implementation.
    pub file_path: PathBuf,
    /// Language of the implementing file.
    pub language: Language,
}

// ═══════════════════════════════════════════════════════════════
// FR-004: GraphData — Composite Graph Structure
// ═══════════════════════════════════════════════════════════════

/// Composite structure containing all graph data.
/// Built by graph_builder, queried by consumer crates.
#[derive(Debug, Default)]
pub struct GraphData {
    /// Reverse links: file -> list of files that import it.
    pub reverse_links: std::collections::HashMap<PathBuf, Vec<PathBuf>>,
    /// Symbol -> defining file map.
    pub definitions: std::collections::HashMap<String, Vec<PathBuf>>,
    /// Trait/interface -> implementor files map.
    pub implementations: std::collections::HashMap<String, Vec<PathBuf>>,
}

// ═══════════════════════════════════════════════════════════════
// Cache / Memory Budget / Config VOs
// ═══════════════════════════════════════════════════════════════

/// Memory budget for file cache and AST cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudgetVO {
    /// Maximum total bytes for file content cache (default 512 MiB).
    pub max_file_cache_bytes: u64,
    /// Maximum per-file size in bytes (default 2 MiB).
    pub max_file_size_bytes: u64,
}

impl Default for MemoryBudgetVO {
    fn default() -> Self {
        Self {
            max_file_cache_bytes: 512 * 1024 * 1024,
            max_file_size_bytes: MAX_LINT_FILE_BYTES,
        }
    }
}

/// Cache statistics after population.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheStatsVO {
    /// Number of files successfully cached.
    pub cached_count: usize,
    /// Number of files that failed to read.
    pub failed_count: usize,
    /// Total bytes cached.
    pub total_bytes: u64,
}

/// Dependency graph statistics.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphStatsVO {
    /// Total nodes (files) in the graph.
    pub node_count: usize,
    /// Total edges (imports) in the graph.
    pub edge_count: usize,
    /// Number of unresolved imports.
    pub unresolved_count: usize,
    /// Number of cycles detected.
    pub cycle_count: usize,
}

/// Configuration for a filesystem scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfigVO {
    /// Root directory to scan.
    pub root: PathBuf,
    /// Paths to ignore (gitignore patterns).
    pub ignored_paths: Vec<String>,
    /// Memory budget.
    pub budget: MemoryBudgetVO,
}

impl Default for ScanConfigVO {
    fn default() -> Self {
        Self {
            root: PathBuf::new(),
            ignored_paths: Vec::new(),
            budget: MemoryBudgetVO::default(),
        }
    }
}

/// Scan pipeline stage identifiers for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScanStage {
    Walk,
    Cache,
    Parse,
    Extract,
    Graph,
}

/// Timing breakdown for scan stages.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScanTiming {
    pub walk_ms: u64,
    pub cache_ms: u64,
    pub parse_ms: u64,
    pub extract_ms: u64,
    pub graph_ms: u64,
    pub total_ms: u64,
}

// ═══════════════════════════════════════════════════════════════
// Graph Analysis VOs — used by orphan-rules and other consumers
// ═══════════════════════════════════════════════════════════════

/// Analysis context produced by filesystem graph construction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphAnalysisContext {
    pub import_graph: ImportGraph,
    pub inbound_links: InboundLinkMap,
    pub inheritance_map: InheritanceMap,
    /// All workspace files used to build the graph.
    pub all_workspace_files: Vec<String>,
}

impl GraphAnalysisContext {
    pub fn new(
        import_graph: ImportGraph,
        inbound_links: InboundLinkMap,
        inheritance_map: InheritanceMap,
        all_workspace_files: Vec<String>,
    ) -> Self {
        Self {
            import_graph,
            inbound_links,
            inheritance_map,
            all_workspace_files,
        }
    }
}

/// Forward dependency graph: file → files it imports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportGraph {
    pub mapping: HashMap<String, Vec<String>>,
}

impl ImportGraph {
    pub fn new(value: HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

/// Reverse dependency map: file → files that import it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InboundLinkMap {
    pub mapping: HashMap<String, Vec<String>>,
}

impl InboundLinkMap {
    pub fn new(value: HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }

    /// Retrieve importers for a file path, falling back to canonical or suffix matching.
    pub fn get_importers(&self, path: &str) -> Option<&Vec<String>> {
        let mut result = self.exact_or_prefixed(path);

        if result.is_none() {
            result = self.marker_relative(path);
        }

        // `middle_dot` always competes against the current best and keeps the
        // longer match (original semantics — not a priority-ordered short-circuit).
        result = self.middle_dot_variant(result, path);

        if result.is_none() {
            result = self.clean_path(path);
        }
        if result.is_none() {
            result = self.normalized_equal(path);
        }
        if result.is_none() {
            result = self.boundary_suffix(path);
        }
        result
    }

    /// Priority 1: exact key match, then `./`-prefixed key with the longest result.
    fn exact_or_prefixed(&self, path: &str) -> Option<&Vec<String>> {
        let mut result = self.mapping.get(path);
        let with_prefix = format!("./{}", path);
        if let Some(v) = self.mapping.get(&with_prefix) {
            if let Some(existing) = result {
                if v.len() > existing.len() {
                    result = Some(v);
                }
            } else {
                result = Some(v);
            }
        }
        result
    }

    /// Priority 2: strip the first `/crates/`, `/packages/`, or `/modules/` marker
    /// and match the remainder as a relative key.
    fn marker_relative(&self, path: &str) -> Option<&Vec<String>> {
        for marker in &["/crates/", "/packages/", "/modules/"] {
            if let Some(pos) = path.find(marker) {
                let rel = &path[pos + 1..];
                if let Some(v) = self.mapping.get(rel) {
                    return Some(v);
                }
            }
        }
        None
    }

    /// Priority 3: insert `/.` after the first `/crates/` marker
    /// (e.g. `a/crates/b.rs` → `a/./crates/b.rs`), keeping the longest match
    /// between `current` and the middle-dot variant.
    fn middle_dot_variant<'a>(
        &'a self,
        current: Option<&'a Vec<String>>,
        path: &str,
    ) -> Option<&'a Vec<String>> {
        let Some(pos) = path.find("/crates/") else {
            return current;
        };
        let with_middle_dot = format!("{}/.{}", &path[..pos], &path[pos..]);
        match self.mapping.get(&with_middle_dot) {
            Some(v) => match current {
                Some(existing) if v.len() > existing.len() => Some(v),
                Some(existing) => Some(existing),
                None => Some(v),
            },
            None => current,
        }
    }

    /// Priority 4: strip a leading `./` and match the cleaned key directly.
    fn clean_path(&self, path: &str) -> Option<&Vec<String>> {
        let clean = path.strip_prefix("./").unwrap_or(path);
        self.mapping.get(clean)
    }

    /// Priority 5: exact match on the `./`-stripped key (original step 6).
    fn normalized_equal(&self, path: &str) -> Option<&Vec<String>> {
        let clean = path.strip_prefix("./").unwrap_or(path);
        self.mapping.iter().find_map(|(k, v)| {
            let k_clean = k.strip_prefix("./").unwrap_or(k);
            (k_clean == clean).then_some(v)
        })
    }

    /// Priority 6: boundary-aligned suffix matching in both directions.
    /// Requires the boundary to sit on a path separator so that e.g.
    /// `foo_vo.rs` does not match `bar_vo.rs`.
    /// Picks the longest (most specific) matching key; equal-length keys are
    /// broken lexicographically so results are deterministic regardless of
    /// HashMap iteration order. Keys are normalized the same way the matcher
    /// normalizes them, so `/src/a.rs` and `src/a.rs` score identically.
    fn boundary_suffix(&self, path: &str) -> Option<&Vec<String>> {
        let clean = normalize_path(path.strip_prefix("./").unwrap_or(path));
        let mut best: Option<(&Vec<String>, usize, String)> = None;
        for (k, v) in &self.mapping {
            let k_norm = normalize_path(k.strip_prefix("./").unwrap_or(k));
            if k_norm.is_empty() || clean.is_empty() {
                continue;
            }
            let matched_len = if boundary_ends_with(k_norm, clean) {
                Some(k_norm.len())
            } else if boundary_ends_with(clean, k_norm) {
                // Reverse direction: the matching key's length is what makes a
                // candidate specific, so score by k_norm.len() here too.
                Some(k_norm.len())
            } else {
                None
            };
            let Some(len) = matched_len else { continue };
            let better = match best {
                None => true,
                Some((_, best_len, ref best_key)) => {
                    len > best_len || (len == best_len && k_norm < best_key.as_str())
                }
            };
            if better {
                best = Some((v, len, k_norm.to_string()));
            }
        }
        best.map(|(v, _, _)| v)
    }
}

/// Strip leading `./` and `/` so equivalent path forms compare identically.
fn normalize_path(p: &str) -> &str {
    p.trim_start_matches("./").trim_start_matches('/')
}

/// True when `full` ends with `suffix` and the boundary before the suffix is
/// either the start of the string or a path separator (`/` or `\`).
/// Leading `./` and `/` are stripped from both sides first so a suffix like
/// `/b_vo.rs` is treated the same as `b_vo.rs` (the separator check then
/// applies to the byte just before the matched suffix).
fn boundary_ends_with(full: &str, suffix: &str) -> bool {
    let full = full.trim_start_matches("./").trim_start_matches('/');
    let suffix = suffix.trim_start_matches("./").trim_start_matches('/');
    if !full.ends_with(suffix) {
        return false;
    }
    let before = full.len() - suffix.len();
    before == 0
        || full
            .as_bytes()
            .get(before - 1)
            .is_some_and(|b| *b == b'/' || *b == b'\\')
}

/// Inheritance relationships: file → inherited trait/interface names.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InheritanceMap {
    pub mapping: HashMap<String, Vec<String>>,
}

impl InheritanceMap {
    pub fn new(value: HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

/// Byte count for file operations (replaces raw `u64` in contract signatures).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ByteCount {
    pub bytes: u64,
}

impl ByteCount {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }
}

/// Unix file mode bits (replaces raw `u32` in contract signatures).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMode {
    pub bits: u32,
}

impl FileMode {
    pub fn new(bits: u32) -> Self {
        Self { bits }
    }
}

/// Git command result (replaces raw `(String, String, bool)` in contract signatures).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitCommandResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl GitCommandResult {
    pub fn new(stdout: String, stderr: String, success: bool) -> Self {
        Self {
            stdout,
            stderr,
            success,
        }
    }
}

/// Parsed command output lines (replaces raw `Vec<String>` in contract signatures).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParsedLines {
    pub lines: Vec<String>,
}

impl ParsedLines {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

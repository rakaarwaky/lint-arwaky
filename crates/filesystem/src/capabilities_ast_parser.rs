// FR-001: AST Parsing + Import Extraction
// Produces: Vec<FileEntry> with parse_metadata + parse_ok flag, parse warnings, import data
// Consumer: role-rules (via parameter), FR-003, FR-004
//
// Capabilities: struct ASTParser — stores parsed Trees in DashMap, parses in parallel
// Metadata extraction delegated to utility_ast_rust/python/typescript
// Import extraction reuses stored Tree to avoid double parsing (P2.2)

use dashmap::DashMap;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use shared::common::taxonomy_language_vo::Language;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, ImportEntry, ParseMetadata, ParseWarning,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ASTParser {
    asts: DashMap<PathBuf, Arc<tree_sitter::Tree>>,
    warnings: OnceLock<Vec<ParseWarning>>,
    imports: RwLock<Vec<ImportEntry>>,
}

impl ASTParser {
    pub fn new() -> Self {
        Self {
            asts: DashMap::new(),
            warnings: OnceLock::new(),
            imports: RwLock::new(Vec::new()),
        }
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IParserProtocol for ASTParser {
    fn parse_warnings(&self) -> &[ParseWarning] {
        self.warnings.get().map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn import_list(&self) -> Vec<ImportEntry> {
        self.imports.read().map(|v| v.clone()).unwrap_or_default()
    }

    fn parse_all(&self, files: &mut [FileEntry]) {
        self.parse_all_inner(files);
    }

    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
        self.imports
            .read()
            .map(|v| {
                v.iter()
                    .filter(|i| i.source_file == path)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry> {
        self.extract_imports(path, content, language)
    }

    fn resolve_barrel_imports(&self, root_dir: &Path) {
        let imports = match self.imports.read() {
            Ok(v) => v.clone(),
            Err(_) => return,
        };
        let count = imports.len();
        let resolved: Vec<ImportEntry> = imports
            .iter()
            .cloned()
            .map(|entry| crate::utility_barrel_resolution::resolve_single_import(entry, root_dir))
            .collect();
        let resolved_count = resolved.iter().filter(|e| e.is_resolved).count();
        eprintln!(
            "[debug resolve_barrel] input={}, resolved={}, root={}",
            count,
            resolved_count,
            root_dir.display()
        );
        if let Ok(mut w) = self.imports.write() {
            *w = resolved;
        }
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl ASTParser {
    /// Parse all files in parallel using rayon.
    /// Each file is enriched with parse_ok flag and parse_metadata.
    /// Warnings and imports are collected and stored for protocol queries.
    fn parse_all_inner(&self, files: &mut [FileEntry]) {
        // Parallel parse + import extraction per file.
        // Returns (warnings, imports) per entry — drained into OnceLock after the parallel section.
        let results: Vec<(Vec<ParseWarning>, Vec<ImportEntry>)> = files
            .par_iter_mut()
            .map(|entry| {
                if entry.content.is_empty() {
                    entry.parse_ok = true;
                    return (Vec::new(), Vec::new());
                }

                let lang = match entry.language {
                    Language::Rust => tree_sitter_rust::LANGUAGE,
                    Language::Python => tree_sitter_python::LANGUAGE,
                    Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
                    // JS shares the TS grammar structure for extraction purposes
                    Language::JavaScript => tree_sitter_javascript::LANGUAGE,
                    Language::Unknown => {
                        entry.parse_ok = false;
                        return (Vec::new(), Vec::new());
                    }
                };

                let mut parser = tree_sitter::Parser::new();
                if parser.set_language(&lang.into()).is_err() {
                    entry.parse_ok = false;
                    return (
                        vec![ParseWarning {
                            file_path: entry.path.clone(),
                            error_detail: format!(
                                "tree-sitter set_language failed for {:?}",
                                entry.language
                            ),
                        }],
                        Vec::new(),
                    );
                }

                match parser.parse(&entry.content, None) {
                    Some(tree) => {
                        let has_errors = tree.root_node().has_error();

                        // Extract imports using the tree directly — no clone needed.
                        let imports = crate::utility_import_extractor::extract_imports(
                            &entry.path,
                            &entry.content,
                            entry.language,
                            Some(&tree),
                        );

                        if has_errors {
                            // Store tree (even with errors) — downstream consumers can use
                            // the partial AST from error-free subtrees, and we avoid
                            // re-parsing on subsequent extract_imports calls.
                            self.asts.insert(entry.path.clone(), Arc::new(tree));
                            entry.parse_ok = false;
                            (
                                vec![ParseWarning {
                                    file_path: entry.path.clone(),
                                    error_detail: format!(
                                        "tree-sitter parse produced errors for {:?}",
                                        entry.language
                                    ),
                                }],
                                imports,
                            )
                        } else {
                            let metadata =
                                extract_metadata_from_tree(&tree, &entry.content, entry.language);
                            entry.parse_metadata = Some(metadata);
                            entry.parse_ok = true;
                            // Store tree after extracting metadata — tree is moved into Arc.
                            self.asts.insert(entry.path.clone(), Arc::new(tree));
                            (Vec::new(), imports)
                        }
                    }
                    None => {
                        entry.parse_ok = false;
                        (
                            vec![ParseWarning {
                                file_path: entry.path.clone(),
                                error_detail: format!(
                                    "tree-sitter parse returned None for {:?}",
                                    entry.language
                                ),
                            }],
                            Vec::new(),
                        )
                    }
                }
            })
            .collect();

        // Drain collected warnings and imports into OnceLock storage.
        let all_warnings: Vec<ParseWarning> = results
            .iter()
            .flat_map(|(w, _)| w.iter().cloned())
            .collect();
        let all_imports: Vec<ImportEntry> = results.into_iter().flat_map(|(_, i)| i).collect();
        let _ = self.warnings.set(all_warnings);
        if let Ok(mut w) = self.imports.write() {
            *w = all_imports;
        }
    }
}

impl Default for ASTParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract language-specific metadata from a parsed AST.
/// Delegates to utility_ast_rust/python/typescript for language-specific logic.
/// Also populates `used_identifiers` for Python and TypeScript via tree-sitter AST walk.
fn extract_metadata_from_tree(
    tree: &tree_sitter::Tree,
    content: &str,
    language: Language,
) -> ParseMetadata {
    match language {
        Language::Rust => ParseMetadata::Rust(crate::utility_ast_rust::extract_rust_metadata(
            tree, content,
        )),
        Language::Python => {
            let mut meta = crate::utility_ast_python::extract_python_metadata(tree, content);
            meta.used_identifiers =
                crate::utility_ast_python::extract_python_identifiers(tree, content);
            ParseMetadata::Python(meta)
        }
        Language::TypeScript => {
            let mut meta = crate::utility_ast_typescript::extract_ts_metadata(tree, content);
            meta.used_identifiers =
                crate::utility_ast_typescript::extract_ts_identifiers(tree, content);
            ParseMetadata::TypeScript(meta)
        }
        Language::JavaScript => {
            let mut meta = crate::utility_ast_typescript::extract_ts_metadata(tree, content);
            meta.used_identifiers =
                crate::utility_ast_typescript::extract_ts_identifiers(tree, content);
            ParseMetadata::JavaScript(meta)
        }
        Language::Unknown => ParseMetadata::Unknown,
    }
}

impl ASTParser {
    /// Extract imports from file content, reusing stored Tree when available.
    /// Uses Arc clone (refcount bump) instead of deep-copying the tree.
    fn extract_imports(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry> {
        let tree = self.asts.get(path).map(|r| Arc::clone(r.value()));
        crate::utility_import_extractor::extract_imports(
            path,
            content,
            language,
            tree.as_ref().map(|t| t.as_ref()),
        )
    }
}

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
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ASTParser {
    asts: Arc<DashMap<PathBuf, tree_sitter::Tree>>,
    warnings: Vec<ParseWarning>,
    imports: Vec<ImportEntry>,
}

impl ASTParser {
    pub fn new() -> Self {
        Self {
            asts: Arc::new(DashMap::new()),
            warnings: Vec::new(),
            imports: Vec::new(),
        }
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IParserProtocol for ASTParser {
    fn parse_warnings(&self) -> &[ParseWarning] {
        &self.warnings
    }

    fn import_list(&self) -> &[ImportEntry] {
        &self.imports
    }

    fn parse_all(&self, files: &mut [FileEntry]) {
        self.parse_all_inner(files);
    }

    fn imports_for(&self, path: &Path) -> Vec<ImportEntry> {
        self.imports
            .iter()
            .filter(|i| i.source_file == path)
            .cloned()
            .collect()
    }

    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry> {
        self.extract_imports(path, content, language)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl ASTParser {
    /// Parse all files in parallel using rayon.
    /// Each file is enriched with parse_ok flag and parse_metadata.
    fn parse_all_inner(&self, files: &mut [FileEntry]) {
        files.par_iter_mut().for_each(|entry| {
            if entry.content.is_empty() {
                entry.parse_ok = true;
                return;
            }

            let lang = match entry.language {
                Language::Rust => tree_sitter_rust::LANGUAGE,
                Language::Python => tree_sitter_python::LANGUAGE,
                Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
                Language::JavaScript => tree_sitter_javascript::LANGUAGE,
                Language::Unknown => return,
            };

            let mut parser = tree_sitter::Parser::new();
            if parser.set_language(&lang.into()).is_err() {
                entry.parse_ok = false;
                return;
            }

            match parser.parse(&entry.content, None) {
                Some(tree) => {
                    if tree.root_node().has_error() {
                        entry.parse_ok = false;
                    } else {
                        let metadata = extract_metadata(&tree, &entry.content, entry.language);
                        entry.parse_metadata = Some(metadata);
                        entry.parse_ok = true;
                        self.asts.insert(entry.path.clone(), tree);
                    }
                }
                None => {
                    entry.parse_ok = false;
                }
            }
        });
    }
}

impl Default for ASTParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract language-specific metadata from a parsed AST.
/// Delegates to utility_ast_rust/python/typescript for language-specific logic.
fn extract_metadata(tree: &tree_sitter::Tree, content: &str, language: Language) -> ParseMetadata {
    match language {
        Language::Rust => {
            ParseMetadata::Rust(crate::utility_ast_rust::extract_rust_metadata(tree, content))
        }
        Language::Python => {
            ParseMetadata::Python(crate::utility_ast_python::extract_python_metadata(tree, content))
        }
        Language::TypeScript => ParseMetadata::TypeScript(
            crate::utility_ast_typescript::extract_ts_metadata(tree, content),
        ),
        Language::JavaScript => ParseMetadata::JavaScript(
            crate::utility_ast_typescript::extract_ts_metadata(tree, content),
        ),
        Language::Unknown => ParseMetadata::Unknown,
    }
}

impl ASTParser {
    /// Extract imports from file content, reusing stored Tree when available.
    fn extract_imports(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry> {
        let tree = self.asts.get(path).map(|r| r.clone());
        crate::utility_import_extractor::extract_imports(path, content, language, tree.as_ref())
    }
}

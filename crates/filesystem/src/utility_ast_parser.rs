// PURPOSE: Utility layer — AST parser using tree-sitter
// Parse once, query many. Parallel parsing via rayon.

use shared::filesystem::IASTParserProtocol;
use shared::filesystem::taxonomy_filesystem_vo::*;
use dashmap::DashMap;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ASTParser {
    asts: Arc<DashMap<PathBuf, String>>,
}

impl ASTParser {
    pub fn new() -> Self {
        Self { asts: Arc::new(DashMap::new()) }
    }

    pub fn parse_all(&self, files: &[FileEntry], cache: &(dyn Fn(&PathBuf) -> Option<String> + Send + Sync)) {
        let asts = self.asts.clone();
        files.par_iter().for_each(|entry| {
            let content = match cache(&entry.path) {
                Some(c) => c,
                None => return,
            };
            let lang = match entry.language {
                Language::Rust => tree_sitter_rust::LANGUAGE,
                Language::Python => tree_sitter_python::LANGUAGE,
                Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
                Language::JavaScript => tree_sitter_javascript::LANGUAGE,
            };
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&lang.into()).ok();
            if let Some(tree) = parser.parse(&content, None) {
                asts.insert(entry.path.clone(), tree.root_node().to_sexp());
            }
        });
    }
}

impl Default for ASTParser {
    fn default() -> Self { Self::new() }
}

impl IASTParserProtocol for ASTParser {
    fn parse(&self, path: &PathBuf, content: &str, language: Language) -> Option<()> {
        let lang = match language {
            Language::Rust => tree_sitter_rust::LANGUAGE,
            Language::Python => tree_sitter_python::LANGUAGE,
            Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            Language::JavaScript => tree_sitter_javascript::LANGUAGE,
        };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang.into()).ok()?;
        let tree = parser.parse(content, None)?;
        self.asts.insert(path.clone(), tree.root_node().to_sexp());
        Some(())
    }

    fn has_ast(&self, path: &PathBuf) -> bool {
        self.asts.contains_key(path)
    }
}

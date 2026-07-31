// PURPOSE: Utility layer — AST parser using tree-sitter
// Parse once, query many. Parallel parsing via rayon.

use crate::contract_filesystem_protocol::IASTParserProtocol;
use crate::taxonomy_filesystem_vo::*;
use camino::Utf8PathBuf;
use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::Arc;
use tree_sitter::Language as TSLanguage;

/// Thread-safe AST cache.
pub struct ASTParser {
    /// Maps file path → tree-sitter tree (stored as raw bytes for Send+Sync).
    asts: Arc<DashMap<Utf8PathBuf, Vec<u8>>>,
    /// Language grammars.
    rust_lang: TSLanguage,
    python_lang: TSLanguage,
    ts_lang: TSLanguage,
    js_lang: TSLanguage,
}

impl ASTParser {
    pub fn new() -> Self {
        Self {
            asts: Arc::new(DashMap::new()),
            rust_lang: tree_sitter_rust::LANGUAGE.into(),
            python_lang: tree_sitter_python::LANGUAGE.into(),
            ts_lang: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            js_lang: tree_sitter_javascript::LANGUAGE.into(),
        }
    }

    fn grammar_for(&self, lang: Language) -> &TSLanguage {
        match lang {
            Language::Rust => &self.rust_lang,
            Language::Python => &self.python_lang,
            Language::TypeScript => &self.ts_lang,
            Language::JavaScript => &self.js_lang,
        }
    }

    /// Parse all files in parallel.
    pub fn parse_all(&self, files: &[FileEntry], contents: &dyn Fn(&Utf8PathBuf) -> Option<String>) {
        let parser = self.asts.clone();
        let grammars = (
            self.rust_lang.clone(),
            self.python_lang.clone(),
            self.ts_lang.clone(),
            self.js_lang.clone(),
        );

        files.par_iter().for_each(|entry| {
            let content = match contents(&entry.path) {
                Some(c) => c,
                None => return,
            };

            let grammar = match entry.language {
                Language::Rust => &grammars.0,
                Language::Python => &grammars.1,
                Language::TypeScript => &grammars.2,
                Language::JavaScript => &grammars.3,
            };

            let mut ts_parser = tree_sitter::Parser::new();
            ts_parser.set_language(grammar).ok();

            if let Some(tree) = ts_parser.parse(&content, None) {
                // Serialize tree to bytes for Send+Sync storage
                let root = tree.root_node();
                let sexp = root.to_sexp();
                parser.insert(entry.path.clone(), sexp.into_bytes());
            }
        });
    }
}

impl Default for ASTParser {
    fn default() -> Self {
        Self::new()
    }
}

impl IASTParserProtocol for ASTParser {
    fn parse(&self, path: &Utf8PathBuf, content: &str, language: Language) -> Option<()> {
        let grammar = self.grammar_for(language);
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(grammar).ok()?;

        let tree = parser.parse(content, None)?;
        let sexp = tree.root_node().to_sexp();
        self.asts.insert(path.clone(), sexp.into_bytes());
        Some(())
    }

    fn has_ast(&self, path: &Utf8PathBuf) -> bool {
        self.asts.contains_key(path)
    }
}

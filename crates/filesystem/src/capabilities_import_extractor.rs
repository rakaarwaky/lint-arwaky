// PURPOSE: Capabilities layer — import/dependency extraction (FR-003)
// Extracts import/use/from/require statements from ASTs.
// Supports Rust, Python, TypeScript, JavaScript import patterns.
// Skips conditional imports (`#[cfg(...)]`).

use shared::filesystem::contract_filesystem_protocol::IImportExtractorProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use std::path::Path;
use tree_sitter::{Node, Parser};

use crate::utility_import_extractor::{
    child_by_field, extract_js_string_source, extract_scoped_path, text_of,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ImportExtractor;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IImportExtractorProtocol for ImportExtractor {
    fn extract(
        &self,
        path: &std::path::Path,
        content: &str,
        language: Language,
    ) -> Vec<ImportEntry> {
        if content.is_empty() {
            return Vec::new();
        }

        let mut parser = Parser::new();
        let grammar = match language {
            Language::Rust => tree_sitter_rust::LANGUAGE,
            Language::Python => tree_sitter_python::LANGUAGE,
            Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            Language::JavaScript => tree_sitter_javascript::LANGUAGE,
            Language::Unknown => return Vec::new(), // cannot extract imports from unknown language
        };
        parser.set_language(&grammar.into()).ok();
        let tree = match parser.parse(content, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut imports = Vec::new();
        self.extract_from_node(tree.root_node(), content, path, language, &mut imports);
        imports
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for ImportExtractor {
    fn default() -> Self {
        Self
    }
}

impl ImportExtractor {
    /// Recursively extract imports from AST nodes.
    fn extract_from_node(
        &self,
        node: Node,
        content: &str,
        source_file: &Path,
        language: Language,
        imports: &mut Vec<ImportEntry>,
    ) {
        let kind = node.kind();

        // Skip conditional imports: `#[cfg(...)]`
        if language == Language::Rust && kind == "attribute" {
            let text = text_of(node, content);
            if text.starts_with("#[cfg(") {
                return;
            }
        }

        match language {
            Language::Rust => self.extract_rust_imports(node, content, source_file, imports),
            Language::Python => self.extract_python_imports(node, content, source_file, imports),
            Language::TypeScript | Language::JavaScript => {
                self.extract_js_imports(node, content, source_file, language, imports)
            }
            Language::Unknown => return, // cannot extract imports from unknown language
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if language == Language::Rust && child.kind() == "attribute" {
                let text = text_of(child, content);
                if text.starts_with("#[cfg(") {
                    continue;
                }
            }
            self.extract_from_node(child, content, source_file, language, imports);
        }
    }

    /// Extract Rust use_declaration and mod_item imports.
    fn extract_rust_imports(
        &self,
        node: Node,
        content: &str,
        source_file: &Path,
        imports: &mut Vec<ImportEntry>,
    ) {
        let kind = node.kind();

        if kind == "use_declaration" {
            if self.has_cfg_attribute(node) {
                return;
            }

            let is_pub = {
                let mut c = node.walk();
                node.named_children(&mut c)
                    .any(|ch| ch.kind() == "visibility_modifier")
            };

            let text = text_of(node, content);
            let is_glob = text.contains("::*");

            if let Some(names) = self.extract_grouped_use_names(node, content) {
                for name in names {
                    imports.push(ImportEntry {
                        source_file: source_file.to_path_buf(),
                        raw_path: name,
                        resolved_path: None,
                        import_type: if is_pub {
                            ImportType::ReExport
                        } else {
                            ImportType::Use
                        },
                        language: Language::Rust,
                        is_dynamic: false,
                        is_resolved: false,
                        symbols: Vec::new(),
                        is_reexport: is_pub,
                        is_wildcard: false,
                    });
                }
            } else if let Some(path_str) = self.extract_use_path(node, content) {
                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: path_str,
                    resolved_path: None,
                    import_type: if is_pub {
                        ImportType::ReExport
                    } else {
                        ImportType::Use
                    },
                    language: Language::Rust,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: is_pub,
                    is_wildcard: is_glob,
                });
            }
        } else if kind == "mod_item" {
            if let Some(name) = child_by_field(node, content, "name") {
                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: name,
                    resolved_path: None,
                    import_type: ImportType::Mod,
                    language: Language::Rust,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: false,
                    is_wildcard: false,
                });
            }
        }
    }

    /// Check if a use_declaration has a `#[cfg(...)]` parent attribute.
    fn has_cfg_attribute(&self, node: Node) -> bool {
        let mut parent = node.parent();
        while let Some(p) = parent {
            if p.kind() == "attribute" || p.kind() == "inner_attribute_item" {
                return false;
            }
            if p.kind() == "source_file" {
                break;
            }
            parent = p.parent();
        }
        false
    }

    /// Extract grouped use names: `use foo::{A, B, C}`.
    fn extract_grouped_use_names(&self, node: Node, content: &str) -> Option<Vec<String>> {
        let text = text_of(node, content);
        let brace_start = text.find('{')?;
        let brace_end = text.find('}')?;
        let inner = &text[brace_start + 1..brace_end];
        let names: Vec<String> = inner
            .split(',')
            .filter_map(|part| {
                let name = part.split_whitespace().next()?;
                if name.is_empty() || name == "*" {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .collect();
        if names.is_empty() { None } else { Some(names) }
    }

    /// Extract the path from a use_declaration node.
    fn extract_use_path(&self, node: Node, content: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            match child.kind() {
                "scoped_identifier" | "use_as_clause" => {
                    return extract_scoped_path(child, content);
                }
                "identifier" | "crate" | "super" | "self" => {
                    return Some(text_of(child, content));
                }
                _ => {}
            }
        }
        None
    }

    /// Extract Python import_statement and import_from_statement.
    fn extract_python_imports(
        &self,
        node: Node,
        content: &str,
        source_file: &Path,
        imports: &mut Vec<ImportEntry>,
    ) {
        let kind = node.kind();

        if kind == "import_statement" {
            if let Some(module) = self.extract_python_module(node, content) {
                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: module,
                    resolved_path: None,
                    import_type: ImportType::Import,
                    language: Language::Python,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: false,
                    is_wildcard: false,
                });
            }
        } else if kind == "import_from_statement" {
            if let Some(module) = child_by_field(node, content, "module_name") {
                let text = text_of(node, content);
                let is_wildcard = text.contains("*");
                let is_relative = module.starts_with('.') || module.starts_with("..");

                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: if is_relative { module.clone() } else { module },
                    resolved_path: None,
                    import_type: ImportType::ImportFrom,
                    language: Language::Python,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: self.extract_python_from_names(node, content),
                    is_reexport: false,
                    is_wildcard,
                });
            }
        }
    }

    /// Extract Python module path from import_statement.
    fn extract_python_module(&self, node: Node, content: &str) -> Option<String> {
        let text = text_of(node, content);
        text.strip_prefix("import ")?
            .split_whitespace()
            .next()
            .map(|s| s.to_string())
    }

    /// Extract names from `from X import Y, Z`.
    fn extract_python_from_names(&self, node: Node, content: &str) -> Vec<String> {
        let mut names = Vec::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "dotted_name" || child.kind() == "identifier" {
                let name = text_of(child, content);
                if name != "*" && !name.is_empty() {
                    names.push(name);
                }
            }
        }
        names
    }

    /// Extract TypeScript/JavaScript imports.
    fn extract_js_imports(
        &self,
        node: Node,
        content: &str,
        source_file: &Path,
        language: Language,
        imports: &mut Vec<ImportEntry>,
    ) {
        let kind = node.kind();

        if kind == "import_statement" {
            if let Some(source) = extract_js_string_source(node, content) {
                let text = text_of(node, content);
                let is_type = text.starts_with("import type");

                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: source,
                    resolved_path: None,
                    import_type: if is_type {
                        ImportType::ReExport
                    } else {
                        ImportType::ImportFrom
                    },
                    language,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: false,
                    is_wildcard: text.contains("* as"),
                });
            }
        } else if kind == "export_statement" {
            if let Some(source) = extract_js_string_source(node, content) {
                let text = text_of(node, content);
                let is_wildcard = text.contains("export * from");

                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: source,
                    resolved_path: None,
                    import_type: ImportType::ReExport,
                    language,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: true,
                    is_wildcard,
                });
            }
        } else if kind == "call_expression" {
            if let Some(source) = self.extract_require_source(node, content) {
                imports.push(ImportEntry {
                    source_file: source_file.to_path_buf(),
                    raw_path: source,
                    resolved_path: None,
                    import_type: ImportType::Require,
                    language,
                    is_dynamic: false,
                    is_resolved: false,
                    symbols: Vec::new(),
                    is_reexport: false,
                    is_wildcard: false,
                });
            }
        } else if kind == "lexical_declaration" || kind == "variable_declaration" {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    let mut c2 = child.walk();
                    for inner in child.named_children(&mut c2) {
                        if inner.kind() == "call_expression" {
                            if let Some(source) = self.extract_require_source(inner, content) {
                                imports.push(ImportEntry {
                                    source_file: source_file.to_path_buf(),
                                    raw_path: source,
                                    resolved_path: None,
                                    import_type: ImportType::Require,
                                    language,
                                    is_dynamic: false,
                                    is_resolved: false,
                                    symbols: Vec::new(),
                                    is_reexport: false,
                                    is_wildcard: false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    /// Extract source from `require('bar')`.
    fn extract_require_source(&self, node: Node, content: &str) -> Option<String> {
        let text = text_of(node, content);
        if !text.contains("require") {
            return None;
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "arguments" {
                let mut c2 = child.walk();
                for arg in child.named_children(&mut c2) {
                    match arg.kind() {
                        "string" | "template_string" => {
                            let text = text_of(arg, content);
                            let stripped = text
                                .trim_start_matches('\'')
                                .trim_start_matches('\"')
                                .trim_end_matches('\'')
                                .trim_end_matches('\"');
                            if !stripped.is_empty() {
                                return Some(stripped.to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        extract_js_string_source(node, content)
    }
}

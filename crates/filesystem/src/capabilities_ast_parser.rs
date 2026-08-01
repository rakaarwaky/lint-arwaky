// PURPOSE: Capabilities layer — AST parsing via tree-sitter (FR-002)
// Parse once, query many. Parallel parsing via rayon.
// Enriches each FileEntry with parse_metadata and parse_ok flag.

use dashmap::DashMap;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use shared::filesystem::contract_filesystem_protocol::IASTParserProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, Language, ParseMetadata, PythonClassItem, PythonFnItem, PythonMetadata, RustFnItem,
    RustImplItem, RustMetadata, RustModItem, RustUseItem, TSClassItem, TSFnItem,
    TypeScriptMetadata,
};
use std::path::PathBuf;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ASTParser {
    asts: Arc<DashMap<PathBuf, String>>,
}

impl ASTParser {
    pub fn new() -> Self {
        Self {
            asts: Arc::new(DashMap::new()),
        }
    }

    /// Parse all files in parallel using rayon.
    /// Each file is enriched with parse_ok flag and parse_metadata.
    pub fn parse_all(&self, files: &mut [FileEntry]) {
        files.par_iter_mut().for_each(|entry| {
            if entry.content.is_empty() {
                entry.parse_ok = true; // Empty file: parse_ok = true, empty metadata
                return;
            }

            let lang = match entry.language {
                Language::Rust => tree_sitter_rust::LANGUAGE,
                Language::Python => tree_sitter_python::LANGUAGE,
                Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
                Language::JavaScript => tree_sitter_javascript::LANGUAGE,
            };

            let mut parser = tree_sitter::Parser::new();
            if parser.set_language(&lang.into()).is_err() {
                entry.parse_ok = false;
                return;
            }

            match parser.parse(&entry.content, None) {
                Some(tree) => {
                    if tree.root_node().has_error() {
                        // tree-sitter produced partial tree with errors
                        entry.parse_ok = false;
                    } else {
                        let metadata = extract_metadata(&tree, &entry.content, entry.language);
                        entry.parse_metadata = Some(metadata);
                        entry.parse_ok = true;
                        // Store AST root for import extraction
                        self.asts
                            .insert(entry.path.clone(), tree.root_node().to_sexp());
                    }
                }
                None => {
                    entry.parse_ok = false;
                }
            }
        });
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IASTParserProtocol for ASTParser {
    fn parse_all(&self, files: &mut [FileEntry]) {
        self.parse_all(files);
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for ASTParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract language-specific metadata from a parsed AST.
fn extract_metadata(tree: &tree_sitter::Tree, content: &str, language: Language) -> ParseMetadata {
    match language {
        Language::Rust => ParseMetadata::Rust(extract_rust_metadata(tree, content)),
        Language::Python => ParseMetadata::Python(extract_python_metadata(tree, content)),
        Language::TypeScript => ParseMetadata::TypeScript(extract_ts_metadata(tree, content)),
        Language::JavaScript => ParseMetadata::JavaScript(extract_ts_metadata(tree, content)),
    }
}

/// Extract Rust-specific metadata from AST.
fn extract_rust_metadata(tree: &tree_sitter::Tree, content: &str) -> RustMetadata {
    let mut meta = RustMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "use_declaration" => {
                let use_item = extract_rust_use(node, content);
                meta.use_statements.push(use_item);
            }
            "mod_item" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let path_attr = extract_path_attribute(node, content);
                meta.mod_declarations.push(RustModItem {
                    name,
                    path_attribute: path_attr,
                });
            }
            "struct_item" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.struct_definitions.push(name);
                }
            }
            "enum_item" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.enum_definitions.push(name);
                }
            }
            "trait_item" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.trait_definitions.push(name);
                }
            }
            "type_item" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.type_definitions.push(name);
                }
            }
            "impl_item" => {
                let impl_item = extract_rust_impl(node, content);
                meta.impl_blocks.push(impl_item);
            }
            "function_item" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let has_body = node.child_by_field_name("body").is_some();
                meta.function_definitions
                    .push(RustFnItem { name, has_body });
            }
            _ => {}
        }
    }

    meta
}

/// Extract a Rust `use` statement from an AST node.
fn extract_rust_use(node: tree_sitter::Node, content: &str) -> RustUseItem {
    let is_pub = {
        let mut c = node.walk();
        node.named_children(&mut c)
            .any(|ch| ch.kind() == "visibility_modifier")
    };

    let path = extract_use_path(node, content).unwrap_or_default();
    let is_glob = path.ends_with("::*") || content[node.byte_range()].contains("*");
    let names = extract_use_names(node, content);

    RustUseItem {
        path: path.trim_end_matches("::*").to_string(),
        is_pub,
        is_glob,
        names,
    }
}

/// Extract the path from a use_declaration node.
fn extract_use_path(node: tree_sitter::Node, content: &str) -> Option<String> {
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

/// Extract path from a scoped_identifier or use_as_clause.
fn extract_scoped_path(node: tree_sitter::Node, content: &str) -> Option<String> {
    let kind = node.kind();
    if kind == "use_as_clause" {
        // `use foo as bar` — extract the path, not the alias
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "scoped_identifier" || child.kind() == "identifier" {
                return extract_scoped_path(child, content);
            }
        }
        return None;
    }

    // scoped_identifier: may have nested scoped_identifiers or identifiers
    let mut parts = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "identifier" | "crate" | "super" | "self" => {
                parts.push(text_of(child, content));
            }
            "scoped_identifier" => {
                if let Some(inner) = extract_scoped_path(child, content) {
                    parts.push(inner);
                }
            }
            _ => {}
        }
    }
    Some(parts.join("::"))
}

/// Extract individual names from grouped use: `use foo::{A, B, C as D}`.
fn extract_use_names(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut names = Vec::new();
    let _cursor = node.walk();
    // For now, we extract names from the text if grouped
    let text = text_of(node, content);
    if let Some(brace_start) = text.find('{')
        && let Some(brace_end) = text.find('}')
    {
        let inner = &text[brace_start + 1..brace_end];
        for part in inner.split(',') {
            let name = part.split_whitespace().next().unwrap_or("");
            if !name.is_empty() {
                names.push(name.to_string());
            }
        }
    }
    names
}
/// Extract `#[path = "..."]` attribute from a mod_item.
fn extract_path_attribute(node: tree_sitter::Node, content: &str) -> Option<String> {
    let text = text_of(node, content);
    // Look for #[path = "..."] pattern
    if let Some(start) = text.find("path") {
        let rest = &text[start..];
        if let Some(eq_pos) = rest.find('=') {
            let after_eq = rest[eq_pos + 1..].trim();
            if let Some(quote_start) = after_eq.find('"') {
                let after_quote = &after_eq[quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    return Some(after_quote[..quote_end].to_string());
                }
            }
        }
    }
    None
}

/// Extract Rust `impl` block metadata.
fn extract_rust_impl(node: tree_sitter::Node, content: &str) -> RustImplItem {
    let text = text_of(node, content);
    let has_generics = text.contains("<");

    let mut trait_name = None;
    let mut trait_path = None;
    let implementor_type;

    if let Some(for_pos) = text.find(" for ") {
        let before_for = text[..for_pos].trim();
        let after_for = text[for_pos + 5..].trim();

        // Extract trait from before_for (skip `impl<T>` generic part)
        if let Some(impl_end) = before_for.rfind('>') {
            let trait_part = before_for[impl_end + 1..].trim();
            trait_name = Some(trait_part.to_string());
            trait_path = Some(trait_part.to_string());
        } else if let Some(trait_part) = before_for.strip_prefix("impl ") {
            trait_name = Some(trait_part.to_string());
            trait_path = Some(trait_part.to_string());
        }

        // Strip everything from first '{' or whitespace onward
        implementor_type = after_for
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('{')
            .trim()
            .to_string();
    } else {
        // Inherent impl: `impl Type { ... }`
        let impl_part = text.strip_prefix("impl").unwrap_or(&text);
        let impl_part = if let Some(generic_end) = impl_part.find('>') {
            impl_part[generic_end + 1..].trim()
        } else {
            impl_part.trim()
        };
        implementor_type = impl_part
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('{')
            .trim()
            .to_string();
    }

    RustImplItem {
        trait_name,
        trait_path,
        implementor_type,
        has_generics,
    }
}

/// Extract Python-specific metadata from AST.
fn extract_python_metadata(tree: &tree_sitter::Tree, content: &str) -> PythonMetadata {
    let mut meta = PythonMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                let module = extract_python_import_module(node, content);
                if let Some(m) = module {
                    meta.import_statements.push(m);
                }
            }
            "import_from_statement" => {
                if let Some(module) = child_by_field(node, content, "module_name") {
                    meta.import_from_statements.push(module);
                }
            }
            "class_definition" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let bases = extract_python_class_bases(node, content);
                meta.class_declarations
                    .push(PythonClassItem { name, bases });
            }
            "function_definition" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let has_body = node.child_by_field_name("body").is_some();
                meta.function_definitions
                    .push(PythonFnItem { name, has_body });
            }
            _ => {}
        }
    }

    meta
}

/// Extract Python import module path from an import_statement node.
fn extract_python_import_module(node: tree_sitter::Node, content: &str) -> Option<String> {
    let text = text_of(node, content);
    text.strip_prefix("import ")?
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

/// Extract Python class base classes.
fn extract_python_class_bases(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut bases = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "argument_list" {
            let mut c2 = child.walk();
            for arg in child.named_children(&mut c2) {
                if let Some(name) = child_by_field(arg, content, "name") {
                    bases.push(name);
                } else {
                    bases.push(text_of(arg, content));
                }
            }
        }
    }
    bases
}

/// Extract TypeScript/JavaScript-specific metadata from AST.
fn extract_ts_metadata(tree: &tree_sitter::Tree, content: &str) -> TypeScriptMetadata {
    let mut meta = TypeScriptMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                if let Some(source) = extract_js_import_source(node, content) {
                    meta.import_statements.push(source);
                }
            }
            "export_statement" => {
                if let Some(source) = extract_js_export_source(node, content) {
                    meta.export_from_statements.push(source);
                }
            }
            "class_declaration" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let implements = extract_ts_implements(node, content);
                meta.class_declarations
                    .push(TSClassItem { name, implements });
            }
            "interface_declaration" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.interface_declarations.push(name);
                }
            }
            "type_alias_declaration" => {
                if let Some(name) = child_by_field(node, content, "name") {
                    meta.type_alias_declarations.push(name);
                }
            }
            "function_declaration" | "function" => {
                let name = child_by_field(node, content, "name").unwrap_or_default();
                let has_body = node.child_by_field_name("body").is_some();
                meta.function_definitions.push(TSFnItem { name, has_body });
            }
            _ => {}
        }
    }

    meta
}

/// Extract TypeScript `implements` clause from a class declaration.
fn extract_ts_implements(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut implements = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "implements_clause" {
            let mut c2 = child.walk();
            for iface in child.named_children(&mut c2) {
                implements.push(text_of(iface, content));
            }
        }
    }
    implements
}

/// Extract JS import source from import_statement.
fn extract_js_import_source(node: tree_sitter::Node, content: &str) -> Option<String> {
    // Look for string literal child (the source)
    extract_string_from_node(node, content)
}

/// Extract JS export source from export_statement.
fn extract_js_export_source(node: tree_sitter::Node, content: &str) -> Option<String> {
    extract_string_from_node(node, content)
}

/// Extract a string literal from a node's children.
fn extract_string_from_node(node: tree_sitter::Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "string" | "template_string" => {
                let text = text_of(child, content);
                // Strip quotes
                let stripped = text
                    .trim_start_matches('\'')
                    .trim_start_matches('"')
                    .trim_end_matches('\'')
                    .trim_end_matches('"');
                if !stripped.is_empty() {
                    return Some(stripped.to_string());
                }
            }
            _ => {}
        }
    }
    None
}

// ─── Shared Helpers ────────────────────────────────────────

/// Get text content of a node.
fn text_of(node: tree_sitter::Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

/// Get a named child by field name.
fn child_by_field(node: tree_sitter::Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
}

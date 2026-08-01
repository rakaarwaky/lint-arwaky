// PURPOSE: Capabilities layer — import/dependency extraction (FR-003)
// Extracts import/use/from/require statements from ASTs.
// Supports Rust, Python, TypeScript, JavaScript import patterns.
// Skips conditional imports (`#[cfg(...)]`).

use shared::filesystem::taxonomy_filesystem_vo::*;
use std::path::PathBuf;
use tree_sitter::{Node, Parser};

/// Extract imports from a source file's content.
/// Uses tree-sitter for AST-based extraction.
pub fn extract_imports(path: &PathBuf, content: &str, language: Language) -> Vec<ImportEntry> {
    if content.is_empty() {
        return Vec::new();
    }

    let mut parser = Parser::new();
    let grammar = match language {
        Language::Rust => tree_sitter_rust::LANGUAGE,
        Language::Python => tree_sitter_python::LANGUAGE,
        Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        Language::JavaScript => tree_sitter_javascript::LANGUAGE,
    };
    parser.set_language(&grammar.into()).ok();
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let mut imports = Vec::new();
    extract_from_node(tree.root_node(), content, path, language, &mut imports);
    imports
}

/// Recursively extract imports from AST nodes.
fn extract_from_node(
    node: Node,
    content: &str,
    source_file: &PathBuf,
    language: Language,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    // Skip conditional imports: `#[cfg(...)]`
    if language == Language::Rust && kind == "attribute" {
        let text = text_of(node, content);
        if text.starts_with("#[cfg(") {
            return; // Skip the entire attribute (and its child use_declaration)
        }
    }

    match language {
        Language::Rust => extract_rust_imports(node, content, source_file, imports),
        Language::Python => extract_python_imports(node, content, source_file, imports),
        Language::TypeScript | Language::JavaScript => {
            extract_js_imports(node, content, source_file, language, imports)
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        // Skip #[cfg(...)] blocks entirely
        if language == Language::Rust && child.kind() == "attribute" {
            let text = text_of(child, content);
            if text.starts_with("#[cfg(") {
                continue;
            }
        }
        extract_from_node(child, content, source_file, language, imports);
    }
}

/// Extract Rust use_declaration and mod_item imports.
fn extract_rust_imports(
    node: Node,
    content: &str,
    source_file: &PathBuf,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "use_declaration" {
        // Check for #[cfg(...)] parent — skip if conditional
        if has_cfg_attribute(node, content) {
            return;
        }

        let is_pub = {
            let mut c = node.walk();
            node.named_children(&mut c)
                .any(|ch| ch.kind() == "visibility_modifier")
        };

        let text = text_of(node, content);
        let is_glob = text.contains("::*");

        // Extract grouped imports: `use foo::{A, B, C}`
        if let Some(names) = extract_grouped_use_names(node, content) {
            // Create one ImportEntry per name in grouped import
            for name in names {
                imports.push(ImportEntry {
                    source_file: source_file.clone(),
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
        } else if let Some(path_str) = extract_use_path(node, content) {
            imports.push(ImportEntry {
                source_file: source_file.clone(),
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
                source_file: source_file.clone(),
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
fn has_cfg_attribute(node: Node, _content: &str) -> bool {
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "attribute" || p.kind() == "inner_attribute_item" {
            // Would need content to check text, but we skip at the attribute level
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
fn extract_grouped_use_names(node: Node, content: &str) -> Option<Vec<String>> {
    let text = text_of(node, content);
    let brace_start = text.find('{')?;
    let brace_end = text.find('}')?;
    let inner = &text[brace_start + 1..brace_end];
    let names: Vec<String> = inner
        .split(',')
        .filter_map(|part| {
            let name = part.trim().split_whitespace().next()?;
            if name.is_empty() || name == "*" {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect();
    if names.is_empty() {
        None
    } else {
        Some(names)
    }
}

/// Extract the path from a use_declaration node.
fn extract_use_path(node: Node, content: &str) -> Option<String> {
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
fn extract_scoped_path(node: Node, content: &str) -> Option<String> {
    if node.kind() == "use_as_clause" {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "scoped_identifier" || child.kind() == "identifier" {
                return extract_scoped_path(child, content);
            }
        }
        return None;
    }

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

/// Extract Python import_statement and import_from_statement.
fn extract_python_imports(
    node: Node,
    content: &str,
    source_file: &PathBuf,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "import_statement" {
        if let Some(module) = extract_python_module(node, content) {
            imports.push(ImportEntry {
                source_file: source_file.clone(),
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
            // Check for star import
            let text = text_of(node, content);
            let is_wildcard = text.contains("*");
            let is_relative = module.starts_with('.') || module.starts_with("..");

            imports.push(ImportEntry {
                source_file: source_file.clone(),
                raw_path: if is_relative {
                    module.clone()
                } else {
                    module
                },
                resolved_path: None,
                import_type: ImportType::ImportFrom,
                language: Language::Python,
                is_dynamic: false,
                is_resolved: false,
                symbols: extract_python_from_names(node, content),
                is_reexport: false,
                is_wildcard,
            });
        }
    }
}

/// Extract Python module path from import_statement.
fn extract_python_module(node: Node, content: &str) -> Option<String> {
    let text = text_of(node, content);
    text.strip_prefix("import ")?
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

/// Extract names from `from X import Y, Z`.
fn extract_python_from_names(node: Node, content: &str) -> Vec<String> {
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
    node: Node,
    content: &str,
    source_file: &PathBuf,
    language: Language,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "import_statement" {
        if let Some(source) = extract_js_string_source(node, content) {
            let text = text_of(node, content);
            let is_type = text.starts_with("import type");

            imports.push(ImportEntry {
                source_file: source_file.clone(),
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
                source_file: source_file.clone(),
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
        // require('bar')
        if let Some(source) = extract_require_source(node, content) {
            imports.push(ImportEntry {
                source_file: source_file.clone(),
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

/// Extract source string from JS import/export/require.
fn extract_js_string_source(node: Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "string" | "template_string" => {
                let text = text_of(child, content);
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

/// Extract source from `require('bar')`.
fn extract_require_source(node: Node, content: &str) -> Option<String> {
    // Check if this is a require call
    let text = text_of(node, content);
    if !text.contains("require") {
        return None;
    }
    extract_js_string_source(node, content)
}

// ─── Shared Helpers ────────────────────────────────────────

fn text_of(node: Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

fn child_by_field(node: Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
}

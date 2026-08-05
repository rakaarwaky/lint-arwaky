// FR-001: Import Data Extraction
// Produces: Vec<ImportEntry>
// Consumer: import-rules, FR-004
//
// Utility: standalone functions, no struct needed (stateless)
// Language-specific extraction: Rust, Python, TypeScript, JavaScript
// Accepts optional pre-parsed Tree to avoid double parsing (P2.2)

use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use std::path::Path;

fn text_of(node: tree_sitter::Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

fn child_by_field(node: tree_sitter::Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
}

fn extract_use_path(node: tree_sitter::Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "scoped_identifier" | "use_as_clause" => {
                return extract_scoped_path(child, content);
            }
            "use_wildcard" => {
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

fn extract_scoped_path(node: tree_sitter::Node, content: &str) -> Option<String> {
    let kind = node.kind();
    if kind == "use_as_clause" {
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

fn extract_js_string_child(node: tree_sitter::Node, content: &str) -> Option<String> {
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

// ═══════════════════════════════════════════════════════════════
// Public API — FR-001
// ═══════════════════════════════════════════════════════════════

/// Extract all imports from a file.
/// If `pre_parsed` is provided, reuses it instead of re-parsing.
pub fn extract_imports(
    path: &Path,
    content: &str,
    language: Language,
    pre_parsed: Option<&tree_sitter::Tree>,
) -> Vec<ImportEntry> {
    if content.is_empty() {
        return Vec::new();
    }

    let tree = match pre_parsed {
        Some(t) => t.clone(),
        None => {
            let grammar = match language {
                Language::Rust => tree_sitter_rust::LANGUAGE,
                Language::Python => tree_sitter_python::LANGUAGE,
                Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
                Language::JavaScript => tree_sitter_javascript::LANGUAGE,
                Language::Unknown => return Vec::new(),
            };

            let mut parser = tree_sitter::Parser::new();
            if parser.set_language(&grammar.into()).is_err() {
                return Vec::new();
            }

            match parser.parse(content, None) {
                Some(t) => t,
                None => return Vec::new(),
            }
        }
    };

    let mut imports = Vec::new();
    extract_from_node(tree.root_node(), content, path, language, &mut imports);
    imports
}

// ═══════════════════════════════════════════════════════════════
// Utility functions (stateless)
// ═══════════════════════════════════════════════════════════════

fn extract_from_node(
    node: tree_sitter::Node,
    content: &str,
    source_file: &Path,
    language: Language,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    // Skip conditional imports: #[cfg(...)]
    if language == Language::Rust && kind == "attribute" {
        let text = text_of(node, content);
        if text.starts_with("#[cfg(") {
            return;
        }
    }

    match language {
        Language::Rust => extract_rust_imports(node, content, source_file, imports),
        Language::Python => extract_python_imports(node, content, source_file, imports),
        Language::TypeScript | Language::JavaScript => {
            extract_js_imports(node, content, source_file, language, imports)
        }
        Language::Unknown => return,
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
        extract_from_node(child, content, source_file, language, imports);
    }
}

fn extract_rust_imports(
    node: tree_sitter::Node,
    content: &str,
    source_file: &Path,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "use_declaration" {
        let is_pub = {
            let mut c = node.walk();
            node.named_children(&mut c)
                .any(|ch| ch.kind() == "visibility_modifier")
        };
        let text = text_of(node, content);
        let is_glob = text.contains("::*");

        if let Some(names) = extract_grouped_use_names(node, content) {
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
        } else if let Some(path_str) = extract_use_path(node, content) {
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
    } else if kind == "mod_item"
        && let Some(name) = child_by_field(node, content, "name")
    {
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

fn extract_python_imports(
    node: tree_sitter::Node,
    content: &str,
    source_file: &Path,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "import_statement" {
        let text = text_of(node, content);
        if let Some(module) = text
            .strip_prefix("import ")
            .and_then(|s| s.split_whitespace().next())
        {
            imports.push(ImportEntry {
                source_file: source_file.to_path_buf(),
                raw_path: module.to_string(),
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
    } else if kind == "import_from_statement"
        && let Some(module) = child_by_field(node, content, "module_name")
    {
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
            symbols: extract_python_from_names(node, content),
            is_reexport: false,
            is_wildcard,
        });
    }
}

fn extract_python_from_names(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut names = Vec::new();
    let module_name_node = node.child_by_field_name("module_name");
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "dotted_name" || child.kind() == "identifier" {
            // Skip the module_name field — it's the import path, not an imported symbol
            if Some(child.id()) == module_name_node.map(|n| n.id()) {
                continue;
            }
            let name = text_of(child, content);
            if name != "*" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

fn extract_js_imports(
    node: tree_sitter::Node,
    content: &str,
    source_file: &Path,
    language: Language,
    imports: &mut Vec<ImportEntry>,
) {
    let kind = node.kind();

    if kind == "import_statement" {
        if let Some(source) = extract_js_string_child(node, content) {
            let text = text_of(node, content);
            let is_type = text.starts_with("import type");
            let symbols = extract_js_named_imports(&text);
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
                symbols,
                is_reexport: false,
                is_wildcard: text.contains("* as"),
            });
        }
    } else if kind == "export_statement" {
        if let Some(source) = extract_js_string_child(node, content) {
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
        if let Some(source) = extract_require_source(node, content) {
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
                    if inner.kind() == "call_expression"
                        && let Some(source) = extract_require_source(inner, content)
                    {
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

// ═══════════════════════════════════════════════════════════════
// Shared helpers
// ═══════════════════════════════════════════════════════════════

/// Extract named import symbols from JS/TS import statement text.
/// e.g. `import { CalculatorProtocol, ExpressionVO } from "..."` → ["CalculatorProtocol", "ExpressionVO"]
fn extract_js_named_imports(text: &str) -> Vec<String> {
    if let Some(brace_start) = text.find('{') {
        if let Some(brace_end) = text.find('}') {
            let inner = &text[brace_start + 1..brace_end];
            return inner
                .split(',')
                .filter_map(|part| {
                    let name = part.split_whitespace().next()?;
                    if name.is_empty() || name == "*" || name == "type" {
                        None
                    } else {
                        Some(name.to_string())
                    }
                })
                .collect();
        }
    }
    Vec::new()
}

fn extract_grouped_use_names(node: tree_sitter::Node, content: &str) -> Option<Vec<String>> {
    let text = text_of(node, content);
    let brace_start = text.find('{')?;
    let brace_end = text.find('}')?;
    // Module path: everything before '{', trimmed of trailing whitespace/colons.
    let module_path = text[..brace_start]
        .trim_end_matches("::")
        .trim();
    let inner = &text[brace_start + 1..brace_end];
    let names: Vec<String> = inner
        .split(',')
        .filter_map(|part| {
            let name = part.split_whitespace().next()?;
            if name.is_empty() || name == "*" {
                None
            } else if module_path.is_empty() {
                Some(name.to_string())
            } else {
                Some(format!("{}::{}", module_path, name))
            }
        })
        .collect();
    if names.is_empty() { None } else { Some(names) }
}

fn extract_require_source(node: tree_sitter::Node, content: &str) -> Option<String> {
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
        }
    }
    extract_js_string_child(node, content)
}

// ═══════════════════════════════════════════════════════════════
// FR-003: Protocol Implementation
// ═══════════════════════════════════════════════════════════════
// Import extraction is now handled by ASTParser via IParserProtocol.
// This file provides the stateless extract_imports() function only.
// ═══════════════════════════════════════════════════════════════

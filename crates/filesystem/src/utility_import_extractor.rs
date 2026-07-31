// PURPOSE: Utility layer — extract imports from source files using tree-sitter
// Supports Rust, Python, TypeScript, JavaScript import patterns.

use shared::filesystem::{ImportEntry, ImportType, Language};
use std::path::PathBuf;
use tree_sitter::{Node, Parser};

/// Extract imports from a source file's content.
/// Uses tree-sitter for AST-based extraction (faster and more accurate than regex).
pub fn extract_imports(path: &PathBuf, content: &str, language: Language) -> Vec<ImportEntry> {
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

    match language {
        Language::Rust => {
            // use_statement: `use foo::bar;` or `use foo::bar as baz;`
            // mod_declaration: `mod foo;` or `mod foo { ... }`
            if kind == "use_declaration" {
                if let Some(path_str) = extract_use_path(node, content) {
                    let is_pub = node
                        .child_by_field_name("visibility")
                        .map_or(false, |v| text_of(v, content) == "pub");
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: path_str,
                        resolved_path: None,
                        import_type: if is_pub {
                            ImportType::ReExport
                        } else {
                            ImportType::Use
                        },
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            } else if kind == "mod_declaration" {
                if let Some(name) = child_by_field(node, content, "name") {
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: name,
                        resolved_path: None,
                        import_type: ImportType::Mod,
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            }
        }
        Language::Python => {
            // import_statement: `import foo` or `import foo.bar`
            // import_from_statement: `from foo import bar`
            if kind == "import_statement" {
                if let Some(module) = extract_python_import_module(node, content) {
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: module,
                        resolved_path: None,
                        import_type: ImportType::Import,
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            } else if kind == "import_from_statement" {
                if let Some(module) = child_by_field(node, content, "module_name") {
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: module,
                        resolved_path: None,
                        import_type: ImportType::ImportFrom,
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            }
        }
        Language::TypeScript | Language::JavaScript => {
            // import_statement: `import foo from 'bar'` or `import { x } from 'bar'`
            // call_expression with `require`: `require('bar')`
            // export_statement with `from`: `export { x } from 'bar'`
            if kind == "import_statement" {
                if let Some(source) = extract_js_import_source(node, content) {
                    let is_type = text_of(node, content).starts_with("import type");
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: source,
                        resolved_path: None,
                        import_type: if is_type {
                            ImportType::ReExport // type-only import
                        } else {
                            ImportType::ImportFrom
                        },
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            } else if kind == "export_statement" {
                if let Some(source) = extract_js_export_source(node, content) {
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: source,
                        resolved_path: None,
                        import_type: ImportType::ReExport,
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            } else if kind == "call_expression" {
                if let Some(source) = extract_require_source(node, content) {
                    imports.push(ImportEntry {
                        source_file: source_file.clone(),
                        raw_path: source,
                        resolved_path: None,
                        import_type: ImportType::Require,
                        language,
                        is_dynamic: false,
                        is_resolved: false,
                    });
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_from_node(child, content, source_file, language, imports);
    }
}

// ─── Rust helpers ──────────────────────────────────────────

fn extract_use_path(node: Node, content: &str) -> Option<String> {
    let scoped = node.child_by_field_name("scoped_use")?;
    let path = scoped.child_by_field_name("path")?;
    Some(text_of(path, content).to_string())
}

// ─── Python helpers ────────────────────────────────────────

fn extract_python_import_module(node: Node, content: &str) -> Option<String> {
    let name = node.child_by_field_name("name")?;
    Some(text_of(name, content).to_string())
}

// ─── JS/TS helpers ─────────────────────────────────────────

fn extract_js_import_source(node: Node, content: &str) -> Option<String> {
    let source = node.child_by_field_name("source")?;
    extract_string_literal(source, content)
}

fn extract_js_export_source(node: Node, content: &str) -> Option<String> {
    let source = node.child_by_field_name("source")?;
    extract_string_literal(source, content)
}

fn extract_require_source(node: Node, content: &str) -> Option<String> {
    // require('foo') — the function name must be "require"
    let fn_node = node.child_by_field_name("function")?;
    if text_of(fn_node, content) != "require" {
        return None;
    }
    let args = node.child_by_field_name("arguments")?;
    let first_arg = args.named_child(0)?;
    extract_string_literal(first_arg, content)
}

fn extract_string_literal(node: Node, content: &str) -> Option<String> {
    let text = text_of(node, content);
    // Strip quotes
    let inner = text.strip_prefix('\'').or_else(|| text.strip_prefix('"')).or_else(|| text.strip_prefix('`'))?;
    inner.strip_suffix('\'').or_else(|| inner.strip_suffix('"')).or_else(|| inner.strip_suffix('`')).map(|s| s.to_string())
}

// ─── Generic helpers ───────────────────────────────────────

fn text_of<'a>(node: Node<'a>, content: &'a str) -> &'a str {
    &content[node.byte_range()]
}

fn child_by_field(node: Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content).to_string())
}

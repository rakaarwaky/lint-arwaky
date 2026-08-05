// PURPOSE: Stateless TypeScript/JavaScript AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use shared::filesystem::taxonomy_filesystem_vo::{TSClassItem, TSFnItem, TypeScriptMetadata};

// ─── Inlined from utility_tree_sitter_helpers (AES201: utility cannot import utility) ───

fn text_of(node: tree_sitter::Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

fn child_by_field(node: tree_sitter::Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
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

/// Extract TypeScript/JavaScript-specific metadata from a parsed AST.
pub fn extract_ts_metadata(tree: &tree_sitter::Tree, content: &str) -> TypeScriptMetadata {
    let mut meta = TypeScriptMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                if let Some(source) = extract_js_string_child(node, content) {
                    meta.import_statements.push(source);
                }
            }
            "export_statement" => {
                if let Some(source) = extract_js_string_child(node, content) {
                    meta.export_from_statements.push(source);
                }
                // Handle exported class/interface/type declarations
                let mut ec = node.walk();
                for child in node.named_children(&mut ec) {
                    match child.kind() {
                        "class_declaration" => {
                            let name = child_by_field(child, content, "name").unwrap_or_default();
                            let implements = extract_ts_implements(child, content);
                            meta.class_declarations
                                .push(TSClassItem { name, implements });
                        }
                        "interface_declaration" => {
                            if let Some(name) = child_by_field(child, content, "name") {
                                meta.interface_declarations.push(name);
                            }
                        }
                        "type_alias_declaration" => {
                            if let Some(name) = child_by_field(child, content, "name") {
                                meta.type_alias_declarations.push(name);
                            }
                        }
                        _ => {}
                    }
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

fn extract_ts_implements(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut implements = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "implements_clause" => {
                let mut c2 = child.walk();
                for iface in child.named_children(&mut c2) {
                    implements.push(text_of(iface, content));
                }
            }
            "class_heritage" => {
                // class_heritage contains both extends_clause and implements_clause
                let mut c2 = child.walk();
                for heritage_child in child.named_children(&mut c2) {
                    if heritage_child.kind() == "implements_clause" {
                        let mut c3 = heritage_child.walk();
                        for iface in heritage_child.named_children(&mut c3) {
                            implements.push(text_of(iface, content));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    implements
}

/// Extract all identifiers from a TypeScript/JavaScript AST using tree-sitter.
/// Skips import statements, string literals, and comments.
pub fn extract_ts_identifiers(tree: &tree_sitter::Tree, content: &str) -> Vec<String> {
    let mut ids = std::collections::HashSet::new();
    extract_identifiers_recursive(tree.root_node(), content, &mut ids);
    let mut result: Vec<String> = ids.into_iter().collect();
    result.sort();
    result
}

fn extract_identifiers_recursive(
    node: tree_sitter::Node,
    content: &str,
    ids: &mut std::collections::HashSet<String>,
) {
    let kind = node.kind();

    // Skip import statements (don't extract identifiers from import source paths)
    // But recurse into export_statement to find identifiers in exported declarations
    if kind == "import_statement" {
        return;
    }

    // Skip comment nodes
    if kind == "comment" || kind == "line_comment" || kind == "block_comment" {
        return;
    }

    // Skip string/template literals — don't extract identifiers from string content
    if kind == "string" || kind == "template_string" || kind == "regex" {
        return;
    }

    // If this is an identifier, type_identifier, or property access, extract it
    // type_identifier covers TypeScript type annotations like `: ExpressionVO`
    if kind == "identifier" || kind == "type_identifier" {
        let text = text_of(node, content);
        if !text.is_empty()
            && text
                .chars()
                .next()
                .is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
            && text
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        {
            ids.insert(text);
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_identifiers_recursive(child, content, ids);
    }
}

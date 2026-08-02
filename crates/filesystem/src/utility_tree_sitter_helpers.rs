// PURPOSE: Shared tree-sitter node helpers
// Used by: capabilities_ast_parser (FR-002), utility_import_extractor (FR-003)
//
// Utility: stateless functions, no struct

/// Extract text content of a tree-sitter node.
pub fn text_of(node: tree_sitter::Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

/// Extract text of a named child by field name.
pub fn child_by_field(node: tree_sitter::Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
}

/// Extract the first `use` path from a `use` statement node.
pub fn extract_use_path(node: tree_sitter::Node, content: &str) -> Option<String> {
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

/// Recursively resolve a `scoped_identifier` into a `::`-joined path.
pub fn extract_scoped_path(node: tree_sitter::Node, content: &str) -> Option<String> {
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

/// Extract the first string/template_string child from a JS/TS node.
pub fn extract_js_string_child(node: tree_sitter::Node, content: &str) -> Option<String> {
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

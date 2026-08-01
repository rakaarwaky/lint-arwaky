// PURPOSE: Utility layer — tree-sitter AST helpers for import extraction
// Pure functions, no self, domain-agnostic. Used by capabilities_import_extractor.

use tree_sitter::Node;

/// Get text content of a node.
pub fn text_of(node: Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

/// Get a named child by field name.
pub fn child_by_field(node: Node, content: &str, field: &str) -> Option<String> {
    let child = node.child_by_field_name(field)?;
    Some(text_of(child, content))
}

/// Extract path from a scoped_identifier or use_as_clause.
pub fn extract_scoped_path(node: Node, content: &str) -> Option<String> {
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

/// Extract source string from JS import/export/require.
pub fn extract_js_string_source(node: Node, content: &str) -> Option<String> {
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

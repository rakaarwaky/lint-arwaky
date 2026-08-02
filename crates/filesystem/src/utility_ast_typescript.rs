// PURPOSE: Stateless TypeScript/JavaScript AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use shared::filesystem::taxonomy_filesystem_vo::{TSClassItem, TSFnItem, TypeScriptMetadata};

use crate::utility_tree_sitter_helpers::{child_by_field, extract_js_string_child, text_of};

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
        if child.kind() == "implements_clause" {
            let mut c2 = child.walk();
            for iface in child.named_children(&mut c2) {
                implements.push(text_of(iface, content));
            }
        }
    }
    implements
}

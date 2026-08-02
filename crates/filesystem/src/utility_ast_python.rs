// PURPOSE: Stateless Python AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use shared::filesystem::taxonomy_filesystem_vo::{PythonClassItem, PythonFnItem, PythonMetadata};

use crate::utility_tree_sitter_helpers::{child_by_field, text_of};

/// Extract Python-specific metadata from a parsed AST.
pub fn extract_python_metadata(tree: &tree_sitter::Tree, content: &str) -> PythonMetadata {
    let mut meta = PythonMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                let text = text_of(node, content);
                if let Some(module) = text
                    .strip_prefix("import ")
                    .and_then(|s| s.split_whitespace().next())
                {
                    meta.import_statements.push(module.to_string());
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

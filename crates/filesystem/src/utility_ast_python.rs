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

/// Extract all identifiers from a Python AST using tree-sitter.
/// Skips import statements, string literals, and comments.
pub fn extract_python_identifiers(tree: &tree_sitter::Tree, content: &str) -> Vec<String> {
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

    // Skip import statements entirely
    if kind == "import_statement" || kind == "import_from_statement" {
        return;
    }

    // Skip comment nodes
    if kind == "comment" {
        return;
    }

    // Skip string literals — don't extract identifiers from string content
    if kind == "string" || kind == "concatenated_string" || kind == "f_string" {
        return;
    }

    // If this is an identifier node, extract it
    if kind == "identifier" || kind == "attribute" {
        let text = text_of(node, content);
        // Only add valid Python identifiers (start with letter or _, rest alphanumeric)
        if !text.is_empty()
            && text
                .chars()
                .next()
                .is_some_and(|c| c.is_alphabetic() || c == '_')
            && text.chars().all(|c| c.is_alphanumeric() || c == '_')
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

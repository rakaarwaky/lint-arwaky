// PURPOSE: Stateless Rust AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use shared::filesystem::taxonomy_filesystem_vo::{RustFnItem, RustImplItem, RustMetadata, RustModItem, RustUseItem};

use crate::utility_tree_sitter_helpers::{child_by_field, extract_use_path, text_of};

/// Extract Rust-specific metadata from a parsed AST.
pub fn extract_rust_metadata(tree: &tree_sitter::Tree, content: &str) -> RustMetadata {
    let mut meta = RustMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "use_declaration" => {
                meta.use_statements.push(extract_rust_use(node, content));
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
                meta.impl_blocks.push(extract_rust_impl(node, content));
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

fn extract_use_names(node: tree_sitter::Node, content: &str) -> Vec<String> {
    let mut names = Vec::new();
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

fn extract_path_attribute(node: tree_sitter::Node, content: &str) -> Option<String> {
    let text = text_of(node, content);
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

fn extract_rust_impl(node: tree_sitter::Node, content: &str) -> RustImplItem {
    let text = text_of(node, content);
    let has_generics = text.contains('<');
    let mut trait_name = None;
    let mut trait_path = None;
    let implementor_type;

    if let Some(for_pos) = text.find(" for ") {
        let before_for = text[..for_pos].trim();
        let after_for = text[for_pos + 5..].trim();
        if let Some(impl_end) = before_for.rfind('>') {
            let trait_part = before_for[impl_end + 1..].trim();
            trait_name = Some(trait_part.to_string());
            trait_path = Some(trait_part.to_string());
        } else if let Some(trait_part) = before_for.strip_prefix("impl ") {
            trait_name = Some(trait_part.to_string());
            trait_path = Some(trait_part.to_string());
        }
        implementor_type = after_for
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('{')
            .trim()
            .to_string();
    } else {
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

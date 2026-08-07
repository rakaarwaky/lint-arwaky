// PURPOSE: Stateless Rust AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use shared::filesystem::taxonomy_filesystem_vo::{
    RustFnItem, RustImplItem, RustMetadata, RustModItem, RustUseItem,
};

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

/// Extract Rust-specific metadata from a parsed AST.
pub fn extract_rust_metadata(tree: &tree_sitter::Tree, content: &str) -> RustMetadata {
    let mut meta = RustMetadata::default();
    let root = tree.root_node();
    let mut cursor = root.walk();

    // Collect byte ranges of use declarations to exclude them from identifier extraction.
    let mut use_ranges: Vec<std::ops::Range<usize>> = Vec::new();

    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "use_declaration" => {
                use_ranges.push(node.byte_range());
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

    // Extract all identifiers from the file, excluding use declarations.
    meta.used_identifiers = extract_identifiers_excluding_uses(root, content, &use_ranges);
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

/// Extract all identifiers from the AST, excluding those inside use declarations.
fn extract_identifiers_excluding_uses(
    root: tree_sitter::Node,
    content: &str,
    use_ranges: &[std::ops::Range<usize>],
) -> Vec<String> {
    let mut identifiers = std::collections::HashSet::new();

    fn is_inside_use(node: tree_sitter::Node, use_ranges: &[std::ops::Range<usize>]) -> bool {
        let range = node.byte_range();
        use_ranges
            .iter()
            .any(|ur| ur.start <= range.start && range.end <= ur.end)
    }

    fn walk_node(
        node: tree_sitter::Node,
        content: &str,
        use_ranges: &[std::ops::Range<usize>],
        identifiers: &mut std::collections::HashSet<String>,
    ) {
        if is_inside_use(node, use_ranges) {
            return;
        }
        // Collect identifier nodes (field_name, identifier, type_identifier, etc.)
        if matches!(
            node.kind(),
            "identifier" | "field_identifier" | "type_identifier" | "macro_identifier"
        ) {
            if let Ok(text) = node.utf8_text(content.as_bytes()) {
                let name = text.to_string();
                // Skip keywords and single-char identifiers
                if name.len() > 1 && !is_rust_keyword(&name) {
                    identifiers.insert(name);
                }
            }
        }
        // Recurse into children
        let mut child_cursor = node.walk();
        for child in node.named_children(&mut child_cursor) {
            walk_node(child, content, use_ranges, identifiers);
        }
    }

    walk_node(root, content, use_ranges, &mut identifiers);
    identifiers.into_iter().collect()
}

/// Check if a name is a Rust keyword that should not be treated as an identifier.
fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "fn" | "let"
            | "mut"
            | "pub"
            | "use"
            | "mod"
            | "struct"
            | "enum"
            | "trait"
            | "impl"
            | "self"
            | "Self"
            | "super"
            | "crate"
            | "return"
            | "if"
            | "else"
            | "match"
            | "for"
            | "while"
            | "loop"
            | "in"
            | "as"
            | "ref"
            | "move"
            | "async"
            | "await"
            | "where"
            | "type"
            | "const"
            | "static"
            | "true"
            | "false"
            | "Some"
            | "None"
            | "Ok"
            | "Err"
            | "Box"
            | "Vec"
            | "String"
            | "str"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "f32"
            | "f64"
            | "bool"
            | "char"
    )
}

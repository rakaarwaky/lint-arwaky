// PURPOSE: Stateless Rust AST metadata extraction functions
// Used by: capabilities_ast_parser (FR-001)
//
// Utility: pure functions, no struct, no trait impl

use crate::taxonomy_filesystem_vo::{
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
    let implementor;

    // Every answer below comes from the impl *header*, never the body. The body
    // is arbitrary user code: an arrow in `fn f() -> T`, a comparison, a
    // nested impl. Searching the whole block mislabels the implementor — for
    // `impl RuleRow { fn into_rule(self) -> ArchitectureRule { .. } }` a `>`-scan
    // over the full text yields "ArchitectureRule" instead of "RuleRow".
    let header = text.split('{').next().unwrap_or(&text);

    if let Some(for_pos) = header.find(" for ") {
        let before_for = header[..for_pos].trim();
        let after_for = header[for_pos + 5..].trim();
        if let Some(trait_part) = trait_name_before_for(before_for) {
            trait_name = Some(trait_part.to_string());
            trait_path = Some(trait_part.to_string());
        }
        implementor = type_name_after_generic(after_for);
    } else {
        let impl_part = header.strip_prefix("impl").unwrap_or(header);
        implementor = type_name_after_generic(impl_part);
    }

    RustImplItem {
        trait_name,
        trait_path,
        implementor_type: implementor,
        has_generics,
    }
}

/// The type name in an impl header, with the impl's own generic list removed.
///
/// `impl<T> Foo for Bar<T>` names `Bar`. The generic list only belongs to the
/// impl when it opens immediately after `impl` (`impl<'a, T> Foo for Bar`); in
/// `impl Foo<u32> for Bar` the `<` belongs to the trait and must be kept, so
/// this is used only on text that has already had its trait stripped.
fn type_name_after_generic(text: &str) -> String {
    let text = text.trim();
    let text = if text.starts_with('<') {
        match text.find('>') {
            Some(offset) => text[offset + 1..].trim_start(),
            None => text,
        }
    } else {
        text
    };
    first_type_token(text)
}

/// First whitespace-delimited type token, minus generic parameters.
fn first_type_token(text: &str) -> String {
    let mut token = String::new();
    let mut depth = 0i32;
    for ch in text.chars() {
        match ch {
            '<' => {
                depth += 1;
            }
            '>' if depth > 0 => {
                depth -= 1;
            }
            _ if depth > 0 => {}
            _ => token.push(ch),
        }
    }
    token
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(['{', ',', ':', '&'])
        .trim()
        .to_string()
}

/// The trait named in the head of an `impl … Trait for Type` block.
///
/// The head has three shapes, and each needs the trait's own name rather than
/// whatever happens to sit last:
///
/// - `impl Foo for Bar` — the whole head after `impl ` is the trait.
/// - `impl<T> Foo for Bar` / `impl<'a, T> Foo for Bar` — a generic list for the
///   impl itself precedes the trait.
/// - `impl Foo<T> for Bar` — the *trait* is parameterized, so its own brackets
///   trail its name.
///
/// Reading up to the last `>` handles the first two but not the third: for
/// `impl Foo<u32> for Bar` it yields `u32`, losing the trait name entirely and
/// hiding the implementation from every consumer that keys on the trait. So the
/// leading generic list is stripped first — and only when it is attached to the
/// `impl` keyword, since that is the only position where it can be one — and
/// the remainder is trimmed of the trait's own trailing parameter list.
fn trait_name_before_for(before_for: &str) -> Option<&str> {
    let head = before_for.strip_prefix("impl")?.trim_start();
    // The impl's own generic list is attached to the keyword with no space —
    // `impl<T> Foo for Bar`, `impl<'a, T> Foo for Bar`. Any other bracket that
    // opens a list belongs to the trait, which is the whole point of this
    // function: in `impl Foo<u32> for Bar` the `<` is the trait's, so treating it
    // as an impl generic list would strip `Foo` and leave nothing behind.
    let head = if head.starts_with('<') {
        let offset = head.find('>')?;
        head[offset + 1..].trim_start()
    } else {
        head
    };
    // Drop the trait's own type parameters: `Foo<u32, Bar>` -> `Foo`.
    let head = head
        .split('<')
        .next()
        .unwrap_or(head)
        .trim()
        .trim_end_matches(['>', ',', ' ']);
    if head.is_empty() { None } else { Some(head) }
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

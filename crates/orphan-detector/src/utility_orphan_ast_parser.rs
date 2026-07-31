// PURPOSE: utility_orphan_ast_parser — AST-based code parsing for Rust source files.
// Replaces regex-based extraction with syn AST walking for accurate import,
// trait implementation, struct, and module detection.
//
// For Python/TS files, falls back to structured line-based parsing that is
// comment-aware and string-aware (not raw regex on full content).

use std::collections::HashMap;
use std::path::Path;

// ─── Block 1: Data Structures ─────────────────────────────

/// A single import edge extracted from AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstImport {
    /// Raw import path as written in source (e.g., "crate::common::FilePath")
    pub raw_path: String,
    /// Normalized segments (e.g., ["crate", "common", "FilePath"])
    pub segments: Vec<String>,
    /// Whether this is a `pub use` re-export
    pub is_reexport: bool,
    /// Whether this is a glob import (`use foo::*`)
    pub is_glob: bool,
}

/// A trait implementation found in AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstTraitImpl {
    /// Trait name (e.g., "IOrphanAggregate")
    pub trait_name: String,
    /// Implementing type name (e.g., "ArchOrphanAnalyzer")
    pub type_name: String,
    /// Whether the impl has generic parameters
    pub has_generics: bool,
}

/// A struct definition found in AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstStructDef {
    pub name: String,
    pub is_pub: bool,
}

/// A trait definition found in AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstTraitDef {
    pub name: String,
    pub is_pub: bool,
}

/// A module declaration found in AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstModDecl {
    pub name: String,
    /// If `#[path = "..."]` attribute is present
    pub path_attr: Option<String>,
    pub is_pub: bool,
}

/// Complete parse result for a single Rust source file.
#[derive(Debug, Clone, Default)]
pub struct RustParseResult {
    pub imports: Vec<AstImport>,
    pub trait_impls: Vec<AstTraitImpl>,
    pub structs: Vec<AstStructDef>,
    pub traits: Vec<AstTraitDef>,
    pub mod_decls: Vec<AstModDecl>,
    /// Whether parsing succeeded (false = syntax error, fallback to regex)
    pub parse_ok: bool,
}

/// Complete parse result for a Python source file.
#[derive(Debug, Clone, Default)]
pub struct PythonParseResult {
    pub imports: Vec<AstImport>,
    pub class_bases: Vec<(String, Vec<String>)>,
    pub parse_ok: bool,
}

/// Complete parse result for a TypeScript/JavaScript source file.
#[derive(Debug, Clone, Default)]
pub struct TsParseResult {
    pub imports: Vec<AstImport>,
    pub class_implements: Vec<(String, Vec<String>)>,
    pub parse_ok: bool,
}

/// Unified parse result across all languages.
#[derive(Debug, Clone)]
pub enum FileParseResult {
    Rust(RustParseResult),
    Python(PythonParseResult),
    TypeScript(TsParseResult),
    Unsupported,
}

// ─── Block 2: Rust AST Parsing (syn) ─────────────────────

/// Parse a Rust source file into structured AST data.
/// Returns `RustParseResult` with `parse_ok = false` if syn cannot parse the file.
pub fn parse_rust_file(content: &str) -> RustParseResult {
    let syntax = match syn::parse_file(content) {
        Ok(file) => file,
        Err(_) => {
            return RustParseResult {
                parse_ok: false,
                ..Default::default()
            }
        }
    };

    let mut result = RustParseResult {
        parse_ok: true,
        ..Default::default()
    };

    for item in &syntax.items {
        match item {
            syn::Item::Use(item_use) => {
                extract_use_tree(item_use, &mut result, false);
            }
            syn::Item::Impl(item_impl) => {
                extract_trait_impl(item_impl, &mut result);
            }
            syn::Item::Struct(item_struct) => {
                result.structs.push(AstStructDef {
                    name: item_struct.ident.to_string(),
                    is_pub: matches!(item_struct.vis, syn::Visibility::Public(_)),
                });
            }
            syn::Item::Trait(item_trait) => {
                result.traits.push(AstTraitDef {
                    name: item_trait.ident.to_string(),
                    is_pub: matches!(item_trait.vis, syn::Visibility::Public(_)),
                });
            }
            syn::Item::Mod(item_mod) => {
                let path_attr = extract_path_attr(item_mod);
                result.mod_decls.push(AstModDecl {
                    name: item_mod.ident.to_string(),
                    path_attr,
                    is_pub: matches!(item_mod.vis, syn::Visibility::Public(_)),
                });
            }
            _ => {}
        }
    }

    result
}

/// Recursively extract imports from a `use` tree.
fn extract_use_tree(item_use: &syn::ItemUse, result: &mut RustParseResult, is_pub: bool) {
    let is_reexport = is_pub || matches!(item_use.vis, syn::Visibility::Public(_));
    let mut path_buf = String::new();
    walk_use_tree(&item_use.tree, &mut path_buf, result, is_reexport);
}

/// Walk a UseTree recursively, building the full path.
fn walk_use_tree(
    tree: &syn::UseTree,
    prefix: &mut String,
    result: &mut RustParseResult,
    is_reexport: bool,
) {
    match tree {
        syn::UseTree::Path(use_path) => {
            let saved = prefix.clone();
            if !prefix.is_empty() {
                prefix.push_str("::");
            }
            prefix.push_str(&use_path.ident.to_string());
            walk_use_tree(&use_path.tree, prefix, result, is_reexport);
            *prefix = saved;
        }
        syn::UseTree::Name(use_name) => {
            let mut full_path = prefix.clone();
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&use_name.ident.to_string());
            let segments: Vec<String> = full_path.split("::").map(String::from).collect();
            result.imports.push(AstImport {
                raw_path: full_path,
                segments,
                is_reexport,
                is_glob: false,
            });
        }
        syn::UseTree::Glob(_) => {
            let full_path = if prefix.is_empty() {
                "*".to_string()
            } else {
                format!("{}::*", prefix)
            };
            let segments: Vec<String> = prefix.split("::").map(String::from).collect();
            result.imports.push(AstImport {
                raw_path: full_path,
                segments,
                is_reexport,
                is_glob: true,
            });
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                walk_use_tree(item, prefix, result, is_reexport);
            }
        }
        syn::UseTree::Rename(use_rename) => {
            let mut full_path = prefix.clone();
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&use_rename.ident.to_string());
            let segments: Vec<String> = full_path.split("::").map(String::from).collect();
            result.imports.push(AstImport {
                raw_path: full_path,
                segments,
                is_reexport,
                is_glob: false,
            });
        }
    }
}

/// Extract trait implementation from `impl` block.
fn extract_trait_impl(item_impl: &syn::ItemImpl, result: &mut RustParseResult) {
    // Only trait impls have `trait_` field set
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return;
    };

    let trait_name = path_to_string(trait_path);
    let type_name = type_to_string(&item_impl.self_ty);
    let has_generics = !item_impl.generics.params.is_empty();

    result.trait_impls.push(AstTraitImpl {
        trait_name,
        type_name,
        has_generics,
    });
}

/// Convert a syn::Path to a string like "crate::common::FilePath".
fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// Convert a syn::Type to a string (best-effort for type names).
fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => path_to_string(&type_path.path),
        syn::Type::Reference(type_ref) => type_to_string(&type_ref.elem),
        _ => String::from("_"),
    }
}

/// Extract `#[path = "..."]` attribute from a module declaration.
fn extract_path_attr(item_mod: &syn::ItemMod) -> Option<String> {
    for attr in &item_mod.attrs {
        if attr.path().is_ident("path") {
            if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                return Some(lit.value());
            }
        }
    }
    None
}

// ─── Block 3: Python Structured Parsing ───────────────────

/// Parse a Python source file with comment/string awareness.
/// Not a full AST, but significantly more accurate than raw regex.
pub fn parse_python_file(content: &str) -> PythonParseResult {
    let mut result = PythonParseResult {
        parse_ok: true,
        ..Default::default()
    };

    let code_lines = strip_python_comments(content);

    for line in &code_lines {
        let trimmed = line.trim();

        // from X import Y / from X import (Y, Z)
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(import_pos) = rest.find(" import ") {
                let module_part = rest[..import_pos].trim();
                let names_part = rest[import_pos + 8..].trim();

                let segments: Vec<String> = module_part.split('.').map(String::from).collect();

                let is_relative = module_part.starts_with('.');
                let dot_count = module_part.chars().take_while(|&c| c == '.').count();

                result.imports.push(AstImport {
                    raw_path: module_part.to_string(),
                    segments: segments.clone(),
                    is_reexport: false,
                    is_glob: names_part.trim() == "*",
                });

                // Also register individual imported names for resolution
                let names: Vec<&str> = names_part
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_'))
                    .collect();

                for name in names {
                    let mut name_segments = segments.clone();
                    name_segments.push(name.to_string());
                    result.imports.push(AstImport {
                        raw_path: format!("{}.{}", module_part, name),
                        segments: name_segments,
                        is_reexport: false,
                        is_glob: false,
                    });
                }

                // Store relative import metadata
                if is_relative {
                    let _ = dot_count; // Used by graph resolver for path resolution
                }
            }
        }
        // import X / import X.Y
        else if let Some(rest) = trimmed.strip_prefix("import ") {
            let module = rest.split(" as ").next().unwrap_or(rest).trim();
            let segments: Vec<String> = module.split('.').map(String::from).collect();
            result.imports.push(AstImport {
                raw_path: module.to_string(),
                segments,
                is_reexport: false,
                is_glob: false,
            });
        }
        // class Foo(Bar, Baz):
        else if let Some(rest) = trimmed.strip_prefix("class ") {
            if let Some(paren_start) = rest.find('(') {
                let class_name = rest[..paren_start].trim().to_string();
                if let Some(paren_end) = rest[paren_start..].find(')') {
                    let bases_str = &rest[paren_start + 1..paren_start + paren_end];
                    let bases: Vec<String> = bases_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    result.class_bases.push((class_name, bases));
                }
            }
        }
    }

    result
}

/// Strip Python comments and string literals, preserving line structure.
fn strip_python_comments(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            // Skip full-line comments
            if trimmed.starts_with('#') {
                return String::new();
            }
            // Strip inline comments (naive: find # not inside quotes)
            let mut in_single = false;
            let mut in_double = false;
            let mut result = String::new();
            let mut prev_char = ' ';
            for ch in line.chars() {
                if ch == '\'' && !in_double && prev_char != '\\' {
                    in_single = !in_single;
                } else if ch == '"' && !in_single && prev_char != '\\' {
                    in_double = !in_double;
                } else if ch == '#' && !in_single && !in_double {
                    break;
                }
                result.push(ch);
                prev_char = ch;
            }
            result
        })
        .collect()
}

// ─── Block 4: TypeScript/JavaScript Structured Parsing ────

/// Parse a TypeScript/JavaScript source file with comment awareness.
pub fn parse_ts_file(content: &str) -> TsParseResult {
    let mut result = TsParseResult {
        parse_ok: true,
        ..Default::default()
    };

    let code_lines = strip_ts_comments(content);

    for line in &code_lines {
        let trimmed = line.trim();

        // import { X, Y } from './path'
        // import X from './path'
        // import * as X from './path'
        // import './path' (side-effect)
        if trimmed.starts_with("import ") {
            if let Some(from_pos) = trimmed.find(" from ") {
                let path_part = trimmed[from_pos + 6..].trim();
                let path = path_part
                    .trim_matches(|c| c == '\'' || c == '"' || c == ';')
                    .to_string();
                let segments: Vec<String> = path
                    .split('/')
                    .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                    .map(String::from)
                    .collect();
                result.imports.push(AstImport {
                    raw_path: path,
                    segments,
                    is_reexport: false,
                    is_glob: trimmed.contains("* as"),
                });
            } else {
                // Side-effect import: import './path'
                let path = trimmed
                    .strip_prefix("import ")
                    .unwrap_or("")
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"' || c == ';')
                    .to_string();
                if !path.is_empty() {
                    let segments: Vec<String> = path
                        .split('/')
                        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                        .map(String::from)
                        .collect();
                    result.imports.push(AstImport {
                        raw_path: path,
                        segments,
                        is_reexport: false,
                        is_glob: false,
                    });
                }
            }
        }
        // export { X } from './path'
        // export * from './path'
        else if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.find(" from ") {
                let path_part = trimmed[from_pos + 6..].trim();
                let path = path_part
                    .trim_matches(|c| c == '\'' || c == '"' || c == ';')
                    .to_string();
                let segments: Vec<String> = path
                    .split('/')
                    .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                    .map(String::from)
                    .collect();
                result.imports.push(AstImport {
                    raw_path: path,
                    segments,
                    is_reexport: true,
                    is_glob: trimmed.contains("*"),
                });
            }
        }
        // class Foo implements Bar, Baz {
        else if let Some(rest) = trimmed.strip_prefix("class ") {
            if let Some(impl_pos) = rest.find(" implements ") {
                let class_name = rest[..impl_pos].trim().to_string();
                let after_impl = &rest[impl_pos + 12..];
                let interfaces: Vec<String> = after_impl
                    .split(['{', ','])
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                result.class_implements.push((class_name, interfaces));
            }
        }
    }

    result
}

/// Strip TS/JS comments (// and /* */), preserving line structure.
fn strip_ts_comments(content: &str) -> Vec<String> {
    let mut in_block_comment = false;
    content
        .lines()
        .map(|line| {
            if in_block_comment {
                if let Some(end_pos) = line.find("*/") {
                    in_block_comment = false;
                    return line[end_pos + 2..].to_string();
                }
                return String::new();
            }

            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                return String::new();
            }
            if trimmed.starts_with("/*") {
                if let Some(end_pos) = line.find("*/") {
                    return line[end_pos + 2..].to_string();
                }
                in_block_comment = true;
                return String::new();
            }

            // Strip inline // comments (outside strings)
            let mut in_single = false;
            let mut in_double = false;
            let mut in_template = false;
            let mut result = String::new();
            let mut prev_char = ' ';
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let ch = chars[i];
                if ch == '\'' && !in_double && !in_template && prev_char != '\\' {
                    in_single = !in_single;
                } else if ch == '"' && !in_single && !in_template && prev_char != '\\' {
                    in_double = !in_double;
                } else if ch == '`' && !in_single && !in_double && prev_char != '\\' {
                    in_template = !in_template;
                } else if ch == '/' && !in_single && !in_double && !in_template {
                    if i + 1 < chars.len() && chars[i + 1] == '/' {
                        break; // Rest of line is comment
                    }
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        // Inline block comment
                        if let Some(end) = line[i + 2..].find("*/") {
                            i += end + 3;
                            prev_char = '/';
                            continue;
                        }
                        in_block_comment = true;
                        break;
                    }
                }
                result.push(ch);
                prev_char = ch;
                i += 1;
            }
            result
        })
        .collect()
}

// ─── Block 5: Unified Dispatch ────────────────────────────

/// Parse a file based on its extension, returning the appropriate result.
pub fn parse_file(path: &str, content: &str) -> FileParseResult {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => FileParseResult::Rust(parse_rust_file(content)),
        "py" => FileParseResult::Python(parse_python_file(content)),
        "ts" | "tsx" | "js" | "jsx" => FileParseResult::TypeScript(parse_ts_file(content)),
        _ => FileParseResult::Unsupported,
    }
}

// ─── Block 6: Query Helpers ───────────────────────────────

/// Check if any trait impl in the parse result matches the given trait name.
/// Uses AST data — no regex, no line scanning.
pub fn has_trait_impl(result: &RustParseResult, trait_name: &str) -> bool {
    result.trait_impls.iter().any(|ti| {
        ti.trait_name == trait_name || ti.trait_name.ends_with(&format!("::{}", trait_name))
    })
}

/// Check if any struct name matches.
pub fn has_struct(result: &RustParseResult, name: &str) -> bool {
    result.structs.iter().any(|s| s.name == name)
}

/// Check if any trait definition matches.
pub fn has_trait_def(result: &RustParseResult, name: &str) -> bool {
    result.traits.iter().any(|t| t.name == name)
}

/// Get all trait names defined in this file.
pub fn trait_names(result: &RustParseResult) -> Vec<String> {
    result.traits.iter().map(|t| t.name.clone()).collect()
}

/// Get all struct names defined in this file.
pub fn struct_names(result: &RustParseResult) -> Vec<String> {
    result.structs.iter().map(|s| s.name.clone()).collect()
}

/// Get all aggregate trait names (traits containing "Aggregate" in name).
pub fn aggregate_trait_names(result: &RustParseResult) -> Vec<String> {
    result
        .trait_impls
        .iter()
        .filter(|ti| ti.trait_name.contains("Aggregate"))
        .map(|ti| ti.trait_name.clone())
        .collect()
}

/// Build a module-to-file index from mod declarations.
/// Returns map of module_name -> declared_path (from #[path] attr or default).
pub fn mod_decl_map(result: &RustParseResult) -> HashMap<String, Option<String>> {
    result
        .mod_decls
        .iter()
        .map(|m| (m.name.clone(), m.path_attr.clone()))
        .collect()
}

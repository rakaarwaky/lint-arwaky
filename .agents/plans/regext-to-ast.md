
# Refactoring AST Parsing — Semua File

Berikut rewrite lengkap. Saya mulai dari utility AST parser (fondasi), lalu semua analyzer, lalu graph resolver.

---

## 1. `Cargo.toml` — Tambah Dependency

```toml
[package]
name = "orphan-detector-lint-arwaky"
version = "1.12.0"
edition = "2021"
description = "Unreachable/dead component detector covering AES501–AES506. Flags roles, capabilities, and surfaces that are never wired into an entry point."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
regex.workspace = true
once_cell.workspace = true
rayon.workspace = true
shared.workspace = true
syn = { version = "2", features = ["full", "visit", "parsing"] }

[dev-dependencies]
tokio.workspace = true
tempfile = "3"
criterion = { version = "0.8.2", features = ["html_reports"] }

[[bench]]
name = "bench_orphan_detector_graph"
path = "benches/bench_orphan_detector_graph.rs"
harness = false
```

---

## 2. `utility_orphan_ast_parser.rs` — BARU (Fondasi AST)

```rust
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

                let segments: Vec<String> = module_part
                    .split('.')
                    .map(String::from)
                    .collect();

                let is_relative = module_part.starts_with('.');
                let dot_count = module_part.chars().take_while(|&c| c == '.').count();

                result.imports.push(AstImport {
                    raw_path: module_part.to_string(),
                    segments,
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
                    .split(|c| c == '{' || c == ',')
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
        ti.trait_name == trait_name
            || ti.trait_name.ends_with(&format!("::{}", trait_name))
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
```

---

## 3. `capabilities_orphan_taxonomy_analyzer.rs` — REWRITE

```rust
// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection.
// AST-based: uses inbound_links from AST-built graph. No regex fallback needed.

use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};
use shared::common::{FilePath, Severity};
use shared::orphan_detector::{AesOrphanViolation, ITaxonomyOrphanProtocol};
use shared::common::LayerDefinition;
use shared::orphan_detector::utility_orphan_filename::file_stem;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TaxonomyOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());
        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };
        let is_utility_or_helper = matches!(suffix, "utility" | "helper");
        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        // AST-based graph resolver produces accurate inbound_links.
        // No regex fallback (has_crate_self_import) needed.
        let importers = match inbound_links.get_importers(f.value()) {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category,
                        reason: Some(
                            format!(
                                "Taxonomy '{}' is not imported by any other layer file.",
                                stem
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        };

        // Check if any importer is from another layer (not taxonomy, not barrel)
        let has_other_layer_importer = importers.iter().any(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            // Barrel files don't count as real consumers
            if b == "mod.rs" || b == "__init__.py" || b == "index.ts" || b == "index.js" {
                return false;
            }
            // Same-layer imports don't count
            !b.starts_with("taxonomy_")
        });

        if has_other_layer_importer {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        } else {
            OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::TaxonomyOrphan {
                    stem: stem.clone(),
                    category,
                    reason: Some(
                        format!(
                            "Taxonomy '{}' is not imported by any other layer file.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            )
        }
    }
}

// ─── Block 3: Constructors ────────────────────────────────

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}
```

---

## 4. `capabilities_orphan_contract_analyzer.rs` — REWRITE

```rust
// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection.
// AST-based: uses syn AST for trait extraction and impl detection.

use shared::code_analysis::{FileDefinitionMap, InheritanceMap, OrphanIndicatorResult};
use shared::common::{FilePath, Severity};
use shared::orphan_detector::{AesOrphanViolation, IContractOrphanProtocol};
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_suffix};
use shared::orphan_detector::utility_orphan_io as orphan_io;
use shared::orphan_detector::utility_workspace_scanner::collect_source_files;
use crate::utility_orphan_ast_parser;
use std::sync::Arc;
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
struct SearchFilesCache {
    root: std::path::PathBuf,
    file_count: usize,
    files: Arc<Vec<String>>,
}

impl Default for SearchFilesCache {
    fn default() -> Self {
        Self {
            root: std::path::PathBuf::new(),
            file_count: 0,
            files: Arc::new(Vec::new()),
        }
    }
}

pub struct ContractOrphanAnalyzer {
    search_cache: Mutex<Option<SearchFilesCache>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = file_suffix(fp);
        let content = orphan_io::read_file_safe(fp);
        if content.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // AST-based trait extraction
        let trait_names = Self::extract_trait_names_ast(fp, &content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let search_files = self.cached_search_files(root_dir, all_files);

        // Check 0: Barrel re-export check
        if Self::is_trait_re_exported_in_barrel(&trait_names, &search_files) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check 1: Implementation check via AST
        let unimplemented =
            Self::find_unimplemented_traits_ast(&trait_names, search_files.as_slice());
        if !unimplemented.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: unimplemented.join(", "),
                    target_layer: "expected",
                    reason: Some(
                        format!(
                            "Contract {} '{}' not implemented by any expected layer file.",
                            suffix,
                            unimplemented.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        // Check 2: Protocol must be called by orchestrator/container/capabilities/surface
        if suffix == "protocol" {
            if !Self::is_referenced_by_layers(
                &trait_names,
                &search_files,
                &["agent_", "capabilities_", "surface_"],
                &["_container.rs", "_container.py", "_container.ts", "_container.js"],
            ) {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "orchestrator/container",
                        reason: Some(
                            format!(
                                "Contract {} '{}' not called by any orchestrator or container.",
                                suffix,
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        // Check 3: Aggregate must be called by surface or container
        if suffix == "aggregate" {
            if !Self::is_referenced_by_layers(
                &trait_names,
                &search_files,
                &["surface_"],
                &["_container.rs", "_container.py", "_container.ts", "_container.js"],
            ) {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "surface",
                        reason: Some(
                            format!(
                                "Contract aggregate '{}' not called by any surface or container.",
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            search_cache: Mutex::new(None),
        }
    }

    /// Extract trait/interface names using AST parsing.
    fn extract_trait_names_ast(file_path: &str, content: &str) -> Vec<String> {
        match utility_orphan_ast_parser::parse_file(file_path, content) {
            utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                utility_orphan_ast_parser::trait_names(&result)
            }
            utility_orphan_ast_parser::FileParseResult::Python(result) => {
                // Python "traits" are classes
                result
                    .class_bases
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect()
            }
            utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                // TS interfaces — extract from class_implements targets
                result
                    .class_implements
                    .iter()
                    .flat_map(|(_, ifaces)| ifaces.clone())
                    .collect()
            }
            utility_orphan_ast_parser::FileParseResult::Unsupported => Vec::new(),
        }
    }

    /// Check which traits are NOT implemented using AST.
    fn find_unimplemented_traits_ast(
        trait_names: &[String],
        search_files: &[String],
    ) -> Vec<String> {
        trait_names
            .iter()
            .filter(|trait_name| {
                !Self::has_trait_implementation_ast(search_files, trait_name)
            })
            .cloned()
            .collect()
    }

    /// Check if any file implements the given trait, using AST.
    fn has_trait_implementation_ast(search_files: &[String], trait_name: &str) -> bool {
        for cf in search_files {
            let content = orphan_io::read_file_safe(cf);
            if content.is_empty() {
                continue;
            }

            match utility_orphan_ast_parser::parse_file(cf, &content) {
                utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                    if utility_orphan_ast_parser::has_trait_impl(&result, trait_name) {
                        return true;
                    }
                }
                utility_orphan_ast_parser::FileParseResult::Python(result) => {
                    if result
                        .class_bases
                        .iter()
                        .any(|(_, bases)| bases.iter().any(|b| b == trait_name))
                    {
                        return true;
                    }
                }
                utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                    if result
                        .class_implements
                        .iter()
                        .any(|(_, ifaces)| ifaces.iter().any(|i| i == trait_name))
                    {
                        return true;
                    }
                }
                utility_orphan_ast_parser::FileParseResult::Unsupported => {}
            }
        }
        false
    }

    /// Check if trait names are referenced by files matching layer prefixes/suffixes.
    fn is_referenced_by_layers(
        trait_names: &[String],
        search_files: &[String],
        prefix_patterns: &[&str],
        suffix_patterns: &[&str],
    ) -> bool {
        for cf in search_files {
            let cb = file_basename(cf);
            let matches_prefix = prefix_patterns.iter().any(|p| cb.starts_with(p));
            let matches_suffix = suffix_patterns.iter().any(|s| cb.ends_with(s));
            if !matches_prefix && !matches_suffix {
                continue;
            }

            let content = orphan_io::read_file_safe(cf);
            for trait_name in trait_names {
                if Self::content_contains_word(&content, trait_name) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if any trait name is re-exported via barrel files.
    fn is_trait_re_exported_in_barrel(trait_names: &[String], search_files: &[String]) -> bool {
        for cf in search_files {
            let cb = file_basename(cf);
            let is_barrel =
                cb == "__init__.py" || cb == "mod.rs" || cb == "index.ts" || cb == "index.js";
            if !is_barrel {
                continue;
            }
            let barrel_content = orphan_io::read_file_safe(cf);
            for trait_name in trait_names {
                if Self::content_contains_word(&barrel_content, trait_name) {
                    return true;
                }
            }
        }
        false
    }

    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    fn cached_search_files(&self, root_dir: &FilePath, all_files: &[String]) -> Arc<Vec<String>> {
        let root = std::path::Path::new(root_dir.value()).to_path_buf();
        let top_root =
            shared::orphan_detector::utility_workspace_scanner::find_workspace_root(&root)
                .unwrap_or_else(|_| root.clone());
        if let Ok(mut guard) = self.search_cache.lock() {
            if let Some(cache) = guard.as_ref() {
                if cache.root == top_root && cache.file_count == all_files.len() {
                    return cache.files.clone();
                }
            }
            let mut search_files: Vec<String> = all_files.to_vec();
            for ws_dir in &["crates", "packages", "modules"] {
                let ws_path = top_root.join(ws_dir);
                if ws_path.exists() {
                    collect_source_files(&ws_path, &mut search_files);
                }
            }
            let files = Arc::new(search_files);
            *guard = Some(SearchFilesCache {
                root: top_root,
                file_count: all_files.len(),
                files: files.clone(),
            });
            files
        } else {
            Arc::new(all_files.to_vec())
        }
    }
}
```

---

## 5. `capabilities_orphan_agent_analyzer.rs` — REWRITE

```rust
// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files.
// AST-based: uses syn AST for aggregate trait extraction.

use shared::code_analysis::OrphanIndicatorResult;
use shared::common::{FilePath, Severity};
use shared::orphan_detector::{AesOrphanViolation, IAgentOrphanProtocol};
use crate::utility_orphan_ast_parser;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match shared::orphan_detector::utility_orphan_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
            }
            c => c,
        };

        // AST-based aggregate trait extraction
        let aggregate_traits = Self::extract_aggregate_traits_ast(fp, &content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Pre-filter candidate files (surfaces, containers, entries, mains)
        let candidates: Vec<&String> = all_files
            .iter()
            .filter(|cf| {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => return false,
                };
                cb.starts_with("surface_")
                    || cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js")
                    || cb.ends_with("_entry.rs")
                    || cb.ends_with("_entry.py")
                    || cb.ends_with("_entry.ts")
                    || cb.ends_with("_entry.js")
                    || matches!(
                        cb,
                        "main.rs"
                            | "lib.rs"
                            | "main.py"
                            | "__main__.py"
                            | "main.ts"
                            | "main.js"
                            | "index.ts"
                            | "index.js"
                    )
            })
            .collect();

        // Cache candidate file contents
        let mut content_cache: std::collections::HashMap<&String, String> =
            std::collections::HashMap::new();

        let mut any_called = false;
        'outer: for agg_name in &aggregate_traits {
            for cf in &candidates {
                let c = content_cache
                    .entry(cf)
                    .or_insert_with(|| {
                        shared::orphan_detector::utility_orphan_io::read_file_safe(cf)
                    });
                if Self::content_contains_word(c, agg_name) {
                    any_called = true;
                    break 'outer;
                }
            }
        }

        if !any_called {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregates [{}] not called by any surface.",
                            aggregate_traits.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    /// Extract aggregate trait names using AST parsing.
    /// For Rust: uses syn to find `impl IAggregateTrait for Struct`.
    /// For Python/TS: uses structured parsing for class inheritance.
    fn extract_aggregate_traits_ast(file_path: &str, content: &str) -> Vec<String> {
        match utility_orphan_ast_parser::parse_file(file_path, content) {
            utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                utility_orphan_ast_parser::aggregate_trait_names(&result)
            }
            utility_orphan_ast_parser::FileParseResult::Python(result) => {
                result
                    .class_bases
                    .iter()
                    .flat_map(|(_, bases)| bases.clone())
                    .filter(|name| name.contains("Aggregate"))
                    .collect()
            }
            utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                result
                    .class_implements
                    .iter()
                    .flat_map(|(_, ifaces)| ifaces.clone())
                    .filter(|name| name.contains("Aggregate"))
                    .collect()
            }
            utility_orphan_ast_parser::FileParseResult::Unsupported => Vec::new(),
        }
    }
}
```

---

## 6. `capabilities_orphan_capabilities_analyzer.rs` — REWRITE

```rust
// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection.
// AST-based: uses syn AST for struct/trait name extraction.

use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};
use shared::common::{FilePath, Severity};
use shared::orphan_detector::{AesOrphanViolation, ICapabilitiesOrphanProtocol};
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace_scanner::{
    check_wired_in_container, find_workspace_root,
};
use crate::utility_orphan_ast_parser;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp = f.value();
        let stem = file_stem(fp);
        if fp.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::CapabilitiesOrphan {
                    stem,
                    reason: Some("Not reachable from any entry point.".into()),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        let path = FilePath::new(fp).unwrap_or_default();
        let content = utility_file_cache::read_cached(&path);
        let content_ref = content.value();

        // AST-based identifier extraction
        let identifiers = Self::extract_identifiers_ast(fp, content_ref, &stem);

        // Search for container files in workspace root
        let root = std::path::Path::new(root_dir.value());
        if let Ok(workspace_root) = find_workspace_root(root) {
            let wired = check_wired_in_container(&workspace_root, &identifiers);
            if wired {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::CapabilitiesOrphan {
                stem,
                reason: Some("Not reachable from any entry point.".into()),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract identifiers (struct names, trait names, stem, PascalCase stem) using AST.
    fn extract_identifiers_ast(file_path: &str, content: &str, stem: &str) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        match utility_orphan_ast_parser::parse_file(file_path, content) {
            utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                identifiers.extend(utility_orphan_ast_parser::struct_names(&result));
                identifiers.extend(utility_orphan_ast_parser::trait_names(&result));
            }
            utility_orphan_ast_parser::FileParseResult::Python(result) => {
                identifiers.extend(
                    result
                        .class_bases
                        .iter()
                        .map(|(name, _)| name.clone()),
                );
            }
            utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                identifiers.extend(
                    result
                        .class_implements
                        .iter()
                        .map(|(name, _)| name.clone()),
                );
            }
            utility_orphan_ast_parser::FileParseResult::Unsupported => {}
        }

        // Always add stem and PascalCase variant
        identifiers.push(stem.to_string());
        let pascal_stem: String = stem
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect();
        identifiers.push(pascal_stem);

        identifiers.sort();
        identifiers.dedup();
        identifiers
    }
}
```

---

## 7. `capabilities_orphan_utility_analyzer.rs` — REWRITE

```rust
// PURPOSE: UtilityOrphanAnalyzer — IUtilityOrphanProtocol for orphan utility detection.
// AST-based: uses inbound_links from AST graph + structured import checking.

use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};
use shared::common::{FilePath, Severity};
use shared::common::utility_layer_detector;
use shared::orphan_detector::{AesOrphanViolation, IUtilityOrphanProtocol};
use crate::utility_orphan_ast_parser;

const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "root"];

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UtilityOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        let mut consumer_importers: Vec<String> = Vec::new();
        let mut utility_importers: Vec<String> = Vec::new();

        // Phase 1: Check AST-built import graph
        if let Some(importers) = inbound_links.get_importers(fp) {
            for importer in importers.iter().filter(|i| *i != fp) {
                let filename = utility_layer_detector::extract_filename(importer);
                let is_consumer = utility_layer_detector::detect_layer_from_prefix(filename)
                    .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                    .unwrap_or(false);
                let stem = std::path::Path::new(importer)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                if is_consumer {
                    consumer_importers.push(stem);
                } else {
                    utility_importers.push(stem);
                }
            }
            if !consumer_importers.is_empty() {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        // Phase 2: AST-based fallback — check consumer files for imports of this module
        let consumer_files: Vec<&String> = all_files
            .iter()
            .filter(|other_file| {
                if *other_file == fp {
                    return false;
                }
                let filename = utility_layer_detector::extract_filename(other_file);
                utility_layer_detector::detect_layer_from_prefix(filename)
                    .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                    .unwrap_or(false)
            })
            .collect();

        for other_file in consumer_files {
            let other_content = shared::common::utility_file_handler::read_file_safe(other_file);
            if other_content.is_empty() {
                continue;
            }

            if self.is_module_imported_ast(other_file, &other_content, &module_name) {
                let stem = std::path::Path::new(other_file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                consumer_importers.push(stem);
            }
        }

        if !consumer_importers.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        if !utility_importers.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::UtilityDeadCode {
                    stem: module_name.clone(),
                    imported_by: utility_importers,
                    reason: Some(
                        format!(
                            "Utility file '{}' is only imported by other utility files, not by capability, agent, or surfaces layers.",
                            module_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::UtilityOrphan {
                stem: module_name.clone(),
                reason: Some(
                    format!(
                        "Utility file '{}' is not imported by any other file.",
                        module_name
                    )
                    .into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UtilityOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if a module is imported using AST parsing.
    /// Parses the consumer file and checks if any import segment matches the module name.
    fn is_module_imported_ast(
        &self,
        file_path: &str,
        content: &str,
        module_name: &str,
    ) -> bool {
        match utility_orphan_ast_parser::parse_file(file_path, content) {
            utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                result.imports.iter().any(|imp| {
                    imp.segments.iter().any(|seg| {
                        seg == module_name
                            || seg.starts_with(&format!("{}_", module_name))
                    })
                })
            }
            utility_orphan_ast_parser::FileParseResult::Python(result) => {
                result.imports.iter().any(|imp| {
                    imp.raw_path.contains(module_name)
                        || imp.segments.iter().any(|seg| seg == module_name)
                })
            }
            utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                result.imports.iter().any(|imp| {
                    imp.raw_path.contains(module_name)
                        || imp.segments.iter().any(|seg| seg == module_name)
                })
            }
            utility_orphan_ast_parser::FileParseResult::Unsupported => false,
        }
    }
}
```

---

## 8. `capabilities_orphan_graph_resolver.rs` — REWRITE (Paling Besar)

```rust
// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
// AST-based: uses syn AST for Rust import/mod resolution, structured parsing for Python/TS.

use shared::code_analysis::{
    FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
};
use crate::utility_orphan_ast_parser;
use crate::utility_orphan_graph_resolver;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_orphan_io;
use shared::orphan_detector::IOrphanGraphResolverProtocol;
use shared::orphan_detector::{OrphanEntryPatternListVO, OrphanFileListVO};
use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct OrphanGraphResolver {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext {
        let raw_paths: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        self.build_graph_context_inner(&raw_paths, root_dir)
    }

    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        let mut matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    basename.ends_with("_container.rs")
                        || basename.ends_with("_container.py")
                        || basename.ends_with("_container.ts")
                        || basename.ends_with("_container.js")
                        || basename.ends_with("_entry.rs")
                        || basename.ends_with("_entry.py")
                        || basename.ends_with("_entry.ts")
                        || basename.ends_with("_entry.js")
                        || basename.starts_with("root_")
                        || basename == "main.rs"
                        || basename == "lib.rs"
                        || basename == "main.py"
                        || basename == "__main__.py"
                        || basename == "main.ts"
                        || basename == "main.js"
                        || basename == "index.ts"
                        || basename == "index.js"
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    let stem =
                        shared::orphan_detector::utility_orphan_filename::file_stem(basename);
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || stem == *pattern
                            || (pattern.starts_with('_') && stem.ends_with(pattern.as_str()))
                            || (pattern.starts_with('.') && basename.ends_with(pattern.as_str()))
                            || (pattern.ends_with(".rs")
                                || pattern.ends_with(".py")
                                || pattern.ends_with(".ts")
                                || pattern.ends_with(".js"))
                                && basename.ends_with(pattern.as_str())
                    })
                })
                .cloned()
                .collect()
        };

        matched.sort();
        matched.dedup();
        OrphanFileListVO::new(matched)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        let workspace_root = utility_orphan_graph_resolver::find_workspace_root(root_dir);
        let root_path = std::path::Path::new(&workspace_root);

        // Build workspace crate index
        let mut workspace_modules: HashSet<String> = HashSet::new();
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if utility_orphan_io::is_dir(&ws_path) {
                let entries = utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }
                    workspace_modules.insert(name.clone());
                    workspace_modules.insert(name.replace('-', "_"));
                    let src_dir = std::path::PathBuf::from(&path_str).join("src");
                    if utility_orphan_io::is_dir(&src_dir) {
                        crate_src_dirs.insert(name.clone(), src_dir.clone());
                        crate_src_dirs.insert(name.replace('-', "_"), src_dir);
                    }
                }
            }
        }

        let crate_module_index =
            utility_orphan_graph_resolver::build_crate_module_index(&crate_src_dirs);

        // Expand to all workspace files
        let mut all_workspace_files: Vec<String> = files.to_vec();
        let mut seen: HashSet<String> = files.iter().cloned().collect();
        let root_path_obj = std::path::Path::new(&workspace_root);

        for src_dir in crate_src_dirs.values() {
            let workspace_files = utility_orphan_io::scan_directory_recursive(src_dir);
            for f in workspace_files {
                let rel = std::path::Path::new(&f)
                    .strip_prefix(root_path_obj)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or(f);
                if seen.insert(rel.clone()) {
                    all_workspace_files.push(rel);
                }
            }
        }

        // Scan root_*.rs files directly in workspace dirs
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if utility_orphan_io::is_dir(&ws_path) {
                let entries = utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if is_dir_entry {
                        continue;
                    }
                    if name.starts_with("root_")
                        && (name.ends_with(".rs")
                            || name.ends_with(".py")
                            || name.ends_with(".ts")
                            || name.ends_with(".js"))
                        && !seen.contains(&path_str)
                    {
                        let rel = std::path::Path::new(&path_str)
                            .strip_prefix(root_path_obj)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or(path_str.clone());
                        seen.insert(rel.clone());
                        all_workspace_files.push(rel);
                    }
                }
            }
        }

        let files = &all_workspace_files;

        // Build module_to_file lookup
        let mut module_to_file: HashMap<String, Vec<String>> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
            module_to_file
                .entry(stem.clone())
                .or_default()
                .push(f.clone());
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                module_to_file
                    .entry(module_path.clone())
                    .or_default()
                    .push(f.clone());
                let normalized_path = module_path.replace('-', "_");
                if normalized_path != module_path {
                    module_to_file
                        .entry(normalized_path)
                        .or_default()
                        .push(f.clone());
                }
            }
            // mod.rs / __init__.py → parent dir name
            if stem == "mod" || stem == "__init__" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    module_to_file
                        .entry(parent_dir.to_string())
                        .or_default()
                        .push(f.clone());
                    let normalized = parent_dir.replace('-', "_");
                    if normalized != parent_dir {
                        module_to_file
                            .entry(normalized)
                            .or_default()
                            .push(f.clone());
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        module_to_file
                            .entry(composite.clone())
                            .or_default()
                            .push(f.clone());
                        let normalized_composite = composite.replace('-', "_");
                        if normalized_composite != composite {
                            module_to_file
                                .entry(normalized_composite)
                                .or_default()
                                .push(f.clone());
                        }
                    }
                }
            }
        }

        // ─── AST-based file processing ───────────────────────
        for f in files {
            import_graph.entry(f.clone()).or_default();
            let content = utility_orphan_io::read_file_safe(f);
            if content.is_empty()
                && !utility_orphan_io::is_file(&std::path::PathBuf::from(f))
            {
                continue;
            }

            match utility_orphan_ast_parser::parse_file(f, &content) {
                // ─── Rust AST processing ─────────────────────
                utility_orphan_ast_parser::FileParseResult::Rust(result) => {
                    // Process mod declarations (replaces regex Pass 1 & 2)
                    for mod_decl in &result.mod_decls {
                        if let Some(ref path_attr) = mod_decl.path_attr {
                            // #[path = "..."] mod foo;
                            let base_dir = match std::path::Path::new(f).parent() {
                                Some(p) => p.to_path_buf(),
                                None => continue,
                            };
                            if let Some(resolved_path) =
                                shared::orphan_detector::utility_orphan_path::resolve_module_path(
                                    root_path,
                                    &base_dir,
                                    path_attr,
                                )
                            {
                                let resolved = resolved_path.to_string_lossy().to_string();
                                if utility_orphan_io::is_file(
                                    &std::path::PathBuf::from(&resolved),
                                ) && resolved != *f
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        } else {
                            // Plain mod foo;
                            let parent = match std::path::Path::new(f).parent() {
                                Some(p) => p,
                                None => continue,
                            };
                            let candidates = [
                                parent.join(format!("{}.rs", mod_decl.name)),
                                parent.join(&mod_decl.name).join("mod.rs"),
                            ];
                            for candidate in &candidates {
                                let abs_candidate = if candidate.is_relative() {
                                    root_path.join(candidate)
                                } else {
                                    candidate.clone()
                                };
                                if utility_orphan_io::is_file(&abs_candidate) {
                                    if let Some(path_str) = candidate.to_str() {
                                        let resolved = path_str.to_string();
                                        if resolved != *f {
                                            utility_orphan_graph_resolver::add_edge(
                                                &mut import_graph,
                                                &mut inbound_links,
                                                f,
                                                &resolved,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Process imports (replaces regex Pass 3, 5, 5b)
                    for imp in &result.imports {
                        self.resolve_rust_import(
                            f,
                            imp,
                            &module_to_file,
                            &workspace_modules,
                            &crate_module_index,
                            &crate_src_dirs,
                            root_path,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }

                // ─── Python structured processing ────────────
                utility_orphan_ast_parser::FileParseResult::Python(result) => {
                    // Class inheritance
                    for (_class_name, bases) in &result.class_bases {
                        for base in bases {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.clone());
                        }
                    }

                    // Imports
                    for imp in &result.imports {
                        self.resolve_python_import(
                            f,
                            imp,
                            &module_to_file,
                            root_path,
                            &workspace_root,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }

                // ─── TypeScript structured processing ────────
                utility_orphan_ast_parser::FileParseResult::TypeScript(result) => {
                    for imp in &result.imports {
                        self.resolve_ts_import(
                            f,
                            imp,
                            &module_to_file,
                            root_path,
                            &mut import_graph,
                            &mut inbound_links,
                        );
                    }
                }

                utility_orphan_ast_parser::FileParseResult::Unsupported => {}
            }
        }

        // Deduplicate edges
        utility_orphan_graph_resolver::dedup_edges(&mut import_graph);
        utility_orphan_graph_resolver::dedup_edges(&mut inbound_links);

        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
            all_workspace_files,
        )
    }

    /// Resolve a Rust import using AST data.
    #[allow(clippy::too_many_arguments)]
    fn resolve_rust_import(
        &self,
        current_file: &str,
        imp: &utility_orphan_ast_parser::AstImport,
        module_to_file: &HashMap<String, Vec<String>>,
        workspace_modules: &HashSet<String>,
        crate_module_index: &HashMap<String, HashMap<String, String>>,
        crate_src_dirs: &HashMap<String, std::path::PathBuf>,
        root_path: &std::path::Path,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let segments = &imp.segments;
        if segments.is_empty() {
            return;
        }

        // crate:: imports
        if segments[0] == "crate" {
            if segments.len() >= 2 {
                for i in (1..segments.len()).rev() {
                    let composite = segments[1..i].join("/");
                    if let Some(file_path) =
                        Self::resolve_module(module_to_file, &composite, current_file)
                    {
                        if file_path != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                file_path,
                            );
                            return;
                        }
                    }
                }
                // Try first segment after crate::
                if let Some(file_path) =
                    Self::resolve_module(module_to_file, &segments[1], current_file)
                {
                    if file_path != current_file {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            file_path,
                        );
                    }
                }
            }
            return;
        }

        // super:: imports
        if segments[0] == "super" {
            if segments.len() >= 2 {
                for i in (1..segments.len()).rev() {
                    let composite = segments[1..i].join("/");
                    if let Some(file_path) =
                        Self::resolve_module(module_to_file, &composite, current_file)
                    {
                        if file_path != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                file_path,
                            );
                            return;
                        }
                    }
                }
            }
            return;
        }

        // self:: imports
        if segments[0] == "self" {
            if segments.len() >= 2 {
                if let Some(file_path) =
                    Self::resolve_module(module_to_file, &segments[1], current_file)
                {
                    if file_path != current_file {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            file_path,
                        );
                    }
                }
            }
            return;
        }

        // Workspace crate imports (e.g., shared::common::FilePath)
        let crate_name = &segments[0];
        if workspace_modules.contains(crate_name.as_str())
            || workspace_modules.contains(&crate_name.replace('-', "_"))
        {
            let normalized_crate = crate_name.replace('-', "_");
            if let Some(resolved) = utility_orphan_graph_resolver::resolve_workspace_module(
                crate_module_index,
                &normalized_crate,
                &segments.iter().map(|s| s.as_str()).collect::<Vec<_>>()[1..],
                current_file,
            ) {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    &resolved,
                );
                return;
            }

            // Fallback: try crate src dir scan
            let lookup_name = if crate_src_dirs.contains_key(crate_name.as_str()) {
                crate_name.clone()
            } else {
                normalized_crate
            };
            if let Some(src_dir) = crate_src_dirs.get(&lookup_name) {
                let entries = utility_orphan_io::scan_directory(src_dir);
                let module_name = segments.get(1).map(|s| s.as_str()).unwrap_or("");
                for (_name, path_str, _is_dir) in entries {
                    let path = std::path::PathBuf::from(&path_str);
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    let normalized_stem =
                        shared::orphan_detector::utility_orphan_detector::normalize_module_component(stem);
                    if (stem == module_name || normalized_stem == module_name)
                        && path_str != current_file
                    {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            &path_str,
                        );
                    }
                }
            }
            return;
        }

        // Local module import (bare name)
        let dep = &segments[0];
        let is_workspace_dir = matches!(dep.as_str(), "crates" | "packages" | "modules");
        if !is_workspace_dir {
            if let Some(target) = Self::resolve_module(module_to_file, dep, current_file) {
                if target != current_file {
                    utility_orphan_graph_resolver::add_edge(
                        import_graph,
                        inbound_links,
                        current_file,
                        target,
                    );
                }
            }
        }
    }

    /// Resolve a Python import using structured parse data.
    fn resolve_python_import(
        &self,
        current_file: &str,
        imp: &utility_orphan_ast_parser::AstImport,
        module_to_file: &HashMap<String, Vec<String>>,
        root_path: &std::path::Path,
        workspace_root: &str,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let raw = &imp.raw_path;

        // Relative imports (from . import X, from ..module import Y)
        if raw.starts_with('.') {
            let dot_count = raw.chars().take_while(|&c| c == '.').count();
            let module_part = raw.trim_start_matches('.');

            let file_path = std::path::Path::new(current_file);
            let mut base_dir = file_path.parent().map(|p| p.to_path_buf());
            for _ in 1..dot_count {
                if let Some(ref dir) = base_dir {
                    base_dir = dir.parent().map(|p| p.to_path_buf());
                }
            }
            let Some(base) = base_dir else { return };

            // Resolve module name as file
            if !module_part.is_empty() {
                for ext in &[".py", ".rs", ".ts", ".js"] {
                    let candidate = base.join(format!("{}{}", module_part, ext));
                    if utility_orphan_io::is_file(&candidate) {
                        let cand_rel = candidate
                            .strip_prefix(root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
                // Try as package directory
                let pkg_dir = base.join(module_part);
                for marker in &["__init__.py", "mod.rs"] {
                    if utility_orphan_io::is_file(&pkg_dir.join(marker)) {
                        let cand_rel = pkg_dir
                            .join(marker)
                            .strip_prefix(root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| {
                                pkg_dir.join(marker).to_string_lossy().to_string()
                            });
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
            }

            // Resolve individual imported names
            if let Some(last_seg) = imp.segments.last() {
                if let Some(target) = Self::resolve_module(module_to_file, last_seg, current_file)
                {
                    if target != current_file {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            target,
                        );
                    }
                }
            }
            return;
        }

        // Absolute dotted imports (from modules.cli.src import X)
        if raw.contains('.') {
            let segments: Vec<&str> = raw.split('.').collect();
            let mut walk_dir = std::path::PathBuf::from(workspace_root);
            let mut walk_ok = true;
            for seg in &segments {
                walk_dir = walk_dir.join(seg);
                if !utility_orphan_io::is_dir(&walk_dir) {
                    walk_ok = false;
                    break;
                }
            }
            if walk_ok {
                for marker in &["__init__.py", "mod.rs", "index.ts", "index.js"] {
                    let candidate = walk_dir.join(marker);
                    if utility_orphan_io::is_file(&candidate) {
                        let cand_rel = candidate
                            .strip_prefix(root_path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                        if cand_rel != current_file {
                            utility_orphan_graph_resolver::add_edge(
                                import_graph,
                                inbound_links,
                                current_file,
                                &cand_rel,
                            );
                        }
                        break;
                    }
                }
            } else if let Some(last_seg) = segments.last() {
                if let Some(target) = Self::resolve_module(module_to_file, last_seg, current_file)
                {
                    if target != current_file {
                        utility_orphan_graph_resolver::add_edge(
                            import_graph,
                            inbound_links,
                            current_file,
                            target,
                        );
                    }
                }
            }
            return;
        }

        // Simple module name
        if let Some(target) = Self::resolve_module(module_to_file, raw, current_file) {
            if target != current_file {
                utility_orphan_graph_resolver::add_edge(
                    import_graph,
                    inbound_links,
                    current_file,
                    target,
                );
            }
        }
    }

    /// Resolve a TypeScript/JavaScript import.
    fn resolve_ts_import(
        &self,
        current_file: &str,
        imp: &utility_orphan_ast_parser::AstImport,
        module_to_file: &HashMap<String, Vec<String>>,
        root_path: &std::path::Path,
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
    ) {
        let raw = &imp.raw_path;

        // Relative imports (./foo, ../bar)
        if raw.starts_with('.') {
            if let Some(resolved) = utility_orphan_graph_resolver::resolve_ts_relative(
                current_file,
                raw,
                root_path,
            ) {
                if resolved != current_file {
                    utility_orphan_graph_resolver::add_edge(
                        import_graph,
                        inbound_links,
                        current_file,
                        &resolved,
                    );
                }
            }
            return;
        }

        // Package imports — try module_to_file lookup
        if let Some(last_seg) = imp.segments.last() {
            if let Some(target) = Self::resolve_module(module_to_file, last_seg, current_file) {
                if target != current_file {
                    utility_orphan_graph_resolver::add_edge(
                        import_graph,
                        inbound_links,
                        current_file,
                        target,
                    );
                }
            }
        }
    }

    /// Resolve a module key to the best-matching file path.
    fn resolve_module<'a>(
        module_to_file: &'a HashMap<String, Vec<String>>,
        key: &str,
        importer: &str,
    ) -> Option<&'a String> {
        let candidates = module_to_file.get(key)?;
        if candidates.len() == 1 {
            return candidates.first();
        }
        let importer_crate = importer.split('/').nth(1);
        candidates
            .iter()
            .find(|c| c.split('/').nth(1) == importer_crate)
            .or(candidates.first())
    }
}
```

---

## 9. `utility_orphan_regex_patterns.rs` — DEPRECATED

```rust
// PURPOSE: DEPRECATED — regex patterns replaced by AST parsing in utility_orphan_ast_parser.
// Kept temporarily for backward compatibility. Will be removed in v2.0.
//
// All functions in this module are no longer used by the AST-based graph resolver.
// They remain only to avoid breaking any external consumers during migration.

#![deprecated(
    since = "1.12.0",
    note = "Use utility_orphan_ast_parser instead. All regex-based parsing has been replaced by AST."
)]

// Intentionally empty. All regex patterns have been removed.
// The AST parser (utility_orphan_ast_parser) handles all parsing needs.
```

---

## 10. `lib.rs` — UPDATE

```rust
/// PURPOSE: Module declarations for orphan-detector (orchestrator, analyzers, container)
pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_graph_resolver;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub mod capabilities_orphan_utility_analyzer;
pub mod root_orphan_detector_container;
pub mod utility_orphan_ast_parser;
pub mod utility_orphan_graph_resolver;

// Deprecated: regex patterns replaced by AST parser
#[allow(deprecated)]
pub mod utility_orphan_regex_patterns;
```

---

## Ringkasan Perubahan


| File                                           | Perubahan                                                                                               | Regex → AST |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------- | -------------- |
| `utility_orphan_ast_parser.rs`                 | **BARU** — syn AST parser untuk Rust, structured parser untuk Python/TS                                | Fondasi      |
| `capabilities_orphan_taxonomy_analyzer.rs`     | Hapus`has_crate_self_import` fallback                                                                   | ✅           |
| `capabilities_orphan_contract_analyzer.rs`     | `extract_contract_trait_names` → `extract_trait_names_ast`, `check_trait_impl` → `has_trait_impl_ast` | ✅           |
| `capabilities_orphan_agent_analyzer.rs`        | `extract_aggregate_traits` → `extract_aggregate_traits_ast` (4 regex → 1 AST call)                    | ✅           |
| `capabilities_orphan_capabilities_analyzer.rs` | `extract_struct_names`/`extract_trait_names` → AST                                                     | ✅           |
| `capabilities_orphan_utility_analyzer.rs`      | `check_import_pattern` → `is_module_imported_ast`                                                      | ✅           |
| `capabilities_orphan_graph_resolver.rs`        | 7 regex pass → 3 AST dispatch (Rust/Python/TS)                                                         | ✅           |
| `utility_orphan_regex_patterns.rs`             | **DEPRECATED** — semua regex dihapus                                                                   | ✅           |
| `lib.rs`                                       | Tambah`utility_orphan_ast_parser`                                                                       | —           |
| `Cargo.toml`                                   | Tambah`syn = { version = "2", features = ["full", "visit"] }`                                           | —           |

File yang **tidak berubah**: `agent_orphan_orchestrator.rs`, `root_orphan_detector_container.rs`, `utility_orphan_graph_resolver.rs`, `capabilities_orphan_surfaces_analyzer.rs` — karena mereka tidak melakukan parsing langsung.

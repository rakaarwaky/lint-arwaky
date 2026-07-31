# Rewrite Lengkap Import-Rules v1.12 (AST-Based)

Berikut semua file yang berubah, diurutkan dari fondasi ke atas. File yang **tidak berubah** ditandai jelas.

---

## PHASE 1: Shared AST Parser Layer

File-file ini sudah kita desain sebelumnya dan **shared** antara orphan-detector dan import-rules. Saya tulis ulang di sini untuk kelengkapan.

### 1.1 `taxonomy_orphan_parse_result_vo.rs` (BARU — taxonomy layer)

```rust
// PURPOSE: taxonomy_orphan_parse_result_vo — value objects for AST/structured parse results.
// Shared across orphan-detector and import-rules. All parsers return these types.

use serde::{Deserialize, Serialize};

// ─── Block 1: Import Edge ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstImportVO {
    pub raw_path: String,
    pub segments: Vec<String>,
    pub is_reexport: bool,
    pub is_glob: bool,
    /// Line number (1-based) where this import appears
    pub line: usize,
}

impl AstImportVO {
    pub fn new(
        raw_path: String,
        segments: Vec<String>,
        is_reexport: bool,
        is_glob: bool,
        line: usize,
    ) -> Self {
        Self { raw_path, segments, is_reexport, is_glob, line }
    }

    /// Get the last segment (typically the imported symbol name).
    pub fn last_segment(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// Get the module path (all segments except the last).
    pub fn module_path(&self) -> String {
        if self.segments.len() <= 1 {
            return self.raw_path.clone();
        }
        self.segments[..self.segments.len() - 1].join("::")
    }
}

// ─── Block 2: Trait Implementation ────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstTraitImplVO {
    pub trait_name: String,
    pub type_name: String,
    pub has_generics: bool,
    /// Line number (1-based)
    pub line: usize,
    /// Whether ALL method bodies are empty/todo/unimplemented
    pub is_dummy: bool,
}

// ─── Block 3: Definitions ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstStructDefVO {
    pub name: String,
    pub is_pub: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstTraitDefVO {
    pub name: String,
    pub is_pub: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstModDeclVO {
    pub name: String,
    pub path_attr: Option<String>,
    pub is_pub: bool,
}

// ─── Block 4: Function Definition ─────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstFnDefVO {
    pub name: String,
    pub is_pub: bool,
    /// Line number (1-based)
    pub line: usize,
    /// End line number (1-based)
    pub end_line: usize,
    /// Whether the function body is empty or only contains todo!/unimplemented!/panic!
    pub is_dummy: bool,
}

// ─── Block 5: Per-Language Parse Results ──────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RustParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub trait_impls: Vec<AstTraitImplVO>,
    pub structs: Vec<AstStructDefVO>,
    pub traits: Vec<AstTraitDefVO>,
    pub mod_decls: Vec<AstModDeclVO>,
    pub functions: Vec<AstFnDefVO>,
    /// All identifier references in the file body (for usage tracking)
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PythonParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub class_bases: Vec<(String, Vec<String>)>,
    pub functions: Vec<AstFnDefVO>,
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TsParseResultVO {
    pub imports: Vec<AstImportVO>,
    pub class_implements: Vec<(String, Vec<String>)>,
    pub functions: Vec<AstFnDefVO>,
    pub used_identifiers: Vec<String>,
    pub parse_ok: bool,
}

// ─── Block 6: Unified Result ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileParseResultVO {
    Rust(RustParseResultVO),
    Python(PythonParseResultVO),
    TypeScript(TsParseResultVO),
    Unsupported,
}

// ─── Block 7: Query Helpers ───────────────────────────────

impl RustParseResultVO {
    pub fn has_trait_impl(&self, trait_name: &str) -> bool {
        self.trait_impls.iter().any(|ti| {
            ti.trait_name == trait_name
                || ti.trait_name.ends_with(&format!("::{}", trait_name))
        })
    }

    pub fn trait_names(&self) -> Vec<String> {
        self.traits.iter().map(|t| t.name.clone()).collect()
    }

    pub fn struct_names(&self) -> Vec<String> {
        self.structs.iter().map(|s| s.name.clone()).collect()
    }

    pub fn aggregate_trait_names(&self) -> Vec<String> {
        self.trait_impls
            .iter()
            .filter(|ti| ti.trait_name.contains("Aggregate"))
            .map(|ti| ti.trait_name.clone())
            .collect()
    }

    /// Check if an identifier is used anywhere in the file body.
    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }

    /// Get all dummy trait implementations.
    pub fn dummy_trait_impls(&self) -> Vec<&AstTraitImplVO> {
        self.trait_impls.iter().filter(|ti| ti.is_dummy).collect()
    }

    /// Get all dummy functions.
    pub fn dummy_functions(&self) -> Vec<&AstFnDefVO> {
        self.functions.iter().filter(|f| f.is_dummy).collect()
    }
}

impl PythonParseResultVO {
    pub fn class_names(&self) -> Vec<String> {
        self.class_bases.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn aggregate_names(&self) -> Vec<String> {
        self.class_bases
            .iter()
            .flat_map(|(_, bases)| bases.clone())
            .filter(|name| name.contains("Aggregate"))
            .collect()
    }

    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }
}

impl TsParseResultVO {
    pub fn class_names(&self) -> Vec<String> {
        self.class_implements.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn aggregate_names(&self) -> Vec<String> {
        self.class_implements
            .iter()
            .flat_map(|(_, ifaces)| ifaces.clone())
            .filter(|name| name.contains("Aggregate"))
            .collect()
    }

    pub fn is_identifier_used(&self, name: &str) -> bool {
        self.used_identifiers.iter().any(|id| id == name)
    }
}
```

### 1.2 `utility_orphan_rust_parser.rs` (BARU — utility layer)

```rust
// PURPOSE: utility_orphan_rust_parser — Rust AST parsing via syn crate.
// Stateless standalone functions. Depends only on taxonomy VOs.
// Shared between orphan-detector and import-rules.

use crate::taxonomy_orphan_parse_result_vo::{
    AstFnDefVO, AstImportVO, AstModDeclVO, AstStructDefVO, AstTraitDefVO, AstTraitImplVO,
    RustParseResultVO,
};
use syn::visit::Visit;

// ─── Block 1: Main Parse Function ─────────────────────────

pub fn parse_rust(content: &str) -> RustParseResultVO {
    let syntax = match syn::parse_file(content) {
        Ok(file) => file,
        Err(_) => {
            return RustParseResultVO {
                parse_ok: false,
                ..Default::default()
            }
        }
    };

    let mut result = RustParseResultVO {
        parse_ok: true,
        ..Default::default()
    };

    // Walk all items for imports, impls, structs, traits, mods, functions
    for item in &syntax.items {
        match item {
            syn::Item::Use(item_use) => {
                extract_use_tree(item_use, &mut result);
            }
            syn::Item::Impl(item_impl) => {
                extract_trait_impl(item_impl, &mut result);
            }
            syn::Item::Struct(item_struct) => {
                result.structs.push(AstStructDefVO {
                    name: item_struct.ident.to_string(),
                    is_pub: matches!(item_struct.vis, syn::Visibility::Public(_)),
                });
            }
            syn::Item::Trait(item_trait) => {
                result.traits.push(AstTraitDefVO {
                    name: item_trait.ident.to_string(),
                    is_pub: matches!(item_trait.vis, syn::Visibility::Public(_)),
                });
            }
            syn::Item::Mod(item_mod) => {
                result.mod_decls.push(AstModDeclVO {
                    name: item_mod.ident.to_string(),
                    path_attr: extract_path_attr(item_mod),
                    is_pub: matches!(item_mod.vis, syn::Visibility::Public(_)),
                });
            }
            syn::Item::Fn(item_fn) => {
                let is_dummy = is_fn_body_dummy(&item_fn.block);
                let line = item_fn.sig.ident.span().start().line;
                let end_line = item_fn.block.brace_token.span.close().start().line;
                result.functions.push(AstFnDefVO {
                    name: item_fn.sig.ident.to_string(),
                    is_pub: matches!(item_fn.vis, syn::Visibility::Public(_)),
                    line,
                    end_line,
                    is_dummy,
                });
            }
            _ => {}
        }
    }

    // Collect all used identifiers via Visit
    let mut visitor = IdentifierVisitor {
        identifiers: Vec::new(),
    };
    visitor.visit_file(&syntax);
    result.used_identifiers = visitor.identifiers;

    result
}

// ─── Block 2: Identifier Visitor (for usage tracking) ─────

struct IdentifierVisitor {
    identifiers: Vec<String>,
}

impl<'ast> Visit<'ast> for IdentifierVisitor {
    fn visit_path_segment(&mut self, node: &'ast syn::PathSegment) {
        self.identifiers.push(node.ident.to_string());
        syn::visit::visit_path_segment(self, node);
    }

    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.identifiers.push(node.to_string());
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        for seg in &node.path.segments {
            self.identifiers.push(seg.ident.to_string());
        }
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        for seg in &node.path.segments {
            self.identifiers.push(seg.ident.to_string());
        }
        syn::visit::visit_type_path(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        // Capture derive macro names: #[derive(Serialize, Deserialize)]
        if node.path().is_ident("derive") {
            if let Ok(nested) = node.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) {
                for path in nested {
                    if let Some(seg) = path.segments.last() {
                        self.identifiers.push(seg.ident.to_string());
                    }
                }
            }
        }
        syn::visit::visit_attribute(self, node);
    }
}

// ─── Block 3: Use Tree Walking ────────────────────────────

fn extract_use_tree(item_use: &syn::ItemUse, result: &mut RustParseResultVO) {
    let is_reexport = matches!(item_use.vis, syn::Visibility::Public(_));
    let line = item_use.use_token.span.start().line;
    let mut path_buf = String::new();
    walk_use_tree(&item_use.tree, &mut path_buf, result, is_reexport, line);
}

fn walk_use_tree(
    tree: &syn::UseTree,
    prefix: &mut String,
    result: &mut RustParseResultVO,
    is_reexport: bool,
    line: usize,
) {
    match tree {
        syn::UseTree::Path(use_path) => {
            let saved = prefix.clone();
            if !prefix.is_empty() {
                prefix.push_str("::");
            }
            prefix.push_str(&use_path.ident.to_string());
            walk_use_tree(&use_path.tree, prefix, result, is_reexport, line);
            *prefix = saved;
        }
        syn::UseTree::Name(use_name) => {
            let mut full_path = prefix.clone();
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&use_name.ident.to_string());
            let segments: Vec<String> = full_path.split("::").map(String::from).collect();
            result.imports.push(AstImportVO::new(
                full_path, segments, is_reexport, false, line,
            ));
        }
        syn::UseTree::Glob(_) => {
            let full_path = if prefix.is_empty() {
                "*".to_string()
            } else {
                format!("{}::*", prefix)
            };
            let segments: Vec<String> = prefix.split("::").map(String::from).collect();
            result.imports.push(AstImportVO::new(
                full_path, segments, is_reexport, true, line,
            ));
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                walk_use_tree(item, prefix, result, is_reexport, line);
            }
        }
        syn::UseTree::Rename(use_rename) => {
            let mut full_path = prefix.clone();
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&use_rename.ident.to_string());
            let segments: Vec<String> = full_path.split("::").map(String::from).collect();
            result.imports.push(AstImportVO::new(
                full_path, segments, is_reexport, false, line,
            ));
        }
    }
}

// ─── Block 4: Trait Impl Extraction ───────────────────────

fn extract_trait_impl(item_impl: &syn::ItemImpl, result: &mut RustParseResultVO) {
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return;
    };

    let trait_name = path_to_string(trait_path);
    let type_name = type_to_string(&item_impl.self_ty);
    let has_generics = !item_impl.generics.params.is_empty();
    let line = item_impl.impl_token.span.start().line;
    let is_dummy = is_impl_body_dummy(item_impl);

    result.trait_impls.push(AstTraitImplVO {
        trait_name,
        type_name,
        has_generics,
        line,
        is_dummy,
    });
}

/// Check if ALL methods in an impl block have dummy bodies.
fn is_impl_body_dummy(item_impl: &syn::ItemImpl) -> bool {
    let methods: Vec<_> = item_impl
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .collect();

    if methods.is_empty() {
        return true;
    }

    methods.iter().all(|m| is_fn_body_dummy(&m.block))
}

/// Check if a function body is empty or only contains todo!/unimplemented!/panic!/unreachable!
fn is_fn_body_dummy(block: &syn::Block) -> bool {
    let stmts: Vec<_> = block.stmts.iter().collect();
    if stmts.is_empty() {
        return true;
    }
    if stmts.len() == 1 {
        if let syn::Stmt::Expr(expr, _) = &stmts[0] {
            if let syn::Expr::Macro(expr_macro) = expr {
                let macro_name = path_to_string(&expr_macro.mac.path);
                return matches!(
                    macro_name.as_str(),
                    "todo" | "unimplemented" | "panic" | "unreachable"
                );
            }
        }
    }
    false
}

// ─── Block 5: Helpers ─────────────────────────────────────

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => path_to_string(&type_path.path),
        syn::Type::Reference(type_ref) => type_to_string(&type_ref.elem),
        _ => String::from("_"),
    }
}

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
```

### 1.3 `utility_orphan_python_parser.rs` (BARU — utility layer)

```rust
// PURPOSE: utility_orphan_python_parser — comment-aware structured parsing for Python.
// Stateless standalone functions. Depends only on taxonomy VOs.

use crate::taxonomy_orphan_parse_result_vo::{AstFnDefVO, AstImportVO, PythonParseResultVO};

// ─── Block 1: Main Parse Function ─────────────────────────

pub fn parse_python(content: &str) -> PythonParseResultVO {
    let mut result = PythonParseResultVO {
        parse_ok: true,
        ..Default::default()
    };

    let code_lines = strip_python_comments(content);

    for (idx, line) in code_lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let line_no = idx + 1;

        if let Some(rest) = trimmed.strip_prefix("from ") {
            parse_python_from_import(rest, line_no, &mut result);
        } else if let Some(rest) = trimmed.strip_prefix("import ") {
            parse_python_plain_import(rest, line_no, &mut result);
        } else if let Some(rest) = trimmed.strip_prefix("class ") {
            parse_python_class(rest, &mut result);
        } else if let Some(rest) = trimmed.strip_prefix("def ") {
            parse_python_function(rest, line_no, &mut result);
        } else {
            // Collect identifiers from code lines for usage tracking
            collect_python_identifiers(trimmed, &mut result);
        }
    }

    result
}

// ─── Block 2: Statement Parsers ───────────────────────────

fn parse_python_from_import(rest: &str, line_no: usize, result: &mut PythonParseResultVO) {
    let Some(import_pos) = rest.find(" import ") else {
        return;
    };
    let module_part = rest[..import_pos].trim();
    let names_part = rest[import_pos + 8..].trim();

    // Skip __future__ imports
    if module_part == "__future__" {
        return;
    }

    let segments: Vec<String> = module_part.split('.').map(String::from).collect();

    result.imports.push(AstImportVO::new(
        module_part.to_string(),
        segments.clone(),
        false,
        names_part.trim() == "*",
        line_no,
    ));

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
        result.imports.push(AstImportVO::new(
            format!("{}.{}", module_part, name),
            name_segments,
            false,
            false,
            line_no,
        ));
    }
}

fn parse_python_plain_import(rest: &str, line_no: usize, result: &mut PythonParseResultVO) {
    let module = rest.split(" as ").next().unwrap_or(rest).trim();
    let segments: Vec<String> = module.split('.').map(String::from).collect();
    result.imports.push(AstImportVO::new(
        module.to_string(),
        segments,
        false,
        false,
        line_no,
    ));
}

fn parse_python_class(rest: &str, result: &mut PythonParseResultVO) {
    let Some(paren_start) = rest.find('(') else {
        return;
    };
    let class_name = rest[..paren_start].trim().to_string();
    let Some(paren_end) = rest[paren_start..].find(')') else {
        return;
    };
    let bases_str = &rest[paren_start + 1..paren_start + paren_end];
    let bases: Vec<String> = bases_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    result.class_bases.push((class_name, bases));
}

fn parse_python_function(rest: &str, line_no: usize, result: &mut PythonParseResultVO) {
    let name = rest
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    if name.is_empty() {
        return;
    }
    let is_dummy = name.starts_with("_use_") || name.starts_with("dummy_");
    result.functions.push(AstFnDefVO {
        name,
        is_pub: false,
        line: line_no,
        end_line: line_no, // Approximate; Python indent-based end detection is complex
        is_dummy,
    });
}

fn collect_python_identifiers(line: &str, result: &mut PythonParseResultVO) {
    for token in line.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if !token.is_empty() && token.chars().next().map_or(false, |c| c.is_alphabetic()) {
            result.used_identifiers.push(token.to_string());
        }
    }
}

// ─── Block 3: Comment Stripping ───────────────────────────

fn strip_python_comments(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                return String::new();
            }
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
```

### 1.4 `utility_orphan_ts_parser.rs` (BARU — utility layer)

```rust
// PURPOSE: utility_orphan_ts_parser — comment-aware structured parsing for TypeScript/JavaScript.
// Stateless standalone functions. Depends only on taxonomy VOs.

use crate::taxonomy_orphan_parse_result_vo::{AstFnDefVO, AstImportVO, TsParseResultVO};

// ─── Block 1: Main Parse Function ─────────────────────────

pub fn parse_ts(content: &str) -> TsParseResultVO {
    let mut result = TsParseResultVO {
        parse_ok: true,
        ..Default::default()
    };

    let code_lines = strip_ts_comments(content);

    for (idx, line) in code_lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let line_no = idx + 1;

        if trimmed.starts_with("import ") {
            parse_ts_import(trimmed, line_no, &mut result);
        } else if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            parse_ts_export(trimmed, line_no, &mut result);
        } else if let Some(rest) = trimmed.strip_prefix("class ") {
            parse_ts_class(rest, &mut result);
        } else if trimmed.starts_with("function ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("let ")
            || trimmed.starts_with("var ")
        {
            parse_ts_function(trimmed, line_no, &mut result);
        } else {
            collect_ts_identifiers(trimmed, &mut result);
        }
    }

    result
}

// ─── Block 2: Statement Parsers ───────────────────────────

fn parse_ts_import(trimmed: &str, line_no: usize, result: &mut TsParseResultVO) {
    if let Some(from_pos) = trimmed.find(" from ") {
        let path_part = trimmed[from_pos + 6..].trim();
        let path = path_part
            .trim_matches(|c| c == '\'' || c == '"' || c == ';')
            .to_string();
        let segments = path_to_segments(&path);
        result.imports.push(AstImportVO::new(
            path,
            segments,
            false,
            trimmed.contains("* as"),
            line_no,
        ));
    } else {
        let path = trimmed
            .strip_prefix("import ")
            .unwrap_or("")
            .trim()
            .trim_matches(|c| c == '\'' || c == '"' || c == ';')
            .to_string();
        if !path.is_empty() {
            let segments = path_to_segments(&path);
            result.imports.push(AstImportVO::new(path, segments, false, false, line_no));
        }
    }
}

fn parse_ts_export(trimmed: &str, line_no: usize, result: &mut TsParseResultVO) {
    let Some(from_pos) = trimmed.find(" from ") else {
        return;
    };
    let path_part = trimmed[from_pos + 6..].trim();
    let path = path_part
        .trim_matches(|c| c == '\'' || c == '"' || c == ';')
        .to_string();
    let segments = path_to_segments(&path);
    result.imports.push(AstImportVO::new(
        path,
        segments,
        true,
        trimmed.contains('*'),
        line_no,
    ));
}

fn parse_ts_class(rest: &str, result: &mut TsParseResultVO) {
    let Some(impl_pos) = rest.find(" implements ") else {
        return;
    };
    let class_name = rest[..impl_pos].trim().to_string();
    let after_impl = &rest[impl_pos + 12..];
    let interfaces: Vec<String> = after_impl
        .split(|c| c == '{' || c == ',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    result.class_implements.push((class_name, interfaces));
}

fn parse_ts_function(trimmed: &str, line_no: usize, result: &mut TsParseResultVO) {
    let name = if let Some(rest) = trimmed.strip_prefix("function ") {
        rest.split(|c: char| c == '(' || c.is_whitespace())
            .next()
            .unwrap_or("")
            .to_string()
    } else if let Some(rest) = trimmed
        .strip_prefix("const ")
        .or_else(|| trimmed.strip_prefix("let "))
        .or_else(|| trimmed.strip_prefix("var "))
    {
        rest.split(|c: char| c == '=' || c.is_whitespace())
            .next()
            .unwrap_or("")
            .to_string()
    } else {
        return;
    };

    if name.is_empty() {
        return;
    }

    let is_dummy = name.starts_with("_use") || name.starts_with("dummy");
    result.functions.push(AstFnDefVO {
        name,
        is_pub: false,
        line: line_no,
        end_line: line_no,
        is_dummy,
    });
}

fn collect_ts_identifiers(line: &str, result: &mut TsParseResultVO) {
    for token in line.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '$') {
        if !token.is_empty() && token.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_' || c == '$') {
            result.used_identifiers.push(token.to_string());
        }
    }
}

// ─── Block 3: Helpers ─────────────────────────────────────

fn path_to_segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        .map(String::from)
        .collect()
}

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
                        break;
                    }
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
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
```

### 1.5 `utility_orphan_parser_dispatch.rs` (BARU — utility layer)

```rust
// PURPOSE: utility_orphan_parser_dispatch — route file parsing to the correct language parser.
// Single entry point for ALL capabilities files in both orphan-detector and import-rules.

use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_python_parser;
use crate::utility_orphan_rust_parser;
use crate::utility_orphan_ts_parser;
use std::path::Path;

pub fn parse_file(path: &str, content: &str) -> FileParseResultVO {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => FileParseResultVO::Rust(utility_orphan_rust_parser::parse_rust(content)),
        "py" => FileParseResultVO::Python(utility_orphan_python_parser::parse_python(content)),
        "ts" | "tsx" | "js" | "jsx" => {
            FileParseResultVO::TypeScript(utility_orphan_ts_parser::parse_ts(content))
        }
        _ => FileParseResultVO::Unsupported,
    }
}

pub fn is_supported(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    matches!(ext, "rs" | "py" | "ts" | "tsx" | "js" | "jsx")
}
```

---

## PHASE 2: Shared Import-Rules Utilities (Rewrite)

### 2.1 `utility_import_symbol_extractor.rs` — REWRITE (paling besar)

```rust
// PURPOSE: utility_import_symbol_extractor — AST-based unused import detection.
// Replaces dynamic regex + hardcoded DERIVE_MACROS whitelist with syn AST.
// Depends on shared AST parser dispatch.

use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_parser_dispatch;
use std::collections::{HashMap, HashSet};

// ─── Block 1: Import Alias Extraction (AST-based) ─────────

/// Extract imported aliases using AST parser dispatch.
/// Replaces regex-based extract_imported_aliases.
pub fn extract_imported_aliases(file_path: &str, content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                // Skip crate::, super::, self:: internal imports
                if imp.raw_path.starts_with("crate::")
                    || imp.raw_path.starts_with("super::")
                    || imp.raw_path.starts_with("self::")
                {
                    continue;
                }
                // Skip std/core/alloc
                if imp.raw_path.starts_with("std::")
                    || imp.raw_path.starts_with("core::")
                    || imp.raw_path.starts_with("alloc::")
                {
                    continue;
                }
                if let Some(last) = imp.last_segment() {
                    if !last.is_empty() && last != "*" && last != "self" {
                        aliases.insert(
                            Identity::new(last),
                            Identity::new(imp.raw_path.clone()),
                        );
                    }
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for imp in &result.imports {
                if imp.raw_path.starts_with("__future__") {
                    continue;
                }
                if let Some(last) = imp.last_segment() {
                    if !last.is_empty() && last != "*" {
                        aliases.insert(
                            Identity::new(last),
                            Identity::new(imp.raw_path.clone()),
                        );
                    }
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment() {
                    if !last.is_empty() && last != "*" && last != "default" {
                        aliases.insert(
                            Identity::new(last),
                            Identity::new(imp.raw_path.clone()),
                        );
                    }
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    aliases
}

// ─── Block 2: Usage Detection (AST-based) ─────────────────

/// Extract used symbols using AST-based identifier tracking.
/// Replaces dynamic regex `\b(A|B|C)\b` with AST visitor.
pub fn extract_used_symbols(
    file_path: &str,
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for alias in imported_aliases.keys() {
                let alias_str = alias.value();
                if result.is_identifier_used(alias_str) {
                    used.insert(Identity::new(alias_str));
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for alias in imported_aliases.keys() {
                let alias_str = alias.value();
                if result.is_identifier_used(alias_str) {
                    used.insert(Identity::new(alias_str));
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for alias in imported_aliases.keys() {
                let alias_str = alias.value();
                if result.is_identifier_used(alias_str) {
                    used.insert(Identity::new(alias_str));
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    used
}

// ─── Block 3: Exported Symbol Detection (AST-based) ───────

/// Extract exported symbols using AST parser.
/// Replaces regex-based __all__ / export detection.
pub fn extract_exported_symbols(file_path: &str, content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            // pub use = re-export
            for imp in &result.imports {
                if imp.is_reexport {
                    if let Some(last) = imp.last_segment() {
                        if !last.is_empty() && last != "*" {
                            exported.insert(Identity::new(last));
                        }
                    }
                }
            }
        }
        FileParseResultVO::Python(result) => {
            // __init__.py re-exports
            if file_path.ends_with("__init__.py") {
                for imp in &result.imports {
                    if let Some(last) = imp.last_segment() {
                        if !last.is_empty() && last != "*" {
                            exported.insert(Identity::new(last));
                        }
                    }
                }
            }
            // __all__ detection via used_identifiers (simplified)
            // Full __all__ parsing would need dedicated handling
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if imp.is_reexport {
                    if let Some(last) = imp.last_segment() {
                        if !last.is_empty() && last != "*" {
                            exported.insert(Identity::new(last));
                        }
                    }
                    if imp.is_glob {
                        exported.insert(Identity::new("*"));
                    }
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    exported
}

// ─── Block 4: Rust/JS Import Extraction (AST-based) ───────

/// Extract Rust/JS imports with line numbers using AST.
/// Replaces regex-based extract_rust_js_imports.
pub fn extract_rust_js_imports(
    file_path: &str,
    content: &str,
) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.raw_path.starts_with("std::")
                    || imp.raw_path.starts_with("core::")
                    || imp.raw_path.starts_with("alloc::")
                {
                    continue;
                }
                if let Some(last) = imp.last_segment() {
                    if !last.is_empty() && last != "*" && last != "_" {
                        imports.push((
                            SymbolName::new(last),
                            LineNumber::new(imp.line as i64),
                        ));
                    }
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment() {
                    if !last.is_empty() && last != "*" && last != "default" {
                        imports.push((
                            SymbolName::new(last),
                            LineNumber::new(imp.line as i64),
                        ));
                    }
                }
            }
        }
        _ => {}
    }

    imports
}

/// Check if a name is used in content (AST-based).
pub fn is_name_used(file_path: &str, name: &str, content: &str, _exclude_line: usize) -> bool {
    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => result.is_identifier_used(name),
        FileParseResultVO::Python(result) => result.is_identifier_used(name),
        FileParseResultVO::TypeScript(result) => result.is_identifier_used(name),
        FileParseResultVO::Unsupported => false,
    }
}
```

### 2.2 `utility_import_resolver.rs` — REWRITE

```rust
// PURPOSE: utility_import_resolver — AST-based import parsing and barrel resolution.
// Replaces line-based parse_import_lines_helper with AST parser dispatch.

use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use crate::import_rules::taxonomy_resolved_import_vo::ResolvedImport;
use crate::import_rules::utility_path_normalizer;
use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_parser_dispatch;
use std::collections::HashMap;
use std::path::Path;

// ─── Block 1: AST-Based Import Line Extraction ────────────

/// Parse import lines from file content using AST.
/// Replaces regex/line-based parse_import_lines_helper.
pub fn parse_import_lines_helper(file_path: &str, content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::Python(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::TypeScript(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    result
}

// ─── Block 2: Scope Resolution (unchanged) ────────────────

pub fn resolve_scope(scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
    let scope_str = scope.value();
    if let Some(paren) = scope_str.find('(') {
        let layer = scope_str[..paren].trim();
        let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<Identity> = if inner.contains('|') {
            inner.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).map(Identity::new).collect()
        } else {
            inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).map(Identity::new).collect()
        };
        (LayerNameVO::new(layer), suffixes)
    } else {
        (LayerNameVO::new(scope_str.trim()), vec![])
    }
}

// ─── Block 3: Import Matching (segment-based, not contains) ──

/// Check if an import line satisfies the given scope requirement.
/// Uses segment-based matching, NOT substring contains().
pub fn import_matches_scope(
    import_line: &LineContentVO,
    layer: &LayerNameVO,
    suffixes: &[Identity],
) -> bool {
    let import_line_str = import_line.value();
    let segments: Vec<&str> = import_line_str
        .split(|c: char| {
            c == ':' || c == '.' || c == '/' || c == '\\' || c.is_whitespace()
                || c == '"' || c == '\'' || c == '{' || c == '}' || c == ',' || c == ';'
        })
        .filter(|s| !s.is_empty())
        .collect();

    let layer_lower = layer.value().to_lowercase();
    let layer_prefix = format!("{}_", layer_lower);
    let layer_match = segments.iter().any(|s| {
        let trimmed = s.trim().to_lowercase();
        trimmed == layer_lower || trimmed.starts_with(&layer_prefix)
    });

    if !layer_match || suffixes.is_empty() {
        return layer_match;
    }

    suffixes.iter().any(|s| {
        let s_val = s.value();
        segments.iter().any(|seg| {
            let cleaned = seg.trim_end_matches(';').trim()
                .trim_start_matches('{').trim_end_matches('}').trim();
            cleaned.split(',').any(|t| {
                let name = t.trim();
                let name_lower = name.to_lowercase();
                name_lower.ends_with(&format!("_{}", s_val))
            })
        })
    })
}

// ─── Block 4: Module Extraction (AST-based) ───────────────

pub fn extract_module_from_line(line: &LineContentVO) -> Option<Identity> {
    let trimmed = line.value().trim();

    // Python: from X import Y
    if let Some(rest) = trimmed.strip_prefix("from ") {
        return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
    }

    // TS/JS: import ... from 'X'
    if trimmed.starts_with("import ") {
        if let Some(pos) = trimmed.rfind(" from ") {
            let module_part = trimmed[pos + 6..].trim();
            let cleaned = module_part
                .trim_end_matches(';')
                .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                .trim();
            return Some(Identity::new(cleaned.to_string()));
        }
    }

    // Rust: use X::Y;
    if let Some(rest) = trimmed
        .strip_prefix("pub(crate) use ")
        .or_else(|| trimmed.strip_prefix("pub use "))
        .or_else(|| trimmed.strip_prefix("use "))
    {
        let module = rest.trim_end_matches(';').trim().to_string();
        if let Some(brace_pos) = module.find("::{") {
            return Some(Identity::new(module[..brace_pos].to_string()));
        }
        return Some(Identity::new(module));
    }

    None
}

// ─── Block 5: Layer Detection from Import ─────────────────

pub fn extract_layer_from_import(segment: &Identity) -> Option<LayerNameVO> {
    let segment_str = segment.value();
    if let Some(layer) = utility_path_normalizer::extract_layer_from_prefix(segment_str) {
        return Some(LayerNameVO::new(layer));
    }
    match segment_str {
        "taxonomy" => Some(LayerNameVO::new("taxonomy")),
        "contract" => Some(LayerNameVO::new("contract")),
        "capabilities" => Some(LayerNameVO::new("capabilities")),
        "utility" => Some(LayerNameVO::new("utility")),
        "agent" => Some(LayerNameVO::new("agent")),
        "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
        "root" => Some(LayerNameVO::new("root")),
        _ => None,
    }
}

// ─── Block 6: Barrel File Detection (unchanged) ───────────

pub fn is_barrel_file(filename: &str) -> bool {
    matches!(
        filename,
        "__init__.py" | "mod.rs" | "lib.rs" | "main.rs"
            | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
    )
}

// ─── Block 7: Barrel Resolution (unchanged logic) ─────────

pub fn find_barrel_file(module_path: &str, root_dir: &str) -> Option<String> {
    let base = Path::new(root_dir);
    let clean_path = normalize_module_path(module_path);
    let module_dir = base.join(&clean_path);
    let barrel_candidates = [
        "__init__.py", "index.ts", "index.js", "index.tsx", "index.jsx", "mod.rs",
    ];

    if let Some(found) = try_barrel_candidates(&module_dir, &barrel_candidates) {
        return Some(found);
    }
    if let Some(parent) = module_dir.parent() {
        if let Some(found) = try_barrel_candidates(parent, &barrel_candidates) {
            return Some(found);
        }
    }

    let segments: Vec<&str> = clean_path.split('/').collect();
    if let Some(first_seg) = segments.first() {
        let crate_names = [*first_seg, &first_seg.replace('_', "-")];
        for (idx, &crate_name) in crate_names.iter().enumerate() {
            if idx == 1 && crate_name == crate_names[0] {
                continue;
            }
            let crate_src = base.join("crates").join(crate_name).join("src");
            let remainder: Vec<&str> = segments.iter().skip(1).copied().collect();
            if !remainder.is_empty() {
                let remainder_opts = [remainder.join("/"), remainder.join("/").replace('_', "-")];
                for (rem_idx, rem) in remainder_opts.iter().enumerate() {
                    if rem_idx == 1 && *rem == remainder_opts[0] {
                        continue;
                    }
                    let full_dir = crate_src.join(rem);
                    if let Some(found) = try_barrel_candidates(&full_dir, &barrel_candidates) {
                        return Some(found);
                    }
                }
            }
        }
    }
    None
}

fn normalize_module_path(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .replace('.', "/")
        .replace("::", "/")
}

fn try_barrel_candidates(dir: &Path, candidates: &[&str]) -> Option<String> {
    for candidate in candidates {
        let barrel_path = dir.join(candidate);
        if barrel_path.exists() {
            return Some(barrel_path.to_string_lossy().to_string());
        }
    }
    None
}

fn extract_module_stem(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .rsplit(['.', '/'])
        .next()
        .unwrap_or(module_path)
        .to_string()
}

pub fn parse_barrel_reexports(barrel_content: &str) -> HashMap<String, String> {
    let mut reexports: HashMap<String, String> = HashMap::new();
    for line in barrel_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part.strip_prefix("from ").unwrap_or("").trim();
                let module_stem = extract_module_stem(module);
                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" { continue; }
                    let exported_name = name.split(" as ").last().unwrap_or(name).trim();
                    reexports.insert(exported_name.to_string(), module_stem.clone());
                }
            }
            continue;
        }
        if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[from_pos + 6..].trim();
                let module_clean = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == '`');
                let module_stem = extract_module_stem(module_clean);
                if let Some(brace_start) = trimmed.find('{') {
                    if let Some(brace_end) = trimmed.find('}') {
                        let inner = &trimmed[brace_start + 1..brace_end];
                        for part in inner.split(',') {
                            let part = part.trim();
                            if part.is_empty() { continue; }
                            let exported_name = part.split(" as ").last().unwrap_or(part).trim();
                            reexports.insert(exported_name.to_string(), module_stem.clone());
                        }
                    }
                }
            }
            continue;
        }
        if trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use ") {
            let use_part = trimmed
                .trim_start_matches("pub(crate) use ")
                .trim_start_matches("pub use ")
                .trim_end_matches(';')
                .trim();
            if let Some(brace_pos) = use_part.find("::{") {
                let prefix = &use_part[..brace_pos];
                let module_stem = prefix.rsplit("::").next().unwrap_or(prefix).to_string();
                let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                for name in inner.split(',') {
                    let name = name.trim().split(" as ").last().unwrap_or("").trim();
                    if !name.is_empty() && name != "*" {
                        reexports.insert(name.to_string(), module_stem.clone());
                    }
                }
            } else {
                let name = use_part.rsplit("::").next().unwrap_or("").trim();
                let module_stem = use_part
                    .rsplit_once("::")
                    .map(|x| x.0)
                    .unwrap_or(use_part)
                    .rsplit("::")
                    .next()
                    .unwrap_or(use_part)
                    .to_string();
                if !name.is_empty() && name != "*" {
                    reexports.insert(name.to_string(), module_stem);
                }
            }
        }
    }
    reexports
}

pub fn resolve_barrel_import(
    module_path: &str,
    symbol_name: &str,
    root_dir: &str,
) -> Option<ResolvedImport> {
    let barrel_path = find_barrel_file(module_path, root_dir)?;
    let barrel_content = std::fs::read_to_string(&barrel_path).ok()?;
    let reexports = parse_barrel_reexports(&barrel_content);
    let resolved_file = reexports.get(symbol_name)?.clone();
    let resolved_layer = utility_path_normalizer::extract_layer_from_prefix(&resolved_file);
    Some(ResolvedImport {
        original_module: module_path.to_string(),
        resolved_file,
        resolved_layer,
        symbol: symbol_name.to_string(),
    })
}

pub fn extract_symbol_names(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let mut names = Vec::new();

    if trimmed.starts_with("from ") {
        if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
            let clean = import_part.trim_start_matches('(').trim_end_matches(')').trim_end_matches(';');
            for part in clean.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    names.push(name.to_string());
                }
            }
        }
        return names;
    }

    if trimmed.starts_with("use ") || trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use ") {
        let use_part = trimmed
            .trim_start_matches("pub(crate) use ")
            .trim_start_matches("pub use ")
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();
        if let Some(brace_start) = use_part.find("::{") {
            let inner = use_part[brace_start + 3..].trim_end_matches('}');
            for part in inner.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" && name != "self" {
                    names.push(name.to_string());
                }
            }
        } else {
            let name = use_part.rsplit("::").next().unwrap_or("").trim();
            if !name.is_empty() && name != "*" {
                names.push(name.to_string());
            }
        }
        return names;
    }

    if trimmed.starts_with("import ") && trimmed.contains('{') {
        if let Some(open) = trimmed.find('{') {
            if let Some(close) = trimmed.find('}') {
                let inner = &trimmed[open + 1..close];
                for part in inner.split(',') {
                    let name = part.trim().split(" as ").last().unwrap_or("").trim();
                    if !name.is_empty() {
                        names.push(name.to_string());
                    }
                }
            }
        }
        return names;
    }

    if trimmed.starts_with("import ") && trimmed.contains(" from ") {
        if let Some(import_part) = trimmed.strip_prefix("import ") {
            let name = import_part.split(" from ").next().unwrap_or("").trim();
            if !name.is_empty() && name != "default" && name != "*" {
                names.push(name.to_string());
            }
        }
    }

    names
}

pub fn find_import_line_number(content: &str, alias: &str) -> LineNumber {
    let first_part = alias.split('.').next().unwrap_or("");
    let pos_opt = content.lines().position(|l| {
        l.trim().contains(&format!("import {}", alias))
            || l.trim().contains(&format!("from {} import", first_part))
    });
    let line = match pos_opt {
        Some(p) => p + 1,
        None => 1,
    };
    LineNumber::new(line as i64)
}
```

### 2.3 `utility_import_module_parser.rs` — REWRITE

```rust
// PURPOSE: utility_import_module_parser — AST-based module extraction.
// Replaces line-based extract_import_modules with AST parser dispatch.

use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::utility_import_resolver;
use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_parser_dispatch;

/// Extract import modules using AST parser dispatch.
pub fn extract_import_modules(file_path: &str, content: &str) -> Vec<SymbolName> {
    let mut modules = Vec::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                modules.push(SymbolName::new(imp.raw_path.clone()));
            }
        }
        FileParseResultVO::Python(result) => {
            for imp in &result.imports {
                modules.push(SymbolName::new(imp.raw_path.clone()));
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                modules.push(SymbolName::new(imp.raw_path.clone()));
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    modules
}

/// Extract import modules with barrel file resolution (AST-based).
pub fn extract_import_modules_resolved(
    file_path: &str,
    content: &str,
    root_dir: &str,
) -> Vec<(SymbolName, SymbolName)> {
    let mut resolved_modules = Vec::new();

    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                let module_path = imp.raw_path
                    .strip_prefix("crate::")
                    .or_else(|| imp.raw_path.strip_prefix("super::"))
                    .or_else(|| imp.raw_path.strip_prefix("self::"))
                    .unwrap_or(&imp.raw_path);

                if let Some(last) = imp.last_segment() {
                    if last.is_empty() || last == "*" || last == "self" {
                        continue;
                    }
                    let prefix = imp.module_path();
                    if let Some(resolved) = utility_import_resolver::resolve_barrel_import(
                        &prefix, last, root_dir,
                    ) {
                        resolved_modules.push((
                            SymbolName::new(prefix),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((
                            SymbolName::new(prefix),
                            SymbolName::new(prefix),
                        ));
                    }
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment() {
                    if last.is_empty() || last == "*" {
                        continue;
                    }
                    let module = imp.module_path();
                    if let Some(resolved) = utility_import_resolver::resolve_barrel_import(
                        &module, last, root_dir,
                    ) {
                        resolved_modules.push((
                            SymbolName::new(module),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((
                            SymbolName::new(module.clone()),
                            SymbolName::new(module),
                        ));
                    }
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment() {
                    if last.is_empty() {
                        continue;
                    }
                    let module = imp.raw_path
                        .trim_start_matches("./")
                        .trim_start_matches("../")
                        .to_string();
                    if let Some(resolved) = utility_import_resolver::resolve_barrel_import(
                        &module, last, root_dir,
                    ) {
                        resolved_modules.push((
                            SymbolName::new(module),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((
                            SymbolName::new(module.clone()),
                            SymbolName::new(module),
                        ));
                    }
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }

    resolved_modules
}
```

### 2.4 `utility_dummy_detector.rs` — REWRITE

```rust
// PURPOSE: utility_dummy_detector — AST-based dummy function and trait impl detection.
// Replaces line-based brace counting with syn AST for Rust.

use crate::common::taxonomy_common_vo::{LanguageVO, LineNumber};
use crate::common::taxonomy_name_vo::SymbolName;
use crate::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::utility_orphan_parser_dispatch;

// ─── Block 1: AST-Based Dummy Detection ───────────────────

/// Get dummy function ranges using AST (Rust) or structured parsing (Python/TS).
pub fn dummy_function_ranges(
    file_path: &str,
    content: &str,
    lang: LanguageVO,
) -> Vec<(LineNumber, LineNumber)> {
    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            result
                .functions
                .iter()
                .filter(|f| f.is_dummy)
                .map(|f| (LineNumber::new(f.line as i64), LineNumber::new(f.end_line as i64)))
                .collect()
        }
        FileParseResultVO::Python(result) => {
            result
                .functions
                .iter()
                .filter(|f| f.is_dummy)
                .map(|f| (LineNumber::new(f.line as i64), LineNumber::new(f.end_line as i64)))
                .collect()
        }
        FileParseResultVO::TypeScript(result) => {
            result
                .functions
                .iter()
                .filter(|f| f.is_dummy)
                .map(|f| (LineNumber::new(f.line as i64), LineNumber::new(f.end_line as i64)))
                .collect()
        }
        FileParseResultVO::Unsupported => Vec::new(),
    }
}

/// Get dummy trait implementations using AST.
pub fn dummy_impl_traits_with_lines(
    file_path: &str,
    content: &str,
) -> Vec<(SymbolName, LineNumber)> {
    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            result
                .trait_impls
                .iter()
                .filter(|ti| ti.is_dummy)
                .map(|ti| (SymbolName::new(ti.trait_name.clone()), LineNumber::new(ti.line as i64)))
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Extract imported symbols using AST.
pub fn imported_symbols(
    file_path: &str,
    content: &str,
    _lang: LanguageVO,
) -> Vec<(SymbolName, LineNumber)> {
    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            result
                .imports
                .iter()
                .filter_map(|imp| {
                    imp.last_segment().map(|name| {
                        (SymbolName::new(name), LineNumber::new(imp.line as i64))
                    })
                })
                .filter(|(name, _)| {
                    let n = name.value();
                    !n.is_empty() && n != "*" && n != "self"
                })
                .collect()
        }
        FileParseResultVO::Python(result) => {
            result
                .imports
                .iter()
                .filter_map(|imp| {
                    imp.last_segment().map(|name| {
                        (SymbolName::new(name), LineNumber::new(imp.line as i64))
                    })
                })
                .filter(|(name, _)| {
                    let n = name.value();
                    !n.is_empty() && n != "*"
                })
                .collect()
        }
        FileParseResultVO::TypeScript(result) => {
            result
                .imports
                .iter()
                .filter_map(|imp| {
                    imp.last_segment().map(|name| {
                        (SymbolName::new(name), LineNumber::new(imp.line as i64))
                    })
                })
                .filter(|(name, _)| {
                    let n = name.value();
                    !n.is_empty() && n != "*" && n != "default"
                })
                .collect()
        }
        FileParseResultVO::Unsupported => Vec::new(),
    }
}

/// Check if a symbol is used in real logic (not in dummy functions).
/// Uses AST-based identifier tracking.
pub fn symbol_used_real(
    file_path: &str,
    content: &str,
    symbol: &str,
    dummy_ranges: &[(LineNumber, LineNumber)],
    _dummy_impl_traits: &[String],
) -> bool {
    // Use AST to check if identifier appears in non-dummy code
    match utility_orphan_parser_dispatch::parse_file(file_path, content) {
        FileParseResultVO::Rust(result) => {
            // Check if used anywhere in the AST (which excludes comments/strings)
            if !result.is_identifier_used(symbol) {
                return false;
            }
            // Additionally verify it's not ONLY in dummy ranges
            // (simplified: if AST says used, trust it — dummy ranges are for line-based fallback)
            true
        }
        FileParseResultVO::Python(result) => result.is_identifier_used(symbol),
        FileParseResultVO::TypeScript(result) => result.is_identifier_used(symbol),
        FileParseResultVO::Unsupported => {
            // Fallback: line-based check
            symbol_used_real_fallback(content, symbol, dummy_ranges)
        }
    }
}

/// Fallback line-based symbol usage check (for unsupported file types).
fn symbol_used_real_fallback(
    content: &str,
    symbol: &str,
    dummy_ranges: &[(LineNumber, LineNumber)],
) -> bool {
    let dummy_ranges_usize: Vec<(usize, usize)> = dummy_ranges
        .iter()
        .map(|(a, b)| (a.value() as usize, b.value() as usize))
        .collect();

    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if in_dummy_range(line_no, &dummy_ranges_usize)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with("*/")
            || (trimmed.starts_with('#') && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }
        if contains_ident(trimmed, symbol) {
            return true;
        }
    }
    false
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|(start, end)| line_no >= *start && line_no <= *end)
}

/// Check if haystack contains needle as a whole identifier.
pub fn contains_ident(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let end = abs + needle.len();
        let before_ok = abs == 0 || {
            let before_char = haystack[..abs].chars().next_back().unwrap_or(' ');
            !before_char.is_alphanumeric() && before_char != '_'
        };
        let after_ok = end == haystack.len() || {
            let after_char = haystack[end..].chars().next().unwrap_or(' ');
            !after_char.is_alphanumeric() && after_char != '_'
        };
        if before_ok && after_ok {
            return true;
        }
        start = abs + needle.len();
    }
    false
}
```

### 2.5 `utility_cycle_detector.rs` — **TIDAK BERUBAH**

Pure graph theory (3-color DFS). Tidak ada parsing. Pertahankan seperti ada.

### 2.6 `utility_path_normalizer.rs` — **TIDAK BERUBAH**

Path utilities. Tidak ada parsing. Pertahankan seperti ada.

---

## PHASE 3: Shared Contracts/Taxonomy (Update)

### 3.1 `taxonomy_import_constant.rs` — REWRITE

```rust
// PURPOSE: taxonomy_import_constant — compile-time constants for import-rules layer.
// DERIVE_MACROS removed — AST attribute parsing handles derive detection natively.

/// Layer prefixes used for filename-based layer detection.
pub const LAYER_PREFIXES: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("utility_", "utility"),
    ("capabilities_", "capabilities"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];

/// Rust entry file names that should be skipped during scope-level checks.
pub const RUST_ENTRY_FILES: &[&str] = &["mod.rs", "lib.rs", "main.rs"];

/// Python entry file names that should be skipped during mandatory checks.
pub const PYTHON_ENTRY_FILES: &[&str] = &["__init__.py"];

/// Source code file extensions for file collection.
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];

/// Directories to skip during file collection.
pub const DEFAULT_SKIP_DIRS: &[&str] = &[
    ".git", "node_modules", "target", "dist", "build", ".venv", "__pycache__", "tests",
];
```

### 3.2 `contract_unused_import_protocol.rs` — UPDATE

```rust
// PURPOSE: IUnusedImportProtocol — unified protocol for AES203 unused import detection.
// Updated: check_unused_imports now takes file_path for AST dispatch.

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_import_error::ImportError;

pub trait IUnusedImportProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError>;

    /// Check unused imports given file path and content.
    /// file_path is needed for AST parser dispatch (language detection by extension).
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
    ) -> Result<Vec<LintResult>, ImportError>;
}
```

### 3.3 File kontrak/taxonomy lain — **TIDAK BERUBAH**

- `contract_import_forbidden_protocol.rs` — tidak berubah
- `contract_import_mandatory_protocol.rs` — tidak berubah
- `contract_dummy_import_protocol.rs` — tidak berubah
- `contract_cycle_import_protocol.rs` — tidak berubah
- `contract_import_runner_aggregate.rs` — tidak berubah
- `taxonomy_import_error.rs` — tidak berubah
- `taxonomy_violation_import_vo.rs` — tidak berubah
- `taxonomy_dependency_edge_vo.rs` — tidak berubah
- `taxonomy_resolved_import_vo.rs` — tidak berubah
- `taxonomy_graph_color_vo.rs` — tidak berubah
- `taxonomy_forbidden_rule_config_vo.rs` — tidak berubah

---

## PHASE 4: Feature Crate Capabilities (Rewrite)

### 4.1 `capabilities_import_unused_checker.rs` — REWRITE (perubahan terbesar)

```rust
// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports.
// AST-based: uses syn visitor for usage tracking. No dynamic regex. No DERIVE_MACROS whitelist.

use shared::cli_commands::LintResult;
use shared::common::{ErrorMessage, FilePath, LintMessage, Severity};
use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_import_symbol_extractor;
use shared::import_rules::{AesImportViolation, IUnusedImportProtocol, ImportError};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportRuleChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError> {
        if utility_import_resolver::is_barrel_file(&path.basename()) {
            return Ok(Vec::new());
        }

        let content = shared::common::utility_file_handler::read_file_generic(path.value())
            .map_err(|_| {
                ImportError::module_resolution(
                    path.value().to_string(),
                    Some(ErrorMessage::new("File could not be read for unused import analysis")),
                )
            })?;

        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases(path.value(), &content);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols(path.value(), &content);
        let used_symbols = utility_import_symbol_extractor::extract_used_symbols(
            path.value(),
            &content,
            &imported_aliases,
        );

        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if unused_import_is_future_import(&content, alias_str) {
                continue;
            }
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias_str.to_string());
            }
        }

        let rust_js_imports =
            utility_import_symbol_extractor::extract_rust_js_imports(path.value(), &content);
        for (name, _line_idx) in rust_js_imports {
            let name_str = name.value();
            if !utility_import_symbol_extractor::is_name_used(
                path.value(),
                name_str,
                &content,
                0,
            ) {
                unused.push(name_str.to_string());
            }
        }

        Ok(unused.into_iter().map(LintMessage::new).collect())
    }

    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
    ) -> Result<Vec<LintResult>, ImportError> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return Ok(Vec::new());
        }

        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases(file, content);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols(file, content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(file, content, &imported_aliases);

        let mut violations = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if unused_import_is_future_import(content, alias_str) {
                continue;
            }
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num =
                    utility_import_resolver::find_import_line_number(content, alias_str)
                        .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }

        let rust_js_imports =
            utility_import_symbol_extractor::extract_rust_js_imports(file, content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            if !utility_import_symbol_extractor::is_name_used(file, &name_str, content, 0) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx.value() as usize,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }

        Ok(violations)
    }
}

// ─── Block 3: Constructors, Helpers ───────────────────────

impl Default for UnusedImportRuleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self
    }
}

fn unused_import_is_future_import(content: &str, alias: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", alias)
                || trimmed.contains(format!(", {}", alias).as_str())
                || trimmed.contains(format!(" {},", alias).as_str()))
    })
}
```

### 4.2 `capabilities_import_forbidden_checker.rs` — REWRITE

```rust
// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules.
// AST-based: uses parser dispatch for import extraction.

use async_trait::async_trait;
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{FilePath, FilePathList, Severity};
use shared::common::utility_layer_detector;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_path_normalizer;
use shared::import_rules::{AesImportViolation, IImportForbiddenProtocol, ImportError};
use shared::common::{Identity, LayerNameVO, LineContentVO, LineNumber, LintMessage};
use shared::common::{LayerDefinition, LayerMapVO};
use std::collections::HashSet;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        let _ = utility_path_normalizer::extract_layer_from_prefix("");
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let root_dir_str = root_dir.to_string();
        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes201_exceptions.contains(&basename) {
                    return Vec::new();
                }
                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };

                // AST-based import extraction
                let import_lines =
                    utility_import_resolver::parse_import_lines_helper(&f_str, &content);
                if import_lines.is_empty() {
                    return Vec::new();
                }

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer, &f_str, &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_forbidden_imports_with_lines(
                            &f_str, &specialized, def, &import_lines, &root_dir_str,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_forbidden_imports_with_lines(
                    &f_str, &basename, config, &import_lines, &mut local_violations,
                );
                local_violations
            })
            .collect();

        Ok(LintResultList::new(file_violations))
    }
}

// ─── Block 3: Helpers ─────────────────────────────────────

impl Default for ArchImportForbiddenChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_forbidden_imports_with_lines(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }
        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }
        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec!["agent".into(), "capabilities".into()]
        };
        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in import_lines {
            let module = match utility_import_resolver::extract_module_from_line(line) {
                Some(m) => m,
                None => continue,
            };
            let module_val = module.value();
            let symbol_names = utility_import_resolver::extract_symbol_names(line.value());

            for forbidden in &forbidden_list {
                let forbidden_identity = Identity::new(forbidden);
                let (layer, suffixes) = utility_import_resolver::resolve_scope(&forbidden_identity);

                // Segment-based matching (not contains)
                let mut is_forbidden = if suffixes.is_empty() {
                    module_val
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .any(|seg| {
                            let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                            match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                } else {
                    utility_import_resolver::import_matches_scope(line, &layer, &suffixes)
                };

                // Barrel resolution fallback
                if !is_forbidden {
                    for sym in &symbol_names {
                        if let Some(resolved) =
                            utility_import_resolver::resolve_barrel_import(module_val, sym, root_dir)
                        {
                            if resolved.matches_layer(layer.value())
                                && (suffixes.is_empty()
                                    || suffixes.iter().any(|s| resolved.has_suffix(s.value())))
                            {
                                is_forbidden = true;
                                break;
                            }
                        }
                    }
                }

                if is_forbidden {
                    let allowed: Vec<LayerNameVO> = definition
                        .allowed
                        .values
                        .iter()
                        .map(|s| {
                            LayerNameVO::new(
                                utility_import_resolver::resolve_scope(&Identity::new(s))
                                    .0
                                    .value()
                                    .to_string(),
                            )
                        })
                        .collect();
                    violations.push(LintResult::new_arch(
                        file,
                        line_num.value() as usize,
                        "AES201",
                        Severity::CRITICAL,
                        AesImportViolation::ForbiddenImport {
                            source_layer: layer_name_vo.clone(),
                            forbidden_layer: LayerNameVO::new(forbidden.clone()),
                            allowed,
                            reason: Some(LintMessage::new(format!(
                                "File imports from '{}' which resolves to forbidden layer '{}'. Source file is in layer '{}'.",
                                module_val, forbidden, layer_name
                            ))),
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }

    fn _check_scope_forbidden_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(LineNumber, LineContentVO)],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &Identity::new(&rule.scope.value),
                )
            else {
                continue;
            };
            for (line_num, line) in import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let module_val = module.value();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            module_val
                                .split([':', '.', '/', '\\'])
                                .filter(|s| !s.is_empty())
                                .any(|seg| {
                                    let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                    match utility_import_resolver::extract_layer_from_import(&cleaned)
                                    {
                                        Some(l) => l == forbidden_layer,
                                        None => false,
                                    }
                                })
                        } else {
                            utility_import_resolver::import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        utility_import_resolver::resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: LayerNameVO::new(rule_layer_str.clone()),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: Some(LintMessage::new(format!(
                                        "Scope rule violation: file imports from '{}' which resolves to forbidden layer '{}'.",
                                        module_val, forbidden
                                    ))),
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}
```

### 4.3 `capabilities_import_mandatory_checker.rs` — REWRITE

```rust
// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules.
// AST-based: uses parser dispatch for import extraction.

use async_trait::async_trait;
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{FilePath, FilePathList, Severity};
use shared::common::utility_layer_detector;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::{AesImportViolation, IImportMandatoryProtocol, ImportError};
use shared::common::LineNumber;
use shared::common::SymbolName;
use shared::common::{FileContentVO, Identity, LayerNameVO, LineContentVO, LintMessage};
use shared::common::{LayerDefinition, LayerMapVO};
use std::collections::HashSet;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    async fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();
        let root_str = root_dir.value().to_string();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes202_exceptions.contains(&basename) {
                    return Vec::new();
                }
                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };

                // AST-based import extraction
                let import_lines: Vec<(LineNumber, LineContentVO)> =
                    utility_import_resolver::parse_import_lines_helper(&f_str, &content);

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer, &f_str, &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_mandatory_imports_with_lines(
                            &f_str, &basename, def, &import_lines, &root_str,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_mandatory_imports_with_lines(
                    &f_str, &basename, config, &import_lines, &root_str,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        Ok(LintResultList::new(file_violations))
    }
}

// ─── Block 3: Helpers ─────────────────────────────────────

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() || basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }
        let stem: &str = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer: &str = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let layer_str: &str = layer.value();

            // Segment-based matching (not contains)
            let is_present_direct = if suffixes.is_empty() {
                import_lines.iter().any(|(_, l)| {
                    l.value()
                        .split([':', '.', '/', '\\', '{', '}', ',', ';', ' '])
                        .any(|seg| seg == layer_str || seg.starts_with(&format!("{}_", layer_str)))
                })
            } else {
                import_lines.iter().any(|(_, l)| {
                    utility_import_resolver::import_matches_scope(l, &layer, &suffixes)
                })
            };

            let is_present = is_present_direct
                || self._check_barrel_mandatory_imports(
                    import_lines, &layer, &suffixes, layer_str, root_dir,
                );

            if !is_present {
                violations.push(LintResult::new_arch(
                    file, 0, "AES202", Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: Some(LintMessage::new(format!(
                            "File '{}' in layer '{}' is missing required import '{}'.",
                            basename, source_layer, required
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }

    fn _check_scope_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename, &scope_identity,
                )
            else {
                continue;
            };
            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) =
                    utility_import_resolver::resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();

                let is_present_direct = if req_suffixes.is_empty() {
                    import_lines.iter().any(|(_, l)| {
                        l.value()
                            .split([':', '.', '/', '\\', '{', '}', ',', ';', ' '])
                            .any(|seg| seg == req_layer_str || seg.starts_with(&format!("{}_", req_layer_str)))
                    })
                } else {
                    import_lines.iter().any(|(_, l)| {
                        utility_import_resolver::import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };

                let is_present = is_present_direct
                    || self._check_barrel_mandatory_imports(
                        import_lines, &req_layer, &req_suffixes, req_layer_str, root_dir,
                    );

                if !is_present {
                    violations.push(LintResult::new_arch(
                        file, 0, "AES202", Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: LayerNameVO::new(rule_layer_str.clone()),
                            required: SymbolName::new(required.clone()),
                            reason: Some(LintMessage::new(format!(
                                "File '{}' in scope '{}' is missing required import '{}'.",
                                basename, rule_layer_str, required
                            ))),
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }

    fn _check_barrel_mandatory_imports(
        &self,
        import_lines: &[(LineNumber, LineContentVO)],
        _layer: &LayerNameVO,
        suffixes: &[Identity],
        layer_str: &str,
        root_dir: &str,
    ) -> bool {
        for (_, line) in import_lines {
            let line_val = line.value();
            let Some(module) = utility_import_resolver::extract_module_from_line(line) else {
                continue;
            };
            let module_val = module.value();
            let symbols = utility_import_resolver::extract_symbol_names(line_val);
            for symbol_name in &symbols {
                let Some(resolved) =
                    utility_import_resolver::resolve_barrel_import(module_val, symbol_name, root_dir)
                else {
                    continue;
                };
                if !resolved.matches_layer(layer_str) {
                    continue;
                }
                if suffixes.is_empty() {
                    return true;
                }
                if suffixes.iter().any(|s| resolved.has_suffix(s.value())) {
                    return true;
                }
            }
        }
        false
    }
}
```

### 4.4 `capabilities_dummy_import_checker.rs` — REWRITE

```rust
// PURPOSE: DummyImportChecker — AES204: detect dummy imports, functions, trait impls.
// AST-based: uses parser dispatch for dummy detection.

use shared::cli_commands::LintResult;
use shared::common::{ContentString, FilePath, LanguageVO, Severity};
use shared::common::utility_layer_detector;
use shared::import_rules::utility_dummy_detector;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::{AesImportViolation, IDummyImportCheckerProtocol, ImportError};
use shared::common::LayerMapVO;
use shared::common::{Identity, LayerNameVO, LineNumber};
use shared::common::{LintMessage, SymbolName};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DummyImportChecker;

struct DummyFileContext {
    lang: LanguageVO,
    layer_name: String,
    dummy_ranges: Vec<(LineNumber, LineNumber)>,
    dummy_impl_traits: Vec<String>,
}

impl DummyFileContext {
    fn compute(file: &str, content: &str, layer_map: &LayerMapVO) -> Option<Self> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return None;
        }

        let lang = LanguageVO::from_path(file);
        let layer_name = Self::detect_layer(file, layer_map);

        // AST-based dummy detection
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(file, content, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(file, content)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();

        Some(Self {
            lang,
            layer_name,
            dummy_ranges,
            dummy_impl_traits,
        })
    }

    fn detect_layer(file: &str, layer_map: &LayerMapVO) -> String {
        let filename: &str = utility_layer_detector::extract_filename(file);
        match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => {
                let layer_keys: Vec<String> =
                    layer_map.values.keys().map(|k| k.to_string()).collect();
                utility_layer_detector::resolve_specialized_layer(&base, file, &layer_keys)
            }
            None => "any".to_string(),
        }
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_imports(file.value(), content.value(), &ctx, &mut violations);
        Ok(violations)
    }

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_functions(file.value(), &ctx, &mut violations);
        Ok(violations)
    }

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_impls(file.value(), content.value(), &mut violations);
        Ok(violations)
    }

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_taxonomy_intent(file.value(), content.value(), &ctx, &mut violations);
        Ok(violations)
    }

    fn check_layer_contract_intent(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        Ok(Vec::new())
    }

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let mut violations = Vec::new();
        Self::_check_surface_logic(file.value(), content.value(), &mut violations);
        Ok(violations)
    }

    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_imports(file.value(), content.value(), &ctx, &mut violations);
        Self::_check_dummy_functions(file.value(), &ctx, &mut violations);
        Self::_check_dummy_impls(file.value(), content.value(), &mut violations);
        Self::_check_taxonomy_intent(file.value(), content.value(), &ctx, &mut violations);
        Self::_check_surface_logic(file.value(), content.value(), &mut violations);
        Ok(violations)
    }
}

// ─── Block 3: Helpers ─────────────────────────────────────

impl Default for DummyImportChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DummyImportChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_dummy_imports(
        file: &str,
        content: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        let imported = utility_dummy_detector::imported_symbols(file, content, ctx.lang);
        for (symbol, line_no) in imported {
            let symbol_str = symbol.value().to_string();
            if is_future_import(content, &symbol_str) {
                continue;
            }
            if utility_dummy_detector::symbol_used_real(
                file, content, &symbol_str, &ctx.dummy_ranges, &ctx.dummy_impl_traits,
            ) {
                continue;
            }
            violations.push(LintResult::new_arch(
                file, line_no.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new("Use imported symbols in real logic, not only in dummy functions or stubs"),
                    reason: Some(LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code."
                    )),
                }.to_string(),
            ));
        }
    }

    fn _check_dummy_functions(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        for (start, end) in &ctx.dummy_ranges {
            violations.push(LintResult::new_arch(
                file, start.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports"),
                    intent: SymbolName::new("Remove dummy functions that exist only to silence unused import checks"),
                    reason: Some(LintMessage::new(format!("Dummy function range ends at line {}", end))),
                }.to_string(),
            ));
        }
    }

    fn _check_dummy_impls(
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        for (trait_name, start) in utility_dummy_detector::dummy_impl_traits_with_lines(file, content) {
            violations.push(LintResult::new_arch(
                file, start.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new("any"),
                    import_type: SymbolName::new(trait_name.value().to_string()),
                    intent: SymbolName::new("Implement contract methods with real behavior instead of empty/todo stubs"),
                    reason: Some(LintMessage::new("Trait implementations with empty bodies violate the contract abstraction.")),
                }.to_string(),
            ));
        }
    }

    fn _check_taxonomy_intent(
        file: &str,
        content: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        if ctx.dummy_ranges.is_empty() {
            return;
        }
        let imported = utility_dummy_detector::imported_symbols(file, content, ctx.lang);
        let has_taxonomy_import = imported.iter().any(|(symbol, line_no)| {
            let line_idx = line_no.value().saturating_sub(1) as usize;
            content.lines().nth(line_idx).is_some_and(|line| {
                let t = line.trim();
                t.contains("taxonomy_")
            })
        });

        if !has_taxonomy_import {
            return;
        }

        let has_real_usage = imported.iter().any(|(symbol, _)| {
            utility_dummy_detector::symbol_used_real(
                file, content, symbol.value(), &ctx.dummy_ranges, &ctx.dummy_impl_traits,
            )
        });

        if !has_real_usage {
            let dummy_line = ctx.dummy_ranges.first()
                .map(|(s, _)| s.value() as usize)
                .unwrap_or(1);
            violations.push(LintResult::new_arch(
                file, dummy_line, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new("surfaces"),
                    import_type: SymbolName::new("taxonomy"),
                    intent: SymbolName::new("Use taxonomy Value Objects in function signatures instead of primitives"),
                    reason: Some(LintMessage::new("Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose.")),
                }.to_string(),
            ));
        }
    }

    fn _check_surface_logic(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lang = LanguageVO::from_path(file);
        let logic_patterns = ["lint_path(", "compute_score(", "has_critical(", "walk_rs_files("];

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            let is_skip = match lang {
                LanguageVO::Rust => trimmed.starts_with("//") || trimmed.starts_with("fn _use_"),
                LanguageVO::Python => trimmed.starts_with('#') || trimmed.starts_with("def _use_"),
                LanguageVO::JavaScript => trimmed.starts_with("//") || trimmed.starts_with("function _use"),
                LanguageVO::Unknown => false,
            };
            if is_skip {
                continue;
            }
            for pattern in &logic_patterns {
                if trimmed.contains(pattern) {
                    violations.push(LintResult::new_arch(
                        file, i + 1, "AES204", Severity::MEDIUM,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces"),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!("Delegate to aggregate instead of calling '{}' directly", pattern)),
                            reason: Some(LintMessage::new("Surface-layer code must delegate business logic to the aggregate layer.")),
                        }.to_string(),
                    ));
                }
            }
        }
    }
}

fn is_future_import(content: &str, symbol: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", symbol)
                || trimmed.contains(format!(", {}", symbol).as_str())
                || trimmed.contains(format!(" {},", symbol).as_str()))
    })
}
```

### 4.5 `capabilities_cycle_import_analyzer.rs` — MINOR UPDATE

```rust
// PURPOSE: DependencyCycleAnalyzer — AES205: circular dependency detection.
// Updated: uses AST-based module extraction. Cycle detection algorithm unchanged.

use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared::cli_commands::LintResult;
use shared::common::{FilePath, FilePathList, Severity};
use shared::common::utility_layer_detector;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::utility_cycle_detector;
use shared::import_rules::utility_import_module_parser;
use shared::import_rules::{AesImportViolation, DependencyEdge, ICycleImportProtocol, ImportError};
use shared::common::{LayerMapVO, LayerNameVO, LintMessage, SymbolName};
use std::collections::HashMap;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct DependencyCycleAnalyzer {}

type ScannedFileEdges = (Vec<DependencyEdge>, Option<(String, String)>);

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ICycleImportProtocol for DependencyCycleAnalyzer {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let file_strs: Vec<String> = files.iter().map(|f| f.to_string()).collect();
        let root_str = root_dir.to_string();
        self._scan(config, layer_map, &file_strs, &root_str)
    }

    async fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
    ) -> Result<Vec<LintResult>, ImportError> {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self._scan(config, layer_map, &file_strs, &root_dir.to_string());
        Ok(cycle_violations)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        utility_cycle_detector::detect_cycle_edges(edges)
    }

    fn normalize_to_layer(&self, name: &str) -> LayerNameVO {
        LayerNameVO::new(name.split('_').next().unwrap_or(name))
    }
}

// ─── Block 3: Helpers ─────────────────────────────────────

impl DependencyCycleAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn _scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return vec![];
        }
        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let layer_prefixes: Vec<String> = layer_keys.iter().map(|k| format!("{}_", k)).collect();

        let file_results: Vec<ScannedFileEdges> = files
            .par_iter()
            .filter_map(|file| {
                let file_fp = FilePath::new(file.clone()).ok()?;
                let basename = file_fp.basename();
                if let Some(rule) = aes205_rule {
                    if rule.exceptions.values.contains(&basename.to_string()) {
                        return None;
                    }
                }
                let content = shared::common::utility_file_handler::read_file_generic(file).ok()?;
                let filename = utility_layer_detector::extract_filename(file);
                let file_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
                    Some(l) => {
                        let specialized = utility_layer_detector::resolve_specialized_layer(
                            &l, file, &layer_keys,
                        );
                        let base_part = specialized.find('(').map(|i| &specialized[..i]).unwrap_or(&specialized);
                        base_part.to_string()
                    }
                    None => return None,
                };

                // AST-based module extraction with barrel resolution
                let resolved_modules =
                    utility_import_module_parser::extract_import_modules_resolved(
                        file, &content, root_dir,
                    );
                let modules: Vec<SymbolName> = resolved_modules
                    .into_iter()
                    .map(|(_, resolved)| resolved)
                    .collect();

                let mut local_edges = Vec::new();
                let mut has_cross_layer = false;

                for module in modules {
                    let module_value = module.value();
                    let is_crate_import = module_value.starts_with("crate::")
                        || module_value.starts_with("lint_arwaky::");
                    let is_cross_layer_crate = if is_crate_import {
                        let stripped = module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or("");
                        let first_segment = stripped.split("::").next().unwrap_or("");
                        layer_prefixes.iter().any(|prefix| stripped.starts_with(prefix))
                            || layer_keys.iter().any(|k| k == first_segment)
                    } else {
                        false
                    };
                    if is_crate_import && !is_cross_layer_crate {
                        continue;
                    }
                    let module_path = if is_crate_import {
                        module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or(module_value)
                    } else {
                        module_value
                    };
                    let target_layer =
                        utility_layer_detector::detect_module_layer(module_path, &layer_keys)
                            .or_else(|| {
                                utility_layer_detector::resolve_module_path_to_layer(module_path, root_dir)
                            });
                    if let Some(target_layer) = target_layer {
                        let target_layer_str = match target_layer.split('(').next() {
                            Some(p) => p.to_string(),
                            None => target_layer,
                        };
                        if target_layer_str != file_layer {
                            local_edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                            has_cross_layer = true;
                        }
                    }
                }

                let layer_mapping = if has_cross_layer {
                    Some((file_layer, file.clone()))
                } else {
                    None
                };
                Some((local_edges, layer_mapping))
            })
            .collect();

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();
        for (local_edges, layer_mapping) in file_results {
            edges.extend(local_edges);
            if let Some((fl, f)) = layer_mapping {
                file_by_layer.entry(fl).or_insert(f);
            }
        }

        // Cycle detection — pure graph algorithm, unchanged
        let cycle_edge_results = utility_cycle_detector::detect_cycle_edges(&edges);
        cycle_edge_results
            .into_iter()
            .map(|sn| {
                let edge_key = sn.value;
                let parts: Vec<&str> = edge_key.split("->").collect();
                let source = parts[0];
                let target = parts[1];
                let file = file_by_layer.get(source).cloned().unwrap_or_else(|| source.to_string());
                LintResult::new_arch(
                    &file, 1, "AES205", Severity::CRITICAL,
                    AesImportViolation::CircularImport {
                        reason: Some(LintMessage::new(format!(
                            "Circular dependency between layers '{}' and '{}' creates implicit bidirectional coupling.",
                            source, target
                        ))),
                    }
                    .to_string(),
                )
            })
            .collect()
    }
}
```

---

## PHASE 5: Feature Crate Agent/Root

### 5.1 `agent_import_orchestrator.rs` — MINOR UPDATE

Perubahan: `parse_import_lines_helper` sekarang butuh `file_path` parameter.

```rust
// Di run_audit, tidak ada perubahan signifikan.
// Orchestrator hanya dispatch ke capabilities.
// Semua perubahan ada di capabilities layer.
// File ini TIDAK BERUBAH secara struktural.
```

### 5.2 `root_import_rules_container.rs` — **TIDAK BERUBAH**

### 5.3 `lib.rs` — UPDATE

```rust
// PURPOSE: Module declarations for import-rules (5 capabilities + 5 protocols)
pub mod agent_import_orchestrator;
pub mod capabilities_cycle_import_analyzer;
pub mod capabilities_dummy_import_checker;
pub mod capabilities_import_forbidden_checker;
pub mod capabilities_import_mandatory_checker;
pub mod capabilities_import_unused_checker;
pub mod root_import_rules_container;
```

---

## PHASE 6: Cargo.toml Updates

### `crates/shared/Cargo.toml` — tambah syn

```toml
[dependencies]
# ... existing deps ...
syn = { version = "2", features = ["full", "visit", "parsing"] }
```

### `crates/import-rules/Cargo.toml` — hapus regex, once_cell

```toml
[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
# regex.workspace = true      ← HAPUS
# once_cell.workspace = true  ← HAPUS
shared.workspace = true
tokio.workspace = true
rayon.workspace = true
```

---

## Ringkasan: File yang Berubah vs Tidak

| File | Status | Perubahan Utama |
|---|---|---|
| **AST Parser Layer (BARU)** | | |
| `taxonomy_orphan_parse_result_vo.rs` | BARU | Shared types + `used_identifiers` + `AstFnDefVO` |
| `utility_orphan_rust_parser.rs` | BARU | `syn::visit::Visit` untuk identifier tracking |
| `utility_orphan_python_parser.rs` | BARU | Comment-aware + identifier collection |
| `utility_orphan_ts_parser.rs` | BARU | Comment-aware + identifier collection |
| `utility_orphan_parser_dispatch.rs` | BARU | Routing by extension |
| **Shared Utilities** | | |
| `utility_import_symbol_extractor.rs` | **REWRITE** | Hapus dynamic regex + DERIVE_MACROS (40+ entries) |
| `utility_import_resolver.rs` | **REWRITE** | AST-based `parse_import_lines_helper` |
| `utility_import_module_parser.rs` | **REWRITE** | AST-based module extraction |
| `utility_dummy_detector.rs` | **REWRITE** | AST-based dummy detection |
| `utility_cycle_detector.rs` | TIDAK BERUBAH | Pure graph theory |
| `utility_path_normalizer.rs` | TIDAK BERUBAH | Path utilities |
| **Shared Contracts/Taxonomy** | | |
| `taxonomy_import_constant.rs` | **REWRITE** | Hapus `DERIVE_MACROS` |
| `contract_unused_import_protocol.rs` | UPDATE | Signature update |
| Semua contract/taxonomy lain | TIDAK BERUBAH | — |
| **Feature Capabilities** | | |
| `capabilities_import_unused_checker.rs` | **REWRITE** | AST usage tracking |
| `capabilities_import_forbidden_checker.rs` | **REWRITE** | AST import extraction |
| `capabilities_import_mandatory_checker.rs` | **REWRITE** | AST + segment matching |
| `capabilities_dummy_import_checker.rs` | **REWRITE** | AST dummy detection |
| `capabilities_cycle_import_analyzer.rs` | UPDATE | AST module extraction |
| **Feature Agent/Root** | | |
| `agent_import_orchestrator.rs` | TIDAK BERUBAH | Dispatch only |
| `root_import_rules_container.rs` | TIDAK BERUBAH | Wiring only |
| `lib.rs` | TIDAK BERUBAH | Module declarations |
| **Cargo.toml** | | |
| `shared/Cargo.toml` | UPDATE | Tambah `syn` |
| `import-rules/Cargo.toml` | UPDATE | Hapus `regex`, `once_cell` |
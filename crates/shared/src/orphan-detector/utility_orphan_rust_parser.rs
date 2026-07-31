// PURPOSE: utility_orphan_rust_parser — Rust AST parsing via syn crate.
// Stateless standalone functions. Depends only on taxonomy VOs.

use super::taxonomy_orphan_parse_result_vo::{
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
            };
        }
    };
    let mut result = RustParseResultVO {
        parse_ok: true,
        ..Default::default()
    };
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
    let mut visitor = IdentifierVisitor {
        identifiers: Vec::new(),
    };
    visitor.visit_file(&syntax);
    result.used_identifiers = visitor.identifiers;
    result
}

// ─── Block 2: Identifier Visitor ──────────────────────────

struct IdentifierVisitor {
    identifiers: Vec<String>,
}

impl<'ast> Visit<'ast> for IdentifierVisitor {
    fn visit_item_use(&mut self, _node: &'ast syn::ItemUse) {
        // Skip identifiers inside import statements
    }
    fn visit_use_tree(&mut self, _node: &'ast syn::UseTree) {
        // Skip identifiers inside import trees
    }
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
    // Skip #[cfg(...)] gated imports (consistent with old line-based parser)
    if item_use.attrs.iter().any(|a| a.path().is_ident("cfg")) {
        return;
    }
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
                full_path,
                segments,
                is_reexport,
                false,
                line,
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
                full_path,
                segments,
                is_reexport,
                true,
                line,
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
                full_path,
                segments,
                is_reexport,
                false,
                line,
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

fn is_fn_body_dummy(block: &syn::Block) -> bool {
    let stmts: Vec<_> = block.stmts.iter().collect();
    if stmts.is_empty() {
        return true;
    }
    if stmts.len() == 1 {
        if let syn::Stmt::Expr(syn::Expr::Macro(expr_macro), _) = &stmts[0] {
            let macro_name = path_to_string(&expr_macro.mac.path);
            return matches!(
                macro_name.as_str(),
                "todo" | "unimplemented" | "panic" | "unreachable"
            );
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

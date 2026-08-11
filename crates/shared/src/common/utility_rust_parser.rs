// PURPOSE: utility_rust_parser — Rust AST parsing via syn crate.
// Stateless standalone functions. Depends only on taxonomy VOs.

use crate::orphan_rules::taxonomy_orphan_parse_result_vo::{
    AstFnDefVO, AstImportVO, AstModDeclVO, AstStructDefVO, AstTraitDefVO, AstTraitImplVO,
    IdentifierVisitor, RustParseResultVO,
};
use syn::visit::Visit;

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
        process_rust_item(item, &mut result);
    }
    let mut visitor = IdentifierVisitor {
        identifiers: Vec::new(),
    };
    visitor.visit_file(&syntax);
    result.used_identifiers = visitor.identifiers;
    result
}

/// Process a single Rust item (use, impl, struct, trait, mod, fn).
fn process_rust_item(item: &syn::Item, result: &mut RustParseResultVO) {
    match item {
        syn::Item::Use(item_use) => extract_use_tree(item_use, result),
        syn::Item::Impl(item_impl) => extract_trait_impl(item_impl, result),
        syn::Item::Struct(item_struct) => result.structs.push(AstStructDefVO {
            name: item_struct.ident.to_string(),
            is_pub: matches!(item_struct.vis, syn::Visibility::Public(_)),
        }),
        syn::Item::Trait(item_trait) => result.traits.push(AstTraitDefVO {
            name: item_trait.ident.to_string(),
            is_pub: matches!(item_trait.vis, syn::Visibility::Public(_)),
        }),
        syn::Item::Mod(item_mod) => result.mod_decls.push(AstModDeclVO {
            name: item_mod.ident.to_string(),
            path_attr: extract_path_attr(item_mod),
            is_pub: matches!(item_mod.vis, syn::Visibility::Public(_)),
        }),
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

fn extract_use_tree(item_use: &syn::ItemUse, result: &mut RustParseResultVO) {
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
        syn::UseTree::Path(use_path) => walk_use_path(use_path, prefix, result, is_reexport, line),
        syn::UseTree::Name(use_name) => walk_use_name(use_name, prefix, result, is_reexport, line),
        syn::UseTree::Glob(_) => walk_use_glob(prefix, result, is_reexport, line),
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                walk_use_tree(item, prefix, result, is_reexport, line);
            }
        }
        syn::UseTree::Rename(use_rename) => {
            walk_use_rename(use_rename, prefix, result, is_reexport, line)
        }
    }
}

/// Walk a use tree path node, recursing into its child tree.
fn walk_use_path(
    use_path: &syn::UsePath,
    prefix: &mut String,
    result: &mut RustParseResultVO,
    is_reexport: bool,
    line: usize,
) {
    let saved = prefix.clone();
    if !prefix.is_empty() {
        prefix.push_str("::");
    }
    prefix.push_str(&use_path.ident.to_string());
    walk_use_tree(&use_path.tree, prefix, result, is_reexport, line);
    *prefix = saved;
}

/// Walk a use tree name node and push the import.
fn walk_use_name(
    use_name: &syn::UseName,
    prefix: &str,
    result: &mut RustParseResultVO,
    is_reexport: bool,
    line: usize,
) {
    let mut full_path = prefix.to_string();
    if !full_path.is_empty() {
        full_path.push_str("::");
    }
    full_path.push_str(&use_name.ident.to_string());
    let segments: Vec<String> = full_path.split("::").map(String::from).collect();
    result.imports.push(AstImportVO::new(full_path, segments, is_reexport, false, line));
}

/// Walk a use tree glob and push the import.
fn walk_use_glob(
    prefix: &str,
    result: &mut RustParseResultVO,
    is_reexport: bool,
    line: usize,
) {
    let full_path = if prefix.is_empty() {
        "*".to_string()
    } else {
        format!("{}::*", prefix)
    };
    let segments: Vec<String> = prefix.split("::").map(String::from).collect();
    result.imports.push(AstImportVO::new(full_path, segments, is_reexport, true, line));
}

/// Walk a use tree rename and push the import with rename field.
fn walk_use_rename(
    use_rename: &syn::UseRename,
    prefix: &str,
    result: &mut RustParseResultVO,
    is_reexport: bool,
    line: usize,
) {
    let mut full_path = prefix.to_string();
    if !full_path.is_empty() {
        full_path.push_str("::");
    }
    full_path.push_str(&use_rename.ident.to_string());
    let segments: Vec<String> = full_path.split("::").map(String::from).collect();
    let mut import = AstImportVO::new(full_path, segments, is_reexport, false, line);
    import.rename = Some(use_rename.rename.to_string());
    result.imports.push(import);
}

fn extract_trait_impl(item_impl: &syn::ItemImpl, result: &mut RustParseResultVO) {
    let Some((trait_path, _)) = &item_impl.trait_ else {
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
    is_single_macro_stub(&stmts)
}

/// Return true if the statement is a single macro stub (todo, unimplemented, etc).
fn is_single_macro_stub(stmts: &[&syn::Stmt]) -> bool {
    if stmts.len() != 1 {
        return false;
    }
    match &stmts[0] {
        syn::Stmt::Expr(syn::Expr::Macro(expr_macro), _) => {
            let macro_name = path_to_string(&expr_macro.mac.path);
            matches!(
                macro_name.as_str(),
                "todo" | "unimplemented" | "panic" | "unreachable"
            )
        }
        _ => false,
    }
}

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

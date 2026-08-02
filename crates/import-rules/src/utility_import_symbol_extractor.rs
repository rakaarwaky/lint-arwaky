// PURPOSE: utility_import_symbol_extractor — AST-based unused import detection.
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_rules::taxonomy_parser_dispatcher::parse_file_content;
use std::collections::{HashMap, HashSet};

// ─── Block 1: Import Alias Extraction (AST-based) ─────────

pub fn extract_imported_aliases(file_path: &str, content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.is_glob {
                    continue;
                }
                if imp.raw_path.starts_with("crate::")
                    || imp.raw_path.starts_with("super::")
                    || imp.raw_path.starts_with("self::")
                {
                    continue;
                }
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                    && last != "self"
                {
                    aliases.insert(Identity::new(last), Identity::new(imp.raw_path.clone()));
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for imp in &result.imports {
                if imp.raw_path.starts_with("__future__") {
                    continue;
                }
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                {
                    aliases.insert(Identity::new(last), Identity::new(imp.raw_path.clone()));
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                    && last != "default"
                {
                    aliases.insert(Identity::new(last), Identity::new(imp.raw_path.clone()));
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }
    aliases
}

// ─── Block 2: Usage Detection (AST-based) ─────────────────

pub fn extract_used_symbols(
    file_path: &str,
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for alias in imported_aliases.keys() {
                if result.is_identifier_used(alias.value()) {
                    used.insert(Identity::new(alias.value()));
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for alias in imported_aliases.keys() {
                if result.is_identifier_used(alias.value()) {
                    used.insert(Identity::new(alias.value()));
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for alias in imported_aliases.keys() {
                if result.is_identifier_used(alias.value()) {
                    used.insert(Identity::new(alias.value()));
                }
            }
        }
        FileParseResultVO::Unsupported => {}
    }
    used
}

// ─── Block 3: Exported Symbol Detection (AST-based) ───────

pub fn extract_exported_symbols(file_path: &str, content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.is_reexport
                    && let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                {
                    exported.insert(Identity::new(last));
                }
            }
        }
        FileParseResultVO::Python(result) => {
            if file_path.ends_with("__init__.py") {
                for imp in &result.imports {
                    if let Some(last) = imp.last_segment()
                        && !last.is_empty()
                        && last != "*"
                    {
                        exported.insert(Identity::new(last));
                    }
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if imp.is_reexport {
                    if let Some(last) = imp.last_segment()
                        && !last.is_empty()
                        && last != "*"
                    {
                        exported.insert(Identity::new(last));
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

pub fn extract_rust_js_imports(file_path: &str, content: &str) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.is_glob {
                    continue;
                }
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                    && last != "_"
                {
                    imports.push((SymbolName::new(last), LineNumber::new(imp.line as i64)));
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                    && last != "default"
                {
                    imports.push((SymbolName::new(last), LineNumber::new(imp.line as i64)));
                }
            }
        }
        _ => {}
    }
    imports
}

pub fn is_name_used(file_path: &str, name: &str, content: &str, _exclude_line: usize) -> bool {
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(result) => result.is_identifier_used(name),
        FileParseResultVO::Python(result) => result.is_identifier_used(name),
        FileParseResultVO::TypeScript(result) => result.is_identifier_used(name),
        FileParseResultVO::Unsupported => false,
    }
}

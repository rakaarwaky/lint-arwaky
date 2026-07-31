// PURPOSE: utility_import_symbol_extractor — AST-based unused import detection.
// Replaces dynamic regex + hardcoded DERIVE_MACROS whitelist with syn AST.

use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use std::collections::{HashMap, HashSet};

// ─── Block 1: Import Alias Extraction (AST-based) ─────────

pub fn extract_imported_aliases(
    file_path: &str,
    content: &str,
) -> HashMap<Identity, (Identity, usize)> {
    let mut aliases = HashMap::new();
    match FileParseResultVO::parse_path_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.is_glob || imp.is_reexport {
                    continue;
                }
                if imp.raw_path.starts_with("crate::")
                    || imp.raw_path.starts_with("super::")
                    || imp.raw_path.starts_with("self::")
                {
                    continue;
                }
                if !imp.alias_name().is_empty()
                    && imp.alias_name() != "*"
                    && imp.alias_name() != "self"
                {
                    aliases.insert(
                        Identity::new(imp.alias_name()),
                        (Identity::new(imp.raw_path.clone()), imp.line),
                    );
                }
            }
        }
        FileParseResultVO::Python(result) => {
            for imp in &result.imports {
                if imp.is_reexport {
                    continue;
                }
                if imp.raw_path.starts_with("__future__") {
                    continue;
                }
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                {
                    aliases.insert(
                        Identity::new(last),
                        (Identity::new(imp.raw_path.clone()), imp.line),
                    );
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if imp.is_reexport {
                    continue;
                }
                if let Some(last) = imp.last_segment()
                    && !last.is_empty()
                    && last != "*"
                    && last != "default"
                {
                    aliases.insert(
                        Identity::new(last),
                        (Identity::new(imp.raw_path.clone()), imp.line),
                    );
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
    imported_aliases: &HashMap<Identity, (Identity, usize)>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    match FileParseResultVO::parse_path_content(file_path, content) {
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
    match FileParseResultVO::parse_path_content(file_path, content) {
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
    match FileParseResultVO::parse_path_content(file_path, content) {
        FileParseResultVO::Rust(result) => {
            for imp in &result.imports {
                if imp.is_glob || imp.is_reexport {
                    continue;
                }
                let name = imp.alias_name();
                if !name.is_empty() && name != "*" && name != "_" {
                    imports.push((SymbolName::new(name), LineNumber::new(imp.line as i64)));
                }
            }
        }
        FileParseResultVO::TypeScript(result) => {
            for imp in &result.imports {
                if imp.is_reexport {
                    continue;
                }
                let name = imp.alias_name();
                if !name.is_empty() && name != "*" && name != "default" {
                    imports.push((SymbolName::new(name), LineNumber::new(imp.line as i64)));
                }
            }
        }
        _ => {}
    }
    imports
}

pub fn is_name_used(file_path: &str, name: &str, content: &str, _exclude_line: usize) -> bool {
    match FileParseResultVO::parse_path_content(file_path, content) {
        FileParseResultVO::Rust(result) => result.is_identifier_used(name),
        FileParseResultVO::Python(result) => result.is_identifier_used(name),
        FileParseResultVO::TypeScript(result) => result.is_identifier_used(name),
        FileParseResultVO::Unsupported => false,
    }
}

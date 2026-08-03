// PURPOSE: utility_import_symbol_extractor — AST-based unused import detection.
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
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

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (consume filesystem crate's parsed data)
// ═══════════════════════════════════════════════════════════════

/// Extract imported aliases from ImportEntry list (replaces extract_imported_aliases).
/// Returns HashMap<alias, raw_path> — same shape as the legacy function.
pub fn extract_imported_aliases_from_entries(
    entries: &[ImportEntry],
) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    for entry in entries {
        // Skip glob imports
        if entry.is_wildcard {
            continue;
        }
        // Skip crate::, super::, self:: (Rust internal)
        if entry.raw_path.starts_with("crate::")
            || entry.raw_path.starts_with("super::")
            || entry.raw_path.starts_with("self::")
        {
            continue;
        }
        // Skip __future__ imports (Python)
        if entry.raw_path.starts_with("__future__") {
            continue;
        }
        // For grouped imports (use foo::{A, B}), each symbol is a separate alias
        if !entry.symbols.is_empty() {
            for sym in &entry.symbols {
                if sym == "*" || sym == "self" || sym == "default" || sym.is_empty() {
                    continue;
                }
                aliases.insert(
                    Identity::new(sym.clone()),
                    Identity::new(entry.raw_path.clone()),
                );
            }
        } else {
            // Single import — use last segment of raw_path as alias
            let last = entry
                .raw_path
                .rsplit("::")
                .next()
                .unwrap_or(&entry.raw_path);
            if last.is_empty() || last == "*" || last == "self" || last == "default" {
                continue;
            }
            aliases.insert(
                Identity::new(last.to_string()),
                Identity::new(entry.raw_path.clone()),
            );
        }
    }
    aliases
}

/// Extract exported symbols from ImportEntry list (replaces extract_exported_symbols).
pub fn extract_exported_symbols_from_entries(entries: &[ImportEntry]) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    for entry in entries {
        if !entry.is_reexport {
            continue;
        }
        if !entry.symbols.is_empty() {
            for sym in &entry.symbols {
                if sym == "*" || sym.is_empty() {
                    if entry.is_wildcard {
                        exported.insert(Identity::new("*"));
                    }
                    continue;
                }
                exported.insert(Identity::new(sym.clone()));
            }
        } else {
            let last = entry
                .raw_path
                .rsplit("::")
                .next()
                .unwrap_or(&entry.raw_path);
            if last == "*" {
                exported.insert(Identity::new("*"));
            } else if !last.is_empty() {
                exported.insert(Identity::new(last.to_string()));
            }
        }
    }
    exported
}

/// Extract used symbols from ImportEntry list + usage data (replaces extract_used_symbols).
/// Still needs FileParseResultVO for identifier usage tracking.
pub fn extract_used_symbols_from_entries(
    file_path: &str,
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    // Usage tracking still requires the shared crate's parse result
    // (FileParseResultVO has used_identifiers computed from AST)
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

/// Extract Rust/JS import symbols from ImportEntry list (replaces extract_rust_js_imports).
pub fn extract_rust_js_imports_from_entries(
    entries: &[ImportEntry],
) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    for entry in entries {
        if entry.is_wildcard {
            continue;
        }
        // Get the last segment as the symbol name
        let last = entry
            .raw_path
            .rsplit("::")
            .next()
            .unwrap_or(&entry.raw_path);
        if last.is_empty() || last == "*" || last == "_" || last == "default" {
            continue;
        }
        // For grouped imports, emit each symbol separately
        if !entry.symbols.is_empty() {
            for sym in &entry.symbols {
                if sym.is_empty() || sym == "*" || sym == "_" || sym == "default" {
                    continue;
                }
                imports.push((SymbolName::new(sym.clone()), LineNumber::new(1)));
            }
        } else {
            imports.push((SymbolName::new(last.to_string()), LineNumber::new(1)));
        }
    }
    imports
}

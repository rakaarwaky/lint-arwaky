use crate::utility_path_normalizer;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (direct field access)
// ═══════════════════════════════════════════════════════════════

/// Get the module path from an ImportEntry's raw_path.
pub fn entry_module_path(entry: &ImportEntry) -> &str {
    if let Some(brace_pos) = entry.raw_path.find("::{") {
        return &entry.raw_path[..brace_pos];
    }
    &entry.raw_path
}

/// Get the symbols imported by an ImportEntry.
pub fn entry_symbols(entry: &ImportEntry) -> Vec<&str> {
    if !entry.symbols.is_empty() {
        return entry.symbols.iter().map(|s| s.as_str()).collect();
    }
    let last = entry
        .raw_path
        .rsplit("::")
        .next()
        .unwrap_or(&entry.raw_path);
    if last.is_empty() || last == "*" || last == "self" || last == "_" {
        vec![]
    } else {
        vec![last]
    }
}

/// Check if an ImportEntry's raw_path matches the given layer scope.
pub fn entry_matches_scope(
    entry: &ImportEntry,
    layer: &LayerNameVO,
    suffixes: &[Identity],
) -> bool {
    let segments: Vec<&str> = entry
        .raw_path
        .split(|c: char| {
            c == ':'
                || c == '.'
                || c == '/'
                || c == '\\'
                || c.is_whitespace()
                || c == '"'
                || c == '\''
                || c == '{'
                || c == '}'
                || c == ','
                || c == ';'
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

    let symbols = entry_symbols(entry);
    suffixes.iter().any(|s| {
        let s_val = s.value();
        symbols.iter().any(|sym| {
            let sym_lower = sym.to_lowercase();
            if sym_lower.ends_with(&format!("_{}", s_val)) {
                return true;
            }
            if let Some(rest) = sym_lower.strip_suffix(&s_val.to_lowercase()) {
                if rest.is_empty() || rest.ends_with('_') {
                    return true;
                }
                if sym.len() >= s_val.len() {
                    let suffix_in_orig = &sym[sym.len() - s_val.len()..];
                    if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                        return true;
                    }
                }
            }
            false
        }) || segments.iter().any(|seg| {
            let cleaned = seg
                .trim_end_matches(';')
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim();
            cleaned.split(',').any(|t| {
                let name = t.trim();
                let name_lower = name.to_lowercase();
                if name_lower.ends_with(&format!("_{}", s_val)) {
                    return true;
                }
                if let Some(rest) = name_lower.strip_suffix(&s_val.to_lowercase()) {
                    if rest.is_empty() || rest.ends_with('_') {
                        return true;
                    }
                }
                false
            })
        })
    })
}

/// Extract layer name from an ImportEntry's raw_path.
pub fn entry_layer(entry: &ImportEntry) -> Option<LayerNameVO> {
    let module = entry_module_path(entry);
    let first_segment = module.split("::").next()?;
    extract_layer_from_import(&Identity::new(first_segment.to_string()))
}

// ═══════════════════════════════════════════════════════════════
// Shared helper functions
// ═══════════════════════════════════════════════════════════════

/// Parse a scope value (e.g. "contract(protocol)") into layer + suffix matches.
pub fn resolve_scope(scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
    let scope_str = scope.value();
    if let Some(paren) = scope_str.find('(') {
        let layer = scope_str[..paren].trim();
        let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<Identity> = if inner.contains('|') {
            inner
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        } else {
            inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        };
        (LayerNameVO::new(layer), suffixes)
    } else {
        (LayerNameVO::new(scope_str.trim()), vec![])
    }
}

/// Extract layer name from an import segment.
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

/// Find the line number of an import statement containing the given alias.
pub fn find_import_line_number(content: &str, alias: &str) -> LineNumber {
    let first_part = alias.split('.').next().unwrap_or("");
    let pos_opt = content.lines().position(|l| {
        let t = l.trim();
        let is_use =
            t.starts_with("use ") || t.starts_with("pub use ") || t.starts_with("pub(crate) use ");
        let alias_in_use = is_use && (t.contains(alias));
        let py_import = t.contains(&format!("import {alias}"))
            || t.contains(&format!("from {first_part} import"));
        alias_in_use || py_import
    });
    let line = match pos_opt {
        Some(p) => p + 1,
        None => 1,
    };
    LineNumber::new(line as i64)
}

/// Check if a filename is a barrel/re-export file.
pub fn is_barrel_file(filename: &str) -> bool {
    matches!(
        filename,
        "__init__.py"
            | "mod.rs"
            | "lib.rs"
            | "main.rs"
            | "index.ts"
            | "index.js"
            | "index.tsx"
            | "index.jsx"
    )
}

/// Check if a symbol is a Python `__future__` import.
pub fn is_future_import(content: &str, symbol: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", symbol)
                || trimmed.contains(format!(", {}", symbol).as_str())
                || trimmed.contains(format!(" {},", symbol).as_str()))
    })
}

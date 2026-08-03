use crate::utility_path_normalizer;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
use shared::import_rules::taxonomy_resolved_import_vo::ResolvedImport;
use std::collections::HashMap;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (consume filesystem crate's parsed data)
// ═══════════════════════════════════════════════════════════════

/// Convert ImportEntry list to the legacy (LineNumber, LineContentVO) format.
/// Bridge function for checkers that haven't migrated to ImportEntry yet.
pub fn import_entries_to_lines(entries: &[ImportEntry]) -> Vec<(LineNumber, LineContentVO)> {
    entries
        .iter()
        .map(|e| {
            (
                LineNumber::new(1), // ImportEntry doesn't carry line number
                LineContentVO::new(format_raw_path_for_line(e)),
            )
        })
        .collect()
}

/// Format an ImportEntry's raw_path into the line content format expected by checkers.
fn format_raw_path_for_line(entry: &ImportEntry) -> String {
    match entry.import_type {
        shared::filesystem::taxonomy_filesystem_vo::ImportType::Use => {
            let prefix = if entry.is_reexport {
                "pub use "
            } else {
                "use "
            };
            if entry.symbols.is_empty() {
                format!("{}{};", prefix, entry.raw_path)
            } else {
                format!(
                    "{}{}::{{{}}};",
                    prefix,
                    entry.raw_path,
                    entry.symbols.join(", ")
                )
            }
        }
        shared::filesystem::taxonomy_filesystem_vo::ImportType::Import => {
            format!("import {}", entry.raw_path)
        }
        shared::filesystem::taxonomy_filesystem_vo::ImportType::ImportFrom => {
            if entry.symbols.is_empty() {
                format!("import {} from '{}'", entry.raw_path, entry.raw_path)
            } else {
                format!(
                    "from {} import {}",
                    entry.raw_path,
                    entry.symbols.join(", ")
                )
            }
        }
        shared::filesystem::taxonomy_filesystem_vo::ImportType::ReExport => {
            format!("pub use {};", entry.raw_path)
        }
        _ => entry.raw_path.clone(),
    }
}

// ═══════════════════════════════════════════════════════════════
// Legacy line-based functions (kept for backward compat, will be removed)
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

/// Check if an import line satisfies the given scope requirement.
pub fn import_matches_scope(
    import_line: &LineContentVO,
    layer: &LayerNameVO,
    suffixes: &[Identity],
) -> bool {
    let import_line_str = import_line.value();
    let segments: Vec<&str> = import_line_str
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
    suffixes.iter().any(|s| {
        let s_val = s.value();
        segments.iter().any(|seg| {
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
                if let Some(rest) = name_lower.strip_suffix(s_val) {
                    if rest.is_empty() || rest.ends_with('_') {
                        return true;
                    }
                    if name.len() >= s_val.len() {
                        let suffix_in_orig = &name[name.len() - s_val.len()..];
                        if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                            return true;
                        }
                    }
                }
                false
            })
        })
    })
}

/// Extract the module path from an import line.
pub fn extract_module_from_line(line: &LineContentVO) -> Option<Identity> {
    let trimmed = line.value().trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
    }
    if trimmed.starts_with("import ") {
        if let Some(pos) = trimmed.rfind(" from ") {
            let module_part = trimmed[pos + 6..].trim();
            let cleaned = module_part
                .trim_end_matches(';')
                .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                .trim();
            return Some(Identity::new(cleaned.to_string()));
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                let cleaned = rest
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                return Some(Identity::new(cleaned.to_string()));
            }
            let first_token = rest.split_whitespace().next()?;
            return Some(Identity::new(first_token.to_string()));
        }
    }
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
    if !trimmed.is_empty() {
        return Some(Identity::new(trimmed.to_string()));
    }
    None
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

// ─── Barrel Import Resolution ─────────────────────────────

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

/// Normalize a module path for filesystem lookup.
fn normalize_module_path(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .replace('.', "/")
        .replace("::", "/")
}

/// Try to find a barrel file at the given base path with all candidate names.
/// Uses content_map when available to avoid filesystem I/O.
fn try_barrel_candidates(
    dir: &Path,
    candidates: &[&str],
    content_map: Option<&HashMap<String, String>>,
) -> Option<String> {
    for candidate in candidates {
        let barrel_path = dir.join(candidate);
        let path_str = barrel_path.to_string_lossy().to_string();
        let exists = match content_map {
            Some(cm) => cm.contains_key(&path_str),
            None => barrel_path.exists(),
        };
        if exists {
            return Some(path_str);
        }
    }
    None
}

/// Find the barrel file for a module path.
/// Uses content_map when available to avoid filesystem I/O.
pub fn find_barrel_file(
    module_path: &str,
    root_dir: &str,
    content_map: Option<&HashMap<String, String>>,
) -> Option<String> {
    let base = Path::new(root_dir);
    let clean_path = normalize_module_path(module_path);
    let module_dir = base.join(&clean_path);

    let barrel_candidates = [
        "__init__.py",
        "index.ts",
        "index.js",
        "index.tsx",
        "index.jsx",
        "mod.rs",
    ];

    if let Some(found) = try_barrel_candidates(&module_dir, &barrel_candidates, content_map) {
        return Some(found);
    }

    if let Some(parent) = module_dir.parent()
        && let Some(found) = try_barrel_candidates(parent, &barrel_candidates, content_map)
    {
        return Some(found);
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
                    if let Some(found) =
                        try_barrel_candidates(&full_dir, &barrel_candidates, content_map)
                    {
                        return Some(found);
                    }
                }
            }

            if remainder.len() > 1 {
                let remainder_str = remainder.join("/");
                let parent_path = Path::new(&remainder_str);
                if let Some(parent_dir) = parent_path.parent() {
                    let dir = crate_src.join(parent_dir);
                    if let Some(found) =
                        try_barrel_candidates(&dir, &barrel_candidates, content_map)
                    {
                        return Some(found);
                    }
                    let parent_hyphen = parent_dir.to_string_lossy().replace('_', "-");
                    let dir_hyphen = crate_src.join(&parent_hyphen);
                    if let Some(found) =
                        try_barrel_candidates(&dir_hyphen, &barrel_candidates, content_map)
                    {
                        return Some(found);
                    }
                }
            }
        }
    }

    None
}

/// Extract the file stem from a module path.
fn extract_module_stem(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .rsplit(['.', '/'])
        .next()
        .unwrap_or(module_path)
        .to_string()
}

/// Parse re-export mappings from a barrel file's content.
pub fn parse_barrel_reexports(barrel_content: &str) -> HashMap<String, String> {
    let mut reexports: HashMap<String, String> = HashMap::new();

    for line in barrel_content.lines() {
        let trimmed = line.trim();

        // Python: from .module import X, Y
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part.strip_prefix("from ").unwrap_or("").trim();
                let module_stem = extract_module_stem(module);

                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    let exported_name = name.split(" as ").last().unwrap_or(name).trim();
                    reexports.insert(exported_name.to_string(), module_stem.clone());
                }
            }
            continue;
        }

        // TS/JS: export { X, Y } from './module'
        if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[from_pos + 6..].trim();
                let module_clean = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == '`');
                let module_stem = extract_module_stem(module_clean);

                if let Some(brace_start) = trimmed.find('{')
                    && let Some(brace_end) = trimmed.find('}')
                {
                    let inner = &trimmed[brace_start + 1..brace_end];
                    for part in inner.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }
                        let exported_name = part.split(" as ").last().unwrap_or(part).trim();
                        reexports.insert(exported_name.to_string(), module_stem.clone());
                    }
                }
            }
            continue;
        }

        // Rust: pub use submodule::Type; / pub use submodule::{A, B};
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

/// Resolve an import through a barrel file to its original source file.
/// Uses content_map when available to avoid filesystem I/O.
pub fn resolve_barrel_import(
    module_path: &str,
    symbol_name: &str,
    root_dir: &str,
    content_map: Option<&HashMap<String, String>>,
) -> Option<ResolvedImport> {
    let barrel_path = find_barrel_file(module_path, root_dir, content_map)?;
    let barrel_content = match content_map {
        Some(cm) => cm.get(&barrel_path)?.clone(),
        None => std::fs::read_to_string(&barrel_path).ok()?,
    };
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

/// Extract imported symbol names from an import line.
pub fn extract_symbol_names(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let mut names = Vec::new();

    // Python: from X import A, B
    if trimmed.starts_with("from ") {
        if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
            let clean = import_part
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_end_matches(';');
            for part in clean.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    names.push(name.to_string());
                }
            }
        }
        return names;
    }

    // Rust: use shared::module::{A, B}; / use module::Type;
    if trimmed.starts_with("use ")
        || trimmed.starts_with("pub use ")
        || trimmed.starts_with("pub(crate) use ")
    {
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

    // TS/JS: import { A, B } from './module'
    if trimmed.starts_with("import ") && trimmed.contains('{') {
        if let Some(open) = trimmed.find('{')
            && let Some(close) = trimmed.find('}')
        {
            let inner = &trimmed[open + 1..close];
            for part in inner.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
        return names;
    }

    // TS/JS: import X from './module'
    if trimmed.starts_with("import ")
        && trimmed.contains(" from ")
        && let Some(import_part) = trimmed.strip_prefix("import ")
    {
        let name = import_part.split(" from ").next().unwrap_or("").trim();
        if !name.is_empty() && name != "default" && name != "*" {
            names.push(name.to_string());
        }
    }

    names
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

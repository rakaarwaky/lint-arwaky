// PURPOSE: Import parsing utility functions — stateless, domain-agnostic, multi-consumer
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_resolved_import_vo::ResolvedImport;
use crate::import_rules::utility_path_normalizer;
use std::collections::HashMap;
use std::path::Path;

/// Convert a Result<FilePath, _> to FilePath, using default on error.
pub fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Convert an optional OsStr reference to a string slice.
pub fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

/// Parse import lines from file content.
/// Handles: use, pub use, pub(crate) use, import, from, extern crate.
/// Skips: #[cfg(...)] conditional blocks (test, feature, etc.).
/// Handles multi-line use statements with braces and trailing commas.
pub fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut in_cfg_block = false;
    let mut cfg_brace_depth = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Skip #[cfg(...)] attribute lines and their associated blocks
        if trimmed.starts_with("#[cfg(") {
            in_cfg_block = true;
            cfg_brace_depth = 0;
            i += 1;
            continue;
        }
        if in_cfg_block {
            // Count braces to find the end of the cfg block
            for ch in trimmed.chars() {
                match ch {
                    '{' => cfg_brace_depth += 1,
                    '}' => {
                        cfg_brace_depth -= 1;
                        if cfg_brace_depth <= 0 {
                            in_cfg_block = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
            continue;
        }

        if trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("extern crate ")
        {
            result.push((
                LineNumber::new((i + 1) as i64),
                LineContentVO::new(lines[i].to_string()),
            ));
            i += 1;
            continue;
        }
        if trimmed.starts_with("use ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("pub(crate) use ")
        {
            let mut combined = lines[i].to_string();
            if combined.contains('{') && !combined.contains('}') {
                let start = i;
                i += 1;
                while i < lines.len() {
                    let part = lines[i].trim().to_string();
                    combined.push_str(&format!(" {}", part));
                    if part.contains('}') || combined.ends_with(';') {
                        break;
                    }
                    i += 1;
                }
                // Normalize whitespace and clean trailing commas before closing brace
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                combined = clean_trailing_commas(&combined);
                result.push((
                    LineNumber::new((start + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else if !combined.ends_with(';') {
                while i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next.starts_with("use ")
                        || next.starts_with("pub use ")
                        || next.starts_with("pub(crate) use ")
                        || next.starts_with("#[cfg(")
                        || next.is_empty()
                    {
                        break;
                    }
                    combined.push_str(&format!(" {}", next));
                    if next.ends_with(';') {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                combined = clean_trailing_commas(&combined);
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else {
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            }
        }
        i += 1;
    }
    result
}

/// Clean trailing commas before closing braces in use statements.
/// e.g., `use foo::{A, B,}` → `use foo::{A, B}`
fn clean_trailing_commas(s: &str) -> String {
    if let Some(brace_pos) = s.rfind('{') {
        let after_brace = &s[brace_pos..];
        if after_brace.ends_with("}") || after_brace.ends_with("};") {
            // Find the last non-whitespace char before }
            let bytes = after_brace.as_bytes();
            let mut end = bytes.len();
            // Skip trailing } and ;
            while end > 0 && (bytes[end - 1] == b'}' || bytes[end - 1] == b';') {
                end -= 1;
            }
            let inner = &after_brace[..end];
            if let Some(trimmed_inner) = inner.strip_suffix(',') {
                let before = &s[..brace_pos];
                let after = &after_brace[end..];
                return format!("{before}{trimmed_inner}{after}");
            }
        }
    }
    s.to_string()
}

/// Parse a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
/// into layer + suffix matches. Returns (LayerNameVO, Vec<Identity>).
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
        l.trim().contains(&format!("import {}", alias))
            || l.trim().contains(&format!("from {} import", first_part))
    });
    let line = match pos_opt {
        Some(p) => p + 1,
        None => 1,
    };
    LineNumber::new(line as i64)
}

// ─── Barrel Import Resolution ─────────────────────────────

/// Check if a module path points to a barrel file.
/// Returns the barrel file path if it exists.
///
/// # Examples
/// - `"mypackage"` → checks `mypackage/__init__.py`, `mypackage/index.ts`, `mypackage/mod.rs`
/// - `"./services"` → checks `./services/__init__.py`, `./services/index.ts`
pub fn find_barrel_file(module_path: &str, root_dir: &str) -> Option<String> {
    let base = Path::new(root_dir);
    let module_dir = base.join(module_path.replace('.', "/"));

    let barrel_candidates = [
        "__init__.py",
        "index.ts",
        "index.js",
        "index.tsx",
        "index.jsx",
        "mod.rs",
    ];

    for candidate in &barrel_candidates {
        let barrel_path = module_dir.join(candidate);
        if barrel_path.exists() {
            return Some(barrel_path.to_string_lossy().to_string());
        }
    }
    None
}

/// Parse re-export mappings from a barrel file's content.
/// Returns a map: symbol_name → source_module_path
///
/// Handles:
/// - Python: `from .module import X`, `from .sub.module import X as Y`
/// - TS/JS:  `export { X } from './module'`, `export { default as X } from './module'`
/// - Rust:   `pub use submodule::Type;`, `pub use submodule::{A, B};`
pub fn parse_barrel_reexports(barrel_content: &str) -> HashMap<String, String> {
    let mut reexports: HashMap<String, String> = HashMap::new();

    for line in barrel_content.lines() {
        let trimmed = line.trim();

        // ── Python: from .module import X, Y ──
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part.strip_prefix("from ").unwrap_or("").trim();
                // Convert relative module to path: ".capabilities_payment_service" → "capabilities_payment_service"
                let module_path = module.trim_start_matches('.').replace('.', "/");

                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    // Handle "X as Y" → exported name is Y, source is X
                    let (source_name, exported_name) = match name.split_once(" as ") {
                        Some((src, alias)) => (src.trim(), alias.trim()),
                        None => (name, name),
                    };
                    reexports.insert(
                        exported_name.to_string(),
                        format!("{}/{}", module_path, source_name),
                    );
                }
            }
            continue;
        }

        // ── TS/JS: export { X, Y } from './module' ──
        if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[from_pos + 6..].trim();
                let module_path = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == '`')
                    .trim_start_matches("./");

                if let Some(brace_start) = trimmed.find('{') {
                    if let Some(brace_end) = trimmed.find('}') {
                        let inner = &trimmed[brace_start + 1..brace_end];
                        for part in inner.split(',') {
                            let part = part.trim();
                            if part.is_empty() {
                                continue;
                            }
                            let (source_name, exported_name) = match part.split_once(" as ") {
                                Some((src, alias)) => (src.trim(), alias.trim()),
                                None => (part, part),
                            };
                            reexports.insert(
                                exported_name.to_string(),
                                format!("{}/{}", module_path, source_name),
                            );
                        }
                    }
                }
            }
            continue;
        }

        // ── Rust: pub use submodule::Type; ──
        if trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use ") {
            let use_part = trimmed
                .trim_start_matches("pub(crate) use ")
                .trim_start_matches("pub use ")
                .trim_end_matches(';')
                .trim();

            if let Some(brace_pos) = use_part.find("::{") {
                let prefix = &use_part[..brace_pos];
                let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                for name in inner.split(',') {
                    let name = name.trim().split(" as ").last().unwrap_or("").trim();
                    if !name.is_empty() && name != "*" {
                        reexports.insert(
                            name.to_string(),
                            format!("{}::{}", prefix, name),
                        );
                    }
                }
            } else {
                let name = use_part.rsplit("::").next().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    reexports.insert(name.to_string(), use_part.to_string());
                }
            }
        }
    }

    reexports
}

/// Resolve an import through a barrel file to its original source file.
///
/// # Arguments
/// * `module_path` - The module path from the import statement (e.g., "mypackage")
/// * `symbol_name` - The imported symbol (e.g., "PaymentService")
/// * `root_dir` - Workspace root directory
///
/// # Returns
/// `Some(ResolvedImport)` if the symbol was found in a barrel file's re-exports,
/// `None` if no barrel file exists or symbol not found.
pub fn resolve_barrel_import(
    module_path: &str,
    symbol_name: &str,
    root_dir: &str,
) -> Option<ResolvedImport> {
    // Step 1: Find the barrel file
    let barrel_path = find_barrel_file(module_path, root_dir)?;

    // Step 2: Read barrel file content
    let barrel_content = std::fs::read_to_string(&barrel_path).ok()?;

    // Step 3: Parse re-export mappings
    let reexports = parse_barrel_reexports(&barrel_content);

    // Step 4: Look up the symbol
    let resolved_source = reexports.get(symbol_name)?;

    // Step 5: Extract the file name from resolved source for layer detection
    // "capabilities_payment_service/PaymentService" → "capabilities_payment_service"
    // "./services/user-service/UserService" → "user-service"
    let resolved_file = resolved_source
        .rsplit('/')
        .next()
        .or_else(|| resolved_source.rsplit("::").next())
        .unwrap_or(resolved_source)
        .to_string();

    // Step 6: Detect layer from resolved file name
    let resolved_layer = utility_path_normalizer::extract_layer_from_prefix(&resolved_file);

    Some(ResolvedImport {
        original_module: module_path.to_string(),
        resolved_file,
        resolved_layer,
        symbol: symbol_name.to_string(),
    })
}

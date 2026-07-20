// PURPOSE: Import parsing utility functions — stateless, domain-agnostic, multi-consumer
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::utility_path_normalizer;

/// Convert a Result<FilePath, _> to FilePath, using default on error.
pub fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Convert an optional OsStr reference to a string slice.
pub fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

/// Parse import lines from file content.
pub fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
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
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
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

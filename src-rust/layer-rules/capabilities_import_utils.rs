// PURPOSE: Shared import parsing utilities for AES001 + AES002 checkers.
use std::fs;
use std::path::Path;

/// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
/// into layer + suffix matches. Returns (layer_name, suffixes).
pub fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
    if let Some(paren) = scope.find('(') {
        let layer = scope[..paren].trim();
        let inner = scope[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<&str> = if inner.contains('|') {
            inner
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        };
        (layer, suffixes)
    } else {
        (scope.trim(), vec![])
    }
}

/// Check if an import line satisfies the given scope requirement.
pub fn import_matches_scope(import_line: &str, layer: &str, suffixes: &[&str]) -> bool {
    let segments: Vec<&str> = import_line.split("::").collect();
    let layer_lower = layer.to_lowercase();
    let layer_prefix = format!("{}_", layer_lower);
    let layer_match = segments.iter().any(|s| {
        let trimmed = s.trim().to_lowercase();
        trimmed == layer_lower || trimmed.starts_with(&layer_prefix)
    });
    if !layer_match || suffixes.is_empty() {
        return layer_match;
    }
    suffixes.iter().any(|s| {
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
                if name_lower.ends_with(&format!("_{}", s)) {
                    return true;
                }
                if let Some(rest) = name_lower.strip_suffix(s) {
                    if rest.is_empty() || rest.ends_with('_') {
                        return true;
                    }
                    if name.len() >= s.len() {
                        let suffix_in_orig = &name[name.len() - s.len()..];
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

pub fn get_basename(file: &str) -> String {
    Path::new(file)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn read_import_lines(file: &str) -> Vec<(usize, String)> {
    let Ok(content) = fs::read_to_string(file) else {
        return vec![];
    };
    parse_import_lines(&content)
}

pub fn parse_import_lines(content: &str) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("extern crate ")
        {
            result.push((i + 1, lines[i].to_string()));
            i += 1;
            continue;
        }
        if trimmed.starts_with("use ") {
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
                result.push((start + 1, combined));
            } else if !combined.ends_with(';') {
                while i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next.starts_with("use ") || next.is_empty() {
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
                result.push((i + 1, combined));
            } else {
                result.push((i + 1, combined));
            }
        }
        i += 1;
    }
    result
}

pub fn extract_module_from_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        return Some(rest.split_whitespace().next()?.to_string());
    }
    if let Some(rest) = trimmed.strip_prefix("import ") {
        return Some(rest.split_whitespace().next()?.to_string());
    }
    if let Some(rest) = trimmed.strip_prefix("use ") {
        let module = rest.trim_end_matches(';').trim().to_string();
        if let Some(brace_pos) = module.find("::{") {
            return Some(module[..brace_pos].to_string());
        }
        return Some(module);
    }
    None
}

pub fn extract_layer_from_import(segment: &str) -> Option<String> {
    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("infrastructure_", "infrastructure"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
    ];
    for (prefix, layer) in PREFIX_MAP {
        if segment.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }
    match segment {
        "taxonomy" => Some("taxonomy"),
        "contract" => Some("contract"),
        "capabilities" => Some("capabilities"),
        "infrastructure" => Some("infrastructure"),
        "agent" => Some("agent"),
        "surfaces" | "surface" => Some("surfaces"),
        _ => None,
    }
    .map(|s| s.to_string())
}

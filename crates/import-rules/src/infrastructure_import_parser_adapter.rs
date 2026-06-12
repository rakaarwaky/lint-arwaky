// PURPOSE: ImportParserAdapter — infrastructure implementation of IImportParserPort using standard filesystem and string search utilities

use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use std::fs;

pub struct ImportParserAdapter {}

impl ImportParserAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ImportParserAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl IImportParserPort for ImportParserAdapter {
    /// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
    /// into layer + suffix matches. Returns (LayerNameVO, Vec<Identity>).
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
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
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool {
        let import_line_str = import_line.value();
        let segments: Vec<&str> = import_line_str.split("::").collect();
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

    fn get_basename(&self, file: &FilePath) -> Identity {
        Identity::new(file.basename())
    }

    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        let Ok(content) = fs::read_to_string(file.value()) else {
            return vec![];
        };
        // Use helper function to parse
        parse_import_lines_helper(&content)
    }

    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        parse_import_lines_helper(content.value())
    }

    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity> {
        let trimmed = line.value().trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
        }
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';').trim().to_string();
            if let Some(brace_pos) = module.find("::{") {
                return Some(Identity::new(module[..brace_pos].to_string()));
            }
            return Some(Identity::new(module));
        }
        None
    }

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        let segment_str = segment.value();
        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
        ];
        for (prefix, layer) in PREFIX_MAP {
            if segment_str.starts_with(prefix) {
                return Some(LayerNameVO::new(*layer));
            }
        }
        match segment_str {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" => Some(LayerNameVO::new("capabilities")),
            "infrastructure" => Some(LayerNameVO::new("infrastructure")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
            _ => None,
        }
    }
}

/// Helper function to parse import lines from content, decoupled from any struct method call rules
fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
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
                result.push((
                    LineNumber::new((start + 1) as i64),
                    LineContentVO::new(combined),
                ));
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

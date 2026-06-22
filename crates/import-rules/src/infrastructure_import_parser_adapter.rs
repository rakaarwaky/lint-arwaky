// PURPOSE: ImportParserAdapter — infrastructure implementation of IImportParserPort using standard filesystem and string search utilities

use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_path_helper;
use shared::import_rules::{
    taxonomy_cycle_helper, taxonomy_dummy_helper, taxonomy_parser_helper, taxonomy_unused_helper,
};
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
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
    /// into layer + suffix matches. Returns (`LayerNameVO`, `Vec<Identity>`).
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
                let first_token = rest.split_whitespace().next().unwrap_or("");
                return Some(Identity::new(first_token.to_string()));
            }
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
        // Strategy 1: Prefix-based — reuse canonical helper (avoids duplicating PREFIX_MAP)
        if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(segment_str) {
            return Some(LayerNameVO::new(layer));
        }
        // Strategy 2: Direct segment match — bare layer names without underscore suffix
        match segment_str {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" => Some(LayerNameVO::new("capabilities")),
            "infrastructure" => Some(LayerNameVO::new("infrastructure")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
            "root" => Some(LayerNameVO::new("root")),
            _ => None,
        }
    }

    fn read_file_to_string(&self, file: &FilePath) -> Result<String, std::io::Error> {
        fs::read_to_string(file.value())
    }

    fn extract_import_modules(&self, content: &str) -> Vec<String> {
        taxonomy_parser_helper::extract_import_modules(content)
    }

    fn get_language_from_path(&self, path: &str) -> LanguageVO {
        LanguageVO::from_path(path)
    }

    fn get_dummy_function_ranges(&self, lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)> {
        taxonomy_dummy_helper::dummy_function_ranges(lines, lang)
    }

    fn get_imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)> {
        taxonomy_dummy_helper::imported_symbols(lines, lang)
    }

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(String, usize)> {
        taxonomy_dummy_helper::dummy_impl_traits_with_lines(lines)
    }

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool {
        taxonomy_dummy_helper::symbol_used_real(lines, symbol, dummy_ranges, dummy_impl_traits)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        taxonomy_cycle_helper::detect_cycle_edges(edges)
    }

    fn extract_imported_aliases(&self, content: &str) -> std::collections::HashMap<String, String> {
        taxonomy_unused_helper::extract_imported_aliases(content)
    }

    fn extract_exported_symbols(&self, content: &str) -> std::collections::HashSet<String> {
        taxonomy_unused_helper::extract_exported_symbols(content)
    }

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashSet<String> {
        taxonomy_unused_helper::extract_used_symbols(content, imported_aliases)
    }

    fn find_import_line_number(&self, content: &str, alias: &str) -> usize {
        content
            .lines()
            .position(|l| {
                l.trim().contains(&format!("import {}", alias))
                    || l.trim().contains(&format!(
                        "from {} import",
                        alias.split('.').next().unwrap_or("")
                    ))
            })
            .map(|p| p + 1)
            .unwrap_or(1)
    }

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(String, usize)> {
        taxonomy_unused_helper::extract_rust_js_imports(content)
    }

    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool {
        taxonomy_unused_helper::is_name_used(name, content, exclude_line)
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

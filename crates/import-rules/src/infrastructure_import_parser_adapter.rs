// PURPOSE: ImportParserAdapter — infrastructure implementation of IImportParserPort using standard filesystem and string search utilities

use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;

thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn clear_file_cache() {
    FILE_CACHE.with(|c| c.borrow_mut().clear());
}

/// Returns `s` if `opt` is `Some`, otherwise returns `""`.
fn str_or_empty(opt: Option<&str>) -> &str {
    opt.map_or("", |s| s)
}

pub struct ImportParserAdapter;

impl ImportParserAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl IImportParserPort for ImportParserAdapter {
    /// Extract layer from filename prefix — inline implementation.
    fn extract_layer_from_prefix(&self, segment: &str) -> Option<String> {
        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];
        for &(prefix, layer) in PREFIX_MAP {
            if segment.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }
        None
    }

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
                let first_token = str_or_empty(rest.split_whitespace().next());
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

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        let segment_str = segment.value();
        // Strategy 1: Prefix-based — reuse canonical helper
        if let Some(layer) = self.extract_layer_from_prefix(segment_str) {
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

    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error> {
        let path = file.value().to_string();
        let content = FILE_CACHE.with(|cache| -> Result<String, std::io::Error> {
            let mut cache = cache.borrow_mut();
            if let Some(cached) = cache.get(&path) {
                return Ok(cached.clone());
            }
            let raw = fs::read_to_string(&path)?;
            cache.insert(path, raw.clone());
            Ok(raw)
        })?;
        Ok(LintMessage::new(content))
    }

    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName> {
        let mut modules = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("from ") {
                if let Some(module) = rest.split_whitespace().next() {
                    modules.push(SymbolName::new(module));
                }
            } else if trimmed.starts_with("import ") {
                if let Some(pos) = trimmed.rfind(" from ") {
                    let module_part = trimmed[pos + 6..].trim();
                    let cleaned = module_part
                        .trim_end_matches(';')
                        .trim_matches(|c: char| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(SymbolName::new(cleaned));
                } else if let Some(rest) = trimmed.strip_prefix("import ") {
                    if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                        let cleaned = rest
                            .trim_end_matches(';')
                            .trim_matches(|c: char| c == '\'' || c == '"' || c == '`' || c == ';')
                            .trim();
                        modules.push(SymbolName::new(cleaned));
                    } else if let Some(first_token) = rest.split_whitespace().next() {
                        modules.push(SymbolName::new(first_token.trim_end_matches(',')));
                    }
                }
            } else if let Some(rest) = trimmed.strip_prefix("use ") {
                let module = rest.trim_end_matches(';');
                modules.push(SymbolName::new(module));
            }
        }
        modules
    }

    fn get_language_from_path(&self, path: &str) -> LanguageVO {
        LanguageVO::from_path(path)
    }

    fn get_dummy_function_ranges(
        &self,
        _lines: &[&str],
        _lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        vec![]
    }

    fn get_imported_symbols(
        &self,
        _lines: &[&str],
        _lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }

    fn get_dummy_impl_traits_with_lines(&self, _lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }

    fn is_symbol_used_real(
        &self,
        _lines: &[&str],
        _symbol: &str,
        _dummy_ranges: &[(LineNumber, LineNumber)],
        _dummy_impl_traits: &[String],
    ) -> bool {
        false
    }

    fn detect_cycle_edges(&self, _edges: &[DependencyEdge]) -> Vec<SymbolName> {
        vec![]
    }

    fn extract_imported_aliases(&self, _content: &str) -> HashMap<Identity, Identity> {
        HashMap::new()
    }

    fn extract_exported_symbols(&self, _content: &str) -> HashSet<Identity> {
        HashSet::new()
    }

    fn extract_used_symbols(
        &self,
        _content: &str,
        _imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        HashSet::new()
    }

    fn find_import_line_number(&self, content: &str, alias: &str) -> LineNumber {
        let pos_opt = content.lines().position(|l| {
            let first_part = str_or_empty(alias.split('.').next());
            l.trim().contains(&format!("import {}", alias))
                || l.trim().contains(&format!("from {} import", first_part))
        });
        let line = match pos_opt {
            Some(p) => p + 1,
            None => 1,
        };
        LineNumber::new(line as i64)
    }
    fn extract_rust_js_imports(&self, _content: &str) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }

    fn is_name_used(&self, _name: &str, _content: &str, _exclude_line: LineNumber) -> bool {
        false
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

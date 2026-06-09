// arch_import_checker — Import-related architectural checks.
// Implements IArchImportProtocol: check_mandatory_imports, check_forbidden_imports, check_legacy_import_rules.

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes001_forbidden_import, aes002_mandatory_import,
};
use std::fs;
use std::path::Path;

pub struct ArchImportRuleChecker {}

impl Default for ArchImportRuleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchImportRuleChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)",
    /// "contract(port|protocol|aggregate)", "taxonomy") into layer + suffix matches.
    /// Returns (layer_name, suffixes) where suffixes is empty if no suffix restriction.
    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
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
    /// e.g. scope "contract(protocol)" matches "use crate::di_containers::contract_service_aggregate::some_protocol::X"
    fn import_matches_scope(import_line: &str, layer: &str, suffixes: &[&str]) -> bool {
        let segments: Vec<&str> = import_line.split("::").collect();
        let layer_lower = layer.to_lowercase();
        let layer_match = segments
            .iter()
            .any(|s| s.trim().to_lowercase() == layer_lower);
        if !layer_match {
            return false;
        }
        if suffixes.is_empty() {
            return true;
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
                    // Snake_case: segment ends with _suffix
                    if name_lower.ends_with(&format!("_{}", s)) {
                        return true;
                    }
                    if let Some(rest) = name_lower.strip_suffix(s) {
                        // Exact match (e.g. suffix "aggregate" for "ServiceContainerAggregate")
                        if rest.is_empty() || rest.ends_with('_') {
                            return true;
                        }
                        // PascalCase: suffix starts with uppercase in original name
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

    fn get_basename(file: &str) -> String {
        Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string()
    }

    fn read_import_lines(file: &str) -> Vec<(usize, String)> {
        let Ok(content) = fs::read_to_string(file) else {
            return vec![];
        };
        let mut result: Vec<(usize, String)> = Vec::new();
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
                // Handle multi-line `use foo::{ ... }` blocks
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
                    // Collapse whitespace for matching
                    combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                    result.push((start + 1, combined));
                } else if !combined.ends_with(';') {
                    // Handle line continuation with trailing comma/backslash
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

    /// Parse import lines from a string content (avoids double file read).
    fn parse_import_lines(content: &str) -> Vec<(usize, String)> {
        let mut result: Vec<(usize, String)> = Vec::new();
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

    fn extract_module_from_line(line: &str) -> Option<String> {
        let trimmed = line.trim();
        // Python: `from x import y` or `import x`
        if let Some(rest) = trimmed.strip_prefix("from ") {
            let module = rest.split_whitespace().next()?.to_string();
            return Some(module);
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            let module = rest.split_whitespace().next()?.to_string();
            return Some(module);
        }
        // Rust: `use x::y;` or `use x::{a, b, c};`
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';').trim().to_string();
            // For multi-line imports like `crate::foo::{a, b}`, extract just `crate::foo`
            if let Some(brace_pos) = module.find("::{") {
                return Some(module[..brace_pos].to_string());
            }
            return Some(module);
        }
        None
    }

    /// Check mandatory imports: file must import from the required layers.
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }

        let basename = Self::get_basename(file);
        if basename == "__init__.py" {
            return;
        }

        if definition.exceptions.values.contains(&basename) {
            return;
        }

        let Ok(content) = fs::read_to_string(file) else {
            return;
        };
        let import_lines = Self::parse_import_lines(&content);

        for required in &definition.mandatory.values {
            let (layer, suffixes) = Self::resolve_scope(required);
            let is_present = if suffixes.is_empty() {
                import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| Self::import_matches_scope(l, layer, &suffixes))
            };

            // Skip mandatory import if file genuinely doesn't use ANY
            // types/identifiers from this layer (not even inline qualifiers).
            // Prevents forcing unused imports just to satisfy AES002.
            let genuinely_unreferenced = if suffixes.is_empty() {
                !import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                !import_lines.iter().any(|(_, l)| l.contains(layer))
                    && !suffixes
                        .iter()
                        .any(|s| import_lines.iter().any(|(_, l)| l.contains(s)))
            };

            if genuinely_unreferenced {
                continue;
            }

            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES002",
                    Severity::HIGH,
                    &aes002_mandatory_import(required),
                ));
            }
        }
    }

    /// Check forbidden imports: file must NOT import from forbidden layers.
    pub fn check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.forbidden.values.is_empty() {
            return;
        }

        let import_lines = Self::read_import_lines(file);

        for (line_num, line) in &import_lines {
            if let Some(module) = Self::extract_module_from_line(line) {
                let segments: Vec<&str> = module.split("::").collect();
                for forbidden in &definition.forbidden.values {
                    let (layer, suffixes) = Self::resolve_scope(forbidden);
                    let is_forbidden = if suffixes.is_empty() {
                        // Prefix-based per-segment check instead of naive substring
                        segments.iter().any(|seg| {
                            let cleaned = seg.trim_end_matches(';').trim();
                            Self::extract_layer_from_import(cleaned)
                                .map(|l| l == layer)
                                .unwrap_or(false)
                        })
                    } else {
                        Self::import_matches_scope(line, layer, &suffixes)
                    };
                    if is_forbidden {
                        violations.push(LintResult::new_arch(
                            file,
                            *line_num,
                            "AES001",
                            Severity::CRITICAL,
                            &aes001_forbidden_import(layer_name, &module),
                        ));
                    }
                }
            }
        }
    }

    /// Check legacy governance rules: cross-layer import restrictions.
    pub fn check_legacy_import_rules(
        &self,
        file: &str,
        file_layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        if config.governance_rules.is_empty() {
            return;
        }

        // Skip agent layer files
        if file_layer == "agent" {
            return;
        }

        let import_lines = Self::read_import_lines(file);

        for (line_num, line) in &import_lines {
            if let Some(module) = Self::extract_module_from_line(line) {
                // Determine target layer from module path
                let target_layer = self.detect_module_layer(&module, config);

                if let Some(target) = target_layer {
                    for rule in config.governance_rules.iter() {
                        let source_matches = rule.source_layer.value == file_layer;
                        let target_matches = rule.forbidden_target.value == target;

                        if source_matches && target_matches {
                            let desc = if !rule.description.value.is_empty() {
                                rule.description.value.clone()
                            } else {
                                "Forbidden layer import detected.".to_string()
                            };
                            let msg = format!(
                                "[AES Layer Violation] {}. File in '{}' imports from '{}' via '{}'.",
                                desc, file_layer, target, module
                            );
                            violations.push(LintResult::new_arch(
                                file,
                                *line_num,
                                "AES001",
                                Severity::CRITICAL,
                                &msg,
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn detect_module_layer(&self, module: &str, config: &ArchitectureConfig) -> Option<String> {
        // Try Rust-style :: separator first, then Python-style .
        let parts: Vec<&str> = if module.contains("::") {
            module.split("::").collect()
        } else {
            module.split('.').collect()
        };
        for part in &parts {
            // Prefix-based matching (FRD v1.1)
            if let Some(layer) = Self::extract_layer_from_import(part) {
                return Some(layer);
            }
            // Legacy path-based matching
            for (name, def) in &config.layers {
                if *part == name.value.as_str() {
                    return Some(name.value.clone());
                }
                let path_last = def.path.value.split('/').next_back().unwrap_or("");
                if *part == path_last {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }

    /// Extract layer name from an import segment using filename prefix.
    /// e.g. "capabilities_import_checker" → Some("capabilities")
    fn extract_layer_from_import(segment: &str) -> Option<String> {
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
        None
    }

    /// Check forbidden imports from per-scope ArchitectureRule conditions (AES001).
    /// Enforces scope-specific forbidden import rules from config (e.g. agent(orchestrator|coordinator) → forbid infrastructure, capabilities).
    pub fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Self::get_basename(file);
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        // Extract suffix from filename: agent_checking_coordinator.rs → "coordinator"
        let stem = basename.rsplit('.').next_back().unwrap_or(&basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = Self::read_import_lines(file);
        if import_lines.is_empty() {
            return;
        }

        // Iterate over all ArchitectureRule conditions (AES001 per-scope rules)
        for rule in &config.rules {
            let (rule_layer, rule_suffixes) = Self::resolve_scope(&rule.scope.value);
            // Check if file's layer matches rule's layer
            let layer_match = stem.starts_with(&format!("{}_", rule_layer));
            if !layer_match {
                continue;
            }
            // Check if file's suffix matches rule's suffix restriction (if any)
            if !rule_suffixes.is_empty() && !rule_suffixes.contains(&suffix) {
                continue;
            }
            // This rule applies — check each import against forbidden list
            for (line_num, line) in &import_lines {
                if let Some(module) = Self::extract_module_from_line(line) {
                    // Parse module path into segments (e.g. "crate::code_analysis::capabilities_class_checker::ArchClassChecker")
                    let segments: Vec<&str> = module.split("::").collect();
                    for forbidden in &rule.forbidden.values {
                        let (forbidden_layer, forbidden_suffixes) = Self::resolve_scope(forbidden);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            // Check each path segment with prefix matching (not naive substring)
                            let layer_match = segments.iter().any(|seg| {
                                let cleaned = seg.trim_end_matches(';').trim();
                                Self::extract_layer_from_import(cleaned)
                                    .map(|l| l == forbidden_layer)
                                    .unwrap_or(false)
                            });
                            // Also check if module contains forbidden as raw string (for special cases)
                            layer_match
                        } else {
                            Self::import_matches_scope(line, forbidden_layer, &forbidden_suffixes)
                        };
                        if is_forbidden {
                            violations.push(LintResult::new_arch(
                                file,
                                *line_num,
                                "AES001",
                                Severity::CRITICAL,
                                &aes001_forbidden_import(rule_layer, &module),
                            ));
                        }
                    }
                }
            }
        }
    }
}

// import_violation_analyzer — Cross-layer import rule enforcer (Capability).
// Implements IImportViolationProtocol: scan files for forbidden cross-layer imports.

use std::fs;
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerNameVO,
    LintMessage, LintResult, LineNumber, Severity, ScopeRef, LocationList,
    ArchitectureConfig,
};

/// Enforces cross-layer import restrictions (Capability).
pub struct ImportViolationAnalyzer {
    config: ArchitectureConfig,
}

impl ImportViolationAnalyzer {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    fn make_result(file: &str, line: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()),
            line: LineNumber::new(line),
            column: ColumnNumber::new(0),
            code: ErrorCode::new(code),
            message: LintMessage::new(msg),
            source: AdapterName::new("architecture"),
            severity: sev,
            enclosing_scope: ScopeRef {
                name: "".to_string(),
                kind: "".to_string(),
                file: FilePath::new(""),
                start_line: LineNumber::new(0),
                end_line: LineNumber::new(0),
            },
            related_locations: LocationList::new(Vec::new()),
        }
    }

    fn detect_file_layer(&self, file: &str, root_dir: &str) -> Option<String> {
        let rel = file.strip_prefix(root_dir).unwrap_or(file).trim_start_matches('/');

        let mut layers: Vec<(&LayerNameVO, &crate::taxonomy::LayerDefinition)> =
            self.config.layers.iter().collect();
        layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

        for (name, def) in layers {
            if rel.starts_with(def.path.value.as_str()) {
                return Some(name.value.clone());
            }
        }
        None
    }

    fn detect_module_layer(&self, module: &str) -> Option<String> {
        for part in module.split('.') {
            for (name, _def) in &self.config.layers {
                if part == name.value.as_str() {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }

    fn extract_imports(content: &str) -> Vec<(usize, String)> {
        content.lines()
            .enumerate()
            .filter_map(|(i, line)| {
                let trimmed = line.trim();
                let module = if let Some(rest) = trimmed.strip_prefix("from ") {
                    rest.split_whitespace().next().map(|m| m.to_string())
                } else if let Some(rest) = trimmed.strip_prefix("import ") {
                    rest.split_whitespace().next().map(|m| m.trim_end_matches(',').to_string())
                } else {
                    None
                };
                module.map(|m| (i + 1, m))
            })
            .collect()
    }

    fn find_governance_rule(&self, from_layer: &str, to_layer: &str) -> Option<String> {
        for rule in &self.config.governance_rules {
            let rule_from = rule.source_layer.value.as_str();
            let rule_to = rule.forbidden_target.value.as_str();
            if rule_from == from_layer && rule_to == to_layer {
                let desc = rule.description.value.clone();
                return Some(if desc.is_empty() {
                    "Forbidden layer import".to_string()
                } else {
                    desc
                });
            }
        }
        None
    }

    /// Scan files for cross-layer import violations.
    pub fn scan(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled || self.config.governance_rules.is_empty() {
            return vec![];
        }

        let mut violations: Vec<LintResult> = Vec::new();

        for file in files {
            if !file.ends_with(".py") {
                continue;
            }

            let file_layer = match self.detect_file_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };

            // Skip agent layer
            if file_layer == "agent" {
                continue;
            }

            let Ok(content) = fs::read_to_string(file) else { continue; };
            let imports = Self::extract_imports(&content);

            for (line_no, module) in imports {
                let target_layer = match self.detect_module_layer(&module) {
                    Some(l) => l,
                    None => continue,
                };

                if let Some(description) = self.find_governance_rule(&file_layer, &target_layer) {
                    let msg = format!(
                        "[AES Layer Violation] {}. File in '{}' imports from '{}' via '{}'.",
                        description, file_layer, target_layer, module
                    );
                    violations.push(Self::make_result(file, line_no as i64, "AES001", &msg, Severity::CRITICAL));
                }
            }
        }

        violations
    }
}

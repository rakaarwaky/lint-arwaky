// arch_import_checker — Import-related architectural checks.
// Implements IArchImportProtocol: check_mandatory_imports, check_forbidden_imports, check_legacy_import_rules.

use std::fs;
use std::path::Path;
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LayerNameVO, LintMessage, LintResult, LineNumber, Severity,
    ScopeRef, LocationList, ArchitectureConfig,
};

pub struct ArchImportRuleChecker;

impl ArchImportRuleChecker {
    pub fn new() -> Self {
        Self
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
        content.lines()
            .enumerate()
            .filter(|(_, line)| {
                let trimmed = line.trim();
                trimmed.starts_with("import ")
                    || trimmed.starts_with("from ")
                    || trimmed.starts_with("use ")
                    || trimmed.starts_with("extern crate ")
            })
            .map(|(i, l)| (i + 1, l.to_string()))
            .collect()
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
        // Rust: `use x::y;`
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';').to_string();
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
        if definition.mandatory_import.values.is_empty() {
            return;
        }

        let basename = Self::get_basename(file);
        if basename == "__init__.py" {
            return;
        }

        if definition.exceptions.values.contains(&basename) {
            return;
        }

        let import_lines = Self::read_import_lines(file);
        let Ok(content) = fs::read_to_string(file) else { return; };

        for required in &definition.mandatory_import.values {
            let is_present = content.contains(required.as_str())
                || import_lines.iter().any(|(_, l)| l.contains(required.as_str()));

            if !is_present {
                let msg = if !definition.mandatory_import_violation_message.value.is_empty() {
                    definition.mandatory_import_violation_message.value.clone()
                } else {
                    format!("AES002 MANDATORY_IMPORT: Missing required import: '{}'.", required)
                };
                violations.push(Self::make_result(file, 0, "AES002", &msg, Severity::HIGH));
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
        if definition.forbidden_import.values.is_empty() {
            return;
        }

        let import_lines = Self::read_import_lines(file);

        for (line_num, line) in &import_lines {
            if let Some(module) = Self::extract_module_from_line(line) {
                for forbidden in &definition.forbidden_import.values {
                    if module.contains(forbidden.as_str()) {
                        let msg = if !definition.forbidden_import_violation_message.value.is_empty() {
                            definition.forbidden_import_violation_message.value.clone()
                        } else {
                            format!(
                                "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden module '{}'.",
                                layer_name, module
                            )
                        };
                        violations.push(Self::make_result(file, *line_num as i64, "AES001", &msg, Severity::CRITICAL));
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
                    for rule in &config.governance_rules {
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
                            violations.push(Self::make_result(file, *line_num as i64, "AES001", &msg, Severity::CRITICAL));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn detect_module_layer(&self, module: &str, config: &ArchitectureConfig) -> Option<String> {
        let parts: Vec<&str> = module.split('.').collect();
        for part in &parts {
            for (name, def) in &config.layers {
                let path_last = def.path.value.split('/').last().unwrap_or("");
                if *part == name.value.as_str() || *part == path_last {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }
}

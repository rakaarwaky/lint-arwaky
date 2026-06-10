// PURPOSE: AES001 — Enforce forbidden import rules: definition-level, scope-specific, and legacy governance.
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::capabilities_import_utils::{
    extract_layer_from_import, extract_module_from_line, get_basename, import_matches_scope,
    read_import_lines, resolve_scope,
};
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
fn aes001_forbidden_import(layer_name: &str, module: &str) -> String {
    format!(
        "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden module '{}'.",
        layer_name, module
    )
}

pub struct ArchImportForbiddenChecker {}

impl Default for ArchImportForbiddenChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }

        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec![
                "agent".to_string(),
                "infrastructure".to_string(),
                "capabilities".to_string(),
                "contract(port)".to_string(),
                "contract(protocol)".to_string(),
            ]
        };

        let import_lines = read_import_lines(file);
        for (line_num, line) in &import_lines {
            if let Some(module) = extract_module_from_line(line) {
                let segments: Vec<&str> = module.split("::").collect();
                for forbidden in &forbidden_list {
                    let (layer, suffixes) = resolve_scope(forbidden);
                    let is_forbidden = if suffixes.is_empty() {
                        segments.iter().any(|seg| {
                            let cleaned = seg.trim_end_matches(';').trim();
                            extract_layer_from_import(cleaned)
                                .map(|l| l == layer)
                                .unwrap_or(false)
                        })
                    } else {
                        import_matches_scope(line, layer, &suffixes)
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

    pub fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = get_basename(file);
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(&basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = read_import_lines(file);
        if import_lines.is_empty() {
            return;
        }

        for rule in &config.rules {
            let (rule_layer, rule_suffixes) = resolve_scope(&rule.scope.value);
            let layer_match = stem.starts_with(&format!("{}_", rule_layer));
            if !layer_match {
                continue;
            }
            if !rule_suffixes.is_empty() && !rule_suffixes.contains(&suffix) {
                continue;
            }
            for (line_num, line) in &import_lines {
                if let Some(module) = extract_module_from_line(line) {
                    let segments: Vec<&str> = module.split("::").collect();
                    for forbidden in &rule.forbidden.values {
                        let (forbidden_layer, forbidden_suffixes) = resolve_scope(forbidden);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = seg.trim_end_matches(';').trim();
                                extract_layer_from_import(cleaned)
                                    .map(|l| l == forbidden_layer)
                                    .unwrap_or(false)
                            })
                        } else {
                            import_matches_scope(line, forbidden_layer, &forbidden_suffixes)
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
        if file_layer == "agent" {
            return;
        }

        let import_lines = read_import_lines(file);
        for (line_num, line) in &import_lines {
            if let Some(module) = extract_module_from_line(line) {
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
        let parts: Vec<&str> = if module.contains("::") {
            module.split("::").collect()
        } else {
            module.split('.').collect()
        };
        for part in &parts {
            if let Some(layer) = extract_layer_from_import(part) {
                return Some(layer);
            }
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
}

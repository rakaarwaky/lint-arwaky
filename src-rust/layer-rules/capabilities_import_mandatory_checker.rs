// PURPOSE: ArchImportMandatoryChecker — AES002: enforce mandatory import rules per layer definition and scope rules
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::contract_import_parser_port::ImportParser;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
fn aes002_mandatory_import(required: &str) -> String {
    format!(
        "AES002 MANDATORY_IMPORT: Missing required import: '{}'.",
        required
    )
}

pub struct ArchImportMandatoryChecker {}

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check mandatory imports from layer definition (legacy path).
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }

        let basename = ImportParser::get_basename(file);
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename) {
            return;
        }

        let Ok(content) = std::fs::read_to_string(file) else {
            return;
        };
        let import_lines = ImportParser::parse_import_lines(&content);

        for required in &definition.mandatory.values {
            let (layer, suffixes) = ImportParser::resolve_scope(required);
            let is_present = if suffixes.is_empty() {
                import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| ImportParser::import_matches_scope(l, layer, &suffixes))
            };

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

    /// Check mandatory imports from config rules (AES001 conditions per scope).
    /// This is the primary path — reads mandatory from rules.AES001.conditions.
    pub fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        eprintln!("[DEBUG AES002] ENTERED check_scope_mandatory_imports for file: {}", file);
        let basename = ImportParser::get_basename(file);
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(&basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = ImportParser::read_import_lines(file);

        // Debug: print rules with mandatory imports
        for rule in &config.rules {
            if !rule.mandatory.values.is_empty() {
                eprintln!("[DEBUG AES002] Rule: {} - Scope: {} - Mandatory: {:?}", 
                    rule.name.value, rule.scope.value, rule.mandatory.values);
            }
        }

        for rule in &config.rules {
            // Only check rules that have mandatory imports defined
            if rule.mandatory.values.is_empty() {
                continue;
            }

            let (rule_layer, rule_suffixes) = ImportParser::resolve_scope(&rule.scope.value);
            let layer_match = stem.starts_with(&format!("{}_", rule_layer));
            if !layer_match {
                eprintln!("[DEBUG AES002] File {} - stem {} doesn't match rule_layer {}", file, stem, rule_layer);
                continue;
            }
            if !rule_suffixes.is_empty() && !rule_suffixes.contains(&suffix) {
                continue;
            }

            eprintln!("[DEBUG AES002] Checking file {} against rule {}", file, rule.name.value);
            for required in &rule.mandatory.values {
                let (req_layer, req_suffixes) = ImportParser::resolve_scope(required);
                let is_present = if req_suffixes.is_empty() {
                    if import_lines.is_empty() {
                        false
                    } else {
                        import_lines.iter().any(|(_, l)| l.contains(req_layer))
                    }
                } else {
                    import_lines
                        .iter()
                        .any(|(_, l)| ImportParser::import_matches_scope(l, req_layer, &req_suffixes))
                };

                eprintln!("[DEBUG AES002] Required: {} - is_present: {}", required, is_present);
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
    }
}

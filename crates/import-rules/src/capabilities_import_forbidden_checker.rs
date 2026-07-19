// PURPOSE: ImportForbiddenChecker — AES201: enforce forbidden import rules via layer definition and scoped rules
// AES201 rule: Each architectural layer defines which other layers it must NOT import from.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::LineContentVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_import_constant::RUST_ENTRY_FILES;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::collections::HashSet;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ImportForbiddenChecker {
    parser: Arc<dyn IImportParserPort>,
}

// ─── Block 2: Public Contract (IImportForbiddenProtocol) ───

#[async_trait]
impl IImportForbiddenProtocol for ImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let config = analyzer.config();

        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value.as_str() == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let default_surface_forbidden = vec![
            "agent".to_string(),
            "infrastructure".to_string(),
            "capabilities".to_string(),
        ];

        let mut processed_violations: HashSet<(String, usize, String)> = HashSet::new();

        let _root_dir_str = root_dir.to_string();

        for f in &files.values {
            let basename = f.basename();

            if aes201_exceptions.contains(&basename) {
                continue;
            }

            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                let layer_str = layer.clone();
                if let Some(def) = analyzer.get_layer_def(&layer_str) {
                    self.check_forbidden_imports_layer(
                        f,
                        &layer_str.value,
                        &def,
                        &default_surface_forbidden,
                        &mut results.values,
                        &mut processed_violations,
                    );
                }
            }

            self.check_scope_forbidden_imports(
                f,
                config,
                &mut results.values,
                &mut processed_violations,
            );
        }
    }
}

// ─── Block 3: Constructors & Private Helpers ───

impl ImportForbiddenChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Check forbidden imports from layer definition (global layer rules).
    pub fn check_forbidden_imports_layer(
        &self,
        file_path: &FilePath,
        layer_name: &str,
        definition: &LayerDefinition,
        default_forbidden: &[String],
        violations: &mut Vec<LintResult>,
        processed: &mut HashSet<(String, usize, String)>,
    ) {
        let basename = file_path.basename();

        if definition
            .exceptions
            .values
            .iter()
            .any(|ex| ex == &basename)
        {
            return;
        }

        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }

        let forbidden_list = if !definition.forbidden.values.is_empty() {
            &definition.forbidden.values
        } else {
            default_forbidden
        };

        let import_lines = self.parser.read_import_lines(file_path);
        if import_lines.is_empty() {
            return;
        }

        let layer_name_vo = LayerNameVO::new(layer_name);
        let rule_config =
            shared::import_rules::contract_import_forbidden_protocol::ForbiddenRuleConfig {
                forbidden_list,
                source_layer: &layer_name_vo,
                allowed_values: definition.allowed.values.as_slice(),
            };
        self.check_imports_against_forbidden(
            file_path,
            &import_lines,
            &rule_config,
            violations,
            processed,
        );
    }

    /// Check forbidden imports from per-rule scope definitions.
    pub fn check_scope_forbidden_imports(
        &self,
        file_path: &FilePath,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
        processed: &mut HashSet<(String, usize, String)>,
    ) {
        let basename_identity = self.parser.get_basename(file_path);
        let basename = basename_identity.value();

        if RUST_ENTRY_FILES.contains(&basename) {
            return;
        }

        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = self.parser.read_import_lines(file_path);
        if import_lines.is_empty() {
            return;
        }

        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }

            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();

            if !stem.starts_with(rule_layer_str) {
                continue;
            }
            if stem.len() > rule_layer_str.len() && stem.as_bytes()[rule_layer_str.len()] != b'_' {
                continue;
            }

            if !rule_suffixes.is_empty() && !rule_suffixes.iter().any(|s| s.value() == suffix) {
                continue;
            }

            let rule_config =
                shared::import_rules::contract_import_forbidden_protocol::ForbiddenRuleConfig {
                    forbidden_list: &rule.forbidden.values,
                    source_layer: &rule_layer,
                    allowed_values: rule.allowed.values.as_slice(),
                };
            self.check_imports_against_forbidden(
                file_path,
                &import_lines,
                &rule_config,
                violations,
                processed,
            );
        }
    }

    /// Core: scan import lines against a list of forbidden layers and build violations.
    fn check_imports_against_forbidden(
        &self,
        file_path: &FilePath,
        import_lines: &[(LineNumber, LineContentVO)],
        rule: &shared::import_rules::contract_import_forbidden_protocol::ForbiddenRuleConfig<'_>,
        violations: &mut Vec<LintResult>,
        processed: &mut HashSet<(String, usize, String)>,
    ) {
        if import_lines.is_empty() || rule.forbidden_list.is_empty() {
            return;
        }

        let file_str = file_path.to_string();

        let resolved_forbidden: Vec<(&String, LayerNameVO, Vec<Identity>)> = rule
            .forbidden_list
            .iter()
            .map(|forbidden| {
                let identity = Identity::new(forbidden);
                let (layer, suffixes) = self.parser.resolve_scope(&identity);
                (forbidden, layer, suffixes)
            })
            .collect();

        let resolved_allowed_strs: Vec<String> = rule
            .allowed_values
            .iter()
            .map(|s| {
                self.parser
                    .resolve_scope(&Identity::new(s))
                    .0
                    .value()
                    .to_string()
            })
            .collect();

        let resolved_allowed_vos: Vec<LayerNameVO> = resolved_allowed_strs
            .iter()
            .map(|s| LayerNameVO::new(s.clone()))
            .collect();

        for (line_num, line) in import_lines {
            let Some(module) = self.parser.extract_module_from_line(line) else {
                continue;
            };

            let module_str = module.value();
            let segments: Vec<&str> = module_str
                .split([':', '.', '/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();

            let line_val = line_num.value() as usize;

            for (forbidden_name, forbidden_layer, forbidden_suffixes) in &resolved_forbidden {
                let violation_key = (file_str.clone(), line_val, "AES201".to_string());
                if processed.contains(&violation_key) {
                    continue;
                }

                let is_explicitly_allowed = resolved_allowed_strs.iter().any(|allowed| {
                    module_str.contains(allowed) || segments.contains(&allowed.as_str())
                });

                if is_explicitly_allowed {
                    continue;
                }

                let is_forbidden = if forbidden_suffixes.is_empty() {
                    segments.iter().any(|seg| {
                        let cleaned = seg.trim_end_matches(';').trim();
                        match self
                            .parser
                            .extract_layer_from_import(&Identity::new(cleaned))
                        {
                            Some(l) => l == *forbidden_layer,
                            None => false,
                        }
                    })
                } else {
                    self.parser
                        .import_matches_scope(line, forbidden_layer, forbidden_suffixes)
                };

                if is_forbidden {
                    processed.insert(violation_key);

                    violations.push(LintResult::new_arch(
                        &file_str,
                        line_val,
                        "AES201",
                        Severity::CRITICAL,
                        AesImportViolation::ForbiddenImport {
                            source_layer: rule.source_layer.clone(),
                            forbidden_layer: LayerNameVO::new(forbidden_name.to_string()),
                            allowed: resolved_allowed_vos.clone(),
                            reason: None,
                        }
                        .to_string(),
                    ));

                    break;
                }
            }
        }
    }
}

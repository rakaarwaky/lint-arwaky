// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules per layer definition and scope rules
// AES202 rule: Each architectural layer (or scoped sub-layer) may declare a set of mandatory imports.
// Files belonging to that layer MUST import at least one symbol from each required scope.

use crate::utils_import::filepath_or_default;
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_import_constant::{PYTHON_ENTRY_FILES, RUST_ENTRY_FILES};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// Enforces AES202 mandatory import rules — both layer-level and scope-level.
pub struct ArchImportMandatoryChecker {
    parser: Arc<dyn IImportParserPort>,
}

// ─── Block 2: Public Contract (IImportMandatoryProtocol) ───

#[async_trait]
impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    async fn run_mandatory_imports(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();

            let mut is_exception = false;
            for r in &analyzer.config().rules {
                if r.name.value.as_str() == "AES202" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            let _root_dir_str = root_dir.to_string();
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                if let Some(def) = analyzer.get_layer_def(&layer) {
                    self.check_mandatory_imports(&f_str, &def, &mut results.values);
                }
            }
            self.check_scope_mandatory_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }
}

// ─── Block 3: Constructors & Private Helpers ───

impl ArchImportMandatoryChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Check mandatory imports from layer definition (global layer rules).
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if PYTHON_ENTRY_FILES.contains(&basename) {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let Ok(content_msg) = self.parser.read_file_to_message(&file_path) else {
            return;
        };
        let content = content_msg.value().to_string();
        let file_content = FileContentVO::new(content);
        let import_lines = self.parser.parse_import_lines(&file_content);

        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = self.parser.resolve_scope(&required_identity);
            let layer_str = layer.value();

            let is_present = if suffixes.is_empty() {
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| self.parser.import_matches_scope(l, &layer, &suffixes))
            };

            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES202",
                    Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: None,
                    }
                    .to_string(),
                ));
            }
        }
    }

    /// Check mandatory imports from per-rule scope definitions.
    pub fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if RUST_ENTRY_FILES.contains(&basename) {
            return;
        }
        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = self.parser.read_import_lines(&file_path);

        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }

            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();
            let layer_match = stem.starts_with(&format!("{}_", rule_layer_str));
            if !layer_match {
                continue;
            }
            if !rule_suffixes.is_empty() {
                let suffix_match = rule_suffixes.iter().any(|s| s.value() == suffix);
                if !suffix_match {
                    continue;
                }
            }

            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) = self.parser.resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();

                let is_present = if req_suffixes.is_empty() {
                    if import_lines.is_empty() {
                        false
                    } else {
                        import_lines
                            .iter()
                            .any(|(_, l)| l.value().contains(req_layer_str))
                    }
                } else {
                    import_lines.iter().any(|(_, l)| {
                        self.parser
                            .import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };

                if !is_present {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES202",
                        Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: rule_layer.clone(),
                            required: SymbolName::new(required.clone()),
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

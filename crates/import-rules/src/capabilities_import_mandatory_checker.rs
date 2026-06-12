// PURPOSE: ArchImportMandatoryChecker — AES002: enforce mandatory import rules per layer definition and scope rules
use async_trait::async_trait;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::output_report::taxonomy_result_vo::{LintResult, LintResultList};
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_violation_message::AesViolation;
use std::sync::Arc;

pub struct ArchImportMandatoryChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl ArchImportMandatoryChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
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

        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let Ok(content) = std::fs::read_to_string(file) else {
            return;
        };
        let file_content = FileContentVO::new(content);
        let import_lines = self.parser.parse_import_lines(&file_content);

        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let source_layer = stem.split('_').next().unwrap_or("unknown");

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
                    "AES002",
                    Severity::HIGH,
                    AesViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: None,
                    },
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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
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
                        "AES002",
                        Severity::HIGH,
                        AesViolation::MissingImport {
                            source_layer: rule_layer.clone(),
                            required: SymbolName::new(required.clone()),
                            reason: None,
                        },
                    ));
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES002")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportMandatoryChecker {
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_mandatory_imports(&f_str, def, &mut results.values);
                }
            }
            self.check_scope_mandatory_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }

    async fn check_legacy_import_rules(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }
}

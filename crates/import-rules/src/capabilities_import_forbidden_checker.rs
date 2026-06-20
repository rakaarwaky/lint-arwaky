// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules via layer definition and scoped rules
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::sync::Arc;

pub struct ArchImportForbiddenChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl ArchImportForbiddenChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
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
            ]
        };

        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let import_lines = self.parser.read_import_lines(&file_path);
        let layer_name_vo = LayerNameVO::new(layer_name);
        for (line_num, line) in &import_lines {
            if let Some(module) = self.parser.extract_module_from_line(line) {
                let segments: Vec<&str> = module
                    .value()
                    .split(|c: char| c == ':' || c == '.' || c == '/' || c == '\\')
                    .filter(|s| !s.is_empty())
                    .collect();
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) = self.parser.resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        segments.iter().any(|seg| {
                            let cleaned = seg.trim_end_matches(';').trim();
                            let cleaned_identity = Identity::new(cleaned);
                            self.parser
                                .extract_layer_from_import(&cleaned_identity)
                                .map(|l| l == layer)
                                .unwrap_or(false)
                        })
                    } else {
                        self.parser.import_matches_scope(line, &layer, &suffixes)
                    };
                    if is_forbidden {
                        let allowed: Vec<LayerNameVO> = definition
                            .allowed
                            .values
                            .iter()
                            .map(|s| {
                                LayerNameVO::new(
                                    self.parser
                                        .resolve_scope(&Identity::new(s))
                                        .0
                                        .value()
                                        .to_string(),
                                )
                            })
                            .collect();
                        violations.push(LintResult::new_arch(
                            file,
                            line_num.value() as usize,
                            "AES201",
                            Severity::CRITICAL,
                            AesImportViolation::ForbiddenImport {
                                source_layer: layer_name_vo.clone(),
                                forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                allowed,
                                reason: None,
                            }
                            .to_string(),
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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let import_lines = self.parser.read_import_lines(&file_path);
        if import_lines.is_empty() {
            return;
        }

        for rule in &config.rules {
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
            for (line_num, line) in &import_lines {
                if let Some(module) = self.parser.extract_module_from_line(line) {
                    let segments: Vec<&str> = module
                        .value()
                        .split(|c: char| c == ':' || c == '.' || c == '/' || c == '\\')
                        .filter(|s| !s.is_empty())
                        .collect();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            self.parser.resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = seg.trim_end_matches(';').trim();
                                let cleaned_identity = Identity::new(cleaned);
                                self.parser
                                    .extract_layer_from_import(&cleaned_identity)
                                    .map(|l| l == forbidden_layer)
                                    .unwrap_or(false)
                            })
                        } else {
                            self.parser.import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        self.parser
                                            .resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: rule_layer.clone(),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: None,
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportForbiddenChecker {
    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }

    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                let layer_str = layer.value();
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_forbidden_imports(&f_str, layer_str, def, &mut results.values);
                }
            }
            self.check_scope_forbidden_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }
}

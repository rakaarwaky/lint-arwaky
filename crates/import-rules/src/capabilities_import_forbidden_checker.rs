use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_file;
use shared::common::utility_file_handler;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};

// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Pre-compute layer_keys once per audit run (was previously per-file)
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();
            let mut is_exception = false;
            for r in &config.rules {
                if r.name.value.as_str() == "AES201" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            let filename = utility_layer_detector::extract_filename(&f_str);
            if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename) {
                let specialized = utility_layer_detector::resolve_specialized_layer(
                    &base_layer,
                    &f_str,
                    &layer_keys,
                );
                let layer_name = LayerNameVO::new(specialized.as_str());
                if let Some(def) = layer_map.values.get(&layer_name) {
                    self._check_forbidden_imports(&f_str, &specialized, def, &mut results.values);
                }
            }
            self._check_scope_forbidden_imports(&f_str, config, &mut results.values);
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportForbiddenChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }
        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec!["agent".into(), "capabilities".into()]
        };

        let content = match utility_file_handler::read_file_generic(file).ok() {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in &import_lines {
            if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                let segments: Vec<&str> = module
                    .value()
                    .split([':', '.', '/', '\\'])
                    .filter(|s| !s.is_empty())
                    .collect();
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) =
                        utility_import_resolver::resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        segments.iter().any(|seg| {
                            let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                            match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                    } else {
                        utility_import_resolver::import_matches_scope(line, &layer, &suffixes)
                    };
                    if is_forbidden {
                        let allowed: Vec<LayerNameVO> = definition
                            .allowed
                            .values
                            .iter()
                            .map(|s| {
                                LayerNameVO::new(
                                    utility_import_resolver::resolve_scope(&Identity::new(s))
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

    fn _check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        let content = match utility_file_handler::read_file_generic(file).ok() {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
        if import_lines.is_empty() {
            return;
        }

        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            // Use shared scope-matching utility to check if file belongs to scope
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename.as_str(),
                    &Identity::new(&rule.scope.value),
                )
            else {
                continue;
            };

            for (line_num, line) in &import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let segments: Vec<&str> = module
                        .value()
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .collect();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                    Some(l) => l == forbidden_layer,
                                    None => false,
                                }
                            })
                        } else {
                            utility_import_resolver::import_matches_scope(
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
                                        utility_import_resolver::resolve_scope(&Identity::new(s))
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
                                    source_layer: LayerNameVO::new(rule_layer_str.clone()),
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

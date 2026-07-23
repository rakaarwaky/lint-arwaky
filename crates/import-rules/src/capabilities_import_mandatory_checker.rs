use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;

// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    async fn run_mandatory_imports(
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
                if r.name.value.as_str() == "AES202" && r.exceptions.values.contains(&basename) {
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
                    self._check_mandatory_imports(&f_str, def, &mut results.values);
                }
            }
            self._check_scope_mandatory_imports(&f_str, config, &mut results.values);
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let content = match shared::common::utility_file_handler::read_file_generic(file).ok() {
            Some(c) => c,
            None => return,
        };
        let file_content = FileContentVO::new(content);
        let import_lines: Vec<(LineNumber, LineContentVO)> =
            utility_import_resolver::parse_import_lines_helper(file_content.value());
        let stem: &str = basename
            .rsplit('.')
            .next_back()
            .map_or(basename.as_str(), |s| s);
        let source_layer: &str = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let layer_str: &str = layer.value();
            let is_present: bool = if suffixes.is_empty() {
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                import_lines.iter().any(|(_, l)| {
                    utility_import_resolver::import_matches_scope(l, &layer, &suffixes)
                })
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

    fn _check_scope_mandatory_imports(
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

        let content = match shared::common::utility_file_handler::read_file_generic(file).ok() {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);

        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            // Use shared utility to check if file belongs to scope
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename.as_str(),
                    &scope_identity,
                )
            else {
                continue;
            };

            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) =
                    utility_import_resolver::resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();
                let is_present = if req_suffixes.is_empty() {
                    import_lines
                        .iter()
                        .any(|(_, l)| l.value().contains(req_layer_str))
                } else {
                    import_lines.iter().any(|(_, l)| {
                        utility_import_resolver::import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };
                if !is_present {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES202",
                        Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: LayerNameVO::new(rule_layer_str.clone()),
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

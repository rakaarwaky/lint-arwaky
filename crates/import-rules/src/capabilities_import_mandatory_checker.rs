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
use std::collections::HashSet;

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
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes202_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };
                let file_content = FileContentVO::new(content);
                let import_lines: Vec<(LineNumber, LineContentVO)> =
                    utility_import_resolver::parse_import_lines_helper(file_content.value());

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer,
                        &f_str,
                        &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_mandatory_imports_with_lines(
                            &f_str,
                            &basename,
                            def,
                            &import_lines,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_mandatory_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        results.values.extend(file_violations);
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

    fn _check_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() || basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let stem: &str = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer: &str = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let layer_str: &str = layer.value();
            let is_present = if suffixes.is_empty() {
                // First try: direct match (existing behavior)
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                // First try: scope match (existing behavior)
                import_lines.iter().any(|(_, l)| {
                    utility_import_resolver::import_matches_scope(l, &layer, &suffixes)
                })
            };

            // Fallback: barrel resolution — when direct match fails, try resolving
            // through barrel files to detect contracts exported via __init__.py / mod.rs
            let is_present = is_present || self._check_barrel_mandatory_imports(
                file,
                import_lines,
                &layer,
                &suffixes,
                layer_str,
            );

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

    fn _check_scope_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(LineNumber, LineContentVO)],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
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

    /// Check if any import line resolves to a contract (or required layer) through barrel files.
    ///
    /// Handles cases like `from modules.shared.src.asset import AssetSearchProtocol` where
    /// the barrel file `modules/shared/src/asset/__init__.py` re-exports contract protocols.
    fn _check_barrel_mandatory_imports(
        &self,
        file: &str,
        import_lines: &[(LineNumber, LineContentVO)],
        layer: &LayerNameVO,
        suffixes: &[Identity],
        layer_str: &str,
    ) -> bool {
        // Extract workspace root for barrel resolution
        let root_dir = shared::common::utility_file_handler::find_workspace_root(file)
            .and_then(|p| Some(p.to_string_lossy().to_string()))
            .unwrap_or_else(|| ".".to_string());

        for (_, line) in import_lines {
            let line_val = line.value();

            // Extract module path from import line
            if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                let module_val = module.value();

                // Try barrel resolution — look for __init__.py / mod.rs / index.ts
                if let Some(barrel_path) = utility_import_resolver::find_barrel_file(&module_val, &root_dir) {
                    if let Ok(barrel_content) = std::fs::read_to_string(&barrel_path) {
                        let reexports = utility_import_resolver::parse_barrel_reexports(&barrel_content);

                        // Check if any imported symbol resolves to the required layer
                        for (_, import_part) in line_val.split_once("import ") {
                            for name in import_part.split(',') {
                                let name = name.trim();
                                if name.is_empty() || name == "*" {
                                    continue;
                                }
                                let symbol_name = match name.split_once(" as ") {
                                    Some((_, alias)) => alias.trim(),
                                    None => name.split_whitespace().next().unwrap_or(""),
                                };

                                if let Some(resolved_source) = reexports.get(symbol_name) {
                                    // Extract file name from resolved source for layer detection
                                    let resolved_file = resolved_source
                                        .rsplit('/')
                                        .next()
                                        .or_else(|| resolved_source.rsplit("::").next())
                                        .unwrap_or(resolved_source);

                                    // Check if resolved file has the required layer prefix
                                    if let Some(resolved_layer) = utility_layer_detector::detect_layer_from_prefix(resolved_file) {
                                        if resolved_layer == layer_str || !suffixes.is_empty() {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

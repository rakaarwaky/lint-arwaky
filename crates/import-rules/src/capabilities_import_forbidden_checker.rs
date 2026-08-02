// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.
//
// Barrel resolution: when direct module-path matching fails (e.g. import
// through __init__.py / mod.rs / index.ts hides the original file name),
// resolves each imported symbol through the barrel file to detect the
// original source file and its layer prefix.

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{
    FilePath, FilePathList, Identity, LineContentVO, LineNumber, LintMessage, Severity,
};

use crate::utility_import_resolver;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }

    fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let root_dir_str = root_dir.to_string();

        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes201_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content = match content_map.get(&f_str) {
                    Some(c) => c.clone(),
                    None => return Vec::new(),
                };
                let import_lines =
                    utility_import_resolver::parse_import_lines_helper(&f_str, &content);
                if import_lines.is_empty() {
                    return Vec::new();
                }

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
                        self._check_forbidden_imports_with_lines(
                            &f_str,
                            &specialized,
                            def,
                            &import_lines,
                            &root_dir_str,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_forbidden_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        Ok(LintResultList::new(file_violations))
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

    fn _check_forbidden_imports_with_lines(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
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

        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in import_lines {
            let module = match utility_import_resolver::extract_module_from_line(line) {
                Some(m) => m,
                None => continue,
            };
            let module_val = module.value();

            let symbol_names = utility_import_resolver::extract_symbol_names(line.value());

            for forbidden in &forbidden_list {
                let forbidden_identity = Identity::new(forbidden);
                let (layer, suffixes) = utility_import_resolver::resolve_scope(&forbidden_identity);

                let mut is_forbidden = if suffixes.is_empty() {
                    module_val
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .any(|seg| {
                            let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                            match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                } else {
                    utility_import_resolver::import_matches_scope(line, &layer, &suffixes)
                };

                if !is_forbidden {
                    for sym in &symbol_names {
                        if let Some(resolved) = utility_import_resolver::resolve_barrel_import(
                            module_val, sym, root_dir,
                        ) && resolved.matches_layer(layer.value())
                            && (suffixes.is_empty()
                                || suffixes.iter().any(|s| resolved.has_suffix(s.value())))
                        {
                            is_forbidden = true;
                            break;
                        }
                    }
                }

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
                            reason: Some(LintMessage::new(format!(
                                "File imports from '{}' which resolves to forbidden layer '{}'. Source file is in layer '{}'.",
                                module_val, forbidden, layer_name
                            ))),
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }

    fn _check_scope_forbidden_imports_with_lines(
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
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &Identity::new(&rule.scope.value),
                )
            else {
                continue;
            };

            for (line_num, line) in import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let module_val = module.value();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            module_val
                                .split([':', '.', '/', '\\'])
                                .filter(|s| !s.is_empty())
                                .any(|seg| {
                                    let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                    match utility_import_resolver::extract_layer_from_import(
                                        &cleaned,
                                    ) {
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
                                    reason: Some(LintMessage::new(format!(
                                        "Scope rule violation: file imports from '{}' which resolves to forbidden layer '{}'.",
                                        module_val, forbidden
                                    ))),
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

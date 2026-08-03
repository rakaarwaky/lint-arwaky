// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses ImportEntry fields directly — no text-based parsing, no bridge functions.

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{FilePath, FilePathList, Identity, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

use crate::utility_import_resolver;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let mut file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes202_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let entries = match imports_map.get(&f_str) {
                    Some(e) => e.as_slice(),
                    None => return Vec::new(),
                };

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
                        self._check_mandatory_imports(
                            &f_str,
                            &basename,
                            def,
                            entries,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_mandatory_imports(
                    &f_str,
                    &basename,
                    config,
                    entries,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        // Deduplicate violations by (file, line, code)
        file_violations.dedup_by(|a, b| a.file == b.file && a.line == b.line && a.code == b.code);

        Ok(LintResultList::new(file_violations))
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
        basename: &str,
        definition: &LayerDefinition,
        entries: &[ImportEntry],
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

            let is_present = entries.iter().any(|entry| {
                utility_import_resolver::entry_matches_scope(entry, &layer, &suffixes)
            }) || self
                ._check_barrel_mandatory(entries, &layer, &suffixes, layer_str);

            if !is_present {
                let v = LintResult::new_arch(
                    file,
                    1,
                    "AES202",
                    Severity::HIGH,
                    format!(
                        "AES202 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                            WHY? Layer '{}' must import '{}' to satisfy architectural requirements.\n\
                            FIX: Add the required import statement.",
                        source_layer, required, source_layer, required
                    ),
                );
                if !violations.contains(&v) {
                    violations.push(v);
                }
            }
        }
    }

    fn _check_scope_mandatory_imports(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        entries: &[ImportEntry],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.name.value != "AES202" && rule.rule_type.code() != "AES202" {
                continue;
            }

            if !rule.mandatory.values.is_empty() {
                let scope_identity = Identity::new(&rule.scope.value);
                if let Some((rule_layer_str, _rule_suffixes)) =
                    shared::common::utility_scope_matcher::file_belongs_to_scope(
                        basename,
                        &scope_identity,
                    )
                {
                    for required in &rule.mandatory.values {
                        self._check_single_scope_requirement(
                            file,
                            basename,
                            &rule_layer_str,
                            required,
                            entries,
                            violations,
                        );
                    }
                }
            }
        }
    }

    fn _check_single_scope_requirement(
        &self,
        file: &str,
        _basename: &str,
        rule_layer_str: &str,
        required: &str,
        entries: &[ImportEntry],
        violations: &mut Vec<LintResult>,
    ) {
        let required_identity = Identity::new(required);
        let (req_layer, req_suffixes) = utility_import_resolver::resolve_scope(&required_identity);
        let req_layer_str = req_layer.value();

        let is_present =
            entries.iter().any(|entry| {
                utility_import_resolver::entry_matches_scope(entry, &req_layer, &req_suffixes)
            }) || self._check_barrel_mandatory(entries, &req_layer, &req_suffixes, req_layer_str);

        if !is_present {
            let v = LintResult::new_arch(
                file,
                1,
                "AES202",
                Severity::HIGH,
                format!(
                    "AES202 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                        WHY? Layer '{}' must import '{}' to satisfy architectural requirements.\n\
                        FIX: Add the required import statement.",
                    rule_layer_str, required, rule_layer_str, required
                ),
            );
            if !violations.contains(&v) {
                violations.push(v);
            }
        }
    }

    fn _check_barrel_mandatory(
        &self,
        entries: &[ImportEntry],
        _layer: &LayerNameVO,
        suffixes: &[Identity],
        layer_str: &str,
    ) -> bool {
        for entry in entries {
            // Use resolved_path from filesystem's barrel resolution
            if let Some(ref resolved_path) = entry.resolved_path {
                let resolved_file = resolved_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                let resolved_layer =
                    crate::utility_path_normalizer::extract_layer_from_prefix(&resolved_file);
                let layer_matches = resolved_layer.as_deref() == Some(layer_str);
                let suffix_matches = suffixes.is_empty()
                    || suffixes.iter().any(|s| {
                        let suffix_lower = s.value().to_lowercase();
                        resolved_file
                            .to_lowercase()
                            .contains(&format!("_{}", suffix_lower))
                    });
                if layer_matches && suffix_matches {
                    return true;
                }
            }
        }
        false
    }
}

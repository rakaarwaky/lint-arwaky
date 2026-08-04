// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses ImportEntry fields directly — no text-based parsing, no bridge functions.
//
// Barrel resolution: when direct module-path matching fails (e.g. import
// through __init__.py / mod.rs / index.ts hides the original file name),
// resolves each imported symbol through the barrel file to detect the
// original source file and its layer prefix.

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{FilePath, FilePathList, Identity, Severity};
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::{AstImportVO, FileParseResultVO};
use shared::common::parse_file_content;

use crate::utility_import_resolver;
use crate::utility_path_normalizer;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
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
        _root_dir: &FilePath,
        _content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

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

                let entries = match imports_map.get(&f_str) {
                    Some(e) if !e.is_empty() => e,
                    _ => return Vec::new(),
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
                        self._check_forbidden_imports(
                            &f_str,
                            &specialized,
                            def,
                            entries,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_forbidden_imports(
                    &f_str,
                    &basename,
                    config,
                    entries,
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

/// Build ImportEntry list from file content — test helper.
pub fn build_test_import_entries(file_path: &str, content: &str) -> Vec<ImportEntry> {
    let language = if file_path.ends_with(".rs") {
        Language::Rust
    } else if file_path.ends_with(".py") {
        Language::Python
    } else {
        Language::TypeScript
    };
    let mut entries = Vec::new();
    let push = |entries: &mut Vec<ImportEntry>, imp: &AstImportVO| {
        entries.push(ImportEntry {
            source_file: std::path::PathBuf::new(),
            raw_path: imp.raw_path.clone(),
            resolved_path: None,
            import_type: if imp.is_reexport {
                ImportType::ReExport
            } else {
                ImportType::Use
            },
            language,
            is_dynamic: false,
            is_resolved: false,
            symbols: Vec::new(),
            is_reexport: imp.is_reexport,
            is_wildcard: imp.is_glob,
        });
    };
    match parse_file_content(file_path, content) {
        FileParseResultVO::Rust(r) => r.imports.iter().for_each(|i| push(&mut entries, i)),
        FileParseResultVO::Python(p) => p.imports.iter().for_each(|i| push(&mut entries, i)),
        FileParseResultVO::TypeScript(t) => t.imports.iter().for_each(|i| push(&mut entries, i)),
        FileParseResultVO::Unsupported => {}
    }
    entries
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self
    }

    /// Check a single file's content for forbidden imports — test helper.
    pub fn check_single_file(
        &self,
        file_path: &str,
        content: &str,
        root_dir: &str,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
    ) -> Result<LintResultList, ImportError> {
        let Ok(fp) = FilePath::new(file_path.to_string()) else {
            return Ok(LintResultList::new(vec![]));
        };
        let Ok(root) = FilePath::new(root_dir.to_string()) else {
            return Ok(LintResultList::new(vec![]));
        };
        let files = FilePathList::new(vec![fp]);
        let mut content_map = HashMap::new();
        content_map.insert(file_path.to_string(), content.to_string());
        let entries = build_test_import_entries(file_path, content);
        let mut imports_map = HashMap::new();
        imports_map.insert(file_path.to_string(), entries);
        self.check_forbidden_imports(config, layer_map, &files, &root, &content_map, &imports_map)
    }

    fn _check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        entries: &[ImportEntry],
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

        for (idx, entry) in entries.iter().enumerate() {
            let module_val = utility_import_resolver::entry_module_path(entry);

            for forbidden in &forbidden_list {
                let forbidden_identity = Identity::new(forbidden);
                let (layer, suffixes) = utility_import_resolver::resolve_scope(&forbidden_identity);

                let mut is_forbidden = if suffixes.is_empty() {
                    // Direct layer match from raw_path segments
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
                    utility_import_resolver::entry_matches_scope(entry, &layer, &suffixes)
                };

                // Barrel file resolution — use resolved_path from filesystem
                if !is_forbidden {
                    if let Some(ref resolved_path) = entry.resolved_path {
                        let resolved_file = resolved_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let resolved_layer =
                            utility_path_normalizer::extract_layer_from_prefix(&resolved_file);
                        let layer_matches = resolved_layer.as_deref() == Some(layer.value());
                        let suffix_matches = suffixes.is_empty()
                            || suffixes.iter().any(|s| {
                                let suffix_lower = s.value().to_lowercase();
                                resolved_file
                                    .to_lowercase()
                                    .contains(&format!("_{}", suffix_lower))
                            });
                        if layer_matches && suffix_matches {
                            is_forbidden = true;
                        }
                    }
                }

                if is_forbidden {
                    let message = if layer_name == forbidden {
                        // Same-layer import — provide specific guidance
                        match layer_name.as_ref() {
                            "utility" => format!(
                                "AES201 FORBIDDEN_IMPORT: Layer 'utility' is importing from itself.\n\
                                    WHY? Utility files must be stateless and independent. Utility→utility imports create hidden dependencies.\n\
                                    FIX: Extract the dependent function to a taxonomy VO if it's data, or consolidate both utilities into a single file if they are tightly coupled."
                            ),
                            "capabilities" => format!(
                                "AES201 FORBIDDEN_IMPORT: Layer 'capabilities' is importing from itself.\n\
                                    WHY? Capabilities must communicate through contract protocols to maintain loose coupling.\n\
                                    FIX: Define a contract protocol (contract_*_protocol.rs) and use dependency injection to wire the capability implementation."
                            ),
                            "agent" => format!(
                                "AES201 FORBIDDEN_IMPORT: Layer 'agent' is importing from itself.\n\
                                    WHY? Agent orchestrators must coordinate through contract aggregates, not directly reference each other.\n\
                                    FIX: Define a contract aggregate (contract_*_aggregate.rs) and use dependency injection to wire the agent implementation."
                            ),
                            _ => format!(
                                "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                    WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                    FIX: Remove the import or refactor to use one of the allowed layers.",
                                layer_name, forbidden, layer_name, forbidden
                            ),
                        }
                    } else {
                        format!(
                            "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                FIX: Remove the import or refactor to use one of the allowed layers.",
                            layer_name, forbidden, layer_name, forbidden
                        )
                    };
                    violations.push(LintResult::new_arch(
                        file,
                        idx + 1,
                        "AES201",
                        Severity::CRITICAL,
                        message,
                    ));
                }
            }
        }
    }

    fn _check_scope_forbidden_imports(
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

            for (idx, entry) in entries.iter().enumerate() {
                let module_val = utility_import_resolver::entry_module_path(entry);

                for forbidden in &rule.forbidden.values {
                    let forbidden_identity = Identity::new(forbidden);
                    let (forbidden_layer, forbidden_suffixes) =
                        utility_import_resolver::resolve_scope(&forbidden_identity);

                    let mut is_forbidden = if forbidden_suffixes.is_empty() {
                        module_val
                            .split([':', '.', '/', '\\'])
                            .filter(|s| !s.is_empty())
                            .any(|seg| {
                                let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                    Some(l) => l == forbidden_layer,
                                    None => false,
                                }
                            })
                    } else {
                        utility_import_resolver::entry_matches_scope(
                            entry,
                            &forbidden_layer,
                            &forbidden_suffixes,
                        )
                    };

                    // Barrel file resolution — use resolved_path from filesystem
                    if !is_forbidden {
                        if let Some(ref resolved_path) = entry.resolved_path {
                            let resolved_file = resolved_path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let resolved_layer =
                                utility_path_normalizer::extract_layer_from_prefix(&resolved_file);
                            let layer_matches =
                                resolved_layer.as_deref() == Some(forbidden_layer.value());
                            let suffix_matches = forbidden_suffixes.is_empty()
                                || forbidden_suffixes.iter().any(|s| {
                                    let suffix_lower = s.value().to_lowercase();
                                    resolved_file
                                        .to_lowercase()
                                        .contains(&format!("_{}", suffix_lower))
                                });
                            if layer_matches && suffix_matches {
                                is_forbidden = true;
                            }
                        }
                    }

                    if is_forbidden {
                        let message = if rule_layer_str == *forbidden {
                            // Same-layer import — provide specific guidance
                            match rule_layer_str.as_ref() {
                                "utility" => format!(
                                    "AES201 FORBIDDEN_IMPORT: Layer 'utility' is importing from itself.\n\
                                        WHY? Utility files must be stateless and independent. Utility→utility imports create hidden dependencies.\n\
                                        FIX: Extract the dependent function to a taxonomy VO if it's data, or consolidate both utilities into a single file if they are tightly coupled."
                                ),
                                "capabilities" => format!(
                                    "AES201 FORBIDDEN_IMPORT: Layer 'capabilities' is importing from itself.\n\
                                        WHY? Capabilities must communicate through contract protocols to maintain loose coupling.\n\
                                        FIX: Define a contract protocol (contract_*_protocol.rs) and use dependency injection to wire the capability implementation."
                                ),
                                "agent" => format!(
                                    "AES201 FORBIDDEN_IMPORT: Layer 'agent' is importing from itself.\n\
                                        WHY? Agent orchestrators must coordinate through contract aggregates, not directly reference each other.\n\
                                        FIX: Define a contract aggregate (contract_*_aggregate.rs) and use dependency injection to wire the agent implementation."
                                ),
                                _ => format!(
                                    "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                        WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                        FIX: Remove the import or refactor to use one of the allowed layers.",
                                    rule_layer_str, forbidden, rule_layer_str, forbidden
                                ),
                            }
                        } else {
                            format!(
                                "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                    WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                    FIX: Remove the import or refactor to use one of the allowed layers.",
                                rule_layer_str, forbidden, rule_layer_str, forbidden
                            )
                        };
                        violations.push(LintResult::new_arch(
                            file,
                            idx + 1,
                            "AES201",
                            Severity::CRITICAL,
                            message,
                        ));
                    }
                }
            }
        }
    }
}

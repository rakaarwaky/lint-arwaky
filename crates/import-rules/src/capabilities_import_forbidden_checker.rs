// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses ImportEntry fields directly — no text-based parsing, no bridge functions.
//
// Architecture: config rules OVERRIDE layer definitions.
// 1. Layer definitions provide default forbidden lists per layer.
// 2. Config rules (YAML) can override/extend forbidden lists for specific scopes.
// 3. Single check per file — no duplicate violations.

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{FilePath, FilePathList, Identity, Severity};
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::{AstImportVO, FileParseResultVO};
use shared::common::parse_file_content;

use crate::utility_import_resolver;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use std::collections::{HashMap, HashSet};

const AES201_RULE_CODE: &str = "AES201";

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new(AES201_RULE_CODE)
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
            .filter(|r| r.name.value == AES201_RULE_CODE)
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
                self.check_file_forbidden(
                    &f_str,
                    &basename,
                    config,
                    layer_map,
                    &layer_keys,
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

    /// Unified forbidden import check: layer definitions + config overrides.
    fn check_file_forbidden(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        layer_keys: &[String],
        entries: &[ImportEntry],
        violations: &mut Vec<LintResult>,
    ) {
        // 1. Determine the file's layer from its filename prefix
        let filename = utility_layer_detector::extract_filename(file);
        let layer_name = match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys),
            None => {
                // No layer prefix — skip (naming rules handles prefix correctness)
                return;
            }
        };

        // 2. Get the default forbidden list from layer definitions
        let layer_name_vo = LayerNameVO::new(layer_name.as_str());
        let default_forbidden = layer_map
            .values
            .get(&layer_name_vo)
            .map(|def| {
                let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
                if !def.forbidden.values.is_empty() {
                    def.forbidden.values.clone()
                } else if is_surfaces {
                    vec!["agent".into(), "capabilities".into(), "utility".into()]
                } else {
                    vec![]
                }
            })
            .unwrap_or_default();

        // 3. Check if a config rule overrides the forbidden list for this layer
        let config_overrides = self.find_config_overrides(&layer_name, basename, config);

        // 4. Use config overrides if present, otherwise use layer definition defaults
        let forbidden_list = config_overrides.unwrap_or(default_forbidden);

        // 5. Check layer exceptions (config exceptions already filtered at top level)
        let layer_exceptions: HashSet<String> = layer_map
            .values
            .get(&layer_name_vo)
            .map(|def| def.exceptions.values.iter().cloned().collect())
            .unwrap_or_default();
        if layer_exceptions.contains(basename) {
            return;
        }

        // 6. Delegate to shared import-vs-forbidden check
        self.check_imports_against_forbidden(
            file,
            &layer_name,
            &forbidden_list,
            entries,
            violations,
        );
    }

    /// Find config rules that override the default forbidden list for a layer.
    /// Returns None if no override exists (caller falls back to layer definition).
    fn find_config_overrides(
        &self,
        layer_name: &str,
        basename: &str,
        config: &ArchitectureConfig,
    ) -> Option<Vec<String>> {
        for rule in &config.rules {
            if rule.name.value != AES201_RULE_CODE {
                continue;
            }
            if rule.forbidden.values.is_empty() {
                continue;
            }
            // Check if this rule's scope matches the file's layer
            if let Some((rule_layer, _)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &Identity::new(&rule.scope.value),
                )
            {
                if rule_layer == layer_name {
                    return Some(rule.forbidden.values.clone());
                }
            }
        }
        None
    }

    /// Core import-vs-forbidden check — shared by all code paths.
    fn check_imports_against_forbidden(
        &self,
        file: &str,
        source_layer: &str,
        forbidden_list: &[String],
        entries: &[ImportEntry],
        violations: &mut Vec<LintResult>,
    ) {
        // Pre-resolve all forbidden scopes once (not per entry)
        let resolved_forbidden: Vec<(&str, LayerNameVO, Vec<Identity>)> = forbidden_list
            .iter()
            .map(|forbidden| {
                let forbidden_identity = Identity::new(forbidden);
                let (forbidden_layer, forbidden_suffixes) =
                    utility_import_resolver::resolve_scope(&forbidden_identity);
                (forbidden.as_str(), forbidden_layer, forbidden_suffixes)
            })
            .collect();

        for (idx, entry) in entries.iter().enumerate() {
            let module_val = utility_import_resolver::entry_module_path(entry);

            // Pre-split module path once per entry (not per forbidden item)
            let module_segments: Vec<&str> = module_val
                .split([':', '.', '/', '\\'])
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_end_matches(';').trim())
                .collect();

            for &(forbidden, ref forbidden_layer, ref forbidden_suffixes) in &resolved_forbidden {

                let mut is_forbidden = if forbidden_suffixes.is_empty() {
                    module_segments.iter().any(|seg| {
                        let cleaned = Identity::new(*seg);
                        match utility_import_resolver::extract_layer_from_import(&cleaned) {
                            Some(l) => l == *forbidden_layer,
                            None => false,
                        }
                    })
                } else {
                    utility_import_resolver::entry_matches_scope(
                        entry,
                        forbidden_layer,
                        forbidden_suffixes,
                    )
                };

                // Barrel file resolution
                if !is_forbidden {
                    if let Some(ref resolved_path) = entry.resolved_path {
                        let resolved_file = resolved_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let resolved_layer =
                            shared::common::utility_layer_detector::detect_layer_from_prefix(&resolved_file);
                        let layer_matches =
                            resolved_layer.as_deref() == Some(forbidden_layer.value());
                        let suffix_matches = forbidden_suffixes.is_empty()
                            || forbidden_suffixes.iter().any(|s| {
                                let suffix_lower = s.value().to_lowercase();
                                let resolved_lower = resolved_file.to_lowercase();
                                let resolved_stem = std::path::Path::new(&resolved_lower)
                                    .file_stem()
                                    .and_then(|st| st.to_str())
                                    .unwrap_or(&resolved_lower);
                                // Whole-word segment matching per FRD
                                resolved_stem.split('_').any(|seg| seg == suffix_lower)
                            });
                        if layer_matches && suffix_matches {
                            is_forbidden = true;
                        }
                    }
                }

                if is_forbidden {
                    let message = if source_layer == forbidden {
                        match source_layer {
                            "utility" => "AES201 FORBIDDEN_IMPORT: Layer 'utility' is importing from itself.\n\
                                    WHY? Utility files must be stateless and independent. Utility→utility imports create hidden dependencies.\n\
                                    FIX: Extract the dependent function to a taxonomy VO if it's data, or consolidate both utilities into a single file if they are tightly coupled.".to_string(),
                            "capabilities" => "AES201 FORBIDDEN_IMPORT: Layer 'capabilities' is importing from itself.\n\
                                    WHY? Capabilities must communicate through contract protocols to maintain loose coupling.\n\
                                    FIX: Define a contract protocol (contract_*_protocol.rs) and use dependency injection to wire the capability implementation.".to_string(),
                            "agent" => "AES201 FORBIDDEN_IMPORT: Layer 'agent' is importing from itself.\n\
                                    WHY? Agent orchestrators must coordinate through contract aggregates, not directly reference each other.\n\
                                    FIX: Define a contract aggregate (contract_*_aggregate.rs) and use dependency injection to wire the agent implementation.".to_string(),
                            _ => format!(
                                "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                    WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                    FIX: Remove the import or refactor to use one of the allowed layers.",
                                source_layer, forbidden, source_layer, forbidden
                            ),
                        }
                    } else {
                        format!(
                            "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                                WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                                FIX: Remove the import or refactor to use one of the allowed layers.",
                            source_layer, forbidden, source_layer, forbidden
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

// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses ImportEntry fields directly — no text-based parsing, no bridge functions.

use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{FilePath, FilePathList, Identity, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

use crate::utility_import_resolver;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

const AES202_RULE_CODE: &str = "AES202";
static EMPTY_HASHSET: LazyLock<HashSet<String>> = LazyLock::new(HashSet::new);

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new(AES202_RULE_CODE)
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
            .filter(|r| r.name.value == AES202_RULE_CODE)
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        // Pre-compute layer exceptions map (avoids per-file rebuild)
        let layer_exceptions_map: HashMap<String, HashSet<String>> = layer_map
            .values
            .iter()
            .map(|(name, def)| {
                (
                    name.value().to_string(),
                    def.exceptions.values.iter().cloned().collect(),
                )
            })
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
                self.check_file_mandatory(
                    &f_str,
                    &basename,
                    config,
                    layer_map,
                    &layer_keys,
                    entries,
                    &aes202_exceptions,
                    &layer_exceptions_map,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        // Deduplicate violations by (file, line, code) — HashSet for non-consecutive dedup
        let mut seen: HashSet<(String, i64, String)> = HashSet::new();
        file_violations.retain(|v| {
            seen.insert((
                v.file.value().to_string(),
                v.line.value(),
                v.code.code().to_string(),
            ))
        });

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

    /// Unified mandatory import check: layer definitions + config overrides.
    fn check_file_mandatory(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        layer_keys: &[String],
        entries: &[ImportEntry],
        config_exceptions: &HashSet<String>,
        layer_exceptions_map: &HashMap<String, HashSet<String>>,
        violations: &mut Vec<LintResult>,
    ) {
        if utility_import_resolver::is_barrel_file(basename) {
            return;
        }

        // 1. Determine the file's layer
        let filename = utility_layer_detector::extract_filename(file);
        let source_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => {
                let specialized = utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys);
                // Extract clean layer name from stem for message
                let stem: &str = basename.rsplit('.').next_back().map_or(basename, |s| s);
                let clean_layer = stem.split('_').next().map_or("unknown", |s| s);
                (specialized, clean_layer.to_string())
            }
            None => {
                // No layer prefix — skip (naming rules handles prefix correctness)
                return;
            }
        };
        let (layer_name, source_layer_clean) = source_layer;

        // 2. Get default mandatory list from layer definitions
        let layer_name_vo = LayerNameVO::new(layer_name.as_str());
        let default_mandatory: Vec<String> = layer_map
            .values
            .get(&layer_name_vo)
            .map(|def| def.mandatory.values.clone())
            .unwrap_or_default();

        // 3. Check if config overrides
        let config_overrides = self.find_config_overrides(&layer_name, basename, config);

        // 4. Use config overrides if present, otherwise defaults
        let mandatory_list = config_overrides.unwrap_or(default_mandatory);

        if mandatory_list.is_empty() {
            return;
        }

        // 5. Check exceptions using pre-computed sets (no per-file rebuild)
        let layer_exceptions: &HashSet<String> = layer_exceptions_map
            .get(&layer_name)
            .unwrap_or(&EMPTY_HASHSET);

        if layer_exceptions.contains(basename) || config_exceptions.contains(basename) {
            return;
        }

        // 6. Single pass: check all requirements
        self.check_requirements(file, &source_layer_clean, &mandatory_list, entries, violations);
    }

    /// Find config rules that override the default mandatory list for a layer.
    fn find_config_overrides(
        &self,
        layer_name: &str,
        basename: &str,
        config: &ArchitectureConfig,
    ) -> Option<Vec<String>> {
        for rule in &config.rules {
            if rule.name.value != AES202_RULE_CODE && rule.rule_type.code() != AES202_RULE_CODE {
                continue;
            }
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            if let Some((rule_layer, _)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &scope_identity,
                )
            {
                if rule_layer == layer_name {
                    return Some(rule.mandatory.values.clone());
                }
            }
        }
        None
    }

    /// Core requirement check — shared by all code paths.
    fn check_requirements(
        &self,
        file: &str,
        source_layer: &str,
        required_list: &[String],
        entries: &[ImportEntry],
        violations: &mut Vec<LintResult>,
    ) {
        for required in required_list {
            let required_identity = Identity::new(required);
            let (req_layer, req_suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let req_layer_str = req_layer.value();

            let is_present = entries.iter().any(|entry| {
                utility_import_resolver::entry_matches_scope(entry, &req_layer, &req_suffixes)
            }) || self._check_barrel_mandatory(entries, &req_layer, &req_suffixes, req_layer_str);

            if !is_present {
                let v = LintResult::new_arch(
                    file,
                    1,
                    AES202_RULE_CODE,
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
                    shared::common::utility_layer_detector::detect_layer_from_prefix(&resolved_file);
                let layer_matches = resolved_layer.as_deref() == Some(layer_str);
                let suffix_matches = suffixes.is_empty()
                    || suffixes.iter().any(|s| {
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
                    return true;
                }
            }
        }
        false
    }
}

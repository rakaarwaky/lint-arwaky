// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict, cross-layer)
use crate::utility_naming_checker::string_filename_result;
use crate::utility_naming_checker::{get_stem, get_suffix, rule_exception_set};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::ISuffixPrefixChecker;
use shared::naming_rules::{RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT};

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
pub struct SuffixPrefixChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISuffixPrefixChecker for SuffixPrefixChecker {
    fn check_domain_suffixes(
        &self,
        _config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let suffix_to_layer = Self::build_suffix_to_layer_map(layer_map);
        let exceptions = rule_exception_set(_config, RULE_CODE_SUFFIX_PREFIX);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = f.rsplit('/').next().unwrap_or(&f_str);
                // Rule-level exceptions evaluated before layer detection (FRD FR-002).
                if exceptions.contains(filename) {
                    return None;
                }
                let layer = self._detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));

                // No recognised layer prefix → no suffix policy applies → skip.
                // (AES000 removed: unknown-prefix signalling is out of scope.)
                layer.as_ref()?;

                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self.check_domain_suffixes_internal(
                    &f_str,
                    filename,
                    def,
                    &layer_name,
                    &suffix_to_layer,
                )
            })
            .collect();

        results.values.extend(violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SuffixPrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPrefixChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Build a mapping from suffix → base layer name for cross-layer validation.
    pub fn build_suffix_to_layer_map(
        layer_map: &LayerMapVO,
    ) -> std::collections::HashMap<String, String> {
        let mut suffix_to_layer = std::collections::HashMap::new();
        for (layer_name, def) in &layer_map.values {
            if layer_name.value().contains('(') {
                continue;
            }
            if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
                for suffix in &def.naming.allowed_suffix.values {
                    suffix_to_layer
                        .entry(suffix.clone())
                        .or_insert_with(|| layer_name.value().to_string());
                }
            }
        }
        suffix_to_layer
    }

    fn _detect_layer(&self, file: &str, layer_keys: &[String]) -> Option<String> {
        let filename = utility_layer_detector::extract_filename(file);
        utility_layer_detector::detect_layer_from_prefix(filename)
            .map(|base| utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys))
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules + cross-layer validation).
    pub fn check_domain_suffixes_internal(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&shared::common::taxonomy_definition_vo::LayerDefinition>,
        layer_name: &Option<LayerNameVO>,
        suffix_to_layer: &std::collections::HashMap<String, String>,
    ) -> Option<LintResult> {
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let def = definition?;
        if def.exceptions.values.contains(&filename.to_string()) {
            return None;
        }

        let stem = get_stem(filename)?;
        let suffix = get_suffix(stem);
        let layer_display = layer_name
            .as_ref()
            .map(|l| l.value().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // 1. Forbidden suffix check (always enforced regardless of policy)
        if let Some(suf) = &suffix
            && def.naming.forbidden_suffix.values.iter().any(|v| v == *suf)
        {
            return Some(string_filename_result(
                file,
                RULE_CODE_SUFFIX_PREFIX,
                format!(
                    "Suffix '{}' is not permitted in the '{}' layer. Each architectural layer allows only \
                     specific suffixes that match its role. The suffix '{}' belongs to a different layer's domain. \
                     Rename the file with an allowed suffix for '{}', or move it to the appropriate layer.",
                    suf, layer_display, suf, layer_display
                ),
                Severity::HIGH,
            ));
        }

        // 2. Cross-layer suffix validation (FR-002: PrefixSuffixMismatch)
        if let Some(suf) = &suffix
            && let Some(suffix_belonging_layer) = suffix_to_layer.get(*suf)
        {
            let current_base = layer_display.split('(').next().unwrap_or(&layer_display);
            if suffix_belonging_layer != current_base {
                return Some(string_filename_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    format!(
                        "Suffix '{}' belongs to the '{}' layer's suffix set, but this file is in the '{}' layer. \
                         Rename the file with a suffix appropriate for the '{}' layer, or move it to the '{}' layer.",
                        suf,
                        suffix_belonging_layer,
                        layer_display,
                        layer_display,
                        suffix_belonging_layer
                    ),
                    Severity::HIGH,
                ));
            }
        }

        // 3. Strict policy check (suffix not in this layer's allowed list)
        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.iter().any(|v| v == *s),
                None => false,
            };
            if !valid {
                let allowed_list = &def.naming.allowed_suffix.values;
                let suffix_display = suffix.unwrap_or("(none)");
                return Some(string_filename_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    format!(
                        "Suffix '{}' is not in the allowed list for layer '{}'. \
                         Allowed suffixes for '{}': {}. \
                         A suffix outside this list means either the file belongs in a different layer \
                         or needs a different architectural role suffix.",
                        suffix_display,
                        layer_display,
                        layer_display,
                        allowed_list.join(", ")
                    ),
                    Severity::HIGH,
                ));
            }
        }

        None
    }
}

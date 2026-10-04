// PURPOSE: SuffixPolicyChecker — AES102 suffix/prefix policy enforcement capability
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared_common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_naming_rules::contract_naming_checker_protocol::ISuffixPolicyProtocol;
use shared_naming_rules::taxonomy_naming_rules_constant::{
    RULE_CODE_SUFFIX_PREFIX, SPECIALIZED_LAYER_MARKER, SUFFIX_POLICY_STRICT,
};
use shared_naming_rules::utility_naming_checker::{
    basename_of, detect_layer, get_stem, get_suffix, parse_path, rule_exception_set,
    string_filename_result,
};

use std::collections::HashMap;

// ─── Block 1: Struct Definition ────────────────────────────

/// Stateless AES102 suffix/prefix policy checker.
///
/// Validates that each file's suffix matches its layer's allowed set, enforces
/// forbidden-suffix rules, and catches cross-layer suffix mismatches. No
/// internal state.
pub struct SuffixPolicyChecker {}

// ─── Protocol Trait Implementation ────────────────────────────────────────

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ISuffixPolicyProtocol for SuffixPolicyChecker {
    /// FR-NamingRules-002 — AES102: check each file's layer suffix and role
    /// prefix against the per-layer policy.
    fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let suffix_to_layer = Self::build_suffix_to_layer_map(layer_map);
        let exceptions = rule_exception_set(config, RULE_CODE_SUFFIX_PREFIX);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = basename_of(&f_str);
                // Rule-level exceptions evaluated before layer detection (FRD FR-002).
                if exceptions.iter().any(|v| v == filename) {
                    return None;
                }
                let layer = detect_layer(&f_str, &layer_keys);
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

// ─── Constructors, Helpers, Private Methods ────────────────────────────────

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for SuffixPolicyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPolicyChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Build a mapping from suffix → base layer name for cross-layer validation.
    pub fn build_suffix_to_layer_map(layer_map: &LayerMapVO) -> HashMap<String, String> {
        let mut suffix_to_layer = HashMap::new();
        for (layer_name, def) in &layer_map.values {
            // Sub-layers (e.g. "capabilities(command)") share their base layer's
            // suffix set, so skipping them here avoids redundant cross-layer entries.
            if layer_name.value().contains(SPECIALIZED_LAYER_MARKER) {
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

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules + cross-layer validation).
    pub fn check_domain_suffixes_internal(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        layer_name: &Option<LayerNameVO>,
        suffix_to_layer: &HashMap<String, String>,
    ) -> Option<LintResult> {
        let fp = parse_path(filename)?;
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let def = definition?;
        if def.exceptions.values.iter().any(|v| v == filename) {
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
                "SUFFIX_PREFIX",
                format!(
                    "Suffix '{}' is not permitted in the '{}' layer. Each architectural layer allows only \
                     specific suffixes that match its role.",
                    suf, layer_display
                ),
                format!(
                    "Rename the file with an allowed suffix for '{}', or move it to the appropriate layer.",
                    layer_display
                ),
            ));
        }

        // 2. Cross-layer suffix validation (FR-002: PrefixSuffixMismatch)
        if let Some(suf) = &suffix
            && let Some(suffix_belonging_layer) = suffix_to_layer.get(*suf)
        {
            let current_base = base_layer_of(&layer_display);
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
                    "SUFFIX_PREFIX",
                    format!(
                        "Suffix '{}' belongs to the '{}' layer's suffix set, but this file is in the '{}' layer.",
                        suf, suffix_belonging_layer, layer_display
                    ),
                    format!(
                        "Rename the file with a suffix appropriate for the '{}' layer, or move it to the '{}' layer.",
                        layer_display, suffix_belonging_layer
                    ),
                ));
            }
        }

        // 3. Strict policy check (suffix not in this layer's allowed list)
        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.iter().any(|v| v == s),
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
                    "SUFFIX_PREFIX",
                    format!(
                        "Suffix '{}' is not in the allowed list for layer '{}'. \
                         A suffix outside this list means either the file belongs in a different layer \
                         or needs a different architectural role suffix.",
                        suffix_display, layer_display
                    ),
                    format!(
                        "Allowed suffixes for '{}': {}. Rename the file with one of the allowed suffixes.",
                        layer_display,
                        allowed_list.join(", ")
                    ),
                ));
            }
        }

        None
    }
}

/// Extract the base layer name from a potentially specialized layer display string.
/// e.g. `"surfaces(command)"` → `"surfaces"`, `"utility"` → `"utility"`.
/// `split('(')` always yields at least one element, so `unwrap_or` is unreachable
/// but satisfies the linter's no-unwrap policy.
fn base_layer_of(layer_display: &str) -> &str {
    layer_display.split('(').next().unwrap_or(layer_display)
}

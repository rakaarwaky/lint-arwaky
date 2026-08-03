// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict, cross-layer)
use crate::utility_naming_checker::string_filename_result;
use crate::utility_naming_checker::{get_stem, get_suffix};
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
use shared::naming_rules::{LAYER_PREFIXES, RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT};

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
        let all_suffixes = Self::build_all_suffixes(layer_map);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = f.rsplit('/').next().unwrap_or(&f_str);
                let layer = self._detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));

                // AES102 UnknownPrefix: no layer detected from filename prefix
                if layer.is_none() {
                    return self._check_unknown_prefix(&f_str, filename);
                }

                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self._check_domain_suffixes(
                    &f_str,
                    filename,
                    def,
                    &layer_name,
                    &suffix_to_layer,
                    &all_suffixes,
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
    fn build_suffix_to_layer_map(
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

    /// Collect all allowed suffixes from all layers (base layers only) for UnknownSuffix check.
    fn build_all_suffixes(layer_map: &LayerMapVO) -> Vec<String> {
        let mut all = Vec::new();
        for (layer_name, def) in &layer_map.values {
            if layer_name.value().contains('(') {
                continue;
            }
            for suffix in &def.naming.allowed_suffix.values {
                if !all.contains(suffix) {
                    all.push(suffix.clone());
                }
            }
        }
        all
    }

    fn _detect_layer(&self, file: &str, layer_keys: &[String]) -> Option<String> {
        let filename = utility_layer_detector::extract_filename(file);
        utility_layer_detector::detect_layer_from_prefix(filename)
            .map(|base| utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys))
    }

    /// AES102 UnknownPrefix — file prefix does not match any recognised layer prefix.
    fn _check_unknown_prefix(&self, file: &str, filename: &str) -> Option<LintResult> {
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let stem = get_stem(filename)?;
        let actual_prefix = stem.split('_').next().unwrap_or_default();

        if actual_prefix.is_empty() || LAYER_PREFIXES.iter().any(|p| stem.starts_with(p)) {
            return None;
        }

        Some(string_filename_result(
            file,
            RULE_CODE_SUFFIX_PREFIX,
            format!(
                "The prefix '{}' is not one of the {} recognised AES layer prefixes. \
                 Every source file must start with a valid layer prefix so it can be assigned to the correct architectural layer. \
                 Likely causes: typo in the prefix name, or the file is in the wrong directory.",
                actual_prefix,
                LAYER_PREFIXES.len()
            ),
            Severity::HIGH,
        ))
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules + cross-layer validation).
    fn _check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&shared::common::taxonomy_definition_vo::LayerDefinition>,
        layer_name: &Option<LayerNameVO>,
        suffix_to_layer: &std::collections::HashMap<String, String>,
        all_suffixes: &[String],
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

        // 3. Unknown suffix check (strict policy only — suffix not in any layer's set)
        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT
            && let Some(suf) = &suffix
            && !all_suffixes.iter().any(|v| v == *suf)
        {
            return Some(string_filename_result(
                file,
                RULE_CODE_SUFFIX_PREFIX,
                format!(
                    "Suffix '{}' does not belong to any recognised layer's suffix set. \
                     Only suffixes defined in the architecture configuration are valid. \
                     This means the suffix is either a typo or the file belongs in a different layer.",
                    suf
                ),
                Severity::HIGH,
            ));
        }

        // 4. Strict policy check (suffix not in this layer's allowed list)
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

#[cfg(test)]
mod tests {
    use super::*;
    use shared::common::taxonomy_common_vo::{PatternList, SuffixPolicyVO};
    use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
    use shared::common::taxonomy_layer_vo::LayerNameVO;
    use std::collections::HashMap;

    fn checker() -> SuffixPrefixChecker {
        SuffixPrefixChecker::new()
    }

    fn layer_map_with_strict_capabilities() -> LayerMapVO {
        let mut def = LayerDefinition::default();
        def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
        def.naming.allowed_suffix =
            PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
        def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);
        let mut layers = HashMap::new();
        layers.insert(LayerNameVO::new("capabilities"), def);
        LayerMapVO::new(layers)
    }

    fn layer_def(map: &LayerMapVO) -> &LayerDefinition {
        map.values.get(&LayerNameVO::new("capabilities")).unwrap()
    }

    #[test]
    fn construction_succeeds() {
        let _ = checker();
    }

    #[test]
    fn allowed_suffix_no_violation() {
        let map = layer_map_with_strict_capabilities();
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities_user_checker.rs",
            "capabilities_user_checker.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(result.is_none());
    }

    #[test]
    fn forbidden_suffix_produces_violation() {
        let map = layer_map_with_strict_capabilities();
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities_user_vo.rs",
            "capabilities_user_vo.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(
            result.is_some(),
            "forbidden suffix 'vo' must produce a violation"
        );
    }

    #[test]
    fn strict_policy_wrong_suffix_produces_violation() {
        let map = layer_map_with_strict_capabilities();
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities_user_handler.rs",
            "capabilities_user_handler.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(
            result.is_some(),
            "suffix not in allowed list under strict policy must produce a violation"
        );
    }

    #[test]
    fn barrel_file_skipped() {
        let map = layer_map_with_strict_capabilities();
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities/mod.rs",
            "mod.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(result.is_none(), "barrel files must be skipped");
    }

    #[test]
    fn unknown_suffix_strict_produces_violation() {
        let map = layer_map_with_strict_capabilities();
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities_user_foo.rs",
            "capabilities_user_foo.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(
            result.is_some(),
            "unknown suffix under strict policy must produce violation"
        );
    }

    #[test]
    fn unknown_suffix_flexible_no_violation() {
        let mut def = LayerDefinition::default();
        def.naming.suffix_policy = SuffixPolicyVO::new("flexible".to_string());
        def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);
        let mut layers = HashMap::new();
        layers.insert(LayerNameVO::new("capabilities"), def);
        let map = LayerMapVO::new(layers);
        let suffix_map = SuffixPrefixChecker::build_suffix_to_layer_map(&map);
        let all = SuffixPrefixChecker::build_all_suffixes(&map);
        let result = checker()._check_domain_suffixes(
            "src/capabilities_user_foo.rs",
            "capabilities_user_foo.rs",
            Some(layer_def(&map)),
            &Some(LayerNameVO::new("capabilities")),
            &suffix_map,
            &all,
        );
        assert!(
            result.is_none(),
            "unknown suffix under flexible policy must not produce violation"
        );
    }

    #[test]
    fn unknown_prefix_produces_violation() {
        let result = checker()._check_unknown_prefix("src/foo_bar_baz.rs", "foo_bar_baz.rs");
        assert!(
            result.is_some(),
            "unknown prefix must produce AES102 violation"
        );
    }

    #[test]
    fn unknown_prefix_barrel_skipped() {
        let result = checker()._check_unknown_prefix("src/foo/mod.rs", "mod.rs");
        assert!(
            result.is_none(),
            "barrel files must be skipped for unknown prefix"
        );
    }
}

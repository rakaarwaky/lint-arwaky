// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict, cross-layer)
use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{FilePath, FilePathList, Severity};

use crate::utility_naming_checker::string_filename_result;
use crate::utility_naming_checker::{get_stem, get_suffix};
use shared::common::utility_layer_detector;
use shared::common::{LayerMapVO, LayerNameVO, LintMessage};
use shared::config_system::ArchitectureConfig;
use shared::naming_rules::ISuffixPrefixChecker;
use shared::naming_rules::NamingViolation;
use shared::naming_rules::{RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT};

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
pub struct SuffixPrefixChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait]
impl ISuffixPrefixChecker for SuffixPrefixChecker {
    async fn check_domain_suffixes(
        &self,
        _config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        // Build suffix→layer mapping for cross-layer validation
        let suffix_to_layer = Self::build_suffix_to_layer_map(layer_map);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = f.rsplit('/').next().unwrap_or(&f_str);
                let layer = self._detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));
                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self._check_domain_suffixes(&f_str, filename, def, &layer_name, &suffix_to_layer)
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
    /// Only maps to base layers (not specialized sub-layers like `taxonomy(vo)`) to avoid
    /// false positives where a file in `taxonomy(vo)` is incorrectly flagged because
    /// its suffix is mapped to a different sub-layer of the same base layer.
    fn build_suffix_to_layer_map(
        layer_map: &LayerMapVO,
    ) -> std::collections::HashMap<String, String> {
        let mut suffix_to_layer = std::collections::HashMap::new();
        for (layer_name, def) in &layer_map.values {
            // Skip specialized sub-layers — only base layers participate in cross-layer
            // validation. Sub-layers inherit the parent's suffix list, so mapping them
            // causes false positives when resolve_specialized_layer resolves to a
            // different sub-layer of the same base.
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
    fn _check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&shared::taxonomy_definition_vo::LayerDefinition>,
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

        // 1. Forbidden suffix check (always enforced regardless of policy)
        if let Some(suf) = &suffix
            && def.naming.forbidden_suffix.values.iter().any(|v| v == *suf)
        {
            let layer_display = layer_name
                .as_ref()
                .map(|l| l.value().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            return Some(string_filename_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::SuffixForbidden {
                        layer_name: layer_display.clone(),
                        forbidden_suffix: suf.to_string(),
                        reason: Some(LintMessage::new(format!(
                            "Suffix '{}' is not permitted in the '{}' layer. Each architectural layer allows only \
                             specific suffixes that match its role. The suffix '{}' belongs to a different layer's domain. \
                             Rename the file with an allowed suffix for '{}', or move it to the appropriate layer.",
                            suf, layer_display, suf, layer_display
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
        }

        // 2. Cross-layer suffix validation (FR-002: PrefixSuffixMismatch)
        // If the suffix belongs to a DIFFERENT base layer's strict suffix set, emit PrefixSuffixMismatch.
        // This check runs before strict policy to provide a more specific error message.
        // Note: suffix_to_layer only contains base layers, so we normalize current_layer
        // to its base name too (e.g. "taxonomy(vo)" → "taxonomy") to avoid false positives
        // where specialized sub-layers of the same base are compared against each other.
        if let Some(suf) = &suffix
            && let Some(suffix_belonging_layer) = suffix_to_layer.get(*suf)
        {
            let current_layer = layer_name
                .as_ref()
                .map(|l| l.value().to_string())
                .unwrap_or_default();
            let current_base = current_layer.split('(').next().unwrap_or(&current_layer);
            if suffix_belonging_layer != current_base {
                return Some(string_filename_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::PrefixSuffixMismatch {
                        expected_layer: current_layer.clone(),
                        actual_suffix: suf.to_string(),
                        suffix_layer: suffix_belonging_layer.clone(),
                        reason: Some(LintMessage::new(format!(
                            "Suffix '{}' belongs to the '{}' layer's suffix set, but this file is in the '{}' layer. \
                             Rename the file with a suffix appropriate for the '{}' layer, or move it to the '{}' layer.",
                            suf, suffix_belonging_layer, current_layer, current_layer, suffix_belonging_layer
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }

        // 3. Strict policy check (fallback if no cross-layer match)
        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.iter().any(|v| v == *s),
                None => false,
            };
            if !valid {
                let allowed_list = def.naming.allowed_suffix.values.clone();
                let layer_display = layer_name
                    .as_ref()
                    .map(|l| l.value().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                let suffix_display = suffix.unwrap_or("(none)");
                return Some(string_filename_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::SuffixMismatch {
                        layer_name: layer_display.clone(),
                        used_suffix: suffix_display.to_string(),
                        allowed: allowed_list.clone(),
                        reason: Some(LintMessage::new(format!(
                            "Suffix '{}' is not in the allowed list for layer '{}'. \
                             Allowed suffixes for '{}': {}. \
                             A suffix outside this list means either the file belongs in a different layer \
                             or needs a different architectural role suffix.",
                            suffix_display,
                            layer_display,
                            layer_display,
                            allowed_list.join(", ")
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }

        None
    }
}

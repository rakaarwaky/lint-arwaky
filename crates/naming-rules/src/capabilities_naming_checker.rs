// PURPOSE: NamingChecker — combined AES101 + AES102 naming checker capability
use crate::utility_naming_checker::{
    basename_of, detect_layer, get_stem, get_suffix, parse_path, rule_exception_set,
    string_filename_result,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::taxonomy_naming_constant::{
    RULE_CODE_NAMING_CONVENTION, RULE_CODE_SUFFIX_PREFIX, SPECIALIZED_LAYER_MARKER,
    SUFFIX_POLICY_STRICT,
};

use std::collections::HashMap;
use std::sync::OnceLock;

const MIN_WORDS_DEFAULT: usize = 3;

// ─── Block 1: Struct Definition ───────────────────────────

/// Stateless naming checker for AES101 (convention) and AES102 (suffix/prefix).
///
/// A file is evaluated against both rule families in one pass over the file
/// list: the stem pattern check and the suffix/prefix policy check share the
/// same layer detection, so they share one traversal. No internal state.
pub struct NamingChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl INamingCheckerProtocol for NamingChecker {
    /// AES101 — check each file's stem against the layer_concern_role convention.
    fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let min_words = Self::min_words_from_config(config);
        let exceptions = rule_exception_set(config, RULE_CODE_NAMING_CONVENTION);

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = basename_of(&f_str);
                // Rule-level exceptions evaluated before layer detection (FRD FR-001).
                if exceptions.iter().any(|v| v == filename) {
                    return None;
                }
                let layer = detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));
                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self.check_file_naming_internal(&f_str, filename, &layer_name, def, min_words)
            })
            .collect();

        results.values.extend(violations);
    }

    /// AES102 — check each file's layer suffix and role prefix against the per-layer policy.
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for NamingChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn min_words_from_config(config: &ArchitectureConfig) -> usize {
        let value = config.naming.word_count.value;
        if value <= 0 {
            return MIN_WORDS_DEFAULT;
        }
        usize::try_from(value).unwrap_or(MIN_WORDS_DEFAULT)
    }

    /// Slots map 1:1 to word counts 1..=10; counts > 10 clamp to the 10-word slot.
    fn naming_regex(min_words: usize) -> Option<&'static Regex> {
        static REGEX_TABLE: [OnceLock<Option<Regex>>; 10] = [
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
        ];
        let clamped = min_words.clamp(1, 10);
        REGEX_TABLE[clamped - 1]
            .get_or_init(|| {
                let pattern = format!(r"^[a-z0-9]+(_[a-z0-9]+){{{},}}$", clamped.saturating_sub(1));
                Regex::new(&pattern).ok()
            })
            .as_ref()
    }

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min N words).
    pub fn check_file_naming_internal(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<LayerNameVO>,
        definition: Option<&LayerDefinition>,
        min_words: usize,
    ) -> Option<LintResult> {
        let fp = parse_path(filename)?;
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let stem = get_stem(filename)?;

        if let Some(def) = definition
            && def.exceptions.values.iter().any(|v| v == filename)
        {
            return None;
        }

        if !Self::naming_regex(min_words).is_some_and(|re| re.is_match(stem)) {
            let layer_hint = layer_name
                .as_ref()
                .map(|l| format!(" (detected layer: '{}')", l.value()))
                .unwrap_or_default();
            return Some(string_filename_result(
                file,
                RULE_CODE_NAMING_CONVENTION,
                format!(
                    "The stem '{}' does not match the required pattern 'prefix_concept_suffix'. \
                     Expected: lowercase alphanumeric words separated by underscores, minimum {} words. \
                     Example valid names: 'capabilities_user_checker', 'capabilities_db_adapter'. \
                     Issue: '{}' may have uppercase characters, wrong separator, or fewer than {} words{}.",
                    stem, min_words, stem, min_words, layer_hint
                ),
                Severity::HIGH,
            ));
        }

        None
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

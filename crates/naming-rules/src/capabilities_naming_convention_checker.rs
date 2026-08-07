// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min N words)
use crate::utility_naming_checker::{
    basename_of, detect_layer, get_stem, parse_path, rule_exception_set, string_filename_result,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use shared::naming_rules::taxonomy_naming_constant::RULE_CODE_NAMING_CONVENTION;

use std::sync::OnceLock;

const MIN_WORDS_DEFAULT: usize = 3;

// ─── Block 1: Struct Definition ───────────────────────────

/// Stateless AES101 naming convention checker.
///
/// Validates that file stems follow `prefix_concept_suffix` snake_case
/// pattern with configurable minimum word count. No internal state.
pub struct NamingConventionChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl INamingConventionChecker for NamingConventionChecker {
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
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for NamingConventionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingConventionChecker {
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
        definition: Option<&shared::common::taxonomy_definition_vo::LayerDefinition>,
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
}

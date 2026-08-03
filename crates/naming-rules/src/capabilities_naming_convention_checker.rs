// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min N words)
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;

use crate::utility_naming_checker::get_stem;
use crate::utility_naming_checker::string_filename_result;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingConventionChecker;
use shared::naming_rules::RULE_CODE_NAMING_CONVENTION;

use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
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

        let violations: Vec<LintResult> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = match f.rsplit('/').next() {
                    Some(name) => name,
                    None => &f_str,
                };
                let layer = self._detect_layer(&f_str, &layer_keys);
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
            return 3;
        }
        usize::try_from(value).unwrap_or(3)
    }

    /// Build naming regex dynamically based on min_words.
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
        let idx = min_words.min(9);
        REGEX_TABLE[idx]
            .get_or_init(|| {
                let pattern = format!(
                    r"^[a-z0-9]+(_[a-z0-9]+){{{},}}$",
                    min_words.saturating_sub(1)
                );
                Regex::new(&pattern).ok()
            })
            .as_ref()
    }

    fn _detect_layer(&self, file: &str, layer_keys: &[String]) -> Option<String> {
        let filename = utility_layer_detector::extract_filename(file);
        utility_layer_detector::detect_layer_from_prefix(filename)
            .map(|base| utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys))
    }

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min N words).
    pub fn check_file_naming_internal(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<LayerNameVO>,
        definition: Option<&shared::common::taxonomy_definition_vo::LayerDefinition>,
        min_words: usize,
    ) -> Option<LintResult> {
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let stem = get_stem(filename).unwrap_or_default();

        if let Some(def) = definition
            && def.exceptions.values.contains(&filename.to_string())
        {
            return None;
        }

        if Self::naming_regex(min_words).is_none_or(|re| !re.is_match(stem)) {
            return Some(string_filename_result(
                file,
                RULE_CODE_NAMING_CONVENTION,
                format!(
                    "The stem '{}' does not match the required pattern 'prefix_concept_suffix'. \
                     Expected: lowercase alphanumeric words separated by underscores, minimum {} words. \
                     Example valid names: 'capabilities_user_checker', 'capabilities_db_adapter'. \
                     Issue: '{}' may have uppercase characters, wrong separator, or fewer than {} words.",
                    stem, min_words, stem, min_words
                ),
                Severity::HIGH,
            ));
        }

        None
    }
}

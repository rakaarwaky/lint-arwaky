// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min 3 words)
use async_trait::async_trait;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    ADAPTER_NAME, LAYER_PREFIXES, RULE_CODE_NAMING_CONVENTION, RULE_CODE_SUFFIX_PREFIX,
    SNAKE_CASE_SEPARATOR,
};
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared::naming_rules::utility_naming::get_stem;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
pub struct NamingConventionChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl INamingConventionChecker for NamingConventionChecker {
    async fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let min_words = Self::min_words_from_config(config);
        for f in &files.values {
            let f_str: String = f.to_string();
            let filename: &str = match f.rsplit('/').next() {
                Some(name) => name,
                None => &f_str,
            };
            let layer: Option<String> = self._detect_layer(&f_str, &layer_keys);
            let layer_name: Option<LayerNameVO> =
                layer.as_ref().map(|l: &String| LayerNameVO::new(l.clone()));
            let def: Option<&shared::taxonomy_definition_vo::LayerDefinition> =
                layer_name.as_ref().and_then(|l| layer_map.values.get(l));
            self._check_file_naming(
                &f_str,
                filename,
                &layer_name,
                def,
                config,
                min_words,
                &mut results.values,
            );
        }
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
        static RE3: OnceLock<Option<Regex>> = OnceLock::new();
        static RE4: OnceLock<Option<Regex>> = OnceLock::new();
        let re_lock = match min_words {
            3 => &RE3,
            _ => &RE4,
        };
        re_lock
            .get_or_init(|| {
                let pattern = format!(r"^[a-z0-9]+(_[a-z0-9]+){{{},}}$", min_words - 1);
                Regex::new(&pattern).ok()
            })
            .as_ref()
    }

    fn _detect_layer(&self, file: &str, layer_keys: &[String]) -> Option<String> {
        let filename = utility_layer_detector::extract_filename(file);
        utility_layer_detector::detect_layer_from_prefix(filename)
            .map(|base| utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys))
    }

    fn _make_result(file: &str, code: &str, msg: impl Into<String>, sev: Severity) -> LintResult {
        let file_path = FilePath::new(file).unwrap_or_default();
        LintResult {
            file: file_path,
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw(ADAPTER_NAME)),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min N words).
    fn _check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<LayerNameVO>,
        definition: Option<&shared::taxonomy_definition_vo::LayerDefinition>,
        _config: &ArchitectureConfig,
        min_words: usize,
        violations: &mut Vec<LintResult>,
    ) {
        let layer_prefixes = LAYER_PREFIXES;

        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return;
        }

        if layer_name.is_none() {
            let stem = get_stem(filename).unwrap_or_default();
            let actual_prefix = stem.split('_').next().unwrap_or_default().to_string();

            if !actual_prefix.is_empty() && !layer_prefixes.iter().any(|p| stem.starts_with(p)) {
                let allowed: Vec<String> = layer_prefixes
                    .iter()
                    .map(|p| p.trim_end_matches('_').to_string())
                    .collect();
                violations.push(Self::_make_result(
                    file,
                    RULE_CODE_SUFFIX_PREFIX,
                    NamingViolation::UnknownPrefix {
                        prefix: actual_prefix.clone(),
                        allowed,
                        reason: Some(LintMessage::new(format!(
                            "The prefix '{}' is not one of the {} recognised AES layer prefixes. \
                             Every source file must start with a valid layer prefix so it can be assigned to the correct architectural layer. \
                             Likely causes: typo in the prefix name, or the file is in the wrong directory.",
                            actual_prefix, layer_prefixes.len()
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }

            let stem = get_stem(filename).unwrap_or_default();
            violations.push(Self::_make_result(
                file,
                RULE_CODE_NAMING_CONVENTION,
                NamingViolation::NamingConvention {
                    min_words,
                    separator: SNAKE_CASE_SEPARATOR.to_string(),
                    reason: Some(LintMessage::new(format!(
                        "No architectural layer could be determined for '{}', and the stem '{}' does not follow \
                         the 'prefix_concept_suffix' naming pattern. Files must contain at least {} underscore-separated \
                         lowercase words (e.g., 'capabilities_user_checker'). A valid layer prefix is the first word.",
                        file, stem, min_words
                    ))),
                }
                .to_string(),
                Severity::HIGH,
            ));
            return;
        }

        if let Some(def) = definition {
            if def.exceptions.values.contains(&filename.to_string()) {
                return;
            }
        }

        let stem = get_stem(filename).unwrap_or_default();

        if Self::naming_regex(min_words).is_none_or(|re| !re.is_match(stem)) {
            violations.push(Self::_make_result(
                file,
                RULE_CODE_NAMING_CONVENTION,
                NamingViolation::NamingConvention {
                    min_words,
                    separator: SNAKE_CASE_SEPARATOR.to_string(),
                    reason: Some(LintMessage::new(format!(
                        "The stem '{}' does not match the required pattern 'prefix_concept_suffix'. \
                         Expected: lowercase alphanumeric words separated by underscores, minimum {} words. \
                         Example valid names: 'capabilities_user_checker', 'capabilities_db_adapter'. \
                         Issue: '{}' may have uppercase characters, wrong separator, or fewer than {} words.",
                        stem, min_words, stem, min_words
                    ))),
                }
                .to_string(),
                Severity::HIGH,
            ));
        }
    }
}

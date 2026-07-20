// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min 2 words)
use crate::utility_naming::get_stem;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Clone)]
pub struct NamingConventionChecker {}

static NAMING_REGEX: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)+$").ok());

impl Default for NamingConventionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingConventionChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_result(
        file: &str,
        code: &str,
        msg: impl Into<String>,
        sev: Severity,
    ) -> LintResult {
        let file_path = FilePath::new(file).unwrap_or_default();
        LintResult {
            file: file_path,
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
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

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min 2 words).
    pub fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<LayerNameVO>,
        definition: Option<&LayerDefinition>,
        _config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // AES layer prefixes (must match extract_layer_from_prefix in LayerDetectionAnalyzer)
        const LAYER_PREFIXES: &[&str] = &[
            "taxonomy_",
            "contract_",
            "capabilities_",
            "capabilities_",
            "agent_",
            "surface_",
            "root_",
        ];

        // Step 1: Skip naming verification for barrel files (e.g. __init__.py, mod.rs, index.ts) and entry point files (e.g. main.rs, app.py).
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return;
        }

        // Step 2: Handle cases where the layer could not be determined.
        if layer_name.is_none() {
            let stem = get_stem(filename).unwrap_or_default();
            let actual_prefix = stem.split('_').next().unwrap_or_default().to_string();

            // Check if the file starts with an unrecognized/invalid prefix (not corresponding to a standard AES layer).
            if !actual_prefix.is_empty() && !LAYER_PREFIXES.iter().any(|p| stem.starts_with(p)) {
                let allowed: Vec<String> = LAYER_PREFIXES
                    .iter()
                    .map(|p| p.trim_end_matches('_').to_string())
                    .collect();
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::UnknownPrefix {
                        prefix: actual_prefix.clone(),
                        allowed,
                        reason: Some(LintMessage::new(format!(
                            "The prefix '{}' is not one of the {} recognised AES layer prefixes. \
                             Every source file must start with a valid layer prefix so it can be assigned to the correct architectural layer. \
                             Likely causes: typo in the prefix name, or the file is in the wrong directory.",
                            actual_prefix, LAYER_PREFIXES.len()
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }

            // If the prefix is recognized or is empty, but there is no underscore or does not meet basic naming requirements.
            let stem = get_stem(filename).unwrap_or_default();
            violations.push(Self::make_result(
                file,
                "AES101",
                NamingViolation::NamingConvention {
                    min_words: 2,
                    separator: "_".to_string(),
                    reason: Some(LintMessage::new(format!(
                        "No architectural layer could be determined for '{}', and the stem '{}' does not follow \
                         the 'prefix_concept_suffix' naming pattern. Files must contain at least 2 underscore-separated \
                         lowercase words (e.g., 'capabilities_user_checker'). A valid layer prefix is the first word.",
                        file, stem
                    ))),
                }
                .to_string(),
                Severity::HIGH,
            ));
            return;
        }

        // Step 3: Skip validation if the file name is explicitly listed in the layer's exception config.
        if let Some(def) = definition {
            if def.exceptions.values.contains(&filename.to_string()) {
                return;
            }
        }

        // Step 4: Validate the file stem pattern using a regular expression.
        // It must consist of lowercase letters and digits separated by underscores (e.g., prefix_concept_suffix).
        let stem = get_stem(filename).unwrap_or_default();

        if NAMING_REGEX.as_ref().is_none_or(|re| !re.is_match(stem)) {
            violations.push(Self::make_result(
                file,
                "AES101",
                NamingViolation::NamingConvention {
                    min_words: 2,
                    separator: "_".to_string(),
                    reason: Some(LintMessage::new(format!(
                        "The stem '{}' does not match the required pattern 'prefix_concept_suffix'. \
                         Expected: lowercase alphanumeric words separated by underscores, minimum 2 words. \
                         Example valid names: 'capabilities_user_checker', 'capabilities_db_adapter'. \
                         Issue: '{}' may have uppercase characters, wrong separator, or only 1 word.",
                        stem, stem
                    ))),
                }
                .to_string(),
                Severity::HIGH,
            ));
        }
    }
}

#[async_trait]
impl INamingCheckerProtocol for NamingConventionChecker {
    // Implement check_file_naming from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_file_naming(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Step 1: Iterate over each file path in the checklist.
        for f in &files.values {
            let f_str = f.to_string();
            // Step 2: Extract the raw filename from the path.
            let filename = match f.rsplit('/').next() {
                Some(name) => name,
                None => &f_str,
            };
            // Step 3: Determine the architectural layer for the file.
            let layer = analyzer.detect_layer(f, root_dir);
            // Step 4: Fetch layer-specific definition properties.
            let def = layer
                .as_ref()
                .and_then(|l| analyzer.layer_map().values.get(l));
            // Step 5: Execute the naming checker function.
            self.check_file_naming(
                &f_str,
                filename,
                &layer,
                def,
                analyzer.config(),
                &mut results.values,
            );
        }
    }

    async fn check_domain_suffixes(
        &self,
        _analyzer: &dyn INamingAnalyzerProtocol,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for naming convention checker
    }
}

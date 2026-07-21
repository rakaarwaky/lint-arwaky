// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict)
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::ISuffixPrefixChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    ADAPTER_NAME, RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT,
};
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
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

use shared::naming_rules::utility_naming::{get_stem, get_suffix};

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
        for f in &files.values {
            let f_str = f.to_string();
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            let layer = self._detect_layer(&f_str, &layer_keys);
            let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));
            let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
            self._check_domain_suffixes(&f_str, filename, def, &layer_name, &mut results.values);
        }
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

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules).
    fn _check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&shared::taxonomy_definition_vo::LayerDefinition>,
        _layer_name: &Option<LayerNameVO>,
        violations: &mut Vec<LintResult>,
    ) {
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let stem = match get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = get_suffix(stem);

        if let Some(suf) = &suffix {
            if def.naming.forbidden_suffix.values.iter().any(|v| v == *suf) {
                let layer_display = _layer_name
                    .as_ref()
                    .map(|l| l.value().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                violations.push(Self::_make_result(
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
                return;
            }
        }

        if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.iter().any(|v| v == *s),
                None => false,
            };
            if !valid {
                let allowed_list = def.naming.allowed_suffix.values.clone();
                let layer_display = _layer_name
                    .as_ref()
                    .map(|l| l.value().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                let suffix_display = suffix.unwrap_or("(none)");
                violations.push(Self::_make_result(
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
    }
}

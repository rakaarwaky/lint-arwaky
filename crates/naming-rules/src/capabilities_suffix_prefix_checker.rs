// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict)
use shared::naming_rules::utility_naming::{get_stem, get_suffix};
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::taxonomy_naming_constant::{ADAPTER_NAME, RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT};
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

// ─── Block 1: Struct Definition ───────────────────────────
#[derive(Clone)]
pub struct SuffixPrefixChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait]
impl INamingCheckerProtocol for SuffixPrefixChecker {
    async fn check_file_naming(
        &self,
        _analyzer: &dyn INamingAnalyzerProtocol,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for suffix/prefix checker
    }

    // Implement check_domain_suffixes from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_domain_suffixes(
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
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            // Step 3: Determine the architectural layer for the file.
            let layer = analyzer.detect_layer(f, root_dir);
            // Step 4: Fetch layer-specific definition properties.
            let def = layer
                .as_ref()
                .and_then(|l| analyzer.layer_map().values.get(l));
            // Step 5: Execute the suffix checker function.
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
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
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<LayerNameVO>,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip checking for barrel files and system entry point files.
        let fp = FilePath::new(filename.to_string()).unwrap_or_default();
        if fp.is_barrel_file() || fp.is_entry_point() {
            return;
        }

        // Step 2: Retrieve the layer definition config.
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        // Step 3: Skip validation if the filename matches an exception rule.
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        // Step 4: Extract the file stem (name without extension) and get the suffix (word after the last underscore).
        let stem = match get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = get_suffix(stem);

        // Step 5: Check if the suffix is explicitly forbidden for the current layer.
        if let Some(suf) = &suffix {
            if def.naming.forbidden_suffix.values.iter().any(|v| v == *suf) {
                let layer_display = _layer_name
                    .as_ref()
                    .map(|l| l.value().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                violations.push(Self::make_result(
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

        // Step 6: If the layer configuration enforces a strict suffix policy, ensure the suffix matches the allowed list.
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
                violations.push(Self::make_result(
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



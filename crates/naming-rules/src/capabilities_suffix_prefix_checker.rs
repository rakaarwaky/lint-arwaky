// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict)
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
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
pub struct SuffixPrefixChecker {}

impl Default for SuffixPrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPrefixChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn make_result(file: &str, code: &str, msg: impl Into<String>, sev: Severity) -> LintResult {
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
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

    fn is_barrel_file(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    fn is_entry_point(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }

    fn get_stem(filename: &str) -> Option<String> {
        if let Some(pos) = filename.rfind('.') {
            Some(filename[..pos].to_string())
        } else {
            Some(filename.to_string())
        }
    }

    fn get_suffix(stem: &str) -> Option<String> {
        stem.rfind('_').map(|pos| stem[pos + 1..].to_string())
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules).
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip checking for barrel files and system entry point files.
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
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
        let stem = match Self::get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = Self::get_suffix(&stem);

        // Step 5: Check if the suffix is explicitly forbidden for the current layer.
        if let Some(ref suf) = suffix {
            if def.naming.forbidden_suffix.values.contains(suf) {
                let layer_display =
                    _layer_name.as_deref().unwrap_or("unknown").to_string();
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixForbidden {
                        layer_name: layer_display.clone(),
                        forbidden_suffix: suf.clone(),
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
        if def.naming.suffix_policy.value == "strict" {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.contains(s),
                None => false,
            };
            if !valid {
                let allowed_list = def.naming.allowed_suffix.values.clone();
                let layer_display =
                    _layer_name.as_deref().unwrap_or("unknown").to_string();
                let suffix_display = suffix.as_deref().unwrap_or("(none)");
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixMismatch {
                        layer_name: layer_display.clone(),
                        allowed: allowed_list.clone(),
                        reason: Some(LintMessage::new(format!(
                            "Layer '{}' enforces a strict suffix policy, but the file uses suffix '{}'. \
                             Expected one of: {}. \
                             The suffix determines the file's architectural role — a missing or incorrect suffix \
                             breaks the layer-to-role mapping that automated checks depend on.",
                            layer_display,
                            suffix_display,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_barrel_file_recognizes_mod_rs() {
        assert!(SuffixPrefixChecker::is_barrel_file("mod.rs"));
    }

    #[test]
    fn is_barrel_file_recognizes_init_py() {
        assert!(SuffixPrefixChecker::is_barrel_file("__init__.py"));
    }

    #[test]
    fn is_barrel_file_rejects_normal_file() {
        assert!(!SuffixPrefixChecker::is_barrel_file("checker.rs"));
    }

    #[test]
    fn is_entry_point_recognizes_main_rs() {
        assert!(SuffixPrefixChecker::is_entry_point("main.rs"));
    }

    #[test]
    fn is_entry_point_recognizes_lib_rs() {
        assert!(SuffixPrefixChecker::is_entry_point("lib.rs"));
    }

    #[test]
    fn is_entry_point_rejects_regular_file() {
        assert!(!SuffixPrefixChecker::is_entry_point("service.rs"));
    }

    #[test]
    fn get_stem_removes_extension() {
        assert_eq!(SuffixPrefixChecker::get_stem("checker.rs"), Some("checker".to_string()));
    }

    #[test]
    fn get_stem_handles_no_extension() {
        assert_eq!(SuffixPrefixChecker::get_stem("checker"), Some("checker".to_string()));
    }

    #[test]
    fn get_stem_handles_multiple_dots() {
        assert_eq!(SuffixPrefixChecker::get_stem("my.test.file.rs"), Some("my.test.file".to_string()));
    }

    #[test]
    fn get_suffix_returns_last_underscore_part() {
        assert_eq!(SuffixPrefixChecker::get_suffix("capabilities_checker"), Some("checker".to_string()));
    }

    #[test]
    fn get_suffix_no_underscore_returns_none() {
        assert_eq!(SuffixPrefixChecker::get_suffix("checker"), None);
    }

    #[test]
    fn get_suffix_single_underscore() {
        assert_eq!(SuffixPrefixChecker::get_suffix("_checker"), Some("checker".to_string()));
    }

    #[test]
    fn check_domain_suffixes_skips_barrel_file() {
        let checker = SuffixPrefixChecker::new();
        let mut violations = Vec::new();
        checker.check_domain_suffixes("mod.rs", "mod.rs", None, &None, &mut violations);
        assert!(violations.is_empty(), "barrel files should be skipped");
    }

    #[test]
    fn check_domain_suffixes_skips_entry_point() {
        let checker = SuffixPrefixChecker::new();
        let mut violations = Vec::new();
        checker.check_domain_suffixes("main.rs", "main.rs", None, &None, &mut violations);
        assert!(violations.is_empty(), "entry points should be skipped");
    }

    #[test]
    fn check_domain_suffixes_no_definition_no_op() {
        let checker = SuffixPrefixChecker::new();
        let mut violations = Vec::new();
        checker.check_domain_suffixes("random.rs", "random.rs", None, &None, &mut violations);
        assert!(violations.is_empty(), "no definition means no check");
    }

    #[test]
    fn check_domain_suffixes_skips_exceptions() {
        use shared::taxonomy_common_vo::PatternList;
        use shared::taxonomy_definition_vo::LayerDefinition;

        let checker = SuffixPrefixChecker::new();
        let mut violations = Vec::new();
        let def = LayerDefinition {
            exceptions: PatternList::new(vec!["skip.rs".to_string()]),
            ..Default::default()
        };
        checker.check_domain_suffixes("skip.rs", "skip.rs", Some(&def), &Some("capabilities".to_string()), &mut violations);
        assert!(violations.is_empty(), "exceptions should be skipped");
    }

    #[test]
    fn make_result_produces_lint_result_with_code() {
        let result = SuffixPrefixChecker::make_result("test.rs", "AES102", "msg", Severity::HIGH);
        assert_eq!(result.code.to_string(), "AES102");
        assert_eq!(result.message.to_string(), "msg");
        assert_eq!(result.severity, Severity::HIGH);
    }
}

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
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            // Step 4: Fetch layer-specific definition properties.
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            // Step 5: Execute the suffix checker function.
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
        }
    }
}

// PURPOSE: ArchNamingChecker — INamingCheckerProtocol for AES011 (naming convention) and AES012 (suffix/prefix rules)

use async_trait::async_trait;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
use shared::output_report::taxonomy_result_vo::{LintResult, LintResultList};
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
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
fn aes011_naming_convention(_expected_word_count: i32) -> String {
    String::from(
        "AES011 NAMING_CONVENTION: Filename must follow prefix_concept_suffix pattern.\n\
        WHY? Prefix identifies layer, suffix identifies role, concept describes feature.\n\
        FIX: Rename to at least prefix_suffix (e.g., capabilities_user_checker.rs).",
    )
}

use regex::Regex;
use shared::taxonomy_violation_message_rs_error::AesViolation;

#[derive(Clone)]
pub struct ArchNamingChecker {}

impl Default for ArchNamingChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchNamingChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn make_result(file: &str, code: &str, msg: impl Into<String>, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
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

    /// Check file naming conventions (AES011: pattern validation — lowercase, underscore, min 2 words).
    pub fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        _config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        if let Some(def) = definition {
            if def.exceptions.values.contains(&filename.to_string()) {
                return;
            }
        }

        let stem = filename.split('.').next().unwrap_or("");
        // Unlimited words: at least `prefix_suffix` (2 words), any concept words in between
        let naming_regex = r"^[a-z0-9]+(_[a-z0-9]+)+$";

        if let Ok(re) = Regex::new(naming_regex) {
            if !re.is_match(stem) {
                violations.push(Self::make_result(
                    file,
                    "AES011",
                    aes011_naming_convention(0),
                    Severity::HIGH,
                ));
            }
        }
    }

    /// Check domain suffix rules per layer (AES012: suffix/prefix rules).
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let stem = match Self::get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = Self::get_suffix(&stem);

        // AES012: Forbidden suffix check
        if let Some(ref suf) = suffix {
            if def.forbidden_suffix.values.contains(suf) {
                violations.push(Self::make_result(
                    file,
                    "AES012",
                    AesViolation::SuffixForbidden { reason: None }.to_string(),
                    Severity::HIGH,
                ));
                return;
            }
        }

        // AES012: Strict suffix policy check
        if def.suffix_policy.value == "strict" {
            let valid = suffix
                .as_ref()
                .map(|s| def.allowed_suffix.values.contains(s))
                .unwrap_or(false);
            if !valid {
                let allowed_list = def.allowed_suffix.values.join(", ");
                if _layer_name.as_ref().map(|l| l.as_str()) == Some("contract") {
                    violations.push(Self::make_result(
                        file,
                        "AES012",
                        AesViolation::SuffixMismatch { reason: None }.to_string(),
                        Severity::HIGH,
                    ));
                } else {
                    let msg = format!(
                        "AES012 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n\
                        WHY? Suffix/prefix rules require specific suffix per layer.\n\
                        FIX: Add one of the required suffixes: {}.",
                        allowed_list
                    );
                    violations.push(Self::make_result(file, "AES012", &msg, Severity::HIGH));
                }
            }
        }
    }
}

#[async_trait]
impl INamingCheckerProtocol for ArchNamingChecker {
    async fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
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
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
        }
    }
}

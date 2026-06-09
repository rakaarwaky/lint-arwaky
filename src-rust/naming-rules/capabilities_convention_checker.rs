// naming_rule_checker — File naming convention checks.
// Implements INamingRuleProtocol: validates snake_case, PascalCase, word counts.

use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::shared_common::taxonomy_layer_vo::Identity;
use /* UNKNOWN: LayerNameVO */ crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use /* UNKNOWN: LineNumber */ crate::shared_common::taxonomy_common_vo::LineNumber;
use /* UNKNOWN: LintMessage */ crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_violation_constant::aes003_naming_convention;
use /* UNKNOWN: LocationList */ crate::shared_common::taxonomy_lint_vo::LocationList;
use /* UNKNOWN: ScopeRef */ crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::output_report::taxonomy_severity_vo::Severity;
use regex::Regex;
use std::path::Path;

pub struct NamingRuleChecker {}

impl NamingRuleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rule_name(&self) -> Identity {
        Identity::new("naming")
    }

    fn has_snake_case(name: &str) -> bool {
        let re = match Regex::new(r"^_?[a-z][a-z0-9]*(_[a-z0-9]+)*$") {
            Ok(r) => r,
            Err(_) => return false,
        };
        re.is_match(name)
    }

    fn get_stem(basename: &str) -> &str {
        if let Some(pos) = basename.rfind('.') {
            &basename[..pos]
        } else {
            basename
        }
    }

    fn is_special_file(basename: &str) -> bool {
        matches!(
            basename,
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "mod.rs"
                | "lib.rs"
                | "index.ts"
                | "index.js"
                | "app.py"
        )
    }

    fn make_lint_result(
        file: &str,
        line: i64,
        col: i64,
        code: &str,
        msg: &str,
        sev: Severity,
    ) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string())
                .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default()),
            line: LineNumber::new(line),
            column: ColumnNumber::new(col),
            code: ErrorCode::new(code).unwrap_or_else(|_| ErrorCode::raw(code)),
            message: LintMessage::new(msg),
            source: Some(
                AdapterName::new("architecture")
                    .unwrap_or_else(|_| AdapterName::raw("architecture")),
            ),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    /// Check that files have the correct number of underscore-separated words.
    pub fn check_file_naming(
        &self,
        files: &[String],
        root_dir: &str,
        config: &ArchitectureConfig,
        results: &mut Vec<LintResult>,
        detect_layer_fn: impl Fn(&str, &str) -> Option<String>,
    ) {
        let global_expected = config.naming.word_count.value as i32;

        for file in files {
            let basename = Path::new(file)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("");

            if basename.is_empty() || Self::is_special_file(basename) {
                continue;
            }

            let layer_name = detect_layer_fn(file, root_dir);

            let (expected, exceptions) = if let Some(ref name) = layer_name {
                let key = LayerNameVO::new(name);
                if let Some(def) = config.layers.get(&key) {
                    let exp = if def.word_count.value > 0 {
                        def.word_count.value as i32
                    } else {
                        global_expected
                    };
                    (exp, def.exceptions.values.clone())
                } else {
                    (global_expected, vec![])
                }
            } else {
                (global_expected, vec![])
            };

            if exceptions.contains(&basename.to_string()) {
                continue;
            }

            let stem = Self::get_stem(basename);
            let actual_words = stem.split('_').count() as i32;

            if actual_words != expected {
                let msg = aes003_naming_convention(expected);
                results.push(Self::make_lint_result(
                    file,
                    1,
                    1,
                    "AES003",
                    &msg,
                    Severity::HIGH,
                ));
            }
        }
    }

    /// Check that class names follow PascalCase.
    pub fn check_class_naming_raw(
        &self,
        file: &str,
        class_name: &str,
        line: i64,
        column: i64,
        results: &mut Vec<LintResult>,
    ) {
        let is_pascal = class_name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
            && !class_name.contains('_');

        if !is_pascal {
            results.push(Self::make_lint_result(
                file,
                line,
                column,
                "NAMING_CLASS_PASCAL_CASE",
                &format!("Class '{}' should be PascalCase", class_name),
                Severity::HIGH,
            ));
        }
    }

    /// Check that function names follow snake_case (skipping dunder methods).
    pub fn check_function_naming_raw(
        &self,
        file: &str,
        func_name: &str,
        line: i64,
        column: i64,
        results: &mut Vec<LintResult>,
    ) {
        if func_name.starts_with("__") && func_name.ends_with("__") {
            return;
        }

        if !Self::has_snake_case(func_name) {
            results.push(Self::make_lint_result(
                file,
                line,
                column,
                "NAMING_FUNCTION_SNAKE_CASE",
                &format!("Function '{}' should be snake_case", func_name),
                Severity::HIGH,
            ));
        }
    }
}

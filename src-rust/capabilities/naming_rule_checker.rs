// naming_rule_checker — File naming convention checks.
// Implements INamingRuleProtocol: validates snake_case, PascalCase, word counts.

use std::path::Path;
use regex::Regex;
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath,
    LayerNameVO, LintMessage, LintResult, LineNumber,
    Severity, Identity, ArchitectureConfig, ScopeRef, LocationList,
};

pub struct NamingRuleChecker;

impl NamingRuleChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn rule_name(&self) -> Identity {
        Identity::new("naming")
    }

    fn has_snake_case(name: &str) -> bool {
        let re = Regex::new(r"^_?[a-z][a-z0-9]*(_[a-z0-9]+)*$").unwrap();
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
        matches!(basename,
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs"
            | "index.ts" | "index.js" | "app.py"
        )
    }

    fn make_lint_result(file: &str, line: i64, col: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap(),
            line: LineNumber::new(line),
            column: ColumnNumber::new(col),
            code: ErrorCode::new(code).unwrap(),
            message: LintMessage::new(msg),
            source: Some(AdapterName::new("architecture").unwrap()),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: String::new(),
                kind: String::new(),
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
                let msg = if let Some(ref name) = layer_name {
                    let key = LayerNameVO::new(name);
                    config.layers.get(&key)
                        .filter(|def| !def.word_count_violation_message.value.is_empty())
                        .map(|def| def.word_count_violation_message.value.clone())
                        .unwrap_or_else(|| format!(
                            "AES003 NAMING_CONVENTION: File '{}' has {} words, expected {}.\n\
                            WHY? Strict naming ensures architectural consistency.\n\
                            FIX: Rename to exactly {} words separated by underscores.",
                            basename, actual_words, expected, expected
                        ))
                } else {
                    format!(
                        "AES003 NAMING_CONVENTION: File '{}' has {} words, expected {}.\n\
                        WHY? Strict naming ensures architectural consistency.\n\
                        FIX: Rename to exactly {} words separated by underscores.",
                        basename, actual_words, expected, expected
                    )
                };
                results.push(Self::make_lint_result(file, 1, 1, "AES003", &msg, Severity::HIGH));
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
        let is_pascal = class_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            && !class_name.contains('_');

        if !is_pascal {
            results.push(Self::make_lint_result(
                file, line, column,
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
                file, line, column,
                "NAMING_FUNCTION_SNAKE_CASE",
                &format!("Function '{}' should be snake_case", func_name),
                Severity::HIGH,
            ));
        }
    }
}

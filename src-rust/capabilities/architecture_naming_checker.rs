// arch_naming_checker — Architectural naming convention checks.
// Implements INamingCheckerProtocol: check_file_naming and check_domain_suffixes.

use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LineNumber, LintMessage, LintResult, LocationList, ScopeRef, Severity,
};
use regex::Regex;

pub struct ArchNamingChecker {}

impl ArchNamingChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn make_result(file: &str, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::taxonomy::DescriptionVO::new(String::new()),
                kind: crate::taxonomy::DescriptionVO::new(String::new()),
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
        if let Some(pos) = stem.rfind('_') {
            Some(stem[pos + 1..].to_string())
        } else {
            None
        }
    }

    /// Check file naming conventions (underscore word count) per layer definition.
    pub fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        _layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        config: &ArchitectureConfig,
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

        let expected_word_count = if let Some(def) = definition {
            if def.word_count.value > 0 {
                def.word_count.value as i32
            } else {
                config.naming.word_count.value as i32
            }
        } else {
            config.naming.word_count.value as i32
        };

        let stem = filename.split('.').next().unwrap_or("");
        let naming_regex = format!(r"^[a-z0-9]+(_[a-z0-9]+){{{}}}$", expected_word_count - 1);

        if let Ok(re) = Regex::new(&naming_regex) {
            if !re.is_match(stem) {
                let msg = if let Some(def) = definition {
                    if !def.word_count_violation_message.value.is_empty() {
                        def.word_count_violation_message.value.clone()
                    } else if !config.naming.word_count_violation_message.value.is_empty() {
                        config.naming.word_count_violation_message.value.clone()
                    } else {
                        format!(
                            "AES003 NAMING_CONVENTION: Filename does not follow the {}-word underscore-separated pattern.\n\
                            WHY? Strict three-word names ensure architectural consistency.\n\
                            FIX: Rename the file to exactly {} words separated by underscores.",
                            expected_word_count, expected_word_count
                        )
                    }
                } else {
                    format!(
                        "AES003 NAMING_CONVENTION: Filename does not follow the {}-word pattern.",
                        expected_word_count
                    )
                };
                violations.push(Self::make_result(file, "AES003", &msg, Severity::HIGH));
            }
        }
    }

    /// Check domain suffix rules per layer (AES010/AES011).
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

        // AES011: Forbidden suffix check
        if let Some(ref suf) = suffix {
            if def.forbidden_suffix.values.contains(suf) {
                let msg = if !def.suffix_violation_message.value.is_empty() {
                    def.suffix_violation_message.value.clone()
                } else {
                    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
                    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.".to_string()
                };
                violations.push(Self::make_result(file, "AES011", &msg, Severity::HIGH));
                return;
            }
        }

        // AES010: Strict suffix policy check
        if def.suffix_policy.value == "strict" {
            let valid = suffix
                .as_ref()
                .map(|s| def.allowed_suffix.values.contains(s))
                .unwrap_or(false);
            if !valid {
                let allowed_list = def.allowed_suffix.values.join(", ");
                let msg = if !def.suffix_violation_message.value.is_empty() {
                    def.suffix_violation_message.value.clone()
                } else {
                    format!(
                        "AES011 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n\
                        WHY? Strict suffixes ensure every component has a clear role.\n\
                        FIX: Add one of the required suffixes: {}.",
                        allowed_list
                    )
                };
                let code = if _layer_name.as_ref().map(|l| l.as_str()) == Some("contract") {
                    "AES008"
                } else {
                    "AES010"
                };
                violations.push(Self::make_result(file, code, &msg, Severity::HIGH));
            }
        }
    }
}

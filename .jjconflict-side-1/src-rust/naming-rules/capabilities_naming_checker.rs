// arch_naming_checker — Architectural naming convention checks.
// Implements INamingCheckerProtocol: check_file_naming and check_domain_suffixes.

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes010_naming_convention, AES011_SUFFIX_FORBIDDEN, AES011_SUFFIX_MISMATCH,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;
use regex::Regex;

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

    fn make_result(file: &str, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
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

    /// Check file naming conventions (underscore word count) per layer definition.
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
                    "AES010",
                    &aes010_naming_convention(0),
                    Severity::HIGH,
                ));
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
                violations.push(Self::make_result(
                    file,
                    "AES011",
                    AES011_SUFFIX_FORBIDDEN,
                    Severity::HIGH,
                ));
                return;
            }
        }

        // AES001: Strict suffix policy check
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
                        "AES011",
                        AES011_SUFFIX_MISMATCH,
                        Severity::HIGH,
                    ));
                } else {
                    let msg = format!(
                        "AES011 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n\
                        WHY? Strict suffixes ensure every component has a clear role.\n\
                        FIX: Add one of the required suffixes: {}.",
                        allowed_list
                    );
                    violations.push(Self::make_result(file, "AES011", &msg, Severity::HIGH));
                }
            }
        }
    }
}

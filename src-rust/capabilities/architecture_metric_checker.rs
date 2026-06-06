// arch_metric_checker — Architectural metric checks (line counts, mandatory classes).
// Implements IMetricCheckerProtocol: check_line_counts, check_mandatory_class_definition.

use std::fs;
use std::path::Path;
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LayerNameVO, LintMessage, LintResult, LineNumber, Severity,
    ScopeRef, LocationList, ArchitectureConfig,
};

pub struct ArchMetricChecker;

impl ArchMetricChecker {
    pub fn new() -> Self {
        Self
    }

    fn make_result(file: &str, line: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap(),
            line: LineNumber::new(line),
            column: ColumnNumber::new(0),
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

    fn count_lines(file: &str) -> i64 {
        fs::read_to_string(file)
            .map(|c| c.lines().count() as i64)
            .unwrap_or(0)
    }

    fn get_basename(file: &str) -> String {
        Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string()
    }

    fn file_has_class_definition(file: &str) -> bool {
        if let Ok(content) = fs::read_to_string(file) {
            // Check for class definitions in Python, Rust, TypeScript/JavaScript
            return content.contains("\nclass ")
                || content.starts_with("class ")
                || content.contains("\npub struct ")
                || content.contains("\nstruct ")
                || content.contains("\nexport class ")
                || content.contains("\nexport default class ");
        }
        false
    }

    /// Check file line counts against min/max thresholds (AES004/AES005).
    pub fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Self::get_basename(file);

        // Skip barrel/entry files
        if basename == "__init__.py" || basename == "mod.rs" {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let count = Self::count_lines(file);

        // AES005: Too short
        if def.min_lines.value > 0 && count < def.min_lines.value {
            let msg = if !def.min_lines_violation_message.value.is_empty() {
                def.min_lines_violation_message.value.clone()
            } else {
                format!(
                    "AES005 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? Excessively small files clutter the project structure.\n\
                    FIX: Expand the component or merge this logic into a related module (min: {}).",
                    def.min_lines.value
                )
            };
            violations.push(Self::make_result(file, 0, "AES005", &msg, Severity::HIGH));
        }

        // AES004: Too large
        if def.max_lines.value > 0 && count > def.max_lines.value {
            let msg = if !def.max_lines_violation_message.value.is_empty() {
                def.max_lines_violation_message.value.clone()
            } else {
                format!(
                    "AES004 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? Large files violate the Single Responsibility Principle.\n\
                    FIX: Split the module into smaller, more focused files (max: {}).",
                    def.max_lines.value
                )
            };
            violations.push(Self::make_result(file, 0, "AES004", &msg, Severity::HIGH));
        }
    }

    /// Check mandatory class definition requirement (AES009).
    pub fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Self::get_basename(file);

        // Skip special files
        if matches!(basename.as_ref(), "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs") {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if !def.mandatory_class_definition.value {
            return;
        }

        if def.exceptions.values.contains(&basename) {
            return;
        }

        if !Self::file_has_class_definition(file) {
            let msg = if !def.mandatory_class_definition_violation_message.value.is_empty() {
                def.mandatory_class_definition_violation_message.value.clone()
            } else {
                "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.\n\
                WHY? Encapsulation in classes is required for proper dependency injection.\n\
                FIX: Move standalone functions into a class that implements its corresponding domain contract.".to_string()
            };
            violations.push(Self::make_result(file, 0, "AES009", &msg, Severity::HIGH));
        }
    }
}

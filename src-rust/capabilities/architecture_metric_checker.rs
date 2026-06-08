// arch_metric_checker — Architectural metric checks (line counts, mandatory classes).
// Implements IMetricCheckerProtocol: check_line_counts, check_mandatory_class_definition.

use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerDefinition, LineNumber, LintMessage,
    LintResult, LocationList, ScopeRef, Severity,
};
use std::fs;
use std::path::Path;

pub struct ArchMetricChecker;

impl ArchMetricChecker {
    pub fn new() -> Self {
        Self
    }

    fn make_result(file: &str, line: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line),
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
            // Check for class/trait/enum definitions in Python, Rust, TypeScript/JavaScript
            return content.contains("\nclass ")
                || content.starts_with("class ")
                || content.contains("\npub struct ")
                || content.contains("\nstruct ")
                || content.contains("\npub trait ")
                || content.contains("\ntrait ")
                || content.contains("\npub enum ")
                || content.contains("\nenum ")
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

    /// Check constant purity in _constant files (AES033).
    /// _constant files may ONLY contain pub const / pub static declarations.
    pub fn check_constant_purity(
        &self,
        file: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Self::get_basename(file);
        if !basename.ends_with("_constant.rs") && !basename.ends_with("_constant.py") {
            return;
        }

        if let Ok(content) = fs::read_to_string(file) {
            for (i, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                let lineno = (i + 1) as i64;

                // Skip empty lines, comments, attributes
                if trimmed.is_empty()
                    || trimmed.starts_with("//")
                    || trimmed.starts_with('#')
                    || trimmed.starts_with("#[")
                {
                    continue;
                }

                // ALLOWED: pub const, pub static
                if trimmed.starts_with("pub const ") || trimmed.starts_with("pub static ") {
                    continue;
                }

                // ALLOWED: use/import statements (needed for type annotations in consts)
                if trimmed.starts_with("use ") || trimmed.starts_with("pub(crate) use ") {
                    continue;
                }

                // FORBIDDEN: struct, enum, fn, impl, mod, trait, type, pub use
                if trimmed.starts_with("pub struct ")
                    || trimmed.starts_with("struct ")
                    || trimmed.starts_with("pub enum ")
                    || trimmed.starts_with("enum ")
                    || trimmed.starts_with("pub fn ")
                    || trimmed.starts_with("fn ")
                    || trimmed.starts_with("impl ")
                    || trimmed.starts_with("pub mod ")
                    || trimmed.starts_with("mod ")
                    || trimmed.starts_with("pub trait ")
                    || trimmed.starts_with("trait ")
                    || trimmed.starts_with("pub use ")
                    || trimmed.starts_with("pub type ")
                    || trimmed.starts_with("type ")
                {
                    let msg = "AES033 CONSTANT_PURITY: Taxonomy _constant file contains non-constant declaration.\n\
                        WHY? _constant files must contain ONLY pub const / pub static declarations.\n\
                        FIX: Move non-constant declarations to the appropriate _vo or _entity file."
                        .to_string();
                    violations.push(Self::make_result(file, lineno, "AES033", &msg, Severity::HIGH));
                }
            }
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
        if matches!(
            basename.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs"
        ) {
            return;
        }

        // AES033 constant-purity rule overrides AES009: _constant files must only have const/static
        if basename.ends_with("_constant.rs") || basename.ends_with("_constant.py") {
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
            let msg = if !def
                .mandatory_class_definition_violation_message
                .value
                .is_empty()
            {
                def.mandatory_class_definition_violation_message
                    .value
                    .clone()
            } else {
                "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.\n\
                WHY? Encapsulation in classes is required for proper dependency injection.\n\
                FIX: Move standalone functions into a class that implements its corresponding domain contract.".to_string()
            };
            violations.push(Self::make_result(file, 0, "AES009", &msg, Severity::HIGH));
        }
    }
}

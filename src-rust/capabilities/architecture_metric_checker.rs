// arch_metric_checker — Architectural metric checks (line counts, mandatory classes).
// Implements IMetricCheckerProtocol: check_line_counts, check_mandatory_class_definition.

use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerDefinition, LineNumber, LintMessage,
    LintResult, LocationList, ScopeRef, Severity,
};
use std::fs;
use std::path::Path;

pub struct ArchMetricChecker {}

impl ArchMetricChecker {
    pub fn new() -> Self {
        Self {}
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

    fn count_lines(file: &str) -> Option<i64> {
        fs::read_to_string(file)
            .map(|c| Some(c.lines().count() as i64))
            .unwrap_or(None)
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
            return content.contains("\nclass ")
                || content.starts_with("class ")
                || content.contains("\npub struct ")
                || content.starts_with("pub struct ")
                || content.contains("\nstruct ")
                || content.starts_with("struct ")
                || content.contains("\npub trait ")
                || content.starts_with("pub trait ")
                || content.contains("\ntrait ")
                || content.starts_with("trait ")
                || content.contains("\npub enum ")
                || content.starts_with("pub enum ")
                || content.contains("\nenum ")
                || content.starts_with("enum ")
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

        let count = match Self::count_lines(file) {
            Some(c) => c,
            None => return, // unreadable file — skip to avoid false positive AES005
        };

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
    pub fn check_constant_purity(&self, file: &str, violations: &mut Vec<LintResult>) {
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
                    violations.push(Self::make_result(
                        file,
                        lineno,
                        "AES033",
                        &msg,
                        Severity::HIGH,
                    ));
                }
            }
        }
    }

    /// Check primitive usage in taxonomy/contract files (AES006).
    pub fn check_primitive_usage(
        &self,
        file: &str,
        content: &str,
        filename: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if !def.no_primitives.value { return; }
        if filename.ends_with("_vo.rs")
            || filename.ends_with("_vo.py")
            || filename.ends_with("_constant.rs")
            || filename.ends_with("_constant.py")
        {
            return;
        }
        let (rust_primitives, py_primitives, js_primitives) = (
            &["String","i8","i16","i32","i64","i128","isize","u8","u16","u32","u64","u128","usize","f32","f64","bool","char","Vec<","HashMap<","Option<","Result<","Box<","Cell<","RefCell<","Arc<","Mutex<","Rc<"][..],
            &["str","int","float","bool","list","dict","tuple","set","bytes","None","Any","Optional","Union","List","Dict","Tuple","Set","FrozenSet"][..],
            &["string","number","boolean","any","object","Array","Record","Map","Set","Promise","unknown","never","void","null","undefined","bigint","symbol"][..],
        );
        let primitives: &[&str] = if file.ends_with(".rs") { rust_primitives }
            else if file.ends_with(".py") { py_primitives }
            else if file.ends_with(".ts") || file.ends_with(".tsx") || file.ends_with(".js") || file.ends_with(".jsx") { js_primitives }
            else { return };
        let msg = if !def.no_primitives_violation_message.value.is_empty() {
            def.no_primitives_violation_message.value.clone()
        } else {
            String::new()
        };
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.contains(':') { continue; }
            if !(t.ends_with(',') || t.ends_with('}') || t.ends_with(')') || t.contains("-> ")) { continue; }
            let after_colon = if let Some((_, rest)) = t.split_once(':') { rest.trim() } else { continue };
            let type_candidate = after_colon.trim_end_matches(',').trim_end_matches(')').trim_end_matches('}').trim();
            for p in primitives {
                if type_candidate.starts_with(p) || type_candidate == p.trim_end_matches('<') {
                    let violation_msg = if msg.is_empty() {
                        format!("AES006 PRIMITIVE_USAGE: Direct primitive '{}' in taxonomy.", p)
                    } else {
                        msg.clone()
                    };
                    violations.push(Self::make_result(file, (i + 1) as i64, "AES006", &violation_msg, Severity::HIGH));
                    break;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::taxonomy::{
        BooleanVO, Count, DirectoryPath, ErrorMessage, PatternList, SuffixPolicyVO,
    };

    fn test_definition() -> LayerDefinition {
        LayerDefinition {
            path: DirectoryPath::new("src-rust/taxonomy".to_string()).unwrap_or_default(),
            suffix_policy: SuffixPolicyVO::new("strict".to_string()),
            allowed_suffix: PatternList::new(vec!["_vo".to_string(), "_entity".to_string()]),
            forbidden_suffix: PatternList::new(Vec::<String>::new()),
            allowed_import: PatternList::new(Vec::<String>::new()),
            forbidden_import: PatternList::new(Vec::<String>::new()),
            mandatory_import: PatternList::new(Vec::<String>::new()),
            mandatory_import_violation_message: ErrorMessage::new(String::new()),
            forbidden_import_violation_message: ErrorMessage::new(String::new()),
            word_count: Count::new(0),
            exceptions: PatternList::new(vec!["mod.rs".to_string()]),
            recursive: BooleanVO::new(true),
            no_primitives: BooleanVO::new(false),
            mandatory_imports: vec![],
            barrel_completeness: BooleanVO::new(false),
            min_lines: Count::new(10),
            max_lines: Count::new(50),
            word_count_violation_message: ErrorMessage::new(String::new()),
            suffix_violation_message: ErrorMessage::new(String::new()),
            no_primitives_violation_message: ErrorMessage::new(String::new()),
            min_lines_violation_message: ErrorMessage::new(String::new()),
            max_lines_violation_message: ErrorMessage::new(String::new()),
            barrel_completeness_violation_message: ErrorMessage::new(String::new()),
            forbid_internal_all: BooleanVO::new(false),
            forbid_internal_all_violation_message: ErrorMessage::new(String::new()),
            forbidden_bypass: PatternList::new(Vec::<String>::new()),
            forbidden_bypass_violation_message: ErrorMessage::new(String::new()),
            forbidden_bypass_custom_messages: vec![],
            mandatory_class_definition: BooleanVO::new(false),
            mandatory_class_definition_violation_message: ErrorMessage::new(String::new()),
            dead_inheritance_bypass: BooleanVO::new(false),
            dead_inheritance_bypass_violation_message: ErrorMessage::new(String::new()),
            dead_inheritance_bypass_custom_messages: vec![],
            check_orphan: BooleanVO::new(false),
            orphan_entry_points: PatternList::new(Vec::<String>::new()),
            orphan_violation_message: ErrorMessage::new(String::new()),
            check_unused_mandatory_imports: BooleanVO::new(false),
            check_unused_mandatory_imports_violation_message: ErrorMessage::new(String::new()),
            forbidden_inheritance: PatternList::new(Vec::<String>::new()),
            forbidden_inheritance_violation_message: ErrorMessage::new(String::new()),
            no_domain_logic: BooleanVO::new(false),
            no_domain_logic_violation_message: ErrorMessage::new(String::new()),
            must_implement_service_container_aggregate: BooleanVO::new(false),
            must_implement_service_container_aggregate_violation_message: ErrorMessage::new(
                String::new(),
            ),
            lazy_eager_initialization_only: BooleanVO::new(false),
            lazy_eager_initialization_only_violation_message: ErrorMessage::new(String::new()),
            stateless_execution: BooleanVO::new(false),
            stateless_execution_violation_message: ErrorMessage::new(String::new()),
            single_execution_goal: BooleanVO::new(false),
            single_execution_goal_violation_message: ErrorMessage::new(String::new()),
            high_level_policy_only: BooleanVO::new(false),
            high_level_policy_only_violation_message: ErrorMessage::new(String::new()),
            coordinates_multiple_orchestrators: BooleanVO::new(false),
            coordinates_multiple_orchestrators_violation_message: ErrorMessage::new(String::new()),
            crud_only: BooleanVO::new(false),
            crud_only_violation_message: ErrorMessage::new(String::new()),
            no_decision_logic: BooleanVO::new(false),
            no_decision_logic_violation_message: ErrorMessage::new(String::new()),
            thread_async_safe: BooleanVO::new(false),
            thread_async_safe_violation_message: ErrorMessage::new(String::new()),
            no_domain_data_storage: BooleanVO::new(false),
            no_domain_data_storage_violation_message: ErrorMessage::new(String::new()),
            owns_system_health_transitions: BooleanVO::new(false),
            owns_system_health_transitions_violation_message: ErrorMessage::new(String::new()),
            lifecycle_tracking_only: BooleanVO::new(false),
            lifecycle_tracking_only_violation_message: ErrorMessage::new(String::new()),
            forbid_any_type: BooleanVO::new(false),
            forbid_any_type_violation_message: ErrorMessage::new(String::new()),
        }
    }

    #[test]
    fn test_count_lines_normal_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_count_lines_normal.txt");
        let content = "line1\nline2\nline3\n";
        let _ = std::fs::write(&path, content);
        let result = ArchMetricChecker::count_lines(path.to_str().unwrap_or(""));
        let _ = std::fs::remove_file(&path);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_count_lines_empty_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("_test_count_lines_empty.txt");
        let _ = std::fs::write(&path, "");
        let result = ArchMetricChecker::count_lines(path.to_str().unwrap_or(""));
        let _ = std::fs::remove_file(&path);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_count_lines_missing_file() {
        let result = ArchMetricChecker::count_lines("/nonexistent/path/file.rs");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_basename_normal() {
        assert_eq!(
            ArchMetricChecker::get_basename("/some/path/file.rs"),
            "file.rs"
        );
    }

    #[test]
    fn test_get_basename_root() {
        assert_eq!(ArchMetricChecker::get_basename("file.rs"), "file.rs");
    }

    #[test]
    fn test_get_basename_empty() {
        assert_eq!(ArchMetricChecker::get_basename(""), "");
    }

    #[test]
    fn test_check_line_counts_barrel_file_skipped() {
        let checker = ArchMetricChecker::new();
        let def = test_definition();
        let mut violations = Vec::new();
        checker.check_line_counts("/path/__init__.py", Some(&def), &mut violations);
        assert_eq!(violations.len(), 0);
        checker.check_line_counts("/path/mod.rs", Some(&def), &mut violations);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_check_line_counts_exception_skipped() {
        let checker = ArchMetricChecker::new();
        let mut def = test_definition();
        def.exceptions = PatternList::new(vec!["main.rs".to_string()]);
        let mut violations = Vec::new();
        checker.check_line_counts("/path/main.rs", Some(&def), &mut violations);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_check_line_counts_unreadable_file_skipped() {
        let checker = ArchMetricChecker::new();
        let def = test_definition();
        let mut violations = Vec::new();
        checker.check_line_counts("/nonexistent/path/file.rs", Some(&def), &mut violations);
        // Should not flag AES005 false positive on unreadable file
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_check_line_counts_no_definition() {
        let checker = ArchMetricChecker::new();
        let mut violations = Vec::new();
        checker.check_line_counts("/path/file.rs", None, &mut violations);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_check_line_counts_min_lines_violation() {
        let checker = ArchMetricChecker::new();
        let def = test_definition();
        let dir = std::env::temp_dir();
        let path = dir.join("_test_aes005_too_short.txt");
        let _ = std::fs::write(&path, "line1\n");
        let mut violations = Vec::new();
        checker.check_line_counts(path.to_str().unwrap_or(""), Some(&def), &mut violations);
        let _ = std::fs::remove_file(&path);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.code().contains("AES005"));
    }

    #[test]
    fn test_check_line_counts_max_lines_violation() {
        let checker = ArchMetricChecker::new();
        let def = test_definition(); // max_lines = 50
        let dir = std::env::temp_dir();
        let path = dir.join("_test_aes004_too_large.txt");
        let content = (0..60).map(|i| format!("line{}\n", i)).collect::<String>();
        let _ = std::fs::write(&path, content);
        let mut violations = Vec::new();
        checker.check_line_counts(path.to_str().unwrap_or(""), Some(&def), &mut violations);
        let _ = std::fs::remove_file(&path);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].code.code().contains("AES004"));
    }

    #[test]
    fn test_check_line_counts_within_limits() {
        let checker = ArchMetricChecker::new();
        let def = test_definition(); // min_lines = 10, max_lines = 50
        let dir = std::env::temp_dir();
        let path = dir.join("_test_aes004_005_ok.txt");
        let content = (0..20).map(|i| format!("line{}\n", i)).collect::<String>();
        let _ = std::fs::write(&path, content);
        let mut violations = Vec::new();
        checker.check_line_counts(path.to_str().unwrap_or(""), Some(&def), &mut violations);
        let _ = std::fs::remove_file(&path);
        assert_eq!(violations.len(), 0);
    }
}

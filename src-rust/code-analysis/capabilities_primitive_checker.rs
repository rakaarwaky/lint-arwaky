use regex::Regex;
use std::fs;

use crate::code_analysis::contract_primitive_protocol::PrimitiveViolation;
use crate::layer_rules::taxonomy_definition_vo::LayerDefinition;
use crate::naming_rules::taxonomy_symbols_vo::PrimitiveTypeList;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct ArchPrimitiveChecker {}

impl ArchPrimitiveChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check primitive usage in struct/class field type annotations (AES006).
    /// Supports Rust, Python, TypeScript/JavaScript.
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
                    violations.push(LintResult::new_arch(file, i + 1, "AES006", Severity::HIGH, &violation_msg));
                    break;
                }
            }
        }
    }

    /// Analyze Python class attributes for primitive type usage.
    /// Uses regex to detect `attr: type` patterns inside class bodies.
    pub fn find_python_primitive_violations(
        &self,
        file_path: &str,
        primitive_types: &[&str],
    ) -> Vec<PrimitiveViolation> {
        let Ok(content) = fs::read_to_string(file_path) else {
            return vec![];
        };

        let mut violations: Vec<PrimitiveViolation> = Vec::new();
        let mut inside_class = false;
        let mut class_indent: usize = 0;

        let attr_pattern = match Regex::new(r"^(\s+)([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_][A-Za-z0-9_\[\], |]*)") {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        let class_pattern = match Regex::new(r"^(\s*)class\s+[A-Za-z_]") {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        for (i, line) in content.lines().enumerate() {
            if let Some(caps) = class_pattern.captures(line) {
                class_indent = caps.get(1).map(|m| m.as_str().len()).unwrap_or(0);
                inside_class = true;
                continue;
            }

            if !inside_class { continue; }

            let stripped = line.trim();
            if !stripped.is_empty() && !stripped.starts_with('#') {
                let current_indent = line.len() - line.trim_start().len();
                if current_indent <= class_indent && !stripped.starts_with("class ") {
                    inside_class = false;
                    continue;
                }
            }

            if let Some(caps) = attr_pattern.captures(line) {
                let type_annotation = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");
                let column = caps.get(2).map(|m| m.start()).unwrap_or(0);
                let base_type = type_annotation.split(['[', '|', ' ']).next().unwrap_or("").trim();

                if primitive_types.contains(&base_type) {
                    violations.push(PrimitiveViolation {
                        line: LineNumber::new((i + 1) as i64),
                        column: ColumnNumber::new(column as i64),
                        type_name: AdapterName::raw(base_type),
                    });
                }
            }
        }

        violations
    }
}

impl crate::code_analysis::contract_primitive_protocol::IPrimitiveCheckerProtocol for ArchPrimitiveChecker {
    fn check_primitive_usage(
        &self,
        file: &FilePath,
        content: &str,
        filename: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        self.check_primitive_usage(&file.value, content, filename, def, violations);
    }

    fn find_python_primitive_violations(
        &self,
        file_path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> Vec<crate::code_analysis::contract_primitive_protocol::PrimitiveViolation> {
        let raw_types: Vec<&str> = primitive_types.values.iter().map(|s| s.value.as_str()).collect();
        let results = self.find_python_primitive_violations(&file_path.value, &raw_types);
        results.into_iter().map(|v| {
            crate::code_analysis::contract_primitive_protocol::PrimitiveViolation {
                line: LineNumber::new(v.line.value()),
                column: ColumnNumber::new(v.column.value()),
                type_name: AdapterName::raw(v.type_name.value()),
            }
        }).collect()
    }
}

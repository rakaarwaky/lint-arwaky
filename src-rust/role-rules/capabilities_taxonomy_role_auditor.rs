// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleChecker for AES0301: taxonomy VO/entity/error/event/constant audits
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use crate::shared_common::taxonomy_source_vo::SourceContentVO;
fn aes0301_primitive_usage(primitive: &str) -> String {
    format!(
        "AES0301 TAXONOMY_ROLE: Direct primitive '{}' in taxonomy.",
        primitive
    )
}

use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;
use std::path::Path;

pub struct TaxonomyRoleChecker {}

impl Default for TaxonomyRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    const RUST_PRIMITIVES: &'static [&'static str] = &[
        "String", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
        "usize", "f32", "f64", "bool", "char", "Vec<", "HashMap<", "Option<", "Result<", "Box<",
        "Cell<", "RefCell<", "Arc<", "Mutex<", "Rc<",
    ];
    const PY_PRIMITIVES: &'static [&'static str] = &[
        "str",
        "int",
        "float",
        "bool",
        "list",
        "dict",
        "tuple",
        "set",
        "bytes",
        "None",
        "Any",
        "Optional",
        "Union",
        "List",
        "Dict",
        "Tuple",
        "Set",
        "FrozenSet",
    ];
    const JS_PRIMITIVES: &'static [&'static str] = &[
        "string",
        "number",
        "boolean",
        "any",
        "object",
        "Array",
        "Record",
        "Map",
        "Set",
        "Promise",
        "unknown",
        "never",
        "void",
        "null",
        "undefined",
        "bigint",
        "symbol",
    ];

    fn scan_primitives(source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let primitives: &[&str] = if file.ends_with(".rs") {
            Self::RUST_PRIMITIVES
        } else if file.ends_with(".py") {
            Self::PY_PRIMITIVES
        } else if file.ends_with(".ts")
            || file.ends_with(".tsx")
            || file.ends_with(".js")
            || file.ends_with(".jsx")
        {
            Self::JS_PRIMITIVES
        } else {
            return;
        };
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.contains(':') {
                continue;
            }
            // Skip value object newtype wrappers: pub(crate) value: Primitive
            if t.contains("pub(crate) value:") || t.trim_start().starts_with("pub value:") {
                continue;
            }
            // Skip trait-mandated conversion boundaries: From<Primitive>::from()
            // and Visitor::visit_*() method parameters. The primitive type is
            // mandated by the trait definition and cannot be replaced with a VO.
            if t.starts_with("fn from(") || t.starts_with("fn visit_") {
                continue;
            }
            if !(t.ends_with(',') || t.ends_with('}') || t.ends_with(')') || t.contains("-> ")) {
                continue;
            }
            let after_colon = match t.split_once(':') {
                Some((_, r)) => r.trim(),
                None => continue,
            };
            let type_candidate = after_colon
                .trim_end_matches(',')
                .trim_end_matches(')')
                .trim_end_matches('}')
                .trim();
            for p in primitives {
                // For generic wrappers like Option<X>, Vec<X>, check if X is a primitive
                if p.ends_with('<') {
                    if type_candidate.starts_with(p) {
                        let inner = type_candidate
                            .strip_prefix(p)
                            .unwrap_or(type_candidate)
                            .trim_end_matches('>');
                        let inner_trimmed = inner.trim();
                        if primitives.iter().any(|prim| {
                            let prim_clean = prim.trim_end_matches('<');
                            inner_trimmed == prim_clean || inner_trimmed.starts_with(prim_clean)
                        }) {
                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES0301",
                                Severity::HIGH,
                                &aes0301_primitive_usage(p),
                            ));
                            break;
                        }
                    }
                    continue; // Skip starts_with for generic wrappers
                }
                // Direct primitive types (String, i64, etc.)
                if type_candidate.starts_with(p) || type_candidate == *p {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES0301",
                        Severity::HIGH,
                        &aes0301_primitive_usage(p),
                    ));
                    break;
                }
            }
        }
    }

    pub fn check_vo(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        if !file.ends_with("_entity.rs") && !file.ends_with("_entity.py") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        if !file.ends_with("_error.rs") && !file.ends_with("_error.py") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        if !file.ends_with("_event.rs") && !file.ends_with("_event.py") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        if !basename.ends_with("_constant.rs") && !basename.ends_with("_constant.py") {
            return;
        }
        let content = source.content.value();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.is_empty() || t.starts_with("//") || t.starts_with('#') || t.starts_with("#[") {
                continue;
            }
            if t.starts_with("pub const ") || t.starts_with("pub static ") {
                continue;
            }
            if t.starts_with("use ")
                || t.starts_with("pub use ")
                || t.starts_with("pub(crate) use ")
            {
                continue;
            }
            if t.starts_with("pub struct ")
                || t.starts_with("struct ")
                || t.starts_with("pub enum ")
                || t.starts_with("enum ")
                || t.starts_with("pub fn ")
                || t.starts_with("fn ")
                || t.starts_with("impl ")
                || t.starts_with("pub mod ")
                || t.starts_with("mod ")
                || t.starts_with("pub trait ")
                || t.starts_with("trait ")
                || t.starts_with("class ")
                || t.starts_with("pub type ")
                || t.starts_with("type ")
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES0301",
                    Severity::HIGH,
                    AesViolation::ConstantPurity { reason: None },
                ));
            }
        }
    }
}

impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_vo(&self) -> Vec<crate::output_report::taxonomy_result_vo::LintResult> {
        self.check_vo()
    }
    fn check_entity(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_entity(source, violations);
    }
    fn check_error(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_error(source, violations);
    }
    fn check_event(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_event(source, violations);
    }
    fn check_constant(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
    ) {
        self.check_constant(source, violations);
    }
}

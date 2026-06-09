use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes031_primitive_usage, AES031_CONSTANT_PURITY,
};
use std::fs;
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

    fn scan_primitives(file: &str, content: &str, violations: &mut Vec<LintResult>) {
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
                if type_candidate.starts_with(p) || type_candidate == p.trim_end_matches('<') {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES031",
                        Severity::HIGH,
                        &aes031_primitive_usage(p),
                    ));
                    break;
                }
            }
        }
    }

    pub fn check_vo(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_entity(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.ends_with("_entity.rs") && !file.ends_with("_entity.py") {
            return;
        }
        Self::scan_primitives(file, content, violations);
    }

    pub fn check_error(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.ends_with("_error.rs") && !file.ends_with("_error.py") {
            return;
        }
        Self::scan_primitives(file, content, violations);
    }

    pub fn check_event(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.ends_with("_event.rs") && !file.ends_with("_event.py") {
            return;
        }
        Self::scan_primitives(file, content, violations);
    }

    pub fn check_constant(&self, file: &str, violations: &mut Vec<LintResult>) {
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        if !basename.ends_with("_constant.rs") && !basename.ends_with("_constant.py") {
            return;
        }
        if let Ok(content) = fs::read_to_string(file) {
            for (i, line) in content.lines().enumerate() {
                let t = line.trim();
                if t.is_empty() || t.starts_with("//") || t.starts_with('#') || t.starts_with("#[")
                {
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
                        "AES031",
                        Severity::HIGH,
                        AES031_CONSTANT_PURITY,
                    ));
                }
            }
        }
    }
}

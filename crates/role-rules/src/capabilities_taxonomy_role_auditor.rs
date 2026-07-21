// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleChecker for AES401: taxonomy primitive usage + constant purity
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::common::taxonomy_language_vo::Language as DetLang;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;
use shared::common::utility_language_detector::detect_language_info_from_source;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct TaxonomyRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_vo(&self) -> Vec<LintResult> {
        self.check_vo_impl()
    }

    fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self.check_entity_impl(source, violations);
    }

    fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self.check_error_impl(source, violations);
    }

    fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self.check_event_impl(source, violations);
    }

    fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self.check_constant_impl(source, violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
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
        let li = detect_language_info_from_source(source);
        let primitives: &[&str] = match li.lang {
            DetLang::Rust => Self::RUST_PRIMITIVES,
            DetLang::Python => Self::PY_PRIMITIVES,
            DetLang::JavaScript | DetLang::TypeScript => Self::JS_PRIMITIVES,
            _ => return,
        };
        let is_rs = li.is_rs;
        let is_py = li.is_py;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.contains(':') {
                continue;
            }
            if t.starts_with("class ") || t.starts_with("pub struct ") || t.starts_with("struct ") {
                continue;
            }
            if t.contains("pub(crate) value:") || t.trim_start().starts_with("pub value:") {
                continue;
            }
            if t.starts_with("fn from(") || t.starts_with("fn visit_") {
                continue;
            }
            if !(t.ends_with(',')
                || t.ends_with(';')
                || t.ends_with('}')
                || t.ends_with(')')
                || t.ends_with(':')
                || t.contains("-> "))
            {
                continue;
            }
            let after_colon = match t.split_once(':') {
                Some((_, r)) => r.trim(),
                None => continue,
            };
            let type_candidate = after_colon
                .trim_end_matches(',')
                .trim_end_matches(';')
                .trim_end_matches(')')
                .trim_end_matches('}')
                .trim();
            for p in primitives {
                if p.ends_with('<') {
                    if type_candidate.starts_with(p) {
                        let inner = match type_candidate.strip_prefix(p) {
                            Some(s) => s,
                            None => type_candidate,
                        }
                        .trim_end_matches('>');
                        let inner_trimmed = inner.trim();
                        if primitives.iter().any(|prim| {
                            let prim_clean = prim.trim_end_matches('<');
                            inner_trimmed == prim_clean || inner_trimmed.starts_with(prim_clean)
                        }) {
                            let primitive_clean = p.trim_end_matches('<');
                            let lang = if is_rs {
                                Language::Rust
                            } else if is_py {
                                Language::Python
                            } else {
                                Language::JavaScript
                            };
                            let msg = AesRoleViolation::PrimitiveUsage {
                                primitive: SymbolName::new(primitive_clean),
                                reason: None,
                            }
                            .with_language(lang)
                            .to_string();

                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES401",
                                Severity::HIGH,
                                msg,
                            ));
                            break;
                        }
                    }
                    continue;
                }
                if type_candidate.starts_with(p) || type_candidate == *p {
                    let primitive_clean = p.trim_end_matches('<');
                    let lang = if is_rs {
                        Language::Rust
                    } else if is_py {
                        Language::Python
                    } else {
                        Language::JavaScript
                    };
                    let msg = AesRoleViolation::PrimitiveUsage {
                        primitive: SymbolName::new(primitive_clean),
                        reason: None,
                    }
                    .with_language(lang)
                    .to_string();

                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES401",
                        Severity::HIGH,
                        msg,
                    ));
                    break;
                }
            }
        }
    }

    fn check_vo_impl(&self) -> Vec<LintResult> {
        vec![]
    }

    fn check_entity_impl(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(source.file_path.value(), "_entity") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    fn check_error_impl(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(source.file_path.value(), "_error") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    fn check_event_impl(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(source.file_path.value(), "_event") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    fn check_constant_impl(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_default();
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
                    "AES401",
                    Severity::HIGH,
                    AesRoleViolation::ConstantPurity { reason: None }.to_string(),
                ));
            }
        }
    }

    fn has_suffix(file: &str, suffix: &str) -> bool {
        let path = Path::new(file);
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            stem.ends_with(suffix)
        } else {
            false
        }
    }
}

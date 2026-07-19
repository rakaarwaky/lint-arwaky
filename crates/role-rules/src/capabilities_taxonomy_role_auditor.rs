// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleChecker for AES401: taxonomy primitive usage + constant purity
//
// ALGORITHM:
//   1. scan_primitives (entity/error/event) — Detects primitive type annotations
//      in taxonomy files. For each line with a `:`, extracts the type after the colon
//      and checks against language-specific primitive lists (RUST_PRIMITIVES,
//      PY_PRIMITIVES, JS_PRIMITIVES). Handles generic wrappers (Option<X>, Vec<X>)
//      by checking the inner type. Skips: pub(crate) value: Primitive (newtype pattern),
//      From<Primitive>/visit_* from() methods (trait-mandated boundaries).
//   2. check_constant — Scans _constant files for non-constant declarations.
//      Allows only: pub const, pub static, use/pub use/pub(crate) use.
//      Flags struct, enum, fn, impl, mod, trait, class, type declarations.
//
// NOTE: scan_primitives uses language-specific primitive sets. Only Rust, Python,
//      and JavaScript/TypeScript are currently supported.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::common::contract_language_detector_port::Language as DetLang;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::role_rules::taxonomy_path_utility::has_suffix;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct TaxonomyRoleChecker {}

#[async_trait::async_trait]
// ─── Block 2: Public Contract ─────────────────────────────
impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_vo(&self) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_vo()
    }
    fn check_entity(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_entity(source, violations);
    }
    fn check_error(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_error(source, violations);
    }
    fn check_event(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_event(source, violations);
    }
    fn check_constant(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_constant(source, violations);
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
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
        let li = crate::taxonomy_language_info_vo::LanguageInfo::new(source);
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
            // Skip class/struct definitions and value object newtype wrappers
            if t.starts_with("class ") || t.starts_with("pub struct ") || t.starts_with("struct ") {
                continue;
            }
            if t.contains("pub(crate) value:") || t.trim_start().starts_with("pub value:") {
                continue;
            }
            // Skip trait-mandated conversion boundaries: From<Primitive>::from()
            // and Visitor::visit_*() method parameters. The primitive type is
            // mandated by the trait definition and cannot be replaced with a VO.
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
                // For generic wrappers like Option<X>, Vec<X>, check if X is a primitive
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
                    continue; // Skip starts_with for generic wrappers
                }
                // Direct primitive types (String, i64, etc.)
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

    pub fn check_vo(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_entity") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_error") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_event") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
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
}

impl Default for TaxonomyRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

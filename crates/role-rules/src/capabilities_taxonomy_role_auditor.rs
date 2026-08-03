// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleChecker for AES401: taxonomy primitive usage + constant purity
//
// ALGORITHM:
//   Uses FileEntry from the filesystem crate. ParseMetadata (if available) provides
//   structured type definitions. For primitive type scanning, falls back to content
//   line scanning since ParseMetadata does not yet expose field-level type info.
//   For constant purity, uses ParseMetadata struct/enum/trait/fn definitions to
//   detect non-constant declarations.

use shared::common::LintResult;
use shared::common::Severity;
use shared::common::{LintMessage, SymbolName};
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::{AesRoleViolation, ITaxonomyRoleChecker};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct TaxonomyRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_entity(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(&file.path, "_entity") {
            return;
        }
        Self::scan_primitives(file, violations);
    }

    fn check_error(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(&file.path, "_error") {
            return;
        }
        Self::scan_primitives(file, violations);
    }

    fn check_event(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if !Self::has_suffix(&file.path, "_event") {
            return;
        }
        Self::scan_primitives(file, violations);
    }

    fn check_constant(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let basename = file
            .path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_default();
        if !basename.ends_with("_constant.rs")
            && !basename.ends_with("_constant.py")
            && !basename.ends_with("_constant.ts")
            && !basename.ends_with("_constant.js")
        {
            return;
        }

        // Use ParseMetadata if available for structured detection
        if let Some(meta) = &file.parse_metadata {
            Self::check_constant_with_metadata(file, meta, violations);
        } else {
            // Fallback: line-based scanning when no parse metadata
            Self::check_constant_fallback(file, violations);
        }
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
        "String",
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "isize",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "usize",
        "f32",
        "f64",
        "bool",
        "char",
        "Vec<",
        "HashMap<",
        "Option<",
        "Result<",
        "Box<",
        "Cell<",
        "RefCell<",
        "Arc<",
        "Mutex<",
        "Rc<",
        "BTreeMap<",
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

    fn fmt_msg(v: &AesRoleViolation) -> String {
        Self::fmt(v)
    }

    fn fmt(v: &AesRoleViolation) -> String {
        match v {
            AesRoleViolation::PrimitiveUsage { primitive, reason } => {
                let why = reason.as_ref().map_or(
                    "Primitive type used in taxonomy layer — domain types must use VOs."
                        .to_string(),
                    |r| r.to_string(),
                );
                format!(
                    "AES401 TAXONOMY_ROLE: Primitive type '{}' used in taxonomy file.\n\
                        WHY? {why}\n\
                        FIX: Wrap primitive in a dedicated Value Object.",
                    primitive.value()
                )
            }
            AesRoleViolation::ConstantPurity { reason } => {
                let why = reason.as_ref().map_or(
                    "Non-constant declaration found in constant file.".to_string(),
                    |r| r.to_string(),
                );
                format!(
                    "AES401 TAXONOMY_ROLE: Constant purity violation.\n\
                        WHY? {why}\n\
                        FIX: Only const/static declarations allowed in taxonomy constant files."
                )
            }
            other => {
                let why = format!("Unhandled taxonomy violation variant: {other:?}");
                format!(
                    "AES401 TAXONOMY_ROLE: Taxonomy role violation.\n\
                        WHY? {why}\n\
                        FIX: Ensure file follows taxonomy conventions."
                )
            }
        }
    }

    fn scan_primitives(file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let primitives: &[&str] = match file.language {
            shared::filesystem::taxonomy_filesystem_vo::Language::Rust => Self::RUST_PRIMITIVES,
            shared::filesystem::taxonomy_filesystem_vo::Language::Python => Self::PY_PRIMITIVES,
            shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
            | shared::filesystem::taxonomy_filesystem_vo::Language::JavaScript => {
                Self::JS_PRIMITIVES
            }
            _ => return,
        };
        let is_rs = matches!(
            file.language,
            shared::filesystem::taxonomy_filesystem_vo::Language::Rust
        );
        let is_py = matches!(
            file.language,
            shared::filesystem::taxonomy_filesystem_vo::Language::Python
        );

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.contains(':') {
                continue;
            }
            if t.starts_with("class ") || t.starts_with("pub struct ") || t.starts_with("struct ") {
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
                        let inner = type_candidate
                            .strip_prefix(p)
                            .unwrap_or(type_candidate)
                            .trim_end_matches('>');
                        let inner_trimmed = inner.trim();
                        if primitives.iter().any(|prim| {
                            let prim_clean = prim.trim_end_matches('<');
                            inner_trimmed == prim_clean || inner_trimmed.starts_with(prim_clean)
                        }) {
                            let primitive_clean = p.trim_end_matches('<');
                            let msg = Self::fmt(&AesRoleViolation::PrimitiveUsage {
                                primitive: SymbolName::new(primitive_clean),
                                reason: Some(LintMessage::new(format!(
                                    "Primitive type '{}' used on line {} of {}",
                                    primitive_clean,
                                    i + 1,
                                    path_str
                                ))),
                            });

                            violations.push(LintResult::new_arch(
                                &path_str,
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
                if type_candidate == *p
                    || (type_candidate.starts_with(p)
                        && !type_candidate[p.len()..]
                            .starts_with(|c: char| c.is_alphanumeric() || c == '_'))
                {
                    let primitive_clean = p.trim_end_matches('<');
                    let msg = Self::fmt(&AesRoleViolation::PrimitiveUsage {
                        primitive: SymbolName::new(primitive_clean),
                        reason: Some(LintMessage::new(format!(
                            "Primitive type '{}' used on line {} of {}",
                            primitive_clean,
                            i + 1,
                            path_str
                        ))),
                    }, shared::common::Language::Rust);

                    violations.push(LintResult::new_arch(
                        &path_str,
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

    /// Check constant purity using structured ParseMetadata.
    fn check_constant_with_metadata(
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        match meta {
            ParseMetadata::Rust(rust_meta) => {
                for name in &rust_meta.struct_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Struct '{}' found in constant file {}",
                                name, path_str
                            ))),
                        }),
                    ));
                }
                for name in &rust_meta.enum_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Enum '{}' found in constant file {}",
                                name, path_str
                            ))),
                        }),
                    ));
                }
                for name in &rust_meta.trait_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Trait '{}' found in constant file {}",
                                name, path_str
                            ))),
                        }),
                    ));
                }
                for fn_item in &rust_meta.function_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Function '{}' found in constant file {}",
                                fn_item.name, path_str
                            ))),
                        }),
                    ));
                }
                if !rust_meta.impl_blocks.is_empty() {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Impl block found in constant file {}",
                                path_str
                            ))),
                        }),
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                for class in &py_meta.class_declarations {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Class '{}' found in constant file {}",
                                class.name, path_str
                            ))),
                        }),
                    ));
                }
                for fn_item in &py_meta.function_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Function '{}' found in constant file {}",
                                fn_item.name, path_str
                            ))),
                        }),
                    ));
                }
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                for class in &ts_meta.class_declarations {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Class '{}' found in constant file {}",
                                class.name, path_str
                            ))),
                        }),
                    ));
                }
                for name in &ts_meta.interface_declarations {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Interface '{}' found in constant file {}",
                                name, path_str
                            ))),
                        }),
                    ));
                }
                for name in &ts_meta.type_alias_declarations {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Type alias '{}' found in constant file {}",
                                name, path_str
                            ))),
                        }),
                    ));
                }
                for fn_item in &ts_meta.function_definitions {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES401",
                        Severity::HIGH,
                        Self::fmt(&AesRoleViolation::ConstantPurity {
                            reason: Some(LintMessage::new(format!(
                                "Function '{}' found in constant file {}",
                                fn_item.name, path_str
                            ))),
                        }),
                    ));
                }
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    /// Fallback constant purity check via line scanning (no parse metadata).
    fn check_constant_fallback(file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
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
                    &path_str,
                    i + 1,
                    "AES401",
                    Severity::HIGH,
                    Self::fmt(&AesRoleViolation::ConstantPurity {
                        reason: Some(LintMessage::new(format!(
                            "Non-constant declaration '{}' found in constant file on line {} of {}",
                            t,
                            i + 1,
                            path_str
                        ))),
                    }, ),
                ));
            }
        }
    }

    fn has_suffix(path: &std::path::Path, suffix: &str) -> bool {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            stem.ends_with(suffix)
        } else {
            false
        }
    }
}

// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleProtocol for AES401: taxonomy primitive usage + constant purity
//
// ALGORITHM:
//   Uses FileEntry from the filesystem crate. ParseMetadata (if available) provides
//   structured type definitions. For primitive type scanning, falls back to content
//   line scanning since ParseMetadata does not yet expose field-level type info.
//   For constant purity, uses ParseMetadata struct/enum/trait/fn definitions to
//   detect non-constant declarations.

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared_role_rules::contract_role_protocol::ITaxonomyRoleProtocol;

// ─── Block 1: Module Declaration ───────────────────────────
pub struct TaxonomyRoleChecker {}

impl TaxonomyRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TaxonomyRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ───────────────
impl ITaxonomyRoleProtocol for TaxonomyRoleChecker {
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

// ─── Block 3: Module Helpers ──────

impl TaxonomyRoleChecker {
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

    fn scan_primitives(file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let primitives: &[&str] = match file.language {
            shared_filesystem::taxonomy_filesystem_vo::Language::Rust => Self::RUST_PRIMITIVES,
            shared_filesystem::taxonomy_filesystem_vo::Language::Python => Self::PY_PRIMITIVES,
            shared_filesystem::taxonomy_filesystem_vo::Language::TypeScript
            | shared_filesystem::taxonomy_filesystem_vo::Language::JavaScript => {
                Self::JS_PRIMITIVES
            }
            _ => return,
        };

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
                            violations.push(LintResult::new_arch_with_name(
                                &path_str,
                                i + 1,
                                "AES401",
                                Severity::HIGH,
                                format!("Primitive type '{primitive_clean}' in a taxonomy entity, error, or event."),
                                "TAXONOMY_PRIMITIVE",
                                format!(
                                    "The taxonomy layer declares values, not built-in types: a field typed '{primitive_clean}' erases the domain value that crosses the layer boundary.",
                                    primitive_clean = primitive_clean,
                                ),
                                "Replace the primitive field with the taxonomy value object that carries it.",
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
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        i + 1,
                        "AES401",
                        Severity::HIGH,
                        format!("Primitive type '{primitive_clean}' in a taxonomy entity, error, or event."),
                        "TAXONOMY_PRIMITIVE",
                        format!(
                            "The taxonomy layer declares values, not built-in types: a field typed '{primitive_clean}' erases the domain value that crosses the layer boundary.",
                            primitive_clean = primitive_clean,
                        ),
                        "Replace the primitive field with the taxonomy value object that carries it.",
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
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Struct '{name}' declared in a const-only file."),
                        "TAXONOMY_CONSTANT_STRUCT",
                        "A const-only file holds values; a struct needs a lifetime and an owner and cannot be a compile-time constant.",
                        format!("Move '{name}' to the taxonomy entity layer."),
                    ));
                }
                for name in &rust_meta.enum_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Enum '{name}' declared in a const-only file."),
                        "TAXONOMY_CONSTANT_ENUM",
                        "A const-only file holds values; an enum is a type that varies and does not belong among constants.",
                        format!("Move '{name}' to the taxonomy entity layer."),
                    ));
                }
                for name in &rust_meta.trait_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Trait '{name}' declared in a const-only file."),
                        "TAXONOMY_CONSTANT_TRAIT",
                        "A const-only file holds values; a trait declares behaviour rather than a value and does not belong among constants.",
                        format!("Move '{name}' to the taxonomy entity layer."),
                    ));
                }
                for name in &rust_meta.type_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Type alias '{name}' declared in a const-only file."),
                        "TAXONOMY_CONSTANT_TYPE_ALIAS",
                        "A const-only file holds values; a type alias names a type rather than declaring a constant and does not belong among constants.",
                        format!("Move '{name}' to the appropriate layer."),
                    ));
                }
                for fn_item in &rust_meta.function_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Function '{name}' declared in a const-only file.", name = fn_item.name),
                        "TAXONOMY_CONSTANT_FUNCTION",
                        "A const-only file holds values; a function is executable behaviour and does not belong among constants.",
                        format!("Move '{name}' out of the const-only file.", name = fn_item.name),
                    ));
                }
                if !rust_meta.impl_blocks.is_empty() {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        "Impl block declared in a const-only file.".to_string(),
                        "TAXONOMY_CONSTANT_IMPL",
                        "A const-only file holds values; an impl block attaches behaviour to a type and does not belong among constants.",
                        "Remove the impl block or move it to the appropriate layer.".to_string(),
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                for class in &py_meta.class_declarations {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Class '{}' declared in a const-only file.", class.name),
                        "TAXONOMY_CONSTANT_CLASS",
                        "A const-only file holds values; a class is a type, not a value, and does not belong among constants.",
                        format!("Move '{}' to the appropriate layer.", class.name),
                    ));
                }
                for fn_item in &py_meta.function_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Function '{}' declared in a const-only file.", fn_item.name),
                        "TAXONOMY_CONSTANT_FUNCTION",
                        "A const-only file holds values; a function is executable behaviour and does not belong among constants.",
                        format!("Move '{}' out of the const-only file.", fn_item.name),
                    ));
                }
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                for class in &ts_meta.class_declarations {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Class '{}' declared in a const-only file.", class.name),
                        "TAXONOMY_CONSTANT_CLASS",
                        "A const-only file holds values; a class is a type, not a value, and does not belong among constants.",
                        format!("Move '{}' to the appropriate layer.", class.name),
                    ));
                }
                for name in &ts_meta.interface_declarations {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Interface '{}' declared in a const-only file.", name),
                        "TAXONOMY_CONSTANT_INTERFACE",
                        "A const-only file holds values; an interface names a type shape rather than declaring a constant and does not belong among constants.",
                        format!("Move '{}' to the appropriate layer.", name),
                    ));
                }
                for name in &ts_meta.type_alias_declarations {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Type alias '{}' declared in a const-only file.", name),
                        "TAXONOMY_CONSTANT_TYPE_ALIAS",
                        "A const-only file holds values; a type alias names a type rather than declaring a constant and does not belong among constants.",
                        format!("Move '{}' to the appropriate layer.", name),
                    ));
                }
                for fn_item in &ts_meta.function_definitions {
                    violations.push(LintResult::new_arch_with_name(
                        &path_str,
                        crate::WHOLE_FILE,
                        "AES401",
                        Severity::HIGH,
                        format!("Function '{}' declared in a const-only file.", fn_item.name),
                        "TAXONOMY_CONSTANT_FUNCTION",
                        "A const-only file holds values; a function is executable behaviour and does not belong among constants.",
                        format!("Move '{}' out of the const-only file.", fn_item.name),
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
                || t.starts_with("export class ")
                || t.starts_with("export interface ")
                || t.starts_with("export enum ")
                || t.starts_with("export type ")
                || t.starts_with("export function ")
            {
                let kind = const_declaration_kind(t);
                let name = declaration_name(t).unwrap_or("<declaration>");
                violations.push(LintResult::new_arch_with_name(
                    &path_str,
                    i + 1,
                    "AES401",
                    Severity::HIGH,
                    format!("{} '{name}' declared in a const-only file.", kind.label()),
                    kind.subtype(),
                    "A const-only file holds values; any other declaration carries a type or behaviour that varies at runtime.",
                    "Move the declaration to the appropriate layer, or replace it with a constant declaration.",
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

/// Classify a non-constant declaration line in a const-only file into its
/// defect kind, so each sub-type of const-file impurity keeps its own subtype.
fn const_declaration_kind(t: &str) -> ConstDeclarationKind {
    if t.starts_with("pub struct ") || t.starts_with("struct ") {
        ConstDeclarationKind::Struct
    } else if t.starts_with("pub enum ") || t.starts_with("enum ") {
        ConstDeclarationKind::Enum
    } else if t.starts_with("pub trait ") || t.starts_with("trait ") {
        ConstDeclarationKind::Trait
    } else if t.starts_with("pub type ") || t.starts_with("type ") || t.starts_with("export type ")
    {
        ConstDeclarationKind::TypeAlias
    } else if t.starts_with("pub fn ") || t.starts_with("fn ") || t.starts_with("export function ")
    {
        ConstDeclarationKind::Function
    } else if t.starts_with("class ") || t.starts_with("export class ") {
        ConstDeclarationKind::Class
    } else if t.starts_with("export interface ") {
        ConstDeclarationKind::Interface
    } else if t.starts_with("impl ") {
        ConstDeclarationKind::Impl
    } else {
        ConstDeclarationKind::Module
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConstDeclarationKind {
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Function,
    Class,
    Interface,
    Impl,
    Module,
}

impl ConstDeclarationKind {
    const fn label(&self) -> &'static str {
        match self {
            Self::Struct => "Struct",
            Self::Enum => "Enum",
            Self::Trait => "Trait",
            Self::TypeAlias => "Type alias",
            Self::Function => "Function",
            Self::Class => "Class",
            Self::Interface => "Interface",
            Self::Impl => "Impl block",
            Self::Module => "Module",
        }
    }

    const fn subtype(&self) -> &'static str {
        match self {
            Self::Struct => "TAXONOMY_CONSTANT_STRUCT",
            Self::Enum => "TAXONOMY_CONSTANT_ENUM",
            Self::Trait => "TAXONOMY_CONSTANT_TRAIT",
            Self::TypeAlias => "TAXONOMY_CONSTANT_TYPE_ALIAS",
            Self::Function => "TAXONOMY_CONSTANT_FUNCTION",
            Self::Class => "TAXONOMY_CONSTANT_CLASS",
            Self::Interface => "TAXONOMY_CONSTANT_INTERFACE",
            Self::Impl => "TAXONOMY_CONSTANT_IMPL",
            Self::Module => "TAXONOMY_CONSTANT_MODULE",
        }
    }
}

/// The declared name out of a declaration line, when one can be read.
fn declaration_name(t: &str) -> Option<&str> {
    let mut body = t;
    if let Some(stripped) = t.strip_prefix("export ") {
        body = stripped;
    }
    if let Some(stripped) = body.trim().strip_prefix("pub(crate) ") {
        body = stripped;
    } else if let Some(stripped) = body.trim().strip_prefix("pub ") {
        body = stripped;
    }
    let body = body.trim();
    let first_word = body.split_whitespace().next()?;
    match first_word {
        "struct" | "enum" | "trait" | "type" | "fn" | "class" | "interface" | "impl" | "mod" => {
            body.split_whitespace().nth(1)
        }
        _ => None,
    }
    .map(str::trim)
}

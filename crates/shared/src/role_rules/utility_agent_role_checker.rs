// PURPOSE: Shared AES405 sub-checks for agent role auditors.
//
// The type budget, implementor, and Any-annotation checks use `ParseMetadata`
// when available and fall back to line scanning, so they live here once for
// all three language auditors. The checks whose token set is
// language-specific (block order, I/O, constant placement, stateless, free
// functions, abstract methods) stay in the per-language auditors.

use std::path::Path;

pub use crate::taxonomy_role_rules_entity::check_agent_protocol_forbidden;
pub use crate::taxonomy_role_rules_entity::check_block_markers;
pub use crate::taxonomy_role_rules_entity::check_constructor_in_block1;
pub use crate::taxonomy_role_rules_entity::is_aggregate_name;
pub use crate::taxonomy_role_rules_entity::is_comment;
use crate::taxonomy_role_rules_entity::{
    annotation_of, declared_names, dynamic_any, field_types, is_injected_field, is_protocol_name,
    owning_crate_name, params_of, protocol_names, push_distinct, read_wrapped_params, rust_any,
    scan_implementor, scan_type_declarations, workspace_root,
};

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language, ParseMetadata};

/// True when `layer` names the agent layer, as a bare name or a parameterised
/// variant such as `agent(feature_x)`.
pub fn is_agent_layer(layer: &str) -> bool {
    layer == "agent" || layer.starts_with("agent(")
}

/// Rule 2 — at most 3 type declarations per agent file. HIGH.
pub fn check_type_budget(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let count = match file.parse_metadata.as_ref() {
        Some(ParseMetadata::Rust(r)) => r.struct_definitions.len() + r.enum_definitions.len(),
        Some(ParseMetadata::Python(p)) => p.class_declarations.len(),
        Some(ParseMetadata::TypeScript(t)) | Some(ParseMetadata::JavaScript(t)) => {
            t.class_declarations.len()
                + t.interface_declarations.len()
                + t.type_alias_declarations.len()
        }
        // A `Some` metadata of an unhandled variant, or a file with no
        // metadata at all, falls back to the line scan.
        Some(_) | None => scan_type_declarations(&file.content),
    };

    if count > 3 {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES405",
            Severity::HIGH,
            format!(
                "AES405 AGENT_ROLE: Agent declares too many types.\n\
                 WHY? Found {count} type declarations, max 3 allowed.\n\
                 FIX: Keep at most 3 types in an agent file. \
                 Move excess types to the taxonomy layer, or give the agent a \
                 companion file that owns the helper types."
            ),
        ));
    }
}

/// Rule 1 — at least 1 type implements an aggregate trait or base. MEDIUM.
///
/// The implementor is a type declared in this same file, so a helper struct
/// with no aggregate impl does not satisfy the rule; the file needs one.
pub fn check_implementor(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let (found, why) = match file.parse_metadata.as_ref() {
        Some(ParseMetadata::Rust(r)) => {
            let structs: Vec<&str> = r.struct_definitions.iter().map(String::as_str).collect();
            let has = r.impl_blocks.iter().any(|imp| {
                is_aggregate_name(imp.trait_name.as_deref())
                    && structs.contains(&imp.implementor_type.as_str())
            });
            (
                has,
                "No `impl I<Feature>Aggregate for <Type>` where <Type> is declared in this file.",
            )
        }
        Some(ParseMetadata::Python(p)) => {
            let has = p
                .class_declarations
                .iter()
                .any(|c| c.bases.iter().any(|b| is_aggregate_name(Some(b))));
            (
                has,
                "No class in this file inherits from an `I<Feature>Aggregate` base.",
            )
        }
        Some(ParseMetadata::TypeScript(t)) | Some(ParseMetadata::JavaScript(t)) => {
            let has = t
                .class_declarations
                .iter()
                .any(|c| c.implements.iter().any(|i| is_aggregate_name(Some(i))));
            (
                has,
                "No class in this file declares `implements I<Feature>Aggregate`.",
            )
        }
        Some(_) | None => (scan_implementor(&file.content), ""),
    };

    if !found {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES405",
            Severity::MEDIUM,
            format!(
                "AES405 AGENT_ROLE: No type implements an _aggregate trait.\n\
                 WHY? {why} An agent is the composition root for its feature, so it \
                 is the type that implements the feature's aggregate.\n\
                 FIX: Have one type declared in this file implement the \
                 feature aggregate, or add the feature aggregate with the `aes-contract` skill."
            ),
        ));
    }
}

/// No `Any` / `any` type annotations in the agent file. MEDIUM.
///
/// The check is language-specific in what it looks for: Rust spells the escape
/// hatch `Any` and uses `<>` / `[]` bounds, while Python and TS spell it `any`
/// or `Any` as a whole-word annotation.
pub fn check_any_annotation(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let is_rust = file.language == Language::Rust;

    for (i, line) in file.content.lines().enumerate() {
        let t = line.trim();
        if is_comment(t) {
            continue;
        }
        let found = if is_rust { rust_any(t) } else { dynamic_any(t) };
        if found {
            violations.push(LintResult::new_arch(
                &path,
                i + 1,
                "AES405",
                Severity::MEDIUM,
                format!(
                    "AES405 AGENT_ROLE: Any-type annotation detected.\n\
                     WHY? Line {} annotates a value with Any/any, which \
                     erases the domain type the orchestrator is supposed to coordinate.\n\
                     FIX: Use the concrete VO or protocol type the value \
                     actually is, and move the type-erasing boundary to the surface layer.",
                    i + 1
                ),
            ));
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
// P14 — subsystem count
// ───────────────────────────────────────────────────────────────────────────────

/// Count the protocol seams injected into an agent file, returning
/// `(injected, distinct)`: how many protocol fields the agent declares and
/// how many distinct protocol type names appear among them. `Rust` counts
/// struct fields typed `Arc<dyn I*Protocol>` / `Box<dyn I*Protocol>`, so a
/// `new` parameter that takes the same protocol is not double-counted.
/// `Python` counts `__init__` parameters annotated `I*Protocol`.
/// `TypeScript` counts interface members and constructor fields. The result
/// is a tuple rather than a named struct because this module is a utility
/// file, and AES404 forbids a type definition there.
pub fn count_protocol_fields(content: &str, language: Language) -> (usize, usize) {
    let mut injected = 0usize;
    let mut distinct: Vec<String> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, raw) in lines.iter().enumerate() {
        let t = raw.trim();
        if is_comment(t) {
            continue;
        }
        match language {
            Language::Rust => {
                // Count fields on ANY struct in the file (direct or via a
                // separate `Deps` struct) that are injected protocol types.
                if is_injected_field(t) {
                    for name in protocol_names(t) {
                        injected += 1;
                        push_distinct(&mut distinct, name);
                    }
                }
            }
            Language::Python => {
                // Protocol-typed params in `__init__` — counts the
                // `CalculatorOrchestratorDeps` class `__init__` as well as
                // the orchestrator's own, so the total reflects every
                // protocol seam the agent coordinates. A wrapped signature is
                // joined before the params are split, and each param
                // contributes its annotation, not the whole `name: Type` text.
                if t.starts_with("def __init__") {
                    let sig = read_wrapped_params(&lines, i);
                    for param in params_of(&sig) {
                        let ty = annotation_of(&param);
                        if is_protocol_name(ty) {
                            injected += 1;
                            push_distinct(&mut distinct, ty.to_string());
                        }
                    }
                }
            }
            Language::TypeScript | Language::JavaScript => {
                // A field declaration (`private readonly dep: IFooProtocol`)
                // is the injection site, whether it sits on the orchestrator
                // class or on a separate `Deps` class the orchestrator
                // holds. A constructor shorthand also declares fields
                // inline, so those count too. The TS convention keeps the
                // field table in an `interface`, so its members count.
                let is_field = t.starts_with("private readonly")
                    || t.starts_with("public readonly")
                    || t.starts_with("protected readonly")
                    || t.starts_with("readonly");
                let is_shorthand_param = t.contains("constructor(")
                    && (t.contains("private readonly") || t.contains("public readonly"));
                let is_interface_member = t.ends_with(';') && t.contains(':') && !t.contains('=');
                if is_field || is_shorthand_param || is_interface_member {
                    // The type follows the last `:` on the line; a wrapped
                    // declaration appends its continuation lines first.
                    let joined = if t.contains(':') {
                        t.to_string()
                    } else {
                        read_wrapped_params(&lines, i)
                    };
                    for param in field_types(&joined) {
                        if is_protocol_name(&param) {
                            injected += 1;
                            push_distinct(&mut distinct, param.trim().to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    (injected, distinct.len())
}

/// Count the protocol traits a feature's shared module declares, keeping only
/// those whose name appears in `referenced`.
///
/// `referenced` holds the protocol names that appear in a non-comment position
/// of some non-shared feature source file (the agent, the capabilities, the
/// root container). Dead protocols not referenced by anything in the feature
/// are excluded. Pass an empty slice to count every declaration.
fn count_feature_protocol_traits_with(feature_dir: &Path, referenced: &[String]) -> usize {
    count_protocol_traits_in_dir_with_stem_filter(
        feature_dir,
        referenced,
        Some(&|base: &str| base.contains("protocol") || base.contains("Protocol")),
    )
}

/// Count the protocol traits a feature's flat `modules/shared/src/` layout
/// declares. Contracts are named `contract_<feature>_protocol.py`, but a
/// feature's protocols are often split across sibling files such as
/// `contract_<feature>_update_protocol.py`, so every `contract_<feature>_*`
/// file is scanned, not just the exact-stem one. The trailing `starts_with`
/// keeps a `skill_update` feature from picking up `contract_skill_protocol`.
fn count_feature_protocol_traits_in_modules_dir(
    modules_shared: &Path,
    feature: &str,
    referenced: &[String],
) -> usize {
    let stem = format!("contract_{}", feature);
    count_protocol_traits_in_dir_with_stem_filter(
        modules_shared,
        referenced,
        Some(&|base: &str| base == stem || base.starts_with(&format!("{stem}_"))),
    )
}

/// Shared scan core: count the protocol traits declared in `dir`, keeping only
/// names in `referenced` (an empty list counts every declaration). When
/// `stem_filter` is `Some`, a source file is scanned only if its file stem
/// passes the filter.
fn count_protocol_traits_in_dir_with_stem_filter(
    dir: &Path,
    referenced: &[String],
    stem_filter: Option<&dyn Fn(&str) -> bool>,
) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0usize;
    for entry in entries.flatten() {
        let p = entry.path();
        let Some(ext) = p.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if !p.is_file() || !matches!(ext, "rs" | "py" | "ts") {
            continue;
        }
        let Some(base) = p.file_stem().and_then(|n| n.to_str()) else {
            continue;
        };
        if let Some(filter) = stem_filter
            && !filter(base)
        {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&p) else {
            continue;
        };
        for line in content.lines() {
            let t = line.trim();
            if is_comment(t) {
                continue;
            }
            let declares = t.starts_with("pub trait ")
                || t.starts_with("trait ")
                || t.starts_with("class ")
                || t.starts_with("export class ")
                || t.starts_with("abstract class ");
            if declares {
                for name in declared_names(t) {
                    if is_protocol_name(&name)
                        && (referenced.is_empty() || referenced.contains(&name))
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

/// Collect the protocol names referenced by a feature's non-shared source
/// files (the agent, the capabilities, the root container). A protocol that
/// appears nowhere in the feature is dead.
fn referenced_protocols_in(feature_src_dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    let Ok(entries) = std::fs::read_dir(feature_src_dir) else {
        return names;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let Some(ext) = p.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if !matches!(ext, "rs" | "py" | "ts") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&p) else {
            continue;
        };
        for line in content.lines() {
            let t = line.trim();
            if is_comment(t) {
                continue;
            }
            for word in t.split(|c: char| !c.is_alphanumeric() && c != '_') {
                if is_protocol_name(word) && !names.contains(&word.to_string()) {
                    names.push(word.to_string());
                }
            }
        }
    }
    names
}

/// Count the protocol traits declared in any directory (used for the local
/// src/ layout when the shared module is absent), keeping only those whose
/// name appears in `referenced`.
fn count_protocol_traits_in_dir(dir: &Path, referenced: &[String]) -> usize {
    count_protocol_traits_in_dir_with_stem_filter(dir, referenced, None)
}

/// Resolve how many protocols the agent file's own feature declares.
///
/// A feature's protocols live in its shared module
/// (`crates/shared/src/<feature>/`) in the production layout, and beside the
/// agent file itself in the test-workspace layout. Both are counted, because
/// "this feature declares exactly one protocol" means the same thing
/// wherever the declaration sits. Protocols that appear nowhere in any
/// non-shared source file of the feature are dead and excluded. Returns
/// `usize::MAX` when the layout does not match — a file that does not sit
/// under `crates/<feature>/src/`, or an ancestor with no `crates/shared` at
/// all. The sentinel is not equal to 1, so the single-subsystem skip does
/// not fire for an unknown layout.
pub fn resolve_feature_protocol_count(file_path: &Path) -> usize {
    let Some(root) = workspace_root(file_path) else {
        return usize::MAX;
    };
    let Some(crate_name) = owning_crate_name(file_path) else {
        return usize::MAX;
    };
    // The shared module is named after the feature, which uses `_` where the
    // crate directory uses `-`.
    let feature = crate_name.replace('-', "_");

    // The feature's own source files sit beside the agent file; they are what
    // makes a declared protocol live. A protocol nothing in this directory
    // mentions is dead, and a dead protocol is not a subsystem the agent
    // coordinates.
    let feature_src = file_path.parent().unwrap_or(Path::new(""));
    let referenced = referenced_protocols_in(feature_src);

    // Production layout: the feature's shared module.
    let shared_dir = root
        .join("crates")
        .join("shared")
        .join("src")
        .join(&feature);
    let mut count = count_feature_protocol_traits_with(&shared_dir, &referenced);

    // Python modules layout: contracts sit flat in `modules/shared/src/`,
    // named `contract_<feature>_*.{rs,py,ts}` (often split across sibling
    // files), so no per-feature subfolder. Count the feature's own contract
    // files when the nested module is absent.
    if count == 0 {
        let modules_shared = root.join("modules").join("shared").join("src");
        if modules_shared.is_dir() {
            count = count_feature_protocol_traits_in_modules_dir(
                &modules_shared,
                &feature,
                &referenced,
            );
        }
    }

    // Test-workspace layout: contracts sit beside the agent file. When the
    // shared module declares nothing, the local declarations stand in.
    if count == 0 {
        count = count_protocol_traits_in_dir(feature_src, &referenced);
    } else {
        count = count.max(count_protocol_traits_in_dir(feature_src, &referenced));
    }

    count
}

// Shared helpers (moved to taxonomy_role_rules_entity.rs; re-exported via
// `pub use` above).

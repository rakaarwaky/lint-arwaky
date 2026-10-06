// PURPOSE: Shared AES405 sub-checks for agent role auditors.
//
// The type budget, implementor, and Any-annotation checks use `ParseMetadata`
// when available and fall back to line scanning, so they live here once for
// all three language auditors. The checks whose token set is
// language-specific (block order, I/O, constant placement, stateless, free
// functions, abstract methods) stay in the per-language auditors.

use std::path::Path;

use shared_common::taxonomy_lint_result_vo::LintResult;
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

/// Count the protocol seams injected into an agent file.
///
/// Returns `(injected, distinct)`: how many protocol fields the agent
/// declares, and how many distinct protocol type names appear among them.
///
/// `Rust` counts struct fields typed `Arc<dyn I*Protocol>` / `Box<dyn I*Protocol>`,
/// so a `new` parameter that takes the same protocol is not double-counted.
/// `Python` counts `__init__` parameters annotated `I*Protocol`, and
/// `TypeScript` counts interface members and constructor fields typed
/// `I*Protocol`.
///
/// The result is a tuple rather than a named struct because this module is a
/// utility file, and AES404 forbids a type definition there.
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
                // the orchestrator's own `__init__`, so the total reflects
                // every protocol seam the agent coordinates. A signature
                // that wraps is joined before the params are split, and each
                // param contributes its annotation rather than the whole
                // `name: Type` text.
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
                // class or on a separate `Deps` class the orchestrator holds.
                // A constructor shorthand
                // (`constructor(private readonly dep: IFooProtocol, …)`)
                // also declares fields inline, so those are counted too.
                // The TS convention keeps the field table in an
                // `interface`, so its members count as well.
                let is_field = t.starts_with("private readonly")
                    || t.starts_with("public readonly")
                    || t.starts_with("protected readonly")
                    || t.starts_with("readonly");
                let is_shorthand_param = t.contains("constructor(")
                    && (t.contains("private readonly") || t.contains("public readonly"));
                let is_interface_member = t.ends_with(';') && t.contains(':') && !t.contains('=');
                if is_field || is_shorthand_param || is_interface_member {
                    // For a field declaration the type comes after the last
                    // `:` on the line; a wrapped declaration appends its
                    // continuation lines before that `:` lands.
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

/// Count the protocol traits a feature's shared module declares.
///
/// Used by P14 to recognise a genuinely single-subsystem feature. A feature
/// that declares exactly one protocol has nothing else for its agent to
/// coordinate, so flagging its single injected seam would push the author to
/// invent a protocol that does no distinct job.
///
/// A protocol trait that no file in the feature references is dead code and
/// does not count: a declared-but-unused trait is not a subsystem the agent
/// could coordinate. Only `count_feature_protocol_traits_with` applies that
/// filter, using the protocol names the feature's own source files reference.
///
/// Returns 0 when the directory is absent or empty.
pub fn count_feature_protocol_traits(feature_dir: &Path) -> usize {
    count_feature_protocol_traits_with(feature_dir, &[])
}

/// Count the protocol traits a feature's shared module declares, keeping only
/// those whose name appears in `referenced`.
///
/// `referenced` holds the protocol names that appear in a non-comment position
/// of some non-shared feature source file (the agent, the capabilities, the
/// root container). Dead protocols not referenced by anything in the feature
/// are excluded. Pass an empty slice to count every declaration.
pub fn count_feature_protocol_traits_with(feature_dir: &Path, referenced: &[String]) -> usize {
    count_protocol_traits_in_dir_with_stem_filter(
        feature_dir,
        referenced,
        Some(&|base: &str| base.contains("protocol") || base.contains("Protocol")),
    )
}

/// Count the protocol traits in a flat `modules/shared/src/` layout, where
/// contracts are named `contract_<feature>_protocol.py` rather than living in
/// a per-feature subfolder. Only the feature's own contract file is scanned,
/// so protocols of other features are not counted as subsystems.
fn count_feature_protocol_traits_in_modules_dir(
    modules_shared: &Path,
    feature: &str,
    referenced: &[String],
) -> usize {
    let file_stem = format!("contract_{}_protocol", feature);
    for ext in ["rs", "py", "ts"] {
        let path = modules_shared.join(format!("{}.{}", file_stem, ext));
        if path.is_file() {
            return count_protocol_traits_in_dir_with_stem_filter(
                modules_shared,
                referenced,
                Some(&|base: &str| base == file_stem.as_str()),
            );
        }
    }
    0
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
        if !p.is_file() {
            continue;
        }
        let Some(ext) = p.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if !matches!(ext, "rs" | "py" | "ts") {
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
/// "this feature declares exactly one protocol" means the same thing wherever
/// the declaration sits.
///
/// Protocols that appear nowhere in any non-shared source file of the feature
/// are treated as dead code and excluded — an unreferenced protocol trait is
/// not a subsystem the agent can coordinate. This keeps a single-subsystem
/// feature flagged only when the agent coordinates every subsystem the
/// feature actually has.
///
/// Returns `usize::MAX` when the layout does not match — a file that does not
/// sit under `crates/<feature>/src/`, or an ancestor with no `crates/shared`
/// at all. The sentinel is not equal to 1, so the single-subsystem skip does
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
    // named `contract_<feature>_protocol.py`, so no per-feature subfolder.
    // Count the feature's own contract file when the nested module is absent.
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

// ───────────────────────────────────────────────────────────────────────────────
// Shared helpers
// ───────────────────────────────────────────────────────────────────────────────

/// True when a line is a comment in any of the three languages.
pub fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("///")
}

/// True when a trait, class, or interface name is an aggregate seam.
pub fn is_aggregate_name(name: Option<&str>) -> bool {
    name.is_some_and(|n| {
        let base = n.rsplit("::").next().unwrap_or(n).trim();
        base.to_lowercase().contains("aggregate")
    })
}

/// True when a type name looks like a contract protocol seam.
///
/// The name may arrive in any of the shapes a caller writes it: a bare
/// `IScannerProtocol`, a path-qualified `shared::IScannerProtocol`, or a
/// parameterized `IScannerProtocol<T>` / `IScannerProtocol[T]` / `Box<dyn
/// IScannerProtocol>`. Generic brackets and a path qualifier are stripped so
/// every spelling reduces to the bare trait name before it is matched —
/// otherwise a parameterized seam slips past and the agent goes unflagged.
fn is_protocol_name(name: &str) -> bool {
    // Cut at the first generic or trait-object bracket, so `IFooProtocol<T>`
    // and `Box<dyn IFooProtocol>` both reduce to `IFooProtocol`.
    let base = name.split(['<', '[', '{']).next().unwrap_or(name);
    // Then take the last path segment: `crate::IFooProtocol` -> `IFooProtocol`.
    let base = base.rsplit("::").next().unwrap_or(base);
    let base = base.rsplit(['.', ':']).next().unwrap_or(base);
    let base = base.trim().trim_end_matches(['>', ']', ' ', ',']);
    base.starts_with('I') && base.ends_with("Protocol")
}

/// True when a Rust line declares an injected dependency field.
///
/// A field is `pub addition: Box<dyn ICalculatorProtocol>,` — it may be
/// terminated by a comma, a semicolon, or nothing when it is the last field
/// in the struct body. A `pub fn new(… Arc<dyn …>)` parameter and an
/// `impl` block header are not fields.
fn is_injected_field(t: &str) -> bool {
    if t.starts_with("pub fn") || t.starts_with("fn ") || t.starts_with("impl ") {
        return false;
    }
    let has_dyn = t.contains("Arc<dyn ") || t.contains("Box<dyn ");
    has_dyn && t.contains("Protocol") && !t.contains('(') && !t.contains("->")
}

/// Extract protocol trait names from a Rust injected-field line.
///
/// `pub addition: Box<dyn ICalculatorProtocol>,` yields `ICalculatorProtocol`.
/// A struct with several fields on one line yields each name in turn.
fn protocol_names(t: &str) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in t.split("Arc<dyn ").chain(t.split("Box<dyn ")) {
        let name = chunk
            .split([',', '>', ';', ' '])
            .next()
            .unwrap_or("")
            .trim();
        if is_protocol_name(name) {
            out.push(name.to_string());
        }
    }
    out
}

/// Extract protocol type names from a TypeScript field declaration line.
///
/// Handles a bare field (`private readonly dep: IFooProtocol;`), two fields
/// on one line, and the constructor shorthand
/// (`constructor(private readonly dep: IFooProtocol, other: IReportProtocol)`).
fn field_types(t: &str) -> Vec<String> {
    let mut out = Vec::new();
    // The type follows the `:` that terminates the field name, so split on
    // statement boundaries and take the trailing type token of each part.
    for part in t.split([';', ')', ',']) {
        let part = part.trim();
        let Some((_name, ty)) = part.rsplit_once(':') else {
            continue;
        };
        // The type may be a union or generic; take the first protocol-looking
        // token rather than the whole type expression.
        for token in ty.split(['|', '&', '<', '>', ' ']) {
            if is_protocol_name(token.trim()) {
                out.push(token.trim().to_string());
                break;
            }
        }
    }
    out
}

/// Split a parameter list into its parameters, dropping the `self` receiver.
///
/// A signature that wraps past the end of its first line is read from
/// `signature`, which the caller builds by joining the opening line with its
/// continuation lines up to the closing paren.
fn params_of(signature: &str) -> Vec<String> {
    let Some(open) = signature.find('(') else {
        return Vec::new();
    };
    let rest = &signature[open + 1..];
    let body = match rest.find(')') {
        Some(close) => &rest[..close],
        None => rest,
    };
    body.split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty() && p != "self" && p != "*" && p != "**kwargs")
        .collect()
}

/// Read a possibly-wrapped parameter list starting at `start` in `lines`.
///
/// Returns the whole signature including its opening paren, so the caller can
/// pass it straight to `params_of` or `field_types`. A signature whose parens
/// close on the opening line is used as-is; otherwise the following lines are
/// appended until the paren balances.
fn read_wrapped_params(lines: &[&str], start: usize) -> String {
    let first = lines[start];
    let Some(open) = first.find('(') else {
        return first.to_string();
    };
    if first[open + 1..].contains(')') {
        return first.to_string();
    }
    let mut acc = String::from(first);
    for line in lines.iter().skip(start + 1) {
        acc.push(' ');
        acc.push_str(line.trim());
        if line.contains(')') {
            break;
        }
    }
    acc
}

/// Names declared on a trait / class declaration line.
///
/// The declaration keyword is stripped before the name is taken, because
/// `pub trait IFooProtocol:` and `export class IFooProtocol {` both put the name
/// after the keyword and behind a different terminator.
fn declared_names(t: &str) -> Vec<String> {
    let rest = t
        .trim_start_matches("pub ")
        .trim_start_matches("export ")
        .trim_start_matches("abstract ")
        .trim_start_matches("trait ")
        .trim_start_matches("class ");
    let Some(name) = rest.split([' ', '(', ':', '{', '<']).next() else {
        return Vec::new();
    };
    if name.is_empty() {
        Vec::new()
    } else {
        vec![name.to_string()]
    }
}

/// The type annotation of a Python parameter, or the parameter itself when it
/// carries no annotation.
///
/// `addition: ICalculatorProtocol = None` yields `ICalculatorProtocol`; a
/// bare `deps` yields `deps`, which then fails the protocol-name test.
fn annotation_of(param: &str) -> &str {
    let body = param.split('=').next().unwrap_or(param);
    match body.rsplit_once(':') {
        Some((_name, ty)) => ty.trim(),
        None => body.trim(),
    }
}

/// Record a protocol name once, so `distinct` counts types, not occurrences.
fn push_distinct(distinct: &mut Vec<String>, name: String) {
    let normalized = name.trim().to_string();
    if !distinct.contains(&normalized) {
        distinct.push(normalized);
    }
}

/// Count type declarations by line scan, for files with no parse metadata.
fn scan_type_declarations(content: &str) -> usize {
    let mut count = 0usize;
    for line in content.lines() {
        let t = line.trim();
        if is_comment(t) {
            continue;
        }
        if t.starts_with("struct ")
            || t.starts_with("pub struct ")
            || t.starts_with("enum ")
            || t.starts_with("pub enum ")
            || t.starts_with("class ")
            || t.starts_with("interface ")
            || t.starts_with("export interface ")
        {
            count += 1;
        }
    }
    count
}

/// True when a line scan finds an aggregate implementation pattern.
fn scan_implementor(content: &str) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if is_comment(t) {
            continue;
        }
        if !t.to_lowercase().contains("aggregate") {
            continue;
        }
        if t.starts_with("impl ") && t.contains(" for ") {
            return true;
        }
        if t.starts_with("class ") && t.contains('(') {
            return true;
        }
        if t.contains("implements ") {
            return true;
        }
    }
    false
}

/// True when a Rust line uses `Any` as a type.
fn rust_any(t: &str) -> bool {
    t.contains(": Any")
        || t.contains("dyn Any")
        || t.contains("Any<")
        || t.contains("Any[")
        || t.contains("-> Any")
}

/// True when a Python / TS line uses `any` or `Any` as a whole-word type.
fn dynamic_any(t: &str) -> bool {
    for word in t.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if word == "any" || word == "Any" {
            return true;
        }
    }
    false
}

/// Contract protocols implemented by a type declared in this file.
///
/// A contract protocol is an `I<Name>Protocol` seam: the agent implements the
/// feature aggregate and injects protocol seams, it never *implements* one,
/// because that is a capability's job. Std traits (`Default`, `Display`,
/// `Clone`) and aggregate traits are not contract protocols and are excluded
/// here so `impl Default for X` reads as nothing.
///
/// Returns the trait names in declaration order, deduplicated.
pub fn contract_protocol_impls(file: &FileEntry) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |name: &str| {
        if !out.iter().any(|n| n == name) {
            out.push(name.to_string());
        }
    };

    match file.parse_metadata.as_ref() {
        Some(ParseMetadata::Rust(r)) => {
            for imp in &r.impl_blocks {
                if let Some(trait_name) = &imp.trait_name
                    && is_protocol_name(trait_name)
                {
                    push(trait_name);
                }
            }
        }
        Some(ParseMetadata::Python(p)) => {
            for c in &p.class_declarations {
                for base in &c.bases {
                    if is_protocol_name(base) {
                        push(base);
                    }
                }
            }
        }
        Some(ParseMetadata::TypeScript(t)) | Some(ParseMetadata::JavaScript(t)) => {
            for c in &t.class_declarations {
                for iface in &c.implements {
                    if is_protocol_name(iface) {
                        push(iface);
                    }
                }
            }
        }
        Some(_) | None => scan_contract_protocol_impls(&file.content, &mut push),
    }
    out
}

/// Line-scan fallback for `contract_protocol_impls` when parse metadata is
/// absent. `scan` is called once per distinct contract protocol found.
fn scan_contract_protocol_impls(content: &str, mut scan: impl FnMut(&str)) {
    for line in content.lines() {
        let t = line.trim();
        if is_comment(t) {
            continue;
        }
        if let Some(rest) = rust_impl_trait_name(t) {
            if is_protocol_name(rest) {
                scan(rest);
            }
        } else if let Some(base) = class_base_list(t) {
            // Python: `class Bar(IFooProtocol)` — and the TS form
            // `class Bar implements IFooProtocol`.
            for candidate in base.split(',').chain(base.split(" implements ")) {
                let name = candidate.trim().trim_start_matches("public ").trim();
                if is_protocol_name(name) {
                    scan(name);
                }
            }
        }
    }
}

/// The trait name of a Rust `impl … Trait for Type` line, if the line is one.
///
/// `impl` may carry its own generic list before the trait and that list is not
/// always followed by a space — `impl<T> IFoo for X` — so the keyword is matched
/// on its own and whatever follows it, parameters included, is the head. The
/// trait is then the last whitespace-separated token before ` for `, which skips
/// the generic list without having to parse it. A line with no ` for ` is an
/// inherent impl and has no trait.
fn rust_impl_trait_name(t: &str) -> Option<&str> {
    let rest = t.strip_prefix("impl")?;
    let rest = rest.trim_start();
    let (lhs, _rhs) = rest.split_once(" for ")?;
    Some(lhs.rsplit([' ', ',']).next().unwrap_or(lhs).trim())
}

/// The base-class / implements list of a `class` declaration line, when the
/// line carries one. Returns the text inside `(...)` for Python and the part
/// after `implements` for TypeScript.
///
/// TypeScript writes `export class X …` in every module, so the keyword is
/// located rather than required at the start of the line, but only after one of
/// the declaration prefixes has been confirmed. Accepting `class ` anywhere in
/// the line would read a string literal or a trailing comment that mentions a
/// class as if it declared one, and the fallback would then report a protocol
/// that was never implemented.
fn class_base_list(t: &str) -> Option<&str> {
    let idx = t.find("class ")?;
    // Everything before the keyword must be a modifier or nothing at all.
    match t[..idx].trim() {
        ""
        | "export"
        | "export default"
        | "export abstract"
        | "export default abstract"
        | "abstract"
        | "declare" => {}
        _ => return None,
    }
    let rest = t[idx + "class ".len()..].trim_start();
    if let Some(i) = rest.find("implements ") {
        return Some(rest[i + "implements ".len()..].trim_end_matches('{').trim());
    }
    let open = rest.find('(')?;
    let close = rest.rfind(')')?;
    Some(&rest[open + 1..close])
}

/// Rule — an agent file implements no contract protocol. HIGH.
///
/// This is a flat prohibition, not a budget: there is no count at which a second
/// implementation becomes acceptable, and implementing one *alongside* the
/// aggregate is already a violation. An agent composes its feature by injecting
/// protocol seams, so implementing a protocol here makes the orchestration layer
/// duplicate a capability's work — the behaviour belongs in a `capabilities_*`
/// file. See `contract_protocol_impls` for what counts as a contract protocol
/// and what is deliberately excluded (std traits and aggregate traits).
pub fn check_agent_protocol_forbidden(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let protocols = contract_protocol_impls(file);
    if protocols.is_empty() {
        return;
    }
    let path = file.path.to_string_lossy().to_string();
    let names = protocols.join(", ");
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES405",
        Severity::HIGH,
        format!(
            "AES405 AGENT_ROLE: Agent file implements a contract protocol.\n\
             WHY? File implements {names}. An agent is the feature's composition \
             root: it implements the feature aggregate and injects protocol seams. \
             Implementing a protocol here makes the orchestration layer duplicate a \
             capability's work.\n\
             FIX: Move the protocol implementation into a \
             `capabilities_*` file in this feature and inject that capability's \
             protocol into the agent. Keep the aggregate impl as the only contract \
             this file fulfils."
        ),
    ));
}

/// Rule — the `─── Block N:` banner markers must stop at 3. MEDIUM.
///
/// The 3-block structure is the readability contract the HOW-TO documents: a
/// fourth marker means the file has outgrown the shape, and the reader loses
/// the block map. A file carrying no markers is not reported — the marker is
/// a convention the reader can spot by shape, not a requirement the linter
/// enforces from nothing.
pub fn check_block_markers(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let markers = block_marker_numbers(&file.content);
    let Some(max) = markers.iter().max().copied() else {
        return;
    };
    if max <= 3 {
        return;
    }
    let path = file.path.to_string_lossy().to_string();
    let found: Vec<String> = markers.iter().map(|n| format!("Block {n}")).collect();
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES405",
        Severity::MEDIUM,
        format!(
            "AES405 AGENT_ROLE: Agent file carries block markers beyond Block 3.\n\
             WHY? File declares {}. The 3-block structure is Block 1 (types and \
             injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructors, \
             std traits, helpers); a Block 4 means the file has outgrown it.\n\
             FIX: Fold the extra blocks back into Block 3, or move the \
             behaviour they hold into a capability or utility file so the agent \
             returns to 3 blocks.",
            found.join(", ")
        ),
    ));
}

/// The `N` of every `Block N:` banner comment, in declaration order.
///
/// A banner is `Block <digits>:` standing as its own word inside a comment. Both
/// halves matter. The colon separates a marker from prose — the HOW-TO and rule
/// messages describe the structure as `Block 1 (type + injected deps) -> Block
/// 2`, and requiring the colon keeps those sentences from reading as markers.
/// The word boundary keeps a longer word that merely contains "Block" — such as
/// `Sub-Block 4:` — from reading as a banner either. A file carrying no banner
/// is left to the other structural checks.
fn block_marker_numbers(content: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !is_comment(t) {
            continue;
        }
        let Some((idx, rest)) = standalone_word(t, "Block") else {
            continue;
        };
        let rest = &t[idx + rest..];
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() || !rest[digits.len()..].starts_with(':') {
            continue;
        }
        if let Ok(n) = digits.parse::<usize>() {
            out.push(n);
        }
    }
    out
}

/// The byte offset of *word* in `t` when it stands as a standalone token, plus
/// the length of the characters that follow it.
///
/// `Sub-Block 4:` contains the substring "Block" but the character before it is
/// part of a longer word, so it is not a standalone occurrence. Rust identifiers
/// treat `-` as a separator, but a banner comment is prose: a hyphenated
/// `Sub-Block` reads as one word to a human, so a hyphen counts as part of the
/// preceding token here.
fn standalone_word(t: &str, word: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = t[from..].find(word) {
        let idx = from + rel;
        let before_ok = t[..idx]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '-');
        let after = &t[idx + word.len()..];
        let after_ok = after
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        if before_ok && after_ok {
            // Return the offset *after* the word and the character that follows
            // it, measured from the character that follows — never from the
            // word's own first byte, which is always one byte and would leave
            // the slice inside a multi-byte character when the next character is
            // not ASCII (a fullwidth colon in a comment, say).
            let skip = after.chars().next().map_or(0, char::len_utf8);
            return Some((idx, word.len() + skip));
        }
        from = idx + word.len();
    }
    None
}

/// The workspace root: the ancestor that owns a shared source folder
/// (`crates/shared/src` in the Rust layout, `modules/shared/src` in the
/// Python modules layout).
fn workspace_root(file_path: &Path) -> Option<std::path::PathBuf> {
    let mut current = file_path.parent();
    while let Some(dir) = current {
        if dir.join("crates").join("shared").join("src").is_dir()
            || dir.join("modules").join("shared").join("src").is_dir()
        {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

/// The crate directory name that owns a file: the segment before `src`.
///
/// `crates/role-rules/src/agent_role_orchestrator.rs` yields `role-rules`.
fn owning_crate_name(file_path: &Path) -> Option<String> {
    let dir = file_path.parent()?;
    if dir.file_name().and_then(|n| n.to_str()) != Some("src") {
        return None;
    }
    let crate_dir = dir.parent()?;
    crate_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
}

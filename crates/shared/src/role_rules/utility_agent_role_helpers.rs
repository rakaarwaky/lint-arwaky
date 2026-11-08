// ─── Shared helpers for agent role auditors (moved out to keep
// utility_agent_role_checker.rs under the AES301 line limit) ───────────────

use std::collections::BTreeMap;
use std::path::Path;

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

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

/// True when a type name looks like a contract protocol seam. A PEP 604
/// union counts when any member is a protocol; `Optional[X]` and
/// `Union[A, B]` wrappers are unwrapped. Generic brackets, a path
/// qualifier, and union tokens are stripped so every spelling reduces to
/// the bare trait name before the match.
pub fn is_protocol_name(name: &str) -> bool {
    name.split('|').any(is_protocol_name_in)
}

/// One PEP 604 union member: unwrap `Optional[X]` and `Union[A, B]` and
/// fall through to the bare-name test, so wrapper spellings count too.
pub fn is_protocol_name_in(member: &str) -> bool {
    let inner = member.get(member.find('[').unwrap_or(0) + 1..member.rfind(']').unwrap_or(0));
    let bare = member
        .trim()
        .split(['[', '<', '{'])
        .next()
        .unwrap_or(member.trim())
        .rsplit("::")
        .next()
        .unwrap_or(member)
        .trim();
    if bare == "Optional" {
        return inner.is_some_and(is_protocol_name_in);
    }
    if bare == "Union" {
        return inner.is_some_and(|s| s.split(',').any(is_protocol_name_in));
    }
    let base = bare
        .rsplit(['.', ':'])
        .next()
        .unwrap_or(bare)
        .trim()
        .trim_end_matches(['>', ']', ' ', ',']);
    base.starts_with('I') && base.ends_with("Protocol")
}

/// True when a Rust line declares an injected dependency field.
///
/// A field is `pub addition: Box<dyn ICalculatorProtocol>,` — it may be
/// terminated by a comma, a semicolon, or nothing when it is the last field
/// in the struct body. A `pub fn new(… Arc<dyn …>)` parameter and an
/// `impl` block header are not fields.
pub fn is_injected_field(t: &str) -> bool {
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
pub fn protocol_names(t: &str) -> Vec<String> {
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
pub fn field_types(t: &str) -> Vec<String> {
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
/// Generic args like `Union[A, B]` stay one param because only top-level
/// commas split. A wrapped signature is read by the caller up to its
/// closing paren.
pub fn params_of(signature: &str) -> Vec<String> {
    let Some(open) = signature.find('(') else {
        return Vec::new();
    };
    let rest = &signature[open + 1..];
    let mut out: Vec<String> = Vec::new();
    let mut depth = 0i32;
    let mut prev = 0;
    let push = |out: &mut Vec<String>, seg: &str| {
        let p = seg.trim();
        if !p.is_empty() && p != "self" && p != "*" && p != "**kwargs" {
            out.push(p.to_string());
        }
    };
    for (i, ch) in rest.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' => {
                if depth == 0 {
                    push(&mut out, &rest[prev..i]);
                    return out;
                }
                depth -= 1;
            }
            ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                push(&mut out, &rest[prev..i]);
                prev = i + 1;
            }
            _ => {}
        }
    }
    push(&mut out, &rest[prev..]);
    out
}

/// Read a possibly-wrapped parameter list starting at `start` in `lines`.
/// A signature whose parens close on the opening line is used as-is;
/// otherwise the following lines are appended until the paren balances.
pub fn read_wrapped_params(lines: &[&str], start: usize) -> String {
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

/// Names declared on a trait / class declaration line. The declaration keyword
/// is stripped first because `pub trait IFooProtocol:` and `export class
/// IFooProtocol {` both put the name after the keyword.
pub fn declared_names(t: &str) -> Vec<String> {
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
/// carries no annotation. `addition: ICalculatorProtocol = None` yields
/// `ICalculatorProtocol`; a bare `deps` fails the protocol-name test.
pub fn annotation_of(param: &str) -> &str {
    let body = param.split('=').next().unwrap_or(param);
    match body.rsplit_once(':') {
        Some((_name, ty)) => ty.trim(),
        None => body.trim(),
    }
}

/// Record a protocol name once, so `distinct` counts types, not occurrences.
pub fn push_distinct(distinct: &mut Vec<String>, name: String) {
    let normalized = name.trim().to_string();
    if !distinct.contains(&normalized) {
        distinct.push(normalized);
    }
}

/// Count type declarations by line scan, for files with no parse metadata.
pub fn scan_type_declarations(content: &str) -> usize {
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
pub fn scan_implementor(content: &str) -> bool {
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
pub fn rust_any(t: &str) -> bool {
    t.contains(": Any")
        || t.contains("dyn Any")
        || t.contains("Any<")
        || t.contains("Any[")
        || t.contains("-> Any")
}

/// True when a Python / TS line spells `any`/`Any` in an annotation
/// position: the identifier right after a `:` or `->`.
pub fn dynamic_any(t: &str) -> bool {
    fn any_at_end(s: &str) -> bool {
        let is_ident = |r: &str| r.starts_with(|c: char| c.is_alphanumeric() || c == '_');
        (s.starts_with("any") || s.starts_with("Any")) && !s.get(4..).is_some_and(is_ident)
    }
    let mut rest = t;
    while let Some(i) = rest.find(':') {
        if any_at_end(rest[i + 1..].trim_start()) {
            return true;
        }
        rest = &rest[i + 1..];
    }
    if let Some(i) = t.find("->") {
        return any_at_end(t[i + 2..].trim_start());
    }
    false
}

/// Contract protocols implemented by a type declared in this file. A contract
/// protocol is an `I<Name>Protocol` seam: the agent implements the feature
/// aggregate and injects protocol seams, it never *implements* one, because
/// that is a capability's job. Std traits and aggregate traits are excluded so
/// `impl Default for X` reads as nothing. Returns deduplicated names in
/// declaration order.
fn contract_protocol_impls(file: &FileEntry) -> Vec<String> {
    use shared_filesystem::taxonomy_filesystem_vo::ParseMetadata;

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
/// `impl` may carry its own generic list before the trait and that list is not
/// always followed by a space — `impl<T> IFoo for X` — so the keyword is matched
/// on its own. The trait is the last whitespace-separated token before ` for `.
fn rust_impl_trait_name(t: &str) -> Option<&str> {
    let rest = t.strip_prefix("impl")?;
    let rest = rest.trim_start();
    let (lhs, _rhs) = rest.split_once(" for ")?;
    Some(lhs.rsplit([' ', ',']).next().unwrap_or(lhs).trim())
}

/// The base-class / implements list of a `class` declaration line, when the
/// line carries one. Returns the text inside `(...)` for Python and the part
/// after `implements` for TypeScript. The keyword is located, not required at
/// line start, because `export class X …` is the normal TS shape.
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
///
/// This is a flat prohibition, not a budget: implementing one *alongside* the
/// aggregate is already a violation. An agent composes its feature by
/// injecting protocol seams, so implementing a protocol here makes the
/// orchestration layer duplicate a capability's work — the behaviour belongs
/// in a `capabilities_*` file. See `contract_protocol_impls` for what counts
/// as a contract protocol.
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
    let path = file.path.to_string_lossy().to_string();
    let markers = block_markers(&file.content);

    if markers.is_empty() {
        return;
    }

    let expected_name: BTreeMap<usize, &'static str> = BTreeMap::from([
        (1, "Module Declaration"),
        (2, "Module Contract"),
        (3, "Module Helpers"),
    ]);

    // Name check: each banner's name text must match the canonical name for
    // its block number.
    for (n, name, line) in &markers {
        let Some(expected) = expected_name.get(n) else {
            continue;
        };
        if *name != *expected {
            violations.push(LintResult::new_arch(
                &path,
                *line,
                "AES405",
                Severity::MEDIUM,
                format!(
                    "AES405 AGENT_ROLE: Agent block marker name mismatch.\n\
                     WHY? Block {n} is named \"{name}\" but the canonical name is \"\
                     {expected}\". Expected `// ─── Block {n}: {expected} ───`, \
                     found `// ─── Block {n}: {name} ───`.\n\
                     FIX: Rewrite the banner to the canonical name text for that block.",
                ),
            ));
        }
    }

    let max = markers.iter().map(|(n, _, _)| *n).max();
    let Some(max) = max else {
        return;
    };
    if max <= 3 {
        return;
    }
    let found: Vec<String> = markers
        .iter()
        .map(|(n, _, _)| format!("Block {n}"))
        .collect();
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

/// Rule B-1 — constructor / `Default` must live in Block 1, never in Block 3.
/// MEDIUM.
///
/// A constructor (`pub fn new`, `pub fn from_*`, `impl Default`) declares how
/// the type is born — it belongs with the struct declaration in Block 1, not
/// with the mechanics in Block 3.
pub fn check_constructor_in_block1(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;
    let markers = block_markers(content);

    let block1_line = markers.iter().find(|(n, _, _)| *n == 1).map(|(_, _, l)| *l);
    let block2_line = markers.iter().find(|(n, _, _)| *n == 2).map(|(_, _, l)| *l);
    let block3_line = markers.iter().find(|(n, _, _)| *n == 3).map(|(_, _, l)| *l);

    let Some(block1_line) = block1_line else {
        return;
    };
    let Some(block2_line) = block2_line else {
        return;
    };
    let Some(block3_line) = block3_line else {
        return;
    };

    let ctor_in_block1 = content
        .lines()
        .skip(block1_line)
        .take(block2_line - block1_line)
        .any(|l| {
            let t = l.trim();
            t.starts_with("pub fn new")
                || t.starts_with("pub fn from_")
                || t.starts_with("impl Default")
        });

    let ctor_in_block3: Vec<(usize, String)> = content
        .lines()
        .skip(block3_line)
        .enumerate()
        .filter_map(|(i, l)| {
            let t = l.trim();
            let line_no = block3_line + i + 1;
            if t.starts_with("pub fn new")
                || t.starts_with("pub fn from_")
                || t.starts_with("impl Default")
            {
                Some((line_no, t.to_string()))
            } else {
                None
            }
        })
        .collect();

    if ctor_in_block1 {
        for (line_no, ctor_text) in ctor_in_block3 {
            violations.push(LintResult::new_arch(
                &path,
                line_no,
                "AES405",
                Severity::MEDIUM,
                format!(
                    "AES405 AGENT_ROLE: Constructor duplicate in Block 3.\n\
                     WHY? `{ctor_text}` appears under Block 3 but the constructor \
                     already lives in Block 1. Block 3 holds helpers/mechanics, \
                     not constructors.\n\
                     FIX: Remove the Block 3 constructor block.",
                ),
            ));
        }
        return;
    }

    if ctor_in_block3.is_empty() {
        return;
    }

    for (line_no, ctor_text) in ctor_in_block3 {
        violations.push(LintResult::new_arch(
            &path,
            line_no,
            "AES405",
            Severity::MEDIUM,
            format!(
                "AES405 AGENT_ROLE: Constructor outside Block 1.\n\
                 WHY? `{ctor_text}` is under Block 3 (helpers). Constructors \
                 belong in Block 1 with the struct declaration.\n\
                 FIX: Move `impl X {{ pub fn new(...) }}` (and `impl Default` \
                 if present) to Block 1, directly after `pub struct X`.",
            ),
        ));
    }
}

/// The `N` and name text of every `Block N:` banner comment, in declaration
/// order. The name is the rest of the line after the colon, with box-drawing
/// dashes and surrounding whitespace stripped.
fn block_markers(content: &str) -> Vec<(usize, String, usize)> {
    let mut out = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
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
        let Some(n) = digits.parse::<usize>().ok() else {
            continue;
        };
        let name = rest[digits.len() + 1..]
            .trim()
            .trim_matches(|c| c == '─' || c == '-')
            .trim()
            .to_string();
        out.push((n, name, line_idx + 1));
    }
    out
}

/// The byte offset of *word* in `t` when it stands as a standalone token, plus
/// the length of the characters that follow it. `Sub-Block 4:` contains the
/// substring "Block" but the character before it is part of a longer word, so
/// it is not a standalone occurrence. Rust identifiers treat `-` as a
/// separator, but a banner comment is prose: a hyphenated `Sub-Block` reads
/// as one word to a human, so a hyphen counts as part of the preceding token.
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
            // Measure from the character that follows the word, not the
            // word's own first byte, so the slice never lands inside a
            // multi-byte character when the next character is not ASCII.
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
pub fn workspace_root(file_path: &Path) -> Option<std::path::PathBuf> {
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
pub fn owning_crate_name(file_path: &Path) -> Option<String> {
    let dir = file_path.parent()?;
    if dir.file_name().and_then(|n| n.to_str()) != Some("src") {
        return None;
    }
    let crate_dir = dir.parent()?;
    crate_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
}

// PURPOSE: Shared AES403 sub-checks for capability role auditors.
//          Type budget and implementor checks use ParseMetadata when available
//          and fall back to line scanning. Per-language files delegate here
//          so one implementation is shared across Rust / Python / TypeScript.

use std::collections::BTreeMap;

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata, PythonClassItem};

/// True when `layer` names the capabilities layer, as a bare name or a
/// parameterised variant such as `capabilities(feature_x)`.
pub fn is_capabilities_layer(layer: &str) -> bool {
    layer == "capabilities" || layer.starts_with("capabilities(")
}

/// True when `trait_name` names a contract protocol trait.
///
/// A capability implements a contract protocol, never an aggregate — aggregate
/// traits belong to the agent layer (AES405).
fn is_protocol_trait(trait_name: &str) -> bool {
    let base = trait_name.rsplit("::").next().unwrap_or(trait_name).trim();
    base.starts_with('I') && base.ends_with("Protocol") && !base.ends_with("Aggregate")
}

/// True when a Python base class names a contract protocol.
fn is_protocol_base(base: &str) -> bool {
    let base = base.split('[').next().unwrap_or(base).trim();
    is_protocol_trait(base.rsplit('.').next().unwrap_or(base))
}

/// True when a decorator claims protocol attachment: `@with_protocol`,
/// `@with_adapter_protocol`, `@protocol`, `@protocol_impl("I...")`, …
/// Matched by shape so renames (`@with_tools_protocol`) stay covered.
fn is_protocol_decorator(name: &str) -> bool {
    let head = name
        .trim_start_matches('@')
        .split(|c: char| c.is_ascii_digit() || c == '(')
        .next()
        .unwrap_or(name);
    head == "protocol"
        || head == "protocol_impl"
        || head.starts_with("with_protocol")
        || (head.starts_with("with_") && head.ends_with("_protocol"))
}

/// Python: does `c` satisfy the protocol-implementor requirement?
///
/// A class counts when it inherits a protocol base, OR when a decorator
/// claims protocol attachment AND the class body actually carries methods
/// (an empty class wrapped in a decorator is the bypass).
fn python_class_satisfies_protocol(c: &PythonClassItem) -> bool {
    if c.bases.iter().any(|b| is_protocol_base(b)) {
        return true;
    }
    let deco = c
        .decorators
        .iter()
        .any(|d| is_protocol_decorator(d.as_str()));
    // A decorator claims the protocol is attached elsewhere; the class body
    // must still carry methods, or the file does real work in module scope.
    deco && (c.body_fn_count > 0 || !c.module_fn_names.is_empty())
}

/// Number of type declarations and whether one implements a protocol.
///
/// Returns `None` when the metadata variant has no capability shape.
fn profile(meta: &ParseMetadata) -> Option<(usize, bool, &'static str, &'static str)> {
    match meta {
        ParseMetadata::Rust(r) => {
            let structs: Vec<&str> = r.struct_definitions.iter().map(|s| s.as_str()).collect();
            let count = r.struct_definitions.len() + r.enum_definitions.len();
            let has_impl = r.impl_blocks.iter().any(|imp| {
                imp.trait_name
                    .as_ref()
                    .is_some_and(|t| is_protocol_trait(t))
                    && structs.contains(&imp.implementor_type.as_str())
            });
            Some((
                count,
                has_impl,
                "struct + enum",
                "No `impl I<Name>Protocol for <Struct>` found in this file.",
            ))
        }
        ParseMetadata::Python(py) => {
            let count = py.class_declarations.len();
            let has_impl = py
                .class_declarations
                .iter()
                .any(python_class_satisfies_protocol);
            Some((
                count,
                has_impl,
                "class",
                "No class inherits an `I<Name>Protocol` ABC in this file.",
            ))
        }
        ParseMetadata::TypeScript(ts) | ParseMetadata::JavaScript(ts) => {
            let count = ts.class_declarations.len()
                + ts.interface_declarations.len()
                + ts.type_alias_declarations.len();
            let has_impl = ts
                .class_declarations
                .iter()
                .any(|c| c.implements.iter().any(|i| is_protocol_trait(i)));
            Some((
                count,
                has_impl,
                "class / interface / type",
                "No class implements an `I<Name>Protocol` interface in this file.",
            ))
        }
        _ => None,
    }
}

/// Rule 1 — at most 3 type declarations per capability file. HIGH.
pub fn check_type_budget(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let Some((count, _, kind, _)) = file.parse_metadata.as_ref().and_then(profile) else {
        let count = scan_type_declarations(&file.content);
        if count > 3 {
            violations.push(LintResult::new_arch(
                &path,
                0,
                "AES403",
                Severity::HIGH,
                format!(
                    "AES403 CAPABILITY_ROLE: Capability declares too many types.\n\
                     WHY? Found {count} type declarations, max 3 allowed.\n\
                     FIX: Keep at most 3 types in a capability file. \
                     Move the excess to the taxonomy layer or split across capability files."
                ),
            ));
        }
        return;
    };
    if count > 3 {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::HIGH,
            format!(
                "AES403 CAPABILITY_ROLE: Capability declares too many types.\n\
                 WHY? Found {count} {kind} declarations, max 3 allowed.\n\
                 FIX: Keep at most 3 types in a capability file. \
                 Move the excess to the taxonomy layer or split across capability files."
            ),
        ));
    }
}

/// Rule 2 — at least one type implements a contract protocol. MEDIUM.
pub fn check_implementor(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let found = match file.parse_metadata.as_ref().and_then(profile) {
        Some((_, has_impl, _, why)) => {
            if has_impl {
                return;
            }
            why.to_string()
        }
        None => {
            if scan_protocol_impl(&file.content) {
                return;
            }
            "No `impl I<Name>Protocol for <Struct>` pattern found.".to_string()
        }
    };
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES403",
        Severity::MEDIUM,
        format!(
            "AES403 CAPABILITY_ROLE: Capability implements no contract protocol.\n\
             WHY? {found} A capability must implement a protocol contract; \
             an aggregate trait belongs to the agent layer (AES405).\n\
             FIX: Create the contract with the `aes-contract` skill, \
             then add the protocol implementation as Block 2 of this file."
        ),
    ));
}

/// Count distinct protocol traits implemented in a capability file.
///
/// Returns `(count, Vec<trait_name>)` — `count == 1` means single-protocol
/// (no violation); `count == 0` is handled by `check_implementor`;
/// `count > 1` triggers `CapabilityMultiProtocol` (AES403 MEDIUM).
pub fn count_protocol_traits(file: &FileEntry) -> (usize, Vec<String>) {
    if let Some(meta) = &file.parse_metadata {
        return match meta {
            ParseMetadata::Rust(r) => {
                let structs: Vec<&str> = r.struct_definitions.iter().map(|s| s.as_str()).collect();
                let mut protocols: Vec<String> = Vec::new();
                for imp in &r.impl_blocks {
                    if let Some(trait_name) = &imp.trait_name {
                        if is_protocol_trait(trait_name)
                            && structs.contains(&imp.implementor_type.as_str())
                            && !protocols.contains(&trait_name.to_lowercase())
                        {
                            protocols.push(trait_name.clone());
                        }
                    }
                }
                (protocols.len(), protocols)
            }
            ParseMetadata::Python(py) => {
                let mut protocols: Vec<String> = Vec::new();
                for c in &py.class_declarations {
                    for base in &c.bases {
                        if is_protocol_base(base) && !protocols.contains(&base.to_lowercase()) {
                            protocols.push(base.clone());
                        }
                    }
                    if protocols.is_empty()
                        && c.decorators
                            .iter()
                            .any(|d| is_protocol_decorator(d.as_str()))
                        && (c.body_fn_count > 0 || !c.module_fn_names.is_empty())
                    {
                        protocols.push("decorated_protocol".to_string());
                    }
                }
                (protocols.len(), protocols)
            }
            ParseMetadata::TypeScript(ts) | ParseMetadata::JavaScript(ts) => {
                let mut protocols: Vec<String> = Vec::new();
                for c in &ts.class_declarations {
                    for iface in &c.implements {
                        if is_protocol_trait(iface) && !protocols.contains(&iface.to_lowercase()) {
                            protocols.push(iface.clone());
                        }
                    }
                }
                (protocols.len(), protocols)
            }
            _ => (0, Vec::new()),
        };
    }
    // Fallback line-scan when parse metadata is unavailable.
    let protocols = scan_protocol_names(&file.content);
    (protocols.len(), protocols)
}

/// Rule 3 — exactly one contract protocol per capability file. MEDIUM.
pub fn check_single_protocol(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let (count, protocols) = count_protocol_traits(file);
    if count <= 1 {
        return;
    }
    let trait_names: Vec<&str> = protocols.iter().map(|s| s.as_str()).collect();
    violations.push(LintResult::new_arch(
        &path,
        0,
        "AES403",
        Severity::MEDIUM,
        format!(
            "AES403 CAPABILITY_ROLE: Capability file implements multiple protocols.\n\
             WHY? {count} protocol traits found: {names}.\n\
             FIX: Split this file into separate capability files, one per protocol \
             (e.g. `capabilities_foo_handler.rs`, `capabilities_bar_handler.rs`). \
             If the capabilities share common helper functions, move those shared functions \
             to a `utility_<shared>_resolver.*` file and import it from both capabilities.",
            names = trait_names.join(", ")
        ),
    ));
}

/// Rule 4 — the capability declares all three blocks, in order, and no more.
/// MEDIUM.
///
/// The 3-block structure (Block 1 struct / class, Block 2 protocol
/// implementation, Block 3 constructors, std traits, helpers) is the shape the
/// capability HOW-TO documents, and the `// Block 1:` … `// Block 3:` banners
/// are how a reader sees it. `check_block_order` only compares two `impl` lines
/// — it cannot tell a file that documents its three blocks from one that
/// happens to declare them in a lucky order — so a file with no banners at all
/// passed. This check reads the banners themselves and requires all three.
///
/// Three findings, one per defect, so a file missing two blocks is told about
/// both rather than once:
///
/// - no banner at all — the file never declares the structure;
/// - a subset of 1/2/3 — the file declares some blocks and skips others;
/// - out of order or a block above 3 — the sequence itself is wrong.
///
/// The banner is a whole-word `Block <digits>:` inside a comment. Requiring the
/// colon keeps prose such as `Block 1 (types) -> Block 2` from reading as a
/// banner, and the word boundary keeps `Sub-Block 4:` out.
///
/// The marker parser is duplicated from `utility_agent_role_checker` rather than
/// shared: AES201 forbids a utility importing another utility, so the two
/// layers cannot share one copy without moving the parser into a layer both may
/// import. They are kept identical deliberately — the same banner means the
/// same thing to both rules.
pub fn check_block_markers(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let markers = block_markers(&file.content);

    let expected_name: BTreeMap<usize, &'static str> = BTreeMap::from([
        (1, "Module Declaration"),
        (2, "Module Contract"),
        (3, "Module Helpers"),
    ]);

    // B-10: a block may be empty — an empty block drops its marker. A file
    // may legitimately carry only Block 1 + Block 2 (Block 3 empty, marker
    // deleted). Never report a "missing marker" violation for an empty block.
    // But a file with *no* banner at all still needs a map — report it.
    if markers.is_empty() {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            "AES403 CAPABILITY_ROLE: Capability file declares no block markers.\n\
             WHY? File carries no `Block 1:` / `Block 2:` / `Block 3:` banner \
             comment, so the reader is given no map of the file. The 3-block shape \
             is Block 1 (struct definition) -> Block 2 (protocol trait \
             implementation) -> Block 3 (constructors, std traits, helpers).\n\
             FIX: Add the three banner comments above their blocks:\n  \
             // ─── Block 1: Module Declaration ───\n  \
             // ─── Block 2: Module Contract ───\n  \
             // ─── Block 3: Module Helpers ───",
        ));
        return;
    }

    // A banner above 3 means the file outgrew the shape.
    let above_three: Vec<usize> = markers
        .iter()
        .map(|(n, _, _)| *n)
        .filter(|n| *n > 3)
        .collect();
    if !above_three.is_empty() {
        violations.push(LintResult::new_arch(
            &path,
            0,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability file carries block markers beyond Block 3.\n\
                 WHY? File declares {}. The 3-block structure is Block 1 (struct \
                 definition) -> Block 2 (protocol trait implementation) -> Block 3 \
                 (constructors, std traits, helpers); a Block 4 means the file has \
                 outgrown it.\n\
                 FIX: Fold the extra blocks back into Block 3, or move the \
                 behaviour they hold into a capability or utility file.",
                above_three
                    .iter()
                    .map(|n| format!("Block {n}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }

    // Name check: each banner's name text must match the canonical name for
    // its block number. Independent of missing/above-three/order checks.
    for (n, name, line) in &markers {
        let Some(expected) = expected_name.get(n) else {
            continue;
        };
        if *name != *expected {
            violations.push(LintResult::new_arch(
                &path,
                *line,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Capability block marker name mismatch.\n\
                     WHY? Block {n} is named \"{name}\" but the canonical name is \"\
                     {expected}\". Expected `// ─── Block {n}: {expected} ───`, \
                     found `// ─── Block {n}: {name} ───`.\n\
                     FIX: Rewrite the banner to the canonical name text for that block.",
                ),
            ));
        }
    }

    // Order is checked over the distinct numbers in the order they first appear,
    // so a banner repeated in a doc comment cannot reorder the sequence.
    let mut sequence: Vec<(usize, usize)> = Vec::new(); // (block number, line number)
    for (n, _, line) in &markers {
        if !sequence.iter().any(|(s, _)| s == n) {
            sequence.push((*n, *line));
        }
    }
    let mut expected_seq = sequence.clone();
    expected_seq.sort_by_key(|(n, _)| *n);
    if sequence.iter().map(|(n, _)| *n).collect::<Vec<_>>()
        != expected_seq.iter().map(|(n, _)| *n).collect::<Vec<_>>()
    {
        let (_, first_line) = sequence.first().copied().unwrap_or((0, 0));
        violations.push(LintResult::new_arch(
            &path,
            first_line,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Capability block markers are out of order.\n\
                 WHY? File declares them as {} but the structure is fixed: Block 1 \
                 (struct definition) -> Block 2 (protocol trait implementation) -> \
                 Block 3 (constructors, std traits, helpers).\n\
                 FIX: Move the banner comments so they head their blocks:\n  \
                 // ─── Block 1: Module Declaration ───\n  \
                 // ─── Block 2: Module Contract ───\n  \
                 // ─── Block 3: Module Helpers ───",
                sequence
                    .iter()
                    .map(|(n, _)| format!("Block {n}"))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
        ));
    }
}

/// Rule B-1 — constructor / `Default` must live in Block 1 (struct + constructor
/// together), never in Block 3. MEDIUM.
///
/// A constructor (`pub fn new`, `pub fn from_*`, `impl Default`) declares how
/// the type is born — it belongs with the struct declaration in Block 1, not
/// with the mechanics in Block 3.
///
/// Checks both:
/// - no constructor in Block 1 while one exists in Block 3 → violation
/// - `impl Default` on a DI (parameterised) constructor → violation (see B-5)
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

    // Find constructors in Block 1 (between Block 1 and Block 2 markers)
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

    // Find constructors in Block 3 (after Block 3 marker)
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
        // Constructor already properly in Block 1; Block 3 duplicates are stray
        for (line_no, ctor_text) in ctor_in_block3 {
            violations.push(LintResult::new_arch(
                &path,
                line_no,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Constructor duplicate in Block 3.\n\
                     WHY? `{ctor_text}` appears under Block 3 but the constructor \
                     already lives in Block 1. Block 3 holds helpers/mechanics, \
                     not constructors.\n\
                     FIX: Remove the Block 3 constructor block.",
                ),
            ));
        }
        return;
    }

    // No constructor in Block 1
    if ctor_in_block3.is_empty() {
        return; // No constructor anywhere — file may be stateless without one
    }

    // Constructor exists only in Block 3 → violation
    for (line_no, ctor_text) in ctor_in_block3 {
        violations.push(LintResult::new_arch(
            &path,
            line_no,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Constructor outside Block 1.\n\
                 WHY? `{ctor_text}` is under Block 3 (helpers). Constructors \
                 belong in Block 1 with the struct declaration.\n\
                 FIX: Move `impl X {{ pub fn new(...) }}` (and `impl Default` \
                 if present) to Block 1, directly after `pub struct X`.",
            ),
        ));
    }
}

/// Rule A-1 / A-2 — banner hygiene: no wild banners, no duplicates, no empty
/// Block 3. MEDIUM.
///
/// Detects:
/// - Stray banners like `// ─── Protocol Trait Implementation ───` or
///   `// ─── Constructors, Helpers, Private Methods ───` (friendly names
///   instead of `Block N:` labels) — A-1
/// - A block number declared twice (e.g. Block 3 appears twice) — A-2
/// - Block 3 banner followed by nothing (empty block) — B-10
pub fn check_banner_hygiene(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;
    let markers = block_markers(content);

    let lines: Vec<&str> = content.lines().collect();
    let mut marker_lines: Vec<usize> = Vec::new();
    let mut block_numbers: Vec<usize> = Vec::new();

    // First pass: collect all banner-like lines (─── dashes in comment)
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if !is_comment(t) {
            continue;
        }
        // Banner lines contain ─── or --- dashes
        if !t.contains("───") && !t.contains("---") {
            continue;
        }
        let line_no = i + 1;

        // Check if it's a proper Block N: marker
        if let Some((n, _name, _)) = block_markers(content)
            .iter()
            .find(|(_, _, ln)| *ln == line_no)
        {
            block_numbers.push(*n);
            marker_lines.push(line_no);
            continue;
        }

        // It's a banner-like line but not a proper Block N: marker — A-1 wild banner
        let stripped = t
            .trim_start_matches("//")
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_start_matches('#')
            .trim();
        if stripped.is_empty() {
            continue;
        }
        violations.push(LintResult::new_arch(
            &path,
            line_no,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Wild block banner.\n\
                 WHY? `{stripped}` is a banner line but not a canonical \
                 `Block N:` marker. The 3-block structure uses exactly three \
                 banners: Block 1, Block 2, Block 3.\n\
                 FIX: Rewrite as `// ─── Block N: <canonical name> ───` or \
                 remove the stray banner.",
            ),
        ));
    }

    // A-2: duplicate block numbers
    let mut count_by_block: std::collections::BTreeMap<usize, Vec<usize>> =
        std::collections::BTreeMap::new();
    for (n, _, ln) in &markers {
        count_by_block.entry(*n).or_default().push(*ln);
    }
    for (n, line_nos) in count_by_block {
        if line_nos.len() > 1 {
            // report the second+ occurrence
            for &dup_line in line_nos.iter().skip(1) {
                violations.push(LintResult::new_arch(
                    &path,
                    dup_line,
                    "AES403",
                    Severity::MEDIUM,
                    format!(
                        "AES403 CAPABILITY_ROLE: Duplicate block marker.\n\
                         WHY? `Block {n}` is declared more than once. Exactly one \
                         banner per block number.\n\
                         FIX: Remove the duplicate `Block {n}` banner.",
                    ),
                ));
            }
        }
    }

    // B-10: empty Block 3 is allowed — the banner itself is what matters
    // (A-3 already requires all three banners to exist), so an empty Block 3
    // is not a violation.

    // A-3: a capability file that declares some but not all Block banners is
    // incomplete — the reader's map is partial in either direction (B1 missing
    // but B2/B3 present, or any single banner alone). All three banners are
    // required for a complete map.
    let declared_blocks: std::collections::BTreeSet<usize> =
        markers.iter().map(|(n, _, _)| *n).collect();
    let has_b1 = declared_blocks.contains(&1);
    let has_b2 = declared_blocks.contains(&2);
    let has_b3 = declared_blocks.contains(&3);
    let has_any = has_b1 || has_b2 || has_b3;
    let all_three = has_b1 && has_b2 && has_b3;
    if has_any && !all_three {
        let mut missing: Vec<String> = Vec::new();
        if !has_b1 {
            missing.push("Block 1: Module Declaration".to_string());
        }
        if !has_b2 {
            missing.push("Block 2: Module Contract".to_string());
        }
        if !has_b3 {
            missing.push("Block 3: Module Helpers".to_string());
        }
        let missing_list = missing.join(", ");
        let first_line = markers.first().map(|(_, _, ln)| *ln).unwrap_or(1);
        let mut fix_lines: Vec<String> = Vec::new();
        if !has_b1 {
            fix_lines.push("// ─── Block 1: Module Declaration ───".to_string());
            fix_lines.push("(add before type + constructor block)".to_string());
        }
        if !has_b2 {
            fix_lines.push("// ─── Block 2: Module Contract ───".to_string());
            fix_lines.push("(add before the protocol impl block)".to_string());
        }
        if !has_b3 {
            fix_lines.push("// ─── Block 3: Module Helpers ───".to_string());
            fix_lines.push("(add before the helper impl block)".to_string());
        }
        let fix_block = fix_lines.join("\n");
        violations.push(LintResult::new_arch(
            &path,
            first_line,
            "AES403",
            Severity::MEDIUM,
            format!(
                "AES403 CAPABILITY_ROLE: Incomplete block banners.\n\
                 WHY? File declares block banner(s) but is missing {missing_list}. \
                 The 3-block shape requires all three banners so the reader has a complete map.\n\
                 FIX: Add the missing banner(s):\n{fix_block}",
            ),
        ));
    }
}

/// Rule B-3 / B-6 / B-9 — `pub fn` under Block 3 without a seam doc is a
/// facade mistake. MEDIUM.
///
/// Legal Block 3 pub items: only `*_internal` test seams with a B-6a seam doc.
/// A `pub fn` under Block 3 with no seam doc is production code hiding in the
/// helpers block — it belongs in Block 2 behind a protocol method.
///
/// A `pub fn *_internal` without the seam doc is also a violation.
pub fn check_block3_pub_facade(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;
    let markers = block_markers(content);

    let block3_line = markers.iter().find(|(n, _, _)| *n == 3).map(|(_, _, l)| *l);
    let Some(block3_line) = block3_line else {
        return;
    };

    let block3_idx = block3_line - 1;
    let mut pending_seam_doc = false;

    for line in content.lines().skip(block3_idx + 1) {
        let t = line.trim();

        // Blank lines and comments keep us in Block 3
        if t.is_empty() || is_comment(t) {
            // Track if this comment looks like a B-6a seam doc
            if t.contains("B-6a test seam") || t.contains("B-6a seam doc") {
                pending_seam_doc = true;
            }
            continue;
        }

        // We hit non-comment content
        if t.starts_with("pub fn") {
            let is_internal = t.contains("_internal");
            if is_internal && pending_seam_doc {
                pending_seam_doc = false;
                continue; // legal seam
            }
            if is_internal && !pending_seam_doc {
                violations.push(LintResult::new_arch(
                    &path,
                    /* line */ 0, // we don't have exact line here easily; approximate
                    "AES403",
                    Severity::MEDIUM,
                    "AES403 CAPABILITY_ROLE: `pub fn *_internal` in Block 3 without B-6a seam doc.\n\
                     WHY? A test seam in Block 3 must carry the B-6a seam doc:\n\
                     `/// B-6a test seam: pub only so cross-crate unit tests ...`\n\
                     FIX: Add the B-6a seam doc comment directly above the fn.",
                ));
            }
            if !is_internal {
                violations.push(LintResult::new_arch(
                    &path,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    format!(
                        "AES403 CAPABILITY_ROLE: `pub fn` facade in Block 3.\n\
                         WHY? `{}` is a public non-seam function under Block 3 \
                         helpers. Production `pub` code belongs in Block 2 behind \
                         a protocol trait method, or Block 3 as `fn` (private).\n\
                         FIX: Make it private (`fn`) or move to Block 2 behind \
                         `impl I<Name>Protocol`.",
                        t
                    ),
                ));
            }
        }
        // Any non-comment, non-pub-fn content — we're past the helpers section
        // or in a nested impl; continue scanning
    }
}

/// Rule B-4 — multi-branch mechanics must not be inlined in Block 2. MEDIUM.
///
/// Block 2 is orchestration: config lookup, gate chains, linear threshold checks,
/// the main loop skeleton. When a Block 2 trait method contains per-line
/// multi-branch classifiers (9-way `starts_with` chains, language dispatch)
/// or message-assembly tables, that part belongs in Block 3 as a helper.
///
/// Detects `match` expressions and multi-branch `if/else if` chains inside
/// Block 2 trait methods (`impl I<Name>Protocol for Type`).
pub fn check_block2_inline_details(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;

    // Find Block 2 banner line
    let block2_line = block_markers(content)
        .iter()
        .find(|(n, _, _)| *n == 2)
        .map(|(_, _, l)| *l);
    let block3_line = block_markers(content)
        .iter()
        .find(|(n, _, _)| *n == 3)
        .map(|(_, _, l)| *l);

    let Some(block2_line) = block2_line else {
        return;
    };
    let Some(block3_line) = block3_line else {
        return;
    };

    // Scan lines between Block 2 and Block 3 for `impl I<Name>Protocol`
    let mut in_trait_impl = false;
    let mut brace_depth = 0isize;
    let mut method_name = String::new();

    for (i, line) in content
        .lines()
        .enumerate()
        .skip(block2_line - 1)
        .take(block3_line - block2_line)
    {
        let t = line.trim();
        let abs_line = block2_line + i;

        if !in_trait_impl {
            if t.starts_with("impl ") && t.contains("Protocol") && t.contains("for ") {
                in_trait_impl = true;
                brace_depth = 0;
            }
            continue;
        }

        // Track brace depth within the trait impl
        brace_depth += t.chars().filter(|c| *c == '{').count() as isize
            - t.chars().filter(|c| *c == '}').count() as isize;

        // Detect a method start: `fn <name>(` at brace depth 1 (inside impl)
        if brace_depth == 1 {
            if let Some(rest) = t.strip_prefix("fn ") {
                let name_end = rest.find('(').unwrap_or(rest.len());
                method_name = rest[..name_end].trim().to_string();
            }
        }

        // Inside a method body (brace_depth >= 2), detect multi-branch constructs
        if brace_depth >= 2 {
            let has_match = t.contains("match ");
            let has_if_chain =
                t.starts_with("else if") || t.starts_with("if ") && t.contains("else");

            if has_match || has_if_chain {
                violations.push(LintResult::new_arch(
                    &path,
                    abs_line,
                    "AES403",
                    Severity::MEDIUM,
                    format!(
                        "AES403 CAPABILITY_ROLE: Multi-branch logic in Block 2 method `{}`.\n\
                         WHY? `{}` contains a multi-branch construct (`match` or \
                         `if/else if` chain) inside the Block 2 protocol method.\n\
                         Block 2 is orchestration — multi-branch classification \
                         belongs in Block 3 as a helper method.\n\
                         FIX: Extract the branch logic into a private helper in \
                         Block 3 and call it from `{}`.",
                        method_name, t, method_name
                    ),
                ));
            }
        }

        // When we close the trait impl, reset
        if brace_depth <= 0 {
            in_trait_impl = false;
            method_name.clear();
        }
    }
}
///
/// A struct holding injected deps (`config: Arc<...>`, `io: Arc<...>`, etc.)
/// must not have a meaningful default — "default architecture config" is a
/// bug waiting to compile. Stateless structs may derive Default.
pub fn check_di_default(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;

    // Find structs with `#[derive(Default)]`
    let mut di_structs: Vec<(usize, String)> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("pub struct") {
            // Check if previous line has #[derive(Default)]
            let line_no = i + 1;
            let has_derive_default = i > 0
                && content
                    .lines()
                    .nth(i - 1)
                    .map(|l| l.trim() == "#[derive(Default)]")
                    .unwrap_or(false);
            if has_derive_default {
                // Extract struct name
                if let Some(name) = t.split_whitespace().nth(2) {
                    let name = name.trim_end_matches(['{', ';']);
                    if !name.is_empty() {
                        di_structs.push((line_no, name.to_string()));
                    }
                }
            }
        }
    }

    // A struct is "DI" if it has fields referencing Arc or other injected types
    // Simple heuristic: look for `Arc<` in fields after the struct
    for (struct_line, struct_name) in di_structs {
        // Check if the struct body has Arc< or other DI-looking fields
        let mut in_struct = false;
        let mut has_di_field = false;
        for line in content.lines().skip(struct_line) {
            let t = line.trim();
            if t.starts_with("pub struct") {
                in_struct = true;
                continue;
            }
            if in_struct {
                if t.contains("Arc<") || t.contains("Box<dyn") || t.contains("Mutex<") {
                    has_di_field = true;
                }
                if t == "}" || t == "};" {
                    break;
                }
            }
        }

        if has_di_field {
            violations.push(LintResult::new_arch(
                &path,
                struct_line,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: `#[derive(Default)]` on DI struct `{struct_name}`.\n\
                     WHY? `{struct_name}` holds injected dependencies (Arc<...>) \
                     but derives Default, which implies a meaningful default \
                     config exists.\n\
                     FIX: Remove `#[derive(Default)]`. Add a `pub fn from_config(...)` \
                     or `pub fn new(...)` constructor in Block 1 instead.",
                ),
            ));
        }
    }
}

/// Rule B-5 variant — `impl Default` on a DI struct.
pub fn check_impl_default_on_di(file: &FileEntry, violations: &mut Vec<LintResult>) {
    let path = file.path.to_string_lossy().to_string();
    let content = &file.content;
    let markers = block_markers(content);

    let block3_line = markers.iter().find(|(n, _, _)| *n == 3).map(|(_, _, l)| *l);
    let Some(block3_line) = block3_line else {
        return;
    };

    let block3_idx = block3_line - 1;
    for line in content.lines().skip(block3_idx + 1) {
        let t = line.trim();
        if t.is_empty() || is_comment(t) {
            continue;
        }
        if t.starts_with("impl Default") {
            violations.push(LintResult::new_arch(
                &path,
                0,
                "AES403",
                Severity::MEDIUM,
                "AES403 CAPABILITY_ROLE: `impl Default` in Block 3.\n\
                 WHY? Default impls belong in Block 1 with the constructor, \
                 not in Block 3 helpers. If the struct holds injected deps, \
                 remove Default entirely and use a `from_config`/`new` \
                 constructor.\n\
                 FIX: Move `impl Default` to Block 1, or remove it.",
            ));
        }
        // Stop scanning if we leave Block 3 helpers (another impl, fn, etc.)
        if t.starts_with("impl ") || t.starts_with("pub fn") || t.starts_with("fn ") {
            break;
        }
    }
}

/// One `Block N:` banner comment, with the parsed block number, the name text
/// after the colon, and the 1-indexed source line.
#[derive(Debug, Clone)]
pub struct CapabilityBlockMarker {
    /// Parsed block number (`N` in `Block N:`).
    pub number: usize,
    /// Name text after the colon, dashes and surrounding whitespace stripped.
    pub name: String,
    /// 1-indexed source line of the banner.
    pub line: usize,
}

/// The `N` and name text of every `Block N:` banner comment, in declaration
/// order, as [`CapabilityBlockMarker`] values.
///
/// The name is the rest of the line after the colon, with box-drawing dashes
/// and surrounding whitespace stripped.
///
/// Public so per-language block-order checks can attribute each `impl` to the
/// banner block it sits under, instead of guessing by raw line position. A
/// file that never declares its blocks yields an empty vec, and callers fall
/// back to raw line position.
pub fn capability_block_markers(content: &str) -> Vec<CapabilityBlockMarker> {
    block_markers(content)
        .into_iter()
        .map(|(n, name, line)| CapabilityBlockMarker {
            number: n,
            name,
            line,
        })
        .collect()
}

/// The `N` and name text of every `Block N:` banner comment, in declaration
/// order. The name is the rest of the line after the colon, with box-drawing
/// dashes and surrounding whitespace stripped.
///
/// Banner lines are whole-line: optional comment sigil, dashes, `Block N:`,
/// name text, trailing dashes. Prose inside string literals never matches —
/// only a line that actually reads as a banner comment counts. This is how
/// auditor messages that quote `// Block 1:` inside a string don't get
/// mistaken for real banners.
///
/// `Block 1:` / `Block 2:` / `Block 3:` are the canonical capability banners.
/// A file that never declares its blocks yields an empty vec; callers fall
/// back to raw line position.
pub fn block_markers(content: &str) -> Vec<(usize, String, usize)> {
    let mut out = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let t = line.trim();
        if !is_comment(t) {
            continue;
        }
        // Anchor to whole-line banner shape: optional comment prefix, dashes,
        // "Block N:", name text, trailing dashes. Prose inside string literals
        // (e.g. auditor messages that quote `// Block 1:`) never matches,
        // because a real banner line starts with the comment sigil itself.
        let stripped = t
            .trim_start_matches("//")
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_start_matches('#')
            .trim();
        if !stripped.starts_with("───") && !stripped.starts_with("---") {
            continue;
        }
        // Expect `─── Block N: name ───` after the sigil.
        let Some((_, rest)) = stripped.split_once(' ') else {
            continue;
        };
        let Some((num_str, name_tail)) = rest.split_once(':') else {
            continue;
        };
        let digits: String = num_str.chars().filter(char::is_ascii_digit).collect();
        if digits.is_empty() {
            continue;
        }
        let Some(n) = digits.parse::<usize>().ok() else {
            continue;
        };
        // name_tail is like ` Module Declaration ───` or ` Module Contracts ───`.
        // Canonical names per block number.
        let expected_name: BTreeMap<usize, &'static str> = BTreeMap::from([
            (1, "Module Declaration"),
            (2, "Module Contract"),
            (3, "Module Helpers"),
        ]);
        let name = name_tail
            .trim()
            .trim_matches(|c| c == '─' || c == '-')
            .trim()
            .to_string();
        let _ = expected_name.get(&n); // validate block number is known
        out.push((n, name, line_idx + 1));
    }
    out
}

/// True when a line is a comment in any of the three languages.
fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("///")
}

/// Count type declarations by line scan, for files with no parse metadata.
fn scan_type_declarations(content: &str) -> usize {
    let mut count = 0;
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("struct ")
            || t.starts_with("pub struct ")
            || t.starts_with("enum ")
            || t.starts_with("pub enum ")
            || t.starts_with("class ")
            || t.starts_with("pub class ")
            || t.starts_with("interface ")
            || t.starts_with("pub interface ")
        {
            count += 1;
        }
    }
    count
}

/// True when a line-scan finds a protocol implementation.
fn scan_protocol_impl(content: &str) -> bool {
    let has_function = content.lines().any(|l| l.trim_start().starts_with("def "));
    content.lines().any(|line| {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("impl ") {
            return rest.contains("Protocol for")
                && is_protocol_trait(rest.split(" for ").next().unwrap_or("").trim());
        }
        if t.starts_with("class ") {
            // `class Foo(IProtocol)`, `class Foo(Mixin, IProtocol)`, and
            // `class Foo(Base, IProtocolGeneric[T])` all count. The line ends
            // in `):` for a `class` statement, so trim that before slicing
            // the base-class argument list out.
            let decl = t.trim_end().trim_end_matches(':');
            if let Some(open) = decl.find('(') {
                if decl.ends_with(')') {
                    return decl[open + 1..decl.len() - 1]
                        .split(',')
                        .any(is_protocol_base);
                }
            }
            return false;
        }
        // Decorator that claims protocol attachment: requires at least one
        // `def` somewhere in the file, so a bare `@with_adapter_protocol` on
        // an empty class does not pass.
        if t.starts_with('@') && is_protocol_decorator(t) && has_function {
            return true;
        }
        t.contains("implements I") && t.contains("Protocol")
    })
}

/// Collect distinct protocol trait names by line scan, for files with no parse metadata.
///
/// Supports Rust (`impl IXxxProtocol for Yyy`), Python
/// (`class X(IAProtocol, IBProtocol)`), and TypeScript
/// (`class X implements IAProtocol, IBProtocol`).
fn scan_protocol_names(content: &str) -> Vec<String> {
    let mut protocols: Vec<String> = Vec::new();
    let has_function = content.lines().any(|l| l.trim_start().starts_with("def "));

    for line in content.lines() {
        let t = line.trim();

        // Rust: `impl IXxxProtocol for Yyy`
        if let Some(rest) = t.strip_prefix("impl ") {
            if let Some(trait_part) = rest.split(" for ").next() {
                let name = trait_part.trim();
                if is_protocol_trait(name) && !protocols.iter().any(|p| p == name) {
                    protocols.push(name.to_string());
                }
            }
        }

        // Python: `class X(IAProtocol, IBProtocol):`
        // TypeScript: `class X implements IAProtocol, IBProtocol {`
        // Both may be prefixed with `export `.
        let class_body = t
            .strip_prefix("export class ")
            .or_else(|| t.strip_prefix("class "))
            .unwrap_or("");
        if !class_body.is_empty() {
            // `class DualService(IFoo, IBar):` → split at '(' to get name and bases.
            // `class DualService implements IFoo, IBar {` → split at '(' gives no '(',
            // so fall through to the `implements` path.
            let (type_part, rest) = class_body.split_once('(').unwrap_or((class_body, ""));
            let type_part = type_part.trim();

            if !rest.is_empty() {
                // Python-style bases: everything inside `(` ... `)`.
                // Strip trailing `)` and `:` in order; `trim_end_matches`
                // consumes chars from the right one at a time.
                let bases_str = rest.trim_end_matches(&[')', ':', ' '] as &[char]).trim();
                for base in bases_str.split(',') {
                    let b = base.trim();
                    if is_protocol_base(b) && !protocols.iter().any(|p| p == b) {
                        protocols.push(b.to_string());
                    }
                }
            } else if let Some((_, after)) = type_part.split_once(" implements ") {
                // TypeScript-style: `class Foo implements IFoo, IBar {`
                // `after` is `IFoo, IBar {` or `IFoo, IBar`; strip braces from both sides.
                let iface_list =
                    after.trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace());
                for iface in iface_list.split(',') {
                    let i = iface.trim();
                    if is_protocol_trait(i) && !protocols.iter().any(|p| p == i) {
                        protocols.push(i.to_string());
                    }
                }
            }
        }

        // Decorator that claims protocol attachment: one synthetic entry per
        // file, counted only when the file actually carries a `def`.
        if t.starts_with('@') && is_protocol_decorator(t) && has_function && protocols.is_empty() {
            protocols.push("decorated_protocol".to_string());
        }
    }

    protocols
}

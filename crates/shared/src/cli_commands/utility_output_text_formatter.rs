// PURPOSE: UtilityFormatting — Stateless output formatting for all CLI surface actions.
// Single source of truth for text/json/sarif/junit output format.
// Dispatcher returns data (Vec<ViolationItem> / report structs); this module renders it.
// Uses existing VOs from shared: ErrorCode, FilePath, LintMessage, Severity.

use std::collections::{BTreeMap, BTreeSet};

use shared_common::ViolationItem;
use shared_common::taxonomy_format_vo::Format;

/// Format a violation location as "file:line:column".
pub fn format_location(file: &str, line: i64, column: i64) -> String {
    match (line, column) {
        (l, c) if l > 0 && c > 0 => format!("{}:{}:{}", file, l, c),
        (l, _) if l > 0 => format!("{}:{}", file, l),
        _ => file.to_string(),
    }
}

/// Group violations by workspace member name extracted from file paths.
pub fn group_by_member<'a>(
    violations: &'a [ViolationItem],
    root: &str,
    force_member: Option<&str>,
) -> BTreeMap<String, Vec<&'a ViolationItem>> {
    let mut grouped: BTreeMap<String, Vec<&ViolationItem>> = BTreeMap::new();
    for v in violations {
        let member = if let Some(m) = force_member {
            m.to_string()
        } else {
            extract_member_from_path(&v.file.value, root)
        };
        grouped.entry(member).or_default().push(v);
    }
    grouped
}

/// Extract workspace member name from a file path relative to the scan root.
fn extract_member_from_path(file_path: &str, root: &str) -> String {
    let normalized_root = root.trim_end_matches('/');
    let normalized_path = file_path.trim_start_matches("./");

    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];

    // For markdown docs at root level, show the actual file name
    if normalized_path.ends_with(".md") && !normalized_path.contains('/') {
        return normalized_path.to_string();
    }

    // Handle single-file scan (e.g., linting a specific file)
    if normalized_path.contains('/')
        && normalized_path
            .rsplit('/')
            .next()
            .is_some_and(|f| !f.contains('.'))
    {
        if let Some(idx) = normalized_path.find('/') {
            let first_segment = &normalized_path[..idx];
            if !first_segment.is_empty() && !skip_dirs.contains(&first_segment) {
                return first_segment.to_string();
            }
        }
    }

    if let Some(rest) = normalized_path.strip_prefix(normalized_root) {
        let rest = rest.trim_start_matches('/');
        if let Some(member) = rest.split('/').next() {
            if !member.is_empty() && !skip_dirs.contains(&member) {
                return member.to_string();
            }
            if skip_dirs.contains(&member) {
                let deeper = rest
                    .trim_start_matches('/')
                    .trim_start_matches(member)
                    .trim_start_matches('/');
                if let Some(real_member) = deeper.split('/').next()
                    && !real_member.is_empty()
                    && !skip_dirs.contains(&real_member)
                {
                    if real_member.contains('.')
                        && let Some(root_member) = normalized_root.rsplit('/').next()
                        && !root_member.is_empty()
                    {
                        return root_member.to_string();
                    }
                    return real_member.to_string();
                }
                if let Some(root_member) = normalized_root.rsplit('/').next()
                    && !root_member.is_empty()
                {
                    return root_member.to_string();
                }
            }
        }
    }
    for marker in &["crates", "modules", "packages"] {
        if let Some(idx) = normalized_path.find(marker) {
            let after = &normalized_path[idx + marker.len()..].trim_start_matches('/');
            if let Some(member) = after.split('/').next()
                && !member.is_empty()
                && !skip_dirs.contains(&member)
            {
                return member.to_string();
            }
        }
    }
    // Fallback: extract basename for root-level files
    std::path::Path::new(normalized_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| normalized_path.to_string())
}

/// Check if a path points to a recognized source file (not a directory).
fn is_source_file(path: &str) -> bool {
    path.ends_with(".rs")
        || path.ends_with(".py")
        || path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with(".js")
        || path.ends_with(".jsx")
}

/// Output violations in the requested format. `is_specific_member` controls compact vs detailed.
pub fn output_violations(
    violations: &[ViolationItem],
    target_path: &str,
    format: Format,
    is_specific_member: bool,
) {
    let force_member = if is_specific_member {
        let p = std::path::Path::new(target_path);
        p.file_name().map(|n| n.to_string_lossy().to_string())
    } else {
        None
    };
    let grouped = group_by_member(violations, target_path, force_member.as_deref());
    let is_single_file = is_source_file(target_path);
    match format {
        Format::Text => render_text(&grouped, target_path, is_specific_member, is_single_file),
        Format::Json => render_json(&grouped, violations, target_path),
        Format::Sarif => render_sarif(&grouped),
        Format::Junit => render_junit(&grouped),
    }
}

// ─── Text ───────────────────────────────────────────────────

fn render_text(
    grouped: &BTreeMap<String, Vec<&ViolationItem>>,
    target_path: &str,
    _is_specific_member: bool,
    _is_single_file: bool,
) {
    let ver = env!("CARGO_PKG_VERSION");
    println!("Lint Arwaky v{ver} — Scan Report");
    println!("Target: {target_path}");
    println!();

    let norm_target = target_path.trim_end_matches('/');
    let total: usize = grouped.values().map(|r| r.len()).sum();
    if total == 0 {
        println!("Total: 0 violations");
        return;
    }

    // {top} → [member] → (file): each level has its own bracket so a reader
    // can tell a member folder from a file by shape alone.
    let mut hierarchy: BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<&ViolationItem>>>> =
        BTreeMap::new();
    for results in grouped.values() {
        for r in results {
            let rel = make_relative(&r.file.value, norm_target);
            let (top, member, file) = hierarchy_key(&rel);
            hierarchy
                .entry(top)
                .or_default()
                .entry(member)
                .or_default()
                .entry(file)
                .or_default()
                .push(r);
        }
    }

    // {top} → [member] → (file): each level has its own bracket so a reader
    // can tell a member folder from a file by shape alone.
    for (top, members) in &hierarchy {
        println!("{{{top}}}");
        println!();
        for (member, files) in members {
            // An empty member means the finding sat directly under the member
            // dir, with no folder of its own to name.
            if !member.is_empty() {
                println!("[{member}]");
                println!();
            }
            for (file, violations) in files {
                for v in violations {
                    render_violation(file, v);
                }
            }
            println!();
        }
    }

    println!("Total: {total} violations");
    println!();
    render_suggestions(grouped);
}

/// Split a target-relative path into (top folder, member, file path).
///
/// Public because the bracket levels are the report's contract: a file
/// landing in the member slot prints `[report.md]`, which reads as a folder
/// that does not exist.
pub fn hierarchy_key(rel: &str) -> (String, String, String) {
    let segments: Vec<&str> = rel.split('/').filter(|s| !s.is_empty()).collect();
    match segments.len() {
        // No path at all: nothing to group under.
        0 => ("root".to_string(), "root".to_string(), String::new()),
        // A flat document at the target root (`FRD.md`) or a flat file inside a
        // member dir (`crates/root_violation.rs`). Neither has a member level,
        // so the member slot is empty and no `[heading]` prints. Filling it
        // with the file name would print `[FRD.md]`, a folder that is not there.
        1 => ("root".to_string(), String::new(), segments[0].to_string()),
        2 => (
            segments[0].to_string(),
            // `crates/shared_common` is a finding about the member folder;
            // `crates/root_violation.rs` is a finding about a file. A source
            // extension tells them apart, because a member dir has none.
            match is_file_segment(segments[1]) {
                true => String::new(),
                false => segments[1].to_string(),
            },
            segments[1].to_string(),
        ),
        // `<member-dir>/<member>/<file>` and deeper: the normal shape.
        _ => {
            let file = segments[2..].join("/");
            (segments[0].to_string(), segments[1].to_string(), file)
        }
    }
}

/// Whether a path segment names a source file rather than a folder.
///
/// Only used where a segment could be either. A member folder carries no
/// extension; a source file always carries one of these.
fn is_file_segment(segment: &str) -> bool {
    const SOURCE_EXTENSIONS: [&str; 11] = [
        "rs", "py", "ts", "tsx", "js", "jsx", "md", "toml", "json", "yaml", "yml",
    ];
    match segment.rsplit_once('.') {
        Some((stem, ext)) => !stem.is_empty() && SOURCE_EXTENSIONS.contains(&ext),
        None => false,
    }
}

/// One violation block: `(file:line[:col])`, `CODE:NAME`, `WHY: …`, `FIX: …`.
fn render_violation(file: &str, v: &ViolationItem) {
    println!(
        "({})",
        format_location(file, v.line.value(), v.column.value())
    );
    let name = if v.violation_name.is_empty() {
        String::new()
    } else {
        format!(":{}", v.violation_name)
    };
    println!("{}{name}", v.code.code());
    if !v.why.is_empty() {
        println!("WHY: {}", v.why);
    }
    if !v.fix.is_empty() {
        println!("FIX: {}", v.fix);
    }
    println!();
}

/// Bottom `Hint` section: the skills available, grouped by the layer whose
/// files were flagged.
///
/// A per-code lookup answered "which skill fixes AES101" by reading the first
/// file that code happened to appear in, so a naming violation in an agent
/// file pointed at the taxonomy skill. Grouping by layer answers the question
/// a reader actually has — this file is an agent file, what do I read — and
/// leaves the choice of which skill to them.
fn render_suggestions(grouped: &BTreeMap<String, Vec<&ViolationItem>>) {
    let mut layers: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for results in grouped.values() {
        for r in results {
            // A file outside the naming convention still groups under `other`,
            // so a tool violation on it does not vanish from the report.
            let layer = layer_of(&r.file.value).unwrap_or("other");
            layers
                .entry(layer.to_string())
                .or_default()
                .insert(r.code.code().to_string());
        }
    }
    if layers.is_empty() {
        return;
    }

    println!("Hint");
    println!("Skills, by the kind of file they govern:");
    println!();
    for (layer, codes) in &layers {
        println!("{layer}");
        println!("  codes: {}", join_sorted(codes));
        for skill in skills_for_layer(layer) {
            println!("  - lint-arwaky-cli skill read {skill}");
        }
        println!();
    }
}

/// The skills worth reading for a file in `layer`.
///
/// Each layer skill owns that layer's rules; `aes-docs` owns the markdown
/// documents wherever they sit, and `aes-lint-arwaky` covers the rules that
/// hold everywhere — naming, file length, duplication.
fn skills_for_layer(layer: &str) -> Vec<&'static str> {
    let mut skills: Vec<&'static str> = match layer {
        "taxonomy" => vec!["aes-taxonomy", "aes-contract"],
        "contract" => vec!["aes-contract", "aes-taxonomy"],
        "capabilities" => vec!["aes-capabilities"],
        "utility" => vec!["aes-utility"],
        "agent" => vec!["aes-agent"],
        "surfaces" => vec!["aes-surface"],
        "root" => vec!["aes-root"],
        _ => vec!["aes-lint-arwaky"],
    };
    // Every layer also sits under the cross-cutting rules and the doc rules.
    for shared in ["aes-lint-arwaky", "aes-docs"] {
        if !skills.contains(&shared) {
            skills.push(shared);
        }
    }
    skills
}

/// The layer a file belongs to, from its name prefix (the AES102 convention).
///
/// `None` for a file outside the convention — a tool violation on a file with
/// no layer prefix, for instance. The report groups those under `other` rather
/// than dropping them, so an external tool's codes stay visible.
fn layer_of(file_path: &str) -> Option<&'static str> {
    const PREFIXES: [(&str, &str); 8] = [
        ("taxonomy", "taxonomy"),
        ("contract", "contract"),
        ("capabilities", "capabilities"),
        ("utility", "utility"),
        ("agent", "agent"),
        ("surface", "surfaces"),
        ("root", "root"),
        ("cli_", "surface"),
    ];
    let filename = file_path.rsplit(['/', '\\']).next().unwrap_or(file_path);
    // A document is judged by the doc rules wherever it sits.
    if filename.ends_with(".md") {
        return Some("docs");
    }
    let stem = filename.split('.').next().unwrap_or(filename);
    PREFIXES
        .iter()
        .find(|(prefix, _)| stem.starts_with(prefix))
        .map(|(_, layer)| *layer)
}

/// Join a set in a stable order, without pulling in a dependency.
fn join_sorted(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

// ─── JSON ───────────────────────────────────────────────────

fn render_json(
    grouped: &BTreeMap<String, Vec<&ViolationItem>>,
    all_violations: &[ViolationItem],
    _target_path: &str,
) {
    let members: Vec<serde_json::Value> = grouped
        .iter()
        .map(|(name, results)| serde_json::json!({ "member": name, "violations": results.len() }))
        .collect();

    let mut file_to_member: std::collections::HashMap<String, &str> =
        std::collections::HashMap::new();
    for (name, items) in grouped {
        for v in items {
            file_to_member.insert(v.file.value.clone(), name.as_str());
        }
    }

    let results: Vec<serde_json::Value> = all_violations
        .iter()
        .map(|v| {
            let member = file_to_member
                .get(v.file.value.as_str())
                .copied()
                .unwrap_or(".");
            serde_json::json!({
                "code": v.code.code(),
                "file": v.file.value,
                "line": v.line.value(),
                "column": v.column.value(),
                "message": v.message.value,
                "severity": format!("{}", v.severity),
                "member": member,
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "target": _target_path,
            "total_violations": all_violations.len(),
            "members": members,
            "results": results,
        }))
        .unwrap_or_default()
    );
}

// ─── SARIF ──────────────────────────────────────────────────

fn render_sarif(grouped: &BTreeMap<String, Vec<&ViolationItem>>) {
    let ver = env!("CARGO_PKG_VERSION");
    let runs: Vec<serde_json::Value> = grouped
        .iter()
        .map(|(member_name, results)| {
            let items: Vec<serde_json::Value> = results
                .iter()
                .map(|v| {
                    let level = match v.severity_level() {
                        4 | 3 => "error",
                        2 => "warning",
                        _ => "note",
                    };
                    let mut location = serde_json::json!({
                        "physicalLocation": {
                            "artifactLocation": { "uri": v.file.value },
                        }
                    });
                    if v.line.value() > 0 {
                        let mut region = serde_json::json!({ "startLine": v.line.value() });
                        if v.column.value() > 0 {
                            region["startColumn"] = serde_json::json!(v.column.value());
                        }
                        location["physicalLocation"]["region"] = region;
                    }
                    serde_json::json!({
                        "ruleId": v.code.code(),
                        "level": level,
                        "message": { "text": v.message.value },
                        "locations": [location],
                    })
                })
                .collect();
            serde_json::json!({
                "tool": { "driver": { "name": member_name, "version": ver } },
                "results": items,
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "version": "2.1.0",
            "runs": runs,
        }))
        .unwrap_or_default()
    );
}

// ─── JUnit ──────────────────────────────────────────────────

fn render_junit(grouped: &BTreeMap<String, Vec<&ViolationItem>>) {
    println!(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    println!("<testsuites>");
    for (member_name, results) in grouped {
        let failures = results.len();
        println!("  <testsuite name=\"{member_name}\" tests=\"1\" failures=\"{failures}\">");
        if results.is_empty() {
            println!("    <testcase name=\"{member_name}\"/>");
        } else {
            println!("    <testcase name=\"{member_name}\">");
            for r in results {
                let escaped = r
                    .message
                    .value
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                let loc = if r.line.value() > 0 {
                    format!("{}:{}", r.file.value, r.line.value())
                } else {
                    r.file.value.clone()
                };
                println!(
                    "      <failure message=\"[{}] {}\">{}</failure>",
                    r.code.code(),
                    loc,
                    escaped
                );
            }
            println!("    </testcase>");
        }
        println!("  </testsuite>");
    }
    println!("</testsuites>");
}

// ─── Private helpers (UI-only) ──────────────────────────────

/// Render a file path as `<member-dir>/<rest>`, the shape the report groups by.
///
/// Every level above the member dir is dropped, so a scan of one member and a
/// scan of the whole workspace group the same file the same way. Without this
/// a scan of `packages/surface_layer_probe` reported its own folder name as
/// the top level and its files as members.
///
/// A path with no member dir in it - a flat document such as `FRD.md` - has no
/// member to hang off, so its file name is returned on its own.
fn make_relative(file_path: &str, target: &str) -> String {
    // Rules disagree on what a violation's `file` holds: the external adapters
    // and most rules emit an absolute path, while the doc and structure rules
    // emit one relative to the workspace root. Resolving the relative kind
    // against the scan target puts both in the same shape, so a file groups
    // under one heading no matter which rule reported it.
    let absolute = match std::path::Path::new(file_path).is_relative() {
        true => std::path::Path::new(target)
            .join(file_path)
            .to_string_lossy()
            .into_owned(),
        false => file_path.to_string(),
    };

    let (top, rest) = match split_at_member_dir(&absolute) {
        Some(parts) => parts,
        None => {
            return std::path::Path::new(&absolute)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(absolute);
        }
    };
    if rest.is_empty() {
        // The finding is about the member dir itself, which has no file level.
        return top;
    }
    format!("{top}/{rest}")
}

/// Cut a path at its member dir, returning `("packages", "surface/x.ts")`.
///
/// The member dirs are the level the report prints as `{top}`. A relative path
/// already starts there, so it is returned unchanged.
fn split_at_member_dir(file: &str) -> Option<(String, String)> {
    const MEMBER_DIRS: [&str; 3] = ["crates", "packages", "modules"];
    let path = std::path::Path::new(file);
    if path.is_relative() {
        let first = path
            .components()
            .next()?
            .as_os_str()
            .to_string_lossy()
            .into_owned();
        return MEMBER_DIRS.contains(&first.as_str()).then(|| {
            (
                first,
                path.to_string_lossy().trim_start_matches('/').to_string(),
            )
        });
    }
    let mut prefix = std::path::PathBuf::new();
    for part in path.components() {
        prefix.push(part);
        let name = part.as_os_str().to_string_lossy().into_owned();
        if MEMBER_DIRS.contains(&name.as_str()) {
            // Cut *after* this component: the member dir is the `{top}` level,
            // which the report prints on its own, so it must not reappear in
            // the member-and-file part behind it.
            let rest = path
                .strip_prefix(&prefix)
                .ok()?
                .to_string_lossy()
                .trim_start_matches('/')
                .to_string();
            return Some((name, rest));
        }
    }
    None
}

/// Status icon helper for doctor output (NO_COLOR aware).
pub fn status_icon(is_ok: bool) -> &'static str {
    if std::env::var_os("NO_COLOR").is_some() {
        if is_ok { "[OK]  " } else { "[FAIL]" }
    } else if is_ok {
        "✓"
    } else {
        "✗"
    }
}

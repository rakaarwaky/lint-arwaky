// PURPOSE: UtilityFormatting — Stateless output formatting for all CLI surface actions.
// Single source of truth for text/json/sarif/junit output format.
// Dispatcher returns data (Vec<ViolationItem> / report structs); this module renders it.
// Uses existing VOs from shared: ErrorCode, FilePath, LintMessage, Severity.

use std::collections::BTreeMap;

use shared_common::ViolationItem;
use shared_common::taxonomy_format_vo::Format;
use shared_common::taxonomy_skill_hint_vo::resolve_skill_hint_for_path as resolve_skill_hint_for_file;

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
            println!("[{member}]");
            println!();
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
        // A flat file sitting directly in the scan target (e.g. `FRD.md`).
        1 => (
            "root".to_string(),
            segments[0].to_string(),
            segments[0].to_string(),
        ),
        // `<member-dir>/<file>`: a file directly under `crates/`, `modules/`,
        // or `packages/` with no member dir in between. There is no member
        // level here, so printing `[agent_orphan_root_probe.rs]` under
        // `{crates}` would read as a folder that does not exist. The member
        // dir itself carries no violations of its own to report.
        2 => (
            segments[0].to_string(),
            segments[1].to_string(),
            segments[1].to_string(),
        ),
        // `<member-dir>/<member>/<file>` and deeper: the normal shape.
        _ => {
            let file = segments[2..].join("/");
            (segments[0].to_string(), segments[1].to_string(), file)
        }
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

/// Bottom `Hint` section: one line per distinct code, each hint printed once.
fn render_suggestions(grouped: &BTreeMap<String, Vec<&ViolationItem>>) {
    let mut samples: BTreeMap<String, &ViolationItem> = BTreeMap::new();
    for results in grouped.values() {
        for r in results {
            samples.entry(r.code.code().to_string()).or_insert(r);
        }
    }
    println!("Hint");
    for (code, sample) in &samples {
        let hint = resolve_skill_hint_for_file(code, &sample.file.value);
        println!("{code} → {}", hint.guidance());
    }
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

/// Make a file path relative to the workspace root.
fn make_relative(file_path: &str, target: &str) -> String {
    let canon_file = std::path::Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| file_path.to_string());
    let canon_target = std::path::Path::new(target)
        .canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| target.to_string());

    let workspace_root = find_common_workspace_root(&canon_file, &canon_target);

    if let Some(root) = &workspace_root {
        if let Some(rest) = canon_file.strip_prefix(root) {
            let rest = rest.trim_start_matches('/');
            if !rest.is_empty() {
                return rest.to_string();
            }
        }
    }

    if let Some(rest) = canon_file.strip_prefix(&canon_target) {
        let rest = rest.trim_start_matches('/');
        if !rest.is_empty() {
            return rest.to_string();
        }
    }

    // A relative path is already relative to the scan target. Resolving it
    // against the CWD would send it outside the workspace, and the fallback
    // below would reduce it to a bare filename — which then lands in the
    // member slot and prints as `[report.md]`, a folder that does not exist.
    let path = std::path::Path::new(file_path);
    if path.is_relative() && path.components().count() > 1 {
        return file_path.to_string();
    }

    std::path::Path::new(file_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| file_path.to_string())
}

/// Walk up from both paths to find the common workspace root.
fn find_common_workspace_root(path1: &str, path2: &str) -> Option<String> {
    let mut dirs1: Vec<std::path::PathBuf> = vec![];
    let mut p = std::path::PathBuf::from(path1);
    while let Some(parent) = p.parent() {
        let parent_path = parent.to_path_buf();
        dirs1.push(parent_path.clone());
        if parent_path.join("crates").is_dir()
            || parent_path.join("packages").is_dir()
            || parent_path.join("modules").is_dir()
        {
            break;
        }
        p = parent.to_path_buf();
    }

    let mut dirs2: Vec<std::path::PathBuf> = vec![];
    let mut p = std::path::PathBuf::from(path2);
    while let Some(parent) = p.parent() {
        let parent_path = parent.to_path_buf();
        dirs2.push(parent_path.clone());
        if parent_path.join("crates").is_dir()
            || parent_path.join("packages").is_dir()
            || parent_path.join("modules").is_dir()
        {
            break;
        }
        p = parent.to_path_buf();
    }

    dirs1
        .iter()
        .rev()
        .find(|d| dirs2.contains(d))
        .map(|p| p.to_string_lossy().to_string())
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

// PURPOSE: UtilityFormatting — Stateless output formatting for all CLI surface actions.
// Single source of truth for text/json/sarif/junit output format.
// Dispatcher returns data (Vec<ViolationItem> / report structs); this module renders it.
// Uses existing VOs from shared: ErrorCode, FilePath, LintMessage, Severity.

use std::collections::BTreeMap;

use shared::cli_commands::Format;
use shared::common::ViolationItem;

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
    ".".to_string()
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
    is_specific_member: bool,
    is_single_file: bool,
) {
    let ver = env!("CARGO_PKG_VERSION");
    println!("Lint Arwaky v{ver} — Scan Report");
    println!("Target: {target_path}");
    println!();

    let norm_target = target_path.trim_end_matches('/');

    let mut total = 0usize;
    for (member_name, results) in grouped {
        total += results.len();
        if results.is_empty() {
            continue;
        } else if is_single_file {
            println!("[{member_name}] — {} violations", results.len());
            println!();
            for r in results {
                let loc = format_location(&r.file.value, r.line.value(), r.column.value());
                println!("  {} [{}] {}", loc, r.code.code(), r.message.value);
            }
            println!();
        } else if is_specific_member {
            println!("[{member_name}] — violations by file");
            println!();

            let mut file_violations: BTreeMap<String, Vec<&&ViolationItem>> = BTreeMap::new();
            for r in results {
                let rel_path = make_relative(&r.file.value, norm_target);
                file_violations.entry(rel_path).or_default().push(r);
            }
            for (file_path, file_results) in &file_violations {
                println!("  {file_path}");
                for r in file_results {
                    let loc = format_location(&r.file.value, r.line.value(), r.column.value());
                    println!("    {} [{}] {}", loc, r.code.code(), r.message.value);
                }
            }
            println!();
        } else {
            let lang = lang_tag(&results[0].file.value);
            println!("[{lang}] {member_name} — {} violations", results.len());
            let mut code_counts: BTreeMap<String, usize> = BTreeMap::new();
            for r in results {
                *code_counts.entry(r.code.code().to_string()).or_insert(0) += 1;
            }
            for (code, count) in &code_counts {
                println!("  [{code}] {count}");
            }
            println!();
        }
    }

    println!("Total: {total} violations");

    if !is_specific_member {
        println!();
        println!("Tip: Scan specific feature folder for detailed violations:");
        println!("  lint-arwaky-cli scan <member-path>");
        println!("  lint-arwaky-cli scan <root> --member <member-name>");
    } else if !is_single_file {
        println!();
        println!("Tip: Scan specific file for focused output:");
        println!("  lint-arwaky-cli scan <file-path>");
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

fn lang_tag(path: &str) -> &str {
    if path.ends_with(".rs") {
        "rust"
    } else if path.ends_with(".py") {
        "python"
    } else if path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with(".js")
        || path.ends_with(".jsx")
    {
        "typescript"
    } else {
        "unknown"
    }
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

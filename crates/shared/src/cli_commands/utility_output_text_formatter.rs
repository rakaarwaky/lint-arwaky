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
    is_specific_member: bool,
    is_single_file: bool,
) {
    let ver = env!("CARGO_PKG_VERSION");
    println!("Lint Arwaky v{ver} — Scan Report");
    println!("Target: {target_path}");
    println!();

    let norm_target = target_path.trim_end_matches('/');
    let is_global = !is_specific_member && !is_single_file;

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
                let hint = resolve_skill_hint_for_file(r.code.code(), &r.file.value);
                println!("  {} [{}] {}", loc, r.code.code(), r.message.value);
                println!("    ↳ {}", hint.guidance());
                for wf in why_fix_lines(&r.message.value, 4) {
                    println!("{wf}");
                }
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
                    let hint = resolve_skill_hint_for_file(r.code.code(), &r.file.value);
                    println!("    {} [{}] {}", loc, r.code.code(), r.message.value);
                    println!("      ↳ {}", hint.guidance());
                    for wf in why_fix_lines(&r.message.value, 6) {
                        println!("{wf}");
                    }
                }
            }
            println!();
        } else {
            let lang = lang_tag(&results[0].file.value);
            println!("[{lang}] {member_name} — {} violations", results.len());
            println!();
            for line in code_summary_lines(results) {
                println!("{line}");
            }
            // Per-file blocks with per-violation WHY/FIX
            render_member_file_blocks(results, norm_target);
        }
    }

    if is_global && total > 0 {
        render_by_folder_rollup(grouped, total);
    } else {
        println!("Total: {total} violations");
    }

    if !is_specific_member {
        println!();
        if is_global {
            println!("Tip: Scan a folder for per-file WHY/FIX detail:");
            println!("  lint-arwaky-cli scan <folder>");
        } else {
            println!("Tip: Scan a file for focused output:");
            println!("  lint-arwaky-cli scan <file-path>");
        }
    }
}

/// "By folder" rollup for global `.` / `check .` scans.
/// One line per top-level folder (crates/, packages/, modules/, src/) with
/// total count + top-3 codes. No per-file, no WHY/FIX — summary only.
fn render_by_folder_rollup(grouped: &BTreeMap<String, Vec<&ViolationItem>>, total: usize) {
    // Group members into top-level folders
    let mut folder_totals: BTreeMap<String, usize> = BTreeMap::new();
    let mut folder_codes: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();

    for (member, results) in grouped {
        if results.is_empty() {
            continue;
        }
        // Determine top-level folder: the first path segment relative to scan root
        let first_file = &results[0].file.value;
        let norm = first_file.trim_start_matches("./");
        let top_folder = norm
            .split('/')
            .next()
            .filter(|s| {
                !s.ends_with(".rs")
                    && !s.ends_with(".py")
                    && !s.ends_with(".ts")
                    && !s.ends_with(".md")
            })
            .unwrap_or("root");

        let key = if top_folder.is_empty() {
            "root".to_string()
        } else {
            format!("{top_folder}/")
        };

        *folder_totals.entry(key.clone()).or_insert(0) += results.len();
        let code_map = folder_codes.entry(key.clone()).or_default();
        for r in results {
            let code = r.code.code().to_string();
            *code_map.entry(code).or_insert(0) += 1;
        }

        // Member name is used for grouping but folder key is what matters here
        let _ = member;
    }

    println!("Total: {total} violations");
    println!();
    println!("By folder:");
    for (folder, count) in &folder_totals {
        let top3: Vec<String> = folder_codes
            .get(folder)
            .map(|m| {
                let mut entries: Vec<_> = m.iter().collect();
                entries.sort_by(|a, b| b.1.cmp(a.1));
                entries
                    .iter()
                    .take(3)
                    .map(|(c, n)| format!("{c} ({n})"))
                    .collect()
            })
            .unwrap_or_default();
        let codes_str = top3.join(", ");
        println!("  {folder:<12} {count:>4} violations  ({codes_str})");
    }
    println!();
    println!("Run `lint-arwaky-cli scan <folder>` for detail.");
}

/// Per-file blocks under a member for folder scans (mode 2).
/// Groups violations by file, prints `FILE — N violations` header,
/// then per-violation WHY/FIX lines.
fn render_member_file_blocks(results: &[&ViolationItem], norm_target: &str) {
    let mut file_violations: BTreeMap<String, Vec<&&ViolationItem>> = BTreeMap::new();
    for r in results {
        let rel_path = make_relative(&r.file.value, norm_target);
        file_violations.entry(rel_path).or_default().push(r);
    }
    println!();
    for (file_path, file_results) in &file_violations {
        println!("  {file_path} — {} violations", file_results.len());
        for r in file_results {
            let code = r.code.code();
            println!(
                "    {}: [{code}] {}",
                r.line.value(),
                r.message.value.lines().next().unwrap_or("")
            );
            for wf in why_fix_lines(&r.message.value, 4) {
                println!("{wf}");
            }
        }
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
    } else if path.ends_with(".md") {
        "markdown"
    } else if path.ends_with(".yaml") || path.ends_with(".yml") {
        "yaml"
    } else if path.ends_with(".toml") {
        "toml"
    } else if path.ends_with(".json") {
        "json"
    } else if path.ends_with(".go") {
        "go"
    } else if path.ends_with(".java") {
        "java"
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

/// Extract WHY and FIX lines from a `LintMessage` value.
///
/// Message formats:
/// - AES: `CODE DESC.\nWHY? …\nHOW TO FIX? …` (or `FIX:`)
/// - External: short single-line, no WHY/FIX
///
/// Returns `(Option<why>, Option<fix>)` — `None` when the message has no
/// embedded WHY/FIX. Never invents text.
pub fn extract_why_fix(msg: &str) -> (Option<&str>, Option<&str>) {
    let mut why: Option<&str> = None;
    let mut fix: Option<&str> = None;
    for line in msg.lines() {
        let trimmed = line.trim();
        if let Some(w) = trimmed
            .strip_prefix("WHY?")
            .or_else(|| trimmed.strip_prefix("WHY:"))
        {
            why = Some(w.trim());
        } else if let Some(f) = trimmed
            .strip_prefix("HOW TO FIX?")
            .or_else(|| trimmed.strip_prefix("FIX:"))
        {
            fix = Some(f.trim());
        }
    }
    (why, fix)
}

/// Render WHY/FIX sub-lines for a violation, indented to `indent` spaces.
/// Skips lines entirely when the message has no embedded WHY/FIX.
fn why_fix_lines(msg: &str, indent: usize) -> Vec<String> {
    let (why, fix) = extract_why_fix(msg);
    let mut out = Vec::new();
    let prefix = " ".repeat(indent);
    if let Some(w) = why {
        out.push(format!("{prefix}WHY: {w}"));
    }
    if let Some(f) = fix {
        out.push(format!("{prefix}FIX: {f}"));
    }
    out
}

/// One `  [CODE] N  ← guidance` line per distinct code, sorted by code.
///
/// Grouping by code rather than listing every violation is what keeps a member
/// that trips 40 rules to 40 lines instead of 400, and the count beside each
/// code is the first thing a reader scans for. CI parses this exact shape to
/// confirm the external adapters ran end-to-end, so it is a contract: a change
/// to this format has to change `.github/workflows/ci.yml` in the same commit.
pub fn code_summary_lines(results: &[&ViolationItem]) -> Vec<String> {
    let mut code_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut code_examples: BTreeMap<String, String> = BTreeMap::new();
    for r in results {
        let code = r.code.code().to_string();
        *code_counts.entry(code.clone()).or_insert(0) += 1;
        code_examples
            .entry(code)
            .or_insert_with(|| r.file.value.clone());
    }
    code_counts
        .iter()
        .map(|(code, count)| {
            let example_file = code_examples.get(code).map(|s| s.as_str()).unwrap_or("");
            let hint = resolve_skill_hint_for_file(code, example_file);
            format!("  [{code}] {count}  ← {}", hint.guidance())
        })
        .collect()
}

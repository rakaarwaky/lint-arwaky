// PURPOSE: SurfaceOutputComponent — UI-only output formatting for all surface actions.
// Single source of truth for text/json/sarif/junit output format.
// Zero business logic, zero utility functions — pure rendering.
// Uses existing VOs from shared: ErrorCode, FilePath, LintMessage, Severity.

use std::collections::BTreeMap;

use shared::cli_commands::{
    Format,
    LintResult,
};

use shared::common::{
    ColumnNumber,
    LineNumber,
    ErrorCode,
    LintMessage,
    FilePath,
    Severity,
};


/// Minimal violation item for display. Uses existing VOs — no duplicate String wrappers.
#[derive(Debug, Clone)]
pub struct ViolationItem {
    pub code: ErrorCode,
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub message: LintMessage,
    pub severity: Severity,
}

impl ViolationItem {
    pub fn from_lint_result(r: &LintResult) -> Self {
        Self {
            code: r.code.clone(),
            file: r.file.clone(),
            line: r.line.clone(),
            column: r.column.clone(),
            message: r.message.clone(),
            severity: r.severity.clone(),
        }
    }

    pub fn from_json_obj(item: &serde_json::Value) -> Option<Self> {
        Some(Self {
            code: ErrorCode::raw(item.get("code")?.as_str()?),
            file: FilePath::new(item.get("file")?.as_str()?.to_string()).ok()?,
            line: LineNumber::new(item.get("line").and_then(|v| v.as_i64()).unwrap_or(0)),
            column: ColumnNumber::new(item.get("column").and_then(|v| v.as_i64()).unwrap_or(0)),
            message: LintMessage::new(item.get("message")?.as_str()?),
            severity: parse_severity(
                item.get("severity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("INFO"),
            ),
        })
    }

    fn severity_level(&self) -> u8 {
        match self.severity {
            Severity::CRITICAL => 4,
            Severity::HIGH => 3,
            Severity::MEDIUM => 2,
            Severity::LOW => 1,
            Severity::INFO => 0,
        }
    }
}

fn parse_severity(s: &str) -> Severity {
    match s.to_uppercase().as_str() {
        "CRITICAL" => Severity::CRITICAL,
        "HIGH" => Severity::HIGH,
        "MEDIUM" => Severity::MEDIUM,
        "LOW" => Severity::LOW,
        _ => Severity::INFO,
    }
}

/// Group violations by workspace member name extracted from file paths.
/// If `force_member` is Some, all violations are grouped under that name (for member scans).
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
            shared::cli_commands::utility_path_resolver::extract_member_from_path(
                &v.file.value,
                root,
            )
        };
        grouped.entry(member).or_default().push(v);
    }
    grouped
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
    // When scanning a specific member, extract the member name from the path
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

    let mut total = 0usize;
    for (member_name, results) in grouped {
        total += results.len();
        if results.is_empty() {
            continue;
        } else if is_single_file {
            // ── Mode 3: Single file scan — show ALL violations WITH messages, no tip ──
            println!("[{member_name}] — {} violations", results.len());
            println!();
            for r in results {
                let loc = match (r.line.value(), r.column.value()) {
                    (l, c) if l > 0 && c > 0 => format!("{}:{}:{}", r.file.value, l, c),
                    (l, _) if l > 0 => format!("{}:{}", r.file.value, l),
                    _ => r.file.value.clone(),
                };
                println!("  {} [{}] {}", loc, r.code.code(), r.message.value);
            }
            println!();
        } else if is_specific_member {
            // ── Mode 2: Feature folder scan — per-file violations WITHOUT messages, with file tip ──
            println!("[{member_name}] — violations by file");
            println!();

            // Group violations by file path
            let mut file_violations: BTreeMap<String, Vec<&&ViolationItem>> = BTreeMap::new();
            for r in results {
                file_violations
                    .entry(r.file.value.clone())
                    .or_default()
                    .push(r);
            }
            for (file_path, file_results) in &file_violations {
                println!("  {file_path}:");
                for r in file_results {
                    let loc = match (r.line.value(), r.column.value()) {
                        (l, c) if l > 0 && c > 0 => format!("line:{}:{}", l, c),
                        (l, _) if l > 0 => format!("line:{}", l),
                        _ => String::new(),
                    };
                    if loc.is_empty() {
                        println!("    [{}]", r.code.code());
                    } else {
                        println!("    [{}] {}", r.code.code(), loc);
                    }
                }
            }
            println!();
        } else {
            // ── Mode 1: Workspace root scan — per-member AES codes + counts, with member tip ──
            let lang = lang_tag(&results[0].file.value);
            println!("[{lang}] {member_name} — {} violations", results.len());
            // Group by AES code, show count per code
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
        // Mode 1 tip: scan specific feature folder
        println!();
        println!("Tip: Scan specific feature folder for file-level violations:");
        println!("  lint-arwaky-cli scan <member-path>");
        println!("  lint-arwaky-cli scan <root> --member <member-name>");
    } else if !is_single_file {
        // Mode 2 tip: scan specific file for detailed violations
        println!();
        println!("Tip: Scan a specific file for detailed violations:");
        println!("  lint-arwaky-cli scan <file-path>");
    }
    // Mode 3 (single file): no scan tip
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

    // Build reverse lookup: file path → member name from grouped data
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

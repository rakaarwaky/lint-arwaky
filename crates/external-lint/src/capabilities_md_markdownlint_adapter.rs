// PURPOSE: MarkdownLintAdapter — ILinterAdapterProtocol implementation for markdownlint
//
// Runs markdownlint over Markdown files and normalizes the findings into the
// unified `LintResult` shape with tool-native rule codes (`markdownlint::MD041`).
//
// Two CLI variants exist and each reports differently:
//   - `markdownlint-cli` emits a JSON array of issues with `--json`
//   - `markdownlint-cli2` always writes text lines of the form
//     `<file>:<line>[:<col>] error <RULE>/<alias> <description> [Context: "…"]`
// so the adapter probes for the JSON-capable binary first and falls back to
// parsing the text format when only the v2 CLI is installed.
//
// Key handling:
//   - Early-returns empty results for files that are not Markdown
//   - Resolves the working directory by walking up for the nearest project root
//   - Non-JSON, non-matching output yields empty results, never a panic

use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_common::{ComplianceStatus, LintMessage};
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_external_lint_constant::{
    MARKDOWNLINT_CLI_VARIANTS, MARKDOWNLINT_EXTENSIONS, MARKDOWNLINT_FIX_FLAG,
};
use shared_external_lint::utility_path_normalization::{
    resolve_or_fallback, resolve_or_fallback_with_context,
};
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MarkdownLintAdapter {
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    js_resolution: Arc<dyn IJsToolResolutionProtocol>,
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for MarkdownLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("markdownlint")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.io.is_file(Path::new(path_str)) && !is_markdown_file(path_str) {
            return Ok(LintResultList::default());
        }

        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path = self.io.canonicalize_path_str(path);

        // Directory targets: hand the tool a Markdown-only glob. Passing the
        // bare directory makes markdownlint walk *every* file under it and
        // report bogus rule violations against `.rs`/`.toml` sources (MD041
        // "first line should be a top-level heading" on a Rust file), which
        // pollutes the report with findings that are not markdownlint's to
        // make. Mirrors the prettier adapter's JS/TS glob narrowing.
        let is_dir = self.io.is_dir(Path::new(&abs_path.value));
        for (binary, args) in tool_invocations(&abs_path.value, is_dir) {
            // JS tool resolution prefers local `node_modules/.bin/`. For
            // tools that are typically installed globally (like
            // markdownlint-cli2), fall back to a direct PATH lookup when the
            // local resolution returns `None`.
            let cmd = self
                .js_resolution
                .resolve_js_cmd(&ToolName::new(binary), args.clone(), &wd)
                .or_else(|| {
                    if self
                        .tool_resolution
                        .is_executable_in_path(&ToolName::new(binary))
                    {
                        let mut cmd = vec![binary.to_string()];
                        cmd.extend(args);
                        Some(cmd)
                    } else {
                        None
                    }
                });
            let Some(cmd) = cmd else {
                continue;
            };
            // markdownlint-cli2 prints paths relative to its CWD, and the CWD
            // must be a *directory* — `Command::current_dir` on a file fails to
            // spawn. For a directory target that is the scan root itself; for a
            // single-file target (`scan README.md`) it is the file's parent.
            // Either way the tool's relative output resolves back onto the same
            // absolute path the rest of the report uses.
            let Ok(response) = self.lint_executor.exec_cmd_scan(
                cmd,
                exec_working_dir(&abs_path, is_dir),
                60.0,
                Some(self.name()),
                path,
            ) else {
                continue;
            };

            let results = normalize_output(&response.stdout, &response.stderr, path);
            if !results.values.is_empty() {
                return Ok(results);
            }
        }

        Ok(LintResultList::default())
    }

    fn fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        // Both CLI variants spell the autofix flag `--fix`, so probe them in
        // the same order `scan` does. Returning on the first resolvable binary
        // keeps a host that only has `markdownlint-cli2` installed — the common
        // case — from silently reporting a no-op fix.
        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path = self.io.canonicalize_path_str(path);
        let is_dir = self.io.is_dir(Path::new(&abs_path.value));
        let target = markdown_target(&abs_path.value, is_dir);
        for binary in MARKDOWNLINT_CLI_VARIANTS {
            let args = vec![target.clone(), MARKDOWNLINT_FIX_FLAG.to_string()];
            let cmd = self
                .js_resolution
                .resolve_js_cmd(&ToolName::new(binary), args.clone(), &wd)
                .or_else(|| {
                    self.tool_resolution
                        .is_executable_in_path(&ToolName::new(binary))
                        .then(|| {
                            let mut cmd = vec![binary.to_string()];
                            cmd.extend(args);
                            cmd
                        })
                });
            let Some(cmd) = cmd else {
                continue;
            };
            // Run from the same directory `scan` would, so the glob resolves
            // identically and a file target does not fail to spawn.
            let response = self.lint_executor.exec_cmd_adapter(
                cmd,
                exec_working_dir(&abs_path, is_dir),
                60.0,
                self.name(),
            )?;
            return Ok(ComplianceStatus::new(response.returncode == 0));
        }
        // Neither CLI is installed — a no-op status, matching every other
        // adapter's "tool unresolvable" contract.
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl MarkdownLintAdapter {
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        js_resolution: Arc<dyn IJsToolResolutionProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            js_resolution,
            io,
            tool_resolution,
        }
    }
}

/// Markdown extensions markdownlint lints by default.
fn is_markdown_file(path: &str) -> bool {
    MARKDOWNLINT_EXTENSIONS
        .iter()
        .any(|ext| path.ends_with(&format!(".{ext}")))
}

/// The Markdown-only path or glob to hand the CLI for `abs_path`. A directory
/// target becomes a glob so neither variant lints — or autofixes — a `.rs` or
/// `.toml` file as if it were prose.
fn markdown_target(abs_path: &str, is_dir: bool) -> String {
    if is_dir {
        format!("{}/**/*.{{md,markdown}}", abs_path.trim_end_matches('/'))
    } else {
        abs_path.to_string()
    }
}

/// The directory to run the CLI from. `markdownlint-cli2` resolves its glob
/// and prints paths relative to the CWD, and spawning with a *file* as the
/// working directory fails outright — so a single-file target runs from its
/// parent, a directory target from itself.
fn exec_working_dir(abs_path: &FilePath, is_dir: bool) -> FilePath {
    if is_dir {
        return abs_path.clone();
    }
    Path::new(&abs_path.value)
        .parent()
        .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
        .unwrap_or_else(|| abs_path.clone())
}

/// The scan invocation per CLI variant, in probe order: `markdownlint-cli`
/// asks for JSON, `markdownlint-cli2` only writes text. Directory targets get
/// the Markdown-only glob (see `markdown_target`).
fn tool_invocations(abs_path: &str, is_dir: bool) -> Vec<(&'static str, Vec<String>)> {
    let target = markdown_target(abs_path, is_dir);
    vec![
        (
            MARKDOWNLINT_CLI_VARIANTS[0],
            vec!["--json".to_string(), target.clone()],
        ),
        (MARKDOWNLINT_CLI_VARIANTS[1], vec![target]),
    ]
}

/// Parse whichever format the installed CLI produced. JSON is tried first; the
/// text form is the fallback for `markdownlint-cli2`.
fn normalize_output(stdout: &str, stderr: &str, root: &FilePath) -> LintResultList {
    let trimmed = stdout.trim();
    if !trimmed.is_empty() && trimmed.starts_with('[') {
        return parse_json_issues(trimmed, root);
    }
    parse_text_issues(&format!("{}\n{}", stdout, stderr), root)
}

/// markdownlint-cli `--json`: an array of issue objects keyed `fileName`,
/// `lineNumber`, `ruleNames`, and `ruleDescription`.
fn parse_json_issues(raw: &str, root: &FilePath) -> LintResultList {
    let Ok(serde_json::Value::Array(entries)) = serde_json::from_str(raw) else {
        return LintResultList::default();
    };

    let mut results = Vec::new();
    for entry in entries {
        let file = entry
            .get("fileName")
            .and_then(|f| f.as_str())
            .unwrap_or_default();
        let line = entry
            .get("lineNumber")
            .and_then(|l| l.as_i64())
            .unwrap_or(1);
        let code = rule_code(&entry);
        let message = issue_message(
            entry
                .get("ruleDescription")
                .and_then(|m| m.as_str())
                .unwrap_or("markdownlint violation"),
            entry.get("errorContext").and_then(|c| c.as_str()),
        );
        results.push(build_result(
            canonicalize_against(root, file),
            line,
            1,
            code,
            message,
            root,
        ));
    }
    LintResultList::new(results)
}

/// markdownlint-cli2 text form:
/// `bad.md:1:1 error MD018/no-missing-space-atx No space after hash [Context: "#Title"]`
/// A missing column is legal, so the column segment is optional. Paths reported
/// by the CLI may be relative to the CWD; they are canonicalized against the
/// scan root so file paths remain consistent with the rest of the lint output.
fn parse_text_issues(raw: &str, root: &FilePath) -> LintResultList {
    let mut results = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((location, rest)) = trimmed.split_once(" error ") else {
            continue;
        };
        let Some((file, line_no, column)) = split_location(location) else {
            continue;
        };

        let Some((rule, description)) = rest.split_once(' ') else {
            continue;
        };
        let code = rule_code_from_text(rule);
        let message = strip_trailing_context(description);

        results.push(build_result(
            canonicalize_against(root, file),
            line_no,
            column,
            code,
            message,
            root,
        ));
    }

    LintResultList::new(results)
}

fn build_result(
    file: FilePath,
    line: i64,
    column: i64,
    code: String,
    message: String,
    root: &FilePath,
) -> LintResult {
    let file_vo = resolve_or_fallback_with_context(&file.value, file.clone(), Some(root.clone()));
    LintResult {
        file: file_vo,
        line: LineNumber::new(line),
        column: ColumnNumber::new(column),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message),
        source: Some(AdapterName::raw("markdownlint")),
        severity: Severity::MEDIUM,
        enclosing_scope: Default::default(),
        related_locations: LocationList::new(),
        violation_name: String::new(),
        why: String::new(),
        fix: String::new(),
    }
}

/// Append the tool's context snippet to the description when it has one.
fn issue_message(description: &str, context: Option<&str>) -> String {
    match context.filter(|c| !c.is_empty()) {
        Some(c) => format!("{} [Context: {:?}]", description, c),
        None => description.to_string(),
    }
}

/// Drop the trailing `[Context: "…"]` the text format appends, so the message
/// matches the JSON format's description alone.
fn strip_trailing_context(description: &str) -> String {
    match description.find(" [Context: ") {
        Some(idx) => description[..idx].trim_end().to_string(),
        None => description.to_string(),
    }
}

/// Parse a markdownlint-cli2 location segment of the form
/// `<file>:<line>[:<column>]`. The column is optional — some rules report
/// only the line number — and the file path itself may contain colons, so we
/// probe from the right: the last colon-separated segment must be numeric.
/// If the segment before that is also numeric, the trailing number is the
/// column; otherwise the first numeric segment is the line and the rest is
/// the file.
fn split_location(loc: &str) -> Option<(&str, i64, i64)> {
    let (head, last) = loc.rsplit_once(':')?;
    let last_num: i64 = last.parse().ok()?;

    match head.rsplit_once(':') {
        Some((file, line_s)) => {
            let line: i64 = line_s.parse().ok()?;
            (!file.is_empty()).then_some((file, line, last_num))
        }
        None => {
            // `loc` is `file:number` with no column segment — nothing after
            // the last colon to re-split on, so `last` itself is the line.
            (!head.is_empty()).then_some((head, last_num, 1))
        }
    }
}

/// Turn a tool-reported path into an absolute path rooted at `root`. Both CLI
/// variants report the file the way they resolved it — `markdownlint-cli` in
/// JSON mode echoes the argument it was given (already absolute), while
/// `markdownlint-cli2` prints paths relative to its working directory. Joining
/// a relative path onto `root` is what makes both land on the same absolute
/// path the rest of the report uses.
fn canonicalize_against(root: &FilePath, file: &str) -> FilePath {
    let candidate = std::path::Path::new(file);
    if candidate.is_absolute() {
        return resolve_or_fallback(file, root.clone());
    }
    resolve_or_fallback(
        std::path::Path::new(&root.value)
            .join(candidate)
            .to_string_lossy()
            .as_ref(),
        root.clone(),
    )
}

/// Build the tool-native rule code from markdownlint's `ruleNames` array. The
/// first entry is the `MD###` id, which is stable across markdownlint versions.
fn rule_code(entry: &serde_json::Value) -> String {
    entry
        .get("ruleNames")
        .and_then(|n| n.as_array())
        .and_then(|names| names.first())
        .and_then(|id| id.as_str())
        .map(|id| format!("markdownlint::{}", id))
        .unwrap_or_else(|| "markdownlint::unknown".to_string())
}

/// Extract the tool-native rule code from a markdownlint-cli2 rule token of
/// the form `MD018/no-missing-space-atx` (the aliases after the first slash
/// are ignored): `markdownlint::MD018`.
fn rule_code_from_text(rule: &str) -> String {
    rule.split('/')
        .next()
        .filter(|id| !id.is_empty())
        .map(|id| format!("markdownlint::{}", id))
        .unwrap_or_else(|| "markdownlint::unknown".to_string())
}

// PURPOSE: Stateless helper functions for the maintenance feature.
// Pure-string and taxonomy-only — no contract or cross-utility imports.

use crate::taxonomy_maintenance_vo::{ToolStatus, ToolchainDiagnostics};
use std::process::Command;

/// Accept only a plain version string: optional `v` prefix, dot-separated
/// numeric components, optional `-prerelease` suffix.
pub fn is_valid_tag(tag: &str) -> bool {
    let body = tag.strip_prefix('v').unwrap_or(tag);
    let (core, suffix) = match body.split_once('-') {
        Some((core, suffix)) => (core, Some(suffix)),
        None => (body, None),
    };
    let parts: Vec<&str> = core.split('.').collect();
    if parts.is_empty() || parts.len() > 4 {
        return false;
    }
    let core_ok = parts
        .iter()
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()));
    let suffix_ok = suffix
        .is_none_or(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.'));
    core_ok && suffix_ok
}

/// Strip a leading `v` so tags and CARGO_PKG_VERSION compare as dotted numbers.
pub fn normalize_version(tag: &str) -> String {
    tag.trim_start_matches('v').to_string()
}

/// Returns true when `candidate` is a strictly newer dotted version than `current`.
pub fn is_newer_version(candidate: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u64> {
        v.split('.')
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let (mut cand, mut cur) = (parse(candidate), parse(current));
    let width = cand.len().max(cur.len());
    cand.resize(width, 0);
    cur.resize(width, 0);
    cand > cur
}

/// Assemble the standard toolchain diagnostics: rustc, cargo, clippy, rustfmt,
/// python3, ruff, mypy, node, eslint, git, plus the current binary path.
pub fn build_toolchain_diagnostics() -> ToolchainDiagnostics {
    let rust_tools = vec![
        check_tool("rustc", &["--version"], true),
        check_tool("cargo", &["--version"], true),
    ];
    let mut clippy_status = check_tool("cargo", &["clippy", "--version"], true);
    clippy_status.name = "clippy".to_string();
    let rust_tools = [rust_tools, vec![clippy_status]].concat();
    let rust_tools = [
        rust_tools.clone(),
        vec![check_tool("rustfmt", &["--version"], true)],
    ]
    .concat();
    let python_tools = vec![
        check_tool("python3", &["--version"], false),
        check_tool("ruff", &["--version"], false),
        check_tool("mypy", &["--version"], false),
    ];
    let mut js_tools = vec![check_tool("node", &["--version"], false)];
    js_tools.push(check_tool("eslint", &["--version"], false));
    let vcs_tools = vec![check_tool("git", &["--version"], true)];
    // Read-only display of the running binary path in the doctor report; never
    // used to resolve or exec anything, so there is no untrusted-path exposure.
    // nosemgrep: rust.lang.security.current-exe.current-exe
    let binary_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    ToolchainDiagnostics {
        rust_tools,
        python_tools,
        js_tools,
        vcs_tools,
        binary_path,
    }
}

/// Execute a tool command and return its status.
/// Inlined here to avoid cross-utility imports (AES201).
pub fn check_tool(name: &str, args: &[&str], required: bool) -> ToolStatus {
    let mut cmd = Command::new(name);
    cmd.args(args).current_dir(".");
    let output = cmd.output();
    let (stdout, success) = match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            o.status.success(),
        ),
        Err(_) => (String::new(), false),
    };
    let (status, version) = if success {
        let ver = stdout.lines().next().unwrap_or("").trim().to_string();
        ("OK".to_string(), ver)
    } else if required {
        ("FAIL".to_string(), "NOT FOUND".to_string())
    } else {
        ("WARN".to_string(), "NOT FOUND".to_string())
    };
    ToolStatus {
        name: name.to_string(),
        status,
        version,
    }
}

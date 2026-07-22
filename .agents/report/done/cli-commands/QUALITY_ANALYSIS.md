Below is a QA/debugging and optimization review of the uploaded `cli-commands` module and the directly related `shared` helpers it depends on.

I focused only on:

- functional bugs in existing behavior,
- incorrect edge-case handling,
- output correctness,
- performance bottlenecks in existing flows,
- safe fixes that do **not** add new product features.

---

# 1. Summary of Findings

| ID       | Severity | Area                                                | Issue                                                                                                                                                                     |
| -------- | -------: | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| BUG-001  | Critical | `shared/common/utility_file.rs`                     | Directory traversal is broken on non-Unix platforms because`get_inode()` always returns `0`, causing the walker to stop visiting directories after the first one.         |
| BUG-002  | Critical | `surface_check_command.rs`                          | Multi-workspace`scan` prints human-readable banners before JSON/SARIF/JUnit output, corrupting structured output.                                                         |
| BUG-003  |     High | `surface_check_command.rs`                          | Path filtering uses string`starts_with`, which can match unrelated paths, e.g. `/repo/foo` matching `/repo/foobar`.                                                       |
| BUG-004  |     High | `surface_check_command.rs`                          | In`scan_with_discovery`, if workspace canonicalization fails, filtering falls back to `true`, duplicating all results across workspaces.                                  |
| BUG-005  |     High | `surface_check_action.rs`, `surface_git_command.rs` | `check --git-diff` ignores the user-provided path and also ignores the global `--filter` argument.                                                                        |
| BUG-006  |     High | `surface_plugin_command.rs`                         | `adapters` command prints a hardcoded list and ignores the injected `IExternalLintAggregate`, so it does not show actual active adapters.                                 |
| BUG-007  |   Medium | `surface_common_command.rs`                         | CI threshold comparison casts float score to`u32`, causing precision loss.                                                                                                |
| BUG-008  |   Medium | `surface_check_command.rs`                          | `xml_escape` appears to escape to the same character instead of XML entities, producing invalid JUnit XML.                                                                |
| BUG-009  |   Medium | `surface_check_command.rs`                          | Naming/import audit errors are silently discarded with`unwrap_or_default()`.                                                                                              |
| BUG-010  |   Medium | `surface_check_command.rs`                          | `scan --member` uses substring matching, so `--member sha` can incorrectly match `shared`.                                                                                |
| BUG-011  |      Low | `surface_maintenance_command.rs`                    | `dependencies` returns success even when dependency reporting fails.                                                                                                      |
| PERF-001 | Critical | `surface_check_command.rs`                          | `scan_with_discovery` builds the orphan graph using `collect_all_source_files_raw`, which does not apply ignore rules and can scan `node_modules`, `target`, `.git`, etc. |
| PERF-002 |   Medium | `shared/common/utility_file.rs`                     | Default ignore list misses common expensive directories such as`.git`, `dist`, `build`, `coverage`, `.venv`.                                                              |
| PERF-003 |   Medium | `root_cli_container.rs` / binary entrypoint         | `CliContainer::new_default()` eagerly builds all subsystems even for commands that do not need them, e.g. `version`, `adapters`.                                          |
| PERF-004 |      Low | `shared/common/utility_file.rs`                     | Ignore-pattern matching repeatedly splits paths and patterns during traversal.                                                                                            |
| PERF-005 |      Low | `surface_git_command.rs`                            | Git-diff mode analyzes changed files sequentially.                                                                                                                        |

---

# 2. Critical Functional Fixes

---

## BUG-001 — File walker broken on non-Unix platforms

### Location

`crates/shared/src/common/utility_file.rs`

### Problem

The current code uses inode-based cycle detection:

```rust
#[cfg(not(unix))]
fn get_inode(_meta: &std::fs::Metadata) -> u64 {
    0
}
```

On non-Unix platforms, every directory returns inode `0`.

Then the walker does:

```rust
if !visited.insert(inode) {
    continue;
}
```

So after the first directory is visited, every subsequent directory appears already visited and is skipped.

### Impact

On Windows or any non-Unix target:

- `check`
- `scan`
- orphan detection
- file discovery
- workspace scanning

may only see a tiny subset of files.

This is a critical functional bug.

### Additional Problem

The symlink check in `walk_source_files_inner` uses:

```rust
if !target.starts_with(dir) {
    continue;
}
```

This is too strict. If a symlink inside `crates/foo` points to another valid path inside the workspace but outside `crates/foo`, it is skipped even though it is still inside the workspace root.

### Fixed Code

Replace the inode-based walker with a canonical-path-based walker.

Also remove the now-unused Unix metadata import and `get_inode` helper.

```rust
// PURPOSE: File & workspace utility — pure logic + I/O, free functions only
//
// Single source of truth for file walking, ignored path matching, source file detection,
// and workspace root detection.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::default_aes_config;

/// Check if a file extension is a known source file.
pub fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a single source file path into the output vector.
pub fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str() {
        if let Ok(fp) = FilePath::new(path_str.to_string()) {
            files.push(fp);
        }
    }
}

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }

    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();

    for pat in ignored {
        if pat.is_empty() {
            continue;
        }

        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }

            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();

            if pat_segments.is_empty() {
                continue;
            }

            let n_pat = pat_segments.len();
            let n_seg = segments.len();

            if n_seg < n_pat {
                continue;
            }

            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }

            continue;
        }

        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };

            if suffix.is_empty() {
                continue;
            }

            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
                return true;
            }

            continue;
        }

        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();

        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();

            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Build default ignored paths from config, with a hardcoded safety net for
/// build artifacts and dependency trees that must never be linted.
pub fn default_ignored_paths() -> Vec<String> {
    let mut ignored: Vec<String> = vec![
        "target".to_string(),
        "test-workspaces".to_string(),
        ".mimocode".to_string(),
        ".agents".to_string(),
        "node_modules".to_string(),
        "build.rs".to_string(),
        // PERF-002: common expensive directories.
        ".git".to_string(),
        "dist".to_string(),
        "build".to_string(),
        "coverage".to_string(),
        ".venv".to_string(),
    ];

    let config = default_aes_config();
    for fp in config.ignored_paths.values.iter() {
        let v = fp.value.replace('/', std::path::MAIN_SEPARATOR_STR);
        if !v.is_empty() && !ignored.contains(&v) {
            ignored.push(v);
        }
    }

    ignored
}

/// Collect all lintable source files from a directory tree.
pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();

    if dir.exists() && dir.is_dir() {
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
    }

    files
}

/// Collect all lintable source files without applying default ignores.
pub fn collect_all_source_files_raw(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();

    if dir.exists() && dir.is_dir() {
        let ignored: Vec<String> = Vec::new();
        walk_source_files(dir, &mut files, &ignored);
    }

    files
}

/// Scan a directory and return files as FilePathList.
pub fn scan_directory(path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
    let dir = std::path::Path::new(&path.value);

    if !dir.exists() || !dir.is_dir() {
        return Ok(FilePathList { values: vec![] });
    }

    let files = collect_all_source_files(dir);
    Ok(FilePathList { values: files })
}

/// Walk a directory tree collecting all source files, skipping ignored directories.
/// Symlink targets outside the root directory are pruned to prevent path traversal.
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::new();
    visited.insert(root.clone());

    walk_source_files_inner(&root, files, ignored, &mut visited, &root);
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if is_ignored_dir(&path, ignored) {
            continue;
        }

        // Handle symlinks explicitly.
        if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
            if sym_meta.file_type().is_symlink() {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    // Prevent symlink escape.
                    if !target.starts_with(root) {
                        continue;
                    }

                    // Cycle prevention.
                    if !visited.insert(target.clone()) {
                        continue;
                    }

                    if target.is_dir() {
                        walk_source_files_inner(&target, files, ignored, visited, root);
                    } else if target.is_file() {
                        collect_source_file(&target, files);
                    }
                }

                continue;
            }
        }

        if path.is_dir() {
            let dir_name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            if dir_name == "tests" {
                continue;
            }

            let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
            if !visited.insert(canonical) {
                continue;
            }

            walk_source_files_inner(&path, files, ignored, visited, root);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if is_source_file(ext) {
                collect_source_file(&path, files);
            }
        }
    }
}

/// Walk a directory tree collecting all .rs files.
/// Contained to `dir` — symlink targets outside the root are pruned.
pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::new();
    visited.insert(root.clone());

    walk_rs_files_inner(&root, cb, ignored, &mut visited, &root);
}

fn walk_rs_files_inner(
    dir: &Path,
    cb: &mut dyn FnMut(PathBuf),
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let p = entry.path();

        if is_ignored_dir(&p, ignored) {
            continue;
        }

        if let Ok(sym_meta) = std::fs::symlink_metadata(&p) {
            if sym_meta.file_type().is_symlink() {
                if let Ok(target) = std::fs::canonicalize(&p) {
                    if !target.starts_with(root) {
                        continue;
                    }

                    if !visited.insert(target.clone()) {
                        continue;
                    }

                    if target.is_dir() {
                        walk_rs_files_inner(&target, cb, ignored, visited, root);
                    } else if target.is_file()
                        && matches!(target.extension().and_then(|e| e.to_str()), Some("rs"))
                    {
                        cb(target);
                    }
                }

                continue;
            }
        }

        if p.is_dir() {
            let canonical = std::fs::canonicalize(&p).unwrap_or_else(|_| p.clone());
            if !visited.insert(canonical) {
                continue;
            }

            walk_rs_files_inner(&p, cb, ignored, visited, root);
        } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
            cb(p);
        }
    }
}

/// Read file content synchronously.
pub fn read_file_sync(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Get file basename.
pub fn get_basename(path: &str) -> &str {
    std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Get file stem.
pub fn get_file_stem(path: &str) -> &str {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Check if path is a directory.
pub fn is_directory(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

/// Check if path is a file.
pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

/// Get parent directory path.
pub fn get_parent(path: &str) -> &str {
    std::path::Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
}

/// Walk up from `start` looking for workspace root markers.
pub fn find_workspace_root(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();

    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }

    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }

        if !dir.pop() {
            return None;
        }
    }
}
```

### Why this is better

- Works correctly on Windows and Unix.
- Avoids the inode `0` collision.
- Prevents symlink escape using the real workspace root.
- Still avoids cycles.
- Keeps existing public API unchanged.

---

# 3. Critical Output Correctness Fix

---

## BUG-002 — Multi-workspace scan corrupts JSON/SARIF/JUnit output

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

In `scan_with_discovery`, this block runs even when `--format json`, `--format sarif`, or `--format junit` is used:

```rust
if multi {
    println!(
        "Lint Arwaky v{} (Multi-Workspace Mode)",
        env!("CARGO_PKG_VERSION")
    );
    println!("Found {} workspaces in {path}", workspaces.len());
    println!();
}
```

Later, per-workspace text is also printed:

```rust
if multi {
    let total = member_results.len();
    println!("── [{ws_type}] {ws_name} — {total} violations ──");
    ...
}
```

Then structured output is printed:

```rust
Format::Json => {
    let json = serde_json::to_string_pretty(&global_all_results)
        .unwrap_or_else(|_| "[]".to_string());
    println!("{json}");
}
```

This produces invalid JSON/SARIF/JUnit because human-readable text appears before the machine-readable document.

### Fix

Print human-readable banners and per-workspace summaries only when `format == Format::Text`.

Replace the initial multi-workspace banner block with:

```rust
let multi = workspaces.len() > 1;

if multi && matches!(format, Format::Text) {
    println!(
        "Lint Arwaky v{} (Multi-Workspace Mode)",
        env!("CARGO_PKG_VERSION")
    );
    println!("Found {} workspaces in {path}", workspaces.len());
    println!();
}
```

Then replace the per-workspace output block with:

```rust
if multi {
    if matches!(format, Format::Text) {
        let total = member_results.len();
        println!("── [{ws_type}] {ws_name} — {total} violations ──");

        if !member_results.is_empty() {
            let mut code_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();

            for r in &member_results {
                *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
            }

            let mut sorted: Vec<_> = code_counts.into_iter().collect();
            sorted.sort_by_key(|b| std::cmp::Reverse(b.1));

            for (code, count) in &sorted {
                println!("       {code}: {count}");
            }
        } else {
            println!("   (clean)");
        }

        println!();
    }
} else {
    // Single workspace — print full violation detail, respecting --format.
    match format {
        Format::Text => {
            let results_list = LintResultList::new(member_results.clone());
            print!(
                "{}",
                code_analysis_linter.format_report(&results_list, &ws.path)
            );
        }
        Format::Json => {
            let json = serde_json::to_string_pretty(&member_results)
                .unwrap_or_else(|_| "[]".to_string());
            println!("{json}");
        }
        Format::Sarif => {
            let sarif = self.format_sarif_output(&member_results);
            println!("{sarif}");
        }
        Format::Junit => {
            let junit = self.format_junit_output(&member_results);
            println!("{junit}");
        }
    }
}
```

### Result

- `--format text` keeps the human-friendly multi-workspace summary.
- `--format json`
- `--format sarif`
- `--format junit`

now produce clean machine-readable stdout.

---

# 4. Path Filtering Fixes

---

## BUG-003 — String prefix path filtering can include unrelated paths

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

Current filtering uses:

```rust
abs_path.to_string_lossy().starts_with(&canonical_scan_path)
```

This is unsafe because string prefix matching is not path-component aware.

Example:

```text
scan path: /repo/foo
file:      /repo/foobar/main.rs
```

String prefix returns true, but the file is outside `/repo/foo`.

### Fixed `filter_and_display_results`

Replace the existing method with this corrected version:

```rust
/// Filter results to the target path and display the report.
fn filter_and_display_results(
    &self,
    all_results: Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    path: &str,
    filter: Option<&str>,
    reporter: Arc<dyn ICodeAnalysisAggregate>,
    format: &Format,
) -> usize {
    let cwd = crate::surface_common_command::current_dir();
    let cwd_canonical = cwd.canonicalize().unwrap_or_else(|_| cwd.clone());

    let scan_path = std::path::Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| cwd_canonical.join(path));

    let in_scope = |file_value: &str| {
        let file_path = std::path::Path::new(file_value);

        let abs_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            cwd_canonical.join(file_value)
        };

        // Component-aware check.
        abs_path.starts_with(&scan_path)
            // Fallback for cases where the result path is still relative.
            || file_path.starts_with(path)
    };

    let filtered_results: Vec<_> = if let Some(code) = filter {
        all_results
            .into_iter()
            .filter(|r| r.code.code() == code && in_scope(&r.file.value))
            .collect()
    } else {
        all_results
            .into_iter()
            .filter(|r| in_scope(&r.file.value))
            .collect()
    };

    let violation_count = filtered_results.len();

    match format {
        Format::Text => {
            let results_list = LintResultList::new(filtered_results);

            let report_path =
                match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => shared::common::taxonomy_path_vo::FilePath {
                        value: path.to_string(),
                    },
                };

            println!("{}", reporter.format_report(&results_list, &report_path));
        }
        Format::Json => {
            let json = serde_json::to_string_pretty(&filtered_results)
                .unwrap_or_else(|_| "[]".to_string());
            println!("{json}");
        }
        Format::Sarif => {
            let sarif = self.format_sarif_output(&filtered_results);
            println!("{sarif}");
        }
        Format::Junit => {
            let junit = self.format_junit_output(&filtered_results);
            println!("{junit}");
        }
    }

    violation_count
}
```

### Result

- No more accidental inclusion of sibling paths.
- Filtering is component-aware.
- Relative and absolute result paths are handled more robustly.

---

## BUG-004 — Workspace filtering fallback duplicates results

### Location

`crates/cli-commands/src/surface_check_command.rs`, inside `scan_with_discovery`

### Problem

Current code:

```rust
let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();

...

ws_canonical
    .as_ref()
    .map(|c| abs_path.starts_with(c))
    .unwrap_or(true)
```

If canonicalization fails, the filter becomes `true`.

That means every result is included for every workspace, causing duplicated violations in the global report.

### Fixed Workspace Filtering Block

Replace the workspace canonicalization/filtering section inside the `for ws in &workspaces` loop with:

```rust
// Filter results to only those in this workspace member's path.
let cwd_for_ws = match std::env::current_dir() {
    Ok(d) => d,
    Err(_) => std::path::PathBuf::new(),
};

let cwd_canonical = cwd_for_ws.canonicalize().unwrap_or_else(|_| cwd_for_ws.clone());

let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();

let ws_fallback = if std::path::Path::new(&ws.path.value).is_absolute() {
    std::path::PathBuf::from(&ws.path.value)
} else {
    cwd_canonical.join(&ws.path.value)
};

let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

let in_workspace = |file_value: &str| {
    let file_path = std::path::Path::new(file_value);

    let abs_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        cwd_canonical.join(file_value)
    };

    match &ws_canonical {
        Some(canonical_ws) => abs_path.starts_with(canonical_ws),
        None => {
            abs_path.starts_with(&ws_fallback)
                || file_path.starts_with(&ws.path.value)
        }
    }
};

let filtered_results: Vec<_> = if let Some(code) = filter {
    all_results
        .into_iter()
        .filter(|r| r.code.code() == code && in_workspace(&r.file.value))
        .collect()
} else {
    all_results
        .into_iter()
        .filter(|r| in_workspace(&r.file.value))
        .collect()
};

// Filter orphan results to this workspace member's path.
let filtered_orphans: Vec<_> = if let Some(code) = filter {
    _orphan_results_all
        .iter()
        .filter(|r| r.code.code() == code && in_workspace(&r.file.value))
        .cloned()
        .collect()
} else {
    _orphan_results_all
        .iter()
        .filter(|r| in_workspace(&r.file.value))
        .cloned()
        .collect()
};
```

### Result

- No duplicate results when canonicalization fails.
- Safer fallback matching.
- Still filters correctly in normal canonical cases.

---

# 5. Git-Diff Functional Fixes

---

## BUG-005 — `check --git-diff` ignores path and filter

### Location

- `crates/cli-commands/src/surface_check_action.rs`
- `crates/cli-commands/src/surface_git_command.rs`

### Problem

`handle_check` receives a path:

```rust
path: Option<FilePath>
```

and a filter:

```rust
filter: Option<String>
```

But in git-diff mode it calls:

```rust
crate::surface_git_command::handle_git_diff(
    git_agg,
    ctx.code_analysis_linter.clone(),
    GitBranchName::new("HEAD"),
)
```

And `handle_git_diff` hardcodes:

```rust
let project_path = FilePath::new(".".to_string()).unwrap_or_default();
```

So:

- the user-provided path is ignored,
- the global `--filter` is ignored.

### Fix Part 1 — Update `handle_git_diff`

Replace `handle_git_diff` in `surface_git_command.rs` with:

```rust
pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    project_path: FilePath,
    base: GitBranchName,
    filter: Option<String>,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path, &base)
        .await;

    let files: Vec<&shared::common::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
        .filter(|fp| shared::common::utility_language_detector::is_lintable(fp))
        .collect();

    println!("Path: {}", project_path.value());
    println!("Base: {} (changed files)", base.value());
    println!("Files changed: {}", files.len());
    println!();

    let mut total_violations = 0;

    for f in &files {
        let results = code_analysis_linter.run_code_analysis_path(f);

        let results: Vec<_> = results
            .into_iter()
            .filter(|r| match &filter {
                Some(code) => r.code.code() == code.as_str(),
                None => true,
            })
            .collect();

        let fv = results.len();
        total_violations += fv;

        if fv > 0 {
            println!("  {}  -> {} violation(s)", f.value, fv);

            for r in results.iter().take(3) {
                println!(
                    "    {}:{} [{}] {}",
                    r.file.value(),
                    r.line.value(),
                    match r.severity {
                        shared::common::taxonomy_severity_vo::Severity::CRITICAL => "CRITICAL",
                        shared::common::taxonomy_severity_vo::Severity::HIGH => "HIGH",
                        shared::common::taxonomy_severity_vo::Severity::MEDIUM => "MEDIUM",
                        shared::common::taxonomy_severity_vo::Severity::LOW => "LOW",
                        _ => "INFO",
                    },
                    r.message.value()
                );
            }
        } else {
            println!("  {}  -> clean", f.value);
        }
    }

    println!();
    println!(
        "{} violations across {} changed files",
        total_violations,
        files.len()
    );

    if total_violations > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

### Fix Part 2 — Update `handle_check`

In `surface_check_action.rs`, replace the git-diff branch inside `handle_check` with:

```rust
if git_diff {
    let git_agg = match git_aggregate {
        Some(g) => g,
        None => {
            eprintln!("[error] git hooks not available");
            return ExitCode::FAILURE;
        }
    };

    let project_path = FilePath::new(root.clone()).unwrap_or_default();

    let rt = match crate::surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::FAILURE,
    };

    rt.block_on(crate::surface_git_command::handle_git_diff(
        git_agg,
        ctx.code_analysis_linter.clone(),
        project_path,
        GitBranchName::new("HEAD"),
        filter,
    ))
} else {
    let surface = CheckCommandsSurface::new(ctx);
    surface.scan(&root, filter.as_deref(), config, format)
}
```

### Result

- `check <path> --git-diff` now respects `<path>`.
- `--filter AES204` now works in git-diff mode.
- No new command or feature is introduced; existing CLI options now work correctly.

---

# 6. Adapter Listing Fix

---

## BUG-006 — `adapters` command ignores actual adapters

### Location

`crates/cli-commands/src/surface_plugin_command.rs`

### Problem

Current code:

```rust
pub fn handle_adapters(_external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    println!("  - ESLint (JavaScript/TypeScript)");
    ...
}
```

The injected aggregate is unused.

The command should list active adapters, not a static brochure list.

### Fixed Code

```rust
// PURPOSE: PluginCommandsSurface — CLI surface for listing adapters/plugins

use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");

    let adapters = external_lint.adapter_names();

    if adapters.is_empty() {
        println!("  (none enabled)");
    } else {
        for adapter in adapters {
            println!("  - {adapter}");
        }
    }

    ExitCode::SUCCESS
}
```

### Result

- The command now reflects the actual injected external lint subsystem.
- No new feature; it simply implements the existing command contract correctly.

---

# 7. CI Score Comparison Fix

---

## BUG-007 — CI threshold comparison truncates score

### Location

`crates/cli-commands/src/surface_common_command.rs`

### Problem

Current code:

```rust
let below_threshold = (score.value() as u32) < threshold.value();
```

This truncates the float score.

Example:

```text
score: 79.9
threshold: 80
```

This should fail, and it does.

But:

```text
score: 80.9
threshold: 80
```

casts to `80`, which passes. That may be acceptable, but the comparison should be explicit in floating point to avoid accidental precision bugs.

More importantly, if future score logic uses decimals near thresholds, truncation is unsafe.

### Fixed Code

Replace:

```rust
let below_threshold = (score.value() as u32) < threshold.value();
```

with:

```rust
let below_threshold = score.value() < threshold.value() as f64;
```

Also update the failure reason formatting slightly for clarity:

```rust
if below_threshold {
    reasons.push(format!(
        "Score below threshold ({:.1} < {})",
        score.value(),
        threshold.value()
    ));
}
```

### Result

- CI pass/fail logic is numerically explicit.
- Avoids unintended truncation behavior.

---

# 8. JUnit XML Escaping Fix

---

## BUG-008 — `xml_escape` may produce invalid XML

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

As shown in the uploaded code, `xml_escape` appears to push the same character back:

```rust
'&' => escaped.push_str("&"),
'<' => escaped.push_str("<"),
'>' => escaped.push_str(">"),
'"' => escaped.push_str("""),
'\'' => escaped.push_str("'"),
```

If that is the real source and not a rendering artifact, it does not escape XML at all.

That can produce invalid JUnit XML when messages contain:

- `&`
- `<`
- `>`
- quotes
- apostrophes

### Fixed Code

Replace the existing `xml_escape` function with:

```rust
/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len() + 16);

    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&"),
            '<' => escaped.push_str("<"),
            '>' => escaped.push_str(">"),
            '"' => escaped.push_str("""),
            '\'' => escaped.push_str("'"),
            other => escaped.push(other),
        }
    }

    escaped
}
```

### Result

- JUnit output is valid XML.
- Prevents broken CI test reporters.
- Minor performance improvement from slightly larger initial capacity.

---

# 9. Silent Audit Error Fix

---

## BUG-009 — Naming/import audit errors are silently discarded

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

Current code:

```rust
all_results.extend(naming_results.unwrap_or_default());
all_results.extend(import_results.unwrap_or_default());
```

If naming or import auditing fails, the user sees nothing.

That can make a broken scan appear clean.

### Fix in `scan()`

Replace:

```rust
all_results.extend(naming_results.unwrap_or_default());
all_results.extend(import_results.unwrap_or_default());
```

with:

```rust
match naming_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] naming audit failed: {e}"),
}

match import_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] import audit failed: {e}"),
}
```

### Fix in `scan_with_discovery()`

Replace the equivalent block inside the workspace loop with:

```rust
match naming_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] naming audit failed for {}: {e}", ws.path.value),
}

match import_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] import audit failed for {}: {e}", ws.path.value),
}
```

### Result

- Users are informed when a subsystem fails.
- Does not change successful scan behavior.
- Avoids silently producing incomplete reports.

---

# 10. Workspace Member Selection Fix

---

## BUG-010 — `scan --member` uses substring matching

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

Current filter:

```rust
ws_file.contains(member_name) || ws.path.value.contains(member_name)
```

This can match unintended members.

Example:

```bash
lint-arwaky-cli scan . --member sha
```

could match:

- `shared`
- `shader-rules`
- `crates/sha`

even though the user may have wanted an exact member.

### Fixed Code

Replace the member filtering block with:

```rust
let workspaces = if let Some(member_name) = member {
    let member_name = member_name.trim_end_matches('/');

    let filtered: Vec<_> = workspaces
        .into_iter()
        .filter(|ws| {
            let ws_file = std::path::Path::new(&ws.path.value)
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            ws_file == member_name || ws.path.value == member_name
        })
        .collect();

    if filtered.is_empty() {
        eprintln!("[error] no workspace member matching '{member_name}'");
        eprintln!();
        eprintln!("Available members:");

        for ws in &all_workspaces {
            let name = std::path::Path::new(&ws.path.value)
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            eprintln!("  - {} ({})", name, ws.workspace_type);
        }

        eprintln!();
        eprintln!("Usage: lint-arwaky-cli scan {path} --member <name>");

        return ExitCode::from(1);
    }

    filtered
} else {
    workspaces
};
```

### Result

- Exact member selection.
- Avoids scanning the wrong workspace.
- Still supports full path match if the user passes a path-like member.

---

# 11. Performance Bottleneck Fixes

---

## PERF-001 — Orphan detection scans ignored/generated directories

### Location

`crates/cli-commands/src/surface_check_command.rs`

### Problem

In `scan_with_discovery`, the orphan graph is built using:

```rust
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files_raw(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

`collect_all_source_files_raw` applies no ignore rules.

That means it can walk:

- `node_modules`
- `target`
- `.git`
- `dist`
- `build`
- `.venv`
- generated directories

This can massively degrade scan performance and may also produce false orphan results from generated or dependency code.

### Fixed Code

Replace the raw collection with the ignored-aware collection:

```rust
// Collect source files from workspace root for cross-workspace orphan detection.
// Use ignore-aware collection to avoid scanning dependency/build directories.
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

### Result

- Large monorepo scans become much faster.
- Orphan detection avoids dependency/generated code.
- No new functionality; it simply uses the safe existing walker.

---

## PERF-002 — Default ignore list misses expensive directories

### Location

`crates/shared/src/common/utility_file.rs`

### Problem

The hardcoded ignore list did not include common expensive directories such as:

- `.git`
- `dist`
- `build`
- `coverage`
- `.venv`

### Fixed Code

Already included in the full `utility_file.rs` replacement above.

The relevant part is:

```rust
let mut ignored: Vec<String> = vec![
    "target".to_string(),
    "test-workspaces".to_string(),
    ".mimocode".to_string(),
    ".agents".to_string(),
    "node_modules".to_string(),
    "build.rs".to_string(),
    ".git".to_string(),
    "dist".to_string(),
    "build".to_string(),
    "coverage".to_string(),
    ".venv".to_string(),
];
```

### Result

- Faster directory traversal.
- Fewer irrelevant files considered by lint/orphan analysis.
- Lower I/O and CPU usage.

---

## PERF-003 — Eager construction of all aggregates

### Location

`crates/cli-commands/src/root_cli_container.rs`

### Problem

`CliContainer::new_default()` constructs every subsystem:

- code analysis
- import rules
- naming rules
- role rules
- external lint
- orphan detector
- config system
- git hooks

This is expensive for commands that do not need all subsystems, such as:

- `version`
- `adapters`
- `mcp-config`
- `init`
- `doctor`

### Recommended Optimization

This is best fixed in the binary entrypoint, not by changing the container’s public shape.

Instead of:

```rust
let container = CliContainer::new_default();

match cli.command {
    Commands::Version => ...,
    Commands::Adapters => ...,
    _ => ...,
}
```

use:

```rust
match cli.command {
    Commands::Version => {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    Commands::Adapters => {
        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        return cli_commands::surface_plugin_command::handle_adapters(external_lint);
    }

    Commands::McpConfig { client } => {
        return cli_commands::surface_setup_command::handle_mcp_config(&client);
    }

    // Only construct the full container for commands that actually need it.
    _ => {
        let container = cli_commands::root_cli_container::CliContainer::new_default();
        // existing dispatch logic
    }
}
```

### Result

- Faster CLI startup for lightweight commands.
- Lower memory usage for commands that do not need the full lint pipeline.
- No functional behavior change.

---

# 12. Minor Functional Fix

---

## BUG-011 — `dependencies` returns success on failure

### Location

`crates/cli-commands/src/surface_maintenance_command.rs`

### Problem

Current code:

```rust
Err(e) => {
    println!("{e}");
}
```

Then the function still returns:

```rust
ExitCode::SUCCESS
```

If dependency reporting fails, the command should not report success.

### Fixed Code

Replace the `handle_dependencies` error arm with:

```rust
Err(e) => {
    println!("{e}");
    return ExitCode::from(1);
}
```

Full corrected function:

```rust
pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    println!("Dependency Report — {}", target);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");

            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }

            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }
        }
        Err(e) => {
            println!("{e}");
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}
```

---

# 13. Additional Observations Not Fixed Because They Would Add Behavior

These are not patched above because changing them would introduce new behavior rather than purely fixing existing behavior.

## 13.1 Global `--output-dir` is parsed but not used

`Cli` defines:

```rust
pub output_dir: Option<String>
```

But the surfaces print directly to stdout.

Implementing file output would be a new feature, so it is intentionally not added here.

## 13.2 Global `--verbose` and `--quiet` are parsed but not consistently used

These flags exist in the CLI definition but are not consistently wired into all surfaces.

Wiring them would change output behavior beyond debugging.

## 13.3 `scan` default factory ignores `ArchitectureConfig`

In `CheckCommandsSurface::scan`, when no factory is provided:

```rust
Arc::new(move |_cfg: ArchitectureConfig| CheckContext { ... })
```

The config is ignored.

This is documented in the code comment, but it is still a latent correctness issue. Fixing it properly requires DI/factory support for per-config aggregate construction, which is beyond a pure bug-fix patch.

---

# 14. Recommended Regression Tests

After applying the fixes, I recommend adding or running these regression tests.

## 14.1 Windows/Non-Unix File Walk Regression

Test that nested directories are all discovered:

```text
workspace/
  crates/
    a/
      main.rs
    b/
      lib.rs
  packages/
    c/
      index.ts
```

Expected:

- all three files are discovered.

## 14.2 Structured Output Regression

Run:

```bash
lint-arwaky-cli scan . --format json
```

Expected:

- stdout is valid JSON only.
- no human-readable banner before JSON.

Same for:

```bash
lint-arwaky-cli scan . --format sarif
lint-arwaky-cli scan . --format junit
```

## 14.3 Path Prefix Regression

Given:

```text
/repo/foo
/repo/foobar
```

Run:

```bash
lint-arwaky-cli check /repo/foo
```

Expected:

- results from `/repo/foobar` are not included.

## 14.4 Git-Diff Path Regression

Run:

```bash
lint-arwaky-cli check ./crates/shared --git-diff
```

Expected:

- diff is evaluated relative to `./crates/shared`, not always `.`.

## 14.5 Git-Diff Filter Regression

Run:

```bash
lint-arwaky-cli check . --git-diff --filter AES204
```

Expected:

- only `AES204` violations are reported.

## 14.6 Member Selection Regression

Given workspaces:

```text
shared
shader
```

Run:

```bash
lint-arwaky-cli scan . --member sha
```

Expected:

- error: no workspace member matching `sha`.

Run:

```bash
lint-arwaky-cli scan . --member shared
```

Expected:

- scans only `shared`.

---

# 15. Final Recommendation

Apply the patches in this order:

1. `utility_file.rs` walker and ignore-list fix.
2. `surface_check_command.rs` structured output, filtering, error visibility, member selection, and orphan source collection fixes.
3. `surface_git_command.rs` and `surface_check_action.rs` git-diff path/filter fixes.
4. `surface_common_command.rs` CI threshold fix.
5. `surface_plugin_command.rs` adapter listing fix.
6. `surface_maintenance_command.rs` dependency failure exit-code fix.
7. Optional entrypoint optimization to avoid constructing `CliContainer` for lightweight commands.

These changes correct real functional defects and reduce unnecessary I/O/CPU work without adding new product features.

// PURPOSE: Test/bench directory discovery — the one walk that does not skip `tests/`
//
// The default discovery merges `DEFAULT_IGNORED_PATHS`, which lists `tests` and
// `benches`: the per-file rule auditors (AES101/AES102, AES2xx, AES3xx, AES4xx)
// are about production source, and reading a test file through them produces
// noise nobody acts on. AES103 is the exception — it exists precisely to judge
// the files inside those two directories, so it needs a walk that keeps them.
//
// This module supplies the *path* half of that: which directories to read, and
// which ignore patterns to read them under. Reading them is left to the caller,
// because reading is the filesystem aggregate's job and AES201 forbids a utility
// reaching for a contract. Splitting it this way also keeps the two dispatch
// entry points (`scan` and `naming`) agreeing on which directories AES103 is
// meant to see.
//
// The directory names are parameters rather than constants. This module must not
// depend on a sibling feature's vocabulary; the caller that owns the layout rules
// (AES103, via `shared-naming-rules`) passes the names it enforces.
use std::path::{Path, PathBuf};

/// How deep the hunt for a test directory descends.
///
/// A test directory sits directly under a feature folder, and a feature folder
/// is at most one level under a workspace member, so two levels cover every real
/// layout — and the single-folder and single-member scan targets too. The cap
/// stops a symlink cycle or a pathologically deep tree from running unbounded.
const MAX_TEST_DIR_DEPTH: usize = 3;

/// Every directory under *root* whose own name is in *directories*, down to
/// [`MAX_TEST_DIR_DEPTH`], sorted and deduplicated.
///
/// The search is by name, not by position, because the depth of a test directory
/// depends on the scan target: `tests/` is directly under a single feature
/// folder, under `crates/<feature>/` at a workspace root, and under
/// `<member>/<feature>/` when the target is a member directory. One name-based
/// walk serves all three.
pub fn find_test_directories(
    root: &Path,
    directories: &[&str],
    ignored: &[String],
) -> Vec<PathBuf> {
    // The configured list names `tests` and `benches` — they are in
    // `DEFAULT_IGNORED_PATHS` precisely so the production walk prunes them. Here
    // they are the subject, so they are dropped from the pattern list before any
    // prune test runs. Filtering inside the walk instead would prune the suite
    // roots themselves and report nothing at all.
    let effective: Vec<String> = ignored
        .iter()
        .filter(|pattern| !directories.contains(&pattern.as_str()))
        .cloned()
        .collect();

    let mut found = Vec::new();
    // The root itself counts. A scan whose target *is* `crates/calc/tests`
    // would otherwise descend looking for a directory named `tests` beneath it,
    // find nothing, and report no AES103 finding for the very directory the
    // scan was pointed at.
    if directory_name(root).is_some_and(|name| directories.contains(&name)) {
        found.push(root.to_path_buf());
    }
    collect_test_directories(root, directories, 0, &effective, &mut found);
    found.sort();
    found.dedup();
    found
}

/// The final segment of `path`, or `None` when it names the filesystem root.
fn directory_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|n| n.to_str())
}

fn collect_test_directories(
    dir: &Path,
    directories: &[&str],
    depth: usize,
    ignored: &[String],
    found: &mut Vec<PathBuf>,
) {
    if depth > MAX_TEST_DIR_DEPTH {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        // `file_type` reads the entry itself, so a symlinked directory is not
        // descended into — a link into an already-walked tree would otherwise
        // recurse forever.
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        // A configured pattern prunes the subtree here, before it can be opened
        // as a suite root. Filtering only the walk-rooted children's names would
        // let an ignored folder still be discovered, because each suite walk is
        // rooted *inside* it and its own name no longer matches.
        if ignored.iter().any(|pattern| *pattern == name) {
            continue;
        }
        if directories.contains(&name) {
            found.push(path.clone());
            // Don't skip the subtree — a suite can contain subdirectories that
            // themselves hold files the rule must judge (nested unit tests are a
            // known layout). Continuing the walk means those files appear in the
            // returned directory list and get picked up by the caller.
            collect_test_directories(&path, directories, depth + 1, ignored, found);
            continue;
        }
        // Build artifacts and VCS metadata are never a route to a test folder.
        if shared_common::DEFAULT_IGNORED_PATHS.contains(&name) {
            continue;
        }
        collect_test_directories(&path, directories, depth + 1, ignored, found);
    }
}

/// The default skip list minus the directories being *kept*, plus the caller's
/// patterns.
///
/// A caller that explicitly asks to ignore `tests/` still gets it ignored: the
/// config is the authority. This only relaxes the built-in default, and only for
/// the directories it was asked to keep.
///
/// The names are removed because the caller roots each walk at the directory
/// itself: `ignore::WalkBuilder` prunes the whole subtree when an ignore pattern
/// matches the walk root's own name, so leaving them in would discard every file
/// the caller just went looking for.
pub fn ignore_patterns_keeping(kept: &[&str], extra: &[String]) -> Vec<String> {
    let mut merged: Vec<String> = shared_common::DEFAULT_IGNORED_PATHS
        .iter()
        .filter(|name| !kept.contains(name))
        .map(|s| s.to_string())
        .collect();
    for pattern in extra {
        if !merged.contains(pattern) {
            merged.push(pattern.clone());
        }
    }
    merged
}

/// Whether *path* sits inside one of *directories*, at any depth.
///
/// The dispatcher uses this to scope a test-file finding back to the scan target
/// the same way it scopes every other finding, so a member-dir scan does not
/// report a sibling member's test files.
pub fn is_inside_any(path: &str, directories: &[&str]) -> bool {
    let normalized = path.replace('\\', "/");
    normalized
        .split('/')
        .any(|segment| directories.contains(&segment))
}

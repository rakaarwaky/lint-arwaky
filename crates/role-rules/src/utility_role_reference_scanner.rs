// PURPOSE: Workspace-wide identifier reference index for the role rules.
//
// The role rules receive every parsed file, so the index is built in one pass
// over each file's harvested identifiers. Test and bench files are excluded
// from the main index, so they are scanned separately on disk to keep the
// index complete.
//
// This file lives outside the agent orchestrator on purpose: the reference
// index needs filesystem access, and an agent file is not allowed to do I/O
// (AES405 P6) nor to declare free functions (AES405 P11).

use std::collections::HashSet;
use std::path::Path;

use shared::filesystem::taxonomy_filesystem_vo::{ExternalReferenceMap, FileEntry, ParseMetadata};

/// True when a path lies inside a test or bench directory.
pub fn is_test_or_bench_path(path: &str) -> bool {
    path.contains("/tests/")
        || path.contains("/benches/")
        || path.contains("\\tests\\")
        || path.contains("\\benches\\")
}

/// Build the workspace-wide map of which file references which identifier.
pub fn build_external_reference_map(files: &[FileEntry]) -> ExternalReferenceMap {
    let mut map = ExternalReferenceMap::default();
    for file in files {
        let path = file.path.to_string_lossy().to_string();
        if is_test_or_bench_path(&path) {
            map.has_test_references = true;
        }
        let identifiers: Vec<String> = match &file.parse_metadata {
            Some(ParseMetadata::Rust(r)) => r.used_identifiers.clone(),
            Some(ParseMetadata::Python(py)) => py.used_identifiers.clone(),
            Some(ParseMetadata::TypeScript(ts)) | Some(ParseMetadata::JavaScript(ts)) => {
                ts.used_identifiers.clone()
            }
            _ => continue,
        };
        if identifiers.is_empty() {
            continue;
        }
        map.by_file.insert(path, identifiers);
    }
    // Supplement with on-disk test/bench files, which the main index excludes.
    let mut seen: HashSet<String> = map.by_file.keys().cloned().collect();
    if let Ok(ws_root) = std::env::current_dir() {
        for sub in ["crates", "packages", "modules"] {
            let base = ws_root.join(sub);
            if !base.is_dir() {
                continue;
            }
            if let Ok(members) = std::fs::read_dir(&base) {
                for member in members.flatten() {
                    let member_path = member.path();
                    if !member_path.is_dir() {
                        continue;
                    }
                    let src_dir = member_path.join("src");
                    for sub_dir in ["tests", "benches"] {
                        let dir = src_dir.join(sub_dir);
                        if !dir.is_dir() {
                            continue;
                        }
                        collect_test_refs(&dir, &mut map, &mut seen);
                    }
                }
            }
        }
    }
    map
}

/// Recursively harvest lower-case identifiers from a test/bench directory.
pub fn collect_test_refs(dir: &Path, map: &mut ExternalReferenceMap, seen: &mut HashSet<String>) {
    fn walk(d: &Path, map: &mut ExternalReferenceMap, seen: &mut HashSet<String>) {
        let entries = match std::fs::read_dir(d) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                walk(&p, map, seen);
                continue;
            }
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !matches!(ext, "rs" | "py" | "ts" | "js") {
                continue;
            }
            let path_str = p.to_string_lossy().to_string();
            if !seen.insert(path_str.clone()) {
                continue;
            }
            let content = match std::fs::read_to_string(&p) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let identifiers = harvest_identifiers(&content);
            if identifiers.is_empty() {
                continue;
            }
            map.has_test_references = true;
            map.by_file.insert(path_str, identifiers);
        }
    }
    walk(dir, map, seen);
}

/// Trim lines, skip `use` / `import` declarations, then keep whole lower-case
/// identifiers. This is a coarse harvest on purpose: the role rules use the
/// index only to tell "is this name mentioned anywhere else" apart from
/// "is this name private to one file".
pub fn harvest_identifiers(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with("use ") && !t.starts_with("import ")
        })
        .flat_map(|l| l.split(' ').map(|w| w.trim().to_string()))
        .filter(|w| {
            w.chars()
                .next()
                .map(|c| c.is_ascii_lowercase())
                .unwrap_or(false)
                && w.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
        .collect()
}

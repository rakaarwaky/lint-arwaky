// PURPOSE: Resolve external crate/package imports to file paths within a workspace.
// Pure functions — no state, no I/O side effects beyond filesystem reads.

use std::collections::HashSet;
use std::path::Path;

/// Resolve an external crate import (e.g. `use calculator_addition::foo::Bar`)
/// or package import (e.g. `import { X } from "calculator-shared/src/foo"`)
/// by scanning workspace member Cargo.toml / package.json files.
/// Returns the relative path to the target file if found.
pub fn resolve_external_crate_import(
    crate_name: &str,
    sub_path: &str,
    top_root: &Path,
    all_files_set: &HashSet<&str>,
) -> Option<String> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut candidate_dirs: Vec<(String, String)> = Vec::new();
    for member_dir in &member_dirs {
        let base = top_root.join(member_dir);
        if base.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        let member_base = format!("{}/{}", member_dir, dir_name);
                        let src_dir = format!("{}/src", member_base);
                        candidate_dirs.push((member_base, src_dir));
                    }
                }
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir(top_root) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if member_dirs.contains(&dir_name.as_str()) {
                    continue;
                }
                let member_base = dir_name.clone();
                let src_dir = format!("{}/src", member_base);
                if !candidate_dirs.iter().any(|(mb, _)| mb == &member_base) {
                    candidate_dirs.push((member_base, src_dir));
                }
            }
        }
    }

    for (member_base, src_dir) in &candidate_dirs {
        let member_path = top_root.join(member_base);
        let cargo_toml = member_path.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Some(name) = read_cargo_package_name(&cargo_toml) {
                let normalized = name.replace('-', "_");
                if normalized == crate_name
                    || member_base.split('/').next_back().unwrap_or("") == crate_name
                {
                    if let Some(path) = resolve_sub_path(src_dir, sub_path, all_files_set) {
                        return Some(path);
                    }
                }
            }
        }
        let package_json = member_path.join("package.json");
        if package_json.exists() {
            if let Some(name) = read_npm_package_name(&package_json) {
                if name == crate_name || name.replace('-', "_") == crate_name {
                    if let Some(path) = resolve_sub_path(src_dir, sub_path, all_files_set) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

fn read_cargo_package_name(cargo_toml: &Path) -> Option<String> {
    let content = std::fs::read_to_string(cargo_toml).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("name") && trimmed.contains('=') {
            if let Some((_, val)) = trimmed.split_once('=') {
                return Some(val.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

fn read_npm_package_name(package_json: &Path) -> Option<String> {
    let content = std::fs::read_to_string(package_json).ok()?;
    for line in content.lines() {
        let trimmed = line.trim().trim_end_matches(',');
        if trimmed.contains("\"name\"") && trimmed.contains(':') {
            if let Some((_, val)) = trimmed.split_once(':') {
                let val = val.trim().trim_matches('"');
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
    }
    None
}

fn resolve_sub_path(
    src_dir: &str,
    sub_path: &str,
    all_files_set: &HashSet<&str>,
) -> Option<String> {
    let parts: Vec<&str> = sub_path.split('/').collect();
    for i in (0..=parts.len()).rev() {
        let candidate_base = if i == 0 {
            format!("{}/lib", src_dir)
        } else {
            format!("{}/{}", src_dir, parts[..i].join("/"))
        };
        // Try Rust patterns
        let rust_candidates = vec![
            format!("{}.rs", candidate_base),
            format!("{}/mod.rs", candidate_base),
        ];
        for candidate in &rust_candidates {
            if all_files_set.contains(candidate.as_str()) {
                return Some(candidate.clone());
            }
        }
        // Try TypeScript/JavaScript patterns
        let ts_candidates = vec![
            format!("{}.ts", candidate_base),
            format!("{}.tsx", candidate_base),
            format!("{}.js", candidate_base),
            format!("{}.jsx", candidate_base),
            format!("{}/index.ts", candidate_base),
            format!("{}/index.tsx", candidate_base),
            format!("{}/index.js", candidate_base),
        ];
        for candidate in &ts_candidates {
            if all_files_set.contains(candidate.as_str()) {
                return Some(candidate.clone());
            }
        }
        if i > 0 {
            let parent_dir = format!("{}/{}", src_dir, parts[..i].join("/"));
            let parent_mod = format!("{}/mod.rs", parent_dir);
            if all_files_set.contains(parent_mod.as_str()) {
                return Some(parent_mod);
            }
        }
    }
    None
}

/// Given a resolved external crate file path (e.g. "crates/shared/src/taxonomy_result_vo.rs"),
/// derive the crate root lib.rs (e.g. "crates/shared/src/lib.rs").
pub fn derive_crate_lib_rs(resolved_path: &str) -> Option<String> {
    let parts: Vec<&str> = resolved_path.split('/').collect();
    if let Some(src_idx) = parts.iter().position(|&p| p == "src") {
        let lib_rs = format!("{}/lib.rs", parts[..=src_idx].join("/"));
        Some(lib_rs)
    } else {
        None
    }
}

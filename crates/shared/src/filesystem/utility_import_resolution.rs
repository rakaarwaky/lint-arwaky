// PURPOSE: Resolve external crate/package imports to file paths within a workspace.
// Pure functions — no state, no I/O side effects beyond filesystem reads.

use crate::taxonomy_filesystem_vo::{ImportEntry, Language};
use std::collections::{HashMap, HashSet};
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
    // Fallback: a workspace dependency *key* is the ident a consumer writes in
    // `use`, while both the package name and the member directory may differ
    // from it — e.g. `shared-common = { package = "shared-common-lint-arwaky",
    // path = "crates/shared/src/common" }` is reached as `shared_common::…`
    // from a directory three levels deep. Only the workspace manifest ties the
    // three together.
    if let Some(src_dir) = workspace_dep_src_dir(top_root, crate_name) {
        if let Some(path) = resolve_sub_path(&src_dir, sub_path, all_files_set) {
            return Some(path);
        }
    }
    None
}

/// Locate the source directory of `crate_name` through `[workspace.dependencies]`
/// of the manifest at `top_root`.
///
/// The source directory is `<member>/src` normally; a package whose
/// `[lib] path = "mod.rs"` keeps its crate root beside `Cargo.toml`, so those
/// members have no `src/` child and resolve from the member directory itself.
fn workspace_dep_src_dir(top_root: &Path, crate_name: &str) -> Option<String> {
    let content = std::fs::read_to_string(top_root.join("Cargo.toml")).ok()?;
    let mut in_deps = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[workspace.dependencies]";
            continue;
        }
        if !in_deps || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        if key.trim().replace('-', "_") != crate_name {
            continue;
        }
        let member_base = toml_inline_str(value, "path")?;
        let member_path = top_root.join(&member_base);
        return Some(
            if !member_path.join("src").is_dir() && member_path.join("mod.rs").is_file() {
                member_base
            } else {
                format!("{member_base}/src")
            },
        );
    }
    None
}

/// Read `field = "value"` out of a TOML value such as an inline table
/// (`{ package = "…", path = "…", version = "…" }`).
fn toml_inline_str(value: &str, field: &str) -> Option<String> {
    let idx = value.find(&format!("{field} ="))?;
    let rest = &value[idx + field.len()..];
    let start = rest.find('"')? + 1;
    let end = rest[start..].find('"')?;
    Some(rest[start..start + end].to_string())
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
        let mut rust_candidates = vec![
            format!("{}.rs", candidate_base),
            format!("{}/mod.rs", candidate_base),
        ];
        if i == 0 {
            // Crate-root re-export (`use shared_common::FilePath`). A package
            // whose `[lib] path = "mod.rs"` has no `lib.rs`, so its root is
            // `<src_dir>/mod.rs` — the last resort after the `lib` patterns.
            rust_candidates.push(format!("{}/mod.rs", src_dir));
        }
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

/// Given a resolved external crate file path (e.g.
/// "crates/filesystem/src/capabilities_ast_parser.rs"), derive the crate root
/// that re-exports it (e.g. "crates/filesystem/src/lib.rs").
///
/// Most members keep their root at `src/lib.rs`. A package declaring
/// `[lib] path = "mod.rs"` keeps its sources beside `Cargo.toml`, so the root
/// is the nearest `mod.rs` barrel above the file — reached only when the
/// `src/lib.rs` candidate is absent, leaving ordinary crates untouched.
pub fn derive_crate_lib_rs(resolved_path: &str, all_files_set: &HashSet<&str>) -> Option<String> {
    let parts: Vec<&str> = resolved_path.split('/').collect();
    let src_idx = parts.iter().position(|&p| p == "src")?;
    let lib_rs = format!("{}/lib.rs", parts[..=src_idx].join("/"));
    if all_files_set.contains(lib_rs.as_str()) {
        return Some(lib_rs);
    }
    let mut dir = std::path::PathBuf::from(std::path::Path::new(resolved_path).parent()?);
    loop {
        let candidate = dir.join("mod.rs").to_string_lossy().to_string();
        if all_files_set.contains(candidate.as_str()) {
            return (candidate != resolved_path).then_some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Resolves a raw import path against the workspace file set using
/// language-specific rules for Rust, Python, TypeScript, and JavaScript.
///
/// Pure function: no `self`, no I/O, no side effects.
///
/// # Arguments
///
/// * `imp` - Import metadata holding the raw path and detected language.
/// * `src_dir` - Directory of the importing file, workspace-relative.
/// * `src_rel` - Workspace-relative path of the importing file.
/// * `top_root` - Workspace root used to normalize resolved paths.
/// * `all_files_set` - Workspace-relative paths of all discovered files.
/// * `stem_index` - Index of file stems used for Python module matching.
///
/// # Returns
///
/// The workspace-relative target path when the import resolves to a discovered file; otherwise, `None`.
pub fn resolve_by_language(
    imp: &ImportEntry,
    src_dir: &str,
    src_rel: &str,
    top_root: &Path,
    all_files_set: &HashSet<&str>,
    stem_index: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let raw = imp.raw_path.as_str();
    if imp.language == Language::Python {
        if raw.starts_with('.') {
            let module_path = raw.trim_start_matches('.').replace('.', "/");
            let candidates = vec![
                format!("{}/{}.py", src_dir, module_path),
                format!("{}/{}/__init__.py", src_dir, module_path),
            ];
            candidates
                .into_iter()
                .find(|c| all_files_set.contains(c.as_str()))
        } else {
            let module_path = raw.replace('.', "/");
            // A: direct path (import already includes member prefix)
            let try_direct = |suffix: &str| {
                let p = format!("{}{}", module_path, suffix);
                all_files_set.contains(p.as_str()).then_some(p)
            };
            try_direct(".py")
                .or_else(|| try_direct("/__init__.py"))
                // B: prepend modules/ | packages/ | crates/
                .or_else(|| {
                    ["modules", "packages", "crates"].iter().find_map(|md| {
                        let py = format!("{}/{}.py", md, module_path);
                        if all_files_set.contains(py.as_str()) {
                            return Some(py);
                        }
                        let init = format!("{}/{}/__init__.py", md, module_path);
                        all_files_set.contains(init.as_str()).then_some(init)
                    })
                })
                // C: suffix/stem match (bare module name in nested dir)
                .or_else(|| {
                    let stem = raw.rsplit('.').next().unwrap_or(raw);
                    let is_dotted = raw.contains('.');
                    let candidate_suffix = if is_dotted {
                        format!("/{}.py", raw.replace('.', "/"))
                    } else {
                        format!("/{}.py", stem)
                    };
                    let root_candidate = format!(
                        "{}.py",
                        if is_dotted { raw.replace('.', "/") } else { stem.to_string() }
                    );
                    let member_dir = src_dir.split('/').take(2).collect::<Vec<_>>().join("/");
                    let domain_prefix = format!("{}/", member_dir);
                    stem_index.get(stem).and_then(|candidates| {
                        candidates
                            .iter()
                            .filter(|f| {
                                f.as_str() != src_rel
                                    && (**f == root_candidate || f.ends_with(&candidate_suffix))
                            })
                            .min_by_key(|f| (!f.starts_with(&domain_prefix), f.as_str()))
                            .map(|f| f.to_string())
                    })
                })
        }
    } else if imp.language == Language::TypeScript || imp.language == Language::JavaScript {
        let parts: Vec<&str> = raw.split('/').collect();
        if parts.len() >= 2 {
            let pkg_name = parts[0];
            let sub = if parts.len() > 2 && parts[1] == "src" {
                parts[2..].join("/")
            } else {
                parts[1..].join("/")
            };
            resolve_external_crate_import(pkg_name, &sub, top_root, all_files_set)
        } else {
            None
        }
    } else {
        let module = raw
            .strip_prefix("crate::")
            .or_else(|| raw.strip_prefix("super::"))
            .unwrap_or(raw);
        let root_seg = module.split("::").next().unwrap_or("");
        if !root_seg.is_empty() {
            let candidate = if src_dir.is_empty() {
                format!("{}.rs", root_seg)
            } else {
                format!("{}/{}.rs", src_dir, root_seg)
            };
            if all_files_set.contains(candidate.as_str()) {
                Some(candidate)
            } else {
                let sub_path = module.split("::").skip(1).collect::<Vec<_>>().join("/");
                resolve_external_crate_import(root_seg, &sub_path, top_root, all_files_set)
            }
        } else {
            None
        }
    }
}

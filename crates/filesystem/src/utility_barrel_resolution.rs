// FR-001: Barrel File Resolution
// Resolves imports through barrel files (__init__.py, mod.rs, index.ts, etc.)
// to their original source files.
//
// Utility: stateless functions
// Consumers: import-rules (via IFilesystemAggregate), agent orchestrator

use crate::utility_filesystem_io;
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

/// Resolve all imports in a list through barrel files.
/// Populates `resolved_path` and `is_resolved` fields in each ImportEntry.
/// Returns a new list (does not mutate input).
pub fn resolve_barrel_imports(entries: Vec<ImportEntry>, root_dir: &Path) -> Vec<ImportEntry> {
    entries
        .into_iter()
        .map(|entry| resolve_single_import(entry, root_dir))
        .collect()
}

/// Resolve a single import through barrel files.
/// If the import resolves, sets `resolved_path` and `is_resolved = true`.
pub fn resolve_single_import(mut entry: ImportEntry, root_dir: &Path) -> ImportEntry {
    if entry.is_wildcard || entry.resolved_path.is_some() {
        return entry;
    }

    // Handle Python relative imports (starting with '.')
    if entry.language == shared::filesystem::taxonomy_filesystem_vo::Language::Python
        && (entry.raw_path.starts_with('.') || entry.raw_path.starts_with(".."))
    {
        return resolve_python_relative_import(entry, root_dir);
    }

    let symbols = if !entry.symbols.is_empty() {
        entry.symbols.clone()
    } else {
        // Single import — use last segment of raw_path
        let last = entry
            .raw_path
            .rsplit("::")
            .next()
            .unwrap_or(&entry.raw_path);
        if last.is_empty() || last == "*" || last == "self" || last == "_" || last == "default" {
            return entry;
        }
        vec![last.to_string()]
    };

    let module_path = entry_module_path(&entry.raw_path);

    for sym in &symbols {
        if sym.is_empty() || sym == "*" || sym == "self" {
            continue;
        }
        if let Some(resolved) = resolve_barrel_import(module_path, sym, root_dir) {
            entry.resolved_path = Some(PathBuf::from(&resolved));
            entry.is_resolved = true;
            return entry;
        }
    }

    entry
}

/// Resolve Python relative imports (e.g., '.taxonomy_expression_vo', '..utils').
/// Uses the source file's directory as the base for resolution.
fn resolve_python_relative_import(mut entry: ImportEntry, root_dir: &Path) -> ImportEntry {
    let source_dir = entry.source_file.parent().unwrap_or(root_dir);

    // Count leading dots to determine relative depth
    let raw_path = &entry.raw_path;
    let dot_count = raw_path.chars().take_while(|&c| c == '.').count();

    // Build the module name (without dots)
    let module_name = raw_path.trim_start_matches('.');

    eprintln!(
        "[debug resolve_python] raw_path='{}', source='{}', module='{}', dots={}",
        raw_path,
        entry.source_file.display(),
        module_name,
        dot_count
    );

    // Resolve the relative path
    let base_dir = if dot_count >= 2 {
        // '..' means go up one directory
        source_dir.parent().unwrap_or(source_dir)
    } else {
        // '.' means current directory
        source_dir
    };

    // Try to find the module as a .py file
    let py_file = base_dir.join(format!("{}.py", module_name));
    eprintln!("[debug resolve_python] looking for: {}", py_file.display());
    if py_file.exists() {
        entry.resolved_path = Some(py_file);
        entry.is_resolved = true;
        return entry;
    }

    // Try to find as a package (directory with __init__.py)
    let pkg_dir = base_dir.join(module_name);
    let init_file = pkg_dir.join("__init__.py");
    if init_file.exists() {
        entry.resolved_path = Some(init_file);
        entry.is_resolved = true;
        return entry;
    }

    eprintln!("[debug resolve_python] NOT FOUND");
    entry
}

// ═══════════════════════════════════════════════════════════════
// Barrel File Resolution
// ═══════════════════════════════════════════════════════════════

/// Extract the module path from a raw_path (before `::{` if grouped).
fn entry_module_path(raw_path: &str) -> &str {
    if let Some(brace_pos) = raw_path.find("::{") {
        &raw_path[..brace_pos]
    } else {
        raw_path
    }
}

/// Normalize a module path for filesystem lookup.
fn normalize_module_path(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .replace('.', "/")
        .replace("::", "/")
}

/// Find the barrel file for a module path.
fn find_barrel_file(module_path: &str, root_dir: &Path) -> Option<String> {
    let base = root_dir;
    let clean_path = normalize_module_path(module_path);
    let module_dir = base.join(&clean_path);

    let barrel_candidates = [
        "__init__.py",
        "index.ts",
        "index.js",
        "index.tsx",
        "index.jsx",
        "mod.rs",
    ];

    // Check the module directory itself
    if let Some(found) = try_barrel_candidates(&module_dir, &barrel_candidates) {
        return Some(found);
    }

    // Check parent directory
    if let Some(parent) = module_dir.parent() {
        if let Some(found) = try_barrel_candidates(parent, &barrel_candidates) {
            return Some(found);
        }
    }

    // Try crate src directory pattern
    let segments: Vec<&str> = clean_path.split('/').collect();
    if let Some(first_seg) = segments.first() {
        let crate_names = [*first_seg, &first_seg.replace('_', "-")];

        for (idx, &crate_name) in crate_names.iter().enumerate() {
            if idx == 1 && crate_name == crate_names[0] {
                continue;
            }
            let crate_src = base.join("crates").join(crate_name).join("src");
            let remainder: Vec<&str> = segments.iter().skip(1).copied().collect();

            if !remainder.is_empty() {
                let remainder_opts = [remainder.join("/"), remainder.join("/").replace('_', "-")];
                for (rem_idx, rem) in remainder_opts.iter().enumerate() {
                    if rem_idx == 1 && *rem == remainder_opts[0] {
                        continue;
                    }
                    let full_dir = crate_src.join(rem);
                    if let Some(found) = try_barrel_candidates(&full_dir, &barrel_candidates) {
                        return Some(found);
                    }
                }
            }

            if remainder.len() > 1 {
                let remainder_str = remainder.join("/");
                let parent_path = Path::new(&remainder_str);
                if let Some(parent_dir) = parent_path.parent() {
                    let dir = crate_src.join(parent_dir);
                    if let Some(found) = try_barrel_candidates(&dir, &barrel_candidates) {
                        return Some(found);
                    }
                    let parent_hyphen = parent_dir.to_string_lossy().replace('_', "-");
                    let dir_hyphen = crate_src.join(&parent_hyphen);
                    if let Some(found) = try_barrel_candidates(&dir_hyphen, &barrel_candidates) {
                        return Some(found);
                    }
                }
            }
        }
    }

    None
}

/// Try to find a barrel file at the given base path with all candidate names.
fn try_barrel_candidates(dir: &Path, candidates: &[&str]) -> Option<String> {
    for candidate in candidates {
        let barrel_path = dir.join(candidate);
        if barrel_path.exists() {
            return Some(barrel_path.to_string_lossy().to_string());
        }
    }
    None
}

/// Parse barrel file content and extract re-export mappings.
/// Returns HashMap<symbol_name, source_file_path>.
pub fn parse_barrel_reexports(barrel_content: &str) -> HashMap<String, String> {
    let mut reexports = HashMap::new();
    for line in barrel_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        // Python: from .module import Name
        if trimmed.starts_with("from .") || trimmed.starts_with("from ..") {
            if let Some(imp_part) = trimmed.split_once(" import ") {
                let source_module = imp_part.0.strip_prefix("from ").unwrap_or("").trim();
                let names_part = imp_part.1.trim();
                let clean = names_part
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim_end_matches(';');
                for name in clean.split(',') {
                    let name = name.trim().split(" as ").last().unwrap_or("").trim();
                    if !name.is_empty() && name != "*" {
                        let rel_path = source_module.replace('.', "/");
                        reexports.insert(name.to_string(), rel_path);
                    }
                }
            }
            continue;
        }

        // Rust: pub use crate::module::Name;
        if trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use ") {
            let use_part = trimmed
                .trim_start_matches("pub(crate) use ")
                .trim_start_matches("pub use ")
                .trim_end_matches(';')
                .trim();
            let path = use_part.rsplit("::").last().unwrap_or(use_part);
            if !path.is_empty() && path != "*" && path != "self" {
                reexports.insert(path.to_string(), use_part.to_string());
            }
            continue;
        }

        // TypeScript: export { Name } from './module'
        if trimmed.starts_with("export") && trimmed.contains(" from ") {
            if let Some((export_part, from_part)) = trimmed.split_once(" from ") {
                let from_path = from_part
                    .trim()
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`');
                if let Some(open) = export_part.find('{')
                    && let Some(close) = export_part.find('}')
                {
                    let inner = &export_part[open + 1..close];
                    for name in inner.split(',') {
                        let name = name.trim().split(" as ").last().unwrap_or("").trim();
                        if !name.is_empty() && name != "type" {
                            reexports.insert(name.to_string(), from_path.to_string());
                        }
                    }
                }
            }
            continue;
        }

        // TypeScript: export * from './module'
        if trimmed.starts_with("export * from ") {
            // Wildcard re-export — we can't resolve individual symbols
            continue;
        }
    }

    reexports
}

/// Resolve an import through a barrel file to its original source file.
/// Returns the resolved file path as a string.
fn resolve_barrel_import(module_path: &str, symbol_name: &str, root_dir: &Path) -> Option<String> {
    let barrel_path = find_barrel_file(module_path, root_dir)?;
    let barrel_content = utility_filesystem_io::read_file_safe(&barrel_path).ok()?;
    let reexports = parse_barrel_reexports(&barrel_content);
    reexports.get(symbol_name).cloned()
}

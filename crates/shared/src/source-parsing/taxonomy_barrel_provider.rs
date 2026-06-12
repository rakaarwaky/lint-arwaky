// PURPOSE: BarrelProvider — detects barrel/index files (_init_.py, mod.rs, index.ts)
use std::collections::HashMap;

/// Centralized barrel import resolver and barrel utility functions.
/// When a file imports from a barrel (mod.rs), resolve through the barrel
/// to find the actual source file that defines the imported symbol.
pub struct BarrelImportResolver;

impl BarrelImportResolver {
    /// Shared: check if a filename is a barrel file.
    pub fn is_barrel_file(filename: &str) -> bool {
        filename.ends_with("mod.rs")
            || filename.ends_with("__init__.py")
            || filename.ends_with("/index.ts")
            || filename.ends_with("/index.js")
            || filename.ends_with("/index.tsx")
            || filename == "lib.rs"
    }

    /// Build a reverse map: barrel mod.rs → list of source file paths it re-exports.
    /// Scans all project files to find barrel files and their pub use/pub mod declarations.
    pub fn build_barrel_map(files: &[String]) -> HashMap<String, Vec<String>> {
        let mut barrel_map: HashMap<String, Vec<String>> = HashMap::new();
        let stem_map = Self::build_stem_map(files);

        for f in files {
            let basename = f.split('/').next_back().unwrap_or("");
            if basename != "mod.rs" && basename != "lib.rs" {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(f) {
                // Collect pub use re-exports: pub use xxx::SomeType;
                for line in content.lines() {
                    let t = line.trim();
                    // pub mod xxx;
                    if let Some(rest) = t
                        .strip_prefix("pub mod ")
                        .or_else(|| t.strip_prefix("mod "))
                    {
                        let mod_name = rest.trim_end_matches(';').trim();
                        let dir = f.trim_end_matches(basename).trim_end_matches('/');
                        let candidates = vec![
                            format!("{}/{}.rs", dir, mod_name),
                            format!("{}/{}/mod.rs", dir, mod_name),
                        ];
                        for cand in &candidates {
                            if std::path::Path::new(cand).exists() {
                                barrel_map.entry(f.clone()).or_default().push(cand.clone());
                                break;
                            }
                        }
                    }
                    // pub use xxx::*; → resolve all files in xxx/ or xxx.rs
                    if t.contains("pub use ") && t.contains("::*") {
                        let prefix = t
                            .strip_prefix("pub use ")
                            .unwrap_or(t)
                            .split("::*")
                            .next()
                            .unwrap_or("")
                            .trim();
                        // Normalize hyphens to underscores for module name matching
                        let normalized = prefix.replace('-', "_");
                        if let Some(resolved) =
                            stem_map.get(&normalized).or_else(|| stem_map.get(prefix))
                        {
                            barrel_map
                                .entry(f.clone())
                                .or_default()
                                .extend(resolved.clone());
                        }
                    }
                    // pub use crate::xxx::SomeType; AND pub use crate::xxx::{A, B, C};
                    if t.starts_with("pub use ") && !t.contains("::*") {
                        let import_path = t
                            .strip_prefix("pub use ")
                            .unwrap_or(t)
                            .trim_end_matches(';')
                            .trim();
                        // Handle braced multi-import: crate::module::{A, B}
                        if let Some(brace_pos) = import_path.find("::{") {
                            let module_part = &import_path[..brace_pos];
                            // Extract module name from crate::xxx::{...}
                            let parts: Vec<&str> = module_part.split("::").collect();
                            if parts.len() >= 2 {
                                let module_name = parts[1].replace('-', "_");
                                if let Some(resolved) = stem_map.get(&module_name) {
                                    barrel_map
                                        .entry(f.clone())
                                        .or_default()
                                        .extend(resolved.clone());
                                }
                            }
                        } else {
                            let parts: Vec<&str> = import_path.split("::").collect();
                            if parts.len() >= 2 {
                                let module_name = parts[1].replace('-', "_");
                                if let Some(resolved) = stem_map.get(&module_name) {
                                    barrel_map
                                        .entry(f.clone())
                                        .or_default()
                                        .extend(resolved.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        barrel_map
    }

    fn build_stem_map(files: &[String]) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for f in files {
            let basename = f.split('/').next_back().unwrap_or("");
            let stem_val = basename.split('.').next().unwrap_or("").to_string();
            map.entry(stem_val.clone()).or_default().push(f.clone());
            let dir = f.trim_end_matches(basename).trim_end_matches('/');
            let dirstem = format!("{}/{}", dir, stem_val);
            map.entry(dirstem).or_default().push(f.clone());
        }
        map
    }

    /// Given a file path, resolve its imports through barrels.
    /// Returns a list of RESOLVED source file paths (not barrel paths).
    pub fn resolve_imports_for_file(
        file_path: &str,
        barrel_map: &HashMap<String, Vec<String>>,
        project_files: &[String],
    ) -> Vec<String> {
        let mut resolved: Vec<String> = Vec::new();
        if let Ok(content) = std::fs::read_to_string(file_path) {
            for line in content.lines() {
                let t = line.trim();
                if t.starts_with("use ") {
                    let path = t
                        .strip_prefix("use ")
                        .unwrap_or(t)
                        .split(" as ")
                        .next()
                        .unwrap_or("")
                        .split("::")
                        .collect::<Vec<_>>();
                    if path.len() >= 2 && path[0] == "crate" {
                        let module_name = path[1].replace('-', "_");
                        // Check if this module has a barrel
                        for (barrel_file, sources) in barrel_map {
                            let barrel_stem = barrel_file.split('/').next_back().unwrap_or("");
                            let barrel_name = if barrel_stem == "mod.rs" || barrel_stem == "lib.rs"
                            {
                                let dir = barrel_file
                                    .trim_end_matches(barrel_stem)
                                    .trim_end_matches('/');
                                dir.split('/').next_back().unwrap_or("")
                            } else {
                                barrel_stem.split('.').next().unwrap_or("")
                            };
                            if module_name == barrel_name
                                || barrel_file.contains(&format!("/{}/", module_name))
                            {
                                resolved.extend(sources.clone());
                            }
                        }
                        // Also add any direct file match
                        for pf in project_files {
                            let pb = pf.split('/').next_back().unwrap_or("");
                            if pb.starts_with(&format!("{}_", module_name)) {
                                resolved.push(pf.clone());
                            }
                        }
                    }
                }
            }
        }
        resolved.sort();
        resolved.dedup();
        resolved
    }

    /// Check if a specific file is reachable through barrel imports from any contract file.
    pub fn is_imported_by_contract(
        target_file: &str,
        barrel_map: &HashMap<String, Vec<String>>,
        project_files: &[String],
    ) -> bool {
        let target_stem = target_file
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("")
            .to_string();
        for cf in project_files {
            let cb = cf.split('/').next_back().unwrap_or("");
            if !cb.starts_with("contract_") {
                continue;
            }
            let resolved = Self::resolve_imports_for_file(cf, barrel_map, project_files);
            if resolved.iter().any(|r| {
                r.split('/')
                    .next_back()
                    .unwrap_or("")
                    .contains(&target_stem)
            }) {
                return true;
            }
            // Also check direct contains fallback.
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&target_stem) {
                    return true;
                }
            }
        }
        false
    }
}

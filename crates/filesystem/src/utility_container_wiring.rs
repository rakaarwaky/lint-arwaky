// PURPOSE: Container-aware wiring propagation for DI traceability (P1 from issues #191-193)
// Stateless, domain-agnostic — reusable across crates.
use std::{collections::HashMap, path::Path};

/// Scan container files for identifier references and add synthetic wiring edges.
/// Container files (e.g. `root_cli_main_container.rs`) reference capability/agent
/// struct names via constructor calls — these are invisible to AST import tracing.
/// We match container-used identifiers against file stems to build DI edges.
pub fn add_container_wiring_edges(
    all_files: &[String],
    top_root: &Path,
    stem_index: &HashMap<String, Vec<String>>,
    forward: &mut HashMap<String, Vec<String>>,
) {
    // Collect container files and their used identifiers
    for file_path in all_files {
        if !file_path.contains("_container") {
            continue;
        }

        // Extract container stem (e.g. "root_cli_main_container" → "root_cli_main")
        let basename = file_path.rsplit('/').next().unwrap_or(file_path);
        let container_stem = if let Some(pos) = basename.find("_container") {
            &basename[..pos]
        } else {
            continue;
        };

        // Match container identifiers against capability/agent/taxonomy stems
        for candidate in all_files {
            if candidate == file_path {
                continue;
            }
            let cand_basename = candidate.rsplit('/').next().unwrap_or(candidate);
            if let Some(pos) = cand_basename.find('_') {
                let cand_prefix = &cand_basename[..pos];
                // Container references capability/agent/taxonomy by struct name or stem
                if cand_prefix.contains(container_stem) || container_stem.contains(cand_prefix) {
                    let cand_rel = path_to_relative(Path::new(candidate), top_root);
                    forward.entry(file_path.clone()).or_default().push(cand_rel);
                }
            }
        }

        // Also match via stem_index: container struct names often contain component names
        let container_lower = container_stem.to_lowercase();
        if let Some(matches) = stem_index.get(&container_stem.to_string()) {
            for m in matches {
                if forward
                    .get(file_path)
                    .map(|v| v.contains(m))
                    .unwrap_or(false)
                {
                    continue;
                }
                forward
                    .entry(file_path.clone())
                    .or_default()
                    .push(m.clone());
            }
        }

        // Match by searching all stems for substrings of container name
        let parts: Vec<&str> = container_lower.split('_').collect();
        if parts.len() >= 2 {
            // "root_cli_main" → match files with "cli" or "main" or "cli_main"
            for (i, part) in parts.iter().enumerate() {
                if *part == "root" || *part == "container" {
                    continue;
                }
                // Build combined stem search: e.g. "cli_main", "cli_main_container"
                let combined: String = parts[i..].join("_");
                if let Some(candidates) = stem_index.get(&combined) {
                    for c in candidates {
                        if forward
                            .get(file_path)
                            .map(|v| v.contains(c))
                            .unwrap_or(false)
                        {
                            continue;
                        }
                        forward
                            .entry(file_path.clone())
                            .or_default()
                            .push(c.clone());
                    }
                }
            }
        }
    }
}

/// Free utility: convert absolute path to workspace-relative string.
pub fn path_to_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}

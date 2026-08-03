// PURPOSE: OrphanGraphResolver — entry point identification (business logic only).
// Graph building has been migrated to filesystem.build_orphan_graph_context().

use crate::utility_orphan_filename::file_stem;
use shared::orphan_rules::IOrphanGraphResolverProtocol;
use shared::orphan_rules::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

pub struct OrphanGraphResolver;

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        let matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    basename.ends_with("_container.rs")
                        || basename.ends_with("_container.py")
                        || basename.ends_with("_container.ts")
                        || basename.ends_with("_container.js")
                        || basename.ends_with("_entry.rs")
                        || basename.ends_with("_entry.py")
                        || basename.ends_with("_entry.ts")
                        || basename.ends_with("_entry.js")
                        || basename.starts_with("root_")
                        || basename == "main.rs"
                        || basename == "lib.rs"
                        || basename == "main.py"
                        || basename == "__main__.py"
                        || basename == "main.ts"
                        || basename == "main.js"
                        || basename == "index.ts"
                        || basename == "index.js"
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    let stem = file_stem(basename);
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || stem == *pattern
                            || (pattern.starts_with('_') && stem.ends_with(pattern.as_str()))
                            || (pattern.starts_with('.') && basename.ends_with(pattern.as_str()))
                            || (pattern == "root_" && basename.starts_with("root_"))
                            || (pattern.ends_with(".rs")
                                || pattern.ends_with(".py")
                                || pattern.ends_with(".ts")
                                || pattern.ends_with(".js"))
                                && basename.ends_with(pattern.as_str())
                    })
                })
                .cloned()
                .collect()
        };
        let mut matched = matched;
        matched.sort();
        matched.dedup();
        OrphanFileListVO::new(matched)
    }
}

// PURPOSE: GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphAnalysisContext {
    pub import_graph: ImportGraph,
    pub inbound_links: InboundLinkMap,
    pub inheritance_map: InheritanceMap,
    /// All workspace files used to build the graph — needed for entry point detection
    /// across modules (FR-001). When scanning modules/cli/, this includes root_cli_main_entry.py
    /// from modules/ so surface files imported by it are not falsely flagged as orphans.
    pub all_workspace_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportGraph {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl ImportGraph {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InboundLinkMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InboundLinkMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }

    /// Retrieve importers for a file path, falling back to canonical or suffix matching if needed.
    /// Merges importers from both exact match and ./ prefix match to handle path normalization.
    pub fn get_importers(&self, path: &str) -> Option<&Vec<String>> {
        let mut result: Option<&Vec<String>> = None;

        // Try exact match first
        if let Some(v) = self.mapping.get(path) {
            result = Some(v);
        }

        // Try with ./ prefix at beginning (graph resolver may add this)
        let with_prefix = format!("./{}", path);
        if let Some(v) = self.mapping.get(&with_prefix) {
            // If we already have a result, prefer the ./ prefix version if it has more importers
            if let Some(existing) = result {
                if v.len() > existing.len() {
                    result = Some(v);
                }
            } else {
                result = Some(v);
            }
        }

        // Fast path: strip absolute workspace prefix to get relative path.
        // Graph resolver stores relative keys (e.g., "crates/shared/src/foo.rs")
        // but analyzers may query with absolute paths (e.g., "/home/user/project/crates/shared/src/foo.rs").
        // This O(1) lookup avoids the O(n) suffix scan below.
        if result.is_none() {
            for marker in &["/crates/", "/packages/", "/modules/"] {
                if let Some(pos) = path.find(marker) {
                    let rel = &path[pos + 1..]; // skip leading '/'
                    if let Some(v) = self.mapping.get(rel) {
                        result = Some(v);
                        break;
                    }
                }
            }
        }

        // Try with ./ in the middle (graph resolver may add this)
        // e.g., /home/user/project/./crates/... instead of /home/user/project/crates/...
        if let Some(pos) = path.find("/crates/") {
            let with_middle_dot = format!("{}/.{}", &path[..pos], &path[pos..]);
            if let Some(v) = self.mapping.get(&with_middle_dot) {
                if let Some(existing) = result {
                    if v.len() > existing.len() {
                        result = Some(v);
                    }
                } else {
                    result = Some(v);
                }
            }
        }

        // NOTE: callers should pass canonicalized paths for best matching

        // Try clean path (without ./ prefix)
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            if let Some(v) = self.mapping.get(clean) {
                result = Some(v);
            }
        }

        // Try exact match after stripping ./ from keys
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            for (k, v) in &self.mapping {
                let k_clean = k.strip_prefix("./").unwrap_or(k);
                if k_clean == clean {
                    result = Some(v);
                    break;
                }
            }
        }

        // Try suffix match
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            for (k, v) in &self.mapping {
                let k_clean = k.strip_prefix("./").unwrap_or(k);
                if k_clean.ends_with(clean) || clean.ends_with(k_clean) {
                    result = Some(v);
                    break;
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InheritanceMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InheritanceMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl GraphAnalysisContext {
    pub fn new(
        import_graph: ImportGraph,
        inbound_links: InboundLinkMap,
        inheritance_map: InheritanceMap,
        all_workspace_files: Vec<String>,
    ) -> Self {
        Self {
            import_graph,
            inbound_links,
            inheritance_map,
            all_workspace_files,
        }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}

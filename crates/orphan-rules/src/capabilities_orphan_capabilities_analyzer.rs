use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_orphan_rules::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_vo::FileParseResultVO;
use shared_orphan_rules::utility_orphan_filename::file_stem;
use shared_quality_rules::taxonomy_quality_rules_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;
use std::sync::Arc;

// ─── Block 1: Struct Definition ────────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
    workspace: Arc<dyn IWorkspaceProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    /// Determines whether a capabilities file is unreachable, unwired, or both.
    ///
    /// A file is considered non-orphan only when it is reachable from an entry file
    /// and wired in a root container. Empty file paths skip the wiring check.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = analyzer.is_capabilities_orphan(
    ///     &file_path,
    ///     &root_dir,
    ///     &alive_files,
    ///     &content_map,
    ///     workspace_root,
    /// );
    /// ```
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        content_map: &HashMap<String, String>,
        workspace_root: &std::path::Path,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let stem = file_stem(fp);

        // Condition 1: not reachable from any _entry file
        let is_reachable = alive_files.paths.contains(f);

        // Condition 2: not wired in any root_*_container
        let mut is_wired = false;
        if !fp.is_empty() {
            // Read file content from the pre-computed content_map (no I/O)
            let content_ref = content_map.get(fp).map(|s| s.as_str()).unwrap_or("");
            let identifiers = self.extract_identifiers(fp, content_ref, &stem);
            is_wired = self
                .workspace
                .check_wired_in_container(workspace_root, &PatternList::new(identifiers));
        }

        // Both conditions must be satisfied for non-orphan
        if is_reachable && is_wired {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build diagnostic message
        let reason = if !is_reachable && !is_wired {
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not reachable and not wired in any container.\nWHY: A capability without reachability or wiring cannot be invoked.\nFIX: Ensure '{}' is imported by an entry file and wired in root_*_container.",
                stem, stem
            )
        } else if !is_reachable {
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not reachable from any entry point.\nWHY: A capability must be reachable from an entry point to be invoked.\nFIX: Add an import from a surface, agent, or entry file.",
                stem
            )
        } else {
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is reachable but not wired in any container.\nWHY: A wired capability is required for dependency injection resolution.\nFIX: Import and register '{}' in root_*_container.rs or root_*_container.py.",
                stem, stem
            )
        };

        OrphanIndicatorResult::new(true, reason, Severity::MEDIUM)
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl CapabilitiesOrphanAnalyzer {
    pub fn new(workspace: Arc<dyn IWorkspaceProtocol>) -> Self {
        Self { workspace }
    }

    fn extract_identifiers(&self, file_path: &str, content: &str, stem: &str) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();
        match shared_orphan_rules::parse_file_content(file_path, content) {
            FileParseResultVO::Rust(result) => {
                identifiers.extend(result.struct_names());
                identifiers.extend(result.trait_names());
            }
            FileParseResultVO::Python(result) => {
                identifiers.extend(result.class_names());
            }
            FileParseResultVO::TypeScript(result) => {
                identifiers.extend(result.class_names());
            }
            FileParseResultVO::Unsupported => {}
        }
        identifiers.push(stem.to_string());
        let pascal_stem: String = stem
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect();
        identifiers.push(pascal_stem);
        identifiers.sort();
        identifiers.dedup();
        identifiers
    }
}

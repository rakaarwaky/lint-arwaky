use crate::utility_orphan_filename::file_stem;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::quality_rules::taxonomy_analysis_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CapabilitiesOrphanAnalyzer {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }

    fn extract_identifiers(&self, file_path: &str, content: &str, stem: &str) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();
        match shared::common::parse_file_content(file_path, content) {
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
                .filesystem
                .check_wired_in_container(workspace_root, &PatternList::new(identifiers));
        }

        // Both conditions must be satisfied for non-orphan
        if is_reachable && is_wired {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build diagnostic message
        let reason = if !is_reachable && !is_wired {
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not reachable and not wired.\nWHY? Capabilities file '{}' is not reachable from any _entry file AND not wired in any root_*_container.\nFIX: Import '{}' from a _entry file AND register it in a root_*_container.rs.",
                stem, stem, stem
            )
        } else if !is_reachable {
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not reachable.\nWHY? Capabilities file '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                stem, stem, stem
            )
        } else {
            // P5 (visibility): the file IS reachable — typically via the DI impl
            // bridge (a contract it implements is reachable) or container wiring —
            // so the report says so and pinpoints the remaining gap: container wiring.
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? Capabilities file '{}' is reachable (via import chain, container wiring, or contract implementation bridge) but not wired in any root_*_container file.\nFIX: Register '{}' in a root_*_container.rs.",
                stem, stem, stem
            )
        };

        OrphanIndicatorResult::new(true, reason, Severity::MEDIUM)
    }
}

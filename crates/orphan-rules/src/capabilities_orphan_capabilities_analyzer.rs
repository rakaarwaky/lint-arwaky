use crate::utility_orphan_filename::file_stem;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::ICapabilitiesOrphanProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::quality_rules::taxonomy_analysis_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::sync::Arc;

pub struct CapabilitiesOrphanAnalyzer {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }

    fn extract_identifiers(&self, file_path: &str, content: &str, stem: &str) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();
        match shared::common::parse_file_content(
            file_path, content,
        ) {
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
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp = f.value();
        let stem = file_stem(fp);
        if fp.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? Not reachable from any entry point.\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs.",
                    stem, stem
                ),
                Severity::MEDIUM,
            );
        }

        let path = match FilePath::new(fp) {
            Ok(p) => p,
            Err(_) => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        let content = self.filesystem.read_cached(&path);
        let content_ref = content.value();

        let identifiers = self.extract_identifiers(fp, content_ref, &stem);

        let root = std::path::Path::new(root_dir.value());
        if let Ok(workspace_root) = self.filesystem.find_workspace_root_from_path(root) {
            let wired = self
                .filesystem
                .check_wired_in_container(&workspace_root, &identifiers);
            if wired {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        OrphanIndicatorResult::new(
            true,
            format!(
                "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? Capabilities file '{}' is not wired in any container.\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs.",
                stem, stem, stem
            ),
            Severity::MEDIUM,
        )
    }
}

// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection.
// AST-based: uses parser dispatch for struct/trait name extraction.

use crate::utility_orphan_filename::file_stem;
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};
use shared::common::{FilePath, Severity};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_detector::{
    AesOrphanViolation, ICapabilitiesOrphanProtocol, IOrphanParserProtocol,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

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
                AesOrphanViolation::CapabilitiesOrphan {
                    stem,
                    reason: Some("Not reachable from any entry point.".into()),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        let path = FilePath::new(fp).unwrap_or_default();
        let content = self.filesystem.read_cached(&path);
        let content_ref = content.value();

        // AST-based identifier extraction
        let identifiers = self.extract_identifiers(fp, content_ref, &stem);

        // Search for container files in workspace root
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
            AesOrphanViolation::CapabilitiesOrphan {
                stem,
                reason: Some(
                    "Capabilities file struct/trait is not wired in any container.".into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        let filesystem: Arc<dyn IFilesystemAggregate> =
            Arc::new(filesystem::FilesystemOrchestrator::new());
        Self::new(
            Arc::new(crate::capabilities_orphan_parser_dispatcher::OrphanParserDispatcher::new()),
            filesystem,
        )
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(
        parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            parser_dispatcher,
            filesystem,
        }
    }

    /// Extract identifiers (struct names, trait names, stem, PascalCase stem) using AST.
    /// Replaces extract_struct_names/extract_trait_names regex from shared utility.
    fn extract_identifiers(&self, file_path: &str, content: &str, stem: &str) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        match self.parser_dispatcher.parse_file(file_path, content) {
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

        // Always add stem and PascalCase variant
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

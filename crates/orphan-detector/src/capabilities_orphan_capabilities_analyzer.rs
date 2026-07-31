// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection.
// AST-based: uses parser dispatch for struct/trait name extraction.

use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};
use shared::common::{FilePath, Severity};
use shared::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace_scanner::{
    check_wired_in_container, find_workspace_root,
};
use shared::orphan_detector::{
    AesOrphanViolation, ICapabilitiesOrphanProtocol, IOrphanParserProtocol,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
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
        let content = utility_file_cache::read_cached(&path);
        let content_ref = content.value();

        // AST-based identifier extraction
        let identifiers = self.extract_identifiers(fp, content_ref, &stem);

        // Search for container files in workspace root
        let root = std::path::Path::new(root_dir.value());
        if let Ok(workspace_root) = find_workspace_root(root) {
            let wired = check_wired_in_container(&workspace_root, &identifiers);
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
        Self::new(Arc::new(
            crate::capabilities_orphan_parser_dispatcher::OrphanParserDispatcher::new(),
        ))
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(parser_dispatcher: Arc<dyn IOrphanParserProtocol>) -> Self {
        Self { parser_dispatcher }
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

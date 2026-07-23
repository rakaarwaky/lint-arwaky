// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::taxonomy_definition_vo::LayerDefinition;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TaxonomyOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };

        let is_utility_or_helper = matches!(suffix, "utility" | "helper");

        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        // Taxonomy orphan = no file from other layers imports it.
        // Other layers: contract, capabilities, agent, utility, surface, root, main, entry, container
        let importers = match inbound_links.get_importers(f.value()) {
            Some(v) => v,
            None => {
                // Fallback: graph resolver may not catch crate:: imports within same crate
                if Self::has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category,
                        reason: Some(
                            format!(
                                "Taxonomy '{}' is not imported by any other layer file.",
                                stem
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        };

        // If importers list is empty or only has mod.rs, also check for crate:: self-imports
        // The graph resolver may not track all crate:: imports within the same crate
        let has_only_mod_or_taxonomy = importers.iter().all(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            b == "mod.rs" || b.starts_with("taxonomy_")
        });
        if (importers.is_empty() || has_only_mod_or_taxonomy)
            && Self::has_crate_self_import(f.value())
        {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if any importer is from another layer or non-taxonomy file
        let has_other_layer_importer = importers.iter().any(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            if b == "mod.rs" || b == "__init__.py" || b == "index.ts" || b == "index.js" {
                return false;
            }
            !b.starts_with("taxonomy_")
        });
        let is_orphan = !has_other_layer_importer;

        if is_orphan {
            OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::TaxonomyOrphan {
                    stem: stem.clone(),
                    category,
                    reason: Some(
                        format!(
                            "Taxonomy '{}' is not imported by any other layer file.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Fallback: check if any source file in the same crate imports this module via
    /// `crate::` path, relative import, or direct reference.
    /// The graph resolver doesn't always track same-crate imports (especially TS/JS).
    fn has_crate_self_import(file_path: &str) -> bool {
        let stem = std::path:: Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if stem.is_empty() {
            return false;
        }

        // Find the crate's src/ directory by walking up from the file
        let file_path_obj = std::path::Path::new(file_path);
        let src_dir = if let Some(parent) = file_path_obj.parent() {
            // Walk up to find src/ directory
            let mut current = parent.to_path_buf();
            loop {
                if current.ends_with("src") {
                    break;
                }
                if !current.pop() {
                    // Reached root without finding src/
                    return false;
                }
            }
            current
        } else {
            return false;
        };

        // Scan all source files (.rs, .py, .ts, .js) in the crate's src/ directory
        let all_files =
            shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&src_dir);
        for f in all_files {
            if f == file_path {
                continue;
            }
            let path = std::path::PathBuf::from(&f);
            // Support Rust, Python, TypeScript, and JavaScript files
            if path.extension().is_some_and(|e| {
                let ext = e.to_str().unwrap_or("");
                matches!(ext, "rs" | "py" | "ts" | "js")
            }) {
                let content = shared::orphan_detector::utility_orphan_io::read_file_safe(&f);
                // Check for any import pattern containing the stem
                // This handles: crate::stem, common::stem, module::stem, etc.
                if content.contains(stem) {
                    return true;
                }
            }
        }
        false
    }
}

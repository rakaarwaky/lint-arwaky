// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
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
        // Other layers: contract, capabilities, agent, utility, surface
        let importers = match inbound_links.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                // Fallback: graph resolver may not catch crate:: imports within same crate
                if Self::has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
                // Debug: check if key exists with different format
                let key = f.value();
                let has_key = inbound_links.mapping.contains_key(key);
                eprintln!(
                    "[DEBUG KEY] '{}' key='{}' has_key={} map_keys={}",
                    stem,
                    key,
                    has_key,
                    inbound_links.mapping.len()
                );
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

        // Check if any importer is from another layer (contract, capabilities, agent, utility, surface)
        let has_other_layer_importer = importers.iter().any(|importer| {
            let basename = importer.split('/').next_back().unwrap_or("");
            let starts = basename.starts_with("contract_")
                || basename.starts_with("capabilities_")
                || basename.starts_with("agent_")
                || basename.starts_with("utility_")
                || basename.starts_with("surface_");
            if !starts
                && !basename.starts_with("taxonomy_")
                && !basename.starts_with("mod.")
                && !basename.starts_with("lib.")
            {
                eprintln!(
                    "[DEBUG LAYER] '{}' importer='{}' basename='{}' starts={}",
                    stem, importer, basename, starts
                );
            }
            starts
        });
        eprintln!(
            "[DEBUG HAS] '{}' has_other_layer_importer={} importers={}",
            stem,
            has_other_layer_importer,
            importers.len()
        );

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

    /// Fallback: check if any sibling .rs file in the same directory imports this module via `crate::` path.
    /// The graph resolver doesn't always track crate:: imports within the same crate.
    fn has_crate_self_import(file_path: &str) -> bool {
        let stem = std::path::Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if stem.is_empty() {
            return false;
        }
        let search = format!("crate::{}", stem);
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            let entries = shared::orphan_detector::utility_orphan_io::scan_directory(parent);
            for (_name, path_str, _is_dir) in entries {
                if path_str == file_path {
                    continue;
                }
                let path = std::path::PathBuf::from(&path_str);
                if path.extension().is_some_and(|e| e == "rs") {
                    let content =
                        shared::orphan_detector::utility_orphan_io::read_file_safe(&path_str);
                    if content.contains(&search) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

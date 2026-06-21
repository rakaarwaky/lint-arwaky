// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct TaxonomyOrphanAnalyzer {}

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let def = match definition {
            Some(d) => d,
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        is_taxonomy_orphan(f, root_dir, def, inbound_links)
    }
}

pub fn is_taxonomy_orphan(
    f: &FilePath,
    _root: &FilePath,
    _def: &LayerDefinition,
    inbound: &InboundLinkMap,
) -> OrphanIndicatorResult {
    let is_orphan = !inbound.mapping.contains_key(f.value());
    let stem = f.value().split('/').next_back().unwrap_or("").replace(".rs", "").replace(".py", "");
    OrphanIndicatorResult::new(
        is_orphan,
        AesOrphanViolation::TaxonomyOrphan {
            stem,
            category: "",
            reason: Some("Taxonomy VO has no inbound imports.".into()),
        }
        .to_string(),
        Severity::LOW,
    )
}

pub fn check_taxonomy_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let stem = basename.replace(".rs", "").replace(".py", "");
    let suffix = stem.split('_').next_back().unwrap_or("");

    let is_utility_or_helper = matches!(suffix, "utility" | "helper");

    let mut imported = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");

        if is_utility_or_helper {
            // utility/helper: can be imported directly by any layer, no contract needed
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        } else {
            // vo, entity, error, event, constant: must be imported via contract layer
            if !cb.starts_with("contract_") {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        }
    }
    if !imported {
        let category = if is_utility_or_helper { "utility" } else { "vo" };
        let reason = if is_utility_or_helper {
            format!("Taxonomy '{}' is not imported by any file.", stem)
        } else {
            format!("Taxonomy '{}' is not imported by any contract.", stem)
        };
        violations.push(crate::mk_orphan_result(
            fp,
            &AesOrphanViolation::TaxonomyOrphan {
                stem: stem.clone(),
                category,
                reason: Some(reason.into()),
            }
            .to_string(),
            Severity::LOW,
            "AES501",
        ));
    }
}

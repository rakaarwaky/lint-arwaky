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
    OrphanIndicatorResult::new(
        is_orphan,
        AesOrphanViolation::OrphanCode {
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
    let mut imported = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");
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
    if !imported {
        violations.push(crate::mk_orphan_result(
            fp,
            &AesOrphanViolation::OrphanCode {
                reason: Some(
                    format!("Taxonomy '{}' is not imported by any contract.", stem).into(),
                ),
            }
            .to_string(),
            Severity::LOW,
            "AES501",
        ));
    }
}

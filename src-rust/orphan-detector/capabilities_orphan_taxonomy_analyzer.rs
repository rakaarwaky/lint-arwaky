// PURPOSE: Analyzer: Taxonomy orphan detection logic
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
        "Taxonomy VO has no inbound imports.".into(),
        Severity::LOW,
    )
}

pub fn check_taxonomy_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
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
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            "Taxonomy not imported by any contract.",
            Severity::LOW,
        ));
    }
}

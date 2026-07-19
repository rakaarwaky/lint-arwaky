// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared::orphan_detector::taxonomy_orphan_filename_utility::file_stem;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
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
        is_taxonomy_orphan(f, root_dir, definition, inbound_links)
    }
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
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path == std::path::Path::new(file_path) {
                    continue;
                }
                if path.extension().is_some_and(|e| e == "rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains(&search) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn is_taxonomy_orphan(
    f: &FilePath,
    _root: &FilePath,
    _def: Option<&LayerDefinition>,
    inbound: &InboundLinkMap,
) -> OrphanIndicatorResult {
    let stem = file_stem(f.value());

    let suffix = match stem.rfind('_') {
        Some(pos) => &stem[pos + 1..],
        None => "",
    };

    let is_utility_or_helper = matches!(suffix, "utility" | "helper");

    let is_orphan = if is_utility_or_helper {
        // utility/helper: must be imported by file outside taxonomy
        let importers = match inbound.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                // Fallback: graph resolver may not catch crate:: imports within same crate
                if has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category: "utility",
                        reason: Some(format!("Taxonomy '{}' (utility/helper) is not imported by any file outside taxonomy.", stem).into()),
                    }.to_string(),
                    Severity::LOW,
                );
            }
        };
        let has_outside_taxonomy = importers.iter().any(|importer| {
            importer
                .split('/')
                .next_back()
                .is_some_and(|b| !b.starts_with("taxonomy_"))
        });
        !has_outside_taxonomy
    } else {
        // vo, entity, error, event, constant: must be imported by contract layer
        let importers = match inbound.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category: "taxonomy",
                        reason: Some(
                            format!("Taxonomy '{}' is not imported by any contract.", stem).into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                )
            }
        };
        let has_any_importer = importers.iter().any(|importer| {
            // Must be imported by a file outside the taxonomy layer
            // (contract_, capabilities_, infrastructure_, surface_, agent_, root_)
            importer
                .split('/')
                .next_back()
                .is_some_and(|b| !b.starts_with("taxonomy_"))
        });
        !has_any_importer
    };

    let category = if is_utility_or_helper {
        "utility"
    } else {
        "taxonomy"
    };

    OrphanIndicatorResult::new(
        is_orphan,
        AesOrphanViolation::TaxonomyOrphan {
            stem,
            category,
            reason: None,
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
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");
    let suffix = match stem.rfind('_') {
        Some(pos) => &stem[pos + 1..],
        None => "",
    };

    let is_utility_or_helper = matches!(suffix, "utility" | "helper");

    let mut imported = false;
    for cf in files {
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => continue,
        };

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
        let category = if is_utility_or_helper {
            "utility"
        } else {
            "vo"
        };
        let reason = if is_utility_or_helper {
            format!("Taxonomy '{}' is not imported by any file.", stem)
        } else {
            format!("Taxonomy '{}' is not imported by any contract.", stem)
        };
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
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

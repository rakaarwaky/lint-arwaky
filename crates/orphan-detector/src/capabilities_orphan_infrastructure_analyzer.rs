// PURPOSE: InfrastructureOrphanAnalyzer — IInfrastructureOrphanProtocol for orphan infrastructure detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IInfrastructureOrphanProtocol;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_orphan::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

pub struct InfrastructureOrphanAnalyzer {}

impl Default for InfrastructureOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureOrphanProtocol for InfrastructureOrphanAnalyzer {
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

        // Check if wired in any container
        let fp = f.value();
        let stem = file_stem(fp);

        if let Ok(content) = std::fs::read_to_string(fp) {
            let mut identifiers: Vec<String> = Vec::new();
            identifiers.extend(extract_struct_names(&content));
            identifiers.extend(extract_trait_names(&content));
            identifiers.push(stem.clone());

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

            let root = std::path::Path::new(root_dir.value());
            if let Ok(workspace_root) =
                crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(root)
            {
                if let Ok(wired) =
                    crate::capabilities_orphan_capabilities_analyzer::check_wired_in_container(
                        &workspace_root,
                        &identifiers,
                    )
                {
                    if wired {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::InfrastructureOrphan {
                stem,
                reason: Some(
                    "Not reachable from any entry point and not wired in any container.".into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

pub fn check_capabilities_orphan(
    fp: &str,
    _basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let stem = shared::orphan_detector::utility_orphan_filename::file_stem(fp);
    let content = std::fs::read_to_string(fp).unwrap_or_default();
    use shared::orphan_detector::utility_orphan::{
        extract_struct_names, extract_trait_names,
    };

    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

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

    let mut wired = false;
    for cf in files {
        let cb = shared::orphan_detector::utility_orphan_filename::file_basename(cf);
        let csuffix = shared::orphan_detector::utility_orphan_filename::file_suffix(&cb);
        if csuffix != "container" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            for id in &identifiers {
                if c.contains(id) {
                    wired = true;
                    break;
                }
            }
            if wired {
                break;
            }
        }
    }
    if !wired {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation::InfrastructureOrphan {
                stem: stem.clone(),
                reason: Some(format!("infrastructure '{}' not wired in container.", stem).into()),
            }
            .to_string(),
            shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM,
            "AES504",
        ));
    }
}

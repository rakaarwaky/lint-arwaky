// PURPOSE: InfrastructureOrphanAnalyzer — IInfrastructureOrphanProtocol for orphan infrastructure detection
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::orphan_detector::contract_orphan_protocol::IInfrastructureOrphanProtocol;
use shared::orphan_detector::taxonomy_orphan_utility::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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
    fn is_infrastructure_orphan(
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
        let basename = std::path::Path::new(fp)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = basename.replace(".rs", "").replace(".py", "");

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
                    c.next()
                        .map(|f| f.to_uppercase().to_string() + c.as_str())
                        .unwrap_or_default()
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

pub fn check_infrastructure_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    crate::capabilities_orphan_capabilities_analyzer::check_capabilities_orphan(
        fp, basename, files, violations,
    )
}

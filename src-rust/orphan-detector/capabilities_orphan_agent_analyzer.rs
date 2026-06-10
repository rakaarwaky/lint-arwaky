// PURPOSE: Analyzer: Agent orphan detection logic
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct AgentOrphanAnalyzer {}

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
    ) -> OrphanIndicatorResult {
        is_agent_orphan(f, root_dir)
    }
}

pub fn is_agent_orphan(_f: &FilePath, _root_dir: &FilePath) -> OrphanIndicatorResult {
    is_agent_orphan_raw_wired(false)
}

pub fn is_agent_orphan_raw_wired(is_wired: bool) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(
        !is_wired,
        "Orchestrator not wired in DI container.".into(),
        Severity::HIGH,
    )
}

pub fn is_agent_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let basename = f.basename();
    let stem = basename.replace(".rs", "").replace(".py", "");
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
    let mut is_wired = false;
    for cf in all_files {
        let cb = cf.split('/').next_back().unwrap_or("");
        let csuffix = cb
            .rsplit('_')
            .next()
            .unwrap_or("")
            .replace(".rs", "")
            .replace(".py", "");
        if csuffix != "container" && csuffix != "aggregate" && csuffix != "registry" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&stem) || c.contains(&format!("mod {}", stem)) || c.contains(&pascal_stem)
            {
                is_wired = true;
                break;
            }
        }
    }
    is_agent_orphan_raw_wired(is_wired)
}

pub fn check_agent_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<crate::output_report::taxonomy_result_vo::LintResult>,
) {
    let stem = basename.replace(".rs", "").replace(".py", "");
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
    let mut wired = false;
    for cf in files {
        let cb = cf.split('/').next_back().unwrap_or("");
        let csuffix = cb
            .rsplit('_')
            .next()
            .unwrap_or("")
            .replace(".rs", "")
            .replace(".py", "");
        if csuffix != "container" && csuffix != "aggregate" && csuffix != "registry" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&stem) || c.contains(&format!("mod {}", stem)) || c.contains(&pascal_stem)
            {
                wired = true;
                break;
            }
        }
    }
    if !wired {
        violations.push(crate::orphan_detector::mk_orphan_result(
            fp,
            &format!("agent '{}' not wired in container.", stem),
            Severity::HIGH,
        ));
    }
}

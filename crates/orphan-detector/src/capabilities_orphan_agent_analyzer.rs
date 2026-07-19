// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
// Agent is orphan if the contract aggregate it implements is NOT called by any surface or container.
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, IOrphanFileCachePort,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;
use std::sync::OnceLock;

// ═══════════════════════════════════════════════════════════════════════════════
// STATIC REGEXES
// ═══════════════════════════════════════════════════════════════════════════════

fn re_impl_generic() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"impl\s*(?:<[^>]+>)?\s+([A-Za-z0-9_]+)\s+for\s+").ok())
        .as_ref()
}

fn re_dyn() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok())
        .as_ref()
}

fn re_py_class() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
}

fn re_ts_implements() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\s+implements\s+(\w+)").ok())
        .as_ref()
}

// ─── Block 1: Struct Definition ───────────────────────────
pub struct AgentOrphanAnalyzer {
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        self.check_agent_orphan(f, all_files)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self {
            cache: Arc::new(crate::infrastructure_file_cache::OrphanFileCache::new()),
        }
    }
}

impl AgentOrphanAnalyzer {
    pub fn new(cache: Arc<dyn IOrphanFileCachePort>) -> Self {
        Self { cache }
    }

    fn check_agent_orphan(&self, f: &FilePath, all_files: &[FilePath]) -> OrphanIndicatorResult {
        let content = self.cache.read_cached(f).value;
        if content.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let aggregate_traits = extract_aggregate_traits(&content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let mut any_called = false;
        for agg_name in &aggregate_traits {
            for cf in all_files {
                let cb = match cf.value().split('/').next_back() {
                    Some(b) => b,
                    None => continue,
                };
                let is_surface = cb.starts_with("surface_");
                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");

                if !is_surface && !is_container {
                    continue;
                }
                let cf_content = self.cache.read_cached(cf).value;
                if cf_content.contains(agg_name) {
                    any_called = true;
                    break;
                }
            }
            if any_called {
                break;
            }
        }

        if !any_called {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregates [{}] not called by any surface.",
                            aggregate_traits.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn extract_aggregate_traits(content: &str) -> Vec<String> {
    let mut traits = Vec::new();

    if let Some(re) = re_impl_generic() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    if let Some(re) = re_dyn() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    if let Some(re) = re_py_class() {
        for cap in re.captures_iter(content) {
            for part in cap[1].split(',') {
                let name = part.trim().to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }
    }

    if let Some(re) = re_ts_implements() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    traits.sort();
    traits.dedup();
    traits
}

pub fn check_agent_orphan(
    fp: &str,
    _basename: &str,
    files: &[FilePath],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    cache: &Arc<dyn IOrphanFileCachePort>,
) {
    let fp_vo = match FilePath::new(fp.to_string()) {
        Ok(p) => p,
        Err(_) => return,
    };
    let analyzer = AgentOrphanAnalyzer::new(cache.clone());
    let result = analyzer.check_agent_orphan(&fp_vo, files);
    if result.is_orphan {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
            "AES505",
        ));
    }
}

// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, IOrphanFileCachePort,
};
use shared::orphan_detector::taxonomy_agent_regex_utility::extract_aggregate_traits;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;

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

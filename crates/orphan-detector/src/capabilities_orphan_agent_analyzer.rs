// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

use regex::Regex;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match shared::orphan_detector::utility_orphan_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
            },
            c => c,
        };

        // Step 1: Find aggregate traits this agent implements
        let aggregate_traits = Self::extract_aggregate_traits(&content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Bug 2 fix: agent is orphan only if ALL aggregates are uncalled (not ANY)
        let mut any_called = false;
        for agg_name in &aggregate_traits {
            for cf in all_files {
                let cb = match cf.split('/').next_back() {
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
                let c = shared::orphan_detector::utility_orphan_io::read_file_safe(cf);
                if c.contains(agg_name) {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract aggregate trait names from agent file content.
    /// Looks for: impl IAggregateTrait for Struct, Box<dyn IAggregateTrait>, Arc<dyn IAggregateTrait>
    fn extract_aggregate_traits(content: &str) -> Vec<String> {
        let mut traits = Vec::new();

        // Rust: impl ITrait for Struct (with optional generics: impl<T> Trait for Struct)
        if let Some(re) = Self::re_impl_generic() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Rust: Box<dyn ITrait> or Arc<dyn ITrait>
        if let Some(re) = Self::re_dyn() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Python: class Struct(ITrait):
        if let Some(re) = Self::re_py_class() {
            for cap in re.captures_iter(content) {
                for part in cap[1].split(',') {
                    let name = part.trim().to_string();
                    if name.contains("Aggregate") || name.ends_with("Aggregate") {
                        traits.push(name);
                    }
                }
            }
        }

        // JS/TS: class Struct implements IAggregateTrait
        if let Some(re) = Self::re_ts_implements() {
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

    /// Cached regex for Rust impl with optional generics (Bug 12: impl<T> Trait for Struct)
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
}

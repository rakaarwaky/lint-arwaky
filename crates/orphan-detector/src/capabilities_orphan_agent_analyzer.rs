// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files.
// AST-based: uses parser dispatch for aggregate trait extraction.

use shared::code_analysis::OrphanIndicatorResult;
use shared::common::{FilePath, Severity};
use shared::orphan_detector::IOrphanParserProtocol;
use shared::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_detector::{AesOrphanViolation, IAgentOrphanProtocol};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {
    pub parser_dispatcher: Arc<dyn IOrphanParserProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match filesystem::utility_filesystem_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
            c => c,
        };

        // AST-based aggregate trait extraction
        let aggregate_traits = self.extract_aggregate_traits(fp, &content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Pre-filter candidate files (surfaces, containers, entries, mains)
        let candidates: Vec<&String> = all_files
            .iter()
            .filter(|cf| {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => return false,
                };
                cb.starts_with("surface_")
                    || cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js")
                    || cb.ends_with("_entry.rs")
                    || cb.ends_with("_entry.py")
                    || cb.ends_with("_entry.ts")
                    || cb.ends_with("_entry.js")
                    || cb == "main.rs"
                    || cb == "main.py"
                    || cb == "main.ts"
                    || cb == "main.js"
            })
            .collect();

        // Pass 1: Word-boundary search across candidate files
        let is_referenced = candidates.iter().any(|cf| {
            let candidate_content = filesystem::utility_filesystem_io::read_file_safe(cf);
            aggregate_traits
                .iter()
                .any(|t| Self::content_contains_word(&candidate_content, t))
        });

        if !is_referenced {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        "Agent file aggregate trait is not used by any surface, container, entry, or main file."
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
        Self::new(Arc::new(
            crate::capabilities_orphan_parser_dispatcher::OrphanParserDispatcher::new(),
        ))
    }
}

impl AgentOrphanAnalyzer {
    pub fn new(parser_dispatcher: Arc<dyn IOrphanParserProtocol>) -> Self {
        Self { parser_dispatcher }
    }

    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    /// Extract aggregate trait names using AST parser dispatch.
    /// Replaces 4 regex patterns (re_impl_generic, re_dyn, re_py_class, re_ts_implements).
    fn extract_aggregate_traits(&self, file_path: &str, content: &str) -> Vec<String> {
        let mut traits = match self.parser_dispatcher.parse_file(file_path, content) {
            FileParseResultVO::Rust(result) => result.aggregate_trait_names(),
            FileParseResultVO::Python(result) => result.aggregate_names(),
            FileParseResultVO::TypeScript(result) => result.aggregate_names(),
            FileParseResultVO::Unsupported => Vec::new(),
        };
        traits.sort();
        traits.dedup();
        traits
    }
}

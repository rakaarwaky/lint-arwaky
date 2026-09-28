// PURPOSE: AES405 P7 — agent that computes totals instead of orchestrating
//
// Fixture for `check_agent_computation`: the agent aggregates a total across
// results, which belongs in a capability, not in an orchestrator.
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::sync::Arc;

pub struct ComputationAgent {
    scanner: Arc<dyn IFileScanProtocol>,
    reporter: Arc<dyn IReportProtocol>,
}

impl IComputationAggregate for ComputationAgent {
    fn execute(&self, files: &[FileEntry]) -> usize {
        let hits: Vec<usize> = files
            .iter()
            .map(|f| self.scanner.scan(f).len())
            .collect();
        // AES405: an agent routes between subsystems; it does not total results.
        hits.iter().sum::<usize>()
    }
}

impl ComputationAgent {
    pub fn new(
        scanner: Arc<dyn IFileScanProtocol>,
        reporter: Arc<dyn IReportProtocol>,
    ) -> Self {
        Self { scanner, reporter }
    }
}

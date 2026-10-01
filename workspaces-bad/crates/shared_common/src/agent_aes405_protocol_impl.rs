// PURPOSE: AES405 P14 — agent that implements contract protocols itself
//
// Fixture for `check_agent_single_aggregate`: an agent is the feature's
// composition root — it implements the feature aggregate and injects protocol
// seams. Implementing a protocol here makes the orchestration layer duplicate a
// capability's work, so both impls below are violations. Each belongs in a
// `capabilities_*` file that the agent then delegates to.
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::sync::Arc;

pub struct ProtocolImplAgent {
    scanner: Arc<dyn IFileScanProtocol>,
    reporter: Arc<dyn IReportProtocol>,
}

// AES405: the agent may fulfil the feature aggregate and nothing else.
impl IProtocolImplAggregate for ProtocolImplAgent {
    fn execute(&self, files: &[FileEntry]) -> usize {
        self.scanner.scan(files).len()
    }
}

// AES405: belongs in `capabilities_file_scanner`, not in the agent.
impl IFileScanProtocol for ProtocolImplAgent {
    fn scan(&self, files: &[FileEntry]) -> Vec<FileEntry> {
        files.to_vec()
    }
}

// AES405: belongs in `capabilities_file_reporter`, not in the agent.
impl IReportProtocol for ProtocolImplAgent {
    fn report(&self) -> String {
        String::new()
    }
}

impl ProtocolImplAgent {
    pub fn new(scanner: Arc<dyn IFileScanProtocol>, reporter: Arc<dyn IReportProtocol>) -> Self {
        Self { scanner, reporter }
    }
}
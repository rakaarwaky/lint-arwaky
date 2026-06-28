/// Messages sent from a background scan thread to the TUI event loop.
#[derive(Debug, Clone)]
pub enum ScanUpdate {
    /// Periodic progress report during the scan.
    Progress {
        phase: String,
        done: usize,
        total: usize,
    },
    /// Scan completed — carry the final result.
    Complete {
        output: String,
        violation_count: usize,
        success: bool,
    },
}

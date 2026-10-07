// PURPOSE: WalkerReporter — real-time feed of what the walker sees.
// Emits enter/skip/discovered events with reasons so a developer
// watching `scan -v` sees exactly what's being read and why dirs
// are skipped.

use shared_logging::contract_logging_protocol::IWalkerReportProtocol;
use shared_logging::taxonomy_logging_vo::{Count, DurationMs, SkipReason};
use tracing::info;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct WalkerReporter {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IWalkerReportProtocol for WalkerReporter {
    fn walker_enter(&self, dir: &str) {
        info!(target: "lint_arwaky::audit", event = "walker_enter", dir = dir, "entering directory");
    }

    fn walker_skip(&self, dir: &str, reason: SkipReason) {
        let reason_str = reason.as_str();
        info!(target: "lint_arwaky::audit", event = "walker_skip", dir = dir, reason = reason_str, "skipping directory");
    }

    fn files_discovered(&self, count: Count, elapsed_ms: DurationMs) {
        info!(target: "lint_arwaky::audit", event = "files_discovered", count = count.value, elapsed_ms = elapsed_ms.value, "walk complete");
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl WalkerReporter {
    pub fn new() -> Self {
        Self {}
    }
}

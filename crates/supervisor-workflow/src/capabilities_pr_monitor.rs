// PURPOSE: Mock PR CI monitor + merger — seedable status sequence, no network.
//
// `MockPrMonitor` implements `IPrMonitorProtocol` by cycling through a stored
// `Vec<CIStatus>`: each call to `check_ci_status` pops the next status off a
// rotating cursor (wrapping around when exhausted, so it can be reused by
// multiple PRs in one cycle). Tests seed the sequence to simulate flaky CI —
// e.g. `[Failing, Failing, Passing]` models "two red polls, then green".

use shared_supervisor::contract_supervisor_protocol::IPrMonitorProtocol;
use shared_supervisor::taxonomy_supervisor_vo::CIStatus;
use shared_supervisor::taxonomy_supervisor_vo::PrInfo;
use shared_supervisor::taxonomy_supervisor_vo::SupervisorError;
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

/// Mock `IPrMonitorProtocol` with a caller-seeded CI status sequence.
///
/// Design: the sequence is consumed in order, one status per
/// `check_ci_status` call, and wraps to index 0 once the tail is reached —
/// so a seed of `[Failing, Failing, Passing]` yields Failing, Failing,
/// Passing, then Failing, Failing, ... for subsequent polls/PRs. This
/// models "re-run the flaky CI until it goes green" without a real Checks
/// API, and stays deterministic across runs (no sleep, no real clock).
pub struct MockPrMonitor {
    statuses: Mutex<Vec<CIStatus>>,
    cursor: Mutex<usize>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IPrMonitorProtocol for MockPrMonitor {
    fn check_ci_status(&self, _pr: &PrInfo) -> Result<CIStatus, SupervisorError> {
        let statuses = self
            .statuses
            .lock()
            .map_err(|_| SupervisorError::PrMonitor("poisoned".to_string()))?;
        if statuses.is_empty() {
            return Ok(CIStatus::Unknown);
        }
        let mut cursor = self
            .cursor
            .lock()
            .map_err(|_| SupervisorError::PrMonitor("poisoned".to_string()))?;
        let idx = *cursor % statuses.len();
        // Advance the shared cursor so the next call moves on.
        *cursor = cursor.wrapping_add(1);
        Ok(statuses[idx])
    }

    /// Mock merge: no-op, records nothing. The orchestrator is responsible
    /// for recording `merged = true` in its own report — the mock's only
    /// job is to prove the call path exists.
    fn merge_pr(&self, pr: &PrInfo) -> Result<(), SupervisorError> {
        if pr.state != "open" {
            return Err(SupervisorError::Merge(format!(
                "cannot merge PR #{}, state={}",
                pr.number, pr.state
            )));
        }
        Ok(())
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl MockPrMonitor {
    /// Seed the monitor with an explicit per-poll status sequence.
    pub fn with_statuses(statuses: Vec<CIStatus>) -> Self {
        Self {
            statuses: Mutex::new(statuses),
            cursor: Mutex::new(0),
        }
    }

    /// A monitor that always reports `status` (sequence of one, wrapping).
    pub fn constant(status: CIStatus) -> Self {
        Self::with_statuses(vec![status])
    }

    /// Number of statuses in the seed sequence (for test assertions).
    pub fn seed_len(&self) -> usize {
        self.statuses.lock().map_or(0, |g| g.len())
    }
}

impl Default for MockPrMonitor {
    fn default() -> Self {
        Self::constant(CIStatus::Passing)
    }
}

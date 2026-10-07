// PURPOSE: PhaseTimer — timing scan phases and emitting structured events.
// One capability seam that wraps any closure with elapsed_ms reporting.

use shared_logging::contract_logging_protocol::IPhaseTimerProtocol;
use shared_logging::taxonomy_logging_vo::PhaseTimerVO;
use tracing::info;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct PhaseTimerCapability {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IPhaseTimerProtocol for PhaseTimerCapability {
    fn phase_started(&self, phase: &'static str) -> PhaseTimerVO {
        PhaseTimerVO {
            phase,
            start: std::time::Instant::now(),
        }
    }

    fn phase_finished(&self, timer: &PhaseTimerVO, count: Option<usize>) {
        let elapsed_ms = timer.start.elapsed().as_millis() as u64;
        match count {
            Some(n) => {
                info!(target: "lint_arwaky::audit", event = "phase_done", phase = timer.phase, elapsed_ms = elapsed_ms, count = n, "{:?} phase complete", timer.phase);
            }
            None => {
                info!(target: "lint_arwaky::audit", event = "phase_done", phase = timer.phase, elapsed_ms = elapsed_ms, "{:?} phase complete", timer.phase);
            }
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl PhaseTimerCapability {
    pub fn new() -> Self {
        Self {}
    }
}

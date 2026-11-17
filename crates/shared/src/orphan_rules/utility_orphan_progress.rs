// PURPOSE: Progress and cancellation helpers for orphan-rules scanning.
// Used by: orphan-rules (ArchOrphanAnalyzer) for Ctrl+C cancellation and progress cadence.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

/// Progress cadence: emit at most one progress event per this many files.
pub const PROGRESS_INTERVAL: usize = 250;

/// Process-wide Ctrl+C flag, initialized once on first use. When `ctrlc`
/// receives SIGINT the flag is set; the orchestrator observes it between
/// phases and returns the findings produced so far with `partial: true`.
static CTRL_C_FLAG: OnceLock<Arc<AtomicBool>> = OnceLock::new();

/// Return the process-wide Ctrl+C cancellation flag, initializing the
/// signal handler on first call. If another handler already owns SIGINT
/// (tests, embeds), the flag stays usable via the OnceLock; cancellation
/// just never fires from the signal.
pub fn ctrl_c_flag() -> Arc<AtomicBool> {
    CTRL_C_FLAG
        .get_or_init(|| {
            let flag = Arc::new(AtomicBool::new(false));
            let f = flag.clone();
            if ctrlc::set_handler(move || f.store(true, Ordering::SeqCst)).is_err() {
                // Another handler already owns SIGINT. The flag stays
                // usable via the OnceLock; cancellation just never fires
                // from the signal.
            }
            flag
        })
        .clone()
}

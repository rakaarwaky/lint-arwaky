// PURPOSE: Panic isolation for lint aggregate calls (utility layer).
//
// One buggy rule group must not take the whole scan or CI evaluation down
// (issue #575): every aggregate invocation in the dispatcher pipelines runs
// behind `catch_unwind`; a panic is converted into an AES999 marker violation
// that names the capability, and the remaining groups still report.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;

/// Marker code for "an aggregate panicked mid-run; results are partial".
pub const PANIC_MARKER_CODE: &str = "AES999";

/// Best-effort panic payload rendering for the marker message.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        format!(": {message}")
    } else if let Some(message) = payload.downcast_ref::<String>() {
        format!(": {message}")
    } else {
        String::new()
    }
}

fn marker_lint_result(capability: &'static str, target: &str, payload: Box<dyn std::any::Any + Send>) -> LintResult {
    LintResult::new_arch(
        target,
        0,
        PANIC_MARKER_CODE,
        Severity::HIGH,
        format!(
            "internal: {capability} aggregate panicked — partial results{}",
            panic_message(payload.as_ref())
        ),
    )
}

/// Run one rule group that yields `LintResult`s; a panic yields the AES999
/// marker instead, so the caller's other groups still report.
pub fn guarded_lint_results(
    capability: &'static str,
    target: &str,
    call: impl FnOnce() -> Vec<LintResult>,
) -> Vec<LintResult> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(call)) {
        Ok(violations) => violations,
        Err(payload) => vec![marker_lint_result(capability, target, payload)],
    }
}

/// `ViolationItem`-shaped variant of [`guarded_lint_results`].
pub fn guarded_items(
    capability: &'static str,
    target: &str,
    call: impl FnOnce() -> Vec<ViolationItem>,
) -> Vec<ViolationItem> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(call)) {
        Ok(items) => items,
        Err(payload) => {
            vec![ViolationItem::from_lint_result(&marker_lint_result(
                capability, target, payload,
            ))]
        }
    }
}

/// Generic guard for non-list results (scores, flags, reports): the caller
/// decides the fail-safe fallback — CI, for instance, scores a panicked
/// evaluation as zero so the gate fails closed.
pub fn run_guarded<T>(
    capability: &'static str,
    target: &str,
    call: impl FnOnce() -> T,
) -> Result<T, ViolationItem> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(call)).map_err(|payload| {
        ViolationItem::from_lint_result(&marker_lint_result(capability, target, payload))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panicked_group_yields_marker() {
        let out = guarded_lint_results("quality", "src/a.rs", || panic!("boom"));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].code.code(), "AES999");
        assert!(out[0].message.value().contains("quality"));
        assert!(out[0].message.value().contains("boom"));
    }

    #[test]
    fn healthy_group_passes_through() {
        let out = guarded_lint_results("quality", "src/a.rs", || Vec::new());
        assert!(out.is_empty());
    }

    #[test]
    fn generic_guard_reports_the_capability() {
        let err = run_guarded::<f64>("score", ".", || panic!("bad math")).unwrap_err();
        assert_eq!(err.code.code(), "AES999");
    }
}

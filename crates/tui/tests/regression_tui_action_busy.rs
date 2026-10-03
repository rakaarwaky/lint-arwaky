//! Regression coverage for #565: background actions surface a visible busy
//! state, and action keypresses during a pending action are explicitly
//! rejected ("Busy: … — please wait") instead of silently swallowed.
#[path = "../../shared/tests/common/mock_filesystem.rs"]
#[allow(dead_code, unused_imports)]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;

use dispatcher::surface_check_action::FilesystemSeam;
use dispatcher::surface_orphan_action::OrphanFactory;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_orphan_rules::{IOrphanAggregate, OrphanRequest, OrphanResponse};
use shared_quality_rules::{
    ICodeAnalysisAggregate, taxonomy_quality_rules_request::CodeAnalysisRequest,
    taxonomy_quality_rules_response::CodeAnalysisResponse,
};
use std::sync::Arc;
use tui_lint_arwaky::surface_event_action::SurfaceActionHandler;
use tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor;
use tui_lint_arwaky::taxonomy_tui_event::TuiEvent;
use tui_lint_arwaky::taxonomy_tui_vo::{AppState, BusySlot};

// Inert code-analysis aggregate (never meaningfully invoked).
struct NoopQuality;
impl ICodeAnalysisAggregate for NoopQuality {
    fn execute(&self, _request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        CodeAnalysisResponse::Analysis {
            violations: Vec::new(),
        }
    }
}

// Inert orphan aggregate (never invoked).
struct NoopOrphan;
impl IOrphanAggregate for NoopOrphan {
    fn execute(&self, _request: OrphanRequest) -> OrphanResponse {
        OrphanResponse::ScanOutcome {
            context: Default::default(),
            violations: Vec::new(),
        }
    }
}

/// A fully wired (but inert) executor for busy-state tests.
fn executor() -> Arc<SurfaceLintExecutor> {
    let mock = Arc::new(MockFilesystem::new());
    let io: Arc<dyn IFileSystemIOProtocol> = mock.clone();
    let code_analysis: Arc<dyn ICodeAnalysisAggregate> = Arc::new(NoopQuality);
    let seam = Arc::new(FilesystemSeam {
        io: mock.clone(),
        workspace: mock.clone(),
        parser: mock.clone(),
        aggregate: mock.clone(),
    });
    let closure_mock = mock.clone();
    let fs_factory: Arc<dyn Fn() -> FilesystemSeam + Send + Sync> =
        Arc::new(move || FilesystemSeam {
            io: closure_mock.clone(),
            workspace: closure_mock.clone(),
            parser: closure_mock.clone(),
            aggregate: closure_mock.clone(),
        });
    let orphan_factory: Arc<OrphanFactory> =
        Arc::new(move |_config, _fs, _workspace| Arc::new(NoopOrphan));
    Arc::new(SurfaceLintExecutor::new(
        code_analysis,
        mock.clone(),
        io,
        mock.clone(),
        mock.clone(),
        seam,
        fs_factory,
        orphan_factory,
    ))
}

fn handler() -> SurfaceActionHandler {
    SurfaceActionHandler::new(executor(), Arc::new(MockFilesystem::new()))
}

fn fresh_state() -> AppState {
    AppState::new("/nonexistent".to_string())
}

// ─── Busy indicator state ────────────────────────────────────────────

/// While a background action is in flight, `actions.pending_label` carries the
/// label the status bar renders next to the "Running" busy prefix (#565).
#[test]
fn pending_action_records_its_label() {
    let mut state = fresh_state();
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    state.actions.pending_label = "doctor".to_string();
    assert!(state.actions.pending);
    assert_eq!(state.actions.pending_label, "doctor");
    assert_eq!(state.busy_slot_holder(), Some(BusySlot::Action));
}

// ─── Key press during a pending action ───────────────────────────────

/// Pressing an action key while a background action is pending produces an
/// explicit busy status instead of a silent no-op (#565).
fn keypress_during_pending_action(action: TuiEvent) {
    let h = handler();
    let mut state = fresh_state();
    // Simulate a pending background action (label recorded as the status bar
    // shows it): the busy slot is held and the label is set.
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    state.actions.pending_label = "install".to_string();
    state.set_status("Running install...");

    // Press another action key while pending.
    h.handle(&mut state, action);

    // The keypress was explicitly rejected with a busy status, not silently
    // swallowed: the status line names the in-flight holder.
    assert_eq!(state.status_message, "Busy: action is running");
    // The pending action is untouched: still holding the slot.
    assert!(state.actions.pending);
    assert_eq!(state.actions.pending_label, "install");
}

#[test]
fn action_check_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionCheck);
}

#[test]
fn action_scan_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionScan);
}

#[test]
fn action_doctor_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionDoctor);
}

#[test]
fn action_mcp_config_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionMcpConfig);
}

#[test]
fn action_install_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionInstall);
}

#[test]
fn action_init_during_pending_action_reports_busy() {
    keypress_during_pending_action(TuiEvent::ActionInit);
}

/// The rejected keypress must NOT start a second background action: the
/// single busy slot (#577) keeps exactly one operation in flight.
#[test]
fn rejected_keypress_does_not_double_claim_the_slot() {
    let h = handler();
    let mut state = fresh_state();
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    state.actions.pending_label = "doctor".to_string();

    // The scan path is the only other slot claimant; it must stay declined.
    let scan_rx = h.start_scan(&mut state);
    assert!(scan_rx.is_none());
    assert_eq!(state.status_message, "Busy: action is running");
    // The declined scan must not have started: the holder is still the action.
    assert!(!state.scan.running);
    assert_eq!(state.busy_slot_holder(), Some(BusySlot::Action));
}

/// Once the pending action completes (slot freed), action keys work again —
/// the busy gate is not a permanent lock.
#[test]
fn action_keys_work_again_after_pending_action_finishes() {
    let h = handler();
    let mut state = fresh_state();
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    state.actions.pending_label = "doctor".to_string();

    // Simulate the background worker finishing: clear the pending flag.
    state.actions.pending = false;
    assert_eq!(state.busy_slot_holder(), None);

    // A new action keypress now passes the busy gate (no rejection status).
    h.handle(&mut state, TuiEvent::ActionAdapters);
    assert_ne!(state.status_message, "Busy: action is running");
}

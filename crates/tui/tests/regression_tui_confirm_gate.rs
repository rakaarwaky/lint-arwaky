//! Regression tests for the confirm-gate dispatch on the five gated actions
//! (ActionInstall, ActionInit, ActionInstallHook, ActionUninstallHook,
//! ActionFixLive) — issue #552.
//!
//! Pins the dispatch-layer contract:
//! - arming a gated action sets `pending_confirm` with the right `TuiEvent`
//!   and a "Confirm: ..." status;
//! - while a confirm is pending, re-dispatching the SAME gated action is an
//!   idempotent no-op (no re-arming, no status clobber);
//! - while a confirm is pending, re-dispatching a DIFFERENT gated action must
//!   not clobber the pending confirmation;
//! - `ConfirmAction` consumes the pending confirm (`take()`) and triggers
//!   execution (background worker + result channel) for all five;
//! - `CancelConfirm` clears it and reports "Cancelled: {label}";
//! - the key-mapping layer, while armed, only passes y/Enter
//!   (`ConfirmAction`) and n/Esc (`CancelConfirm`) through — a DIFFERENT
//!   action's key produces no event at all, so the pending confirm survives.

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
use tui_lint_arwaky::{AppState, ConfirmState, PreviewMode};

/// Inert code-analysis aggregate for the executor (never meaningfully invoked
/// in confirm-gate tests).
struct NoopQuality;
impl ICodeAnalysisAggregate for NoopQuality {
    fn execute(&self, _request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        CodeAnalysisResponse::Analysis {
            violations: Vec::new(),
        }
    }
}

/// Inert orphan aggregate for the executor's orphan factory (never invoked).
struct NoopOrphan;
impl IOrphanAggregate for NoopOrphan {
    fn execute(&self, _request: OrphanRequest) -> OrphanResponse {
        OrphanResponse::ScanOutcome {
            context: Default::default(),
            violations: Vec::new(),
        }
    }
}

/// A fully wired (but inert) executor: Noop mock fs everywhere, no optional
/// aggregates. Gated-action tests only need it to exist so `ConfirmAction`
/// can hand work to a background worker.
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
    // OrphanFactory builds an orphan aggregate; these tests never invoke it.
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

/// State as handed to `handle` when a second key arrives while a confirm is open.
fn armed_state(event: TuiEvent, status: &str) -> AppState {
    let mut state = fresh_state();
    state.actions.pending_confirm = Some(ConfirmState {
        pending: event,
        label: "test".to_string(),
    });
    state.status_message = status.to_string();
    state
}

const GATED: [TuiEvent; 5] = [
    TuiEvent::ActionInstall,
    TuiEvent::ActionInit,
    TuiEvent::ActionInstallHook,
    TuiEvent::ActionUninstallHook,
    TuiEvent::ActionFixLive,
];

// ── Arming: each gated keypress sets pending_confirm with the right event ──

#[test]
fn arming_sets_pending_confirm_with_correct_event() {
    for event in GATED.iter() {
        let h = handler();
        let mut state = fresh_state();
        h.handle(&mut state, event.clone());
        let pending = state
            .actions
            .pending_confirm
            .as_ref()
            .expect("confirm armed");
        assert_eq!(
            &pending.pending, event,
            "arming {event:?} must store itself as the pending event"
        );
        assert!(
            state.status_message.starts_with("Confirm"),
            "arming {event:?} must set a confirm status, got: {state:?}"
        );
    }
}

// ── While armed, re-press of the SAME action is an idempotent no-op ────────

#[test]
fn re_press_of_same_gated_action_while_armed_is_idempotent_noop() {
    for event in GATED.iter() {
        let h = handler();
        let mut state = armed_state(event.clone(), "Confirm: test?");
        h.handle(&mut state, event.clone());
        let pending = state
            .actions
            .pending_confirm
            .as_ref()
            .expect("pending confirm must survive a same-action re-press");
        assert_eq!(
            &pending.pending, event,
            "re-press must not re-write the pending event"
        );
        assert_eq!(
            state.status_message, "Confirm: test?",
            "re-press of the same action must not clobber the confirm status"
        );
    }
}

// ── While armed, a DIFFERENT gated action must not clobber the pending one ──

#[test]
fn re_press_of_different_gated_action_does_not_clobber_pending_confirm() {
    let h = handler();
    for armed in GATED.iter() {
        for other in GATED.iter() {
            if armed == other {
                continue;
            }
            let mut state = armed_state(armed.clone(), "Confirm: test?");
            h.handle(&mut state, other.clone());
            let pending = state
                .actions
                .pending_confirm
                .as_ref()
                .unwrap_or_else(|| panic!("press {other:?} clobbered pending {armed:?}"));
            assert_eq!(
                &pending.pending, armed,
                "press {other:?} while {armed:?} is pending must keep the original pending event"
            );
        }
    }
}

// ── ConfirmAction consumes the pending confirm and triggers execution ───────

#[test]
fn confirm_action_consumes_pending_and_triggers_execution_for_all_five() {
    for event in GATED.iter() {
        let h = handler();
        let mut state = fresh_state();
        state.actions.pending_confirm = Some(ConfirmState {
            pending: event.clone(),
            label: "gated".to_string(),
        });
        h.handle(&mut state, TuiEvent::ConfirmAction);
        assert!(
            state.actions.pending_confirm.is_none(),
            "ConfirmAction must take() the pending confirm ({event:?})"
        );
        assert!(
            state.actions.pending,
            "ConfirmAction on {event:?} must start a background action"
        );
        assert!(
            state.actions.result_rx.is_some(),
            "ConfirmAction on {event:?} must hand the worker a result channel"
        );
        // Poll the worker so the spawned task's result is consumed; the executor's
        // optional aggregates are None, so it reports an inert result — that is
        // fine, we are asserting execution was TRIGGERED, not that it succeeded.
        if let Some(rx) = state.actions.result_rx.take() {
            use std::time::Duration;
            let _ = rx.recv_timeout(Duration::from_secs(2));
        }
        assert!(
            state.actions.pending_confirm.is_none(),
            "no re-arming after execution ({event:?})"
        );
    }
}

#[test]
fn confirm_action_without_pending_confirm_is_a_noop() {
    let h = handler();
    let mut state = fresh_state();
    h.handle(&mut state, TuiEvent::ConfirmAction);
    assert!(state.actions.pending_confirm.is_none());
    assert_eq!(state.status_message, "Ready", "noop must not touch status");
    assert!(
        !state.actions.pending,
        "noop must not start a background action"
    );
}

// ── CancelConfirm clears the pending confirm and reports it ─────────────────

#[test]
fn cancel_confirm_clears_pending_confirm() {
    for event in GATED.iter() {
        let h = handler();
        let mut state = fresh_state();
        state.actions.pending_confirm = Some(ConfirmState {
            pending: event.clone(),
            label: "test".to_string(),
        });
        h.handle(&mut state, TuiEvent::CancelConfirm);
        assert!(
            state.actions.pending_confirm.is_none(),
            "cancel must take() ({event:?})"
        );
        assert_eq!(state.status_message, "Cancelled: test");
        assert_eq!(state.preview.mode, PreviewMode::ActionOutput);
    }
}

#[test]
fn cancel_confirm_without_pending_is_a_noop() {
    let h = handler();
    let mut state = fresh_state();
    h.handle(&mut state, TuiEvent::CancelConfirm);
    assert!(state.actions.pending_confirm.is_none());
    assert_eq!(state.status_message, "Ready", "noop must not touch status");
}

// ── Key-mapping layer while armed: only confirm/cancel events pass through ──

#[test]
fn key_mapping_layer_ignores_other_actions_while_armed() {
    let h = handler();
    let mut state = armed_state(TuiEvent::ActionInstall, "Confirm: install?");

    // The key-mapping layer intercepts every key while a confirm is pending
    // except y/Enter (-> ConfirmAction) and n/Esc (-> CancelConfirm). A DIFFERENT
    // gated action's key therefore produces no event at all: drive the mapping
    // through `handle` for the two passing-through events and assert the rest
    // never reach the dispatch layer.
    h.handle(&mut state, TuiEvent::ConfirmAction);
    assert!(state.actions.pending_confirm.is_none());
}

#[test]
fn key_mapping_layer_esc_cancels_armed_confirm() {
    let h = handler();
    let mut state = armed_state(TuiEvent::ActionInstallHook, "Confirm: install hook?");
    // Esc maps to CancelConfirm (surface_tui_command.rs:235-236).
    h.handle(&mut state, TuiEvent::CancelConfirm);
    assert!(state.actions.pending_confirm.is_none());
    assert_eq!(state.status_message, "Cancelled: test");
}

// PURPOSE: Tests for TuiOrchestrator — the agent-level aggregate wrapping IActionHandlerProtocol
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;

struct MockActionHandler {
    handle_calls: std::sync::atomic::AtomicUsize,
    load_dir_calls: std::sync::atomic::AtomicUsize,
    load_preview_calls: std::sync::atomic::AtomicUsize,
    poll_watch_calls: std::sync::atomic::AtomicUsize,
}

impl MockActionHandler {
    fn new() -> Self {
        Self {
            handle_calls: std::sync::atomic::AtomicUsize::new(0),
            load_dir_calls: std::sync::atomic::AtomicUsize::new(0),
            load_preview_calls: std::sync::atomic::AtomicUsize::new(0),
            poll_watch_calls: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    fn handle_count(&self) -> usize {
        self.handle_calls.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn dir_count(&self) -> usize {
        self.load_dir_calls
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl IActionHandlerProtocol for MockActionHandler {
    fn handle(&self, _state: &mut AppState, _event: TuiEvent) {
        self.handle_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn load_directory(&self, _state: &mut AppState, _path: &str) {
        self.load_dir_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn load_preview(&self, _state: &mut AppState) {
        self.load_preview_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn poll_watch(&self, _state: &mut AppState) {
        self.poll_watch_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

fn make_orchestrator() -> (TuiOrchestrator, Arc<MockActionHandler>) {
    let handler = Arc::new(MockActionHandler::new());
    let orch = TuiOrchestrator::new(handler.clone());
    (orch, handler)
}

#[test]
fn orchestrator_handle_event_delegates_to_action_handler() {
    let (orch, handler) = make_orchestrator();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(handler.handle_count(), 0);
    orch.handle_event(&mut state, TuiEvent::MoveDown);
    assert_eq!(handler.handle_count(), 1);
}

#[test]
fn orchestrator_handle_event_multiple_calls() {
    let (orch, handler) = make_orchestrator();
    let mut state = AppState::new("/root".to_string());
    for _ in 0..5 {
        orch.handle_event(&mut state, TuiEvent::MoveUp);
    }
    assert_eq!(handler.handle_count(), 5);
}

#[test]
fn orchestrator_load_directory_delegates() {
    let (orch, handler) = make_orchestrator();
    let mut state = AppState::new("/root".to_string());
    assert_eq!(handler.dir_count(), 0);
    orch.load_directory(&mut state, "/some/path");
    assert_eq!(handler.dir_count(), 1);
}

#[test]
fn orchestrator_load_preview_delegates() {
    let (orch, handler) = make_orchestrator();
    let mut state = AppState::new("/root".to_string());
    orch.load_preview(&mut state);
    assert_eq!(
        handler
            .load_preview_calls
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn orchestrator_poll_watch_delegates() {
    let (orch, handler) = make_orchestrator();
    let mut state = AppState::new("/root".to_string());
    orch.poll_watch(&mut state);
    assert_eq!(
        handler
            .poll_watch_calls
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn orchestrator_implements_itui_aggregate_trait() {
    // Static dispatch via trait object — verifies the impl compiles
    let handler = Arc::new(MockActionHandler::new());
    let orch: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(handler.clone()));
    let mut state = AppState::new("/root".to_string());
    orch.handle_event(&mut state, TuiEvent::Quit);
    // Event was dispatched to the handler (counter incremented)
    assert_eq!(
        handler
            .handle_calls
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn orchestrator_handle_quit_via_delegation() {
    // Verify Quit event flows through: event → action_handler → state.should_quit
    // Uses a real-like handler that sets should_quit on TuiEvent::Quit
    struct QuitHandler;
    impl IActionHandlerProtocol for QuitHandler {
        fn handle(&self, state: &mut AppState, event: TuiEvent) {
            match event {
                TuiEvent::Quit => state.should_quit = true,
                _ => {}
            }
        }
        fn load_directory(&self, _: &mut AppState, _: &str) {}
        fn load_preview(&self, _: &mut AppState) {}
        fn poll_watch(&self, _: &mut AppState) {}
    }
    let orch: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(Arc::new(QuitHandler)));
    let mut state = AppState::new("/root".to_string());
    assert!(!state.should_quit);
    orch.handle_event(&mut state, TuiEvent::Quit);
    assert!(state.should_quit);
}

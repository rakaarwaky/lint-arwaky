use crate::tui::taxonomy_state_vo::AppState;
use crate::tui::taxonomy_tui_event::TuiEvent;

pub trait IActionHandlerProtocol: Send + Sync {
    fn handle(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &str);
    fn load_preview(&self, state: &mut AppState);
    /// Poll for file watch events and update state. Call every event loop tick.
    fn poll_watch(&self, state: &mut AppState);
}

use crate::common::taxonomy_path_vo::FilePath;
use crate::tui::taxonomy_scan_update_vo::ScanUpdate;
use crate::tui::taxonomy_state_vo::AppState;
use crate::tui::taxonomy_tui_event::TuiEvent;
use std::sync::mpsc::Receiver;

pub trait IActionHandlerProtocol: Send + Sync {
    fn handle(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &FilePath);
    fn load_preview(&self, state: &mut AppState);
    /// Poll for file watch events and update state. Call every event loop tick.
    fn poll_watch(&self, state: &mut AppState);
    /// Spawn a background scan thread and return a channel receiver for progress updates.
    /// Returns `None` if a scan is already in progress.
    fn start_scan(&self, state: &mut AppState) -> Option<Receiver<ScanUpdate>>;
    /// Non-blocking poll of the scan channel — drains all pending updates into state.
    fn poll_scan(&self, state: &mut AppState, rx: &Receiver<ScanUpdate>);
}

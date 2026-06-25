use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiDependencies {
    pub fs_port: Arc<dyn IFileSystemPort>,
    pub lint_port: Arc<dyn ILintExecutorProtocol>,
}

pub trait ITuiAggregate: Send + Sync {
    fn handle_event(&self, state: &mut AppState, event: TuiEvent);
    fn load_directory(&self, state: &mut AppState, path: &str);
    fn load_preview(&self, state: &mut AppState);
}

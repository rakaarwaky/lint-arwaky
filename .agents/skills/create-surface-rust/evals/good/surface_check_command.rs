use std::sync::Arc;

use shared::cli_commands::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::cli_commands::taxonomy_import_scan_request_vo::ImportScanRequest;
use shared::surface::taxonomy_surface_error::SurfaceError;
use shared::tui::taxonomy_tui_event_vo::TuiEvent;
use shared::tui::taxonomy_ui_state_vo::UiState;

pub struct CheckCommand {
    runner: Arc<dyn IImportRunnerAggregate>,
    request: ImportScanRequest,
}

impl CheckCommand {
    pub fn new(runner: Arc<dyn IImportRunnerAggregate>, request: ImportScanRequest) -> Self {
        Self { runner, request }
    }

    pub fn handle(&self, event: &TuiEvent) -> Result<UiState, SurfaceError> {
        match event {
            TuiEvent::RunCheck => {
                let report = self.runner.run(&self.request)
                    .map_err(SurfaceError::execution)?;
                Ok(UiState::from_report(report))
            }
            TuiEvent::Quit => Ok(UiState::exit()),
            _ => Ok(UiState::idle()),
        }
    }
}

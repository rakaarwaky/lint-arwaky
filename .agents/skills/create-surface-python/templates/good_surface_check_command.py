from shared.cli_commands.contract_import_runner_aggregate import IImportRunnerAggregate
from shared.cli_commands.taxonomy_import_scan_request_vo import ImportScanRequest
from shared.surface.taxonomy_surface_error import SurfaceError
from shared.tui.taxonomy_tui_event_vo import TuiEvent
from shared.tui.taxonomy_ui_state_vo import UiState


class CheckCommand:
    def __init__(self, runner: IImportRunnerAggregate, request: ImportScanRequest):
        self._runner = runner
        self._request = request

    def handle(self, event: TuiEvent) -> Result[UiState, SurfaceError]:
        if event == TuiEvent.RUN_CHECK:
            report = self._runner.run(self._request)
            return Ok(UiState.from_report(report))
        elif event == TuiEvent.QUIT:
            return Ok(UiState.exit())
        else:
            return Ok(UiState.idle())

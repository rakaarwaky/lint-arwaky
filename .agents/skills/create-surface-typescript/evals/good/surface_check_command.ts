import { IImportRunnerAggregate } from '../shared/cli_commands/contract_import_runner_aggregate';
import { ImportScanRequest } from '../shared/cli_commands/taxonomy_import_scan_request_vo';
import { SurfaceError } from '../shared/surface/taxonomy_surface_error';
import { TuiEvent } from '../shared/tui/taxonomy_tui_event_vo';
import { UiState } from '../shared/tui/taxonomy_ui_state_vo';

export class CheckCommand {
    constructor(
        private readonly runner: IImportRunnerAggregate,
        private readonly request: ImportScanRequest,
    ) {}

    handle(event: TuiEvent): Result<UiState, SurfaceError> {
        switch (event) {
            case TuiEvent.RUN_CHECK:
                const report = this.runner.run(this.request);
                return Ok(UiState.fromReport(report));
            case TuiEvent.QUIT:
                return Ok(UiState.exit());
            default:
                return Ok(UiState.idle());
        }
    }
}

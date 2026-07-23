# Examples

## GOOD: Smart Surface

```typescript
import { IImportRunnerAggregate } from "../shared/cli_commands/contract_import_runner_aggregate";
import { ImportScanRequest } from "../shared/cli_commands/taxonomy_import_scan_request_vo";
import { SurfaceError } from "../shared/surface/taxonomy_surface_error";
import { TuiEvent } from "../shared/tui/taxonomy_tui_event_vo";
import { UiState } from "../shared/tui/taxonomy_ui_state_vo";

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
```

## GOOD: Utility Surface

```typescript
import { KeyEvent } from "../shared/tui/taxonomy_key_event_vo";
import { ShortcutBindings } from "../shared/tui/taxonomy_shortcut_bindings_vo";
import { TuiAction } from "../shared/tui/taxonomy_tui_action_vo";

export class ShortcutHook {
  constructor(private readonly bindings: ShortcutBindings) {}

  mapKey(key: KeyEvent): TuiAction | null {
    return this.bindings.actionFor(key);
  }
}
```

## GOOD: Passive Surface

```typescript
import { RenderedText } from "../shared/tui/taxonomy_rendered_text_vo";
import { StatusViewModel } from "../shared/tui/taxonomy_status_view_model_vo";

export class StatusComponent {
  constructor(private readonly state: StatusViewModel) {}

  render(): RenderedText {
    return new RenderedText(`Status: ${this.state.status()}`);
  }
}
```

## BAD: Smart Surface Imports Capabilities

```typescript
import { MyChecker } from "../capabilities/my_checker"; // BAD

export class CheckCommand {
  constructor() {
    this._checker = new MyChecker();
  }
}
```

## BAD: Passive Surface Contains Business Logic

```typescript
export class StatusComponent {
  render(): string {
    if (this.state.violations().length > 10) {
      return "Too many violations";
    } else {
      return "OK";
    }
  }
}
```

## BAD: Utility Surface Imports Smart Surface

```typescript
import { CheckCommand } from "./surface_check_command"; // BAD

export class MyAction {
  constructor() {
    this._command = new CheckCommand();
  }
}
```

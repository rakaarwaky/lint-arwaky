# Examples

## GOOD: Smart Surface

```python
from shared.cli_commands.contract_import_runner_aggregate import IImportRunnerAggregate
from shared.cli_commands.taxonomy_import_scan_request_vo import ImportScanRequest
from.*capabilities_|from.*agent_|from.*surface_error import SurfaceError
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
```

## GOOD: Utility Surface

```python
from shared.tui.taxonomy_key_event_vo import KeyEvent
from shared.tui.taxonomy_shortcut_bindings_vo import ShortcutBindings
from shared.tui.taxonomy_tui_action_vo import TuiAction

class ShortcutHook:
    def __init__(self, bindings: ShortcutBindings):
        self._bindings = bindings

    def map_key(self, key: KeyEvent) -> TuiAction | None:
        return self._bindings.action_for(key)
```

## GOOD: Passive Surface

```python
from shared.tui.taxonomy_rendered_text_vo import RenderedText
from shared.tui.taxonomy_status_view_model_vo import StatusViewModel

class StatusComponent:
    def __init__(self, state: StatusViewModel):
        self._state = state

    def render(self) -> RenderedText:
        return RenderedText(f"Status: {self._state.status()}")
```

## BAD: Smart Surface Imports Capabilities

```python
from.*capabilities_|from.*agent_|from.*surface_my_checker import MyChecker  # BAD

class CheckCommand:
    def __init__(self):
        self._checker = MyChecker()
```

## BAD: Passive Surface Contains Business Logic

```python
class StatusComponent:
    def render(self) -> str:
        if len(self._state.violations()) > 10:
            return "Too many violations"
        else:
            return "OK"
```

## BAD: Utility Surface Imports Smart Surface

```python
from.*capabilities_|from.*agent_|from.*surface_check_command import CheckCommand  # BAD

class MyAction:
    def __init__(self):
        self._command = CheckCommand()
```

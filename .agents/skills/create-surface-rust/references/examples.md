# Examples

## GOOD: Smart Surface

```rust
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
```

## GOOD: Utility Surface

```rust
use shared::tui::taxonomy_key_event_vo::KeyEvent;
use shared::tui::taxonomy_shortcut_bindings_vo::ShortcutBindings;
use shared::tui::taxonomy_tui_action_vo::TuiAction;

pub struct ShortcutHook {
    bindings: ShortcutBindings,
}

impl ShortcutHook {
    pub fn new(bindings: ShortcutBindings) -> Self {
        Self { bindings }
    }

    pub fn map_key(&self, key: &KeyEvent) -> Option<TuiAction> {
        self.bindings.action_for(key)
    }
}
```

## GOOD: Passive Surface

```rust
use shared::tui::taxonomy_rendered_text_vo::RenderedText;
use shared::tui::taxonomy_status_view_model_vo::StatusViewModel;

pub struct StatusComponent {
    state: StatusViewModel,
}

impl StatusComponent {
    pub fn new(state: StatusViewModel) -> Self {
        Self { state }
    }

    pub fn render(&self) -> RenderedText {
        RenderedText::from(format!("Status: {}", self.state.status()))
    }
}
```

## BAD: Smart Surface Imports Capabilities

```rust
use crate::capabilities_my_checker::MyChecker; // BAD

pub struct CheckCommand {
    checker: MyChecker,
}
```

## BAD: Passive Surface Contains Business Logic

```rust
impl StatusComponent {
    pub fn render(&self) -> RenderedText {
        if self.state.violations().len() > 10 {
            RenderedText::from("Too many violations")
        } else {
            RenderedText::from("OK")
        }
    }
}
```

## BAD: Utility Surface Imports Smart Surface

```rust
use crate::surface_check_command::CheckCommand; // BAD

pub struct MyAction {
    command: CheckCommand,
}
```

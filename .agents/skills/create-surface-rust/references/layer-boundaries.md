# Layer Boundaries (AES406)

## Three Surface Types

| Type            | Suffixes                                     | Can Import From                 | Forbidden                                              | Description                                     |
| --------------- | -------------------------------------------- | ------------------------------- | ------------------------------------------------------ | ----------------------------------------------- |
| Smart Surface   | `_command`, `_controller`, `_page`, `_entry` | taxonomy,`contract_*_aggregate` | capabilities, concrete agents, concrete smart surfaces | Entry point/controller; delegates to aggregates |
| Utility Surface | `_hook`, `_store`, `_action`, `_screen`      | taxonomy, passive surfaces      | smart surfaces, capabilities, agents                   | Thin event/state adapter                        |
| Passive Surface | `_component`, `_view`, `_layout`             | taxonomy only                   | all other layers, orchestration, business logic        | Pure rendering/display                          |

## Smart Surface

Allowed:

```rust
use shared::common::taxonomy_file_path_vo::FilePath;
use shared::cli_commands::contract_import_runner_aggregate::IImportRunnerAggregate;
```

Forbidden:

```rust
use crate::capabilities_my_checker::MyChecker;
use crate::agent_tui_orchestrator::TuiOrchestrator;
```

## Utility Surface

Allowed:

```rust
use shared::tui::taxonomy_key_event_vo::KeyEvent;
use shared::tui::taxonomy_tui_action_vo::TuiAction;
use crate::surface_shortcut_component::ShortcutComponent;
```

Forbidden:

```rust
use crate::surface_check_command::CheckCommand;
use crate::capabilities_my_checker::MyChecker;
```

## Passive Surface

Allowed:

```rust
use shared::tui::taxonomy_status_view_model_vo::StatusViewModel;
```

Forbidden:

```rust
use crate::surface_check_command::CheckCommand;
use crate::capabilities_my_checker::MyChecker;
```

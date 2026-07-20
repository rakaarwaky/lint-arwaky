# Layer Boundaries (AES406)

## Three Surface Types

| Type            | Suffixes                                             | Can Import From                   | Forbidden                                                              | Description                                     |
| --------------- | ---------------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------- |
| Smart Surface   | `_command`, `_controller`, `_page`, `_entry` | taxonomy,`contract_*_aggregate` | capabilities, concrete agents, concrete smart surfaces | Entry point/controller; delegates to aggregates |
| Utility Surface | `_hook`, `_store`, `_action`, `_screen`      | taxonomy, passive surfaces        | smart surfaces, capabilities, agents                   | Thin event/state adapter                        |
| Passive Surface | `_component`, `_view`, `_layout`               | taxonomy only                     | all other layers, orchestration, business logic                        | Pure rendering/display                          |

## Smart Surface

Allowed:

```typescript
import { FilePath } from '../shared/common/taxonomy_path';
import { IImportRunnerAggregate } from '../shared/cli_commands/contract_import_runner_aggregate';
```

Forbidden:

```typescript
import { MyChecker } from '../capabilities/my_checker';
import { ImportRunner } from '../agent/import_runner';
```

## Utility Surface

Allowed:

```typescript
import { KeyEvent } from '../shared/tui/taxonomy_key_event_vo';
import { TuiAction } from '../shared/tui/taxonomy_tui_action_vo';
import { ShortcutComponent } from './surface_shortcut_component';
```

Forbidden:

```typescript
import { CheckCommand } from './surface_check_command';
import { MyChecker } from '../capabilities/my_checker';
```

## Passive Surface

Allowed:

```typescript
import { StatusViewModel } from '../shared/tui/taxonomy_status_view_model_vo';
```

Forbidden:

```typescript
import { CheckCommand } from './surface_check_command';
import { MyChecker } from '../capabilities/my_checker';
```

# TUI — DESIGN

> Visual and interaction contract for the terminal surface. Tokens, panel
> geometry, and component anatomy.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Brand & Style

`utility_tui_theme.rs` is the single source of truth for every token below. A
renderer resolves a token through the `color()` helper rather than applying a
palette constant, so `NO_COLOR` collapses the whole surface to the terminal's
monochrome default in one place.

### Colors

Every token is a **role**, not a palette entry. There are three accent roles
(`ACCENT`, `KEY`, `CAPABILITIES_BADGE`) and four diagnostic roles
(`VIOLATIONS`, `CLEAN`, `PENDING`, `FOCUS_CONFIRM`); structural roles are
neutral.

#### Base Surfaces

| Token        | Value     | Usage                                       |
| ------------ | --------- | ------------------------------------------- |
| `BACKGROUND` | `Black`   | default canvas behind every pane            |
| `HIGHLIGHT`  | `DarkGray`| selection band in the tree and file list    |

#### Primary Accent

| Token   | Value  | Usage                                                 |
| ------- | ------ | ----------------------------------------------------- |
| `ACCENT`| `Cyan` | window header; the surface's identity colour          |
| `HEADER`| `Cyan` | the header row itself                                  |

#### Secondary Accent

| Token            | Value      | Usage                                          |
| ---------------- | ---------- | ---------------------------------------------- |
| `DIRECTORY`      | `Blue`     | directory rows in the tree                      |
| `CAPABILITIES_BADGE` | `Magenta` | AES layer badge shown beside a file name    |

#### System Diagnostics

| Token           | Value  | Usage                                              |
| --------------- | ------ | -------------------------------------------------- |
| `VIOLATIONS`    | `Red`  | a finding row; the most saturated token on screen   |
| `CLEAN`         | `Green` | an explicit clean result, never an empty pane      |
| `PENDING`       | `Yellow` | work in progress that is not yet a finding        |
| `FOCUS_CONFIRM` | `Green` | the confirm affordance on the path-entry screen    |

#### Hairlines and Contours

| Token      | Value      | Usage                                       |
| ---------- | ---------- | ------------------------------------------- |
| `SEPARATOR`| `DarkGray` | rules between panes and above the status bar |
| `SCROLLBAR`| `DarkGray` | scrollbar track and thumb                    |
| `LABEL`    | `White`    | primary body text                            |

`KEY` and `PATH_INPUT` share `Yellow` with `PENDING` deliberately: a keystroke,
the field awaiting one, and work in progress are the same class of "the user
acts here" signal.

### Typography

There is no size scale. A terminal cell has one font and one size, so hierarchy
comes from colour role and emphasis, never from a point size — a larger stop
cannot map to anything, so the rule is dropped rather than faked.

| Role      | Token(s)              | Usage                                  |
| --------- | --------------------- | -------------------------------------- |
| Body      | `LABEL`               | file names, finding messages           |
| Emphasis  | `ACCENT`, `VIOLATIONS`| header, and the row the user must read |
| De-emphasis| `SEPARATOR`          | rules, scrollbars, inactive chrome     |

### Layout & Spacing

Layout is percentage-constrained rather than spaced in multiples of one unit,
because the three panes share a width the user chooses. The pane split is the
token; padding inside a pane is left to the renderer.

| Token                      | Value | Usage                              |
| -------------------------- | ----- | ---------------------------------- |
| `PANEL_TREE_WIDTH_PCT`     | `35%` | left pane — the file tree           |
| `PANEL_FILE_LIST_WIDTH_PCT`| `20%` | middle pane — the file list         |
| `PANEL_PREVIEW_WIDTH_PCT`  | `45%` | right pane — the findings preview   |
| `HEADER_HEIGHT`            | `1`   | header row                         |
| `STATUS_BAR_HEIGHT`        | `1`   | status line                        |
| `SHORTCUT_ROW_COUNT`       | `3`   | rows in the keybinding hint bar     |
| `MIN_TERMINAL_WIDTH`       | `40`  | below this the surface degrades     |
| `MIN_TERMINAL_HEIGHT`      | `15`  | below this the surface degrades     |

The pane widths live here so the mouse hit-map and the split can never drift
apart.

### Elevation & Depth

No elevation ladder. A terminal cell has no shadow and no z-order; overlay is
expressed by which pane owns the region. `surface-elevated` is therefore the
preview pane drawn over the file list, not a raised surface.

### Shapes

No radius tokens. Terminals offer no rounded corners, and inventing a glyph set
for them would make the surface depend on fonts the user may not have.

### Kind

`tui` — the terminal surface a developer drives when they want to browse
findings rather than read a report. It renders a file tree, runs lint actions
from a keystroke, and shows results without leaving the terminal.

### Request Shape

The user picks an action in the UI, `surface_event_action.rs` turns that into a
typed action, and `surface_lint_action.rs` receives it together with the current
`AppState`. A run's findings are written back into the state; the views render
from the state and hold none of their own.

### Invariants

- Views render from `AppState` and never call a lint aggregate directly. An
  action goes through `surface_lint_action.rs`.
- The event loop owns the state. A view that keeps its own copy will drift.
- A failed run never clears the previous results; the user keeps the last
  findings visible alongside the error.
- No renderer names a `Color` literal. It resolves a token, so `NO_COLOR`
  reaches every pane.

### Change Checklist

A new colour is a new token in `utility_tui_theme.rs` with one role, then a row
in the Colors table above. A new keystroke touches
`surface_event_action.rs` and the shortcut hint bar. A new lint action touches
`surface_lint_action.rs` and the command entry. A new pane touches its view, its
width in the Layout & Spacing table, and `root_tui_container.rs`.

## Components

| Component                   | File                              | States                                    | Token map                             |
| --------------------------- | --------------------------------- | ----------------------------------------- | ------------------------------------- |
| Event loop and app state    | `surface_tui_command.rs`          | running / idle                            | `BACKGROUND`                         |
| Action handler              | `surface_lint_action.rs`          | running / results / clean / error         | —                                     |
| Keystroke mapping           | `surface_event_action.rs`         | idle / ignored-while-running              | `KEY`                                |
| Path-entry screen           | `surface_path_screen.rs`          | empty / filled / focused / confirmed      | `PATH_INPUT`, `FOCUS_CONFIRM`         |
| File-tree pane              | `surface_tree_view.rs`            | browsing / selected                       | `DIRECTORY`, `LABEL`, `HIGHLIGHT`     |
| File-list pane              | `surface_file_list_view.rs`       | browsing / selected                       | `LABEL`, `HIGHLIGHT`                 |
| Findings preview pane       | `surface_preview_view.rs`         | results / clean / error                   | `VIOLATIONS`, `CLEAN`                 |
| Status line                 | `surface_status_component.rs`     | idle / busy / clean / error               | `PENDING`, `CLEAN`, `VIOLATIONS`      |
| Shortcut hint bar           | `surface_shortcut_component.rs`   | idle / busy-acknowledgement               | `LABEL`, `SEPARATOR`, `KEY`           |
| Log controller              | `surface_log_controller.rs`       | streaming / idle                          | `SEPARATOR`                          |
| Theme and colour resolution | `utility_tui_theme.rs`           | colour / no-colour                        | all tokens                           |
| Report rendering            | `utility_report_formatter.rs`     | rendered                                 | `LABEL`                              |
| Directory listing           | `utility_file_system.rs`          | ok / error                                | —                                     |

### States

| State        | Condition                        | What the user sees                    |
| ------------ | -------------------------------- | ------------------------------------- |
| `path_input` | No path chosen yet               | the path-entry screen                 |
| `browsing`   | A path is set and the tree loads | the tree and file list                |
| `running`    | An action is executing           | a busy status line                    |
| `results`    | A run finished                   | findings in the preview pane          |
| `clean`      | A run finished with zero findings| an explicit clean result, not an empty pane |
| `error`      | A run failed                     | the cause, with the tree still navigable |

A run that finds nothing shows `clean` explicitly. An empty pane is ambiguous
between "no findings" and "nothing has run yet".

#### A second action while `running`

A new action keypress arriving while the state is `running` is **ignored**: it
is not queued, and it never races the in-flight run. The status line
acknowledges it — "busy — try again once the current action finishes" — so the
keystroke is visibly declined rather than silently dropped. The transition out
of `running` is driven only by the in-flight run finishing, into `results`,
`clean`, or `error`.

Scans and background actions share **one** busy slot (`AppState::try_claim_busy_slot`,
parameterised by `BusySlot::Scan` / `BusySlot::Action`). Both fan out onto the
process-global rayon pool, so a scan and a background action can never be in
flight at the same time; the second request is declined with
`Busy: scan is running` or `Busy: action is running` on the status line (#577).

| Trigger while `running` | Transition | What the user sees |
| --- | --- | --- |
| Another action key (scan, fix, ci, orphan, security) | None — stays `running` | The busy status line, with a transient "action ignored" hint |
| A navigation key (tree/list movement, pane focus) | None — stays `running` | Navigation applies; browsing never contends for the run |
| Quit | Leaves the loop | The run's result is discarded, not merged into a stale state |

This is enforced by the event loop owning `AppState`, not by disabling terminal
input: a dropped-at-the-terminal keystroke would also drop navigation. Queueing
the second action is deliberately not the contract — a queued run would start
against a tree the user has since navigated away from, which is the "stale
results across navigation" failure this surface already has history with.

### Error States

| Condition                              | Behaviour                                                   |
| -------------------------------------- | ----------------------------------------------------------- |
| The chosen path does not exist         | Report it and stay in `browsing`; the tree is unchanged      |
| A required external tool is missing    | Report the tool and stay navigable                          |
| A run returns findings above threshold | Show the findings and the threshold                          |
| The terminal is too small to draw      | Degrade to the status line alone rather than panicking       |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Rules: [RULES_AES.md](../../RULES_AES.md)

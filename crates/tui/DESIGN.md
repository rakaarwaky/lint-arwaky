# TUI — DESIGN

## Kind

`tui` — the terminal surface a developer drives when they want to browse
findings rather than read a report. It renders a file tree, runs lint actions
from a keystroke, and shows results without leaving the terminal.

## Entry Points

| File | Role |
| --- | --- |
| `surface_tui_command.rs` | Starts the event loop and owns the app state. |
| `surface_lint_action.rs` | Turns a chosen action into a scan, fix, ci, orphan, or security run. |
| `surface_event_action.rs` | Maps a key event to an application action. |
| `surface_path_screen.rs` | The path-entry screen shown before a run starts. |
| `surface_tree_view.rs` | The file-tree pane. |
| `surface_file_list_view.rs` | The file-list pane. |
| `surface_preview_view.rs` | The findings preview pane. |
| `surface_status_component.rs` | The status line. |
| `surface_shortcut_component.rs` | The keybinding hint bar. |
| `surface_logging_controller.rs` | Log output routed into the UI. |
| `root_tui_container.rs` | Wires the views, the action handler, and the aggregates. |
| `utility_tui_theme.rs` | Colours and glyphs. |
| `utility_report_formatter.rs` | Renders findings as text for the preview pane. |
| `utility_file_system.rs` | Directory listing behind the tree. |

## Request Shape

The user picks an action in the UI, `surface_event_action.rs` turns that into a
typed action, and `surface_lint_action.rs` receives it together with the current
`AppState`. A run's findings are written back into the state; the views render
from the state and hold none of their own.

## States

| State | Condition | What the user sees |
| --- | --- | --- |
| `path_input` | No path chosen yet | The path-entry screen |
| `browsing` | A path is set and the tree is loaded | The tree and file list |
| `running` | An action is executing | A busy status line |
| `results` | A run finished | Findings in the preview pane |
| `clean` | A run finished with zero findings | An explicit clean result, not an empty pane |
| `error` | A run failed | The cause, with the tree still navigable |

A run that finds nothing shows `clean` explicitly. An empty pane is ambiguous
between "no findings" and "nothing has run yet".

## Error States

| Condition | Behaviour |
| --- | --- |
| The chosen path does not exist | Report it and stay in `browsing`; the tree is unchanged |
| A required external tool is missing | Report the tool and stay navigable |
| A run returns findings above threshold | Show the findings and the threshold |
| The terminal is too small to draw | Degrade to the status line alone rather than panicking |

## Invariants

- Views render from `AppState` and never call a lint aggregate directly. An
  action goes through `surface_lint_action.rs`.
- The event loop owns the state. A view that keeps its own copy will drift.
- A failed run never clears the previous results; the user keeps the last
  findings visible alongside the error.

## Change Checklist

A new keystroke touches `surface_event_action.rs` and the shortcut hint bar. A
new lint action touches `surface_lint_action.rs` and the command entry. A new
pane touches its view and `root_tui_container.rs`.

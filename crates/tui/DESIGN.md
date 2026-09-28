# TUI — DESIGN

## Kind

`tui` — browse and act on findings in a terminal interface.

## Entry points

| Function | Role |
|----------|------|
| `surface_lint_action` | Translate a keystroke into a lint action. |
| `lint-arwaky-tui` | Process entry that starts the terminal interface. |

## States

| State | Condition |
|-------|-----------|
| `browsing` | The file tree is shown and awaiting a selection. |
| `scanning` | A scan is running against the selected path (background thread; Esc discards the late result). |
| `busy` | Another action is running on a worker thread; new actions report "Busy" instead of starting. |
| `results` | Findings are listed for the selected file. |
| `error` | The scan could not complete. |
| `path-dialog` | First-run: Esc quits. Reopened via `r`: Esc cancels and keeps the current root. |
| `search` | Keystrokes filter the file list; Enter keeps the filter, Esc clears it. |
| `help` | Only `?`/Esc reach the handler; closing restores the previous preview mode. |
| `confirm` | A destructive action awaits `y`/Enter (execute) or `n`/Esc (cancel); all other keys are blocked. |

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
| `scanning` | A scan is running against the selected path. |
| `results` | Findings are listed for the selected file. |
| `error` | The scan could not complete. |

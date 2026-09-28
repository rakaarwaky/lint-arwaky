# CLI Surface — DESIGN

## Kind

`cli` — process a command from stdin / args and return structured output.

## Entry points

| Function | Role |
|----------|------|
| `surface_cli_handler` | Parse the CLI invocation and return the handled command path. |

## States

| State | Condition |
|-------|-----------|
| `command_found` | The CLI matched a known subcommand. |
| `command_missing` | No subcommand matched. |
| `error` | Parsing failed before dispatch. |

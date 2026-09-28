# CLI — DESIGN

## Kind

`cli` — one command surface, driven from the process entry.

## Entry points

| Function | Role |
|----------|------|
| `surface_calculator_command` | Map the parsed arguments to a request and return the rendered result. |

## States

| State | Condition |
|-------|-----------|
| `ready` | The surface is mounted and awaiting an invocation. |
| `busy` | A calculation is in flight. |
| `error` | The request could not be parsed or resolved. |

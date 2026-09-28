# Dispatcher Surface — DESIGN

## Kind

`api` — expose every rule group through a single dispatch boundary.

## Entry points

| Function | Role |
|----------|------|
| `scan` | Run all rule groups against a target path and merge their findings. |
| `check` | Scan and report exit-code-driven output. |

## States

| State | Condition |
|-------|-----------|
| `clean` | Zero violations returned by every group. |
| `violations` | One or more rule groups returned findings. |
| `error` | I/O or parse failure stopped the scan. |

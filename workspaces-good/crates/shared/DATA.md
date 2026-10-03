# DATA — Shared

## Reference

- PRD: [../PRD.md](../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## Data Overview

The shared crate declares the protocols each feature implements. It holds no
data of its own: callers supply the request, and each feature returns the
resolved value.

## Data Domain

| ID | Field | Type | Description |
|---|---|---|---|
| DO-001 | request | request | What the caller supplies to the feature. |
| DO-002 | result | response | What the feature returns once resolved. |

## Assumptions & Constraints

- A request is validated before any work runs, so no field is half-set.
- No field crosses the aggregate boundary in a mutable form.

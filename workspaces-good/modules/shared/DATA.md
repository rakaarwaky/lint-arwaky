# DATA Requirements

## Data Overview

The shared crate declares the protocols each feature implements. It holds no
data of its own: callers supply the request, and each feature returns the
resolved value.

## DT-SHARED-001: Request Shape

| Type | Description |
|------|-------------|
| request | What the caller supplies to the feature. |

## DT-SHARED-002: Resolved Value

| Type | Description |
|------|-------------|
| response | What the feature returns once resolved. |

## Reference

- PRD: [../PRD.md](../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

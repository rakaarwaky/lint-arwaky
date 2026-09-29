# DATA — Shared

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## Data Overview

The shared kernel manages cross-cutting value objects, constants, and contracts
used by all feature and surface folders. It holds no business logic — only
types, utilities, and interface definitions that other layers depend on.

## Value Objects

| ID | Field | Type | Description |
|---|---|---|---|
| DO-001 | Severity | vo | Lint finding severity level (error/warning/info) |
| DO-002 | ViolationItem | entity | Canonical representation of a single architecture violation |
| DO-003 | StructureFinding | vo | Structural violation found during folder layout audit |
| DO-004 | DocFinding | vo | Documentation invariant violation |

## Constants & Config

| Name | Meaning | Scope |
|---|---|---|
| AES100–AES700 | Architecture rule codes by layer | Global |
| SEVERITY_ERROR | High-severity — blocks merge | Global |
| SEVERITY_WARNING | Medium-severity — should fix | Global |
| SEVERITY_INFO | Low-severity — informational | Global |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| All rule crates | Outbound | Report findings up the call stack | Serialization error → finding dropped |
| CLI commands | Inbound | Render findings to terminal/JSON/SARIF | Formatting error → degraded output |

## Assumptions & Constraints

- The shared folder is a dependency sink — no other layer may import from features or surfaces.
- Value object field names are stable; renaming requires updating all consumers.
- Severity ranks must remain contiguous integers starting from zero.

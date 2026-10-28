# DATA Requirements

## Data Overview

The shared kernel manages cross-cutting value objects, constants, and contracts
used by all feature and surface folders. It holds no business logic — only
types, utilities, and interface definitions that other layers depend on.

### Relationships

| From | To | Kind | Rule|
|---|---|---|---|
| LintResult | DT-SHAR-002 ViolationItem | derivation (1:1) | A violation item is a projection of a lint result, copying code, file, line, column, message, and severity. It is never created independently of the result it narrows.|
| DT-SHAR-003 StructureFinding | DT-SHAR-002 ViolationItem | projection (1:1) | The dispatcher maps a structure finding onto a violation item, defaulting line and column to zero and severity to MEDIUM.|
| DT-SHAR-004 DocFinding | DT-SHAR-002 ViolationItem | projection (1:1) | A doc finding maps onto a violation item for JSON/SARIF output: `doc` joined to the audit root becomes the file path, column becomes 1, and severity stays HIGH.|
| DT-SHAR-002 ViolationItem | DT-SHAR-001 Severity | composition | Every violation item carries exactly one severity; severity is shared by all three finding types.|
| DT-SHAR-005 ReportSnapshot | DT-SHAR-002 ViolationItem | aggregation (N:1) | A snapshot is a count of a scan's violation items grouped by member path, plus when the scan ran.|
| DT-SHAR-006 SnapshotStore | DT-SHAR-005 ReportSnapshot | composition (1:N) | The store is one snapshot per scanned target, in one file.|
| DT-SHAR-007 MemberDelta | DT-SHAR-005 ReportSnapshot | derivation | A member row is the current and baseline counts of one member; the baseline is a previous snapshot, not a stored count.|
| DT-SHAR-008 ReportDelta | DT-SHAR-007 MemberDelta | aggregation (N:1) | The delta is every member's row plus the totals and the baseline's timestamp.|

## Data Requirements

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| PRD.md | Product requirements | [PRD.md](../PRD.md)|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../ARCHITECTURE.md)|

# DATA — Lint Arwaky Shared Kernel

## Data Overview

The shared kernel manages cross-cutting value objects, constants, and contracts used by all feature and surface folders. It holds no business logic — only types, utilities, and interface definitions that other layers depend on.

## Data Requirements

### DT-SHARED-001: Severity value object

| Type | Description |
|------|-------------|
| Severity | The lint severity tier (low, medium, high) attached to every finding; shared across all finding types. |

### DT-SHARED-002: ViolationItem value object

| Type | Description |
|------|-------------|
| ViolationItem | One lint finding: code, file, line, column, message, and severity. Produced from a lint result by projection. |

### DT-SHARED-003: StructureFinding value object

| Type | Description |
|------|-------------|
| StructureFinding | A structural scan result emitted by the dispatcher; maps onto a violation item with line and column defaulting to zero and severity set to medium. |

### DT-SHARED-004: DocFinding value object

| Type | Description |
|------|-------------|
| DocFinding | A document audit result emitted for JSON or SARIF output; maps onto a violation item with the audit-root-relative path as file and column set to one. |

### DT-SHARED-005: ReportSnapshot value object

| Type | Description |
|------|-------------|
| ReportSnapshot | A count of a scan's violation items grouped by member path, plus the scan timestamp. |

### DT-SHARED-006: SnapshotStore value object

| Type | Description |
|------|-------------|
| SnapshotStore | One report snapshot per scanned target, held in a single file. |

### DT-SHARED-007: MemberDelta value object

| Type | Description |
|------|-------------|
| MemberDelta | The current and baseline violation counts for one member; the baseline is a previous snapshot, not a stored count. |

### DT-SHARED-008: ReportDelta value object

| Type | Description |
|------|-------------|
| ReportDelta | Every member's delta row plus the totals and the baseline's timestamp. |

### Relationships

| From | To | Kind | Rule |
|------|----|------|------|
| LintResult | DT-SHARED-002 ViolationItem | derivation (1:1) | A violation item is a projection of a lint result, copying code, file, line, column, message, and severity. |
| DT-SHARED-003 StructureFinding | DT-SHARED-002 ViolationItem | projection (1:1) | The dispatcher maps a structure finding onto a violation item, defaulting line and column to zero and severity to MEDIUM. |
| DT-SHARED-004 DocFinding | DT-SHARED-002 ViolationItem | projection (1:1) | A doc finding maps onto a violation item for JSON/SARIF output; the audit-root-relative path becomes the file, column becomes 1, and severity stays HIGH. |
| DT-SHARED-002 ViolationItem | DT-SHARED-001 Severity | composition | Every violation item carries exactly one severity; severity is shared by all three finding types. |
| DT-SHARED-005 ReportSnapshot | DT-SHARED-002 ViolationItem | aggregation (N:1) | A snapshot is a count of a scan's violation items grouped by member path, plus when the scan ran. |
| DT-SHARED-006 SnapshotStore | DT-SHARED-005 ReportSnapshot | composition (1:N) | The store holds one snapshot per scanned target in one file. |
| DT-SHARED-007 MemberDelta | DT-SHARED-005 ReportSnapshot | derivation | A member row is the current and baseline counts of one member; the baseline is a previous snapshot. |
| DT-SHARED-008 ReportDelta | DT-SHARED-007 MemberDelta | aggregation (N:1) | The delta is every member's row plus the totals and the baseline's timestamp. |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| PRD.md | Product requirements | [PRD.md](../PRD.md)|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../ARCHITECTURE.md)|

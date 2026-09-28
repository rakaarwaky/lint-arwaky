# Structure Rules — FRD

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## Requirements

| ID | Requirement |
|----|-------------|
| FR-STR-001 | `scan` and `check` report AES701–AES705 findings for the audit target. |
| FR-STR-002 | Findings carry workspace-root-relative paths so a member-directory scan resolves them correctly. |
| FR-STR-003 | `check .` reports 0 structure violations for this repository. |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Walk cost | The audit reads directory entries only; no file is parsed or indexed | Run a scan over a workspace and confirm no AST parse occurs for the structure group |
| Path resolution | Every finding path resolves against the workspace root on disk | Scan each member directory on its own and confirm each reported path exists |
| Idempotency | A second scan over an unchanged workspace reports the same finding set | Run the scan twice and compare the two finding lists |
| De-duplication | One structural defect produces one finding | Introduce the same misplaced file twice via two paths and confirm a single finding |

## Scenarios

| Scenario | Given | When | Then |
|----------|-------|------|------|
| A shared folder holds a capability file | A stray capability file in the shared folder | `scan` runs | AES701 fires naming the file. |
| A feature folder lacks its doc pair | An orchestrator with no spec or backlog | `scan` runs | AES704 fires naming the missing documents. |
| A surface folder lacks a design document | A surface-dominated folder with no design record | `scan` runs | AES705 fires naming the folder. |

## Integration

The structure audit is aggregated by `dispatcher`, wired through `IStructureAggregate`, and
returned as part of `ScanAggregates`. No external process is involved.

## API

| Symbol | Kind | Notes |
|--------|------|-------|
| `RootStructureRulesContainer::orchestrator()` | Function | Composition root entry. |
| `StructureAuditor::audit` | Method | `IStructureAuditProtocol` implementation. |

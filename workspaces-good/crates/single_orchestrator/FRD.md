# Single — FRD

## Requirements

| ID | Requirement |
|----|-------------|
| FR-SINGLE-001 | The feature resolves the requests its caller supplies. |

## Non-functional

| ID | Requirement |
|----|-------------|
| NFR-SINGLE-001 | No I/O outside the aggregate boundary. |

## Scenarios

| Scenario | Given | When | Then |
|----------|-------|------|------|
| A request is resolved | A valid input | The orchestrator runs | The resolved value is returned. |

## Integration

Called through the shared aggregate contract.

## API

| Symbol | Kind | Notes |
|--------|------|-------|
| `SingleOrchestrator` | Class | The feature's single entry point. |

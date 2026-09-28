# Subtraction — FRD

## Requirements

| ID | Requirement |
|----|-------------|
| FR-SUBTRACTION-001 | The feature resolves the subtraction requests its caller supplies. |

## Non-functional

| ID | Requirement |
|----|-------------|
| NFR-SUBTRACTION-001 | No I/O outside the aggregate boundary. |

## Scenarios

| Scenario | Given | When | Then |
|----------|-------|------|------|
| A request is resolved | A valid pair of operands | The orchestrator runs | The subtraction result is returned. |

## Integration

Called through the shared aggregate contract.

## API

| Symbol | Kind | Notes |
|--------|------|-------|
| `SubtractionOrchestrator` | Class | The feature's single entry point. |

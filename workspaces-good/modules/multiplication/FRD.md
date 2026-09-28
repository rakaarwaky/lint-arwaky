# Multiplication — FRD

## Requirements

| ID | Requirement |
|----|-------------|
| FR-MULTIPLICATION-001 | The feature resolves the multiplication requests its caller supplies. |

## Non-functional

| ID | Requirement |
|----|-------------|
| NFR-MULTIPLICATION-001 | No I/O outside the aggregate boundary. |

## Scenarios

| Scenario | Given | When | Then |
|----------|-------|------|------|
| A request is resolved | A valid pair of operands | The orchestrator runs | The multiplication result is returned. |

## Integration

Called through the shared aggregate contract.

## API

| Symbol | Kind | Notes |
|--------|------|-------|
| `MultiplicationOrchestrator` | Class | The feature's single entry point. |

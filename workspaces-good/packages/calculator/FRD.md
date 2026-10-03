# FRD — Calculator

## Reference

- PRD: [../../PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## System Overview

Calculator coordinates four operation features. The orchestrator routes a
request to the feature that owns the operation and returns the result; the
history capability merges the four operation logs into what the caller reads.

## Functional Requirements

### FR-CALCULATOR-001: Route the request

- **Description**: The orchestrator routes a request to the feature that owns the operation.
- **Input**: A request naming the operation and its operands.
- **Output**: The resolved value, or an error naming the operation.
- **Business Rules**: The orchestrator performs no arithmetic itself.
- **Edge Cases**: An unknown operation returns an error instead of a default.
- **Error Handling**: Returns an error; never panics.

### FR-CALCULATOR-002: Merge the operation history

- **Description**: The history capability merges the four operation logs newest first.
- **Input**: The four per-feature operation logs.
- **Output**: One history ordered by when each entry was recorded.
- **Business Rules**: Merging never reorders entries recorded at the same instant.
- **Edge Cases**: An empty log set returns an empty history rather than an error.
- **Error Handling**: An unreadable log yields an empty history, not a panic.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| route | request | result | error | — | Routes a request to the feature that owns the operation. |
| merge | logs | history | — | — | Merges the four operation logs newest first. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| execute | request | response | error | — | The member's single entry point. |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| Caller | in | Supplies the request | Unknown operation -> returns an error |
| Operation features | in | Owns the arithmetic | Unreachable -> returns an error |

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|---------------------|
| No I/O outside the aggregate boundary | 0 calls | `grep -c "open(" src/` |

## Test Scenarios

- A request naming a known operation reaches that feature and returns a value.
- A request naming an unknown operation returns an error.
- Merging four operation logs returns one history, newest first.

## Assumptions & Constraints

- Each operation lives in its own feature folder behind its own orchestrator.
- The root container constructs the operation features and injects them here.

## Glossary

- **Aggregate**: The layer that owns the state and the rules.
- **Operation log**: One feature's record of the requests it answered.

# FRD — Calculator

## Feature Overview

Calculator coordinates four operation features. The orchestrator routes a
request to the feature that owns the operation and returns the result; the
history capability merges the four operation logs into what the caller reads.

## Functional Requirements

### FR-CALCULATOR-001: Route the request

| Aspect         | Detail                                     |
|----------------|---------------------------------------------|
| Description    | {FR-CALCULATOR-001 description}                    |
| Business Rules | {business rules}                         |
| Edge Cases     | {edge cases}                             |

- **Description**: The orchestrator routes a request to the feature that owns the operation.
- **Input**: A request naming the operation and its operands.
- **Output**: The resolved value, or an error naming the operation.
- **Business Rules**: The orchestrator performs no arithmetic itself.
- **Edge Cases**: An unknown operation returns an error instead of a default.
- **Error Handling**: Returns an error; never panics.

### FR-CALCULATOR-002: Merge the operation history

| Aspect         | Detail                                     |
|----------------|---------------------------------------------|
| Description    | {FR-CALCULATOR-002 description}                    |
| Business Rules | {business rules}                         |
| Edge Cases     | {edge cases}                             |

- **Description**: The history capability merges the four operation logs newest first.
- **Input**: The four per-feature operation logs.
- **Output**: One history ordered by when each entry was recorded.
- **Business Rules**: Merging never reorders entries recorded at the same instant.
- **Edge Cases**: An empty log set returns an empty history rather than an error.
- **Error Handling**: An unreadable log yields an empty history, not a panic.

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description|
|--------|-------|--------|-------|-------|-------------|
| route | request | result | error | — | Routes a request to the feature that owns the operation.|
| merge | logs | history | — | — | Merges the four operation logs newest first.|

### Aggregate API

| Method | Input | Output | Error | Event | Description|
|--------|-------|--------|-------|-------|-------------|
| execute | request | response | error | — | The member's single entry point.|

## Reference

- PRD: [../../PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md)|
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md)|

# FRD — Calculator

## Feature Overview

Calculator is one layer inside the shared contract. The caller supplies a request,
the orchestrator applies the capability, and the result returns to the caller.

## Functional Requirements

### FR-CALCULATOR-001: Resolve the request

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-CALCULATOR-001 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: The feature checks the request before the run and returns the result.
- **Input**: A request supplied by the caller.
- **Output**: The resolved value, or an error.
- **Business Rules**: The caller never sees a partial result.
- **Edge Cases**: An empty request is rejected before any work runs.
- **Error Handling**: Returns an error; never panics.

### FR-CALCULATOR-002: Run the request

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | {FR-CALCULATOR-002 description}                     |
| Business Rules | {business rules}                          |
| Edge Cases     | {edge cases}                              |

- **Description**: The feature returns the resolved value and returns the result.
- **Input**: A request supplied by the caller.
- **Output**: The resolved value, or an error.
- **Business Rules**: The caller never sees a partial result.
- **Edge Cases**: An empty request is rejected before any work runs.
- **Error Handling**: Returns an error; never panics.

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| resolve | request | result | error | — | checks the request before the run |
| run | request | result | error | — | returns the resolved value |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| execute | request | response | error | — | The feature's single entry point. |

## Reference

- PRD: [../PRD.md](../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md) |

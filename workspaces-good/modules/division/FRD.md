# FRD — Division

## Reference

- PRD: [../PRD.md](../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md)

## System Overview

Division is one layer inside the shared contract. The caller supplies a request,
the orchestrator applies the capability, and the result returns to the caller.

## Functional Requirements

### FR-DIVISION-001: Check the request

- **Description**: The feature rejects a zero divisor and returns the result.
- **Input**: A request supplied by the caller.
- **Output**: The resolved value, or an error.
- **Business Rules**: The caller never sees a partial result.
- **Edge Cases**: An empty request is rejected before any work runs.
- **Error Handling**: Returns an error; never panics.

### FR-DIVISION-002: Run the request

- **Description**: The feature returns the quotient and returns the result.
- **Input**: A request supplied by the caller.
- **Output**: The resolved value, or an error.
- **Business Rules**: The caller never sees a partial result.
- **Edge Cases**: An empty request is rejected before any work runs.
- **Error Handling**: Returns an error; never panics.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| check | request | result | error | — | rejects a zero divisor |
| run | request | result | error | — | returns the quotient |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| execute | request | response | error | — | The feature's single entry point. |

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| Caller | in | Supplies the request | Invalid request -> returns an error |
| Shared contract crate | in | Injects the protocols | Compile error at build time |

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|---------------------|
| No I/O outside the aggregate boundary | 0 calls | `grep -c "std::fs" src/` |

## Test Scenarios

- A valid request resolves to the expected value.
- An empty request returns an error instead of a partial result.

## Assumptions & Constraints

- I/O stays outside the aggregate boundary.
- The pinned toolchain builds the crate without warnings.

## Glossary

- **Aggregate**: The layer that owns the state and the rules.
- **Protocol**: A trait the shared crate declares and a feature implements.

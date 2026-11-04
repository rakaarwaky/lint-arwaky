# FRD — Multiplication

## Feature Overview

Multiplication is one layer inside the shared contract. The caller supplies a request,
the orchestrator applies the capability, and the result returns to the caller.

## Functional Requirements

### FR-MULTIPLICATION-001: Check the request

**Description:** The feature validates the request and returns the result.

#### Business Logic

- The caller never sees a partial result.
- The request is validated before the capability runs.
- The orchestrator applies the capability and returns the result.

#### Edge Cases

- An empty request is rejected before work runs.
- A malformed request returns an error.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A request supplied by the caller. |
| **Output** | The resolved value, or an error. |
| **Protocol** | Multiplication::Protocol |
| **Method** | check |
| **Event** | Finding emitted |
| **Error** | Returns an error; never panics. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Multiplication | caller supplies a valid request | result returned without partial output |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Latency | 200ms |

### FR-MULTIPLICATION-002: Run the request

**Description:** The feature executes the request and returns the result.

#### Business Logic

- The caller never sees a partial result.
- The request is validated before the capability runs.
- The orchestrator applies the capability and returns the result.

#### Edge Cases

- An empty request is rejected before work runs.
- A malformed request returns an error.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A request supplied by the caller. |
| **Output** | The resolved value, or an error. |
| **Protocol** | Multiplication::Protocol |
| **Method** | run |
| **Event** | Finding emitted |
| **Error** | Returns an error; never panics. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Multiplication | caller supplies a valid request | result returned without partial output |

#### Metric

| Target Metric | Value |
|---------------|-------|
| Latency | 200ms |

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | Multiplication::Agent     |
| Aggregate  | Multiplication::Aggregate |
| Method     | execute           |
| Request    | request           |
| Response   | response          |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](../BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies        | [ROADMAP.md](../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](../ARCHITECTURE.md) |

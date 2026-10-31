# FRD — Calculator

## Feature Overview

Calculator is one layer inside the shared contract. The caller supplies a request,
the orchestrator applies the capability, and the result returns to the caller.

## Functional Requirements

### FR-Calculator-001: Check the request

**Description:** The feature validates the request and returns the result.

#### Business Logic

- The caller never sees a partial result.
- An empty request is rejected before work runs.

#### Edge Cases

- An empty request is rejected before work runs.
- A malformed request returns an error.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A request supplied by the caller. |
| **Output** | The resolved value, or an error. |
| **Protocol** | Calculator::Protocol |
| **Method** | check |
| **Event** | Finding emitted |
| **Error** | Returns an error; never panics. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A request is supplied | check is invoked | The result is returned. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| 0 doc-rule findings | 0 |

### FR-Calculator-002: Run the request

**Description:** The feature executes the request and returns the result.

#### Business Logic

- The caller never sees a partial result.
- An empty request is rejected before work runs.

#### Edge Cases

- An empty request is rejected before work runs.
- A malformed request returns an error.

#### Technical Specification

| Aspect | Detail |
|--------|--------|
| **Input** | A request supplied by the caller. |
| **Output** | The resolved value, or an error. |
| **Protocol** | Calculator::Protocol |
| **Method** | run |
| **Event** | Finding emitted |
| **Error** | Returns an error; never panics. |

#### Test

| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A request is supplied | run is invoked | The result is returned. |

#### Metric

| Target Metric | Value |
|---------------|-------|
| 0 doc-rule findings | 0 |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | Calculator::Agent |
| Aggregate | Calculator::Aggregate |
| Method | execute |
| Request | request |
| Response | response |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](../../BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

# FRD — Subtraction

## Feature Overview

Subtraction is one layer inside the shared contract. The caller supplies a request,
the orchestrator applies the capability, and the result returns to the caller.

## Functional Requirements

### FR-SUBTRACTION-001: Check the request

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | The feature validates the request and returns the result. |
| Business Rules | The caller never sees a partial result.     |
| Edge Cases     | An empty request is rejected before work runs. |
| Capability     | Subtraction::Capability                          |
| Protocol       | Subtraction::Protocol                            |
| Method         | check                                       |
| Input          | A request supplied by the caller.           |
| Output         | The resolved value, or an error.            |
| Error          | Returns an error; never panics.             |
| Event          | Finding emitted                             |
| Test           | lint-arwaky-cli docs . reports 0 findings   |
| Metric         | 0 doc-rule findings                         |

### FR-SUBTRACTION-002: Run the request

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | The feature executes the request and returns the result. |
| Business Rules | The caller never sees a partial result.     |
| Edge Cases     | An empty request is rejected before work runs. |
| Capability     | Subtraction::Capability                          |
| Protocol       | Subtraction::Protocol                            |
| Method         | run                                         |
| Input          | A request supplied by the caller.           |
| Output         | The resolved value, or an error.            |
| Error          | Returns an error; never panics.             |
| Event          | Finding emitted                             |
| Test           | lint-arwaky-cli docs . reports 0 findings   |
| Metric         | 0 doc-rule findings                         |

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | Subtraction::Agent     |
| Aggregate  | Subtraction::Aggregate |
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

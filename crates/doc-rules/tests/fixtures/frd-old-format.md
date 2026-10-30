# FRD — Legacy Format

## Functional Requirements

### FR-FEAT-001: Do The Thing

| Aspect | Detail |
| --- | --- |
| Description | The feature performs a core responsibility. |
| Business Rules | State is immutable across invocations. |
| Edge Cases | An absent input yields a default. |
| Capability | The capability is stateless. |
| Protocol | One protocol seam per feature. |
| Method | `execute` is the single entry point. |
| Input | A request value object. |
| Output | A response value object. |
| Error | Failures surface as a reason-coded outcome. |
| Event | `Completed` fires on success. |
| Test | Unit test covers the happy path. |
| Metric | P99 latency under 200 ms. |

## Public Interface

| Aspect | Detail |
| --- | --- |
| Protocol | ISampleProtocol |
| Aggregate | ISampleAggregate |

## Reference

| Artifact | Purpose | Link |
| --- | --- | --- |
| PRD | Business goals | [PRD.md](../../PRD.md) |

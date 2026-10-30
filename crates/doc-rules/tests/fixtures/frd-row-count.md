# FRD — Row Count Violation

## Functional Requirements

### FR-FEAT-001: Do The Thing

#### Description

| No | Detail |
| --- | --- |
| 1 | The feature performs a core responsibility. |
| 2 | It integrates with the shared contract layer. |
| ∞ | — |

#### Business Rules

| No | Detail |
| --- | --- |
| 1 | State is immutable across invocations. |
| 2 | Failures surface as reason-coded outcomes. |
| ∞ | — |

#### Edge Cases

| No | Detail |
| --- | --- |
| 1 | An absent input yields a default. |
| 2 | An oversized input is rejected. |
| ∞ | — |

#### Capability

| No | Detail |
| --- | --- |
| 1 | The capability is stateless. |
| 2 | It is also idempotent. |

#### Protocol

| No | Detail |
| --- | --- |
| 1 | One protocol seam per feature. |

#### Method

| No | Detail |
| --- | --- |
| 1 | `execute` is the single entry point. |
| 2 | `dispatch` routes to the seam. |
| ∞ | — |

#### Input

| No | Detail |
| --- | --- |
| 1 | A request value object. |
| 2 | An optional context handle. |

#### Output

| No | Detail |
| --- | --- |
| 1 | A response value object. |
| 2 | A status code. |
| ∞ | — |

#### Error

| No | Detail |
| --- | --- |
| 1 | Invalid input produces a `ValidationError`. |
| 2 | A downstream timeout produces a `TimeoutError`. |

#### Event

| No | Detail |
| --- | --- |
| 1 | `Completed` fires on success. |
| 2 | `Failed` fires on error. |
| ∞ | — |

#### Test

| No | Detail |
| --- | --- |
| 1 | Unit test covers the happy path. |
| 2 | Integration test covers the error path. |
| ∞ | — |

#### Metric

| No | Detail |
| --- | --- |
| 1 | P99 latency under 200 ms. |
| 2 | Error rate under 0.1 %. |
| ∞ | — |

## Public Interface

| Aspect | Detail |
| --- | --- |
| Protocol | ISampleProtocol |
| Aggregate | ISampleAggregate |

## Reference

| Artifact | Purpose | Link |
| --- | --- | --- |
| PRD | Business goals | [PRD.md](../../PRD.md) |

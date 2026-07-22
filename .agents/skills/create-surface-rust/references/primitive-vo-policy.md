# Primitive and VO Rules

Surface state should use shared VOs.

## Primitive Policy

| Primitive        | Rule                                                                                           |
| ---------------- | ---------------------------------------------------------------------------------------------- |
| `String`         | Forbidden for domain/state fields. Use VO. May be used as final rendered presentation output.  |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                                           |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                                           |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                                           |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                                           |
| `char`           | Forbidden for domain values. Use VO.                                                           |
| `bool`           | Allowed for semantic UI toggles when no richer VO is needed.                                   |
| `&str`           | Allowed for borrowed low-level input or accessor usage, but domain identifiers should use VOs. |

Prefer VOs for: UI state, view models, events, actions, requests, reports, status, labels, rendered fragments.

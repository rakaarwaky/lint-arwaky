# Primitive and VO Rules

Surface state should use shared VOs.

## Primitive Policy

| Primitive | Rule                                                                                          |
| --------- | --------------------------------------------------------------------------------------------- |
| `string`  | Forbidden for domain/state fields. Use VO. May be used as final rendered presentation output. |
| `number`  | Forbidden for domain values. Use VO.                                                          |
| `boolean` | Allowed for semantic UI toggles when no richer VO is needed.                                  |

Prefer VOs for: UI state, view models, events, actions, requests, reports, status, labels, rendered fragments.

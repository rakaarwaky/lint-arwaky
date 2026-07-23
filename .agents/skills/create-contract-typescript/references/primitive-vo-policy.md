# Primitive and VO Rules

Contract signatures should use shared taxonomy VOs for domain data.

## Forbidden for domain values

| Primitive           | Rule                                                            |
| ------------------- | --------------------------------------------------------------- |
| `string`            | Forbidden for domain fields and public contract values. Use VO. |
| `number`            | Forbidden for domain values. Use VO.                            |
| `string[]`          | Forbidden for domain collections. Use list VO.                  |
| `Record<string, T>` | Forbidden for domain data. Use VO.                              |

## Allowed with care

| Type      | Rule                                                      |
| --------- | --------------------------------------------------------- |
| `boolean` | Allowed for semantic toggles when no richer VO is needed. |

Prefer VOs for: file paths, symbol names, messages, line numbers, counts, severity, requests, results, identifiers, policies.

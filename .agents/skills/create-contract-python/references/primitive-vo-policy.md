# Primitive and VO Rules

Contract signatures should use shared taxonomy VOs for domain data.

## Forbidden for domain values

| Primitive   | Rule                                                            |
| ----------- | --------------------------------------------------------------- |
| `str`       | Forbidden for domain fields and public contract values. Use VO. |
| `int`       | Forbidden for domain values. Use VO.                            |
| `float`     | Forbidden for domain values. Use VO.                            |
| `list[str]` | Forbidden for domain collections. Use list VO.                  |
| `dict`      | Forbidden for domain data. Use VO.                              |

## Allowed with care

| Type   | Rule                                                      |
| ------ | --------------------------------------------------------- |
| `bool` | Allowed for semantic toggles when no richer VO is needed. |

Prefer VOs for: file paths, symbol names, messages, line numbers, counts, severity, requests, results, identifiers, policies.

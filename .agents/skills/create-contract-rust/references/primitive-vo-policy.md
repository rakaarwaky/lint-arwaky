# Primitive and VO Rules

Contract signatures should use shared taxonomy VOs for domain data.

## Forbidden for domain values

| Primitive        | Rule                                                                  |
| ---------------- | --------------------------------------------------------------------- |
| `String`         | Forbidden for domain fields and public contract values. Use VO.       |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                  |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                  |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                  |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                  |
| `char`           | Forbidden for domain values. Use VO.                                  |
| `Vec<String>`    | Forbidden for domain collections. Use list VO.                        |
| `Option<String>` | Forbidden for optional domain values. Use`Option<VO>` or optional VO. |

## Allowed with care

| Type   | Rule                                                                                |
| ------ | ----------------------------------------------------------------------------------- |
| `bool` | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str` | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for: file paths, symbol names, messages, line numbers, counts, severity, requests, results, identifiers, policies.

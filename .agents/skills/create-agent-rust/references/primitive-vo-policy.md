# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```rust
pub trait I<NameOrchestrator>Aggregate {
    fn execute(&self, files: Vec<String>) -> Vec<String>;
}
```

Good:

```rust
pub trait I<NameOrchestrator>Aggregate {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>>;
}
```

## Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract values. Use VO.                     |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                                |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                                |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                                |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                                |
| `char`             | Forbidden for domain values. Use VO.                                                |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, results, policies, thresholds.

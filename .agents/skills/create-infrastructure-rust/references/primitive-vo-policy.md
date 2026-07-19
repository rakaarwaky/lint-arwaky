# Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```rust
pub trait IFileWriterPort {
    fn write(&self, path: &str, content: &str) -> Result<(), std::io::Error>;
}
```

Good:

```rust
pub trait IFileWriterPort {
    fn write(&self, path: &FilePath, content: &FileContent) -> Result<(), FileWriteError>;
}
```

## Primitive Policy

| Primitive        | Rule |
| ---------------- | ---- |
| `String`         | Forbidden for domain fields and public contract values. Use VO. |
| `i32`, `i64`     | Forbidden for domain values. Use VO. |
| `u32`, `u64`     | Forbidden for domain values. Use VO. |
| `usize`, `isize` | Forbidden for domain values. Use VO. |
| `f32`, `f64`     | Forbidden for domain values. Use VO. |
| `char`           | Forbidden for domain values. Use VO. |
| `bool`           | Allowed for technical toggles when no richer VO is needed. |
| `&str`           | May be used internally for low-level boundary code, but public contracts should prefer VOs. |

Prefer VOs for: file paths, URLs, timeouts, durations, cache keys, cache values, query results, identifiers, messages.

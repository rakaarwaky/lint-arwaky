# Primitive-to-VO Rules

## General Rule

Domain data MUST use VOs, not raw owned primitives.

Bad:

```rust
pub struct LintResult {
    pub file_path: String,
    pub line: u32,
    pub severity: String,
}
```

Good:

```rust
pub struct LintResult {
    file_path: FilePath,
    line: LineNumber,
    severity: Severity,
}
```

## Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract return values. Use VO.              |
| `i32`, `i64`     | Forbidden. Use domain VO.                                                           |
| `u32`, `u64`     | Forbidden. Use domain VO.                                                           |
| `usize`, `isize` | Forbidden for domain values. Use domain VO.                                         |
| `f32`, `f64`     | Forbidden. Use domain VO.                                                           |
| `char`             | Forbidden for domain values. Use domain VO.                                         |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for: file paths, symbol names, messages, line numbers, column numbers, severity, durations, counts, thresholds, identifiers.

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

```rust
impl LineNumber {
    pub fn new(value: u32) -> Result<Self, ValidationError> {
        if value == 0 {
            return Err(ValidationError::positive("LineNumber"));
        }
        Ok(Self(value))
    }
}
```

## Optional and Collection Primitives

Bad:

```rust
pub struct RuleSet {
    pub patterns: Vec<String>,
    pub description: Option<String>,
}
```

Good:

```rust
pub struct RuleSet {
    patterns: PatternList,
    description: Option<RuleDescription>,
}
```

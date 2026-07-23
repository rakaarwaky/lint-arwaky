# Primitive-to-VO Replacement Rules (AES402)

## General Rule

Domain data MUST use shared VOs, not raw primitives.

Bad:

```rust
pub struct <ResultVO> {
    pub target: String,
    pub position: u32,
    pub level: String,
}
```

Good:

```rust
pub struct <ResultVO> {
    target: <Target>VO,
    position: <LineNumber>VO,
    level: <Severity>VO,
}
```

## Primitive Policy

| Primitive        | Rule                                                                                |
| ---------------- | ----------------------------------------------------------------------------------- |
| `String`         | Forbidden for domain fields and public contract return values. Use VO.              |
| `i32`, `i64`     | Forbidden. Use domain VO.                                                           |
| `u32`, `u64`     | Forbidden. Use domain VO.                                                           |
| `usize`, `isize` | Forbidden for domain values. Use domain VO.                                         |
| `f32`, `f64`     | Forbidden. Use domain VO.                                                           |
| `char`           | Forbidden for domain values. Use domain VO.                                         |
| `bool`           | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`           | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- file paths
- symbol names
- messages
- line numbers
- column numbers
- severity
- durations
- counts
- thresholds
- identifiers

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

Good:

```rust
impl <LineNumber>VO {
    pub fn new(value: u32) -> Result<Self, ValidationError> {
        if value == 0 {
            return Err(ValidationError::positive("<LineNumber>VO"));
        }
        Ok(Self(value))
    }
}
```

## Optional and Collection Primitives

Bad:

```rust
pub struct <RuleSet>VO {
    pub patterns: Vec<String>,
    pub description: Option<String>,
}
```

Good:

```rust
pub struct <RuleSet>VO {
    patterns: <PatternList>VO,
    description: Option<<RuleDescription>VO>,
}
```

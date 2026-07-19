# Dataclass Patterns

## Value Objects (`_vo.rs`)

Prefer private inner fields.

Bad:

```rust
pub struct FilePath {
    pub value: String,
}
```

Good:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::empty("FilePath"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

## Composite Value Objects

Use other VOs as fields, not raw primitives.

```rust
pub struct ImportRuleVO {
    pattern: RulePattern,
    message: RuleMessage,
}
```

## Entities (`_entity.rs`)

```rust
pub struct SymbolEntity {
    id: SymbolId,
    name: SymbolName,
}
```

## Error Types (`_error.rs`)

Use `thiserror::Error`.

```rust
#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("Failed to read file: {0}")]
    Io(FilePath, #[source] std::io::Error),

    #[error("File content is invalid: {0}")]
    Validation(FilePath),
}
```

## Event Types (`_event.rs`)

```rust
pub struct ScanCompletedEvent {
    scan_id: ScanId,
}
```

## Constants (`_constant.rs`)

```rust
pub const FPS_DEFAULT: f64 = 24.0;
pub const MIN_REVEAL_SECONDS: f64 = 0.5;
pub const MANIFEST_FILENAME: &str = "manifest.json";
```

Rules: no functions, no I/O, no external layer imports, no mutable state.

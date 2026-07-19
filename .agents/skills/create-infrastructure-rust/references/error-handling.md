# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```rust
let content = std::fs::read_to_string(path.value()).unwrap_or_default();
```

## Rule 2: Fallible port methods should return `Result`

```rust
fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
```

## Rule 3: Use descriptive error types

Prefer custom error types from shared taxonomy.

```rust
pub enum FileReadError {
    Io(FilePath, std::io::Error),
    Validation(ValidationError),
}
```

## Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs. Lint violations belong to capabilities.

Bad:

```rust
fn read(&self, path: &FilePath) -> Vec<LintResult> { // BAD }
```

Good:

```rust
fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> { // OK }
```

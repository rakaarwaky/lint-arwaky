# Error Handling Rules

Capabilities error handling must be explicit.

## Rule 1: Do not silently discard errors

Forbidden:

```rust
let value = result.unwrap_or_default();
```

Forbidden:

```rust
let value = result.ok().unwrap_or_default();
```

## Rule 2: Fallible operations should return `Result`

If a method represents an operation that can fail unexpectedly, return `Result<T, E>`.

```rust
fn parse_manifest(content: &ManifestContent) -> Result<Manifest, ManifestParseError> {
    // ...
}
```

## Rule 3: Check/analysis methods may return `Vec<LintResult>`

For linting/analysis use cases, violations are expected domain outcomes.

```rust
fn check_imports(source: &SourceContentVO) -> Vec<LintResult> {
    let mut violations = Vec::new();
    // analysis logic
    violations
}
```

## Rule 4: I/O errors belong to utility implementations (infrastructure layer removed)

Bad in capabilities:

```rust
fn check_file(path: &FilePath) -> Vec<LintResult> {
    let content = std::fs::read_to_string(path.value()).unwrap_or_default(); // BAD
    Vec::new()
}
```

Good:

```rust
// utility_source_reader.rs
impl ISourceReaderProtocol for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<SourceContentVO, SourceReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(SourceReadError::Io)?;
        SourceContentVO::new(path.clone(), raw)
            .map_err(SourceReadError::Validation)
    }
}
```

```rust
// capabilities_import_checker.rs
impl IImportCheckerProtocol for ImportChecker {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult> {
        // pure analysis using already-read source
        Vec::new()
    }
}
```

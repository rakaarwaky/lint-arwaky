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
fn parse_input(content: &<RawContent>VO) -> Result<<DomainVO>, <Name>ParseError> {
    // ...
}
```

## Rule 3: Check/analysis methods may return `Vec<<ResultVO>>`

## Rule 3: Analysis methods may return a collection of `<ResultVO>`

```rust
fn execute(input: &<DomainVO>) -> Vec<<ResultVO>> {
    let mut results = Vec::new();
    // analysis logic
    results
}
```

## Rule 4: I/O errors belong to utility implementations (layer removed)

Bad in capabilities:

```rust
fn read_input(path: &<Path>VO) -> Vec<<ResultVO>> {
    let content = std::fs::read_to_string(path.value()).unwrap_or_default(); // BAD
    Vec::new()
}
```

Good:

```rust
// utility_source_reader.rs
impl I<NameReader>Protocol for FileSystem<NameReader> {
    fn read(&self, path: &<Path>VO) -> Result<<SourceContent>VO, <Name>ReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(<Name>ReadError::Io)?;
        <SourceContent>VO::new(path.clone(), raw)
            .map_err(<Name>ReadError::Validation)
    }
}
```

```rust
// capabilities_<name>.rs
impl I<NameCapability>Protocol for <NameCapability> {
    fn execute(&self, source: &<SourceContent>VO) -> Vec<<ResultVO>> {
        // pure analysis using already-read source
        Vec::new()
    }
}

```

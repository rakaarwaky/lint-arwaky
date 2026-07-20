# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```rust
let result = checker.check().unwrap_or_default();
```

## Rule 2: Agent may return `Vec<<ResultVO>>` for analysis orchestration

```rust
fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
    let mut results = Vec::new();
    for file in request.files() {
        match self.analyzer.analyze(file) {
            Ok(result) => results.extend(result.into_results()),
            Err(err) => results.push(<ResultVO>::from_analysis_error(file, err)),
        }
    }
    results
}
```

## Rule 3: Agent may return `Result` for execution orchestration

```rust
fn run(&self, request: &<ScanRequest>VO) -> Result<ExecutionReport, AgentExecutionError>;
```

## Rule 4: Agent must not perform I/O error handling directly

Bad:

```rust
let content = match std::fs::read_to_string(path.value()) {
    Ok(c) => c,
    Err(_) => String::new(),
};
```

Good:

```rust
match self.reader.read(file) {
    Ok(content) => { /* delegate to capability */ }
    Err(err) => { results.push(<ResultVO>::from_read_error(file, err)); }
}
```

The agent calls a port. The port implementation lives in infrastructure.

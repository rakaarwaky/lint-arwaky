# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```rust
let result = checker.check().unwrap_or_default();
```

## Rule 2: Agent may return `Vec<LintResult>` for analysis orchestration

```rust
fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
    let mut violations = Vec::new();
    for file in request.files() {
        match self.analyzer.analyze(file) {
            Ok(result) => violations.extend(result.into_violations()),
            Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
        }
    }
    violations
}
```

## Rule 3: Agent may return `Result` for execution orchestration

```rust
fn run(&self, request: &ScanRequest) -> Result<ExecutionReport, AgentExecutionError>;
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
    Err(err) => { violations.push(LintResult::from_read_error(file, err)); }
}
```

The agent calls a port. The port implementation lives in infrastructure.

# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```typescript
const result = checker.check() ?? '';
```

## Rule 2: Agent may return `<ResultVO>[]` for analysis orchestration

```typescript
execute(request: <ScanRequest>VO): <ResultVO>[] {
    const results: <ResultVO>[] = [];
    for (const file of request.files()) {
        try {
            const result = this.analyzer.analyze(file);
            results.push(...result.intoResults());
        } catch (err) {
            results.push(<ResultVO>.fromAnalysisError(file, err));
        }
    }
    return results;
}
```

## Rule 3: Agent may return `Result` for execution orchestration

```typescript
run(request: <ScanRequest>VO): Result<ExecutionReport, AgentExecutionError> { ... }
```

## Rule 4: Agent must not perform I/O error handling directly

Bad:

```typescript
try {
    const content = fs.readFileSync(path.value());
} catch {
    const content = '';
}
```

Good:

```typescript
try {
    const content = this.reader.read(file);
} catch (err) {
    results.push(<ResultVO>.fromReadError(file, err));
}
```

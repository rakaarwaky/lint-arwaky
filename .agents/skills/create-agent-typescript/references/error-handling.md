# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```typescript
const result = checker.check() ?? '';
```

## Rule 2: Agent may return `LintResult[]` for analysis orchestration

```typescript
execute(request: ScanRequest): LintResult[] {
    const violations: LintResult[] = [];
    for (const file of request.files()) {
        try {
            const result = this.analyzer.analyze(file);
            violations.push(...result.intoViolations());
        } catch (err) {
            violations.push(LintResult.fromAnalysisError(file, err));
        }
    }
    return violations;
}
```

## Rule 3: Agent may return `Result` for execution orchestration

```typescript
run(request: ScanRequest): Result<ExecutionReport, AgentExecutionError> { ... }
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
    violations.push(LintResult.fromReadError(file, err));
}
```

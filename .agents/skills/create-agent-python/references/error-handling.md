# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```python
result = checker.check() or ""
```

## Rule 2: Agent may return `list[LintResult]` for analysis orchestration

```python
def execute(self, request: ScanRequest) -> list[LintResult]:
    violations: list[LintResult] = []
    for file in request.files():
        try:
            result = self.analyzer.analyze(file)
            violations.extend(result.into_violations())
        except Exception as e:
            violations.append(LintResult.from_analysis_error(file, e))
    return violations
```

## Rule 3: Agent may return `Result` for execution orchestration

```python
def run(self, request: ScanRequest) -> Result[ExecutionReport, AgentExecutionError]: ...
```

## Rule 4: Agent must not perform I/O error handling directly

Bad:

```python
try:
    content = open(path.value()).read()
except:
    content = ""
```

Good:

```python
try:
    content = self.reader.read(file)
except Exception as e:
    violations.append(LintResult.from_read_error(file, e))
```

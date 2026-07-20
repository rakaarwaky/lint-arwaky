# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```python
result = checker.check() or ""
```

## Rule 2: Agent may return `list[<ResultVO>]` for analysis orchestration

```python
def execute(self, request: <ScanRequest>VO) -> list[<ResultVO>]:
    results: list[<ResultVO>] = []
    for file in request.files():
        try:
            result = self.analyzer.analyze(file)
            results.extend(result.into_results())
        except Exception as e:
            results.append(<ResultVO>.from_analysis_error(file, e))
    return results
```

## Rule 3: Agent may return `Result` for execution orchestration

```python
def run(self, request: <ScanRequest>VO) -> Result[ExecutionReport, AgentExecutionError]: ...
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
    results.append(<ResultVO>.from_read_error(file, e))
```

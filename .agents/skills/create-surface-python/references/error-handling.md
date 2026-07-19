# Error Handling Rules

## Forbidden

```python
self._runner.run(request)  # BAD: no error handling
```

Forbidden:

```python
state = result or UiState.idle()  # BAD: silently discards error
```

## Preferred: Return Result

```python
def handle(self, event: TuiEvent) -> Result[UiState, SurfaceError]:
    try:
        report = self._runner.run(self._request)
        return Ok(UiState.from_report(report))
    except Exception as e:
        return Err(SurfaceError.execution(e))
```

## Preferred: Update Error State

```python
def handle(self, event: TuiEvent) -> UiState:
    try:
        report = self._runner.run(self._request)
        return UiState.from_report(report)
    except Exception as err:
        return UiState.error(ErrorMessage.from_err(err))
```

The surface must not silently discard errors.

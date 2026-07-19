# Error Handling Rules

## Forbidden

```typescript
this.runner.run(request); // BAD: no error handling
```

Forbidden:

```typescript
const state = result ?? UiState.idle(); // BAD: silently discards error
```

## Preferred: Return Result

```typescript
handle(event: TuiEvent): Result<UiState, SurfaceError> {
    try {
        const report = this.runner.run(this.request);
        return Ok(UiState.fromReport(report));
    } catch (e) {
        return Err(SurfaceError.execution(e));
    }
}
```

## Preferred: Update Error State

```typescript
handle(event: TuiEvent): UiState {
    try {
        const report = this.runner.run(this.request);
        return UiState.fromReport(report);
    } catch (err) {
        return UiState.error(ErrorMessage.fromErr(err));
    }
}
```

The surface must not silently discard errors.

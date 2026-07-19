# Error Handling Rules

## Forbidden

```rust
self.runner.run(&request).unwrap();
```

Forbidden:

```rust
let state = result.unwrap_or_default();
```

## Preferred: Return Result

```rust
pub fn handle(&self, event: &TuiEvent) -> Result<UiState, SurfaceError> {
    let report = self.runner.run(&self.request)
        .map_err(SurfaceError::execution)?;
    Ok(UiState::from_report(report))
}
```

## Preferred: Update Error State

```rust
pub fn handle(&self, event: &TuiEvent) -> UiState {
    match self.runner.run(&self.request) {
        Ok(report) => UiState::from_report(report),
        Err(err) => UiState::error(ErrorMessage::from(err)),
    }
}
```

The surface must not silently discard errors.

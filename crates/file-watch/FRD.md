# FRD — file-watch

## Feature Goal
The file-watch crate provides a filesystem monitoring system that detects file changes in real time and automatically re-triggers the linting pipeline. It uses notify and notify-debouncer-mini to optimize performance and avoid redundant processing during rapid changes.

## Requirements & Scope
- In scope:
  - Recursive filesystem watching of project paths for create/modify/delete events.
  - Debounced event aggregation so rapid changes do not trigger multiple lint runs.
  - Automatic re-trigger of the linting pipeline on detected changes.
  - Configurable watch roots and ignore patterns.
  - Low resource usage during long-running watches.
- Out of scope:
  - The lint analysis itself (delegated to the rule crates).
  - Serving results over MCP or building the TUI.

## Success Indicators
- [ ] Responsiveness — file changes detected within 100ms–2s depending on debouncing.
- [ ] Debouncing effectiveness — rapid changes do not trigger multiple lint runs.
- [ ] Resource efficiency — memory usage remains low during long-running watches.
- [ ] Rule conformance — the crate itself passes AES rule checks when complete.

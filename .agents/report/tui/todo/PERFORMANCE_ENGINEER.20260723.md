# Review Report: tui-lint-arwaky — Performance Engineer

## Summary

The `tui-lint-arwaky` crate provides the Ratatui interactive terminal user interface. Performance issues include UI thread freezes during recursive directory loading and frame redraw overhead.

## Performance Profile Analysis

- **Responsiveness:** Delayed on large directory loads due to synchronous `read_dir` on the UI thread.
- **CPU Utilization:** High rendering CPU usage from unconditional `draw()` calls per poll tick.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🔴 CRITICAL | Unconditional `terminal.draw()` on every poll tick | `surface_tui_command.rs:100` | Render only when `needs_redraw` flag is set |
| 2 | 🟡 WARNING | 50ms polling delay in crossterm event loop | `surface_tui_command.rs:115` | Use event-driven channel loop |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🟡 WARNING | Entire file loaded into memory for preview pane | `surface_preview_view.rs` | Window file reading to 500 lines around cursor |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 4 | 🔴 CRITICAL | Synchronous `load_directory` blocks main UI thread | `capabilities_action_handler.rs:45` | Spawn background thread for directory scanning |

### Concurrency & Parallelism

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 5 | 🟡 WARNING | Main thread handles UI rendering and disk traversal | `surface_tui_command.rs:79` | Separate UI thread from directory scanner task |

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Blocking I/O operations directly on the main UI thread.

## Action Items

- [ ] High Priority: Offload `load_directory` tree scanning to a background task/thread.
- [ ] High Priority: Add `needs_redraw` flag to skip redundant `terminal.draw()` calls.
- [ ] Medium Priority: Window file loading in `PreviewView` to limit memory allocation.

## Fixed Code

```rust
// Fixed background directory scanning to keep UI thread responsive
pub fn load_directory_async(state: &mut AppState, path: &str, tx: std::sync::mpsc::Sender<AppStateUpdate>) {
    let path_owned = path.to_string();
    std::thread::spawn(move || {
        let tree = utility_file_system::build_tree(&path_owned);
        let _ = tx.send(AppStateUpdate::DirectoryLoaded(tree));
    });
}
```

---

## Detailed Audit Findings

# Performance Audit: tui-lint-arwaky

## Summary

**Crate:** tui-lint-arwaky
**Files audited:** 16 (src only, excluding tests/benches)
**Performance issues found:** 2 high impact, 2 moderate impact

---

## Critical Issues

### 1. Synchronous File System Directory Scan on UI Thread — HIGH IMPACT
**Location:** `surface_tui_command.rs` (run), `capabilities_action_handler.rs` (load_directory), `utility_file_system.rs`

**Problem:** `self.tui_aggregate.load_directory(&mut state, &cwd)` executes synchronously on the main TUI rendering thread. `load_directory` calls `std::fs::read_dir` recursively to construct the tree view model. When launching the TUI in a large repository or monorepo, the UI thread freezes completely during initial startup or path navigation.

```rust
state.show_path_dialog = false;
self.tui_aggregate.load_directory(&mut state, &cwd); // Blocks main UI thread during dir scan!
```

**Fix:** Offload `load_directory` to a background thread (`std::thread::spawn` or Tokio task) and send scanned tree updates via an MPSC channel to `AppState`.

### 2. Thread Polling Latency Spikes in Event Loop — HIGH IMPACT
**Location:** `surface_tui_command.rs` (event_loop)

**Problem:** `event_loop()` uses `event::poll(Duration::from_millis(50))` combined with a non-blocking `try_recv()` on `scan_rx`. When an interactive scan is running, polling every 50ms introduces up to 50ms of UI latency for key press responses, making keyboard navigation feel sluggish.

```rust
if crossterm::event::poll(Duration::from_millis(50))? { // 50ms event polling delay
    // handle event
}
```

**Fix:** Use an event-driven loop with an explicit crossbeam/tokio channel event queue that wakes up immediately on key events or scan progress updates.

---

## Moderate Issues

### 3. Redundant Full Screen Redraws — MODERATE IMPACT
**Location:** `surface_tui_command.rs` (event_loop)

**Problem:** `terminal.draw(|f| ...)` is called on every loop iteration regardless of whether user input was processed or scan state actually changed, causing unnecessary CPU rendering work.

**Fix:** Set a `state.needs_redraw` boolean flag and render frames only when state mutates or terminal resizes.

### 4. Unbounded File Preview Memory Allocation — LOW/MODERATE IMPACT
**Location:** `surface_preview_view.rs`, `capabilities_lint_executor.rs`

**Problem:** `PreviewView` loads target files completely into memory as `Vec<String>` lines without line windowing (e.g., loading lines 1..100). Opening minified JS files or large log files in the preview pane causes noticeable memory allocation and rendering slowdowns.

**Fix:** Read file content using a line window limit (e.g., maximum 500 lines around active cursor) instead of reading the entire file.

---

## Positive Findings

- Clean surface/capabilities decomposition for Ratatui TUI components (`FileListView`, `PreviewView`, `TreeView`, `StatusComponent`).
- Terminal raw mode setup and teardown handles panic cleanup properly (`disable_raw_mode`).

---

## Estimated Impact

**Worst-case scenario:** Opening TUI in a 10,000-file repository freezes the interface for 1-3 seconds during `load_directory`. Navigating with keyboard incurs 50ms per-keystroke rendering lag.

**Priority fix:** Offload `load_directory` tree scanning to a background thread and render UI frames only when state changes.

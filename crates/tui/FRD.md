# FRD — tui

## Feature Goal
The tui crate is a state-driven 3-panel Ratatui file browser that replaces the flat-menu dialoguer TUI. It provides real-time AES architecture linting with navigation, file preview, and all CLI commands mapped to keyboard shortcuts, plus mouse interaction for panel focus, scrolling, and scrollbar navigation.

## Requirements & Scope
- Core layout — 3-panel tree (left 20%) / file list (middle 35%) / preview (right 45%); header bar, 3-row shortcut bar, bottom status bar.
- Navigation — j/k/h/l/Enter/Home/End, Tab/BackTab panel focus cycling, q/Esc quit.
- Scroll behavior — context-aware arrow keys, PageUp/PageDown (10 lines), mouse wheel, click/drag scrollbar, clamped scroll bounds.
- Lint actions — c (check), s (scan, background), f (fix), t (ci), o (orphan), Ctrl+S (security), Ctrl+D (duplicates), Ctrl+P (dependencies), plus global d/i/I/m/C/H/U/a/v/w mapped to CLI commands; results shown in preview panel with violation count in status bar.
- Copy actions — y (copy preview), Ctrl+Y (copy + save lint-results.txt).
- File display — colored layer badges, / suffix for dirs, file size/extension, dirs sorted first.
- Overlays — search mode (/), help overlay (?), startup path dialog.
- Mouse support — click focus, wheel scroll, scrollbar drag.
- Future (not in this scope) — live file-watch auto-refresh, tree-panel interactive selection/navigation, multi-file selection, sort options name/layer/violations.

## Success Indicators
- [ ] Build & quality — cargo build --release clean; clippy --all-targets -- -D warnings 0 warnings; cargo run --bin lint-arwaky-cli -- check crates/tui 0 violations; cargo test --workspace pass; cargo fmt --all -- --check formatted.
- [ ] Functional — TUI launches with 3-panel layout; j/k/h/l/Enter and Tab/BackTab work; all lint actions run and display results; layer badges render; path dialog works; q quits and restores terminal; search and help overlays work.
- [ ] Scroll & mouse — preview scrolls via arrows when focused; PageUp/Down scroll 10 lines; mouse wheel and scrollbar drag work; scroll clamped to bounds; Home/End jump to top/bottom.
- [ ] Actions — check/scan/fix execute and update the status bar with violation counts; scan runs in background with progress.

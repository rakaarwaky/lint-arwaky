# Feature Requirement Document — TUI (lint-arwaky-tui)

## 1. Feature Goal

State-driven 3-panel Ratatui file browser that replaces the flat-menu dialoguer TUI. Provides real-time AES architecture linting with navigation, file preview, and all CLI commands mapped to keyboard shortcuts.

## 2. Requirements & Scope

### Core

- 3-panel layout: tree (left 20%) / file list (middle 35%) / preview (right 45%)
- Navigation: j/k move, h/l back/forward, Enter open dir, Tab cycle focus, q quit
- Path dialog on startup for project root selection
- Scroll handling for all panels

### Lint Actions

- `c` check, `s` scan, `f` fix, `d` duplicates, `r` orphan, `v` version
- `p` config show, `a` adapters, `e` security, `n` dependencies
- `g` install hook, `G` uninstall hook, `m` doctor
- Results displayed in preview panel with violation count in status bar

### File Display

- Layer badges (colored tags) for taxonomy/contract/capabilities/etc.
- Directory entries show `/` suffix
- File size and extension display

### Overlays

- Search mode: `/` to start, incremental filtering, Enter confirm, Esc cancel
- Help overlay: `?` toggle, context-sensitive shortcuts

### Infrastructure

- All TUI types (taxonomy, contract) live in `shared/src/tui/`
- TUI crate contains only capabilities, infrastructure, agent, surfaces, root
- `init_global_checker()` called before `CodeAnalysisContainer::new()` in container
- `CliContainer` centralizes DI wiring for CLI entry point

### Out of Scope

- Sort options (name/layer/violations) — future
- Mouse click panel focus — basic support exists, not required
- Live file watch auto-refresh — future

## 3. Success Indicators

- `cargo build --release` — clean
- `cargo clippy --all-targets -- -D warnings` — 0 warnings
- `cargo run --bin lint-arwaky-cli -- check crates/tui` — 0 violations
- `cargo test --workspace` — all pass
- TUI launches with 3-panel layout
- Navigation (j/k/h/l/Enter) works
- All lint actions execute and show results
- Layer badges show colored tags
- Path dialog works on startup
- `q` quits and terminal restores cleanly

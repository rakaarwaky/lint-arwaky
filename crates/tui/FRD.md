# Feature Requirement Document — TUI (lint-arwaky-tui)

See [README.md](../../../README.md) for TUI usage and [AUDIT.md](AUDIT.md) for current audit findings.

## 1. Feature Goal

State-driven 3-panel Ratatui file browser that replaces the flat-menu dialoguer TUI. Provides real-time AES architecture linting with navigation, file preview, and all CLI commands mapped to keyboard shortcuts. Supports mouse interaction for panel focus, scrolling, and scrollbar navigation.

## 2. Requirements & Scope

### Core Layout

- 3-panel layout: tree (left 20%) / file list (middle 35%) / preview (right 45%)
- Header bar: shows current path and `[q/Esc] Quit`
- Shortcut bar: 3 rows of key hints below the panels
- Status bar: bottom row with status messages

### Navigation

| Key                     | Action                                                      |
| ----------------------- | ----------------------------------------------------------- |
| `j` / `Down`            | Move down (file list or preview scroll)                     |
| `k` / `Up`              | Move up (file list or preview scroll)                       |
| `h` / `Left`            | Navigate back (parent directory)                            |
| `l` / `Right` / `Enter` | Open directory or preview file                              |
| `Home`                  | Jump to top (first entry or scroll to top)                  |
| `End`                   | Jump to bottom (last entry or scroll to bottom)             |
| `Tab`                   | Cycle panel focus forward: Tree → FileList → Preview → Tree |
| `BackTab` (Shift+Tab)   | Cycle panel focus backward                                  |
| `q` / `Esc`             | Quit                                                        |

### Scroll Behavior

- Arrow keys `j`/`k` are **context-aware**: scrolls preview when `panel_focus == Preview`, moves file list selection otherwise
- `PageUp` / `PageDown`: scrolls preview panel by 10 lines
- `Home` / `End` in Preview focus: jump to top/bottom of preview content
- Mouse scroll wheel: scrolls whichever panel has focus
- Mouse click on preview panel: jumps to proportional scroll position
- Mouse drag on scrollbar area (rightmost 3 columns): scrubs through preview content
- Scroll position is clamped to content bounds (never overflows)

### Lint Actions

| Key      | Action                                 | Scope         |
| -------- | -------------------------------------- | ------------- |
| `c`      | check — AES compliance                 | selected path |
| `s`      | scan — multi-adapter scan (background) | selected path |
| `f`      | fix — auto-fix violations              | selected path |
| `t`      | ci — CI mode with threshold            | selected path |
| `o`      | orphan — dead code detection           | selected path |
| `Ctrl+S` | security — vulnerability scan          | selected path |
| `Ctrl+D` | duplicates — duplication detection     | selected path |
| `Ctrl+P` | dependencies — dependency analysis     | selected path |
| `d`      | doctor — environment diagnostics       | global        |
| `i`      | init — create config files             | global        |
| `I`      | install — install adapter dependencies | global        |
| `m`      | mcp-config — generate MCP config       | global        |
| `C`      | config-show — display current config   | global        |
| `H`      | install-hook — install git hook        | global        |
| `U`      | uninstall-hook — remove git hook       | global        |
| `a`      | adapters — list available adapters     | global        |
| `v`      | version — show version info            | global        |
| `w`      | watch — file watch (redirects to CLI)  | global        |

- All results displayed in preview panel
- Violation count shown in status bar after action completes
- `scan` runs in background thread with progress indicator

### Copy Actions

| Key      | Action                                                 |
| -------- | ------------------------------------------------------ |
| `y`      | Copy preview content to clipboard                      |
| `Ctrl+Y` | Copy preview to clipboard + save to `lint-results.txt` |

### File Display

- Layer badges (colored tags) for taxonomy/contract/capabilities/etc.
- Directory entries show `/` suffix
- File size and extension display
- Directories sorted first, then alphabetically

### Overlays

- **Search mode**: `/` to start, incremental file filtering, `Enter` confirm, `Esc` cancel, `Backspace` delete
- **Help overlay**: `?` toggle, shows all keyboard shortcuts
- **Path dialog**: shown on startup, type project root or `Tab` to use CWD

### Mouse Support

| Input                   | Action                                      |
| ----------------------- | ------------------------------------------- |
| Left click on file list | Select entry + focus FileList               |
| Left click on preview   | Jump to proportional scroll + focus Preview |
| Left click on scrollbar | Jump to position + focus Preview            |
| Scroll wheel            | Scroll focused panel (preview or file list) |
| Left drag on scrollbar  | Scrub through preview content               |

### Infrastructure

- All TUI types (taxonomy, contract) live in `shared/src/tui/`
- TUI crate contains only capabilities, infrastructure, agent, surfaces, root
- `init_global_checker()` called before `CodeAnalysisContainer::new()` in container
- Terminal size tracked via `Resize` events for mouse coordinate mapping
- Non-blocking scan via background thread with `std::sync::mpsc` channel

### Out of Scope

- Sort options (name/layer/violations) — future
- Live file watch auto-refresh — future
- Tree panel interaction — display only, no selection/navigation yet
- Multi-file selection — future

## 3. Success Indicators

### Build & Quality

- `cargo build --release` — clean, no errors
- `cargo clippy --all-targets -- -D warnings` — 0 warnings
- `cargo run --bin lint-arwaky-cli -- check crates/tui` — 0 violations
- `cargo test --workspace` — all pass
- `cargo fmt --all -- --check` — formatted

### Functional

- TUI launches with 3-panel layout
- Navigation (j/k/h/l/Enter) works correctly
- Panel focus cycling via Tab/BackTab works
- All lint actions execute and display results in preview panel
- Layer badges show colored tags for file entries
- Path dialog works on startup
- `q` quits and terminal restores cleanly
- Search mode filters files incrementally
- Help overlay toggles with `?`

### Scroll & Mouse

- Preview panel scrolls via arrow keys when focused
- PageUp/PageDown scroll preview by 10 lines
- Mouse scroll wheel scrolls the focused panel
- Mouse click on preview jumps to proportional position
- Mouse drag on scrollbar scrubs through content
- Scroll position never overflows (clamped to content bounds)
- Home/End in preview focus jump to top/bottom

### Actions

- `check` shows AES violations in preview
- `scan` runs in background with progress, results appear when done
- `fix` auto-fixes and shows updated results
- Status bar updates after each action completes
- Violation count displayed in status bar

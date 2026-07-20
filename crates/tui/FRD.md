# FRD — tui

## Feature Goal
State-driven 3-panel Ratatui file browser that replaces the flat-menu dialoguer TUI. Provides real-time AES architecture linting with navigation, file preview, and all CLI commands mapped to keyboard shortcuts. Supports mouse interaction for panel focus, scrolling, and scrollbar navigation.

## Requirements & Scope
- Core Layout
  - 3-panel layout: tree (left 20%) / file list (middle 35%) / preview (right 45%)
  - Header bar: shows current path and `[q/Esc] Quit`
  - Shortcut bar: 3 rows of key hints below the panels
  - Status bar: bottom row with status messages
- Navigation

  | Key                          | Action                                             |
  | ---------------------------- | -------------------------------------------------- |
  | `j` / `Down`                 | Move down (file list or preview scroll)            |
  | `k` / `Up`                   | Move up (file list or preview scroll)              |
  | `h` / `Left`                 | Navigate back (parent directory)                   |
  | `l` / `Right` / `Enter`      | Open directory or preview file                     |
  | `Home`                       | Jump to top (first entry or scroll to top)         |
  | `End`                        | Jump to bottom (last entry or scroll to bottom)    |
  | `Tab`                        | Cycle panel focus forward: Tree → FileList → Preview → Tree |
  | `BackTab` (Shift+Tab)        | Cycle panel focus backward                         |
  | `q` / `Esc`                  | Quit                                               |

- Scroll Behavior
  - Arrow keys `j`/`k` are context-aware: scrolls preview when `panel_focus == Preview`, moves file list selection otherwise
  - `PageUp` / `PageDown`: scrolls preview panel by 10 lines
  - `Home` / `End` in Preview focus: jump to top/bottom of preview content
  - Mouse scroll wheel: scrolls whichever panel has focus
  - Mouse click on preview panel: jumps to proportional scroll position
  - Mouse drag on scrollbar area (rightmost 3 columns): scrubs through preview content
  - Scroll position is clamped to content bounds (never overflows)
- Lint Actions

  | Key        | Action                                  | Scope         |
  | ---------- | --------------------------------------- | ------------- |
  | `c`        | check — AES compliance                  | selected path |
  | `s`        | scan — multi-adapter scan (background)  | selected path |
  | `f`        | fix — auto-fix violations               | selected path |
  | `t`        | ci — CI mode with threshold             | selected path |
  | `o`        | orphan — dead code detection            | selected path |
  | `Ctrl+S`   | security — vulnerability scan           | selected path |
  | `Ctrl+D`   | duplicates — duplication detection      | selected path |
  | `Ctrl+P`   | dependencies — dependency analysis      | selected path |
  | `d`        | doctor — environment diagnostics        | global        |
  | `i`        | init — create config files              | global        |
  | `I`        | install — install adapter dependencies  | global        |
  | `m`        | mcp-config — generate MCP config        | global        |
  | `C`        | config-show — display current config    | global        |
  | `H`        | install-hook — install git hook         | global        |
  | `U`        | uninstall-hook — remove git hook        | global        |
  | `a`        | adapters — list available adapters      | global        |
  | `v`        | version — show version info             | global        |
  | `w`        | watch — file watch (redirects to CLI)   | global        |

  - All results displayed in preview panel
  - Violation count shown in status bar after action completes
  - `scan` runs in background thread with progress indicator
- Copy Actions

  | Key        | Action                                          |
  | ---------- | ----------------------------------------------- |
  | `y`        | Copy preview content to clipboard               |
  | `Ctrl+Y`   | Copy preview + save to `lint-results.txt`       |

- File Display
  - Layer badges (colored tags) for taxonomy/contract/capabilities/etc.
  - Directory entries show `/` suffix
  - File size and extension display
  - Directories sorted first, then alphabetically
- Overlays
  - Search mode: `/` to start, incremental file filtering, `Enter` confirm, `Esc` cancel, `Backspace` delete
  - Help overlay: `?` toggle, shows all keyboard shortcuts
  - Path dialog: shown on startup, type project root or `Tab` to use CWD
- Mouse Support

  | Input                   | Action                                      |
  | ----------------------- | ------------------------------------------- |
  | Left click on file list | Select entry + focus FileList               |
  | Left click on preview   | Jump to proportional scroll + focus Preview |
  | Left click on scrollbar | Jump to position + focus Preview            |
  | Scroll wheel            | Scroll focused panel (preview or file list) |
  | Left drag on scrollbar  | Scrub through preview content               |

- Future (not in this scope): live file-watch auto-refresh; tree panel interactive selection/navigation (display only); multi-file selection; sort options (name/layer/violations).

## Success Indicators
- [ ] Build & quality — `cargo build --release` clean; `cargo clippy --all-targets -- -D warnings` 0 warnings; `cargo run --bin lint-arwaky-cli -- check crates/tui` 0 violations; `cargo test --workspace` pass; `cargo fmt --all -- --check` formatted.
- [ ] Functional — TUI launches with 3-panel layout; j/k/h/l/Enter and Tab/BackTab work; all lint actions run and display results; layer badges render; path dialog works; q quits and restores terminal; search and help overlays work.
- [ ] Scroll & mouse — preview scrolls via arrows when focused; PageUp/Down scroll 10 lines; mouse wheel and scrollbar drag work; scroll clamped to bounds; Home/End jump to top/bottom.
- [ ] Actions — check/scan/fix execute and update the status bar with violation counts; scan runs in background with progress.

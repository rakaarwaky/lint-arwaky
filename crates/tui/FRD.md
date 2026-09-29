# FRD — tui (v1.0.0)

---

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- DESIGN: [DESIGN.md](DESIGN.md)

## System Overview

The TUI crate provides an interactive terminal-based interface to lint-arwaky. It presents a three-panel layout (tree, file list, preview) with keyboard-driven navigation, search, and lint action execution. Actions run on background threads to keep the event loop responsive.

### Design Goals

| Goal | Mechanism |
|------|-----------|
| Non-blocking | All lint operations run in background worker threads |
| Discoverable | Help overlay, context-sensitive shortcut bar, NO_COLOR support |
| Safe | Destructive actions require explicit confirmation gate |
| Cross-platform | No external GUI dependencies; uses ratatui + crossterm |

### Architecture Layers

```
Entry point → root layer: initializes application state and starts the TUI surface
Event surface → surface layer: event loop, key routing, and render orchestration
State machine → surface layer: maps user events to concrete state mutations
File list view → surface layer: renders file and directory listings
Preview panel → surface layer: displays lint results, file content, or help overlay
Path dialog → surface layer: modal overlay for project-root path input
Tree panel → surface layer: display-only tree visualization
Status bar → surface layer: bottom status information display
Shortcut bar → surface layer: bottom shortcut hint display
Lint executor bridge → surface layer: connects TUI events to lint execution
Theme provider → utility layer: design tokens and NO_COLOR palette switching
Filesystem helpers → utility layer: filesystem utility functions
```

**Shared types:** `crates/shared/src/tui/` — `TuiEvent`, `AppState`, `PreviewMode`, `PanelFocus`, `FileEntry`, `ConfirmState`, `ScanUpdate`, `WatchMessage`, `ActionFlags`, `AdapterInfo`, `AesLayer`.

### Key Invariants (I0–I9)

| ID | Description |
|----|-------------|
| I0 | Event loop runs on the main thread; all I/O goes through channels |
| I1 | Search mode displays live query in file-list title bar |
| I2 | Path dialog is modal — no other keys accepted while visible |
| I3 | Scan can be cancelled mid-flight via Esc |
| I4 | Pressing `r` re-opens the path dialog with current root pre-filled |
| I5 | Destructive actions (live fix, install/uninstall hook) require confirm gate |
| I6 | Watch mode is not supported in TUI; user sees a message instead |
| I7 | Color palette respects `NO_COLOR`; glyphs have ASCII fallbacks when set |
| I8 | Panel layout percentages sourced from theme constants |
| I9 | Help overlay is modal — action keys are blocked until `?` or `q` closes it |

---

## Functional Requirements

### FR-TUI-001: File Browser Panel

- **Description**: Display all entries in the current directory, sorted with directories first then alphabetically.
- **Input**: Directory path from `AppState.current_dir`.
- **Output**: Scrollable list of file/directory names with AES layer badges and violation count.
- **Business Rules**:
  - Hidden files (starting with `.`) are excluded from the listing.
  - Directories are always listed above files.
  - Within each group, alphabetical order (case-insensitive).
  - The file panel title shows the live search query when search mode is active.
- **Edge Cases**:
  - Empty or inaccessible directory → status bar shows "Empty or inaccessible: <path>".
  - Directory not found during navigation → falls back to project root.
- **Error Handling**: Read failures from the filesystem aggregate are silently ignored and result in an empty list.

### FR-TUI-002: Preview Panel

- **Description**: Right panel showing context-sensitive content based on `PreviewMode`.
- **Input**: Current `PreviewMode` value and associated `preview_text` string.
- **Output**: Rendered paragraph in the preview panel area with optional scrollbar.
- **Business Rules**:
  - `FileContent`: up to 100 lines of the selected file with line numbers.
  - `LintResults`: output from check/scan/fix/ci/orphan/security/dependencies actions.
  - `ActionOutput`: output from doctor/init/install/mcp-config/version/adapters actions.
  - `HelpOverlay`: static keyboard shortcut reference.
- **Edge Cases**:
  - File cannot be read → preview shows "Cannot read file: <error>".
  - Large files (>100 lines) show "… (N more lines)" suffix.
  - Preview text exceeds panel height → vertical scrollbar appears.
- **Error Handling**: File I/O errors produce a user-readable message in the preview panel; no crash.

### FR-TUI-003: Navigation

- **Description**: Keyboard-driven navigation through the file tree and between panels.
- **Input**: `j`/`k`/arrow keys, `h`/`l`/arrows, `Tab`/`BackTab`, `Home`/`End`, `PageUp`/`PageDown`.
- **Output**: Updated selection index and scroll position in the focused panel.
- **Business Rules**:
  - Arrow keys move selection within the file list.
  - `h` navigates to parent directory; `l`/`Enter` enters a directory or selects a file.
  - `Tab`/`BackTab` cycle panel focus between FileList and Preview.
  - Scroll position is clamped to valid range on each move.
- **Edge Cases**:
  - Parent navigation at project root boundary → no-op.
  - List has zero items → no selection possible.
- **Error Handling**: Invalid paths during navigation are silently handled; UI remains in a valid state.

### FR-TUI-004: Search

- **Description**: Incremental file-name search within the current directory.
- **Input**: Typed characters in search mode (toggled with `/`).
- **Output**: Filtered file listing; zero-match fallback message when no files match.
- **Business Rules**:
  - Search mode displays the live query in the file-list title bar.
  - Matching is case-insensitive substring match on file names.
  - `Enter` confirms the search and keeps the filter active (exits search mode).
  - `Esc` cancels search and clears the filter.
- **Edge Cases**:
  - Zero matches → shows "No matches for '<query>' — press Esc to clear filter".
  - Search continues to work after navigating into subdirectories.
  - Empty search query → shows full listing.
- **Error Handling**: No file system I/O performed during filtering; all in-memory.

### FR-TUI-005: Lint Actions

- **Description**: Execute lint commands against the selected path or project root on background threads.
- **Input**: Key press for a lint action (`c`, `s`, `f`, `F`, `t`, `o`, `^S`, `^P`).
- **Output**: Action result displayed in the preview panel; status bar shows progress.
- **Business Rules**:
  - All actions run on background threads; event loop remains responsive (50 ms poll cycle).
  - While a scan is in progress, lint action keys are blocked.
  - `f` runs dry-run fix; `F` runs live fix (gated by confirm prompt).
  - `^S` (Ctrl+S) runs dependency scan; `^P` (Ctrl+P) runs security scan.
- **Edge Cases**:
  - Watch mode (`w`) → shows "Watch mode is not supported in TUI." message.
  - Background thread fails → preview shows "Error: <message>".
  - Cancel an in-flight scan with `Esc` → status shows "Cancelling scan..." and scan stops.
- **Error Handling**: Lint action failures produce a human-readable error message in the preview panel; the TUI does not crash.

### FR-TUI-006: Project Root Dialog

- **Description**: Modal path input dialog shown on startup and when `r` is pressed.
- **Input**: User types a path; Tab fills in the current working directory.
- **Output**: `project_root` and `current_dir` updated on valid confirmation; dialog dismissed on cancel.
- **Business Rules**:
  - Empty input defaults to the current working directory.
  - Invalid paths show a status message; the dialog remains open.
  - `Esc` dismisses the dialog without changing the project root.
  - `r` re-opens the dialog with the current root pre-filled.
- **Edge Cases**:
  - CWD cannot be determined → uses "." as default.
  - Typed path does not exist → status shows "Invalid path — type a directory, or press Tab for current dir".
- **Error Handling**: Invalid paths are reported via status bar message; dialog stays open for re-entry.

### FR-TUI-007: Confirm Gate

- **Description**: Destructive actions require explicit user confirmation before executing.
- **Input**: Key press for a gated action (`F`, `H`, `U`, `I`, `i`).
- **Output**: Confirmation prompt shown in status bar; on `y`/`Enter` the action executes; on `n`/`Esc` it is cancelled.
- **Business Rules**:
  - Gated actions: live fix (`F`), install hook (`H`), uninstall hook (`U`), install adapters (`I`), init (`i`).
  - While a confirm prompt is pending, all other keys are ignored.
  - On confirm, the original action executes immediately.
  - On cancel, the status bar shows "Cancelled: <label>".
- **Edge Cases**:
  - User presses an action key while a confirm is already pending → ignored.
  - Cancel then press the same action key again → new confirm prompt shown.
- **Error Handling**: No separate error handling; the pending action either runs or is discarded.

### FR-TUI-008: Help Overlay

- **Description**: Pressing `?` shows a full help overlay in the preview panel with keyboard shortcut reference.
- **Input**: `?` key press.
- **Output**: Help text displayed in the preview panel; previous preview content is preserved and restored on close.
- **Business Rules**:
  - Help overlay replaces the current preview content.
  - Previous `PreviewMode` is saved so it is restored when the overlay closes.
  - While the overlay is active, only `?`, `q`, `Esc`, and navigation keys are processed.
  - `q` or `Esc` closes the overlay; if the overlay is not active, `q`/`Esc` quits the TUI.
- **Edge Cases**:
  - `?` pressed while overlay is active → closes the overlay (toggle behavior).
  - `q` pressed while overlay is active → closes the overlay, does not quit.
- **Error Handling**: No I/O involved; purely UI state transition.

### FR-TUI-009: NO_COLOR Support

- **Description**: When `NO_COLOR` environment variable is set, all Unicode glyphs fall back to ASCII equivalents.
- **Input**: Environment variable `NO_COLOR` (presence = enabled).
- **Output**: Progress bar, separators, and warning indicators use ASCII characters instead of Unicode.
- **Business Rules**:
  - Progress bar filled block → `=`, empty block → ` `.
  - Vertical separator → `|` instead of `│`.
  - Warning indicator → `!` instead of `⚠`.
  - Palette switches to light-background variant when `NO_COLOR` is set.
- **Edge Cases**:
  - Terminal does not support `NO_COLOR` detection → default (colored) mode used.
  - `NO_COLOR` value is non-empty (e.g. `NO_COLOR=0`) → still treated as enabled (presence matters, value does not).
- **Error Handling**: No error path; fallbacks are pure string substitution.

### FR-TUI-010: Terminal Resize Guard

- **Description**: The TUI adapts to terminal resize events and refuses to draw the full layout below the minimum size.
- **Input**: `Resize(width, height)` event from crossterm.
- **Output**: Layout re-rendered with updated dimensions; below minimum size, a centered message is shown.
- **Business Rules**:
  - Minimum terminal size: 40 columns × 15 rows.
  - Below minimum size: "Terminal too small — resize to at least 40x15" shown centered.
  - Above minimum size: renders the full three-panel layout.
  - Terminal height and width are stored in `AppState` for mouse click coordinate mapping.
- **Edge Cases**:
  - Terminal shrinks below minimum while TUI is running → "too small" message replaces all panels.
  - Terminal resizes back to valid size → layout returns automatically.
- **Error Handling**: Invalid dimensions are handled gracefully; the TUI remains responsive after any resize event.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `run` | `&self` | `anyhow::Result<()>` | Any I/O error from terminal setup | — | Start the TUI event loop |
| `handle` | `&mut AppState, TuiEvent` | — | — | All TuiEvent variants | Process one event through the state machine |
| `start_scan` | `&mut AppState` | `Option<Receiver<ScanUpdate>>` | — | `ActionScan` | Spawn scan on background thread |
| `poll_scan` | `&mut AppState, &Receiver` | — | — | `ScanUpdate::*` | Drain scan progress messages |
| `start_background_action` | `&mut AppState, label, closure` | `bool` | — | Global action events | Spawn global action on background thread |
| `load_directory` | `&mut AppState, path` | — | `read_dir` failure → empty list | `NavigateForward` | Read and sort directory entries |
| `load_file_preview` | `&mut AppState, path` | — | File I/O error → error message in preview | `NavigateForward` | Read up to 100 lines of a file |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|--------|-------|--------|-------|-------|-------------|
| `new` | `&self, lint_port, io` | `SurfaceActionHandler` | — | — | Construct handler with lint executor and filesystem ports |
| `new` | `action_handler` | `TuiCommandSurface` | — | — | Construct the event loop runner |

---

## Integration Points

| System | Direction | Purpose | Failure mode |
|--------|-----------|---------|--------------|
| SurfaceLintExecutor | out | Execute lint actions (check/scan/fix/ci/orphan/security/dependencies) | Action returns failure → preview shows error message |
| Filesystem IO protocol | out | Read directory entries and file contents | Read error → preview shows "Cannot read file: <err>" |
| System clipboard | out | Copy preview text to clipboard via xclip or wl-copy | Clipboard unavailable → status shows fallback message |
| External linters | out | Provide lint results via adapter binaries (Clippy, Ruff, ESLint) | Adapter not installed → status shows install hint |

---

## Non-functional Requirements

| Metric | Target | Measurement method |
|--------|--------|-------------------|
| Event loop latency | ≤ 50 ms between polls | Poll interval duration in event loop source |
| Scan responsiveness | UI does not freeze during scan | Background thread + channel architecture verified in tests |
| Terminal minimum size | 40 columns × 15 rows | Guard in render function refuses to draw below threshold |
| NO_COLOR compliance | All glyphs have ASCII fallback | Visual inspection with `NO_COLOR=1` and automated grep for Unicode chars |
| Help discoverability | All actions documented in `?` overlay | Manual verification against key binding source |
| Confirm-gate correctness | No destructive action executes without confirmation | Test all gated actions (live fix, install hook, uninstall hook, install, init) |
| Preview staleness | Preview clears when directory changes | Visual inspection after navigating to new directory |

---

## Test Scenarios / QA Checklist

- **SCEN-001 — Startup**: Path dialog shown on launch; CWD pre-filled; `Tab` fills path; `Esc` dismisses without quitting
- **SCEN-002 — Navigation**: Arrow keys move selection; `h` goes up; `l`/`Enter` enters directory; scroll wraps
- **SCEN-003 — Search**: `/` enters search mode; typing filters list; zero matches shows hint; `Esc` clears filter
- **SCEN-004 — Lint Actions**: `c` runs check; result appears in preview; `s` runs scan; background thread keeps UI responsive
- **SCEN-005 — Fix Live Gate**: `F` shows confirm prompt; `y` applies fix; `n`/`Esc` cancels without modifying files
- **SCEN-006 — Hook Gate**: `H` and `U` show confirm prompts; both require `y`/`Enter` to proceed
- **SCEN-007 — Help Overlay**: `?` shows help; previous preview mode restored on close; action keys blocked while help is open
- **SCEN-008 — NO_COLOR**: Set `NO_COLOR=1`; verify progress bar uses `=`/` `, separator uses `|`, warning uses `!`
- **SCEN-009 — Resize Guard**: Shrink terminal below 40×15; verify "too small" message; resize back and verify layout returns
- **SCEN-010 — Path Dialog**: `r` re-opens dialog with current root; invalid path shows error; valid path switches root; `Esc` dismisses
- **SCEN-011 — Cancel Scan**: Press `Esc` while scan is running; status shows "Cancelling scan..."; scan stops
- **SCEN-012 — Watch Unsupported**: Press `w`; preview shows "Watch mode is not supported in TUI."

---

## Assumptions & Constraints

- The TUI runs on systems with a UTF-8 locale for proper Unicode rendering.
- `NO_COLOR=1` forces ASCII fallbacks — users on light terminals should set it.
- The TUI is a supplementary entry point; the primary interface is `lint-arwaky-cli`.
- Background threads use bounded sync channels to prevent unbounded memory growth.
- The event loop polls every 50 ms; no async runtime is used.
- Mouse support is present but optional — keyboard navigation is the primary interface.
- File previews are capped at 100 lines to avoid rendering large files.
- The TUI binary name is `lint-arwaky-tui` as defined in the Cargo.toml workspace members.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer coding convention this project enforces
- **PreviewMode**: Enum defining what content the right panel shows (`FileContent`, `LintResults`, `ActionOutput`, `HelpOverlay`)
- **PanelFocus**: Which of the two scrollable panels (FileList, Preview) receives keyboard input
- **ConfirmState**: Holds the pending destructive event and its human-readable label
- **ScanUpdate**: Message from the background scan thread (Progress, Complete, Cancelled)
- **I5**: Confirm-gate invariant — destructive actions require explicit user confirmation
- **I9**: Help overlay invariant — action keys are blocked while the overlay is visible

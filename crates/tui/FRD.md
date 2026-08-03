# FRD — tui (v1.11.0)

---

## System Overview

The tui crate is a **Smart Surface** — a Ratatui-based interactive terminal UI for lint-arwaky. It is a thin wrapper that parses keyboard/mouse input, calls dispatcher/aggregate functions, and renders output. No business logic lives here.

### Architecture

```
TUI (Smart Surface)
  ├→ TuiContainer (root) — wires aggregates, starts event loop
  │    └→ TuiCommandSurface — crossterm event loop + ratatui rendering
  │         └→ SurfaceActionHandler — event → action state machine
  │              └→ SurfaceLintExecutor — facade over domain aggregates
  ├→ Views: FileList, Preview, Tree, Path, Shortcuts, Status
  └→ Utilities: file_system, report_formatter
```

### Dependency Rule

- TUI imports: shared (taxonomies, aggregates), dispatcher (optional)
- TUI must NOT own business logic — delegates to aggregates via DI

---

## Functional Requirements

### FR-001: Terminal Setup & Event Loop

**File**: `surface_tui_command.rs`

**What it produces**: Initialized terminal with raw mode, alternate screen, mouse capture, and blocking event loop.

| Output           | Description                                    |
| ------------------ | ------------------------------------------------ |
| Terminal setup  | Raw mode, alternate screen, mouse capture      |
| Event loop      | Poll crossterm events, dispatch to action handler |
| Clean shutdown  | Disable raw mode, leave alternate screen       |

**Input**: `SurfaceActionHandler` (injected via `TuiCommandSurface`).

**Business Rules**:

- Enables raw mode + alternate screen + mouse capture on start.
- Initializes `AppState` with CWD as project root.
- Sets `terminal_height`/`terminal_width` from `terminal_size()` on startup.
- Event loop polls at 50ms intervals.
- Scans run in background thread via `start_scan()`; other long-running actions blocked during scan.
- Clean shutdown restores terminal state on any exit path.

**Edge Cases**:

- Terminal too small (< 5 rows / 10 cols): mouse clicks silently ignored.
- Scan already running: new scan/action requests silently dropped.

**Error Handling**: `anyhow::Result<()>`.

---

### FR-002: Input Translation

**File**: `surface_tui_command.rs` (functions `from_key_event`, `from_mouse_event`, `from_crossterm_event`)

**What it produces**: `TuiEvent` variants from crossterm events.

| Output     | Description                                    |
| ------------ | ------------------------------------------------ |
| TuiEvent   | Normalized event for the action handler        |

**Input**: `crossterm::event::Event`, `&AppState`.

**Business Rules**:

- **Path dialog mode**: all key input → path editing (Char, Backspace, Enter, Tab, Esc).
- **Search mode**: character input → search query (Char, Backspace, Enter, Esc).
- **Normal mode**: vim-style navigation (j/k/h/l), action keys (c/s/f/t/o/D/d/i/I/m/C/H/U/a/v/y/?/), Ctrl combos.
- **Mouse**: left click → selection, drag → scrollbar, scroll up/down → scroll.
- **Ctrl+ combos**: q=quit, s=security, p=dependencies, y=copy-to-file.
- **Key mapping reference**:

  | Key | TuiEvent | Category |
  |-----|----------|----------|
  | `j`/↓ | MoveDown | Navigation |
  | `k`/↑ | MoveUp | Navigation |
  | `h`/← | NavigateBack | Navigation |
  | `l`/→/Enter | NavigateForward | Navigation |
  | Home/End | MoveTop/MoveBottom | Navigation |
  | PgUp/PgDn | PreviewScrollUp/PreviewScrollDown | Preview |
  | Tab/BackTab | FocusNext/FocusPrev | Focus |
  | `c` | ActionCheck | Lint (path) |
  | `s` | ActionScan | Lint (path) |
  | `f` | ActionFix | Lint (path) |
  | `t` | ActionCi | Lint (path) |
  | `o` | ActionOrphan | Lint (path) |
  | `D` | ActionDuplicates | Lint (path) |
  | `d` | ActionDoctor | Global |
  | `i` | ActionInit | Global |
  | `I` | ActionInstall | Global |
  | `m` | ActionMcpConfig | Global |
  | `C` | ActionConfigShow | Global |
  | `H` | ActionInstallHook | Global |
  | `U` | ActionUninstallHook | Global |
  | `a` | ActionAdapters | Global |
  | `v` | ActionVersion | Global |
  | `y` | CopyToClipboard | Export |
  | `?` | ToggleHelp | UI |
  | `/` | ToggleSearch | UI |
  | Esc/q | Quit | Exit |
  | Ctrl+s | ActionSecurity | Lint (path) |
  | Ctrl+p | ActionDependencies | Lint (path) |
  | Ctrl+y | CopyToFile | Export |

**Edge Cases**:

- Unknown key in any mode: returns `TuiEvent::None`.
- Mouse event when terminal too small: ignored.

**Error Handling**: Infallible — always returns a `TuiEvent`.

---

### FR-003: File Navigation

**File**: `surface_action_handler.rs` (methods `navigate_back`, `navigate_forward`, `load_directory`, `load_file_preview`, `load_preview`)

**What it produces**: Updated `AppState` with directory listings and file previews.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Directory listing  | Sorted entries (dirs first, then alpha)        |
| File preview       | First 100 lines of selected file              |
| Project root boundary | Cannot navigate above project root           |

**Input**: `TuiEvent` variants (MoveDown/Up/Top/Bottom, NavigateBack/Forward).

**Business Rules**:

- `load_directory`: lists directory, sorts dirs-first then alphabetically, resets selection.
- `navigate_forward`: if entry is dir → enter it; if file → load preview.
- `navigate_back`: go to parent, clamped at project root.
- `load_file_preview`: reads up to 100 lines via `utility_file_system::read_file_preview`.
- Preview panel shows file content in `PreviewMode::FileContent`.

**Edge Cases**:

- Empty directory: status shows "Empty or inaccessible".
- Inaccessible directory: same as empty.
- Navigate above project root: no-op.

**Error Handling**: Silent — invalid paths produce status messages, no errors.

---

### FR-004: Lint Action Execution

**File**: `surface_action_handler.rs` (methods `run_action`, `run_action_no_path`, `start_scan`, `poll_scan`)

**What it produces**: `LintExecutionResult` with output text and violation count.

| Output            | Description                                    |
| ------------------- | ------------------------------------------------ |
| Lint results     | Text output + violation count for preview panel |
| Scan progress    | Async progress updates via channel             |
| Action blocking  | Long actions blocked during active scan        |

**Input**: `TuiEvent` variants (ActionCheck/Scan/Fix/Ci/Orphan/Security/Duplicates/Dependencies/…).

**Business Rules**:

- **Path-requiring actions** (check, scan, fix, ci, orphan, security, duplicates, dependencies): use `run_action` with selected path.
- **Global actions** (doctor, init, install, mcp-config, config-show, install-hook, uninstall-hook, adapters, version): use `run_action_no_path`.
- **Scan**: runs in background thread, sends `ScanUpdate::Progress`/`Complete` via `mpsc::sync_channel(16)`.
- **Other actions**: run synchronously, block event loop briefly.
- **Duplicates** (`D` key): delegates to `SurfaceLintExecutor::duplicates()`, which uses `ICodeAnalysisAggregate::scan_duplicate_blocks()` to detect code duplication.
- Watch mode: explicitly unsupported in TUI — pressing `w` shows "Watch mode is not supported in TUI" message.

**Edge Cases**:

- Scan already in progress: new scan request silently dropped.
- Action while scanning: blocked (except quit/resize).
- Fix without `FixOrchestratorAggregate`: falls back to scan-only mode.

**Error Handling**: Embedded in `LintExecutionResult` — never returns `Err`.

---

### FR-005: Domain Aggregate Facade

**File**: `surface_lint_executor.rs`

**What it produces**: `LintExecutionResult` for each lint action, delegating to domain aggregates.

| Output            | Description                                    |
| ------------------- | ------------------------------------------------ |
| check             | Code analysis via `ICodeAnalysisAggregate`     |
| scan              | Comprehensive 6-linter scan                    |
| fix               | Auto-fix via `LintFixOrchestratorAggregate`    |
| ci                | CI threshold validation (quality+import+naming+orphan) |
| orphan            | Orphan file detection                          |
| security          | External lint adapter scan                     |
| duplicates        | Code duplication detection                     |
| dependencies      | Dependency report                              |
| doctor            | Toolchain diagnostics                          |
| init/install      | Project setup                                  |
| mcp-config        | MCP client config generation                   |
| config-show       | Active configuration display                   |
| install/uninstall-hook | Git hook management                       |
| adapters          | External adapter listing                       |
| version           | Version display                                |

**Input**: Action method + path + action flags.

**Business Rules**:

- Builder pattern: `SurfaceLintExecutor::new(...).with_fix(...).with_setup(...).with_maintenance(...)`.
- Optional aggregates: if not injected, methods return CLI fallback messages.
- All methods synchronous — async aggregates suggest CLI alternative.
- `discover_adapters`: scans filesystem for known adapter binaries.

**Edge Cases**:

- Missing aggregate: returns helpful CLI command suggestion.
- Async aggregate called from sync context: suggests CLI.

**Error Handling**: Embedded in `LintExecutionResult`.

---

### FR-006: Search Mode

**File**: `surface_action_handler.rs` (events `ToggleSearch`, `SearchInput`, `SearchBackspace`, `SearchConfirm`, `SearchCancel`)

**What it produces**: Filtered file list based on incremental search query.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Filtered entries    | File list filtered by search query             |
| Search persistence  | Filter persists after Enter, clears on Esc     |

**Input**: Character input events.

**Business Rules**:

- `/` toggles search mode on/off.
- Characters append to `search_query`.
- Backspace removes last character.
- `Enter` confirms search, exits search mode, keeps filter active.
- `Esc` cancels search, clears query and filter.
- `compute_filtered_indices()` recomputes after each input.

**Edge Cases**:

- Empty query: shows all files.
- No matches: empty filtered list.

**Error Handling**: Infallible.

---

### FR-007: Path Dialog

**File**: `surface_action_handler.rs` (events `PathInput`, `PathBackspace`, `PathConfirm`, `PathUseCurrent`)

**What it produces**: Updated project root and current directory from user input.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Project root       | Set from typed path or CWD                    |
| Directory reload   | Loads directory listing for new root           |

**Input**: Character input events (when `show_path_dialog = true`).

**Business Rules**:

- Shown on TUI startup — user types project root or presses Tab for CWD.
- `Enter`: validates path exists, sets as project root, reloads directory.
- `Tab`: uses `std::env::current_dir()` as project root.
- `Esc`: quits TUI.
- Invalid path: shows "Invalid path" status.

**Edge Cases**:

- Path does not exist: "Invalid path" message, dialog stays open.
- Empty input + Enter: same as invalid.

**Error Handling**: Silent validation — status message on failure.

---

### FR-008: Mouse Interaction

**File**: `surface_action_handler.rs` (methods `handle_mouse_click`, `handle_mouse_drag`, `jump_to_scroll_position`)

**What it produces**: Updated panel focus and scroll positions from mouse input.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Panel focus        | Click on panel sets focus                     |
| Scroll position    | Click/drag on scrollbar jumps to position     |
| Selection          | Click on file list selects entry              |

**Input**: `MouseClick`, `MouseDrag`, `MouseScrollUp/Down` events.

**Business Rules**:

- Click on file list area → select entry, set focus to FileList.
- Click on preview area → jump to proportional scroll position, set focus to Preview.
- Click/drag on scrollbar thumb → jump to proportional position.
- Mouse scroll up/down → scroll active panel (preview or file list).
- Layout zones computed from `terminal_height`/`terminal_width`.

**Edge Cases**:

- Terminal too small (`h < 5 || w < 10`): all mouse events ignored.
- Click outside all zones: no-op.

**Error Handling**: Infallible.

---

### FR-009: UI Rendering

**Files**: `surface_file_list_view.rs`, `surface_preview_view.rs`, `surface_tree_view.rs`, `surface_shortcut_component.rs`, `surface_status_component.rs`, `surface_path_screen.rs`

**Note**: Help overlay rendering is handled by `PreviewView` in `HelpOverlay` mode. The `surface_help_screen.rs` module exists but is unused dead code (AES506).

**What it produces**: Ratatui widgets for each panel.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Header bar        | Current path + quit hint                      |
| Tree panel (20%)  | Directory tree view                           |
| File list (35%)   | Sorted/filtered file entries                  |
| Preview (45%)     | File content or lint results                  |
| Shortcuts bar     | Key binding hints                             |
| Status bar        | Current status message                        |
| Path dialog       | Full-screen path input overlay                |
| Help overlay      | Toggle-able help screen                      |

**Input**: `&AppState`.

**Business Rules**:

- Layout: vertical split → header | panels | shortcuts | status.
- Panels: horizontal split → tree (20%) | file_list (35%) | preview (45%).
- Path dialog replaces all panels when `show_path_dialog = true`.
- Help overlay replaces preview when `show_help = true`.
- Preview modes: `ActionOutput`, `LintResults`, `FileContent`, `HelpOverlay`.

**Edge Cases**:

- Terminal resize: layout recalculates, terminal dimensions updated.

**Error Handling**: Infallible rendering.

---

### FR-010: Clipboard & File Export

**File**: `surface_action_handler.rs` (methods `copy_to_clipboard`, `copy_to_file`)

**What it produces**: Copied content to clipboard or `lint-results.txt`.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Clipboard copy     | Preview content → system clipboard via arboard |
| File save          | Preview content → lint-results.txt            |

**Input**: `CopyToClipboard`, `CopyToFile` events.

**Business Rules**:

- `CopyToClipboard`: delegates to `utility_file_system::copy_text_to_clipboard()` (arboard/xclip/wl-copy).
- `CopyToFile`: writes `state.preview_text` to `lint-results.txt` via filesystem aggregate.
- Empty preview: shows "Nothing to copy" status.

**Edge Cases**:

- Clipboard unavailable: "install xclip or wl-copy" message.
- File write fails: "Save failed" status.

**Error Handling**: Status messages on failure.

---

### FR-011: Logging

**File**: `surface_logging_controller.rs`

**What it produces**: Structured tracing logs for TUI events.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Event logging      | Each `TuiEvent` variant logged at debug level  |
| Log initialization | File-based tracing with rotation              |

**Input**: `TuiEvent` variants.

**Business Rules**:

- `init()`: sets up tracing subscriber with file appender.
- `record()`: logs event variant name at `tracing::debug!(target = "tui")`.
- **Integration point**: `record(&tui_event)` is called in the event loop (`surface_tui_command.rs`) immediately after `from_crossterm_event()` translates a crossterm event, and before the event is dispatched to the action handler or intercepted for scan management.
- Events logged: MoveDown, MoveUp, MoveTop, MoveBottom, FocusNext/Prev, NavigateBack/Forward, all Action* events, Quit, Resize, mouse events, search/path input events.

**Edge Cases**:

- Init failure: returns error (non-fatal to TUI startup).

**Error Handling**: `anyhow::Result<()>` for init.

---

### FR-012: Utility Functions

**Files**: `utility_file_system.rs`, `utility_report_formatter.rs`

**What it produces**: Stateless helper functions for file operations and result formatting.

| Output              | Description                                    |
| --------------------- | ------------------------------------------------ |
| Directory listing  | `list_directory` → sorted `DirectoryEntry`     |
| File preview       | `read_file_preview` → first N lines           |
| Parent directory   | `parent_directory` → parent `FilePath`        |
| Valid directory    | `is_valid_directory` → bool                   |
| Clipboard          | `copy_text_to_clipboard` → bool               |
| Result formatting  | `format_results`, `format_config_result`      |
| Human file size    | `file_size_human` → `DisplayContent` (unused in production) |
| Path decomposition | `path_components` → `Vec<FilePath>` (unused in production) |
| Doctor report      | `format_doctor_report` → `LintExecutionResult` (unused in production) |
| Dependency report  | `format_dependency_report` → `LintExecutionResult` (unused in production) |

**Input**: File paths, lint results.

**Business Rules**:

- All functions are stateless free functions (AES406 utility pattern).
- `list_directory`: reads dir entries, returns `DirectoryEntry` with name, path, is_dir.
- `read_file_preview`: reads up to N lines, returns string.
- `format_results`: converts `LintResultList` to display string.
- **Unused functions note**: `file_size_human`, `path_components`, `format_doctor_report`, and `format_dependency_report` are defined but never called from production source code — only exercised by tests. Consider removing or consolidating per AES504.

**Error Handling**: Returns empty/default on failure.

---

## Consumer Access Pattern

TUI is a terminal binary — not consumed as a library. Entry point:

```rust
// Binary main.rs
let container = TuiContainer::new();
container.run(lint_executor, filesystem)?;
```

---

## Non-functional Requirements

- **Performance**: Event loop polls at 50ms. Scan runs in background thread.
- **Memory**: Stateless actions — `AppState` is the only mutable state.
- **Terminal**: Requires terminal with raw mode support. Minimum 5 rows × 10 cols.
- **Dependencies**: ratatui, crossterm, arboard (clipboard), tracing (logging).
- **No Business Logic**: All lint logic delegated to aggregates via DI.

---

## Test Scenarios

### FR-002: Input Translation

| # | Scenario                           | Expected                                    |
| --- | ------------------------------------ | --------------------------------------------- |
| 1 | Press 'j' in normal mode           | TuiEvent::MoveDown                          |
| 2 | Press 's' in normal mode           | TuiEvent::ActionScan                        |
| 3 | Press 's' with Ctrl                | TuiEvent::ActionSecurity                    |
| 4 | Type 'a' in search mode            | TuiEvent::SearchInput('a')                  |
| 5 | Type 'a' in path dialog            | TuiEvent::PathInput('a')                    |
| 6 | Unknown key                         | TuiEvent::None                              |

### FR-003: File Navigation

| # | Scenario                           | Expected                                    |
| --- | ------------------------------------ | --------------------------------------------- |
| 1 | Navigate into directory             | Entries loaded, selection reset              |
| 2 | Navigate into file                  | Preview loaded (up to 100 lines)            |
| 3 | Navigate back from root             | No-op (clamped)                             |
| 4 | Enter empty directory               | "Empty or inaccessible" status              |

### FR-004: Lint Actions

| # | Scenario                           | Expected                                    |
| --- | ------------------------------------ | --------------------------------------------- |
| 1 | Press 'c' (check) on file          | Code analysis results in preview            |
| 2 | Press 's' (scan) on directory      | Background scan with progress updates       |
| 3 | Press 'f' (fix) with dry-run       | Dry-run preview output                      |
| 4 | Press 'w' (watch)                  | "Watch mode not supported in TUI" message   |

---

## Glossary

| Term                          | Definition                                                    |
| ------------------------------- | --------------------------------------------------------------- |
| **Smart Surface**            | Thin UI wrapper — parses input, calls aggregates, renders output |
| **SurfaceLintExecutor**      | Facade over domain aggregates for TUI lint actions             |
| **SurfaceActionHandler**     | Event → action state machine                                   |
| **TuiCommandSurface**        | crossterm event loop + ratatui rendering                       |
| **AppState**                  | Mutable TUI state (selection, scroll, focus, preview)          |
| **TuiEvent**                 | Normalized UI event dispatched to action handler               |
| **LintExecutionResult**      | Output text + violation count from a lint action               |

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

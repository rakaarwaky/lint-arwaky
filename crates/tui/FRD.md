# FRD — tui

## System Overview

A state-driven 3-panel Ratatui terminal UI that provides real-time AES architecture linting with file browsing, preview, and all CLI commands mapped to keyboard shortcuts. Replaces the flat-menu dialoguer TUI with an interactive Ratatui-based interface supporting keyboard and mouse navigation.

```
┌──────────────────────────────────────────────────────────┐
│ lint-arwaky TUI │ Path: /home/user/project  [q/Esc] Quit│
├──────────┬───────────────┬───────────────────────────────┤
│  Tree    │  File List    │  Preview                      │
│  (20%)   │  (35%)        │  (45%)                        │
│          │               │                               │
│  src/    │  lib.rs       │  1 │ // PURPOSE: Module ...   │
│  tests/  │  mod.rs       │  2 │ pub mod surface_...      │
│          │  main.rs      │  3 │                           │
│          │               │                               │
├──────────┴───────────────┴───────────────────────────────┤
│ [c]check [s]scan [f]fix [t]ci [o]orphan [d]doctor  ...  │
│ [y]copy [?]help [/]search                               │
├──────────────────────────────────────────────────────────┤
│ Done: /home/user/project | 0 violations                  │
└──────────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Render 3-Panel Layout
- **Description**: Render a three-panel layout with header, shortcut bar, and status bar.
- **Input**: Application state — current application state.
- **Output**: Ratatui frame rendered to terminal.
- **Business Rules**:
  - Layout proportions: Tree (20%) | File List (35%) | Preview (45%).
  - Header bar (1 row): shows "lint-arwaky TUI | Path: <current_dir> [q/Esc] Quit".
  - Shortcut bar (3 rows): key hints for available actions.
  - Status bar (1 row): current status message (e.g., "Done: <path> | N violations").
  - Path dialog overlay: shown on startup, user types project root or presses Tab for CWD.
- **Edge Cases**:
  - Terminal smaller than 5 rows or 10 columns — mouse click handling disabled.
  - Terminal resize — layout recalculates on next render.
  - Empty directory — file list shows "Empty or inaccessible" status.
- **Error Handling**: Terminal I/O errors propagate from the draw call.

### FR-002: Navigate File List
- **Description**: Navigate the file list panel using keyboard shortcuts with context-aware scrolling.
- **Input**: Key events (j/k, Up/Down, Home/End, PageUp/PageDown).
- **Output**: Updated application state with new selection index, scroll offset, and preview content.
- **Business Rules**:
  - `j` / `Down`: Move down — if Preview focused, scroll preview by 3 lines; otherwise move selection.
  - `k` / `Up`: Move up — if Preview focused, scroll preview by 3 lines; otherwise move selection.
  - `Home`: Jump to top — if Preview focused, scroll to top; otherwise select first entry.
  - `End`: Jump to bottom — if Preview focused, scroll to bottom; otherwise select last entry.
  - Selection change triggers automatic file preview loading.
  - Scroll offset resets to 0 when directory changes.
- **Edge Cases**:
  - Selection at first entry — no further upward movement.
  - Selection at last entry — no further downward movement.
  - Preview scroll at bounds — clamped to valid range.
  - Empty file list — no selection changes.
- **Error Handling**: No error paths; bounds checking prevents overflow.

### FR-003: Navigate Directories
- **Description**: Enter directories and navigate back to parent, clamped to project root boundary.
- **Input**: Key events (h/Left, l/Right/Enter).
- **Output**: Updated application state with new current directory and file listing.
- **Business Rules**:
  - `h` / `Left`: Navigate to parent directory, but only if parent starts with project root.
  - `l` / `Right` / `Enter`: If entry is directory, enter it; if file, load preview.
  - Navigation clamped to project root — cannot go above it.
  - After navigation, file list is re-sorted: directories first, then alphabetically.
- **Edge Cases**:
  - At project root — `h`/`Left` does nothing (parent would be outside boundary).
  - Entry is a symlink to directory — treated as directory.
  - Directory is empty — status bar shows "Empty or inaccessible".
  - Entry is a file — preview loaded in Preview panel.
- **Error Handling**: Directory read failures result in empty listing with status message.

### FR-004: Focus Cycling Between Panels
- **Description**: Cycle keyboard focus between Tree, FileList, and Preview panels.
- **Input**: Tab / BackTab (Shift+Tab).
- **Output**: Updated application state with new panel focus value.
- **Business Rules**:
  - Tab: cycle forward — Tree → FileList → Preview → Tree.
  - BackTab: cycle backward — Tree → Preview → FileList → Tree.
  - Focus determines which panel responds to j/k/Home/End keys.
- **Edge Cases**:
  - Only three panels — cycle wraps after third.
- **Error Handling**: No error paths; pure state transition.

### FR-005: Run Lint Actions (Path-Scoped)
- **Description**: Execute lint actions on the currently selected file or directory.
- **Input**: Key events (c, s, f, t, o, Ctrl+S, Ctrl+D, Ctrl+P).
- **Output**: Updated application state with preview text showing action results and violation count.
- **Business Rules**:
  - `c` → check: AES compliance check on selected path.
  - `s` → scan: Multi-adapter scan (runs in background thread with progress indicator).
  - `f` → fix: Auto-fix violations (supports dry-run/live modes via action flags).
  - `t` → ci: CI mode with configurable threshold (PASS/FAIL status).
  - `o` → orphan: Dead code detection on selected path.
  - `Ctrl+S` → security: Vulnerability scan via external linters.
  - `Ctrl+D` → duplicates: Code duplication detection.
  - `Ctrl+P` → dependencies: Dependency analysis report.
  - All results displayed in Preview panel.
  - Violation count shown in status bar after action completes.
  - `scan` runs in background thread; other long-running actions blocked while scan in progress.
- **Edge Cases**:
  - Scan already running — new scan request ignored.
  - Long-running action during active scan — blocked until scan completes.
  - Action on empty directory — action runs on path, may return zero results.
  - Fix orchestrator not available — fallback to violation scan with message.
- **Error Handling**: Action failures return a lint execution result with success: false and error message.

### FR-006: Run Global Actions
- **Description**: Execute actions that operate globally (not path-scoped).
- **Input**: Key events (d, i, I, m, C, H, U, a, v, w).
- **Output**: Updated application state with preview text showing action results.
- **Business Rules**:
  - `d` → doctor: Environment diagnostics (toolchain check).
  - `i` → init: Create config files for detected languages.
  - `I` → install: Install adapter dependencies (Python/JS).
  - `m` → mcp-config: Generate MCP configuration JSON.
  - `C` → config-show: Display current configuration.
  - `H` → install-hook: Install git pre-commit hook.
  - `U` → uninstall-hook: Remove git pre-commit hook.
  - `a` → adapters: List available linter adapters with install status.
  - `v` → version: Show lint-arwaky version.
  - `w` → watch: Redirect to CLI (not implemented in TUI yet).
- **Edge Cases**:
  - Setup aggregate not available — message directs user to CLI.
  - Git hook operations on non-git repo — error message returned.
  - MCP config serialization failure — error message returned.
- **Error Handling**: Failures return a lint execution result with error message.

### FR-007: Search and Filter Files
- **Description**: Incremental file filtering within the current directory listing.
- **Input**: `/` to start, character input, Backspace, Enter, Esc.
- **Output**: Filtered file list in application state.
- **Business Rules**:
  - `/` toggles search mode; shows search query in UI.
  - Character input appends to search query; filtering is case-insensitive substring match.
  - Backspace removes last character from query.
  - Enter confirms search and exits search mode (keeps filter).
  - Esc cancels search and clears filter.
  - Filtered indices computed via the filtering utility.
- **Edge Cases**:
  - Empty query — all entries shown.
  - No matches — empty file list.
  - Search mode active — all keyboard input goes to search (no navigation).
- **Error Handling**: No error paths; pure string matching.

### FR-008: Mouse Interaction
- **Description**: Support mouse clicks, scroll wheel, and drag for panel interaction and scrolling.
- **Input**: Mouse events (click, scroll, drag).
- **Output**: Updated application state with focus changes and scroll position.
- **Business Rules**:
  - Left click on file list: Select entry + focus FileList.
  - Left click on preview: Jump to proportional scroll position + focus Preview.
  - Left click on scrollbar (rightmost 3 columns): Jump to position + focus Preview.
  - Scroll wheel: Scroll focused panel (Preview by 3 lines, FileList by 1 entry).
  - Left drag on scrollbar: Scrub through preview content.
  - Mouse events below shortcut bar (last 4 rows) are ignored.
- **Edge Cases**:
  - Terminal too small (<5 rows, <10 cols) — all mouse events ignored.
  - Click on scrollbar in empty preview — no-op.
  - Scroll at content bounds — clamped to valid range.
- **Error Handling**: No error paths; bounds checking prevents overflow.

### FR-009: Copy Actions
- **Description**: Copy preview content to clipboard or save to file.
- **Input**: `y` (clipboard), `Ctrl+Y` (file).
- **Output**: Updated status message.
- **Business Rules**:
  - `y`: Copy preview text to system clipboard via arboard or xclip/wl-copy fallback.
  - `Ctrl+Y`: Write preview text to `lint-results.txt` in current directory.
  - Empty preview — "Nothing to copy" status message.
- **Edge Cases**:
  - Clipboard unavailable (no xclip/wl-copy) — error message with installation hint.
  - File write permission denied — error message in status bar.
  - Clipboard tool not found — falls back to shell commands.
- **Error Handling**: Clipboard and file write failures return descriptive status messages.

### FR-010: Help Overlay
- **Description**: Toggle a help overlay showing all keyboard shortcuts.
- **Input**: `?` key.
- **Output**: Help overlay rendered in Preview panel.
- **Business Rules**:
  - `?` toggles help visibility state.
  - When active, preview mode switches to help overlay.
  - When inactive, preview mode returns to file content.
- **Edge Cases**:
  - `?` pressed while in search mode — no effect (search takes priority).
- **Error Handling**: No error paths.

### FR-011: Path Input Dialog
- **Description**: Startup dialog for entering project root path.
- **Input**: Character input, Backspace, Enter, Tab.
- **Output**: Updated application state with project root set.
- **Business Rules**:
  - Shown on startup; all keyboard input routed to path editing.
  - Character input appends to path string.
  - Backspace removes last character.
  - Tab: Use current working directory as project root.
  - Enter: Validate path; if valid directory, set as project root and load directory.
  - Esc: Quit the application.
  - Path validated via the file system utility's directory validation function.
- **Edge Cases**:
  - Invalid path entered — "Invalid path" status message, dialog stays open.
  - CWD is inaccessible — Tab uses "." as fallback.
- **Error Handling**: Invalid path stays in dialog; no crash.

### FR-012: Background Scan with Progress
- **Description**: Run multi-adapter scan in a background thread with real-time progress updates.
- **Input**: `s` key (scan action).
- **Output**: Preview panel shows scan output when complete; status bar shows progress during scan.
- **Business Rules**:
  - Scan spawns a new thread (not async) via standard thread spawning.
  - Progress communicated via a bounded channel.
  - Scan update progress messages update status bar with phase, done, total.
  - Scan update complete messages show final output and violation count.
  - While scanning: check/fix/ci/orphan/security/duplicates/dependencies actions are blocked.
  - Only one scan at a time — second `s` press ignored during active scan.
- **Edge Cases**:
  - Runtime creation failure — returns failure result immediately.
  - Channel disconnected — scan thread exits without panic.
  - Scan takes very long — UI remains responsive (event loop continues polling).
- **Error Handling**: Runtime failures return a lint execution result with error message.

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| the TUI command surface run | — | result | Initialize terminal, run event loop, restore terminal on exit. |
| the TUI orchestrator handle event | application state, TUI event | () | Delegate event to action handler. |
| the TUI orchestrator load directory | application state, path | () | Delegate directory load to action handler. |
| the TUI orchestrator load preview | application state | () | Delegate preview load to action handler. |
| the TUI orchestrator start scan | application state | optional channel receiver | Start background scan thread, return progress channel. |
| the TUI orchestrator poll scan | application state, channel receiver | () | Poll scan progress and update state. |
| the action handler handle | application state, TUI event | () | Central event dispatch — maps every TUI event to a state mutation or I/O. |
| the action handler load directory | application state, path | () | Read directory, sort entries (dirs first), reset selection. |
| the action handler load preview | application state | () | Load file preview for selected entry if it's a file. |
| the lint executor check | path, flags | lint execution result | Run AES compliance check. |
| the lint executor scan | path | lint execution result | Run comprehensive multi-adapter scan. |
| the lint executor fix | path, flags | lint execution result | Auto-fix violations. |
| the lint executor ci | path, flags | lint execution result | CI mode with threshold check. |
| the lint executor doctor | — | lint execution result | Environment diagnostics. |
| the file system utility list directory | path | list of file entries | List directory entries (excluding hidden files). |
| the file system utility read file preview | path, max lines | display content | Read file with line numbers, truncate at max_lines. |
| the file system utility copy text to clipboard | string | boolean | Copy text via arboard or xclip/wl-copy fallback. |

## Integration Points

- **Internal**:
  - The TUI aggregate in the shared crate — aggregate trait for TUI orchestration.
  - The action handler protocol in the shared crate — protocol for the central state machine.
  - The lint executor protocol in the shared crate — protocol for all lint action methods.
  - The code analysis aggregate in the shared crate — core AES lint engine.
  - The external lint aggregate in the shared crate — external linter integration.
  - The maintenance commands aggregate in the shared crate — doctor/dependency commands.
  - The config orchestrator aggregate in the shared crate — configuration management.
  - The hook manager protocol in the shared crate — git hook install/uninstall.
  - The analysis pipeline aggregate in the CLI commands crate — comprehensive scan pipeline.
- **External**:
  - `ratatui` — terminal UI rendering framework.
  - `crossterm` — terminal I/O, raw mode, mouse capture, alternate screen.
  - `arboard` — clipboard access (with xclip/wl-copy fallback).

## Non-functional Requirements (Detailed)

- **Performance**: Terminal renders at ~20fps (50ms poll interval). Event processing is O(1) per keypress. Directory listing is O(n) in entry count. File preview limited to 100 lines.
- **Memory**: Preview text capped at 100 lines (~10KB). File listing scales with directory size. Application state is stack-allocated per session.
- **Accuracy**: File extension detection uses OS-provided extension. Scroll position is always clamped to valid bounds. Violation counts are exact.

## Test Scenarios / QA Checklist

- [ ] TUI launches with 3-panel layout and correct proportions.
- [ ] j/k navigation moves selection in file list.
- [ ] h/l/Enter navigates directories and opens files.
- [ ] Tab/BackTab cycles focus between panels.
- [ ] All lint actions (c/s/f/t/o/ctrl+s/ctrl+d/ctrl+p) execute and display results.
- [ ] Status bar updates with violation count after action.
- [ ] scan runs in background with progress indicator; other actions blocked during scan.
- [ ] Path dialog works on startup; Tab uses CWD; Enter validates path.
- [ ] q/Esc quits and restores terminal to normal mode.
- [ ] / search mode filters file list incrementally.
- [ ] ? help overlay toggles correctly.
- [ ] y copies preview to clipboard; Ctrl+Y saves to lint-results.txt.
- [ ] Mouse click selects file in list and focuses FileList panel.
- [ ] Mouse click on preview jumps to proportional scroll position.
- [ ] Mouse scroll wheel scrolls focused panel.
- [ ] Mouse drag on scrollbar scrubs through preview content.
- [ ] Scroll position clamped to bounds (no overflow).
- [ ] Home/End jump to top/bottom of content.
- [ ] PageUp/PageDown scroll preview by 10 lines.
- [ ] Terminal resize handled without crash.
- [ ] doctor/init/install/etc. global actions execute and show results.
- [ ] watch action shows "use CLI" message (not implemented in TUI).

## Assumptions & Constraints

- Terminal must support crossterm (most modern terminals do).
- Mouse support requires terminal with mouse capture capability.
- Clipboard support requires xclip (X11), wl-copy (Wayland), or arboard.
- Background scan uses standard threads (not Tokio) to avoid blocking the render loop.
- File preview limited to first 100 lines; large files are truncated.
- Path dialog is shown on startup; CWD is used as initial directory.
- The TUI requires the full lint-arwaky workspace to be built (all aggregates wired).

## Glossary

| Term | Definition |
|------|-----------|
| Panel Focus | Which panel (Tree/FileList/Preview) receives keyboard input. |
| Preview Mode | What content the Preview panel displays (file content, lint results, help overlay). |
| Application State | Central state holding all TUI state (selection, scroll, focus, etc.). |
| Layer Badge | Colored tag showing the AES layer (taxonomy/contract/capabilities/agent/root/surface/utility) of a file. |

## Reference

- PRD: [PRD.md](../../PRD.md)

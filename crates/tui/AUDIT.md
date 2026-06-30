# TUI Crate — Developer Experience & Functional Audit

**Date**: 2026-06-30
**Scope**: `crates/tui/src/` (15 files) + `crates/tui/tests/` (5 files)
**Baseline**: FRD (`crates/tui/FRD.md`) compliance + DX review + functional bug hunt

See [ARCHITECTURE.md](../../../ARCHITECTURE.md) for AES layer details and [README.md](../../../README.md) for project context.

---

## Executive Summary

| Category        | Status                                   |
| --------------- | ---------------------------------------- |
| Build           | ✅ Clean                                 |
| FRD compliance  | ⚠️ 3 gaps                                |
| Functional bugs | 🔴 4 bugs (1 high, 2 medium, 1 low)      |
| DX issues       | ⚠️ 6 issues                              |
| Code quality    | ⚠️ Minor (orphaned file already removed) |

---

## 🔴 Functional Bugs

### BUG-1 (HIGH): `install_hook` passes empty path to hook installer

**File**: `capabilities_lint_executor.rs:826`
**FRD**: `H` key installs git hook

```rust
fn install_hook(&self) -> LintExecutionResult {
    let exe_path = shared::common::taxonomy_path_vo::FilePath::default(); // ← EMPTY!
    port.install_pre_commit(&exe_path)
}
```

**Impact**: The pre-commit hook is installed with no path to the binary. The hook will be non-functional in production. The `GitHookAdapter` receives an empty/default `FilePath` instead of the actual `lint-arwaky-cli` executable path.

**Fix**: Use `env!("CARGO_BIN_EXE_lint-arwaky-cli")` or resolve the running binary's path at runtime.

---

### BUG-2 (MEDIUM): `navigate_back` uses string length comparison instead of path prefix check

**File**: `capabilities_action_handler.rs:217-221`
**FRD**: `h`/`Left` navigates back (parent directory)

```rust
fn navigate_back(&self, state: &mut AppState) {
    let current = FilePath::new(state.current_dir.clone()).unwrap_or_default();
    if let Some(parent) = self.fs_port.parent_directory(&current) {
        if parent.len() >= state.project_root.len() { // ← string length, not path depth
```

**Impact**: Uses `FilePath::len()` (byte string length) as a proxy for "is parent still inside project root." This breaks when path components have different lengths. Example:

- `project_root = "/ab"`, `parent = "/aa/"` → `parent.len()=4 >= project_root.len()=3` → passes incorrectly
- `/aa/` is NOT inside `/ab/` but the check allows navigation there

**Fix**: Use `parent.starts_with(&state.project_root)` or `Path::starts_with()`.

---

### BUG-3 (MEDIUM): `check()` and `scan()` are identical — `_flags` parameter ignored

**File**: `capabilities_lint_executor.rs:444-450`
**FRD**: `c` = check (AES compliance), `s` = scan (multi-adapter scan)

```rust
fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
    self.run_comprehensive_scan(path)  // ignores _flags
}
fn scan(&self, path: &str) -> LintExecutionResult {
    self.run_comprehensive_scan(path)  // same implementation
}
```

**Impact**: Users pressing `c` (check) and `s` (scan) get identical results. The `_flags` parameter (which carries `dry_run`, `threshold`, `git_diff`) is wasted in `check()`. Per the FRD, `check` should focus on AES compliance only, while `scan` should run multi-adapter scanning.

**Fix**: Either differentiate behavior (check = AES-only, scan = comprehensive) or merge into a single action and update the UI to reflect they're the same.

---

### BUG-4 (LOW): Clipboard fallback always reports success

**File**: `capabilities_action_handler.rs:336-346`
**FRD**: `y` copies preview to clipboard

```rust
.arg("xclip -selection clipboard 2>/dev/null || wl-copy 2>/dev/null || true")
```

**Impact**: The `|| true` means the shell command always exits 0, so `status.success()` returns `true`. User sees "Copied to clipboard!" even when neither `xclip` nor `wl-copy` is installed and nothing was copied.

**Fix**: Remove `|| true` so the shell fails when no clipboard tool is available, or check exit code explicitly.

---

## ⚠️ FRD Compliance Gaps

### GAP-1: Path dialog not shown on startup

**FRD §Overlays**: "Path dialog: shown on startup, type project root or `Tab` to use CWD"

**Implementation** (`surface_tui_command.rs:62-63`):

```rust
state.show_path_dialog = false;
self.tui_aggregate.load_directory(&mut state, &cwd);
```

The TUI skips the path dialog and goes directly to CWD. This was likely a deliberate UX decision (better DX — skip friction), but it's a deviation from the FRD spec. If the path dialog is needed, a key binding (e.g., `Ctrl+O` or `g`) should trigger it.

---

### GAP-2: `fix` returns `success: true` when nothing is fixed

**File**: `capabilities_lint_executor.rs:466-471`
**FRD**: `f` = fix — auto-fix violations

```rust
None => {
    let results = self.code_analysis.run_code_analysis(path);
    let count_before = results.len();
    let output = format!("Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator...", ...);
    LintExecutionResult::success(output, count_before) // ← success: true, but nothing fixed
}
```

**Impact**: When no fix orchestrator is wired, the status bar shows "Done: 0 violations" (or similar) even though violations remain. The user believes the fix succeeded. The `success: true` is misleading — it should be `failure` since the action didn't accomplish its goal.

---

### GAP-3: `SearchBackspace` has no `search_mode` guard

**File**: `capabilities_action_handler.rs:104-106`
**FRD**: `/` to start search, `Backspace` delete

```rust
TuiEvent::SearchBackspace => {
    state.search_query.pop();  // ← no guard, runs even outside search mode
    state.compute_filtered_indices();
}
```

Compare with `SearchInput` (line 98-102) which correctly checks `if state.search_mode`. The `SearchBackspace` handler will silently pop from an empty string when not in search mode (harmless no-op, but inconsistent pattern that could mask bugs if the event routing changes).

---

## ⚠️ Developer Experience Issues

### DX-1: `AppState` is a god object (28 public fields)

**File**: `shared/src/tui/taxonomy_state_vo.rs`

`AppState` has 28 public fields covering navigation, UI mode, display, action flags, filtering, scanning, and control. Every component reads/writes directly to this struct. This makes it:

- Hard to know which component mutated which field
- Impossible to add state validation (no invariants enforced)
- Difficult to test components in isolation

**Recommendation**: Group related fields into sub-structs (`NavigationState`, `ScanState`, `UIState`, etc.) with accessor methods.

---

### DX-2: Mouse coordinate mapping is fragile and duplicated

**File**: `capabilities_action_handler.rs:300-390`

The `handle_mouse_click` and `handle_mouse_drag` methods manually compute panel boundaries using hardcoded layout constraints:

```rust
let shortcuts_start = h - 4;
let file_list_start: u16 = 1;
let file_list_end = shortcuts_start - 1;
```

These must stay in sync with the actual layout in `surface_tui_command.rs`:

```rust
constraints([
    Constraint::Length(1),   // header
    Constraint::Min(10),     // panels
    Constraint::Length(3),   // shortcuts
    Constraint::Length(1),   // status
])
```

If either side changes independently, mouse clicks will map to wrong panels. This is a maintenance hazard.

**Recommendation**: Extract panel boundary calculation into a shared helper used by both the layout and mouse handlers.

---

### DX-3: `ActionHandler` is a 525-line monolith

**File**: `capabilities_action_handler.rs`

Single file handles: navigation, search, lint actions, clipboard, mouse click/drag, scroll, path dialog, resize, and quit. This violates SRP and makes the file hard to navigate.

**Recommendation**: Split into focused modules:

- `navigation_handler.rs` (MoveUp/Down/Left/Right, scroll)
- `lint_action_handler.rs` (ActionCheck/Scan/Fix/etc.)
- `mouse_handler.rs` (MouseClick/Drag/Scroll)
- `clipboard_handler.rs` (CopyToClipboard/CopyToFile)

---

### DX-4: `is_binary_available` uses `which` — not portable

**File**: `capabilities_lint_executor.rs:29-37`

```rust
fn is_binary_available(b: &str) -> bool {
    std::process::Command::new("which")...
}
```

`which` doesn't exist on Windows or some minimal containers. Should use `command -v` (POSIX) or `std::env::var("PATH")` with manual lookup.

---

### DX-5: No error feedback for `load_directory` failures

**File**: `capabilities_action_handler.rs:159-170`

```rust
pub fn load_directory(&self, state: &mut AppState, path: &str) {
    let fp = FilePath::new(path).unwrap_or_default();
    state.entries = self.fs_port.list_directory(&fp);
    // ← no check if entries is empty or path was invalid
```

If the path is invalid or inaccessible, the file list silently becomes empty with no error message. The status bar shows "Dir: /invalid/path" as if it succeeded.

---

### DX-6: `ci` method always returns `success: true` even on FAIL

**File**: `capabilities_lint_executor.rs:475-483`

```rust
let pass = score >= flags.threshold as f64 && !has_critical;
let status = if pass { "PASS" } else { "FAIL" };
let output = format!("...Status: {}", status);
LintExecutionResult::success(output, results.len()) // ← always success, even when FAIL
```

When CI fails (score below threshold or has critical violations), the result is still `success: true`. The status bar shows "Done" instead of "Error". The `pass`/`fail` distinction only appears in the preview text.

---

## 🟢 What Works Well

| Aspect               | Details                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------ |
| **Architecture**     | Clean 7-layer separation: taxonomy → contract → capabilities → infrastructure → agent → surface → root |
| **DI wiring**        | `root_tui_container.rs` cleanly composes all dependencies with builder pattern                         |
| **Event loop**       | 50ms poll interval is responsive; crossterm event mapping is complete                                  |
| **Panel focus**      | Tab/BackTab cycling works correctly across all 3 panels                                                |
| **Scroll clamping**  | `saturating_add/sub` + `min()` prevents overflow in all scroll paths                                   |
| **Background scan**  | `std::sync::mpsc::sync_channel` with thread spawn is correct and non-blocking                          |
| **Layer badges**     | `AesLayer::from_filename()` correctly detects all 7 AES layers                                         |
| **Search filtering** | `compute_filtered_indices()` pre-computes filtered list for efficient rendering                        |
| **Test coverage**    | 48 action handler tests + 32 lint executor tests + 7 orchestrator tests                                |

---

## Priority Fix Roadmap

| #   | Issue                                         | Severity  | Effort | Files                            |
| --- | --------------------------------------------- | --------- | ------ | -------------------------------- |
| 1   | BUG-1: `install_hook` empty path              | 🔴 HIGH   | 5 min  | `capabilities_lint_executor.rs`  |
| 2   | BUG-2: `navigate_back` string length          | 🟡 MEDIUM | 2 min  | `capabilities_action_handler.rs` |
| 3   | GAP-2: `fix` success: true when nothing fixed | 🟡 MEDIUM | 2 min  | `capabilities_lint_executor.rs`  |
| 4   | DX-6: `ci` always returns success             | 🟡 MEDIUM | 2 min  | `capabilities_lint_executor.rs`  |
| 5   | BUG-3: `check`/`scan` identical               | 🟡 MEDIUM | 15 min | `capabilities_lint_executor.rs`  |
| 6   | BUG-4: Clipboard `                            |           | true`  | 🟢 LOW                           | 1 min | `capabilities_action_handler.rs` |
| 7   | GAP-3: `SearchBackspace` no guard             | 🟢 LOW    | 1 min  | `capabilities_action_handler.rs` |
| 8   | GAP-1: Path dialog on startup                 | 🟢 LOW    | 5 min  | `surface_tui_command.rs`         |
| 9   | DX-4: `which` not portable                    | 🟢 LOW    | 3 min  | `capabilities_lint_executor.rs`  |
| 10  | DX-5: No error for invalid dir                | 🟢 LOW    | 5 min  | `capabilities_action_handler.rs` |
| 11  | DX-2: Mouse coordinate duplication            | 🟠 DX     | 30 min | `action_handler` + `tui_command` |
| 12  | DX-3: ActionHandler 525-line monolith         | 🟠 DX     | 2 hr   | refactor                         |
| 13  | DX-1: AppState god object                     | 🟠 DX     | 4 hr   | refactor                         |

**Quick wins (1–2 hours)**: Items 1-10
**Structural improvements (half day+)**: Items 11-13

---

## FRD Compliance Matrix

| FRD Section            | Requirement              | Status | Notes                 |
| ---------------------- | ------------------------ | ------ | --------------------- |
| Core Layout            | 3-panel 20/35/45         | ✅     |                       |
| Header bar             | Path + quit hint         | ✅     |                       |
| Shortcut bar           | 3 rows                   | ✅     |                       |
| Status bar             | Status + violations      | ✅     |                       |
| Navigation             | j/k/h/l/Enter/Home/End   | ✅     |                       |
| Panel focus            | Tab/BackTab              | ✅     |                       |
| Quit                   | q/Esc                    | ✅     |                       |
| Scroll                 | j/k context-aware        | ✅     |                       |
| Scroll                 | PageUp/PageDown ±10      | ✅     |                       |
| Scroll                 | Home/End in preview      | ✅     |                       |
| Scroll                 | Mouse wheel              | ✅     |                       |
| Scroll                 | Mouse click proportional | ✅     |                       |
| Scroll                 | Mouse drag scrollbar     | ✅     |                       |
| Scroll                 | Clamped to bounds        | ✅     |                       |
| Lint: check            | AES compliance           | ⚠️     | Identical to scan     |
| Lint: scan             | Background + progress    | ✅     |                       |
| Lint: fix              | Auto-fix                 | ⚠️     | Misleading success    |
| Lint: ci               | Threshold                | ⚠️     | Always success        |
| Lint: orphan           | Dead code                | ✅     |                       |
| Lint: security         | Vuln scan                | ✅     |                       |
| Lint: duplicates       | Dup detection            | ✅     |                       |
| Lint: deps             | Dep analysis             | ✅     |                       |
| Copy: y                | Clipboard                | ⚠️     | Fallback masks errors |
| Copy: Ctrl+Y           | Save to file             | ✅     |                       |
| File display           | Layer badges             | ✅     |                       |
| File display           | Dir `/` suffix           | ✅     |                       |
| File display           | Dir-first sort           | ✅     |                       |
| Overlay: search        | Incremental filter       | ✅     |                       |
| Overlay: help          | Toggle `?`               | ✅     |                       |
| Overlay: path          | Startup dialog           | ❌     | Skipped, goes to CWD  |
| Mouse: click file      | Select + focus           | ✅     |                       |
| Mouse: click preview   | Proportional scroll      | ✅     |                       |
| Mouse: click scrollbar | Jump position            | ✅     |                       |
| Mouse: scroll wheel    | Focus-aware              | ✅     |                       |
| Mouse: drag scrollbar  | Scrub content            | ✅     |                       |
| Infrastructure         | Types in shared/tui      | ✅     |                       |
| Background scan        | mpsc channel             | ✅     |                       |

**Score**: 33/36 FRD requirements met (92%)

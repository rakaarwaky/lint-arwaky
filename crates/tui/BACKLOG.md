# Feature Backlog: TUI

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: All TUI UX issues (#354–#368) implemented in this change.
- In Progress: None
- Blocked: None
- Next Action: Verify TUI runs correctly with `cargo run --bin lint-arwaky-tui`

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| TUI-01 | FR-TUI-001 | File browser panel with AES layer badges | P0 | Done | Implemented in `surface_file_list_view.rs`; hidden files excluded; dirs first | @raka | None | 2026-09-29 |
| TUI-02 | FR-TUI-002 | Preview panel with 4 modes (FileContent, LintResults, ActionOutput, HelpOverlay) | P0 | Done | Implemented in `surface_preview_view.rs`; file preview capped at 100 lines | @raka | None | 2026-09-29 |
| TUI-03 | FR-TUI-003 | Keyboard navigation between panels and within file list | P0 | Done | Implemented in `surface_tui_command.rs::from_key_event`; Tab/BackTab cycles focus | @raka | None | 2026-09-29 |
| TUI-04 | FR-TUI-004 | Incremental search with zero-match fallback | P0 | Done | `/` toggles search; zero matches shows "No matches" hint instead of full list (#367) | @raka | None | 2026-09-29 |
| TUI-05 | FR-TUI-005 | Lint actions on background threads (check, scan, fix, ci, orphan, security, dependencies) | P0 | Done | Actions spawn via `std::thread`; UI stays responsive; Esc cancels in-flight scan | @raka | None | 2026-09-29 |
| TUI-06 | FR-TUI-006 | Modal path dialog for project root (startup + r key) | P0 | Done | Dialog shown on startup; `Esc` = PathCancel (dismisses without quitting) (#355); `r` re-opens (#I4) | @raka | None | 2026-09-29 |
| TUI-07 | FR-TUI-007 | Confirm gate for destructive actions (FixLive, InstallHook, UninstallHook, Install, Init) | P0 | Done | All five gated; `y`/`Enter` confirms, `n`/`Esc` cancels; action keys blocked while confirm is pending (#354, #364) | @raka | None | 2026-09-29 |
| TUI-08 | FR-TUI-008 | Modal help overlay (`?`) with previous PreviewMode restore | P0 | Done | `?` shows help; saves `last_preview_mode` and restores on close; action keys blocked (#359) | @raka | None | 2026-09-29 |
| TUI-09 | FR-TUI-009 | NO_COLOR support — ASCII fallbacks for all Unicode glyphs | P1 | Done | `theme::no_color()` checks env; progress bar, separator, warning have ASCII variants (#360, #365) | @raka | None | 2026-09-29 |
| TUI-10 | FR-TUI-010 | Terminal resize guard (min 40×15) with layout constants in theme | P1 | Done | `MIN_TERMINAL_WIDTH`/`MIN_TERMINAL_HEIGHT` defined in `utility_tui_theme`; render guard uses them (#362) | @raka | None | 2026-09-29 |
| TUI-11 | FR-TUI-005 | Shift+F (`F`) triggers live fix; Ctrl+S remapped to dependencies to avoid XOFF | P1 | Done | `Char('F')` / `SHIFT+f` → `ActionFixLive`; `Ctrl+S` → `ActionDependencies`; `Ctrl+P` → `ActionSecurity` (#363) | @raka | None | 2026-09-29 |
| TUI-12 | FR-TUI-005 | Clear stale preview text when directory changes | P1 | Done | `load_directory` calls `preview_text.clear()` (#368) | @raka | None | 2026-09-29 |
| TUI-13 | — | Help copy aligned with actual key bindings (^S deps, ^P security, ^Y save to file) | P1 | Done | `surface_shortcut_component.rs` and `surface_preview_view.rs::help_text()` updated (#358, #361) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Last verified |
|----------|------|---------------|
| Startup with path dialog, CWD pre-filled, Tab fills path | Manual | 2026-09-29 |
| `Esc` in path dialog dismisses without quitting | Manual | 2026-09-29 |
| `/` + zero matches shows "No matches" hint | Manual | 2026-09-29 |
| `F` requires confirmation before applying fixes | Manual | 2026-09-29 |
| `H` and `U` require confirmation | Manual | 2026-09-29 |
| `?` shows help; `q`/`Esc` closes and restores preview mode | Manual | 2026-09-29 |
| `NO_COLOR=1` → ASCII progress bar, separators, warning | Manual | 2026-09-29 |
| Terminal below 40×15 shows "too small" message | Manual | 2026-09-29 |
| `Esc` while scanning cancels without quitting TUI | Manual | 2026-09-29 |
| `w` shows "Watch not supported" message | Manual | 2026-09-29 |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p tui --lib --tests` → 0 failures |
| Scenario evidence | Done | 10 scenarios mapped; all Manual |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Initial backlog created; TUI UX issues #354–#368 resolved in this change | @raka |

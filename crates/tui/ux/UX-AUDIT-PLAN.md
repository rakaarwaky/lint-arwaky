# Plan: Interactive find-and-fix workflow (`lint-arwaky-tui` + CLI report surfaces) — UI/UX Designer

Audited at commit `e63ad42` (v3.16.1), 2026-11-09. Filed issues: #1317–#1331.

## Summary

This audit covers the human-facing surfaces of Lint Arwaky: the `lint-arwaky-tui` binary (Ratatui three-panel browser plus overlays), the text output of `scan` / `ci` / `report`, and the contract documents that specify them. The target personas are the **CI engineer** who reads one report and branches on an exit code, the **maintainer** who fixes violations under time pressure, and the **platform engineer** who needs the layer model explained in place. The affected journey is: launch the TUI → pick a project root → browse to a file → run a lint action → read the result → decide whether to fix → verify the fix landed. Five defect classes block that journey today: the path dialog hides the workspace truth (Scope 1), the shortcut bar silently truncates at every supported terminal width (Scope 2), `DESIGN.md` ships 13 design-system sections that are empty tables (Scope 3), drift between the documented and the implemented interaction model (Scope 4), and states that lie — a stale violation badge, a cancel path that cannot be reached, a failure path that never prints its reason (Scope 5). The specification is **not ready for implementation**: the design-system document is a shell, the state matrix is undefined, and three defects put the user's only visible progress signal or their remediation instructions out of reach. Twelve issues follow, one CRITICAL, eight WARNING, three INFO.

---

### Scope 1: Map the user journey
<!-- Check user goal, persona, entry point, task flow, interaction path, navigation logic, drop off risk, and alignment with business process -->

#### Issue UX-1-01-20261109T052544Z (`#1317`)

- **Title**: [UX][CRITICAL] The path dialog cannot be re-opened — the 'r' recovery key is unbound and the project root is fixed for the session
- **Label**: `interaction`, `usability`, `severity-critical`
- **Location Path**:
  `crates/tui/src/surface_tui_command.rs:146-149` (`path_dialog.visible` short-circuit) · `crates/tui/src/surface_tui_command.rs:70-78` (`run`, dialog shown on startup) · `crates/tui/src/surface_event_action.rs:500-506` (`PathCancel`) · `crates/tui/src/surface_event_action.rs:427-430` (`ChangeProjectRoot`) · `crates/tui/src/surface_bindings_store.rs:88-553` (`SHORTCUTS`, no `r` entry) · `crates/tui/src/taxonomy_tui_vo.rs:340-345` (`AppState::new` starts with zero entries)
- **Description**:
  The TUI opens on the "Enter Project Path" modal and does not load a directory until the user confirms. Both escape hatches are broken:

  1. **The recovery instructions name a key that cannot exist.** `PathCancel` (`surface_event_action.rs:503`) sets `status_message` to `"Path dialog cancelled — press 'r' to re-open"`. There is no `Trigger::Plain('r')` anywhere in `SHORTCUTS`, and `ChangeProjectRoot` — the only handler that re-opens the dialog (`surface_event_action.rs:427`) — has **zero producers** in `crates/tui/src/`: grepping `ChangeProjectRoot` returns only the enum declaration, the logging arm, and the handler itself. `from_key_event` (`surface_tui_command.rs:262-313`) resolves every unbound key through `surface_bindings_store::action_for`, which returns `None` for `r`. So the instructions the user is given in the status bar lead to a keypress that does nothing.
  2. **The project root becomes permanent for the session.** `PathCancel` does call `load_directory` on `current_dir`, so the Files panel is populated with the process's working directory rather than left bare — but `project_root` is seeded to that same working directory by `AppState::new` (`taxonomy_tui_vo.rs:336-345`) and is **never reassigned** afterwards except by `PathConfirm` / `PathUseCurrent`, both of which are unreachable once the dialog closes. A user who launches `lint-arwaky-tui` from `$HOME` by mistake now has `$HOME` as their project root: the Tree panel's root is `$HOME`, `h`/`←` clamps at `$HOME`, and the only fix is to quit and relaunch. There is no visible indicator that the root is wrong and no in-app way to correct it.
  3. **There is no non-interactive entry point that bypasses the dialog.** `lint-arwaky-tui` accepts no arguments (`crates/root_tui_main_entry.rs`), so an operator who already names the directory on the command line — or a script that launches the TUI over a specific workspace — still has to type it again into a modal, with no pre-fill beyond the `[cwd]` placeholder.

  This is a broken user flow on the first screen of the product: after one `Esc` the audit target is fixed for the rest of the session, and the recovery text the user is given names a key that does nothing. It is not covered by the closed #355, which fixed `Esc` so it cancels the dialog instead of quitting the TUI — the dead `'r'` promise and the now-unreachable `ChangeProjectRoot` handler are the surviving halves of that change.

- **Acceptance Criteria**:
  - `Esc` on the path dialog leaves a screen where the user can choose a project root again; a test drives `PathCancel` followed by whatever binding re-opens the dialog and asserts `state.path_dialog.visible == true`.
  - Exactly one binding named in the dismissal status text exists in `SHORTCUTS` and dispatches to `TuiEvent::ChangeProjectRoot`; a test asserts `action_for(KeyEvent::new(KeyCode::Char('r'), NONE)) == Some(ShortcutAction::ChangeProjectRoot)`.
  - The help overlay lists the re-open binding under `Navigation`, because `help_text()` is generated from `SHORTCUTS` and cannot drift.
  - `lint-arwaky-tui <PATH>` skips the dialog and loads `<PATH>` directly; `lint-arwaky-tui` with no argument keeps today's behaviour.
  - A cancelled first run leaves a visible, centered empty state in the Files panel ("No project selected — press `r` to choose a folder"), not bare chrome.
- **Recommendation**:
  Add a `ChangeProjectRoot` `Shortcut` entry (`Trigger::Plain('r')`, `key_label: "r"`, `label: "root"`, `section: "Navigation"`, `normal_row: Some(2)`, `result_row: Some(2)`) so the table stays the single source of truth for dispatch, the shortcut bar, and the help overlay. Accept an optional positional `PATH` in `root_tui_main_entry.rs` and pre-seed `AppState` with it, skipping the dialog. Replace the ghost status line with an explicit empty state in `FileListView` when `navigation.entries.is_empty() && !path_dialog.visible`.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_event_action.rs
+++ b/crates/tui/src/surface_event_action.rs
@@ -500,7 +500,9 @@
             TuiEvent::PathCancel => {
                 state.path_dialog.visible = false;
                 state.path_dialog.input.clear();
-                state.set_status("Path dialog cancelled — press 'r' to re-open");
+                // `r` is a real `SHORTCUTS` entry (see bindings table below), so
+                // this status line points at a binding that actually dispatches.
+                state.set_status("Path dialog cancelled — press 'r' to change root");
                 self.load_directory(state, &state.navigation.current_dir.clone());
             }
```

```diff
--- a/crates/tui/src/taxonomy_tui_vo.rs
+++ b/crates/tui/src/taxonomy_tui_vo.rs
@@
         Self {
             navigation: NavigationState {
                 project_root,
                 current_dir,
-                entries: Vec::new(),
+                // Seed the listing immediately: a cancelled path dialog must not
+                // leave the user with a `project_root` they can never correct.
+                entries: load_entries(&current_dir),
                 selected_index: 0,
```

```diff
--- a/crates/tui/src/surface_bindings_store.rs
+++ b/crates/tui/src/surface_bindings_store.rs
@@
     Shortcut {
         triggers: &[Trigger::Plain('/')],
         action: ShortcutAction::ToggleSearch,
         key_label: "/",
         label: "search",
         result_label: "search",
         section: "General",
         normal_row: Some(2),
         result_row: Some(2),
     },
+    // #355 removed the dialog's quit-on-Esc behaviour but left the status line
+    // instructing the user to press `r` — a key with no entry in this table and
+    // therefore no dispatch, no shortcut bar cell, and no help line.
+    Shortcut {
+        triggers: &[Trigger::Plain('r')],
+        action: ShortcutAction::ChangeProjectRoot,
+        key_label: "r",
+        label: "root",
+        result_label: "root",
+        section: "Navigation",
+        normal_row: Some(1),
+        result_row: Some(1),
+    },
 ];
```

- **Open Questions**:
  - Should the first-run dialog default to the shell's `cwd` as committed text (Enter accepts it) instead of an empty field with a placeholder?
  - Does `lint-arwaky-tui <PATH>` belong in v1, or is a `--path` flag preferable for symmetry with `lint-arwaky-cli scan <PATH>`?

#### Issue UX-1-02-20261109T052544Z (`#1318`)

- **Title**: [UX][WARNING] The interactive TUI is still absent from README and from `lint-arwaky-cli --help`
- **Label**: `content`, `usability`, `severity-warning`
- **Location Path**:
  `README.md:12-31` (Quick Start table) · `README.md:34-42` (Reference table) · `crates/root_cli_main_entry.rs:28-31` (clap `Cli`) · `crates/root_cli_main_entry.rs:42-` (`Command` enum) · `scripts/install.sh:285` (only `lat` alias mention) · `crates/tui/DESIGN.md`
- **Description**:
  Closed issue #356 raised this and the fix did not land in the user-facing docs: `grep -i tui README.md` returns **zero** matches today, and the Quick Start ends at `fix --dry-run`. The `README.md` states seven steps across install, scan, report, ci, and fix — none of them the interaction mode that the project ships as a first-class binary (`Cargo.toml:176`, `lint-arwaky-tui`) and builds in every release (`scripts/install.sh:174-208`). `lint-arwaky-cli --help` cannot surface it because the TUI is a separate binary with no subcommand. The only discovery path remains a shell alias written by `install.sh:285`, which a user reading the README on GitHub never sees.

  For a maintainer persona whose entry point *is* the README, the product appears to be a batch linter only. The `.github/ISSUE_TEMPLATE/` and PR template do not mention the TUI either.

- **Acceptance Criteria**:
  - `README.md` Quick Start gains an explicit TUI step (`bash scripts/install.sh` installs all three binaries; `lint-arwaky-tui` starts the browser) between Verify and the batch commands.
  - `README.md` Reference table links `crates/tui/DESIGN.md` alongside `ARCHITECTURE.md`, so the interaction contract is reachable from the front page.
  - A test or CI grep asserts `tui` appears in `README.md`, so a future rewrite cannot drop it silently.
- **Recommendation**:
  Add one Quick Start row and one Reference row; mirror the same two lines in `DEPLOY.md`'s binary list (already present at line 88, but not linked from the README). Prefer naming the binary `lint-arwaky-tui` rather than the `lat` alias, which is shell-specific.
- **Git Diff**:

```diff
--- a/README.md
+++ b/README.md
@@
 | 3    | Verify the installation                      | `lint-arwaky-cli version`                                                                                  |
+| 4    | Browse the workspace interactively           | `lint-arwaky-tui` (three-panel TUI: tree, files, lint output)                                              |
-| 4    | Run all linters                              | `lint-arwaky-cli scan .`                                                                                   |
+| 5    | Run all linters                              | `lint-arwaky-cli scan .`                                                                                   |
```

- **Open Questions**:
  - Should the README push the TUI as the primary entry point for maintainers and the CLI as the CI path, or present them as equals?

#### Issue UX-1-03-20261109T052544Z (`#1319`)

- **Title**: [UX][WARNING] A failing `ci` run prints only `Score` and hides every failure reason
- **Label**: `content`, `usability`, `severity-warning`
- **Location Path**:
  `crates/cli-commands/src/surface_ci_command.rs:43-70` (`handle_ci`) · `crates/dispatcher/src/surface_ci_action.rs:186-221` (`reasons` built, never rendered)
- **Description**:
  `handle_ci` prints `Score`, `Threshold`, and the four severity counters, then branches: on pass it prints `Result: PASS (exit code 0)`; on fail it does **not** print `Result: FAIL`, does not print the exit code, and does not print a single entry from `report.reasons`. The reasons are built for exactly this purpose (`surface_ci_action.rs:186-197`: `"CRITICAL violation(s) detected — auto-fail triggered"` and `"Score below threshold ({:.1} < {})"`) and are then handed only to `tracing::info!`, which the default filter (`"warn"`, `taxonomy_logging_vo.rs:22`) suppresses. A CI engineer watching a red pipeline sees:

  ```
  Lint Arwaky v3.16.1 — CI Architecture Compliance
  Score: 61.4 / 100
  Threshold: 80

  CRITICAL: 0 | HIGH: 12 | MEDIUM: 4 | LOW: 0
  ```

  and then nothing. The two facts they need — *this run failed* and *why* — are absent, while the pass path states its outcome explicitly. This is the persona in PRD `Target Users` row "CI engineer" whose stated need is "branch on exit code instead of parsing text output"; the human reading the log is left to infer failure from the absence of a line.

- **Acceptance Criteria**:
  - The failing branch prints `Result: FAIL (exit code 2)` (matching `ExitCode::POLICY_FAIL`) and one indented reason line per entry in `report.reasons`.
  - Pass and fail branches are structurally symmetric: both print a `Result:` line and both name the exit code.
  - A test asserts a below-threshold report's stdout contains `Result: FAIL` and the threshold reason string.
- **Recommendation**:
  Print `Result: PASS/FAIL (exit code N)` unconditionally, then render `reasons` as an indented bullet block on failure. Keep the `tracing::info!` calls — they serve `-v` — but stop treating them as the human-facing channel.
- **Git Diff**:

```diff
--- a/crates/cli-commands/src/surface_ci_command.rs
+++ b/crates/cli-commands/src/surface_ci_command.rs
@@
             if report.pass {
                 println!("Result: PASS (exit code 0)");
                 ExitCode::OK
             } else {
-                for r in &report.reasons {
-                    info!(result = %r, "scan result");
-                }
+                println!("Result: FAIL (exit code 2)");
+                println!();
+                for reason in &report.reasons {
+                    println!("  - {reason}");
+                    info!(result = %reason, "scan result");
+                }
                 info!("scan result: FAIL");
                 ExitCode::POLICY_FAIL
             }
```

- **Open Questions**:
  - Should the failure block also list the offending rule codes (the data exists in `report`), or is pointing at `lint-arwaky-cli scan` the intended follow-up?

---

### Scope 2: Produce the wireframe prototype
<!-- Check low fidelity wireframe, high fidelity prototype, visual hierarchy, layout structure, responsive behavior, interaction affordance, and user feedback loop -->

#### Issue UX-2-01-20261109T052544Z (`#1320`)

- **Title**: [UX][WARNING] Shortcut bar truncates silently in both modes — up to 158 columns of rules in a 100-column layout
- **Label**: `responsive`, `visual`, `severity-warning`
- **Location Path**:
  `crates/tui/src/surface_shortcut_component.rs:70-85` (`format_shortcuts`) · `crates/tui/src/surface_shortcut_component.rs:37-41` (fixed 3-row `Paragraph`) · `crates/tui/src/surface_bindings_store.rs:88-553` (`SHORTCUTS` row assignment) · `crates/shared/src/tui/utility_tui_theme.rs:47` (`NARROW_BREAKPOINT_WIDTH = 100`)
- **Description**:
  `ShortcutComponent::render` builds three rows of key/label spans and hands them to `Paragraph::new(text)` with no `.wrap()` and no width awareness. `format_shortcuts` concatenates every shortcut assigned to a row, so the rendered width is fixed by the *table*, not the terminal:

  | Mode | Row 0 | Row 1 | Row 2 |
  |---|---|---|---|
  | normal | 15 items / **158 cols** | 7 items / 67 cols | 11 items / **140 cols** |
  | results | 11 items / 131 cols | 11 items / 115 cols | 10 items / 116 cols |

  The TUI declares `MIN_TERMINAL_WIDTH = 40` and flips to the three-column layout at `width >= 100` (`utility_tui_theme.rs:45-47`). At the 100-column breakpoint — the *widest* narrow-mode terminal, the one the layout is tuned for — row 0 loses the last 58 characters, which is `i:init`, and row 2 loses 40 characters, which is `?:help` and `/:search`. Ratatui clips from the right, so the two affordances a first-time user most needs (help and search) are exactly the ones that vanish, with no ellipsis, no scroll indicator, and no hint that more exists. At 40 columns row 1 also truncates.

  The component has no priority concept: the "Setup" cluster (`I/m/C/H/U/a/v`) consumes a whole row ahead of `?:help`, and the `w:watch (CLI only)` entry — an action that cannot run in the TUI at all (`surface_event_action.rs:454-459`) — holds a high-priority cell in row 0 in normal mode while consuming another in results mode.

  Concurrently the shortcut bar consumes a fixed 3 rows of every terminal (`surface_tui_layout.rs:66`, `Constraint::Length(3)`), so on a `MIN_TERMINAL_HEIGHT = 15` terminal it is 20% of the screen, and the third row is empty in results mode (10 items fit in 116 cols).

- **Acceptance Criteria**:
  - At 40, 80, 100, and 160 columns, every rendered shortcut row is either fully visible or ends with an explicit `…` affordance and a `?:help` escape hatch.
  - `?:help` and `/:search` survive truncation at every width ≥ `MIN_TERMINAL_WIDTH`; a rendering test asserts both appear in the buffer at 40 columns.
  - The `w` binding either moves to the results-only row or is removed from the bar (the help overlay keeps it).
  - Shortcut rows carry an explicit priority so truncation drops the lowest-priority cell first, not whichever happens to be last in a `const` array.
- **Recommendation**:
  Give `Shortcut` a `priority: u8` field (General/Navigation `1`, Actions `2`, Setup `3`) and have `rows_for_context` take the available width, emitting cells until the budget is spent and then a `…` cell whose label renders as `?:help`. Drop `w:watch (CLI only)` from `normal_row`/`result_row` — a shortcut that explains it does nothing is worse than its absence — and keep it in the help overlay. Reduce the shortcut band to 2 rows in results mode.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_shortcut_component.rs
+++ b/crates/tui/src/surface_shortcut_component.rs
@@
-        let text = vec![
-            Line::from(format_shortcuts(&row1)),
-            Line::from(format_shortcuts(&row2)),
-            Line::from(format_shortcuts(&row3)),
-        ];
+        // A row wider than the panel is clipped from the right by Paragraph,
+        // which silently ate `?:help` and `/:search` at the 100-column
+        // breakpoint. Budget the cells and end with an explicit affordance.
+        let budget = area.width as usize;
+        let text = vec![
+            Line::from(format_shortcuts_bounded(&row1, budget)),
+            Line::from(format_shortcuts_bounded(&row2, budget)),
+            Line::from(format_shortcuts_bounded(&row3, budget)),
+        ];
```

- **Open Questions**:
  - Should the bar collapse to a single `?:help` line below 60 columns rather than truncating three rows?
  - Is hiding `w:watch (CLI only)` from the bar acceptable, given the PRD states watch is out of scope for the TUI?

#### Issue UX-2-02-20261109T052544Z (`#1321`)

- **Title**: [UX][WARNING] The scan progress bar is the only progress signal and it is drawn from a denominator that starts at zero
- **Label**: `interaction`, `visual`, `severity-warning`
- **Location Path**:
  `crates/tui/src/surface_status_component.rs:47-70` (progress computation) · `crates/tui/src/surface_event_action.rs:56-60` (`start_scan` sets total `0`) · `crates/tui/src/taxonomy_tui_vo.rs:540-547` (`set_scanning` clears totals)
- **Description**:
  `start_scan` calls `set_scanning(true, "Starting scan...", 0)`, which resets `files_done` and `files_total` to `0`. `StatusComponent` then renders `(0, bar_width)` filled blocks — an empty 20-column bar — and `format!(" {}", bar)` with no numbers, because the `files_total > 0` branch is skipped. Until the first `Progress` message arrives, which on a 10k-file workspace can be seconds after the keypress, the scan looks identical to an idle bar; there is no spinner, no elapsed time, and no "starting" indicator beyond the phase string.

  Once counts arrive, the bar is `files_done / files_total` — but nothing on screen says *what* is being counted, and the final phase has no completion affordance: `finish_scan` zeroes the totals and clears the phase, so the moment the scan ends the bar disappears without a `100%` frame or a `Done` flash. A user watching a long scan sees the progress vanish and must infer completion from the appearance of results in the preview panel.

  The `phase` string is whatever the dispatcher emits; `StatusComponent` prefixes it with a space and styles it `ACCENT + BOLD` but does not truncate it, so a long phase name pushes the violation counter off the right edge of a narrow terminal — the same clipping failure as UX-2-01, on the one line that reports progress.

- **Acceptance Criteria**:
  - Between keypress and the first `Progress` message the status line shows an indeterminate indicator (e.g. `Starting scan…`) distinct from the empty determinate bar.
  - The bar always states its unit and its numbers once known: `████░░░ 4021/10240 files`.
  - On completion the status line shows a terminal state (`Done — 128 violations`) for at least one rendered frame before results replace it.
  - Phase text longer than the available width is truncated with `…` and never displaces the violation counter.
- **Recommendation**:
  Add an `Indeterminate` variant to the status line's scan branch, render counts with their unit, and on `finish_scan` set a `"Scan complete"` phase for one tick. Truncate the phase with `Paragraph::wrap` disabled and a computed budget, or place the violation counter first (left) so it can never be pushed out.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_status_component.rs
+++ b/crates/tui/src/surface_status_component.rs
@@
-            let progress_detail = if state.scan.files_total > 0 {
-                format!(
-                    " {} {}/{}",
-                    bar, state.scan.files_done, state.scan.files_total,
-                )
-            } else {
-                format!(" {}", bar)
-            };
+            // A zero denominator means the dispatcher has not published its
+            // file count yet — that is an indeterminate wait, not 0% progress.
+            let progress_detail = if state.scan.files_total > 0 {
+                format!(
+                    " {} {}/{} files",
+                    bar, state.scan.files_done, state.scan.files_total,
+                )
+            } else {
+                format!(" {} waiting for file count…", bar)
+            };
```

- **Open Questions**:
  - Should the scan emit a total file count on start (walk once, then lint) so the bar is determinate from the first frame?
  - Is a per-phase progress bar (walk, then lint, then rules) more honest than one bar over mixed units?

#### Issue UX-2-03-20261109T052544Z (`#1322`)

- **Title**: [UX][INFO] The Files panel has no per-file violation affordance although every entry carries a `violation_count`
- **Label**: `visual`, `severity-info`
- **Location Path**:
  `crates/tui/src/surface_file_list_view.rs:68-92` (row construction) · `crates/tui/src/taxonomy_tui_vo.rs:150-186` (`FileEntry { violation_count }`, only ever `0`)
- **Description**:
  `FileEntry` declares `violation_count: usize` and `FileEntry::from_path` hard-codes it to `0` (`taxonomy_tui_vo.rs:181`); nothing in `crates/tui/src/` ever writes it. `FileListView` therefore has a per-row slot it does not render, while the *only* violation number in the interface is the global counter in the status bar.

  The consequence is a journey gap, not just a missing badge: after a `scan`, the preview panel lists violations grouped by file (`surface_lint_action.rs:700-717`), but the Files panel — the panel the user navigates with — carries no marker telling them which of the 60 visible files have findings. Going from "437 violations" to "which files" requires reading and scrolling the preview text and mentally matching paths back to the list. For the maintainer persona fixing a release gate, that is the highest-frequency loop in the product.

- **Acceptance Criteria**:
  - After a scan completes, each file row with at least one violation shows a count badge (e.g. a right-aligned `12`) styled with the `VIOLATIONS` token; clean files show none.
  - The badge is derived from the same `ViolationItem` set the preview renders, not a second parse, and it is cleared when the directory changes.
  - A rendering test asserts a scanned file row contains its count and a clean file row does not.
- **Recommendation**:
  Populate `FileEntry.violation_count` when a scan result is applied (group `report` results by `file.value` and map onto `navigation.entries` by `full_path`), and render a right-aligned badge using the existing `VIOLATIONS` token. This is INFO because it adds a signal where today the preview still works — but it is the highest-leverage affordance in the list.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_file_list_view.rs
+++ b/crates/tui/src/surface_file_list_view.rs
@@
-                let line = Line::from(vec![
-                    Span::styled(format!("{} ", badge), Style::default().fg(badge_color)),
-                    Span::styled(name, name_style),
-                ]);
+                let mut spans = vec![
+                    Span::styled(format!("{} ", badge), Style::default().fg(badge_color)),
+                    Span::styled(name, name_style),
+                ];
+                // `FileEntry::violation_count` existed but was never written or
+                // drawn, so the panel that owns navigation could not tell the
+                // user which of its rows the scan had flagged.
+                if entry.violation_count > 0 {
+                    spans.push(Span::styled(
+                        format!(" {}", entry.violation_count),
+                        Style::default().fg(theme::color(theme::VIOLATIONS)),
+                    ));
+                }
+                let line = Line::from(spans);
```

- **Open Questions**:
  - Right-aligned in the panel or inline after the filename? Right alignment needs the panel width inside the row builder, which `List` does not provide.

---

### Scope 3: Define the design system
<!-- Check design tokens, color palette, typography scale, spacing scale, component library, iconography, visual assets, and consistency across screens -->

#### Issue UX-3-01-20261109T052544Z (`#1323`)

- **Title**: [UX][WARNING] `crates/tui/DESIGN.md` documents 13 design-system sections as empty tables
- **Label**: `content`, `visual`, `severity-warning`
- **Location Path**:
  `crates/tui/DESIGN.md:18-90` (DS-TUIC-001 … DS-TUIC-013) · `crates/shared/src/tui/utility_tui_theme.rs:21-55` (the real token table) · `crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs:186-198` (DESIGN template contract)
- **Description**:
  Every one of the thirteen `DS-TUIC-*` sections in `crates/tui/DESIGN.md` is the literal stub `| Variant | Use case | Token | Value|` followed by `|         |          |       |       |`. There is no row for `ACCENT`, `KEY`, `LABEL`, `SEPARATOR`, `VIOLATIONS`, `CLEAN`, `PENDING`, `DIRECTORY`, `CAPABILITIES_BADGE`, `BACKGROUND`, `HIGHLIGHT`, `PATH_INPUT`, `FOCUS_CONFIRM`, `SCROLLBAR`, or `HEADER` — the fifteen tokens that actually exist in `utility_tui_theme.rs:21-37`. Likewise empty: `DS-TUIC-004 Layout & Spacing` (the real constants `PANEL_TREE_WIDTH_PCT = 35`, `PANEL_FILE_LIST_WIDTH_PCT = 20`, `PANEL_PREVIEW_WIDTH_PCT = 45`, `MIN_TERMINAL_HEIGHT = 15`, `MIN_TERMINAL_WIDTH = 40`, `NARROW_BREAKPOINT_WIDTH = 100`), `DS-TUIC-006 Shapes` (the `THICK` border for the confirm modal, `VERTICAL_RIGHT` scrollbars, and the ASCII fallback set), `DS-TUIC-011 States`, `DS-TUIC-012 Error States`, and `DS-TUIC-013 Accessibility`.

  The token table's own header comment says `utility_tui_theme.rs` "is the single source of truth for every token below" — a truth the document does not carry. A maintainer adding a layer badge, a new overlay, or a new status line has no written contract for which token to reach for, so the palette drifts by taste. This is the design-system deliverable for the scope, and it is empty.

  The `DS-TUIC-011/012/013` stubs are the root cause of the Scope 5 issues below: with no documented state matrix, "what does this screen show while a scan runs / when a scan fails / when the directory is empty" has no answer to implement against.

- **Acceptance Criteria**:
  - `DS-TUIC-002 Colors` lists all fifteen palette tokens with their ANSI value, their use case, and their NO_COLOR resolution (`Color::Reset` via `color()`).
  - `DS-TUIC-004 Layout & Spacing` states the four vertical rows (1 / min 10 / 3 / 1), the three horizontal percentages, the three breakpoints, and the single-row gutter that separates panels.
  - `DS-TUIC-006 Shapes` documents the border vocabulary (`Borders::ALL` for panels, `THICK` for the confirm modal, ASCII fallback under `NO_COLOR`) and the glyph fallback pairs (`│→|`, `⚠→!`, `█→=`, `░→ `).
  - `DS-TUIC-011` and `DS-TUIC-012` carry a screen × state matrix (idle / loading / empty / error / success / partial) naming the component and the copy for each cell.
  - `DS-TUIC-013` states the accessibility contract: NO_COLOR, `LINT_ARWAKY_TUI_HIGH_CONTRAST`, the screen-reader pointer to `lint-arwaky-cli`, and why full-screen redraw is a hard limit.
  - `cargo build` still passes the generated `DESIGN.md` template conformance constants (`DOC_HEADING_CONTRACTS`), and no stub table survives.
- **Recommendation**:
  Populate the tables from `utility_tui_theme.rs` verbatim — the document should be derivable from the code, and the code comments already carry the rationale. Where a row cannot be filled (for example, typography has no scale because the TUI uses the terminal's font), say so explicitly rather than leaving the stub: "Typography is owned by the terminal emulator; no scale is defined."

- **Git Diff**:

```diff
--- a/crates/tui/DESIGN.md
+++ b/crates/tui/DESIGN.md
@@
 ### DS-TUIC-002: Colors
 
-| Variant | Use case | Token | Value|
-|---------|----------|-------|-------|
-|         |          |       |       |
+| Variant | Use case | Token | Value|
+|---------|----------|-------|-------|
+| Accent | Focused panel border, selected file name, phase label | `ACCENT` | `Color::Cyan` |
+| Key | Shortcut key glyphs, tree expand/collapse markers | `KEY` | `Color::Yellow` |
+| Label | File names, status text, body copy | `LABEL` | `Color::White` |
+| Separator | Unfocused borders, `│` dividers, muted hint copy | `SEPARATOR` | `Color::DarkGray` |
+| Violations | Violation counts > 0 | `VIOLATIONS` | `Color::Red` |
+| Clean | Violation counts = 0, installed adapters | `CLEAN` | `Color::Green` |
+| Pending | Busy state, confirm-modal border, warning glyph | `PENDING` | `Color::Yellow` |
+| Directory | Directory names, Contract layer badge | `DIRECTORY` | `Color::Blue` |
+| Capabilities badge | `[cap]` layer badge | `CAPABILITIES_BADGE` | `Color::Magenta` |
+| Background | Panel and overlay canvas fill | `BACKGROUND` | `Color::Black` |
+| Highlight | Selected row band (default mode) | `HIGHLIGHT` | `Color::DarkGray` |
+| Path input | Path dialog input text | `PATH_INPUT` | `Color::Yellow` |
+| Focus confirm | Path dialog `>` prompt | `FOCUS_CONFIRM` | `Color::Green` |
+| Scrollbar | Scrollbar thumb and track | `SCROLLBAR` | `Color::DarkGray` |
+| Header | App title in the header row | `HEADER` | `Color::Cyan` |
+
+All values resolve through `utility_tui_theme::color()`, which returns
+`Color::Reset` when `NO_COLOR` is set, delegating both foreground and
+background to the terminal.
```

- **Open Questions**:
  - Should the generated doc template be extended to require a non-empty token table, so this cannot regress silently?
  - Are the per-layer badge colors (`[tax]` cyan, `[con]` blue, `[uti]` yellow, `[cap]` magenta, `[agt]` green, `[sur]` red, `[root]` white, `[---]` gray) part of the design system, or an implementation detail of `layer_color`?

#### Issue UX-3-02-20261109T052544Z (`#1324`)

- **Title**: [UX][WARNING] Three severity vocabularies and two badge sets across the report surfaces
- **Label**: `visual`, `content`, `severity-warning`
- **Location Path**:
  `crates/report-formatter/src/capabilities_text_formatter.rs:206-215` (`severity_badge`) · `crates/shared/src/cli_commands/utility_output_text_formatter.rs:261-279` (`render_violation`, no severity at all) · `crates/tui/src/surface_lint_action.rs:700-717` (`format_violations`, third style) · `crates/shared/src/common/taxonomy_severity_vo.rs:19-29` (`Display` = lowercase)
- **Description**:
  Closed issue #361 raised the fragmentation; the three surfaces still disagree:

  | Surface | How a severity renders |
  |---|---|
  | `report-formatter` text | `[!!!]` `[!! ]` `[!  ]` `[.  ]` `[   ]` — a glyph-width ladder, five levels |
  | `scan` text (`utility_output_text_formatter`) | **nothing** — `render_violation` prints `(file:line)`, `CODE:NAME`, `WHY`, `FIX` and omits severity entirely |
  | TUI lint results | `[critical]` / `[high]` — `Display` lowercased, in brackets, on the header line and again on `Severity: critical` |

  The first two are *both* reachable from `lint-arwaky-cli scan .` depending on which formatter handles the report, and they present the same finding with and without a severity marker. The TUI repeats the severity twice per finding (once in brackets, once as a labelled field) while the CLI's own scan output prints it zero times. A reviewer triaging by severity — the CRITICAL-first workflow the `ci` command exists to enforce — cannot do it from the `scan` text output at all.

  `[!! ]` is also a trap: the trailing space is load-bearing for alignment but invisible when found in a log, and a copy-paste into an issue or a grep turns it into `[!!]`, which matches nothing.
- **Acceptance Criteria**:
  - One severity vocabulary serves all text surfaces, exported from a single module in `shared`, with a test that asserts every surface renders the same string for the same `Severity`.
  - The `scan` text output prints severity per finding in the same position across every `Format::Text` path.
  - No badge depends on trailing whitespace for meaning; alignment is achieved by padding at the print site, not by the badge literal.
- **Recommendation**:
  Standardise on a fixed-width, space-free badge (`[CRIT]`, `[HIGH]`, `[MED]`, `[LOW]`, `[INFO]`), place it at the start of the violation line, and delete `severity_badge` from `report-formatter` in favour of the shared helper. In the TUI, keep the label form (`Severity: critical`) and drop the bracketed duplicate.
- **Git Diff**:

```diff
--- a/crates/report-formatter/src/capabilities_text_formatter.rs
+++ b/crates/report-formatter/src/capabilities_text_formatter.rs
@@
 fn severity_badge(sev: &shared_common::Severity) -> &'static str {
     match sev {
-        shared_common::Severity::CRITICAL => "[!!!]",
-        shared_common::Severity::HIGH => "[!! ]",
-        shared_common::Severity::MEDIUM => "[!  ]",
-        shared_common::Severity::LOW => "[.  ]",
-        shared_common::Severity::INFO => "[   ]",
+        // Fixed width without trailing spaces: `[!! ]` only aligned while it
+        // stayed in the source file, and any copy-paste or grep stripped the
+        // space that carried its meaning.
+        shared_common::Severity::CRITICAL => "[CRIT]",
+        shared_common::Severity::HIGH => "[HIGH]",
+        shared_common::Severity::MEDIUM => "[MED] ",
+        shared_common::Severity::LOW => "[LOW] ",
+        shared_common::Severity::INFO => "[INFO]",
     }
 }
```

- **Open Questions**:
  - Do downstream consumers parse the `[!!!]` ladder? If any CI script greps it, the change needs a deprecation window.

#### Issue UX-3-03-20261109T052544Z (`#1325`)

- **Title**: [UX][INFO] ASCII fallback glyphs and border set are re-implemented in each component instead of the theme
- **Label**: `visual`, `severity-info`
- **Location Path**:
  `crates/tui/src/surface_file_list_view.rs:6` (`use ratatui::style::{Color, Style}`) · `crates/tui/src/surface_status_component.rs:14-37` (`vsep`, `warn`, `bar_filled`, `bar_empty`) · `crates/tui/src/surface_confirm_component.rs:22-39` (`ASCII_BORDER`, `warn`) · `crates/tui/src/surface_bindings_store.rs:543-546` (`RESERVED_CONTROL_CHARS`)
- **Description**:
  The theme file's contract (`utility_tui_theme.rs:1-16`) is that no palette constant is applied without going through `color()`, and that the only remaining NO_COLOR handling is "the ASCII glyph substitution in the status bar (#365)". In practice the ASCII fallback is re-implemented per component: `surface_confirm_component.rs` defines its own `warn()` returning `"!"` and its own eight-field `ASCII_BORDER` set, duplicating the `+---+`/`|` vocabulary that a shared helper would own. `surface_file_list_view.rs:6` imports `Color` directly to type `layer_color`'s return, which is the correct type but reads as a palette escape at the import line.

  Adding a fifth overlay today means copying the ASCII border set a third time; a change to the fallback glyphs has four call sites and no test that they agree. Closed #559 fixed this class of drift for the doctor formatter by adding a shared helper, and the pattern has already re-diverged.

- **Acceptance Criteria**:
  - A single `glyph` module in `shared_tui` owns every ASCII fallback (`sep`, `warn`, `bar_filled`, `bar_empty`, `border_set`) keyed on `NO_COLOR`; components call it instead of defining local copies.
  - A test asserts the fallback border set is byte-identical wherever it is used.
  - No component imports `Color` except to name a token type.
- **Recommendation**:
  Move `vsep`, `warn`, `bar_filled`, `bar_empty`, and `ASCII_BORDER` into `utility_tui_theme` next to `color()` and `highlight_style()`. That keeps the "NO_COLOR collapses in one place" claim in the module header true instead of aspirational.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_confirm_component.rs
+++ b/crates/tui/src/surface_confirm_component.rs
@@
-/// ASCII border set used when NO_COLOR is set (#365).
-const ASCII_BORDER: border::Set<'static> = border::Set {
-    top_left: "+",
-    top_right: "+",
-    bottom_left: "+",
-    bottom_right: "+",
-    vertical_left: "|",
-    vertical_right: "|",
-    horizontal_top: "-",
-    horizontal_bottom: "-",
-};
-
-/// Returns the warning glyph shown before the action label.
-/// Falls back to "!" when NO_COLOR is set (#365).
-fn warn() -> &'static str {
-    if theme::no_color() { "!" } else { "\u{26a0}" }
-}
+// Fallbacks live in the theme module so a second overlay cannot ship a
+// different border set under the same NO_COLOR contract.
+use shared_tui::utility_tui_theme::{ascii_border_set, warn_glyph};
```

- **Open Questions**:
  - Should the fallback border set be a `const` (no allocation) even though it is resolved at runtime, given the set is a `Copy` struct?

---

### Scope 4: Specify inclusive usability requirements
<!-- Check accessibility standard, keyboard support, screen reader support, color contrast, target size, cognitive load, discoverability, and usability heuristic compliance -->

#### Issue UX-4-01-20261109T052544Z (`#1326`)

- **Title**: [UX][CRITICAL] A panic or error leaves the terminal in raw mode in the alternate screen
- **Label**: `usability`, `responsive`, `severity-critical`
- **Location Path**:
  `crates/tui/src/surface_tui_command.rs:54-95` (`run`: enter/leave sequences) · `crates/tui/src/surface_tui_command.rs:118-232` (draw + event loop, `?` propagation) · `crates/root_tui_main_entry.rs:1-20` (no guard)
- **Description**:
  `TuiCommandSurface::run` is the only place that restores terminal state:

  ```rust
  enable_raw_mode()?;
  crossterm::execute!(stdout(), EnterAlternateScreen)?;
  crossterm::execute!(stdout(), EnableMouseCapture)?;
  let result = self.event_loop(&mut terminal, &mut state, &views);
  disable_raw_mode()?;
  crossterm::execute!(stdout(), LeaveAlternateScreen)?;
  crossterm::execute!(stdout(), DisableMouseCapture)?;
  result
  ```

  Any `?` after the three enable calls — `Terminal::new`, `terminal.draw` (which returns `io::Result` and is propagated with `?` at `surface_tui_command.rs:200`), `event::poll`, `event::read` — returns early via the `?` operator *before* the restore lines run. There is no `Drop` guard, no `std::panic::set_hook`, and no signal handling anywhere in `crates/tui/src/` or `root_tui_main_entry.rs` (a grep for `set_hook|ctrlc|SIGINT|signal` returns only the unrelated `RESERVED_CONTROL_CHARS` comment). The dispatcher advertises panic isolation as a feature (`crates/dispatcher`), but the TUI runs any dispatcher panic inside `terminal.draw` on the main thread.

  The user-visible outcome, on the most common failure (a panic in a rule, a broken pipe when the terminal closes, or an `io::Error` from `draw`): raw mode stays on, the alternate screen stays active, and the shell prompt is invisible — keystrokes are not echoed and `Enter` produces `^M`. The user's terminal is unusable until they run `reset` or `stty sane`, and the panic message itself scrolls away with the alternate screen. This is the "inaccessible experience / broken user flow" class: one failure costs the user their session, and the recovery command is nowhere on screen.

  Closed #366 addressed action latency and scan cancellation, not teardown. Nothing in `crates/tui/tests/` asserts restore-on-failure, so a regression would be silent.

- **Acceptance Criteria**:
  - A guard type (or a scope-exit closure) restores raw mode, leaves the alternate screen, and disables mouse capture on every return path, including `Err` and unwind.
  - A test drives an event loop that returns `Err` after the enable calls and asserts `disable_raw_mode` and `LeaveAlternateScreen` were both executed (via an injected backend or a recorded call log).
  - A panic inside the render closure prints the panic message *after* the terminal is restored, so the message is readable.
  - No restore call can be skipped by a `?`.
- **Recommendation**:
  Introduce a `TerminalGuard` whose `Drop` performs the three restore calls, constructed immediately after the enables and dropped at end of scope. Additionally install `std::panic::set_hook` to restore before the default hook prints, and use `crossterm::terminal::disable_raw_mode()` inside it. This is the standard ratatui teardown pattern and costs ~20 lines.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_tui_command.rs
+++ b/crates/tui/src/surface_tui_command.rs
@@
     pub fn run(&self) -> anyhow::Result<()> {
         enable_raw_mode()?;
         crossterm::execute!(stdout(), EnterAlternateScreen)?;
         crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture)?;
+        // Every `?` below can return before the restore calls at the end of
+        // this function, leaving the caller's shell in raw mode on the
+        // alternate screen. The guard restores on every path, including unwind.
+        let _guard = TerminalGuard;
 
         let backend = CrosstermBackend::new(stdout());
         let mut terminal = Terminal::new(backend)?;
@@
-        disable_raw_mode()?;
-        crossterm::execute!(stdout(), LeaveAlternateScreen)?;
-        crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture)?;
-
         result
     }
```

- **Open Questions**:
  - Should the guard also flush `stdout` before restoring, so a half-drawn frame does not leak into the restored scrollback?
  - Does the project want SIGINT handled explicitly (restore, then exit 130) rather than relying on the unwinding guard?

#### Issue UX-4-02-20261109T052544Z (`#1327`)

- **Title**: [UX][WARNING] The confirm gate cannot be cancelled from the mouse and blocks navigation while looking modal-only
- **Label**: `usability`, `severity-warning`
- **Location Path**:
  `crates/tui/src/surface_confirm_component.rs:76-105` (modal body) · `crates/tui/src/surface_tui_command.rs:262-271` (key gate) · `crates/tui/src/surface_event_action.rs:786-800` (`handle_mouse_click` has no confirm check) · `crates/tui/src/surface_shortcut_component.rs:22-25` (bar keeps rendering browse shortcuts)
- **Description**:
  The confirm modal is the product's only data-loss gate: it guards live fix (`F`), install, init, install-hook, and uninstall-hook. It renders a `Confirm` block with two lines — the action label and `[Enter/y] confirm  [Esc/n] cancel` — but:

  1. **Mouse users cannot cancel.** `handle_mouse_click` never consults `state.actions.pending_confirm`; a click anywhere passes through to the panel hit-test and moves focus or selection behind the modal. There is no cancel button and no click-outside-to-dismiss. A user who armed `F` by accident with the mouse in hand has exactly one exit: the keyboard. crossterm mouse capture is enabled for the whole session, so the pointer is actively part of the interface everywhere except here.
  2. **The shortcut bar still advertises navigation.** The bar keeps rendering the browse rows underneath the modal, so the two brightest rows on screen offer `j/k/h/l`, `c`, `s`, `f` — every one of which is swallowed by the key gate and returns `TuiEvent::None`. Affordances are shown and then ignored.
  3. **The gate has no visible focus or default.** `[Enter/y] confirm` is listed first and styled with `KEY`, so the *destructive* choice is visually primary. For a data-loss gate the safe default should be the emphasized one, and the modal should name the consequence (`3 files will be rewritten`) rather than the action (`Apply live fixes to files`).

- **Acceptance Criteria**:
  - While a confirm is pending, a left click inside the modal's confirm/cancel regions dispatches `ConfirmAction`/`CancelConfirm`, and a click outside it dispatches `CancelConfirm`.
  - The shortcut bar renders a confirm-specific row set (`Enter confirm`, `Esc cancel`) for the duration of the gate.
  - `cancel` is the visually emphasized affordance; `confirm` uses the warning token only.
  - The modal body states the scope of the change when the executor can supply it.
- **Recommendation**:
  Add a `pending_confirm` branch to `handle_mouse_click` that hit-tests the modal rect (already computed by `modal_area`), and add a `ShortcutComponent` branch mirroring the existing `state.search.mode` branch that replaces the bar with the two gate keys. Swap the `KEY` emphasis to the cancel span.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_event_action.rs
+++ b/crates/tui/src/surface_event_action.rs
@@
     fn handle_mouse_click(&self, state: &mut AppState, col: u16, row: u16) {
+        // A pending confirm is a gate: every click resolves to cancel so the
+        // pointer cannot select a different file underneath the modal, and so
+        // a mouse-only user has an exit without reaching for the keyboard.
+        if state.actions.pending_confirm.is_some() {
+            let layout = crate::surface_tui_layout::compute_panel_layout(
+                state.terminal_width,
+                state.terminal_height,
+            );
+            let modal = crate::surface_confirm_component::modal_area(layout.band());
+            let over_confirm_control = row == modal.bottom().saturating_sub(2)
+                && col >= modal.x
+                && col < modal.right();
+            // Confirm requires the labelled control on the bottom line; every
+            // other click (including click-outside) cancels.
+            self.handle(
+                state,
+                if over_confirm_control {
+                    TuiEvent::ConfirmAction
+                } else {
+                    TuiEvent::CancelConfirm
+                },
+            );
+            return;
+        }
         let h = state.terminal_height;
```

```diff
--- a/crates/tui/src/surface_shortcut_component.rs
+++ b/crates/tui/src/surface_shortcut_component.rs
@@
-        // I1: while search mode is active, show the live search line instead of shortcuts.
+        // A pending confirm is a gate: advertising browse shortcuts underneath
+        // it showed affordances that the key gate swallows.
+        if state.actions.pending_confirm.is_some() {
+            let paragraph = Paragraph::new("Enter:confirm  Esc:cancel")
+                .style(Style::default().bg(theme::color(theme::BACKGROUND)));
+            frame.render_widget(paragraph, area);
+            return;
+        }
+
+        // I1: while search mode is active, show the live search line instead of shortcuts.
         if state.search.mode {
```

- **Open Questions**:
  - Should a click inside the modal's border area confirm or cancel? Cancelling is safer; confirming should require the labelled control.
  - Does `modal_area` belong in `surface_confirm_component` (where it is private today), or in `surface_tui_layout` alongside `hit_test` so overlays share one hit-testing source?

#### Issue UX-4-03-20261109T052544Z (`#1328`)

- **Title**: [UX][WARNING] "Terminal too small" is drawn outside the guaranteed-safe width and is the only guidance the user gets
- **Label**: `responsive`, `usability`, `severity-warning`
- **Location Path**:
  `crates/tui/src/surface_tui_command.rs:121-140` (too-small guard) · `crates/shared/src/tui/utility_tui_theme.rs:45-46` (`MIN_TERMINAL_WIDTH = 40`, `MIN_TERMINAL_HEIGHT = 15`)
- **Description**:
  When the terminal drops below 40×15 the renderer replaces the entire interface with one centered line: `"Terminal too small — resize to at least 40x15"`. That string is 45 characters widened; the guard fires at any width **below 40**, so the message is always too long for the terminal that triggers it. At 30 columns (55% of the minimum) the user sees `"Terminal too small — resize to at…"` and never learns the target size. At 20 columns it reads `"Terminal too small"` with the instruction clipped away — the message consumes the very resource it is telling the user to increase.

  The message is also the wrong shape as a state. It uses `Alignment::Center` on a single line with no wrapping, and it is the *only* content: the app does not draw a minimal fallback (list-only mode) even at 39 columns, where UX-2-02's narrow mode would have been perfectly usable. The threshold is not documented to the user before they hit it: nothing in the help overlay says 40×15 is the floor, so shrinking the window past 100 columns (breakpoint) and then past 40 (floor) produces two silent layout changes and then a message that cannot be read.

- **Acceptance Criteria**:
  - The too-small message wraps within the actual terminal width and always renders both the current size and the required size (e.g. `35×15 → need 40×15`).
  - At widths where the panels cannot render but the message can, the message is centered and fully visible at 20 columns or more.
  - Below the point where the message fits, a single short token (`too small`) renders instead of a clipped sentence.
  - The help overlay states the 40×15 minimum and the 100-column three-column breakpoint.
- **Recommendation**:
  Build the message from the live `area.width`/`area.height`, choose a long or short variant by measured length, and add `.wrap(Wrap { trim: true })`. Consider lowering `MIN_TERMINAL_WIDTH` for the narrow single-panel mode (the layout tests already assert 36 content columns at 40 — a 30-column fallback with the Tree panel suppressed is viable).
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_tui_command.rs
+++ b/crates/tui/src/surface_tui_command.rs
@@
                     if area.height < min_h || area.width < min_w {
-                        let message = format!(
-                            "Terminal too small — resize to at least {}x{}",
-                            min_w, min_h
-                        );
+                        // The long form is 45 columns wide; the guard fires
+                        // below 40, so the instruction was always clipped by
+                        // the very terminal it was asking the user to resize.
+                        let long = format!(
+                            "{}x{} → need {}x{}",
+                            area.width, area.height, min_w, min_h
+                        );
+                        let message = if long.chars().count() <= area.width as usize {
+                            long
+                        } else {
+                            "too small".to_string()
+                        };
                         let line = Line::from(vec![Span::styled(
                             message,
                             Style::default()
                                 .fg(shared_tui::utility_tui_theme::color(
                                     shared_tui::utility_tui_theme::KEY,
                                 ))
                                 .add_modifier(shared_tui::utility_tui_theme::EMPHASIS_HEADING),
                         )]);
                         let paragraph = Paragraph::new(line)
                             .style(Style::default().bg(shared_tui::utility_tui_theme::color(
                                 shared_tui::utility_tui_theme::BACKGROUND,
                             )))
+                            .wrap(ratatui::widgets::Wrap { trim: true })
                             .alignment(Alignment::Center);
                         frame.render_widget(paragraph, area);
                         return;
                     }
```

- **Open Questions**:
  - Is `BACKGROUND` applied here correct under `NO_COLOR`, or should the fallback screen delegate the background to the terminal like every other component?

---

### Scope 5: Define interface state behavior
<!-- Check loading state, empty state, error state, success state, partial data state, disabled state, retry behavior, and feedback clarity -->

#### Issue UX-5-01-20261109T052544Z (`#1329`)

- **Title**: [UX][CRITICAL] Esc cannot cancel a running scan — `CancelScan` is dispatched by no key, and Esc is bound to quit
- **Label**: `interaction`, `usability`, `severity-critical`
- **Location Path**:
  `crates/tui/src/surface_event_action.rs:367-373` (`CancelScan` handler) · `crates/tui/src/surface_bindings_store.rs:88-119` (`Quit` claims `Plain('q')`, `Ctrl('q')`, `Code(Esc)`) · `crates/tui/src/surface_tui_command.rs:262-313` (`from_key_event`) · `crates/tui/src/taxonomy_tui_event.rs:48` (`CancelScan` variant)
- **Description**:
  The scan is the product's headline operation and its only cancellable one (`ScanProgressState::cancel`, the `cancelled_early()` path, `ScanUpdate::Cancelled`). The cancel handler exists and is correct:

  ```rust
  TuiEvent::CancelScan => {
      if let Some(cancel) = state.scan.cancel.as_ref() {
          cancel.store(true, std::sync::atomic::Ordering::Relaxed);
          state.set_status("Cancelling scan...");
      }
  }
  ```

  **No key maps to it.** `CancelScan` appears in exactly three places in the crate: the enum variant, the logging arm, and the handler. It is not in `SHORTCUTS`, so `action_for` can never return it; it is not in `from_key_event`'s contextual branches, so no overlay produces it either. Meanwhile `Esc` — the universal cancel key, and the one the shortcut bar teaches for search cancel and confirm cancel — is bound to `Quit`, which on a running scan terminates the process without setting the cancel flag.

  The consequences, on a 10k-file workspace where a scan is the 15-second-to-60-second operation the PRD budget defines:

  - The status bar counts files and the user has no way to stop it. Pressing `Esc` (the habit the rest of the UI builds) kills the process, leaving the terminal to UX-4-01's teardown bug.
  - The status line during a scan says nothing about cancellation, so even a user who suspects a key exists has no affordance to try.
  - `ScanUpdate::Cancelled`'s careful `"Scan cancelled"` status and its zero-violation reset are dead code paths in practice.

  Closed #366 was about actions freezing the loop and cancellation being "cosmetic"; the flag machinery landed, but the key that sets it did not, so cancellation remains cosmetic — now for a different reason.

- **Acceptance Criteria**:
  - While `state.scan.running`, `Esc` dispatches `TuiEvent::CancelScan` and does **not** quit; a test drives `from_key_event(Esc)` with `scan.running == true` and asserts `CancelScan`.
  - The status line's scan branch names the binding (`Esc cancel`), or the shortcut bar swaps to a scan-context row set.
  - After cancel, the status reads `Scan cancelled` and the preview returns to `ActionOutput` without stale results.
  - `q`/`Ctrl+q` still quit at any time; only `Esc` is contextual.
- **Recommendation**:
  Add an early branch in `from_key_event` mirroring the search/help/confirm branches:

  ```rust
  if state.scan.running {
      return match key.code {
          KeyCode::Esc => TuiEvent::CancelScan,
          _ => /* fall through to the normal table */,
      };
  }
  ```

  and add `Esc cancel` to the scan branch of `StatusComponent`. Prefer this over rebinding `Quit`, so `Esc` keeps its confirm/search/path meaning and only gains a scan meaning.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_tui_command.rs
+++ b/crates/tui/src/surface_tui_command.rs
@@
 fn from_key_event(key: KeyEvent, state: &AppState) -> TuiEvent {
+    // A running scan owns Esc. `CancelScan` had a handler and a cancel token
+    // but no producer, so the only cancellable operation in the product could
+    // not be cancelled — pressing Esc quit the process instead.
+    if state.scan.running && key.code == KeyCode::Esc {
+        return TuiEvent::CancelScan;
+    }
+
     // --- Pending confirmation: only y/Enter = confirm, n/Esc = cancel (#354) ---
     if state.actions.pending_confirm.is_some() {
```

- **Open Questions**:
  - Should `Esc` require a second press to confirm cancellation, given a cancelled scan discards all work?
  - Should the shortcut bar show `Esc cancel` for the whole scan duration, or only after a few seconds?

#### Issue UX-5-02-20261109T052544Z (`#1330`)

- **Title**: [UX][WARNING] Empty, partial, and error states leave the preview panel blank with no recovery
- **Label**: `content`, `usability`, `severity-warning`
- **Location Path**:
  `crates/tui/src/surface_event_action.rs:596-632` (`load_directory`) · `crates/tui/src/surface_event_action.rs:635-670` (`load_file_preview`) · `crates/tui/src/surface_preview_view.rs:22-40` (mode → title/content) · `crates/tui/src/taxonomy_tui_vo.rs:352-360` (`preview.text` starts empty)
- **Description**:
  `load_directory` clears `preview.text` and sets `preview.mode = ActionOutput` with the comment "No file content preview — Preview panel stays empty until action is run". The panel then renders a bordered box with the title `Action Output` and no body. Four distinct states collapse into that same blank box:

  1. **Nothing has run yet** (startup, or after navigating into any directory) — the user sees an empty panel labelled "Action Output" and cannot tell whether the app is broken or simply idle. There is no "run `c` to check this folder" prompt even though the shortcut bar's own label is `c:check`.
  2. **Empty directory** — `load_directory` sets a status-bar message (`"Empty or inaccessible: {path}"`) but the panel is blank, and the status line is the row that the very next keypress overwrites.
  3. **Unreadable file** — `load_file_preview` substitutes `"Cannot read file: {path}"` into the preview (correct) but leaves `preview.mode = FileContent`, so the title says `Preview` and the body is a single error line with no remedy (permissions? binary? too large?).
  4. **Binary or empty file** — `content.is_empty()` collapses an actually-empty file and an unreadable one into the same message, so a legitimately empty file reports "Cannot read".

  The pattern is the one closed #368 identified ("bare error states") and partially fixed: it cleared stale preview text on navigation but did not put anything in its place.

- **Acceptance Criteria**:
  - The preview panel renders a distinct, actionable empty state per state: `Nothing selected`, `No files in this folder`, `Press c to check this folder`, `Cannot read this file (permission denied / binary / empty)`.
  - A genuinely empty file says `Empty file`, not `Cannot read`.
  - Every error state names one next action available from the current screen.
  - A rendering test asserts the panel body is non-empty for `entries.is_empty()` and for an idle post-navigation state.
- **Recommendation**:
  Give `PreviewState` an explicit `Empty` reason enum rather than overloading `preview.text`, and render helper copy from it. This pairs with UX-3-01's `DS-TUIC-011 States` table — the matrix should be the specification for these four cells.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_event_action.rs
+++ b/crates/tui/src/surface_event_action.rs
@@
         state.navigation.selected_index = 0;
         state.navigation.scroll_offset = 0;
         // Clear stale preview when directory changes (#368)
-        state.preview.text.clear();
-        // No file content preview — Preview panel stays empty until action is run
+        // An empty panel cannot be told apart from a broken one, so state what
+        // the panel is waiting for and which key provides it.
+        state.preview.text = "Press `c` to check this folder, or `s` to scan it.\n\n\
+                              Navigate with `j`/`k`, open with `l`/`↵`, and `?` for all keys."
+            .to_string();
         state.preview.mode = PreviewMode::ActionOutput;
```

- **Open Questions**:
  - Is `ActionOutput` the right title for an idle panel, or should the title change with the state (`Ready`, `No files`, `Error`)?

#### Issue UX-5-03-20261109T052544Z (`#1331`)

- **Title**: [UX][WARNING] The global violation badge goes stale — navigation shows the previous folder's count
- **Label**: `interaction`, `content`, `severity-warning`
- **Location Path**:
  `crates/tui/src/taxonomy_tui_vo.rs:328` (`violation_count`) · `crates/tui/src/surface_event_action.rs:115` and `:194` (the only two writers) · `crates/tui/src/surface_event_action.rs:596-632` (`load_directory` does not reset it) · `crates/tui/src/surface_status_component.rs:100-190` (renders it in all three states)
- **Description**:
  `state.violation_count` is written in exactly two places: `ScanUpdate::Complete` (`:115`) and `poll_background_action` (`:194`). It is **never** cleared by navigation. So the status bar's `⚠ N viol.` cell keeps the number from the last completed action no matter where the user goes:

  - Scan `crates/quality-rules` → `42 viol.`. Navigate to `crates/shared` → still `42 viol.`, a folder that has not been examined.
  - Run a check on a clean file → `0 viol.` (green). Navigate to a directory full of violations → the badge stays green and reads `0 viol.`, actively asserting cleanliness.
  - Search or filter changes the visible set; the number does not move.

  Because the badge is colour-coded (`VIOLATIONS` red above zero, `CLEAN` green at zero) it reads as a statement about the current view, not about the last action. The status line also carries `Selected: <file>` immediately beside it, reinforcing the reading that the number applies to what is on screen. A maintainer scanning for a clean member can be shown a green zero by an operation run against a different subtree.

  The per-file counters that would make the badge honest are the unwritten field noted in UX-2-03, so this is the same missing data surfacing as a false statement rather than a missing one.
- **Acceptance Criteria**:
  - `violation_count` is reset (and the badge rendered as `—` or hidden, not `0`) whenever `current_dir` or the selected entry changes.
  - The badge names its scope: `42 viol. in crates/quality-rules` or `42 viol. (last check)`, so a number can never be read as belonging to a different target.
  - `0 viol.` in green only renders for a result that actually covered the current view.
  - A test asserts that after `load_directory` the badge is unset, and after a scan of A the badge scoped to A is not shown while viewing B.
- **Recommendation**:
  Store the scope alongside the count (`violation_scope: Option<String>`) and clear both in `load_directory`/`select_next`/`select_prev`. Render `— viol.` when the scope does not match the current directory.
- **Git Diff**:

```diff
--- a/crates/tui/src/surface_event_action.rs
+++ b/crates/tui/src/surface_event_action.rs
@@
         state.navigation.selected_index = 0;
         state.navigation.scroll_offset = 0;
+        // The violation badge is scoped to the target the last action ran on.
+        // Leaving it in place claimed the previous folder's count for this one.
+        state.violation_count = 0;
+        state.violation_scope = None;
         // Clear stale preview when directory changes (#368)
```

- **Open Questions**:
  - Should the badge be hidden entirely rather than rendered as `—` when no result covers the current view?
  - Does the badge belong in the status bar at all, or should it move to the Files panel header (`Files — 42 violations`) once per-file counts exist?

---

## Violations

| # | Class | Finding | Severity | Issue |
|---|---|---|---|---|
| 1 | Broken flow | Recovery key `r` has no binding, `ChangeProjectRoot` has no producer, and the project root cannot be corrected after the dialog is dismissed | CRITICAL | #1317 |
| 2 | Broken flow | No teardown guard: any `?` after raw mode is enabled leaves the terminal unusable; no panic hook | CRITICAL | #1326 |
| 3 | Missing state | `CancelScan` has no key binding; `Esc` quits mid-scan instead of cancelling | CRITICAL | #1329 |
| 4 | Responsive | Shortcut bar rows are 158/140 columns against a 100-column breakpoint and a 40-column floor; `?` and `/` are the first casualties | WARNING | #1320 |
| 5 | Content | Failing `ci` run prints neither `Result: FAIL`, the exit code, nor any entry from `reasons` | WARNING | #1319 |
| 6 | Discoverability | TUI absent from README Quick Start and Reference; only an install-script alias mentions it (#356 regressed) | WARNING | #1318 |
| 7 | Design system | All 13 `DS-TUIC-*` sections in `crates/tui/DESIGN.md` are empty stub tables | WARNING | #1323 |
| 8 | Design system | Three severity vocabularies; the `scan` text output prints none | WARNING | #1324 |
| 9 | Missing state | Progress bar has a zero denominator at start and no completion frame | WARNING | #1321 |
| 10 | Accessibility | Confirm gate is keyboard-only, keeps browse shortcuts visible, and emphasizes the destructive choice | WARNING | #1327 |
| 11 | Responsive | "Terminal too small" is 45 columns wide and fires below 40 columns, so it always clips | WARNING | #1328 |
| 12 | Missing state | Idle / empty-folder / unreadable-file states all render as one blank `Action Output` panel | WARNING | #1330 |
| 13 | Content | Violation badge is never reset by navigation and states a count for the wrong scope | WARNING | #1331 |
| 14 | Visual | `FileEntry.violation_count` is declared, written by nobody, and never rendered | INFO | #1322 |
| 15 | Visual | ASCII fallback glyphs and border set duplicated across four components | INFO | #1325 |

## Action Items

- [ ] **P0** Add a `TerminalGuard` and panic hook so raw mode and the alternate screen are restored on every exit path (UX-4-01).
- [ ] **P0** Bind `r` to `ChangeProjectRoot` in `SHORTCUTS` and give the dismissed path dialog an actionable empty state (UX-1-01).
- [ ] **P0** Make `Esc` dispatch `CancelScan` while a scan is running (UX-5-01).
- [ ] **P1** Print `Result: FAIL (exit code 2)` and every failure reason in `handle_ci` (UX-1-03).
- [ ] **P1** Budget the shortcut bar by width and priority; guarantee `?:help` and `/:search` survive (UX-2-01).
- [ ] **P1** Clear and scope the violation badge on navigation (UX-5-03).
- [ ] **P1** Populate `crates/tui/DESIGN.md` DS-TUIC-001…013 from `utility_tui_theme.rs` (UX-3-01).
- [ ] **P1** Replace the idle blank preview with per-state guidance (UX-5-02).
- [ ] **P1** Make the confirm gate mouse-cancellable and swap the shortcut bar while it is pending (UX-4-02).
- [ ] **P1** Fix the too-small screen to wrap and to state both sizes (UX-4-03).
- [ ] **P2** Unify severity vocabulary across text surfaces (UX-3-02).
- [ ] **P2** Add the TUI to README Quick Start and Reference (UX-1-02).
- [ ] **P2** Make the scan progress bar determinate-or-indeterminate, never an empty determinate bar (UX-2-02).
- [ ] **P3** Render per-file violation counts in the Files panel (UX-2-03).
- [ ] **P3** Hoist ASCII fallbacks into `utility_tui_theme` (UX-3-03).

## Fixed Design

### Screen: Startup path dialog (`surface_path_screen.rs`)

- Title `Enter Project Path`; body states the current `cwd` as committed text so `Enter` accepts it.
- Footer keeps `[Enter] Confirm  [Tab] Use current dir  [Esc] Cancel` and adds `[r] re-open after cancel` only in the cancelled state.
- `lint-arwaky-tui <PATH>` bypasses the dialog entirely.
- Cancelled first run: Files panel shows a centered `No project selected — press r to choose a folder`; no empty chrome.

### Screen: Three-panel browser (`surface_tui_command.rs`, 100+ columns)

```
 lint-arwaky TUI │ Path: lint-arwaky                          [q/Esc] Quit
┌ Tree ────────────────┐┌ Files ──────────────┐┌ Lint Results ──────────────┐
│ [*] lint-arwaky      ││ [tax] taxonomy_...   ││ {crates}                   │
│   [-] crates/        ││ [sur] surface_tui... ││                            │
│     [-] shared/      ││ [---] README.md      ││ [crates/tui]               │
│       [+] src/       ││ [uti] utility_tui... ││ (src/surface_status_...)   │
│                      ││          ▸ 12        ││ AES501:UNUSED_SYMBOL       │
│                      ││                      ││ WHY: …                     │
│                      ││                      ││ FIX: …                     │
└──────────────────────┘└──────────────────────┘└────────────────────────────┘
 j/↓:down  k/↑:up  h/←:back  l/→/⏎:open  c:check  s:scan  f:fix (dry)  …  ?:help
 I:install  m:mcp  C:config  r:root
 Status: Ready │ Selected: surface_status_component.rs │ ⚠ 0 viol. (last check)
```

- Rows 1–3 of the shortcut band are width-budgeted; the trailing cell is always `… ?:help`.
- The violation cell names its scope; `— viol.` when no result covers the current directory.
- Per-file counts render right-aligned in the Files panel after a scan.

### Screen: Narrow mode (< 100 columns)

- Single active panel fills the band; `Tab`/`BackTab` cycles Tree → Files → Preview.
- Shortcut band collapses to two rows, then to one `?:help` line below 60 columns.
- Below 40 columns: wrapped `35×15 → need 40×15`, or the token `too small` when even that cannot fit.

### Component: Scan progress (status bar)

| Phase | Rendering |
|---|---|
| Keypress → first progress | `⠋ Starting scan… (waiting for file count)` |
| Determinate | `Scanning ████████░░░░ 4021/10240 files │ 128 violations` |
| Cancelling | `Cancelling scan…` (Esc consumed; second Esc ignored) |
| Complete | `Done — 128 violations in 12.4s` for one frame, then results |
| Cancelled | `Scan cancelled — no results written` |

### Component: Confirm gate (overlay)

- Thick border, warning token on the title only; body states the scope (`3 files will be rewritten`).
- `[Esc/n] cancel` is the emphasized affordance; `[Enter/y] confirm` is plain.
- Any click: inside the modal's cancel region and outside the modal both cancel; there is no click-to-confirm.
- Shortcut band swaps to `Enter:confirm  Esc:cancel` for the gate's lifetime.

### Component: Severity badge (all text surfaces)

| Severity | Badge |
|---|---|
| CRITICAL | `[CRIT]` |
| HIGH | `[HIGH]` |
| MEDIUM | `[MED]` |
| LOW | `[LOW]` |
| INFO | `[INFO]` |

Placed at line start, followed by one space, then the code. No badge contains trailing whitespace.

## Severity

| Level | Meaning |
|-------|---------|
| CRITICAL | Broken user flow, inaccessible experience, missing critical state, or design that risks user error and data loss. Immediate fix. |
| WARNING | Usability friction, inconsistent design system, missing feedback, unclear prototype, or incomplete state handling. Fix this cycle. |
| INFO | Visual polish, microinteraction, or nice to have improvement. Deferrable. |

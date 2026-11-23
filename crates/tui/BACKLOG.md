# TUI — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 13 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p tui --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog Status

### DS-TUIC-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Contrast Ownership; source: `crates/shared/src/tui/utility_tui_theme.rs`, `crates/tui/src/taxonomy_tui_vo.rs`; tests: `crates/tui/tests/unit_tui_theme_no_color.rs`, `crates/tui/tests/acceptance_tui.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Colors; source: `crates/shared/src/tui/utility_tui_theme.rs`, `crates/tui/src/taxonomy_tui_vo.rs`; tests: `crates/tui/tests/unit_tui_theme_no_color.rs`, `crates/tui/tests/unit_tui_emphasis_tokens.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Typography; source: `crates/shared/src/tui/utility_tui_theme.rs`, `crates/tui/src/taxonomy_tui_vo.rs`; tests: `crates/tui/tests/unit_tui_emphasis_tokens.rs`, `crates/tui/tests/unit_tui_theme_no_color.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Layout & Spacing; source: `crates/tui/src/surface_tui_layout.rs`, `crates/tui/src/surface_event_action.rs`; tests: `crates/tui/tests/unit_tui_mouse.rs`, `crates/tui/tests/unit_tui_layout_breakpoints.rs`, `crates/tui/tests/unit_tui_help_scroll.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Elevation & Depth; source: `crates/tui/src/surface_tree_view.rs`, `crates/tui/src/surface_preview_view.rs`; tests: `crates/tui/tests/acceptance_tui.rs`, `crates/tui/tests/regression_tui_ux_fixes.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Shapes; source: `crates/tui/src/surface_tui_layout.rs`, `crates/tui/src/surface_confirm_component.rs`; tests: `crates/tui/tests/unit_tui_layout_breakpoints.rs`, `crates/tui/tests/regression_tui_confirm_gate.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Kind; source: `crates/tui/src/surface_tui_command.rs`, `crates/tui/src/taxonomy_tui_event.rs`; tests: `crates/tui/tests/unit_tui_outcome_states.rs`, `crates/tui/tests/acceptance_tui.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-008

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Request Shape; source: `crates/tui/src/surface_event_action.rs`, `crates/tui/src/surface_tui_layout.rs`; tests: `crates/tui/tests/unit_tui_theme_no_color.rs`, `crates/tui/tests/unit_tui_mouse.rs`, `crates/tui/tests/unit_tui_help_scroll.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-009

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Invariants; source: `crates/tui/src/surface_event_action.rs`, `crates/tui/src/surface_tui_command.rs`; tests: `crates/tui/tests/unit_tui_mouse.rs`, `crates/tui/tests/unit_tui_help_scroll.rs`, `crates/tui/tests/regression_tui_ux_fixes.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-010

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Change Checklist; source: `crates/tui/src/surface_tui_command.rs`, `crates/tui/src/surface_file_list_view.rs`; tests: `crates/tui/tests/unit_tui_theme_no_color.rs`, `crates/tui/tests/unit_tui_emphasis_tokens.rs`, `crates/tui/tests/unit_tui_mouse.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-011

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | States; source: `crates/tui/src/surface_file_list_view.rs`, `crates/tui/src/surface_event_action.rs`; tests: `crates/tui/tests/regression_tui_ux_fixes.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-012

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Error States; source: `crates/tui/src/surface_event_action.rs`; tests: `crates/tui/tests/unit_tui_emphasis_tokens.rs`, `crates/tui/tests/regression_tui_ux_fixes.rs`, `crates/tui/tests/dogfood_tui.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-TUIC-013

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Accessibility; source: `crates/tui/src/surface_bindings_store.rs`, `crates/tui/src/surface_shortcut_component.rs`; tests: `crates/tui/tests/unit_tui_help_scroll.rs`, `crates/tui/tests/acceptance_tui.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| DESIGN.md | Surface design | [DESIGN.md](DESIGN.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

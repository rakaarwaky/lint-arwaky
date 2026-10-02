# Dispatcher — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-02

## Current Condition

- Done: `cargo test -p dispatcher --lib --tests` → 0 failures
- Done: single-file scan reaches the external adapters (`collect_single_file_external`)
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| DISP-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| DISP-02 | P0 | Done | On Track | None | — | 2026-10-02  |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| Single-file `.md` target selects the markdown adapter | `has_markdown` true, all other language flags false | Automated | `tests/unit_dispatcher_check_action.rs` | `markdown_file_context_selects_the_markdown_adapter` | `ec7e16a7` |
| Single-file source target selects only its own language | `has_markdown` false for `.rs`/`.py`/`.ts`/`.tsx` | Automated | `tests/unit_dispatcher_check_action.rs` | `source_file_context_selects_only_its_own_language` | `ec7e16a7` |
| Extensionless single-file target selects no adapter | All four language flags false | Automated | `tests/unit_dispatcher_check_action.rs` | `extensionless_file_context_selects_no_adapter` | `ec7e16a7` |

## Blockers

None.

## Dependencies

None.

## Release Readiness

Done.

## Deferred

—

## Change Log

| Date | Change |
|---|---|
| 2026-09-29 | Created backlog entry for dispatcher surface crate |
| 2026-10-02 | `collect_single_file_external` routes a one-file scan target through the external adapters; language flags derive from the file extension instead of a project walk |
| 2026-10-02 | `scan <file.md>` now runs the external adapters: the single-file path derives its language flags from the file extension, so the markdownlint adapter fires on a lone Markdown target |

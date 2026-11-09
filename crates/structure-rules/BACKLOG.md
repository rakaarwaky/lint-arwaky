# Feature Backlog: Structure Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-09

## Progress Overview

- Done: AES702 root-container enforcement added: a feature folder must hold a `root_<concept>_container` or `root_<concept>_entry` file under it, in its `src/` subfolder, or at the member root (a member-root composition file answers for the features beneath it). New violation type `feature_missing_root_container`. `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- Done: AES702 shared-layer mirror check added: a feature folder must be mirrored as a `shared/src/<feature>/` subfolder in its member's layer. New violation type `feature_missing_shared_subfolder`; surface folders are exempt. `cargo nextest run -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- Done: AES704 (test-suite coverage) added; the two contract files were renamed to `contract_`/`unit_` prefixes so AES103 passes, and `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, and `benches/bench_structure_rules.rs` were added to satisfy AES704. `cargo nextest run -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- Done: AES706 UX fixes from issue #1375 — (1) violation names no longer hardcode lint-arwaky's `LA` segment: `pr_la_id_mismatch` → `pr_id_mismatch`, `pr_la_missing_frd` → `pr_missing_frd`, `pr_la_fr_id_mismatch` → `pr_fr_id_mismatch`, `pr_la_folder_without_pr_la_id` → `pr_folder_without_id`; (2) the crate folder order skips non-crate directories (`__pycache__`, `node_modules`, `target`, `dist`, …), so a bytecode cache no longer takes position 0 and shifts every id onto its predecessor's folder; (3) the folder order is derived per member dir, and a document that carries one `PR-<segment>` sequence per language audits each sequence against its own member's order instead of one flattened list. Acceptance tests `violation_names_carry_no_hardcoded_project_segment`, `cache_directory_does_not_shift_the_folder_order`, `dependency_directory_is_not_reported_as_a_crate_folder`, and `multi_language_document_scopes_each_sequence_to_its_own_member` pin each fix.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p structure-rules-lint-arwaky --lib --tests` after any code change to this crate

## Backlog Status

### FR-STR-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-STR-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-STR-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-STR-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-STR-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-STR-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | STR |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-09 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

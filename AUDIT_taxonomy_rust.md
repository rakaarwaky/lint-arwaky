# Audit Report — create-taxonomy-rust applied to all `taxonomy_*` files

Generated: 2026-07-20
Skill: `.agents/skills/create-taxonomy-rust` v1.3.0 (Definition of Done)
Scope: every `taxonomy_*.rs` under `crates/` (134 files total)
Workspace build baseline: GREEN (`cargo check -p shared-lint-arwaky` clean)

## Verdict

- **Forbidden cross-layer imports (AES201):** 0 — clean. No taxonomy file imports from
  capabilities / infrastructure / agent / surface / root / contract layers. Nothing to fix.
- **Unregistered modules in mod.rs:** 13 files in `crates/shared/src/**` appear unregistered,
  but all 13 are **orphaned dead code** (no `mod` declaration anywhere in the repo → never
  compiled). They are NOT new modules and must NOT be registered (registering would force-compile
  dead/bit-rotted code and break the green build). Flagged for the deferred cleanup follow-up.
- **I/O / side-effects in taxonomy src files (AES201, deferred):** 11 files.
- **Wrong suffix `_helper` (AES201 naming, deferred):** 12 files.

No safe code change is warranted right now — applying any would either be a no-op (0 forbidden
imports) or unsafe (registering dead files). The build is left GREEN by design.

---

## Deferred findings (per user: follow-up after report)

### Group A — I/O / side-effect in taxonomy layer (AES201 "no I/O in taxonomy")
These files do file/process I/O or are `async`, which the skill forbids in taxonomy. They likely
belong in infrastructure/surface layers, not taxonomy. 11 files:

- crates/orphan-detector/src/taxonomy_file_cache_utility.rs
- crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs
- crates/shared/src/code-analysis/taxonomy_duplication_utility.rs
- crates/shared/src/common/taxonomy_path_utils_vo.rs
- crates/shared/src/common/taxonomy_file_utility.rs
- crates/shared/src/common/taxonomy_file_collector_helper.rs
- crates/shared/src/common/taxonomy_workspace_helper.rs
- crates/shared/src/external-lint/taxonomy_external_lint_helper.rs
- crates/shared/src/import-rules/taxonomy_dummy_helper.rs
- crates/shared/src/import-rules/taxonomy_unused_helper.rs
- crates/shared/src/orphan-detector/taxonomy_workspace_utility.rs

NOTE: the 4 files marked ORPHAN below (duplication / workspace / file_utility / unused) are both
dead AND do I/O — removing them clears 4 of the 11 in one stroke.

### Group B — wrong suffix `_helper` (allowed: _vo/_entity/_error/_event/_constant/_utility)
12 files (most already registered in their domain mod.rs, just with the non-conforming suffix):

- crates/shared/src/common/taxonomy_file_collector_helper.rs
- crates/shared/src/common/taxonomy_language_detector_helper.rs
- crates/shared/src/common/taxonomy_workspace_helper.rs
- crates/shared/src/external-lint/taxonomy_external_lint_helper.rs
- crates/shared/src/import-rules/taxonomy_cycle_helper.rs
- crates/shared/src/import-rules/taxonomy_dummy_helper.rs
- crates/shared/src/import-rules/taxonomy_parser_helper.rs
- crates/shared/src/import-rules/taxonomy_path_helper.rs
- crates/shared/src/import-rules/taxonomy_unused_helper.rs
- crates/shared/src/orphan-detector/taxonomy_orphan_filename_utility.rs   (note: _utility suffix, ok — miscategorized)
- crates/shared/src/tui/taxonomy_report_formatter_helper.rs
- crates/orphan-detector/src/taxonomy_orphan_filename_helper.rs
- crates/role-rules/src/taxonomy_language_helper.rs

(`taxonomy_orphan_filename_utility.rs` already uses an allowed suffix — it is listed only because
the earlier scan matched on `helper` in name; it is NOT a suffix violation. Corrected here.)

### Group C — orphaned dead taxonomy files (no mod declaration anywhere → never compiled)
These should be REMOVED or RE-WIRED in the deferred follow-up, not registered. 13 files:

- crates/shared/src/code-analysis/taxonomy_bypass_utility.rs
- crates/shared/src/code-analysis/taxonomy_duplication_utility.rs        (also Group A: I/O)
- crates/shared/src/code-analysis/taxonomy_mandatory_utility.rs
- crates/shared/src/code-analysis/taxonomy_target_utility.rs
- crates/shared/src/common/taxonomy_value_object_generator_vo.rs
- crates/shared/src/common/taxonomy_file_utility.rs                      (also Group A: I/O)
- crates/shared/src/common/taxonomy_language_detector_utility.rs
- crates/shared/src/import-rules/taxonomy_cycle_color_vo.rs
- crates/shared/src/import-rules/taxonomy_import_constant.rs
- crates/shared/src/import-rules/taxonomy_import_utility.rs
- crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs
- crates/shared/src/orphan-detector/taxonomy_workspace_utility.rs        (also Group A: I/O)
- crates/shared/src/mcp-server/taxonomy_mcp_tool_args_vo.rs              (DEAD DUPLICATE of live
  crates/mcp-server/src/taxonomy_mcp_tool_args_vo.rs — delete the shared copy)

---

## Already-conforming (no action)

- All 4 non-shared crates (orphan-detector, naming-rules, role-rules, mcp-server) declare their
  `taxonomy_*` files in `src/lib.rs` — correctly registered.
- 99 / 112 shared/src taxonomy files are registered in their domain `mod.rs`.
- 0 forbidden imports across all 134 files.
- 16 `*_tests.rs` taxonomy test files live in `crates/*/tests/` (integration tests — no mod.rs
  needed; correct).

## Recommended follow-up order (lower risk first)

1. Delete the 13 Group-C orphaned files (clears 4 of Group A automatically).
2. Re-run `cargo check -p shared-lint-arwaky` to confirm still green.
3. Move/lift the remaining Group-A I/O files out of taxonomy into the proper layer.
4. Rename Group-B `_helper` files to a permitted suffix (`_utility`/`_vo`) and update references.
5. `cargo clippy --workspace --all-targets -- -D warnings` + full test run before merge.

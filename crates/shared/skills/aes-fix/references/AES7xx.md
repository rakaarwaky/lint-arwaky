# AES7xx — Structure Checks

## What the rule checks

Structure checks live in the `structure-rules` crate. In the codebase these checks emit **AES701–AES706**:

- **AES701** — shared/layer folder purity: shared folders hold only taxonomy, utility, and contract files; each shared layer folder carries DATA.md + BACKLOG.md.
- **AES702** — feature folder health: a feature folder holds capabilities + orchestrator pairs, has no foreign files, and carries FRD.md + BACKLOG.md doc pairs.
- **AES703** — surface folder purity: surface-dominated folders carry only surface files plus permitted support files.
- **AES704** — test-suite coverage: every feature folder owning source carries at least one file per test type (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`) plus one `bench_` benchmark; the prefix is the virtual folder (shared with AES103 vocabulary).
- **AES705** — member-root placement: `crates/`, `modules/`, `packages/` hold folders (not loose files).
- **AES706** — PR-LA folder mapping: every crate folder's FR ids match its folder name.

Checkers: `capabilities_shared_purity_auditor` (AES701), `capabilities_feature_health_auditor` (AES702), `capabilities_surface_purity_auditor` (AES703), `capabilities_test_suite_coverage_auditor` (AES704), `capabilities_member_root_placement_auditor` (AES705), `capabilities_pr_la_folder_auditor` (AES706), orchestrated by `agent_structure_orchestrator` via `root_structure_rules_container`.

## Common violation causes

- A shared folder contains a non-taxonomy/utility/contract file (e.g. an agent or surface).
- A feature folder lacks a capabilities file, an orchestrator, or its FRD/BACKLOG doc pair.
- A surface-dominated folder contains non-surface files.
- A feature folder's `tests/`/`benches/` is missing a test-type category or the bench file.
- A loose `.rs`/`.py`/`.ts` file sits directly in `crates/`, `modules/`, or `packages/` instead of in a folder.
- A crate folder's FR ids don't match its folder name.

## How to fix (concrete steps)

1. Identify the violated rule code (AES701–AES706) and folder from the scan report.
2. For AES701: move non-conforming files out of the shared folder into their correct layer; add DATA.md + BACKLOG.md.
3. For AES702: add the missing capabilities file and/or agent orchestrator; add FRD.md + BACKLOG.md; remove foreign files.
4. For AES703: move non-surface files out of the surface folder.
5. For AES704: add one file per missing test-type prefix in `tests/` and a `bench_<subject>` in `benches/`; keep the directory flat (no subdirectories — the prefix IS the virtual folder).
6. For AES705: move loose files into folders under `crates/`/`modules/`/`packages/`.
7. For AES706: align the crate folder's FR ids with its folder name (or rename the folder).
8. Re-run `lint-arwaky-cli scan .` and confirm the structure finding is gone.

## Example

Before (AES704 — feature folder missing `acceptance_` test):
```
crates/calc/tests/
├── contract_calc.rs
├── unit_calc_core.rs
└── integration_calc.rs
```

After:
```
crates/calc/tests/
├── contract_calc.rs
├── unit_calc_core.rs
├── integration_calc.rs
├── dogfood_pipeline.rs
├── smoke_app.rs
├── e2e_flow.rs
└── acceptance_FR_001.rs
crates/calc/benches/
└── bench_calc.rs
```

## Verify

```bash
lint-arwaky-cli scan .
# Confirm no AES701–AES706 finding remains for the affected folder.
```

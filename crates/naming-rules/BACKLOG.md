# Feature Backlog: Naming Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-09

## Progress Overview

- Done: **UX pass on the naming audit (#1378)** — the three rules now share one finding template (problem, action, rule, in three distinct fields), the emitted text is ASCII only and inside a 72-of-80-column budget, each rule carries more than one severity level, every file is evaluated by path (a file with no content included), and the audit reports how many files each rule read. The crate FRD now carries the developer journey, one rendered sample per rule, the severity model, the suppression section and the column contract. Evidence: `crates/naming-rules/tests/acceptance_naming_output_contract.rs` (the output contract), `crates/naming-rules/tests/acceptance_naming_empty_file_policy.rs` (the empty-file policy), `crates/naming-rules/FRD.md` (the journey, the samples, the suppression section). Run `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` to confirm.
- Done: **AES103** (test/bench file-prefix discipline) added as this crate's third seam, backed by `capabilities_test_file_prefix_checker.rs`, the `ITestFilePrefixProtocol` contract, and a `tests/`+`benches/`-aware discovery walk in the filesystem foundation — the default walk prunes both directories, so AES103 receives its own file set alongside the production source. `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` → 0 failures at `HEAD` (2026-10-02). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p naming-rules-lint-arwaky` after any code change to this crate

## Backlog Status

### FR-NamingRules-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | NamingRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-NamingRules-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | NamingRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

### FR-NamingRules-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | NamingRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-09-29 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

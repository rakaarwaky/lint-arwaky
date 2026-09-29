# Feature Backlog: Doc Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p lint_arwaky_doc_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p lint_arwaky_doc_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| DOC-01 | FR-DOC-001 | AES601–AES607 document invariant enforcement — 24 scenarios verified | P0 | Done | `cargo test -p lint_arwaky_doc_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |
| DOC-02 | FR-DOC-002 | Audit orchestration via single aggregate entry point — verified by `contract_doc_rules.rs` and `acceptance_FR_DOC_004.rs` tests | P0 | Done | `cargo test -p lint_arwaky_doc_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Bare FR-ID without feature prefix | AES601 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD requirement missing required field | AES601 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| API Contract missing Aggregate API subsection | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Integration Points not a markdown table | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| NFR section not a markdown table | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD sections out of template order | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Test Scenarios has no bullet items | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Glossary has no bullet items | AES602 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Spec names a source file (`.rs`) | AES603 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Spec carries status leak | AES603 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD omits BACKLOG.md crosslink | AES604 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD omits PRD.md crosslink | AES604 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Feature backlog restates root state section | AES604 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md has no H1 | AES606 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md has multiple H1s | AES606 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md missing required H2 | AES606 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md has off-template H2 | AES606 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD has 3 requirements, contract has 2 classes | AES607 CRITICAL | Automated | `tests/acceptance_FR_DOC_004.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD has 1 requirement, contract has 3 classes | AES607 CRITICAL | Automated | `tests/acceptance_FR_DOC_004.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Aggregate trait excluded from seam count | AES607 accepts | Automated | `tests/acceptance_FR_DOC_004.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Fenced code block `#` comment not counted as H1 | No violation | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Conforming workspace reports 0 doc findings on `docs .` | No violation | Automated | Self-lint | `lint-arwaky-cli docs .` | `926bd34a` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p lint_arwaky_doc_rules --lib --tests` → 0 failures at `926bd34a` (2026-09-29) |
| Scenario evidence | Done | 24 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Aligned BACKLOG.md and FRD.md to HOW-TO templates (AES606 H2 contracts); expanded scenario evidence table | @raka |

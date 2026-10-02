# Feature Backlog: Doc Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run -p doc-rules-lint-arwaky` → 72 passed, 0 failed. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p doc-rules-lint-arwaky` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| DOC-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| DOC-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| DOC-03 | P0 | Done | On Track | None | — | 2026-10-02  |

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
| AGENTS.md has no H1 | AES605 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md has multiple H1s | AES605 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md missing required H2 | AES605 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| AGENTS.md has off-template H2 | AES605 CRITICAL | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD has 3 requirements, contract has 2 classes | AES601 CRITICAL | Automated | `tests/acceptance_FR_DOC_001.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| FRD has 1 requirement, contract has 3 classes | AES601 CRITICAL | Automated | `tests/acceptance_FR_DOC_001.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Aggregate trait excluded from seam count | AES601 accepts | Automated | `tests/acceptance_FR_DOC_001.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| Fenced code block `#` comment not counted as H1 | No violation | Automated | `tests/contract_doc_rules.rs` | `cargo test -p lint_arwaky_doc_rules` | `926bd34a` |
| A capability seam answers a source-file reference and the other four stay silent | AES603 seam isolation | Automated | `tests/contract_doc_capability_seams.rs` | `cargo nextest run -p doc-rules-lint-arwaky` | working-tree |
| A capability seam answers a missing H2 and the other four stay silent | AES605 seam isolation | Automated | `tests/contract_doc_capability_seams.rs` | `cargo nextest run -p doc-rules-lint-arwaky` | working-tree |
| A backlog reporting progress raises no spec-purity finding | AES603 document scoping | Automated | `tests/contract_doc_capability_seams.rs` | `cargo nextest run -p doc-rules-lint-arwaky` | working-tree |
| Five seams over a conforming workspace report nothing | No violation | Automated | `tests/contract_doc_capability_seams.rs` | `cargo nextest run -p doc-rules-lint-arwaky` | working-tree |
| A document violating two rules reports both rule codes | Fan-out completeness | Automated | `tests/contract_doc_capability_seams.rs` | `cargo nextest run -p doc-rules-lint-arwaky` | working-tree |
| Conforming workspace reports 0 doc findings on `docs .` | No violation | Automated | Self-lint | `lint-arwaky-cli docs .` | `926bd34a` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p doc-rules-lint-arwaky` → 72 passed, 0 failed |
| Scenario evidence | Done | 29 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-10-02 | Split the 876-line invariant auditor into five capability seams, one per AES doc rule | @raka |
| 2026-09-29 | Aligned BACKLOG.md and FRD.md to HOW-TO templates (AES605 H2 contracts); expanded scenario evidence table; merged AES607 into AES601; removed AES605 | @raka |

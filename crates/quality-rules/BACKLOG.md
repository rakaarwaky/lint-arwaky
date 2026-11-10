# Feature Backlog: Quality Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-10

## Progress Overview

- Done: `CARGO_INCREMENTAL=0 cargo nextest run -p quality-rules-lint-arwaky --lib --tests -j 2` → 149 passed, 0 skipped (2026-11-10). Includes the new empty-file, duplication-cardinality, Python-vocabulary, AES305-message, and AES304-kind-naming acceptance tests.
- In Progress: None
- Blocked: None
- Next Action: Re-run `CARGO_INCREMENTAL=0 cargo nextest run -p quality-rules-lint-arwaky --lib --tests -j 2` after any code change to this crate

## Backlog Status

### FR-QualityRules-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | QualityRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

### FR-QualityRules-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | QualityRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

### FR-QualityRules-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | QualityRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

### FR-QualityRules-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | QualityRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

### FR-QualityRules-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | QualityRules |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

### FR-QualityRules-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Ready |
| Done | Per-rule `enabled` flags honoured by the orchestrator (`RuleEnabled`); `skipped_names()` names the rules that did not run |
| In Progress | None |
| Risk | None |
| Updated | 2026-11-10 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

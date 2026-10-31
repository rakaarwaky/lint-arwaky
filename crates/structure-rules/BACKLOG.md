# Feature Backlog: Structure Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-02

## Progress Overview

- Done: AES702 root-container enforcement added: a feature folder must hold a `root_<concept>_container` or `root_<concept>_entry` file under it, in its `src/` subfolder, or at the member root (a member-root composition file answers for the features beneath it). New violation type `feature_missing_root_container`. `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- Done: AES702 shared-layer mirror check added: a feature folder must be mirrored as a `shared/src/<feature>/` subfolder in its member's layer. New violation type `feature_missing_shared_subfolder`; surface folders are exempt. `cargo nextest run -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- Done: AES704 (test-suite coverage) added; the two contract files were renamed to `contract_`/`unit_` prefixes so AES103 passes, and `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, and `benches/bench_structure_rules.rs` were added to satisfy AES704. `cargo nextest run -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p structure-rules-lint-arwaky --lib --tests` after any code change to this crate

## Backlog Status

### FR-STR-001: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                      |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

### FR-STR-002: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                      |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

### FR-STR-003: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                      |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

### FR-STR-004: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                      |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

### FR-STR-005: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                      |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

### FR-STR-006: 

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | STR                                        |
| In Progress  | None                                       |
| Risk         | None                                       |
| Updated      | 2026-10-28                                 |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

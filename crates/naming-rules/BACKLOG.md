# Feature Backlog: Naming Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-02

## Progress Overview

- Done: **AES103** (test/bench file-prefix discipline) added as this crate's third seam, backed by `capabilities_test_file_prefix_checker.rs`, the `ITestFilePrefixProtocol` contract, and a `tests/`+`benches/`-aware discovery walk in the filesystem foundation — the default walk prunes both directories, so AES103 receives its own file set alongside the production source. `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` → 0 failures at `HEAD` (2026-10-02). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p naming-rules-lint-arwaky` after any code change to this crate

## Backlog Status

### {FR-NamingRules-001}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-NamingRules-001                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-NamingRules-002}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-NamingRules-002                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-NamingRules-003}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-NamingRules-003                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

## Reference

| Prd | {Link to PRD.md} | [{link}](PRD.md) |
| Frd | {Link to FRD.md} | [{link}](FRD.md) |
| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md) |

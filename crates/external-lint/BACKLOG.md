# Feature Backlog: External Lint

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p external-lint-lint-arwaky --lib --tests` → 10 passed at `cc63389a` (2026-09-29). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p external-lint-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| EXTE-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| EXTE-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| EXTE-03 | P0 | Done | On Track | None | — | 2026-09-29  |
| EXTE-04 | P0 | Done | On Track | None | — | 2026-09-29  |
| EXTE-05 | P0 | Done | On Track | None | — | 2026-09-29  |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| Rust-only project | Only clippy, rustfmt, cargo-audit run | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Python-only project | Only ruff, mypy, bandit run | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| JS-only project | Only eslint, prettier, tsc run | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Multi-language project | All 10 adapters run | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Empty directory | No adapters run, empty result list | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Single .rs file path | Only Rust adapters run | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Adapter binary not installed | Warning printed, other adapters continue | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Adapter produces JSON output | Correctly parsed into LintResult | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Adapter produces empty output | Empty result list | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| All adapters fail | Returns empty result list with warnings | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| One adapter fails | Other adapters still run | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Timeout exceeded | Adapter returns error, others continue | Automated | `tests/e2e_external_lint_flow.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Sequential execution | Adapters run one after another | Automated | `tests/integration_external_lint.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| ESLint fix | `eslint --fix` executed | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Prettier fix | `prettier --write` executed | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Ruff fix | `ruff check --fix` executed | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Clippy fix | `cargo clippy --fix` executed | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Rustfmt fix | `cargo fmt` executed | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| TSC/MyPy/Bandit/audit fix | No-op (no auto-fix capability) | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Clippy `correctness` lint | Severity CRITICAL, code `clippy::<name>` | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Clippy `style` lint | Severity MEDIUM | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Ruff `E501` (line too long) | Severity LOW, code `ruff::E501` | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Ruff `S105` (hardcoded password) | Severity CRITICAL, code `ruff::S105` | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| ESLint severity 2 (error) | Severity HIGH, code `eslint::<rule>` | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| cargo-audit critical vulnerability | Severity CRITICAL, code `cargo-audit::RUSTSEC-*` | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Tool produces invalid JSON | Empty results, warning logged | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Relative file path in tool output | Canonicalized to absolute path | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| JS tool found in node_modules/.bin | Local binary used | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| JS tool not found locally | Global PATH fallback used | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| JS tool not found anywhere | Error at execution | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| Cargo.toml found in parent directory | Cargo tools use that directory | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| No Cargo.toml in hierarchy | Adapter skipped with warning | Automated | `tests/unit_external_lint_*_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |
| markdownlint fix falls back to `markdownlint-cli2` | `--fix` runs against the first resolvable variant | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | `fix_falls_back_to_cli2_when_only_that_variant_is_installed` | `ec7e16a7` |
| markdownlint fix with no CLI installed | No-op status, no command spawned | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | `fix_is_a_noop_status_when_no_cli_is_installed` | `ec7e16a7` |
| markdownlint `MD041` | Severity MEDIUM, code `markdownlint::MD041` | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | `cargo test -p external-lint-lint-arwaky` | `cc63389a` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p external-lint-lint-arwaky --lib --tests` → 10 passed at `cc63389a` (2026-09-29) |
| Scenario evidence | Done | 31 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
| 2026-09-29 | Restructured: moved utility protocols (CommandExecutor, JsToolResolution, CargoDir) out of capability FRs. Added EXTE-05 for FR-ExternalLint-005 (Normalization). All capability FRs now map 1:1 to protocol traits. | @raka |

# Feature Backlog: Quality Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p quality_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| QUAL-01 | FR-QUALITYRULES-001 | AES302 - Minimum File Line Count — 5 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-02 | FR-QUALITYRULES-002 | AES303 - Mandatory Definitions & Dead Inheritance — 13 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-03 | FR-QUALITYRULES-003 | AES304 - Bypass Detection — 20 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-04 | FR-QUALITYRULES-004 | AES305 - Duplicate Code Detection — 6 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-05 | FR-QUALITYRULES-005 | Configuration — 5 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| File with 1,500 lines, max = 1,000 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with exactly 1,000 lines, max = 1,000 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with 999 lines, max = 1,000 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Barrel file or module entry point with 2,000 lines | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list with 2,000 lines | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with 500 lines of comments + 500 lines of code | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with 3 lines, min = 10 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with exactly 10 lines, min = 10 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with 15 lines, min = 10 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| The package entry point with 1 line | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with only comments (5 lines) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `pub struct Foo { ... }` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with only `use` statements, no struct/enum/trait/type | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with `class Foo:` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with only imports | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with `export interface IFoo { ... }` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `struct Foo;` and no `impl` block | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `struct Foo;` followed by `impl Foo { ... }` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `struct Foo(i32)` (tuple struct) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with `class Foo: pass` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with `class Foo {}` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| The typed constant source file with no definitions | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| `#[cfg(test)]` module with `struct TestFoo;` and no impl | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `foo.unwrap()` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `foo.expect("msg")` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `panic!("error")` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `todo!()` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `#[allow(unused)]` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `foo.unwrap_or_default()` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `foo.unwrap_or(42)` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `let s = "unwrap()"` (string literal) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with `# type: ignore` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with `# noqa` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with `raise NotImplementedError` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with `// @ts-ignore` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with `// @ts-expect-error` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with `// FIXME: refactor this` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with `// HACK: temporary workaround` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with `// TODO: implement later` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `unwrap()` inside `#[cfg(test)]` module | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Cargo.toml with `level = "allow"` under `[lints.clippy]` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with `print!("unwrap()")` (string literal) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Two files with 80% identical code blocks | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Two files with 30% overlap, threshold = 50% | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File shorter than `min_lines` | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Single file in workspace | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Three files all identical | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with only whitespace lines (very short after normalization) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rule AES301 disabled in config | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rule AES304 disabled in config | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Custom `max_lines = 500` in config | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Custom bypass patterns in config | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 55 scenarios mapped; all Automated via `cargo test -p quality_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |

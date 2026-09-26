# Feature Backlog: Quality Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p quality_rules --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| QUAL-01 | FR-001 | AES302 - Minimum File Line Count — 5 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-02 | FR-002 | AES303 - Mandatory Definitions & Dead Inheritance — 13 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-03 | FR-003 | AES304 - Bypass Detection — 20 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-04 | FR-004 | AES305 - Duplicate Code Detection — 6 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| QUAL-05 | FR-005 | Configuration — 5 scenarios verified | P0 | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| File with 3 lines, min = 10 | AES302 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with exactly 10 lines, min = 10 | No violation (strict`<`) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with 15 lines, min = 10 | No violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| `__init__.py` with 1 line | No violation — exception | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with only comments (5 lines) | AES302 violation (comments count) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`pub struct Foo { ... }` | No violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with only`use` statements, no struct/enum/trait/type | AES30… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with`class Foo:` | No violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with only imports | AES303 — MissingDefinition | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with`export interface IFoo { ... }` | No violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`struct Foo;` and no `impl` block | AES303 — DeadInherit… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`struct Foo;` followed by `impl Foo { ... }` | No violat… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`struct Foo(i32)` (tuple struct) | No violation (not uni… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with`class Foo: pass` | AES303 — DeadInheritance | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with`class Foo {}` | AES303 — DeadInheritance | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| `*_constant.rs` file with no definitions | No violation — skipped | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| `#[cfg(test)]` module with `struct TestFoo;` and no impl | No violatio… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | No violation — exception | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`foo.unwrap()` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`foo.expect("msg")` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`panic!("error")` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`todo!()` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`#[allow(unused)]` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`foo.unwrap_or_default()` | No violation (safe variant) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`foo.unwrap_or(42)` | No violation (safe variant) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`let s = "unwrap()"` (string literal) | No violation (in… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with`# type: ignore` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with`# noqa` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Python file with`raise NotImplementedError` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with`// @ts-ignore` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| TS file with`// @ts-expect-error` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with`// FIXME: refactor this` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with`// HACK: temporary workaround` | AES304 violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Any file with`// TODO: implement later` | No violation (TODO not in pa… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`unwrap()` inside `#[cfg(test)]` module | No violation —… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Cargo.toml with`level = "allow"` under `[lints.clippy]` | AES304 viola… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rust file with`print!("unwrap()")` (string literal) | No violation (in… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | No violation — exception | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Two files with 80% identical code blocks | AES305 violation (both file… | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Two files with 30% overlap, threshold = 50% | No violation | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File shorter than`min_lines` | No violation — skipped | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Single file in workspace | No violation (nothing to compare) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Three files all identical | AES305 violation (all three files) | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File with only whitespace lines (very short after normalization) | No … | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rule AES301 disabled in config | No AES301 violations | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Rule AES304 disabled in config | No AES304 violations | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| File in exceptions list | No violation for that file | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Custom`max_lines = 500` in config | AES301 uses 500 instead of 1000 | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |
| Custom bypass patterns in config | AES304 uses custom patterns | Automated | `tests/quality-rules/` | cargo test -p quality_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p quality_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 49 scenarios mapped; all Automated via `cargo test -p quality_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |

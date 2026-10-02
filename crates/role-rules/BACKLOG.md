# Feature Backlog: Role Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-28

## Current Condition

- Done: `cargo nextest run -p role-rules-lint-arwaky` → 222 passed, 0 failed at `72d2d58b` (2026-09-28). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo nextest run -p role-rules-lint-arwaky` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| ROLE-01 | P0 | Done | On Track | None | — | 2026-09-17  |
| ROLE-02 | P0 | Done | On Track | None | — | 2026-09-17  |
| ROLE-03 | P0 | Done | On Track | None | — | 2026-09-17  |
| ROLE-04 | P0 | Done | On Track | None | — | 2026-09-17  |
| ROLE-05 | P0 | Done | On Track | None | — | 2026-09-28  |
| ROLE-06 | P0 | Done | On Track | None | — | 2026-09-17  |
| ROLE-07 | P0 | Done | On Track | None | — | 2026-09-17  |

## Scenario Evidence

| Scenario | Rule / expected | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|---|
| Taxonomy entity file with `String` field type | AES401 violation at ex… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy entity file with custom VO field | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy entity file with `i32` field type | AES401 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy error file with `bool` parameter | AES401 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy event file with `Vec<String>` field | AES401 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy constant file with `pub const` only | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy constant file with `fn helper()` | AES401 violation (function… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy constant file with `struct Foo` | AES401 violation (struct in… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Taxonomy VO file with custom types only | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Empty taxonomy file | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract protocol with `String` in method parameter | AES402 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract protocol with custom VO in method parameter | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract protocol with `bool` return type | AES402 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract aggregate with zero methods | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract aggregate with `i64` in method signature | AES402 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Contract protocol with all VO-typed signatures | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with protocol implementor | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with no protocol implementor | AES403 — MissingProtoco… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with 4 type declarations (max=3) | AES403 — TooManyTyp… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with 3 types including helper struct | No violation (h… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with exactly 3 types, 1 implementor | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Capability file with >3 types, no implementor | AES403 — TooManyTypes … | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Rust utility file with `struct Foo` | AES404 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Rust utility file with only `fn helper()` | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Rust utility file with `enum Bar` | AES404 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Python utility file with `def helper()` | No violation (functions allo… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Python utility file with `class Foo` | AES404 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| TS utility file with `export function helper()` | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| TS utility file with `export class Foo` | AES404 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| TS utility file with `export interface IFoo` | AES404 violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Utility file with `struct` inside comment | No violation (AST ignores … | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Empty utility file | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with aggregate trait implementor | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with no aggregate implementor | AES405 — MissingAggregateIm… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with 4 type declarations (max=3) | AES405 — TooManyTypes | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with helper struct + orchestrator struct (2 types) | No vio… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with implementor + 2 helpers (3 types) | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with `Any` type annotation | AES405 — AnyTypeAnnotation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent file with `: Any` in comment | No violation (comment skipped) | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Agent with aggregate impl after inherent impl (Rust) | AES405 — BlockOrder | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with aggregate impl before inherent impl (Rust) | No violation | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with `std::fs::` call (Rust) | AES405 — ForbiddenIo | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with file-level `const` (Rust) | AES405 — ConstantPlacement | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with `vals.iter().sum::<usize>()` | AES405 — Computation | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with a body-less `fn` inside a `trait` (Rust) | AES405 — AbstractMethod | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with `&mut self` method (Rust) | AES405 — NotStateless | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with a file-level `fn` (Rust) | AES405 — FreeFunction | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Python agent with `@abstractmethod` | AES405 — AbstractMethod | Automated | `unit_role_rules_agent_python_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Python agent with `self.x = …` outside `__init__` | AES405 — NotStateless | Automated | `unit_role_rules_agent_python_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| TS agent with `this.x = …` outside the constructor | AES405 — NotStateless | Automated | `unit_role_rules_agent_ts_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with 1 injected protocol, feature declares 2+ | AES405 — SingleExecutionGoal | Automated | `unit_role_rules_agent_{rust,python,ts}_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with 2 injected protocols | No violation | Automated | `unit_role_rules_agent_{rust,python,ts}_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with 1 injected protocol, feature declares 1 | No violation (single-subsystem feature) | Automated | `unit_role_rules_agent_{rust,python,ts}_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Agent with 4 injected protocols of one type, feature declares 1 | No violation (not a single-subsystem feature) | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Python agent with `__init__` params wrapped over lines | Params counted by annotation | Automated | `unit_role_rules_agent_python_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Python agent with a defaulted `dep: IProtocol = None` param | Param counted by annotation | Automated | `unit_role_rules_agent_python_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| TS agent with a `Deps` interface of protocol fields | Interface members counted | Automated | `unit_role_rules_agent_ts_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| TS agent with wrapped `constructor(private readonly …)` | Shorthand params joined and counted | Automated | `unit_role_rules_agent_ts_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Rust agent whose last struct field has no trailing comma | Last field counted | Automated | `unit_role_rules_agent_rust_role_auditor.rs` | cargo nextest -p role-rules-lint-arwaky | `72d2d58b` |
| Feature declaring a dead protocol trait nothing references | Dead trait excluded from the count | Automated | `crates/doc-rules` self-lint → 0 violations | `lint-arwaky-cli check .` | `72d2d58b` |
| Passive surface with 51 functions (max=50) | AES406 — TooManyMethods | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Passive surface with 50 functions | No violation | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Smart surface with 100 functions | No violation (exempt) | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Smart surface with control-flow statements | No violation (exempt from… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Passive surface with 51 control-flow statements (max=50) | AES406 — Do… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Utility surface with 40 control-flow statements | No violation (below … | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Surface file with unclassifiable suffix | Treated as Passive | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Root layer file (`root_app_entry`) | Completely skipped, zero violatio… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Config `architecture.enabled: false` | Zero violations for entire scan | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Config AES401 `enabled: false` | No AES401 violations, other rules sti… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Config `ignored_paths: ["tests"]` | `tests/` directory files produce n… | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| File with no underscore (`main`) | Silently skipped | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| File with unrecognized prefix (`foobar_x_y`) | Silently skipped | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Barrel file (`mod.rs`) | Skipped | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| File in exceptions list | Skipped for that rule | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |
| Multi-language workspace: same rule across Rust, Python, TS | Correct … | Automated | `tests/role-rules/` | cargo test -p role_rules | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p role_rules --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 55 scenarios mapped; all Automated via `cargo test -p role_rules` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |

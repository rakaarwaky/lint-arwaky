---
name: testing-suite
description: Generates contract to E2E suites plus benches. Use when adding package tests, coverage, perf.
metadata:
  tags: [python, rust, typescript, testing, pytest, cargo, criterion, vitest, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  triggers:
    - "create tests"
    - "create tests python"
    - "create tests rust"
    - "add tests"
    - "add tests python"
    - "add tests rust"
    - "create test suite"
    - "create test suite python"
    - "create test suite typescript"
    - "package tests python"
    - "crate tests rust"
    - "e2e tests"
    - "e2e tests python"
    - "e2e tests rust"
    - "benchmark"
    - "benchmark python"
    - "benchmark rust"
    - "benchmark typescript"
    - "increase coverage"
  dependencies: []
  related:
    - test-driven-development
    - create-capabilities
    - create-agent
    - create-utility
---
# Testing Suite

One test layout for every layer and language: **tests go in `tests/`, benchmarks in `benches/`,
never inline in the source file.** The file-name *prefix* is the virtual folder — keep both
directories flat, with no subdirectories.

Pattern: `<type>_<subject>.<ext>` in `tests/`, `bench_<subject>.<ext>` in `benches/`.

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for that
language's directory tree, benchmark tooling/config file, and run commands.

## Rules

- **Tests** (`tests/`): flat, prefix IS the virtual folder — no real subdirectories.
- **Benchmarks** (`benches/`): separate directory, use the language benchmark library
  (`pytest-benchmark`, `criterion`, `vitest/benchmark`) — never hand-rolled timing.
- Contract tests verify the implementation exists (Python: class/protocol ABC; Rust: trait;
  TypeScript: class/interface).
- Unit tests: happy path, edge cases, error paths — one public function each.
- Integration tests: use the real DI container / entry point (module, crate, or package wiring).
- E2E tests: hit the real CLI/API/entry point and assert on real output.
- Acceptance tests: map 1:1 to a business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

## Test types

| Prefix          | Directory | Scope                     | Speed | Runs when                |
| ----------------- | ----------- | --------------------------- | ------- | -------------------------- |
| `contract_`     | tests/    | Protocol/trait/interface impl exists | ms    | Every PR                 |
| `unit_`         | tests/    | One public function         | ms    | Every PR                 |
| `integration_`  | tests/    | Module / crate / package + DI wiring | ms–s | Every PR                 |
| `smoke_`        | tests/    | App boots + responds        | <5s    | Every PR                 |
| `e2e_`          | tests/    | Full request lifecycle      | s      | Every PR (critical path) |
| `acceptance_`   | tests/    | Business requirement met    | s      | Every PR / release gate  |
| `bench_`        | benches/  | Performance regression      | s–min  | Release gate / nightly   |

## Coverage targets

| Layer          | Minimum |
| ---------------- | --------- |
| Capabilities   | 70%     |
| Agent          | 60%     |
| Utility        | 50%     |

## Verify commands

| Language   | Command                    |
| ------------ | ---------------------------- |
| Python     | `pytest --tb=short`        |
| Rust       | `cargo test --workspace`   |
| TypeScript | `npx vitest run`           |

## Workflow

```
- [ ] Step 1: Analyze module / crate / package structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write contract test
- [ ] Step 4: Write unit tests (happy, edge, error)
- [ ] Step 5: Write integration test
- [ ] Step 6: Write smoke test
- [ ] Step 7: Write e2e test
- [ ] Step 8: Write acceptance test(s) mapped to FRD/PRD IDs
- [ ] Step 9: Write bench_<subject> in benches/ (Rust: also register [[bench]] in Cargo.toml)
- [ ] Step 10: Run the language verify command
- [ ] Step 11: Verify coverage targets met
```

## Checklist

- [ ] No inline tests in source files; everything lives under `tests/` or `benches/`.
- [ ] File names use the type prefix and stay flat (no subdirectories).
- [ ] Contract test proves the protocol/trait/interface implementation exists.
- [ ] Unit tests cover happy path, edge cases, and error paths.
- [ ] Integration test builds the real DI wiring; E2E asserts on real output.
- [ ] Acceptance tests reference requirement IDs; smoke test runs in <5s.
- [ ] Benchmarks use the proper library, not manual timing loops.
- [ ] Coverage meets 70/60/50 for capabilities/agent/utility.
- [ ] Verify command passes.

---
name: aes-testing-suite
description: "Generates contract to E2E suites plus benches. Use when adding package tests, coverage, perf."
when-to-use: "create, add, or audit test and benchmark suites for an AES module, crate, or package (Python, Rust, TypeScript); add contract/unit/integration/dogfood/smoke/e2e/acceptance/bench files; raise coverage to the layer floor."
argument-hint: '<python | rust | typescript> <module | crate | package>'
metadata:
  tags:
    - python
    - rust
    - typescript
    - testing
    - pytest
    - cargo
    - criterion
    - vitest
    - contract
    - unit
    - integration
    - e2e
    - acceptance
    - smoke
    - benchmark
  related_skills:
    - aes-capabilities
    - aes-agent
    - aes-utility
    - aes-lint-arwaky
---

# aes-testing-suite

Give every layer one test layout: tests live in `tests/` with a flat type prefix (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`), benchmarks live in `benches/` with a `bench_` prefix — never inline in `src/`; the file-name prefix is the virtual folder. The audience is the agent creating or validating a module/crate/package test suite; the **test type** decides the prefix, the scope, and when it runs. All seven test types plus one benchmark are required; none are optional. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Placement: tests in `tests/`, benchmarks in `benches/` — never inline in `src/`.
- Naming: flat prefix `<type>_<subject>.<ext>` / `bench_<subject>.<ext>`, no subdirectories — the prefix is the virtual folder (AES103).
- Coverage of types: every one of the seven test types plus one benchmark is owed; support prefixes (`regression_`, `behavioral_`, `mock_`, `fixture_`) are legal but satisfy no category (AES704).
- Contract first: `contract_` proves the protocol/trait/interface implementation exists before behaviour tests.
- Real wiring: integration tests use the real DI container; e2e asserts on real CLI/API output; `dogfood_` hits a live service and auto-skips in CI when unavailable.
- Benchmarks: use the language library — `pytest-benchmark`, `criterion` (Rust also registers `[[bench]]` in `Cargo.toml`), `vitest/benchmark`; hand-rolled timing is not a benchmark.
- Traceability: `acceptance_` maps 1:1 to an FRD/PRD requirement; `smoke_` completes in under 5 seconds.
- Line coverage floor: capabilities 70%, agent 60%, utility 50%.
- Verify: language run command green + `lint-arwaky-cli scan .` → 0 for AES103/AES704.
- One fact, one home: the language HOW-TO carries run commands, config, and the directory tree — read it, do not restate here. Cite the gate, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/python-testing.md](references/python-testing.md) | Python test HOW-TO — prefixes, run commands, coverage, verify |
| [references/rust-testing.md](references/rust-testing.md) | Rust test HOW-TO — prefixes, nextest, criterion, verify |
| [references/typescript-testing.md](references/typescript-testing.md) | TypeScript test HOW-TO — prefixes, vitest, coverage, verify |
| Related skills | aes-capabilities, aes-agent, aes-utility, aes-lint-arwaky |

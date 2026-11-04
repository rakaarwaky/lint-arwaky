---
name: aes-testing-suite
description: Generates contract to E2E suites plus benches. Use when adding package tests, coverage, perf.
when-to-use: "Use when adding test or benchmark suites to an AES module, crate, or package (Python, Rust, TypeScript), raising coverage to the layer floor, or auditing the test layout."
argument-hint: "<module | crate | package>"
---

# aes-testing-suite

Give every layer one test layout: tests live in `tests/` with a flat type prefix (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`), benchmarks live in `benches/` with a `bench_` prefix — never inline in `src/`; the file-name prefix is the virtual folder. The **test type** decides the prefix, the scope, and when it runs.

## Global rules

- **Placement**: tests in `tests/`, benchmarks in `benches/` — never inline in `src/`.
- **Naming (AES103)**: flat prefix `<type>_<subject>.<ext>` / `bench_<subject>.<ext>`, no subdirectories — the prefix is the virtual folder.
- **Coverage of types (AES704)**: every one of the seven test types plus one benchmark is owed; support prefixes (`regression_`, `behavioral_`, `mock_`, `fixture_`) are legal but satisfy no category.
- **Contract first**: `contract_` proves the protocol/trait/interface implementation exists before behaviour tests.
- **Real wiring**: integration tests use the real DI container; e2e asserts on real CLI/API output; `dogfood_` hits a live service and auto-skips in CI when unavailable.
- **Benchmarks**: use the language library — `pytest-benchmark`, `criterion` (Rust also registers `[[bench]]` in `Cargo.toml`), `vitest/benchmark`; hand-rolled timing is not a benchmark.
- **Traceability**: `acceptance_` maps 1:1 to an FRD/PRD requirement; `smoke_` completes in under 5 seconds.
- **Line coverage floor**: capabilities 70%, agent 60%, utility 50%.
- **Verify**: language run command green + `lint-arwaky-cli scan .` → 0 for AES103/AES704.
- **Related**: aes-capabilities (layer under test), aes-agent, aes-utility, aes-lint-arwaky (gate) — each carries its own HOW-TO.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language HOW-TOs | [references/python-testing.md](references/python-testing.md) | Python test — prefixes, run commands, coverage, verify |
| Language HOW-TOs | [references/rust-testing.md](references/rust-testing.md) | Rust test — prefixes, nextest, criterion, verify |
| Language HOW-TOs | [references/typescript-testing.md](references/typescript-testing.md) | TypeScript test — prefixes, vitest, coverage, verify |
| Test types | [references/test-types.md](references/test-types.md) | Prefix · directory · scope · speed · runs-when matrix + coverage floors |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | First-failing-check routing for a broken test layout |
| Workflow | [references/workflow.md](references/workflow.md) | Step sequence from analyze to verify |
| Verification | [references/verification.md](references/verification.md) | Language run command + AES103/AES704 gate |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Checklist before adding a test suite |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common layout, mock, and coverage mistakes |

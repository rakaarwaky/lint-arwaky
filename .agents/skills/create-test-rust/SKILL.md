---
name: create-test-rust
description: "Generates contract, unit, integration, E2E, acceptance, and smoke test suites in tests/ (flat prefix naming), plus benchmark suites in benches/ (separate directory). Use when adding a new capability crate, increasing coverage, preparing a release, or validating performance. Triggers: create tests rust, add tests rust, create test suite rust, crate tests rust, e2e tests rust, benchmark rust."
metadata:
  tags: [rust, testing, criterion, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-python, create-test-typescript]
---

# Create Rust Test Suite

## Directory Layout

```
crates/<name>/
├── src/
│   └── capabilities_my_struct.rs   # NO inline tests. Clean.
├── tests/                          # All test types, flat prefix naming
│   ├── contract_<crate>.rs
│   ├── unit_<crate>_<module>.rs
│   ├── integration_<crate>.rs
│   ├── smoke_<app>.rs
│   ├── e2e_<flow>.rs
│   └── acceptance_<FR_id>.rs
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.rs
└── Cargo.toml                      # [[bench]] path → benches/bench_*.rs
```

## Rules

- **Tests** (`tests/`): flat, prefix IS the virtual folder — no real subdirectories.
- **Benchmarks** (`benches/`): use `criterion` — never hand-rolled timing.
- Prefix pattern: `<type>_<subject>.rs`
- Contract tests verify trait implementation.
- Unit tests: happy path, edge cases, error paths.
- Integration tests: use real DI container.
- E2E tests: hit real entry point, assert on real output.
- Acceptance tests: map 1:1 to business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

## Cargo.toml for Benchmarks

```toml
[[bench]]
name = "bench_<subject>"
path = "benches/bench_<subject>.rs"
harness = false
```

## Test Types

| Prefix | Directory | Scope | Speed | Runs when |
| --- | --- | --- | --- | --- |
| `contract_` | tests/ | Trait impl exists | ms | Every PR |
| `unit_` | tests/ | One public function | ms | Every PR |
| `integration_` | tests/ | Crate / DI wiring | ms–s | Every PR |
| `smoke_` | tests/ | App boots + responds | <5s | Every PR |
| `e2e_` | tests/ | Full request lifecycle | s | Every PR (critical path) |
| `acceptance_` | tests/ | Business requirement met | s | Every PR / release gate |
| `bench_` | benches/ | Performance regression | s–min | Release gate / nightly |

## Coverage Targets

| Layer | Minimum |
| --- | --- |
| Capabilities | 70% |
| Agent | 60% |
| Utility | 50% |

## Workflow

```
- [ ] Step 1: Analyze crate / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write tests/contract_<crate>.rs
- [ ] Step 4: Write tests/unit_<crate>_<module>.rs
- [ ] Step 5: Write tests/integration_<crate>.rs
- [ ] Step 6: Write tests/smoke_<app>.rs
- [ ] Step 7: Write tests/e2e_<flow>.rs
- [ ] Step 8: Write tests/acceptance_<FR_id>.rs
- [ ] Step 9: Write benches/bench_<subject>.rs + register in Cargo.toml
- [ ] Step 10: cargo test --workspace
- [ ] Step 11: Verify coverage targets met
```

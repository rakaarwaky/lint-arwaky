# Testing — Rust

## Directory layout

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

## Language rules

- **Benchmarks** (`benches/`): use `criterion` — never hand-rolled timing.
- Contract tests verify trait implementation.
- Integration tests: use real DI container.
- E2E tests: hit real entry point, assert on real output.

## Cargo.toml for Benchmarks

```toml
[[bench]]
name = "bench_<subject>"
path = "benches/bench_<subject>.rs"
harness = false
```

## Run commands

| Purpose    | Command                    |
| ------------ | ---------------------------- |
| Full suite | `cargo test --workspace`   |
| Benchmarks | `cargo bench`              |

Registering a benchmark requires the `[[bench]]` block above **and** workflow step 9 includes
"+ register in Cargo.toml".

## Workflow steps (Rust names)

1. Analyze crate / app structure.
2. Write `tests/contract_<crate>.rs`, `tests/unit_<crate>_<module>.rs`, `tests/integration_<crate>.rs`.
3. Write `tests/smoke_<app>.rs`, `tests/e2e_<flow>.rs`, `tests/acceptance_<FR_id>.rs`.
4. Write `benches/bench_<subject>.rs` + register in `Cargo.toml`.
5. Run `cargo test --workspace`, then verify coverage targets met.

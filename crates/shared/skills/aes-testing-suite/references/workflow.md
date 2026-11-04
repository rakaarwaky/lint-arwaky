# Workflow

Step sequence from analyze to verify. Read the language HOW-TO for the split directory tree, config file, and run commands.

1. **Analyze the layer** — identify the test types owed (AES704) and the layer's coverage floor.
2. **Resolve the test layout** — `tests/` with a flat type prefix; `benches/` with a `bench_` prefix. Run `lint-arwaky-cli scan .` to find gaps.
3. **Draft each type** — `contract_` proves the protocol/trait/interface exists; `unit_` and `integration_` cover behaviour plus real DI wiring; `smoke_`, `e2e_`, `acceptance_` assert on app output and requirement; `dogfood_` hits a live service and skips in CI.
4. **Add the benchmark** — use the language library (pytest-benchmark, criterion, vitest); register `[[bench]]` in Rust.
5. **Raise coverage** — to the layer floor (capabilities 70%, agent 60%, utility 50%).
6. **Verify** — language run command green + `lint-arwaky-cli scan .` → 0 for AES103/AES704.

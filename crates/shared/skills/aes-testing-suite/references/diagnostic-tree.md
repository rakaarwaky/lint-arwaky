# Diagnostic Tree

Route the first failing check of a broken test layout. Answer each question in order; the first "No" dictates the next action.

1. **Does every file under `tests/` carry a legal test prefix (`contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`) at the flat top level?**
   - *No* → rename to `<type>_<subject>.<ext>`; remove sanctioned subdirectories (AES103).
2. **Do all seven test types plus one benchmark exist — one file per category?**
   - *No* → add the missing `<type>_<subject>` file; a support prefix (`regression_`, `behavioral_`, `mock_`, `fixture_`) satisfies no category (AES704).
3. **Do benchmarks live in `benches/` with a `bench_` prefix, using the language library (pytest-benchmark, criterion, vitest)?**
   - *No* → move inline timing to `benches/`; Rust registers `[[bench]]` in `Cargo.toml`; hand-rolled timing is not a benchmark.
4. **Do integration tests use the real DI container, and does e2e assert on real CLI/API output?**
   - *No* → replace mocks with real wiring; `dogfood_` hits a live service and auto-skips in CI when unavailable.
5. **Is coverage at the layer floor — capabilities 70%, agent 60%, utility 50%?**
   - *No* → add the missing cases to reach the floor.
6. **Does `lint-arwaky-cli scan .` exit 0 for AES103/AES704?**
   - *No* → fix the findings, re-scan.

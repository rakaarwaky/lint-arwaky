# Common Mistakes (Anti-Patterns)

The linter covers prefixes, directories, and category coverage. These need a reader.

- **Nested subdirectories under `tests/`** — the flat type prefix is the virtual folder; any subdirectory trips AES103.
- **A support prefix standing in for a category** — `regression_`, `behavioral_`, `mock_`, and `fixture_` are legal but satisfy no AES704 category; the folder still owes all seven types.
- **Inline timing in `src/`** — a benchmark belongs in `benches/` with a `bench_` prefix; hand-rolled timing is not a benchmark.
- **Mocking the DI container in integration tests** — integration proves the real wiring; mock only the external service.
- **`e2e_` asserting on internal state** — e2e asserts on real CLI/API output, not private fields.
- **`dogfood_` running in CI** — it hits a live service and must auto-skip when unavailable.
- **Coverage below the layer floor** — capabilities 70%, agent 60%, utility 50%; add the missing cases.
- **Restating language HOW-TO rules in SKILL.md** — delegate; the body only routes.

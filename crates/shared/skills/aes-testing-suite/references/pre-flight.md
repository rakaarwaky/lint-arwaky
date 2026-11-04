# Pre-flight Checklist

Run before adding or changing a test suite.

- [ ] The layer's owed test types are listed (AES704) and the coverage floor is known.
- [ ] Test files use a flat `<type>_<subject>` prefix at the top of `tests/` — no subdirectories (AES103).
- [ ] Benchmarks live in `benches/` with a `bench_` prefix and use the language library; Rust registers `[[bench]]` in `Cargo.toml`.
- [ ] Integration tests use the real DI container; e2e asserts on real CLI/API output.
- [ ] `dogfood_` auto-skips in CI when the live service is unavailable.
- [ ] Coverage reaches the layer floor — capabilities 70%, agent 60%, utility 50%.
- [ ] `lint-arwaky-cli scan .` exits 0 for AES103/AES704.
- [ ] The language run command is green.

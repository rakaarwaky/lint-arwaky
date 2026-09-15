# Testing — Python

## Directory layout

```
modules/<name>/
├── src/
│   └── capabilities_my_class.py    # NO inline tests. Clean.
├── tests/                          # All test types, flat prefix naming
│   ├── contract_<module>.py
│   ├── unit_<module>_<subject>.py
│   ├── integration_<module>.py
│   ├── smoke_<app>.py
│   ├── e2e_<flow>.py
│   └── acceptance_<FRD_ID>.py
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.py
└── pyproject.toml
```

## Language rules

- **Benchmarks** (`benches/`): use `pytest-benchmark` — never hand-rolled timing.
- Contract tests verify class/protocol implementation exists.
- Integration tests: use real DI container / entry point.
- E2E tests: hit real CLI/API, assert on real output.

## Run commands

| Purpose     | Command                                             |
| ------------- | ----------------------------------------------------- |
| Full suite  | `pytest --tb=short`                                 |
| Benchmarks  | `pytest --benchmark-only benches/bench_<subject>.py` |
| Coverage    | `pytest --cov=<module> --cov-fail-under=<target>`   |

## Workflow steps (Python names)

1. Analyze module / app structure.
2. Write `tests/contract_<module>.py`, `tests/unit_<module>_<subject>.py`, `tests/integration_<module>.py`.
3. Write `tests/smoke_<app>.py`, `tests/e2e_<flow>.py`, `tests/acceptance_<FRD_ID>.py`.
4. Write `benches/bench_<subject>.py`.
5. Run `pytest --tb=short`, then verify coverage targets met.

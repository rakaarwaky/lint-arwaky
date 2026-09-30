# HOW TO MAKE TESTING PYTHON

> **Purpose**: Give one module a complete test suite — tests in `tests/`, benchmarks in `benches/`, never inline in the source file; the file-name prefix is the virtual folder.
>
> **Audience**: Agents and engineers scaffolding AES test suites for Python modules.
>
> **Scope**: `modules/<name>/` test and bench files — contract, unit, integration, dogfood, smoke, e2e, acceptance, and bench with flat prefix naming. **ALL 8 types are REQUIRED — none are optional.**
>
> **Location**: `modules/<name>/tests/` and `modules/<name>/benches/` — both flat, no subdirectories; source under `src/` stays test-free.
>
> **Length**: One file per subject (`<type>_<subject>.py`); coverage targets 70/60/50 for capabilities/agent/utility.

---

## Rules

### Placement rules

**Allowed:** `tests/` for every test type, `benches/` for benchmarks only.
**Forbidden:** inline tests in `src/` files; subdirectories under `tests/` — the prefix IS the virtual folder.

### Test Location Hierarchy

Tests are placed in TWO locations based on scope:

#### 1. Root Tests (`tests/` at repo root)
For **cross-module** tests that span multiple features:

```text
tests/
├── test_envfile.py           # Cross-module: tests utility used by many modules
├── test_manifest.py          # Cross-module: tests shared manifest reader
├── test_xdg.py               # Cross-module: tests shared XDG helpers
└── test_<cross_module_feature>.py
```

**Use root tests for:**
- Shared utility tests (modules/shared/)
- Cross-module integration tests
- Repository-wide contract tests
- Tests that don't belong to one specific feature

#### 2. Module Tests (`modules/<name>/tests/`)
For **feature-specific** tests scoped to one module:

```text
modules/<name>/
├── src/
│   └── capabilities_my_class.py    # NO inline tests. Clean.
├── tests/                          # ALL 8 types REQUIRED
│   ├── contract_<module>.py
│   ├── unit_<module>_<subject>.py
│   ├── integration_<module>.py
│   ├── dogfood_<pipeline>.py
│   ├── smoke_<app>.py
│   ├── e2e_<flow>.py
│   └── acceptance_<FRD_ID>.py
└── benches/
    └── bench_<subject>.py
```

**Use module tests for:**
- Module-level contract tests (protocol/interface verification)
- Unit tests for module-specific utilities
- Integration tests for module's DI wiring
- Dogfood tests for module's live service interactions
- Smoke tests for module's fast boot checks
- E2E tests for module's full workflows
- Acceptance tests mapped to module's FRD/PRD requirements
- Benchmarks for module's performance

### Test Location Hierarchy

Tests can be placed in two locations depending on scope:

1. **Root tests** (`tests/` at repo root): For cross-module integration tests, shared utilities, and repository-wide checks.
   - Pattern: `tests/test_<subject>.py`
   - Used for: Shared utility tests, cross-module integration, repository-wide contracts

2. **Module tests** (`modules/<name>/tests/`): For feature-specific tests scoped to one module.
   - Pattern: `<type>_<module>.py`
   - Used for: Module-level contract, unit, integration, dogfood, smoke, e2e, acceptance tests

### Naming rules

Pattern: `<type>_<subject>.py` in `tests/`, `bench_<subject>.py` in `benches/`.
Prefixes: `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, `bench_`.

**IMPORTANT: ALL 8 prefixes are REQUIRED — none are optional.**

### Language rules

- **Benchmarks** (`benches/`): use `pytest-benchmark` — never hand-rolled timing.
- Contract tests verify class/protocol implementation exists.
- Integration tests: use real DI container / entry point.
- E2E tests: hit real CLI/API, assert on real output.
- Acceptance tests: map 1:1 to a business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.
- **Dogfood tests: run against LIVE service/session — no mocks allowed for external deps.**

### Coverage targets


| Layer        | Minimum |
| ------------ | ------- |
| Capabilities | 30%     |
| Agent        | 50%     |
| Utility      | 70%     |

### Workflow

1. Analyze module / app structure.
2. Write `tests/contract_<module>.py`, `tests/unit_<module>_<subject>.py`, `tests/integration_<module>.py`.
3. Write `tests/dogfood_<pipeline>.py` (requires live session), then `tests/smoke_<app>.py`, `tests/e2e_<flow>.py`, `tests/acceptance_<FRD_ID>.py`.
4. Write `benches/bench_<subject>.py`.
5. Run `pytest --tb=short`, then verify coverage targets met.

**Note: ALL 8 test types are REQUIRED. Do not skip any type.**

---

## Template

### Directory layout

```text
Repo root:
├── tests/                              # ROOT: Cross-module tests
│   ├── test_envfile.py                 # Tests shared envfile utility
│   ├── test_manifest.py                # Tests shared manifest reader
│   └── test_<cross_module_feature>.py  # Other cross-module tests
│
└── modules/<name>/                     # MODULE: Feature-specific tests
    ├── src/
    │   └── capabilities_my_class.py    # NO inline tests. Clean.
    ├── tests/                          # ALL 8 types REQUIRED (mandatory)
    │   ├── contract_<module>.py        # Verify protocol/interface exists
    │   ├── unit_<module>_<subject>.py  # Test individual functions
    │   ├── integration_<module>.py     # Test real DI wiring
    │   ├── dogfood_<pipeline>.py       # Test LIVE service (no mocks)
    │   ├── smoke_<app>.py             # Fast boot check (<5s)
    │   ├── e2e_<flow>.py              # Full request lifecycle
    │   └── acceptance_<FRD_ID>.py     # Map to business requirement
    └── benches/                        # Benchmark tests only
        └── bench_<subject>.py
```

### When to use which location

| Test Type | Root `tests/` | Module `tests/` |
|-----------|---------------|-----------------|
| Cross-module utility | ✓ | |
| Shared contract | ✓ | |
| Module-specific contract | | ✓ |
| Module-specific unit | | ✓ |
| Module integration | | ✓ |
| Module dogfood | | ✓ |
| Module smoke | | ✓ |
| Module e2e | | ✓ |
| Module acceptance | | ✓ |
| Module benchmark | | ✓ |

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| No inline tests in `src/`. | Source stays clean; the suite lives under `tests/`/`benches/` only. |
| Flat prefix naming, no subdirectories under `tests/`. | The prefix IS the virtual folder — keeps discovery and CI filters uniform. |
| Contract test proves protocol/class implementation exists. | Catches unimplemented seams before behaviour tests. |
| Unit tests cover happy path, edge cases, and error paths. | The baseline every PR must carry. |
| Integration test builds the real DI wiring; e2e asserts on real output. | Proves composition, not mocks. |
| Dogfood test runs against LIVE service/session without mocks; skips when unavailable. | Real-end-to-end validation; CI-safe with skipif. |
| Acceptance tests reference FRD/PRD IDs; smoke runs in <5s. | Requirement traceability plus a fast boot gate. |
| Benchmarks use `pytest-benchmark`, not manual timing loops. | Comparable, stable numbers across runs. |
| Coverage meets 70/60/50 for capabilities/agent/utility. | Per-layer floor before merge. |
| `pytest --tb=short` passes. | The suite is green or the work is not done. |
| ALL 8 test types present per module. | No type is optional — contract, unit, integration, dogfood, smoke, e2e, acceptance, bench. |

---

## Dogfood / Integration Pipeline Tests

**Dogfood tests are REQUIRED — not optional.** They validate against LIVE services.

For tests that exercise actual CLI commands against live services/sessions:

1. **Place** in `tests/` directory (flat, no subdirectories).
2. **Name** with `dogfood_` prefix (e.g., `dogfood_pipeline.py`).
3. **Always provide skip logic** — check for required credentials/sessions before running.
4. **Structure tests first** — verify command exists without external deps.
5. **Functional tests second** — run actual pipeline with real inputs.
6. **Use fixtures** — create temporary test files, cleanup after.
7. **Never require login in CI** — skip gracefully when deps unavailable.
8. **Mark with `@pytest.mark.dogfood`** for identification.

Dogfood tests skip in CI when services unavailable — but the file MUST exist.


## Verify

```bash
pytest --tb=short
pytest --benchmark-only benches/bench_<subject>.py
pytest --cov=<module> --cov-fail-under=<target>
# Checks: ALL 8 test types present (contract/unit/integration/dogfood/smoke/e2e/acceptance/bench)
# All tests green; benchmark runs through pytest-benchmark; per-layer coverage floors 70/60/50 met.
# Manual (not machine-checked): acceptance rows map 1:1 to FRD/PRD IDs; smoke <5s;
# no inline tests left in src/; prefixes flat, no subdirectories.
# Fallback compile gate: python -c "import <module>"
```

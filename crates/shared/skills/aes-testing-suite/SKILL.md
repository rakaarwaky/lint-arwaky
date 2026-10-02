---
name: aes-testing-suite
description: Generates contract to E2E suites plus benches. Use when adding package tests, coverage, perf.
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
    - aes-capabilities
    - aes-agent
    - aes-utility
    - aes-lint-arwaky

  triggers:

    - create tests
    - create tests python
    - create tests rust
    - add tests
    - add tests python
    - add tests rust
    - create test suite
    - create test suite python
    - create test suite typescript
    - package tests python
    - crate tests rust
    - e2e tests
    - e2e tests python
    - e2e tests rust
    - benchmark
    - benchmark python
    - benchmark rust
    - benchmark typescript
    - increase coverage

---

# aes-testing-suite

> **Purpose**: Give every layer one test layout — tests in `tests/`, benchmarks in `benches/`, never inline in the source file; the file-name prefix is the virtual folder.
> **Audience**: The agent creating or validating a module/crate/package test suite.
> **Scope**: Python, Rust, and TypeScript test and bench files — contract, unit, integration, smoke, e2e, acceptance, and bench with flat prefix naming.

The **test types** decide which prefixes, which directories, and which coverage floor apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Test suite | Flat prefix naming; tests/ + benches/; no inline tests | [references/HOW-TO-MAKE-PYTHON-TESTING.md](references/HOW-TO-MAKE-PYTHON-TESTING.md) |
| Rust | Test suite | Flat prefix naming; tests/ + benches/; no inline tests | [references/HOW-TO-MAKE-RUST-TESTING.md](references/HOW-TO-MAKE-RUST-TESTING.md) |
| TypeScript | Test suite | Flat prefix naming; tests/ + benches/; no inline tests | [references/HOW-TO-MAKE-TYPESCRIPT-TESTING.md](references/HOW-TO-MAKE-TYPESCRIPT-TESTING.md) |

**The test chain:**

`contract_` (seam exists) → `unit_` / `integration_` (behaviour + wiring) → `dogfood_` (live service validation, skip in CI) → `smoke_` / `e2e_` / `acceptance_` (app + requirement) → `bench_` (nightly)

Each prefix answers one question. A test in the wrong directory or with the wrong prefix is the defect this skill exists to prevent.

---

## Invariants

Every rule is verified by the language run command (see each HOW-TO § Verify).
Layout, placement, and naming are machine-checked by **AES103** (prefix legality
and flatness) and **AES704** (category coverage), so the linter reports what a
reader would otherwise have to catch.

| Layer | Rule |
| ----- | ---- |
| Placement | Tests live in `tests/`, benchmarks in `benches/` — never inline in `src/`. |
| Naming | Flat prefix IS the virtual folder: `<type>_<subject>.<ext>` / `bench_<subject>.<ext>` — no subdirectories (**AES103**). |
| Coverage | Every one of the seven test types is required, plus one benchmark — none are optional (**AES704**). |
| Benchmarks | Use the language benchmark library (`pytest-benchmark`, `criterion`, `vitest/benchmark`) — never hand-rolled timing; Rust registers `[[bench]]`. |
| Contract | Proves the protocol/trait/interface implementation exists before behaviour tests. |
| Unit | Happy path, edge cases, error paths — one public function each. |
| Integration | Uses the real DI container / entry point (module, crate, or package wiring). |
| E2E | Hits the real CLI/API/entry point and asserts on real output. |
| Acceptance | Maps 1:1 to a business requirement (FRD/PRD ID). |
| Smoke | Completes in under 5 seconds. |
| Line coverage | Per-layer floor: capabilities 70%, agent 60%, utility 50%. |
| Verify | Language run command → green, and `lint-arwaky-cli scan .` → 0 for AES103/AES704. |

Test-type reference (prefix · directory · scope · speed · runs when):

| Prefix | Directory | Scope | Speed | Runs when |
| ------ | --------- | ----- | ----- | --------- |
| `contract_` | tests/ | Protocol/trait/interface impl exists | ms | Every PR |
| `unit_` | tests/ | One public function | ms | Every PR |
| `integration_` | tests/ | Module / crate / package + DI wiring | ms–s | Every PR |
| `dogfood_` | tests/ | CLI against live service/session; skip if unavailable | s–min | Local only; skip CI |
| `smoke_` | tests/ | App boots + responds | <5s | Every PR |
| `e2e_` | tests/ | Full request lifecycle | s | Every PR (critical path) |
| `acceptance_` | tests/ | Business requirement met | s | Every PR / release gate |
| `bench_` | benches/ | Performance regression | s–min | Release gate / nightly |

Coverage targets: capabilities 70% · agent 60% · utility 50%.

### Enforcement

Both halves of this layout are machine-checked. Run the linter rather than
eyeballing a tree.

| Code  | Crate            | Question it answers                                                              |
| ----- | ---------------- | -------------------------------------------------------------------------------- |
| AES103 | `naming-rules`    | Is each file's prefix legal, and does either directory nest?                     |
| AES704 | `structure-rules` | Is every category present — one file per type, plus the benchmark?               |

```bash
lint-arwaky-cli scan .          # both fire on a non-conforming layout
```

Two consequences worth knowing before you name a file:

- **Support prefixes are legal but satisfy no category.** `regression_`,
  `behavioral_`, `mock_`, and `fixture_` pass AES103 because the repository
  already follows those conventions (TEST.md §2.0 requires `regression_`), but a
  folder carrying only regression guards still owes all seven test types under
  AES704.
- **`tests/common/` is the one sanctioned subdirectory.** Rust reaches shared
  support code through `mod common;`, so a support module cannot sit flat beside
  the test files without colliding with them. Every other subdirectory is an
  AES103 violation — the prefix is the virtual folder.

Split directory tree, config file, and run commands: **read the language HOW-TO** — do not restate them here.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Are tests out of `src/` and under `tests/` / `benches/`?**
   - *No* → move them; inline tests are the first defect to clear.
2. **Does every test type have a file, and `benches/` a benchmark?**
   - *No* → run `lint-arwaky-cli scan .`; AES704 names each missing category.
3. **Are file names flat `<type>_<subject>` prefixes with no subdirectories?**
   - *No* → rename; the prefix is the virtual folder (AES103).
4. **Does a contract test prove the seam exists?**
   - *No* → write `contract_*` first, then behaviour tests.
5. **Do integration/e2e tests use real wiring and real output (not mocks)?**
   - *No* → rebuild against the real DI container / entry point.
6. **Do benches use the benchmark library (Rust: `[[bench]]` registered)?**
   - *No* → switch; hand-rolled timing is not a benchmark.
7. **Does the language run command pass and coverage meet 70/60/50?**
   - *No* → add missing cases, re-run.

---

## Workflow

1. Analyze module / crate / package structure and identify untested public API.
2. Write `contract_` (seam), then `unit_` (happy / edge / error), then `integration_` (real DI).
3. Write `dogfood_` (live service; skip gracefully in CI), then `smoke_`, `e2e_`, and `acceptance_` mapped to FRD/PRD IDs.
4. Write `bench_<subject>` in `benches/` (Rust: also register `[[bench]]` in `Cargo.toml`).
5. Run the language verify command, then confirm coverage targets met.

---

## Verification

### Machine Checks

```bash
pytest --tb=short                                   # Python
cargo nextest run --workspace --lib --tests -j 2     # Rust
npx vitest run                                      # TypeScript

lint-arwaky-cli scan .        # AES103 (prefix) + AES704 (coverage) → 0

# Plus per HOW-TO: pytest-benchmark / cargo bench / vitest bench, and coverage (Python/TS).

```text

A pass means every suite is green and every test category is present.

### Human Checks

A machine pass does not mean the suite is right. Real-vs-mock wiring, FRD/PRD traceability, and the smoke time budget still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] Language run command exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] No inline tests in `src/`; everything under `tests/` or `benches/`.
- [ ] Flat prefix naming; no subdirectories except `tests/common/` (**AES103**).
- [ ] All seven test types present, plus one `bench_` file (**AES704**).
- [ ] Contract test proves the protocol/trait/interface implementation exists.
- [ ] Acceptance tests reference FRD/PRD IDs; smoke runs in <5s.
- [ ] Benchmarks use the proper library (Rust: `[[bench]]` registered).
- [ ] Line coverage meets 70/60/50 for capabilities/agent/utility.
- [ ] `lint-arwaky-cli scan .` reports 0 AES103 and 0 AES704.
- [ ] Related skills considered for the layer under test.

---

## Common Mistakes (Anti-Patterns)

The runner and the linter cover green/red, flatness, and coverage. These still
need a reader (HOW-TO § Rules):

- **Inline tests in `src/`**: move to `tests/`; source stays clean.
- **Subdirectories under `tests/`** other than `common/`: the prefix IS the folder — keep it flat (**AES103**).
- **Hand-rolled timing loops**: use `pytest-benchmark` / `criterion` / `vitest/benchmark`.
- **Mocked integration tests**: integration uses the real DI wiring; e2e asserts on real output.
- **Dogfood tests without skip logic**: must auto-skip in CI when service unavailable; never break CI.
- **Acceptance tests without FRD/PRD IDs**: requirement traceability is the point.
- **Counting a `regression_` guard as a category**: it satisfies AES103, not AES704 — all seven types are still owed (**AES704**).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-capabilities`
- `aes-agent`
- `aes-utility`
- `aes-lint-arwaky`


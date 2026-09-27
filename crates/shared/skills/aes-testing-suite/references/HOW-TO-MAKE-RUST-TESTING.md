# HOW TO MAKE TESTING RUST

> **Purpose**: Give one crate a complete test suite — tests in `tests/`, benchmarks in `benches/`, never inline in `src/`; the file-name prefix is the virtual folder.
>
> **Audience**: Agents and engineers scaffolding AES test suites for Rust crates.
>
> **Scope**: `crates/<name>/` test and bench files — contract, unit, integration, smoke, e2e, acceptance, and bench with flat prefix naming.
>
> **Location**: `crates/<name>/tests/` and `crates/<name>/benches/` — both flat, no subdirectories; source under `src/` stays test-free.
>
> **Length**: One file per subject (`<type>_<subject>.rs`); coverage targets 70/60/50 for capabilities/agent/utility.

---

## Rules

### Placement rules

**Allowed:** `tests/` for every test type, `benches/` for benchmarks only.
**Forbidden:** inline `#[cfg(test)]` suites in `src/` files; subdirectories under `tests/` — the prefix IS the virtual folder.

### Test Location Hierarchy

Tests are placed in TWO locations based on scope:

#### 1. Root Tests (`tests/` at repo root)
For **cross-module** tests that span multiple features:

```text
tests/
├── test_shared_utils.rs          # Cross-module: tests utility used by many crates
├── test_manifest.rs              # Cross-module: tests shared manifest reader
└── test_<cross_module_feature>.rs
```

**Use root tests for:**
- Shared utility tests (crates/shared/)
- Cross-module integration tests
- Repository-wide contract tests
- Tests that don't belong to one specific crate

#### 2. Crate Tests (`crates/<name>/tests/`)
For **feature-specific** tests scoped to one crate:

```text
crates/<name>/
├── src/
│   └── capabilities_my_struct.rs   # NO inline tests. Clean.
├── tests/                          # ALL 8 types REQUIRED
│   ├── contract_<crate>.rs
│   ├── unit_<crate>_<module>.rs
│   ├── integration_<crate>.rs
│   ├── dogfood_<pipeline>.rs
│   ├── smoke_<app>.rs
│   ├── e2e_<flow>.rs
│   └── acceptance_<FR_id>.rs
└── benches/
    └── bench_<subject>.rs
```

**Use crate tests for:**
- Crate-level contract tests (trait implementation verification)
- Unit tests for crate-specific utilities
- Integration tests for crate's DI wiring
- Dogfood tests for crate's live service interactions
- Smoke tests for crate's fast boot checks
- E2E tests for crate's full workflows
- Acceptance tests mapped to crate's FRD/PRD requirements
- Benchmarks for crate's performance

### Naming rules

Pattern: `<type>_<subject>.rs` in `tests/`, `bench_<subject>.rs` in `benches/`.
Prefixes: `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, `bench_`.

### Language rules

- **Benchmarks** (`benches/`): use `criterion` — never hand-rolled timing.
- Contract tests verify trait implementation.
- Integration tests: use real DI container.
- E2E tests: hit real entry point, assert on real output.
- Acceptance tests: map 1:1 to a business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

### Coverage targets

| Layer | Minimum |
| ----- | ------- |
| Capabilities | 70% |
| Agent | 60% |
| Utility | 50% |

### Workflow

1. Analyze crate / app structure.
2. Write `tests/contract_<crate>.rs`, `tests/unit_<crate>_<module>.rs`, `tests/integration_<crate>.rs`.
3. Write `tests/dogfood_<pipeline>.rs` (requires live session), then `tests/smoke_<app>.rs`, `tests/e2e_<flow>.rs`, `tests/acceptance_<FR_id>.rs`.
4. Write `benches/bench_<subject>.rs` + register in `Cargo.toml`.
5. Run `cargo test --workspace`, then verify coverage targets met.

---

## Template

### Directory layout

```text
Repo root:
├── tests/                              # ROOT: Cross-crate tests
│   ├── test_shared_utils.rs            # Tests shared utilities
│   ├── test_manifest.rs                # Tests shared manifest
│   └── test_<cross_crate_feature>.rs   # Other cross-crate tests
│
└── crates/<name>/                      # CRATE: Feature-specific tests
    ├── src/
    │   └── capabilities_my_struct.rs   # NO inline tests. Clean.
    ├── tests/                          # ALL 8 types REQUIRED (mandatory)
    │   ├── contract_<crate>.rs
    │   ├── unit_<crate>_<module>.rs
    │   ├── integration_<crate>.rs
    │   ├── dogfood_<pipeline>.rs
    │   ├── smoke_<app>.rs
    │   ├── e2e_<flow>.rs
    │   └── acceptance_<FR_id>.rs
    ├── benches/
    │   └── bench_<subject>.rs
    └── Cargo.toml
```

### When to use which location

| Test Type | Root `tests/` | Crate `tests/` |
|-----------|---------------|----------------|
| Cross-crate utility | ✓ | |
| Shared contract | ✓ | |
| Crate-specific contract | | ✓ |
| Crate-specific unit | | ✓ |
| Crate integration | | ✓ |
| Crate dogfood | | ✓ |
| Crate smoke | | ✓ |
| Crate e2e | | ✓ |
| Crate acceptance | | ✓ |
| Crate benchmark | | ✓ |

### Cargo.toml for benchmarks

```toml
[[bench]]
name = "bench_<subject>"
path = "benches/bench_<subject>.rs"
harness = false
```

Registering a benchmark requires the `[[bench]]` block above **and** workflow step 4 includes
"+ register in Cargo.toml".

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| No inline tests in `src/`. | Source stays clean; the suite lives under `tests/`/`benches/` only. |
| Flat prefix naming, no subdirectories under `tests/`. | The prefix IS the virtual folder — keeps discovery and CI filters uniform. |
| Contract test proves trait implementation exists. | Catches unimplemented seams before behaviour tests. |
| Unit tests cover happy path, edge cases, and error paths. | The baseline every PR must carry. |
| Integration test builds the real DI wiring; e2e asserts on real output. | Proves composition, not mocks. |
| Dogfood test runs against LIVE service/session without mocks; skips when unavailable. | Real-end-to-end validation; CI-safe with skipif. |
| Acceptance tests reference FRD/PRD IDs; smoke runs in <5s. | Requirement traceability plus a fast boot gate. |
| Benchmarks use `criterion` + `[[bench]]` registered in `Cargo.toml`. | Comparable, stable numbers; `cargo bench` discovers the target. |
| Coverage meets 70/60/50 for capabilities/agent/utility. | Per-layer floor before merge. |
| `cargo test --workspace` passes. | The suite is green or the work is not done. |
| ALL 8 test types present per crate. | No type is optional — contract, unit, integration, dogfood, smoke, e2e, acceptance, bench. |

---

## Dogfood / Integration Pipeline Tests

**Dogfood tests are REQUIRED — not optional.** They validate against LIVE services.

For tests that exercise actual CLI commands against live services/sessions:

1. **Place** in `tests/` directory (flat, no subdirectories).
2. **Name** with `dogfood_` prefix (e.g., `dogfood_pipeline.rs`).
3. **Always provide skip logic** — check for required credentials/sessions before running.
4. **Structure tests first** — verify command exists without external deps.
5. **Functional tests second** — run actual pipeline with real inputs.
6. **Use fixtures** — create temporary test files, cleanup after.
7. **Never require login in CI** — skip gracefully when deps unavailable.
8. **Mark with `#[cfg(feature = "dogfood")]`** for identification.

Dogfood tests skip in CI when services unavailable — but the file MUST exist.


## Verify

```bash
cargo test --workspace
cargo bench
# Checks: ALL 8 test types present (contract/unit/integration/dogfood/smoke/e2e/acceptance/bench)
# All tests green; criterion benchmarks discovered through [[bench]] registration;
# per-layer coverage floors 70/60/50 met.
# Manual (not machine-checked): acceptance rows map 1:1 to FRD/PRD IDs; smoke <5s;
# no inline tests left in src/; prefixes flat, no subdirectories; coverage 70/60/50.
# Fallback compile gate: cargo check -p <crate-name>
```

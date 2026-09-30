# HOW TO MAKE TESTING TYPESCRIPT

> **Purpose**: Give one package a complete test suite — tests in `tests/`, benchmarks in `benches/`, never inline in the source file; the file-name prefix is the virtual folder.
>
> **Audience**: Agents and engineers scaffolding AES test suites for TypeScript packages.
>
> **Scope**: `packages/<name>/` test and bench files — contract, unit, integration, smoke, e2e, acceptance, and bench with flat prefix naming.
>
> **Location**: `packages/<name>/tests/` and `packages/<name>/benches/` — both flat, no subdirectories; source under `src/` stays test-free.
>
> **Length**: One file per subject (`<type>_<subject>.ts`); coverage targets 70/60/50 for capabilities/agent/utility.

---

## Rules

### Placement rules

**Allowed:** `tests/` for every test type, `benches/` for benchmarks only.
**Forbidden:** inline tests in `src/` files; subdirectories under `tests/` — the prefix IS the virtual folder.

### Test Location Hierarchy

Tests are placed in TWO locations based on scope:

#### 1. Root Tests (`tests/` at repo root)

For **cross-package** tests that span multiple packages:

```text
tests/
├── test_shared_utils.ts          # Cross-package: tests utility used by many packages
├── test_manifest.ts              # Cross-package: tests shared manifest reader
└── test_<cross_package_feature>.ts
```

**Use root tests for:**

- Shared utility tests (packages/shared/)
- Cross-package integration tests
- Repository-wide contract tests
- Tests that don't belong to one specific package

#### 2. Package Tests (`packages/<name>/tests/`)

For **feature-specific** tests scoped to one package:

```text
packages/<name>/
├── src/
│   └── capabilities_my_class.ts    # NO inline tests. Clean.
├── tests/                          # ALL 8 types REQUIRED
│   ├── contract_<package>.ts
│   ├── unit_<package>_<module>.ts
│   ├── integration_<package>.ts
│   ├── dogfood_<pipeline>.ts
│   ├── smoke_<app>.ts
│   ├── e2e_<flow>.ts
│   └── acceptance_<FRD_ID>.ts
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.ts
└── vitest.config.ts
```

**Use package tests for:**

- Package-level contract tests (class/interface verification)
- Unit tests for package-specific utilities
- Integration tests for package's DI wiring
- Dogfood tests for package's live service interactions
- Smoke tests for package's fast boot checks
- E2E tests for package's full workflows
- Acceptance tests mapped to package's FRD/PRD requirements
- Benchmarks for package's performance

### Naming rules

Pattern: `<type>_<subject>.ts` in `tests/`, `bench_<subject>.ts` in `benches/`.
Prefixes: `contract_`, `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, `bench_`.

### Language rules

- **Benchmarks** (`benches/`): use `vitest/benchmark` — never hand-rolled timing.
- Contract tests verify class/interface implementation.
- Integration tests: use real DI container / entry point.
- E2E tests: hit real API/CLI, assert on real output.
- Acceptance tests: map 1:1 to a business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

### Coverage targets


| Layer        | Minimum |
| ------------ | ------- |
| Capabilities | 30%     |
| Agent        | 50%     |
| Utility      | 70%     |


### Workflow

1. Analyze package / app structure.
2. Write `tests/contract_<package>.ts`, `tests/unit_<package>_<module>.ts`, `tests/integration_<package>.ts`.
3. Write `tests/dogfood_<pipeline>.ts` (requires live session), then `tests/smoke_<app>.ts`, `tests/e2e_<flow>.ts`, `tests/acceptance_<FRD_ID>.ts`.
4. Write `benches/bench_<subject>.ts`.
5. Run `npx vitest run`, then verify coverage targets met.

---

## Template

### Directory layout

```text
Repo root:
├── tests/                              # ROOT: Cross-package tests
│   ├── test_shared_utils.ts            # Tests shared utilities
│   ├── test_manifest.ts                # Tests shared manifest
│   └── test_<cross_package_feature>.ts # Other cross-package tests
│
└── packages/<name>/                    # PACKAGE: Feature-specific tests
    ├── src/
    │   └── capabilities_my_class.ts    # NO inline tests. Clean.
    ├── tests/                          # ALL 8 types REQUIRED (mandatory)
    │   ├── contract_<package>.ts
    │   ├── unit_<package>_<module>.ts
    │   ├── integration_<package>.ts
    │   ├── dogfood_<pipeline>.ts
    │   ├── smoke_<app>.ts
    │   ├── e2e_<flow>.ts
    │   └── acceptance_<FRD_ID>.ts
    ├── benches/                        # Benchmark tests only
    │   └── bench_<subject>.ts
    ├── vitest.config.ts                # Test config + coverage
    └── package.json                    # devDependencies: vitest
```

### When to use which location


| Test Type                 | Root `tests/` | Package `tests/` |
| ------------------------- | ------------- | ---------------- |
| Cross-package utility     | ✓             |                  |
| Shared contract           | ✓             |                  |
| Package-specific contract |               | ✓                |
| Package-specific unit     |               | ✓                |
| Package integration       |               | ✓                |
| Package dogfood           |               | ✓                |
| Package smoke             |               | ✓                |
| Package e2e               |               | ✓                |
| Package acceptance        |               | ✓                |
| Package benchmark         |               | ✓                |


### vitest.config.ts

```typescript
import { defineConfig } from "vitest/config";
export default defineConfig({
  test: {
    globals: true,
    environment: "node",
    include: ["tests/**/*.ts"],
    exclude: ["benches/**/*.ts"],
  },
});
```

---

## Section Contract


| Check                                                                                 | Why it belongs here                                                                        |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| No inline tests in `src/`.                                                            | Source stays clean; the suite lives under `tests/`/`benches/` only.                        |
| Flat prefix naming, no subdirectories under `tests/`.                                 | The prefix IS the virtual folder — keeps discovery and CI filters uniform.                 |
| Contract test proves class/interface implementation exists.                           | Catches unimplemented seams before behaviour tests.                                        |
| Unit tests cover happy path, edge cases, and error paths.                             | The baseline every PR must carry.                                                          |
| Integration test builds the real DI wiring; e2e asserts on real output.               | Proves composition, not mocks.                                                             |
| Dogfood test runs against LIVE service/session without mocks; skips when unavailable. | Real-end-to-end validation; CI-safe with skipif.                                           |
| Acceptance tests reference FRD/PRD IDs; smoke runs in &lt;5s.                         | Requirement traceability plus a fast boot gate.                                            |
| `vitest.config.ts` includes `tests/`, excludes `benches/`.                            | Test runs and benchmark runs never mix.                                                    |
| Benchmarks use `vitest/benchmark`, not manual timing loops.                           | Comparable, stable numbers across runs.                                                    |
| Coverage meets 70/60/50 for capabilities/agent/utility.                               | Per-layer floor before merge.                                                              |
| `npx vitest run` passes.                                                              | The suite is green or the work is not done.                                                |
| ALL 8 test types present per package.                                                 | No type is optional — contract, unit, integration, dogfood, smoke, e2e, acceptance, bench. |


---

## Dogfood / Integration Pipeline Tests

**Dogfood tests are REQUIRED — not optional.** They validate against LIVE services.

For tests that exercise actual CLI commands against live services/sessions:

1. **Place** in `tests/` directory (flat, no subdirectories).
2. **Name** with `dogfood_` prefix (e.g., `dogfood_pipeline.ts`).
3. **Always provide skip logic** — check for required credentials/sessions before running.
4. **Structure tests first** — verify command exists without external deps.
5. **Functional tests second** — run actual pipeline with real inputs.
6. **Use fixtures** — create temporary test files, cleanup after.
7. **Never require login in CI** — skip gracefully when deps unavailable.
8. **Mark with `@dogfood` decorator** for identification.

Dogfood tests skip in CI when services unavailable — but the file MUST exist.

## Verify

```bash
npx vitest run
npx vitest bench benches/bench_<subject>.ts
npx vitest run --coverage
# Checks: ALL 8 test types present (contract/unit/integration/dogfood/smoke/e2e/acceptance/bench)
# All tests green; benchmarks run through vitest/benchmark with benches excluded from test glob;
# coverage floors met.
# Manual (not machine-checked): acceptance rows map 1:1 to FRD/PRD IDs; smoke <5s;
# no inline tests left in src/; prefixes flat, no subdirectories; coverage 70/60/50.
# Fallback compile gate: npx tsc --noEmit
```


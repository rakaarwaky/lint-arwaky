---
name: create-test-typescript
description: "Generates contract, unit, integration, E2E, acceptance, and smoke test suites in tests/ (flat prefix naming), plus benchmark suites in benches/ (separate directory). Use when adding a new capability package, increasing coverage, preparing a release, or validating performance. Triggers: create tests typescript, add tests typescript, create test suite typescript, package tests typescript, e2e tests typescript, benchmark typescript."
metadata:
  tags: [typescript, testing, vitest, jest, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-rust, create-test-python]
---

# Create TypeScript Test Suite

## Directory Layout

```
packages/<name>/
├── src/
│   └── capabilities_my_class.ts    # NO inline tests. Clean.
├── tests/                          # All test types, flat prefix naming
│   ├── contract_<package>.ts
│   ├── unit_<package>_<module>.ts
│   ├── integration_<package>.ts
│   ├── smoke_<app>.ts
│   ├── e2e_<flow>.ts
│   └── acceptance_<FRD_ID>.ts
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.ts
├── vitest.config.ts                # Test config + coverage
└── package.json                    # devDependencies: vitest
```

## Rules

- **Tests** (`tests/`): flat, prefix IS the virtual folder — no real subdirectories.
- **Benchmarks** (`benches/`): use `vitest/benchmark` — never hand-rolled timing.
- Prefix pattern: `<type>_<subject>.ts`
- Contract tests verify class/interface implementation.
- Unit tests: happy path, edge cases, error paths.
- Integration tests: use real DI container / entry point.
- E2E tests: hit real API/CLI, assert on real output.
- Acceptance tests: map 1:1 to business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

## vitest.config.ts

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

Run benchmarks: `npx vitest bench benches/bench_<subject>.ts`

## Test Types

| Prefix | Directory | Scope | Speed | Runs when |
| --- | --- | --- | --- | --- |
| `contract_` | tests/ | Class/interface impl | ms | Every PR |
| `unit_` | tests/ | One public function | ms | Every PR |
| `integration_` | tests/ | Package / DI wiring | ms–s | Every PR |
| `smoke_` | tests/ | App boots + responds | <5s | Every PR |
| `e2e_` | tests/ | Full request lifecycle | s | Every PR (critical path) |
| `acceptance_` | tests/ | Business requirement met | s | Every PR / release gate |
| `bench_` | benches/ | Performance regression | s–min | Release gate / nightly |

## Coverage Targets

| Layer | Minimum |
| --- | --- |
| Capabilities | 70% |
| Agent | 60% |
| Utility | 50% |

## Workflow

```
- [ ] Step 1: Analyze package / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write tests/contract_<package>.ts
- [ ] Step 4: Write tests/unit_<package>_<module>.ts
- [ ] Step 5: Write tests/integration_<package>.ts
- [ ] Step 6: Write tests/smoke_<app>.ts
- [ ] Step 7: Write tests/e2e_<flow>.ts
- [ ] Step 8: Write tests/acceptance_<FRD_ID>.ts
- [ ] Step 9: Write benches/bench_<subject>.ts
- [ ] Step 10: npx vitest run
- [ ] Step 11: Verify coverage targets met
```

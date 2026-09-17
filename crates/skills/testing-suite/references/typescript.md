# Testing — TypeScript

## Directory layout

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

## Language rules

- **Benchmarks** (`benches/`): use `vitest/benchmark` — never hand-rolled timing.
- Contract tests verify class/interface implementation.
- Integration tests: use real DI container / entry point.
- E2E tests: hit real API/CLI, assert on real output.

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

## Run commands

| Purpose         | Command                                            |
| ----------------- | ----------------------------------------------------- |
| Full suite      | `npx vitest run`                                    |
| Benchmarks      | `npx vitest bench benches/bench_<subject>.ts`       |
| Coverage        | `npx vitest run --coverage`                         |

## Workflow steps (TypeScript names)

1. Analyze package / app structure.
2. Write `tests/contract_<package>.ts`, `tests/unit_<package>_<module>.ts`, `tests/integration_<package>.ts`.
3. Write `tests/smoke_<app>.ts`, `tests/e2e_<flow>.ts`, `tests/acceptance_<FRD_ID>.ts`.
4. Write `benches/bench_<subject>.ts`.
5. Run `npx vitest run`, then verify coverage targets met.

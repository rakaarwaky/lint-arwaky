# HOW TO MAKE AGENT TYPESCRIPT

> **Purpose**: Orchestrate only: receive a request, call the aggregate contract, return shared VOs — no I/O, no computation.
>
> **Audience**: Agents and engineers scaffolding AES agent orchestrators.
>
> **Scope**: Python, Rust, and TypeScript `agent_<domain>_orchestrator` files — one aggregate, 3-block structure, zero I/O.
>
> **Location**: Feature domain package; depends only on shared contracts/taxonomy/utility.
>
> **Length**: One `_orchestrator` file per feature; ≥1 aggregate, ≤3 types.

---

## Rules

### Import rules

**Allowed imports:** `shared/*` — taxonomy VOs, constants, aggregate interfaces, protocol interfaces.
**Forbidden imports:** `capabilities_*`, `agent_*`, `surface_*`.

### 3-Block Structure

```text
// Block 1: Class Definition & Constructor
// Block 2: Aggregate Method Implementation
// Block 3: Utility Methods, Factories, Helpers
```

Method placement:

```text
Module-level function?                  → EXTRACT to *_utility.ts
Defined in aggregate interface?         → Block 2
toString / toJSON / valueOf / equals?   → Block 3
static factory?                         → Block 3
private helper (uses this)?             → Block 3
Pure static, no class dep?              → EXTRACT to *_utility.ts
```

### Helper vs Utility

Keep in Block 3 if ANY: uses `this`, coupled to this class, static factory, agent-specific logic, single-use.
Extract to utility only if ALL: no `this`, pure, no side effects, domain-agnostic, reusable.
I/O: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

### Allowed / forbidden operations

**Allowed ops:** `for`/`while`/`for...of`, `if/else`/`switch`, `try/catch`/`throw`, `Promise.all`/`await`, collecting results into shared VOs.
**Forbidden ops:** `fs.*`, `readFile`, `writeFile`, `fetch`, `axios`, `http`, database ops, stdout/stderr write, env mutation, global state mutation.

### Computation, Errors, VOs

**Computation forbidden:** arithmetic, totals, averages, `.reduce`/`.fold`, parsing, normalization.
Allowed: iteration to call deps, routing results, propagating errors.

**Error rules:**
- Rule 1: Never silently discard — no `checker.check() ?? ""`.
- Rule 2: Analysis orchestration → `<ResultVO>[]`, catch per-item into VO.
- Rule 3: Execution orchestration → `Result<ExecutionReport, AgentExecutionError>`.
- Rule 4: Delegate I/O errors to capabilities — agent only wraps into VO.

**VO rules:** `string`/`number` forbidden for domain fields/contracts. `boolean` for toggles only.

### Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy.
2. Class implements aggregate interface? If no → create `contract_<name>_aggregate.ts`.
3. Enforce 3-Block.
4. ≥1 aggregate interface, ≤3 types (class+interface+enum), DI via protocols, shared VOs.
5. No forbidden imports, no I/O, no computation.
6. No silent errors, no raw primitives in contracts, no magic constants.
7. `npx tsc --noEmit`.

---

## Template

```typescript
import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';

// ─── Block 1: Class Definition & Constructor ──────────────
export class Agent<Name> {
    constructor(private readonly aggregate: I<Name>Aggregate) {}

    // ─── Block 2: Aggregate Method Implementation ─────────
    execute(request: <RequestVO>): <ResultVO>[] {
        // orchestration only — delegate to aggregate
        return this.aggregate.process(request);
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'Agent<Name>()';
    }

    static create(): Agent<Name> {
        return new Agent<Name>(/* inject deps */);
    }
}
```

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Block 1 → 2 → 3 order followed. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY aggregate interface method implementations. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 3: utility methods, factories, private helpers. | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 class implements aggregate interface; ≤3 total types. | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain data; DI via protocol interfaces; shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero I/O, zero business logic, zero domain computation. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Aggregate registered in shared package `index.ts`. | Required by AES layer rules and the linter; missing it is a defect. |
| `npx tsc --noEmit` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): zero I/O/computation; no silent error discard; 3-block order; helper-vs-utility matrix.
# Fallback compile gate: npx tsc --noEmit
```

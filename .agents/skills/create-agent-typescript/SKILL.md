---
name: create-agent-typescript
description: "Create and validate TypeScript agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, max 3 types per file, aggregate interface contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags:
    [
      typescript,
      aes,
      agent,
      aggregate,
      structure,
      3-block-structure,
      di,
      orchestration,
      vo,
    ]
  triggers:
    - "create agent typescript"
    - "add agent typescript"
    - "fix agent structure typescript"
    - "create aggregate typescript"
    - "agent missing aggregate typescript"
    - "validate agent logic typescript"
    - "check agent typescript"
    - "audit agent typescript"
  dependencies: []
  related:
    - create-capabilities-typescript
    - create-taxonomy-typescript
    - create-contract-typescript
---

# create-agent-typescript

Agent layer = **orchestration / pipeline execution only**. No I/O, no business logic, no domain computation, no local domain data. Depends only on Taxonomy + Contract + Utility layers.

## Definition of Done

1. At least one class implements an aggregate interface (Block 2).
2. Block 2 contains ONLY aggregate interface method implementations.
3. Utility methods, static factories, private helpers → Block 3.
4. Zero I/O, zero business logic, zero domain computation.
5. No locally defined domain data structures.
6. Service deps via DI using aggregate/protocol interfaces.
7. Config/value fields use shared VOs.
8. Aggregate signatures use shared VOs.
9. Total type count ≤ 3 (class + interface + enum, not counting `type` aliases).
10. `npx tsc --noEmit` passes.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Orchestration flow (`for`, `while`, `for...of`) | Domain computation / arithmetic / analytics |
| Control flow (`if/else`, `switch`) | Data transformation / business rules / domain validation |
| Calling injected aggregate/protocol methods | File I/O (`fs.`, `readFile`, `writeFile`) |
| Error propagation (`try/catch`, `throw`) | Network calls (`fetch`, `axios`, `http`) |
| Collecting results into shared VO types | Database operations |
| Async coordination (`Promise.all`, `await`) | Direct stdout/stderr / env / global mutation |
| Aggregate interface implementation + private helpers | Import from `capabilities_*`, `agent_*`, `surface_*` |
| | Locally defined domain data / raw primitives in aggregate contracts |

**Allowed imports:** `shared/*` — taxonomy VOs, constants, aggregate interfaces, protocol interfaces.

---

## The 3-Block Structure

Every file MUST follow this order:

```text
// ─── Block 1: Class Definition & Constructor ───────────────
// ─── Block 2: Aggregate Method Implementation ──────────────
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
```

- **Block 1** — class declaration + `constructor` only. See `templates/agent_name.ts`.
- **Block 2** — ONLY aggregate interface method implementations. No `toString`, `toJSON`, `static`, `private` here.
- **Block 3** — utility methods (`toString`, `toJSON`), static factories, private helpers.

### Method Placement

```text
Module-level function?                   → EXTRACT to *_utility.ts
Defined in aggregate interface?          → Block 2
toString / toJSON / valueOf / equals?    → Block 3
static factory?                          → Block 3
private helper (uses this)?              → Block 3
Pure static, no class dep?               → EXTRACT to *_utility.ts
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `this`, tightly coupled to this class, static factory, agent-specific pipeline/domain knowledge, stateless but single-use.

**Extract to `*_utility.ts`** only if ALL: stateless (no `this`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

```text
Found reusable code in agent?
  │
  ├─ Knows agent-specific or domain-specific details?
  │   └─ YES → Block 3
  ├─ Needs this or class state?
  │   └─ YES → Block 3
  └─ Stateless, domain-agnostic, reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
```

---

## Computation, Error Handling, and VO Rules

**Computation** — forbidden: arithmetic, totals, averages, counts as domain decisions, `.reduce`, `.fold`, parsing, normalization. Allowed: iterating to call deps, routing results, propagating errors.

**Error handling:**
- Rule 1: Never silently discard — forbidden: `checker.check() ?? ""`.
- Rule 2: Analysis orchestration → return `<ResultVO>[]`, catch per-item into VO.
- Rule 3: Execution orchestration → return `Result<ExecutionReport, AgentExecutionError>`.
- Rule 4: Delegate I/O errors to capabilities — agent only catches and wraps into VO.

**VOs** — aggregate contracts must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `string`, `number` | Forbidden for domain fields/contracts. Use VO. |
| `boolean` | Allowed for semantic toggles only. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/agent_name.ts` | Full agent implementation (3-block structure) |
| `templates/contract_name_aggregate.ts` | Aggregate interface definition |

---

## Workflow

1. **Analyze** — Is this orchestration only? If computation → capabilities. If domain data → taxonomy.
2. **Check aggregate** — Does agent class implement an aggregate interface? If no → create `contract_<name>_aggregate.ts`.
3. **Enforce 3-Block** — Reorganize: class+`constructor` → aggregate methods → utility/factories/helpers.
4. **Type discipline** — ≥1 aggregate interface, ≤3 types (class+interface+enum), DI via protocols, shared VOs.
5. **Helper boundary** — Apply Helper vs Utility rules above.
6. **Layer compliance** — No forbidden imports, no I/O, no business logic, no computation.
7. **Error/VO/constants** — No silent swallowing, no raw primitives in contracts, no magic constants.
8. **Compile** — `npx tsc --noEmit`.

---

## Verification Checklist

- [ ] 3-Block Structure followed (Block 1 → 2 → 3).
- [ ] Block 1: exactly one implementation class + `constructor`.
- [ ] Block 2: ONLY aggregate interface method implementations.
- [ ] Block 3: utility methods, factories, private helpers.
- [ ] Agent class implements an aggregate interface.
- [ ] Aggregate: only public contract methods, no private helpers, no constructors.
- [ ] Reusable stateless domain-agnostic functions extracted to `*_utility.ts`.
- [ ] One implementation class per file, ≤3 total types.
- [ ] No local domain data — all imported from `shared/taxonomy`.
- [ ] Service deps via protocol interfaces (DI).
- [ ] Shared VOs for config fields and aggregate signatures.
- [ ] Zero I/O, zero business logic, zero domain computation.
- [ ] No forbidden imports (`capabilities_*`, `agent_*`, `surface_*`).
- [ ] Aggregate registered in shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes.

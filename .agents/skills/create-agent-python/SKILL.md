---
name: create-agent-python
description: "Create and validate Python agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, max 3 types per file, aggregate ABC contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags:
    [
      python,
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
    - "create agent python"
    - "add agent python"
    - "fix agent structure python"
    - "create aggregate python"
    - "agent missing aggregate python"
    - "validate agent logic python"
    - "check agent python"
    - "audit agent python"
  dependencies: []
  related:
    - create-capabilities-python
    - create-taxonomy-python
    - create-contract-python
---

# create-agent-python

Agent layer = **orchestration / pipeline execution only**. No I/O, no business logic, no domain computation, no local domain data. Depends only on Taxonomy + Contract + Utility layers.

## Definition of Done

1. At least one class inherits an aggregate ABC (Block 2).
2. Block 2 contains ONLY aggregate ABC method implementations.
3. Dunders, factory classmethods, private helpers → Block 3.
4. Zero I/O, zero business logic, zero domain computation.
5. No locally defined domain data structures.
6. Service deps via DI using aggregate/protocol interfaces.
7. Config/value fields use shared VOs.
8. Aggregate signatures use shared VOs.
9. Total class count ≤ 3.
10. `python -c "import <module>"` passes.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Orchestration flow (`for`, `while`, `async for`) | Domain computation / arithmetic / analytics |
| Control flow (`if/else`, `elif`, `match`) | Data transformation / business rules / domain validation |
| Calling injected aggregate/protocol traits | File I/O (`open()`, `Path()`, `os.`) |
| Error propagation (`try/except`, `raise`) | Network calls (`requests.`, `httpx.`) |
| Collecting results into shared VO types | DB operations (`sqlite3.`, `asyncpg.`) |
| Async coordination (`asyncio.wait_for`) | Direct stdout/stderr / env / global-state mutation |
| Aggregate ABC implementation + private helpers | Import from `capabilities_*`, `agent_*`, `surface_*`, concrete `utility_*` |
| | Locally defined domain data / raw primitives in aggregate contracts |

**Allowed imports:** `shared/*` — taxonomy VOs, constants, aggregate ABCs, protocol ABCs.

---

## The 3-Block Structure

Every file MUST follow this order:

```text
# ─── Block 1: Class Definition & Constructor ───────────────
# ─── Block 2: Aggregate Method Implementation ──────────────
# ─── Block 3: Dunder Methods, Factories, Helpers ───────────
```

- **Block 1** — class + `__init__` only. See `templates/block1_class_constructor.py`.
- **Block 2** — ONLY aggregate ABC method implementations. See `templates/block2_aggregate_method.py`. No `__repr__`, `__str__`, `@classmethod`, `@staticmethod` here.
- **Block 3** — dunders, factory classmethods, private helpers. See `templates/block3_dunder_helpers.py`.

### Method Placement

```text
Module-level def?              → EXTRACT to *_utility.py
@abstractmethod in ABC?        → Block 2
Dunder / factory classmethod?  → Block 3
@staticmethod, pure + no class dep? → EXTRACT to *_utility.py
@staticmethod, class-coupled?  → Block 3
Private helper (uses self)?    → Block 3
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `self`, tightly coupled to this class, factory method, agent-specific pipeline/domain knowledge, stateless but single-use.

**Extract to `*_utility.py`** only if ALL: stateless (no `self`/`cls`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

---

## Computation, Error Handling, and VO Rules

**Computation** — forbidden: arithmetic, totals, averages, counts as domain decisions, sum/fold/parsing/normalization. Allowed: iterating to call deps, routing results, propagating errors. See `templates/bad_computation.py` / `templates/good_computation.py`.

**Error handling:**
- Rule 1: Never silently discard errors. See `templates/bad_error_silent.py`.
- Rule 2: Analysis orchestration → return `list[<ResultVO>]`. See `templates/good_error_orchestration.py`.
- Rule 3: Execution orchestration → return `Result[...]`. See `templates/good_error_return.py`.
- Rule 4: Delegate I/O error handling to capabilities. See `templates/bad_error_io.py` / `templates/good_error_io.py`.

**VOs** — aggregate contracts must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `str`, `int`, `float` | Forbidden for domain fields/contracts. Use VO. |
| `bool` | Allowed for semantic toggles only. |

See `templates/bad_primitive_contract.py` / `templates/good_primitive_contract.py`.

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/agent_name_orchestrator.py` | Full agent implementation (3-block structure) |
| `templates/contract_name_aggregate.py` | Aggregate ABC definition |
| `templates/block1_class_constructor.py` | Block 1: class + constructor |
| `templates/block2_aggregate_method.py` | Block 2: aggregate method implementation |
| `templates/block3_dunder_helpers.py` | Block 3: dunders, factories, helpers |
| `templates/bad_*.py` / `templates/good_*.py` | BAD/GOOD code examples |

---

## Workflow

1. **Analyze** — Is this orchestration only? If computation → capabilities. If domain data → taxonomy.
2. **Check aggregate** — Does agent class inherit an aggregate ABC? If no → create `contract_<name>_aggregate.py`.
3. **Enforce 3-Block** — Reorganize: class+`__init__` → aggregate methods → dunders/factories/helpers.
4. **Type discipline** — ≥1 aggregate ABC, ≤3 classes, DI via protocols, shared VOs.
5. **Helper boundary** — Apply Helper vs Utility rules above.
6. **Layer compliance** — No forbidden imports, no I/O, no business logic, no computation.
7. **Error/VO/constants** — No silent swallowing, no raw primitives in contracts, no magic constants.
8. **Compile** — `python -c "import <module>"`.

---

## Verification Checklist

- [ ] 3-Block Structure followed (Block 1 → 2 → 3).
- [ ] Block 1: exactly one implementation class + `__init__`.
- [ ] Block 2: ONLY aggregate ABC method implementations.
- [ ] Block 3: dunders, factories, private helpers.
- [ ] Agent class inherits an aggregate ABC.
- [ ] Aggregate: only public contract methods, no private helpers, no constructors.
- [ ] Reusable stateless domain-agnostic functions extracted to `*_utility.py`.
- [ ] One implementation class per file, ≤3 total.
- [ ] No local domain data — all imported from `shared/taxonomy`.
- [ ] Service deps via protocol interfaces (DI).
- [ ] Shared VOs for config fields and aggregate signatures.
- [ ] Zero I/O, zero business logic, zero domain computation.
- [ ] No forbidden imports (`capabilities_*`, `agent_*`, `surface_*`).
- [ ] Aggregate registered in shared module `__init__.py`.
- [ ] `python -c "import <module>"` passes.

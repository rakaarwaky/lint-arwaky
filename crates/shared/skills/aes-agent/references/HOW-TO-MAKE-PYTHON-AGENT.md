# HOW TO MAKE AGENT PYTHON

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

**Allowed imports:** `shared/*` — taxonomy VOs, constants, aggregate ABCs, protocol ABCs, utility functions.
**Forbidden imports:** `capabilities_*`, `agent_*`, `surface_*`.

### 3-Block Structure

```text
# Block 1: Class Definition & Constructor
# Block 2: Aggregate Method Implementation
# Block 3: Dunder Methods, Factories, Helpers
```

Method placement:

```text
Module-level def?                    → EXTRACT to *_utility.py
@abstractmethod in aggregate ABC?    → Block 2
Dunder / factory @classmethod?       → Block 3
@staticmethod pure + no class dep?   → EXTRACT to *_utility.py
Private helper (uses self)?          → Block 3
```

### Helper vs Utility

Keep in Block 3 if ANY: uses `self`, coupled to this class, factory, agent-specific logic, single-use.
Extract to utility only if ALL: no `self`/`cls`, pure, no side effects, domain-agnostic, reusable.
I/O: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

### Allowed / forbidden operations

**Allowed ops:** `for`/`while`/`async for`, `if/else`/`match`, `try/except`/`raise`, `asyncio.wait_for`, collecting results into shared VOs.
**Forbidden ops:** `open()`, `Path()`, `os.*`, `requests.*`, `httpx.*`, `sqlite3.*`, `asyncpg.*`, stdout/stderr write, env mutation, global state mutation.

### Computation, Errors, VOs

**Computation forbidden:** arithmetic, totals, averages, `.reduce`/`.fold`, parsing, normalization. Allowed: iteration to call deps, routing results, propagating errors.

**Error rules:**
- Rule 1: Never silently discard — no `checker.check() or ""`.
- Rule 2: Analysis orchestration → return `list[<ResultVO>]`, catch per-item into VO.
- Rule 3: Execution orchestration → return `Result[...]`.
- Rule 4: Delegate I/O errors to capabilities — agent only wraps into VO.

**VO rules:** `str`/`int`/`float` forbidden for domain fields/contracts. `bool` for semantic toggles only.

### Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy.
2. Agent class inherits aggregate ABC? If no → create `contract_<name>_aggregate.py`.
3. Enforce 3-Block.
4. ≥1 aggregate ABC, ≤3 classes, DI via protocols, shared VOs.
5. No forbidden imports, no I/O, no computation.
6. No silent errors, no raw primitives in contracts, no magic constants.
7. `python -c "import <module>"`.

---

## Template

```python
from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate

# ─── Block 1: Class Definition & Constructor ──────────────
class Agent<Name>:
    def __init__(self, aggregate: I<Name>Aggregate) -> None:
        self._aggregate = aggregate

    # ─── Block 2: Aggregate Method Implementation ─────────
    def execute(self, request: <RequestVO>) -> list[<ResultVO>]:
        # orchestration only — delegate to aggregate
        results = self._aggregate.process(request)
        return results

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Agent<Name>()"

    @classmethod
    def create_default(cls) -> "Agent<Name>":
        return cls()
```

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Block 1 → 2 → 3 order followed. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY aggregate ABC method implementations. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 3: dunders, factories, private helpers. | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 class inherits aggregate ABC; ≤3 total classes. | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain data; DI via protocol interfaces; shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero I/O, zero business logic, zero domain computation. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Aggregate registered in shared `__init__.py`. | Required by AES layer rules and the linter; missing it is a defect. |
| `python -c "import <module>"` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): zero I/O/computation; no silent error discard; 3-block order; helper-vs-utility matrix.
# Fallback compile gate: python -c "import <shared_package>.<module>"
```

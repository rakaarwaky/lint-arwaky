# AES Agent HOW-TO (Python)

> **Purpose**: Orchestrate only: receive a request, call the aggregate contract, return shared VOs — no I/O, no computation.
>
> **Audience**: Agents and engineers scaffolding AES agent orchestrators.
>
> **Scope**: Python `agent_<domain>_orchestrator` files — one aggregate, 3-block structure, zero I/O.
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
- Rule 2: Analysis orchestration → return the taxonomy-defined response enum variant; each variant
holds VO-wrapped values, not raw primitives.
- Rule 3: Execution orchestration → return `Result[...]` wrapped in a VO variant.
- Rule 4: Delegate I/O errors to capabilities — agent only wraps into VO.

**VO rules:** `str`/`int`/`float` forbidden for domain fields/contracts. `bool` for semantic toggles only.

## Template

```python
from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate
from shared.<domain>.contract_<name>_protocol import I<Seam>Protocol

# ─── Block 1: Class Definition & Constructor ──────────────
class Agent<Name>:
    # Injected deps are protocol ABCs (what the agent dispatches to), not the aggregate.
    def __init__(self, dep: I<Seam>Protocol) -> None:
        self._dep = dep

    # ─── Block 2: Aggregate Method Implementation ─────────
    # Exactly one method — the single entry point. Dispatch to the owning protocol.
    def execute(self, request: <RequestVO>) -> <ResponseVO>:
        # Orchestration only: match the request variant, delegate to the protocol.
        # No computation, no I/O, no silently discarded errors.
        if isinstance(request, <RequestVO>.<Variant1>):
            return <ResponseVO>.<Result1>(self._dep.<operation_1>(..))
        return <ResponseVO>.<Result2>(self._dep.<operation_2>(..))

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "Agent<Name>()"

    @classmethod
    def create_default(cls) -> "Agent<Name>":
        return cls()
```

---

## Section Contract

| Check                                                         | Why it belongs here                                                 |
| ------------------------------------------------------------- | ------------------------------------------------------------------- |
| Block 1 → 2 → 3 order followed.                               | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY aggregate ABC method implementations.           | Required by AES layer rules and the linter; missing it is a defect. |
| Block 3: dunders, factories, private helpers.                 | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 class inherits aggregate ABC; ≤3 total classes.            | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain data; DI via protocol interfaces; shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero I/O, zero business logic, zero domain computation.       | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden imports.                                         | Required by AES layer rules and the linter; missing it is a defect. |
| Aggregate registered in shared `__init__.py`.                 | Required by AES layer rules and the linter; missing it is a defect. |
| `python -c "import <module>"` passes.                         | Required by AES layer rules and the linter; missing it is a defect. |

---

## Workflow

1. **Triage (diagnostic tree)** — check in order; the first "No" dictates the next action: orchestration only (no I/O, no computation, no local domain data)? an `_aggregate` contract exists? — no: create it first with `aes-contract`. Block 2 holds only aggregate methods; ≥1 aggregate; ≤3 types? no silently discarded errors; no raw primitives in signatures (wrap in VOs; propagate errors)? `lint-arwaky-cli scan <layer-path>` exits 0?
2. Verify orchestration only — computation → capabilities, domain data → taxonomy, pure helpers → utility.
3. Ensure an `_aggregate` contract exists — if missing, run `aes-contract` first.
4. Resolve the feature agent dir. Run `lint-arwaky-cli scan <layer-path>` — findings are the work list.
5. Implement the 3 blocks from § Template / § Section Contract; the class inherits the aggregate ABC via DI.
6. Check imports, I/O, computation, error handling, and primitives against § Rules.
7. Register in `__init__.py`, then compose via `aes-root` / consume via `aes-surface`.
8. **Placement (AES702/AES703)** — the file sits in a feature folder (`modules/<feature_name>/src/agent_<domain>_orchestrator`), not in `shared/` or a surface-dominated folder; `FRD.md` + `BACKLOG.md` at the feature folder root beside the source; a capabilities-only folder without its own agent is a split feature.
9. Fallback compile gate: `python -c "import <shared_package>.<module>"`.

## Review

- Block 1 → 2 → 3 order; Block 2 holds only aggregate ABC method implementations; helper-vs-utility split respected (free functions extracted to `*_utility.py`) — § Rules § 3-Block Structure, § Helper vs Utility.
- Orchestration only: zero I/O, zero computation, zero domain data; DI via protocol ABCs; shared VOs in signatures — § Rules § Allowed / forbidden operations, § Computation, Errors, VOs.
- No silently discarded errors (`or ""` on results); errors propagate or wrap into VOs — § Rules § Computation, Errors, VOs (Rule 1).
- No agent file in `shared/`; capabilities-only folders carry an agent of their own — § Workflow step 8.
- `contract_<name>_aggregate.py` exists; ≥1 aggregate, ≤3 types.
## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Language compile is a fallback gate only: `python -c "import <shared_package>.<module>"`
```

# HOW TO MAKE CAPABILITIES PYTHON

> **Purpose**: Implement concrete protocol behaviour: domain rules and external adaptation as protocol implementations.
>
> **Audience**: Agents and engineers scaffolding AES capability files.
>
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.
>
> **Location**: Feature domain package; depends only on taxonomy, `_protocol` contracts, and utility.
>
> **Length**: At most 3 types per file; Block 1 → 2 → 3 order.

---

## Rules

### Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

### Structure rules (Python)

- Rule 1: Internal helper classes without ABC → ALLOWED.
- Rule 2: ≥1 class inherits a protocol ABC.
- Rule 3: Total class count ≤ 3.

### 3-Block Structure

```text
# Block 1: Class Definition & Constructor
# Block 2: Protocol Method Implementation
# Block 3: Dunder Methods, Factories, Helpers
```

Method placement: `@abstractmethod` → Block 2. Dunder/factory/private → Block 3. Stateless free function → extract to `*utility_.py`.

### Helper vs Utility

Keep in Block 3 if ANY: uses `self`, domain-specific, single consumer, factory.
Extract to utility only if ALL: no `self`, pure, no side effects, domain-agnostic, ≥2 consumers.
I/O: stateless + I/O + domain-agnostic = utility OK.

### Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File imports from `_protocol` module — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.py` if missing.
4. Enforce 3-Block.
5. AES403: ≥1 protocol inheritor, ≤3 classes, DI via protocols, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `python -c "import <module>"`.

---

## Template

### 3-block implementation

```python
from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_protocol import I<Name>Protocol

# ─── Block 1: Class Definition & Constructor ──────────────
class Capabilities<Name>(I<Name>Protocol):
    def __init__(self, /* DI params */) -> None:
        # DI fields use protocol interfaces
        # Value fields use shared VOs
        ...

    # ─── Block 2: Protocol Method Implementation ──────────────
    def method_name(self, param: <VO>) -> None:
        # domain behavior
        ...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────────
    def __repr__(self) -> str:
        return "Capabilities<Name>()"

    @classmethod
    def create_default(cls) -> "Capabilities<Name>":
        return cls()
```

### Protocol ABC

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>

class I<Name>Protocol(ABC):
    @abstractmethod
    def method_name(self, param: <VO>) -> None: ...
```

---

## Section Contract


| Check                                                             | Why it belongs here                                                 |
| ----------------------------------------------------------------- | ------------------------------------------------------------------- |
| Block 1 → 2 → 3 order followed.                                   | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY protocol ABC method implementations.                | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 class inherits protocol ABC; ≤3 total classes.                 | Required by AES layer rules and the linter; missing it is a defect. |
| Imports from `_protocol` module only.                             | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain models, no agent/capability imports.              | Required by AES layer rules and the linter; missing it is a defect. |
| DI via protocol interfaces; shared VOs for fields and signatures. | Required by AES layer rules and the linter; missing it is a defect. |
| Constants → `taxonomy_<domain>_constant.py`.                      | Required by AES layer rules and the linter; missing it is a defect. |
| Low-level ops → Utility.                                          | Required by AES layer rules and the linter; missing it is a defect. |
| `python -c "import <module>"` passes.                             | Required by AES layer rules and the linter; missing it is a defect. |


---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): 3-block order; Block 2 only protocol methods; helper-vs-utility matrix; role naming lists.
# Fallback compile gate: python -c "import <shared_package>.<module>"
```


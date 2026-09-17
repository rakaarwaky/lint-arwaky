# Capabilities — Python

File: `capabilities_<domain>_<role>.py`. Capabilities = concrete protocol ABC implementation.

## Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

## Structure rules (Python)

- Rule 1: Internal helper classes without ABC → ALLOWED.
- Rule 2: ≥1 class inherits a protocol ABC.
- Rule 3: Total class count ≤ 3.

## 3-Block Structure

```text
# Block 1: Class Definition & Constructor
# Block 2: Protocol ABC Method Implementation
# Block 3: Dunder Methods, Factories, Helpers
```

Method placement: `@abstractmethod` → Block 2. Dunder/factory/private → Block 3. Stateless free function → extract to `*utility_.py`.

## Helper vs Utility

Keep in Block 3 if ANY: uses `self`, domain-specific, single consumer, factory.
Extract to utility only if ALL: no `self`, pure, no side effects, domain-agnostic, ≥2 consumers.
I/O: stateless + I/O + domain-agnostic = utility OK.

## Templates

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

    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def method_name(self, param: <VO>) -> None:
        # domain behavior
        ...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
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

## Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File imports from `_protocol` module — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.py` if missing.
4. Enforce 3-Block.
5. AES403: ≥1 protocol inheritor, ≤3 classes, DI via protocols, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `python -c "import <module>"`.

## Checklist

- [ ]  Block 1 → 2 → 3 order followed.
- [ ]  Block 2: ONLY protocol ABC method implementations.
- [ ]  ≥1 class inherits protocol ABC; ≤3 total classes.
- [ ]  Imports from `_protocol` module only.
- [ ]  No local domain models, no agent/capability imports.
- [ ]  DI via protocol interfaces; shared VOs for fields and signatures.
- [ ]  Constants → `taxonomy_<domain>_constant.py`.
- [ ]  Low-level ops → Utility.
- [ ]  `python -c "import <module>"` passes.

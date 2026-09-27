# HOW TO MAKE MIGRATION PYTHON

> **Purpose**: Guide phased migration of legacy Python projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Phase-based migration workflow for Python projects; references layer skills for execution.
>
> **Location**: Project root; each phase operates on a layer directory.
>
> **Length**: 9 phases (0–8); total duration depends on violation count.

---

## Rules



## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (1 method) + aggregate (many exports).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

Nine rules. Each one governs one migration phase.

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one method per feature.** Never a mega `execute(op, …)` that dispatches multiple features.
4. **Aggregate = many methods, one per export.** Rich consumer surface; surface/root call specific verbs.
5. **Utility is stateless free functions only.** No class, no state, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

---



## Template

Copy, fill, delete nothing. Migration proceeds phase-by-phase; each phase outputs files matching the layer HOW-TO.

### Phase 1 — Taxonomy

```python
# modules/shared/src/taxonomy_<domain>_vo.py
from __future__ import annotations
from dataclasses import dataclass

@dataclass(frozen=True)
class <VOName>:
    """Domain value object — wraps a primitive to prevent leakage."""
    value: str  # or int, bool, etc. as appropriate

# Register in modules/shared/src/__init__.py
```

### Phase 2 — Contract

```python
# modules/shared/src/contract_<domain>_protocol.py
from abc import ABC, abstractmethod
from modules.shared.src.taxonomy_<domain>_vo import <VO>

class I<Domain>Protocol(ABC):
    """One method for one feature."""

    @abstractmethod
    def <feature>(self, arg: <VO>) -> <ResultVO>:
        ...

# modules/shared/src/contract_<domain>_aggregate.py
class I<Domain>Aggregate(ABC):
    """Many methods, one per export."""

    @abstractmethod
    def list_<thing>(self) -> list[<VO>]:
        ...

    @abstractmethod
    def create_<thing>(self, arg: <VO>) -> ExitCode:
        ...
```

### Phase 3 — Utility

```python
# modules/shared/src/utility_<domain>_<role>.py
from modules.shared.src.taxonomy_<domain>_vo import <VO>

def <helper>(arg: <VO>) -> <VO>:
    """Stateless helper — no state, no class, no upward imports."""
    ...
```

### Phase 4 — Capabilities

```python
# modules/<domain>/src/capabilities_<domain>_<role>.py
from modules.shared.src.contract_<domain>_protocol import I<Domain>Protocol
from modules.shared.src.taxonomy_<domain>_vo import <VO>

class <Capability>(I<Domain>Protocol):
    """Block 1: Struct with DI deps."""
    def __init__(self, dep: SomeDep) -> None:
        self._dep = dep

    """Block 2: Protocol implementation."""
    def <feature>(self, arg: <VO>) -> <ResultVO>:
        # Business logic here
        ...

    """Block 3: Factory."""
    @classmethod
    def create(cls, dep: SomeDep) -> I<Domain>Protocol:
        return cls(dep)
```

### Phase 5 — Agent

```python
# modules/<domain>/src/agent_<domain>_orchestrator.py
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate
from modules.shared.src.contract_<domain>_protocol import I<Domain>Protocol

class <Domain>Orchestrator(I<Domain>Aggregate):
    """Block 1: Struct with protocol deps."""
    def __init__(self, protocol: I<Domain>Protocol) -> None:
        self._protocol = protocol

    """Block 2: Aggregate implementation — orchestrate, don't compute."""
    def list_<thing>(self) -> list[<VO>]:
        return self._protocol.<feature>(...)  # delegate only
```

### Phase 6 — Surface

```python
# modules/<domain>/src/surface_<domain>_command.py
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate

def main(aggregate: I<Domain>Aggregate, argv: list[str]) -> int:
    """Parse input, call aggregate, render output — no business logic."""
    ...
```

### Phase 7 — Root

```python
# modules/<domain>/src/root_<domain>_container.py
from modules.<domain>.src.capabilities_<domain>_<role> import <Capability>
from modules.<domain>.src.agent_<domain>_orchestrator import <Domain>Orchestrator
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate

class <Domain>Container:
    def __init__(self, dep: SomeDep) -> None:
        proto = <Capability>.create(dep)
        self._orchestrator = <Domain>Orchestrator(proto)

    @property
    def aggregate(self) -> I<Domain>Aggregate:
        return self._orchestrator
```

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on. |
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule. |
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure. |
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting. |
| Verify block | Machine check (lint) + manual check (reader) with exact commands. |

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan modules/shared/src  # taxonomy + contract + utility
lint-arwaky-cli scan modules/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
python -m compileall -q modules/
```

A pass means naming, layer imports, primitives, and roles are clean. Migration strategy
(phase order, skip conditions) is **manual** — see Rules §1–2.

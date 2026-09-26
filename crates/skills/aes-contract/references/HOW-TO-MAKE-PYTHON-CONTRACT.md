# HOW TO MAKE CONTRACT PYTHON

> **Purpose**: State a Python contract's public promises (protocol or aggregate) so outer
> layers can depend on the seam without importing a concretion.
>
> **Audience**: Agents and engineers scaffolding AES contract files in the shared domain.
>
> **Scope**: Pure ABC definitions in `contract_<concept>_<suffix>.py` — `_protocol` or
> `_aggregate` only.
>
> **Location**: Shared domain package next to taxonomy, registered in the shared
> package `__init__.py`.
>
> **Length**: `_protocol` = one method per feature; `_aggregate` = every method the
> surface/root exports. Only methods outer layers actually call.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly `_protocol` or `_aggregate`.** Type names: `I<Name>Protocol`,
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.py`.
2. **ABC only — `@abstractmethod`, body is `...` or `pass`.** Never real code, never
 private-helper signatures, never convenience API (`AES101`/`AES102`).
3. **Protocol file = exactly ONE class + ONE method. Never more.** Each
 `contract_<concept>_protocol.py` declares a single ABC (`I<Name>Protocol`) with a
 single `@abstractmethod` for one feature. No second method, no helper methods, no
 second ABC/class in the same file — not even a small leaf/interface. A second
 feature is a second protocol *file* (`contract_<other>_protocol.py`), never a
 second class or method inside this one. Same shape for every capability in the
 domain. Only the `_aggregate` may hold many methods.
4. **Aggregate = many methods, one per exported consumer operation.** The aggregate is
 the **export surface**: every action the CLI/API/MCP/surface/root may call appears as its own method . Rich, typed,
 one row per export — not a single dump-all `execute()`.
5. **Allowed imports: taxonomy types and other contract types only.** Capabilities,
 agents, surface, root invert the dependency arrow (`AES201`/`AES205`).
6. **Signatures use shared VOs** — no `str`/`int`/`float`/`list[str]`/`dict` for domain
 values. `bool` allowed for semantic toggles only. All methods fully type-annotated;
 inherit `abc.ABC`.
7. **Register in shared `__init__.py`** with `__all__` + `_layer_symbols` for harness
 introspection.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, one method) or `_aggregate` (outward, many methods).
2. **Create file** → `contract_<concept>_<suffix>.py`.
3. **Draft protocol** — One class, one `@abstractmethod`, body is `...`.
4. **Draft aggregate** — One class, one method per export, body is `...`.
5. **Register** in shared `__init__.py` with `__all__` + `_layer_symbols`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol ABC — one method for one feature (uniform across capabilities)

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<domain>_vo import <VO>, <ResultVO>

class I<Name>Protocol(ABC):
    """Capability contract for one feature: <feature name> — <one sentence>."""

    @abstractmethod
    def <feature_method>(self, param: <VO>) -> <ResultVO>:
        """Run this one feature for *param*; return the result VO."""
        ...
```

**One file → one class → one method.** Violations: a second `@abstractmethod`, a
second `class` in the same file (including adapter/leaf ABCs), or an `execute(op, …)`
that dispatches multiple features behind one name. Each of those is a *new protocol
file* (or a second capability implementing the same one-method shape). Naming: match
the feature (`run`, `generate`, `install`, …).

### Aggregate ABC — many methods, one per export

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<domain>_vo import (
    ExitCode,
    <ArgsVO>,
    <QueryVO>,
    <ResultVO>,
)

class I<Name>Aggregate(ABC):
    """Export surface over <domain>: one method per consumer operation."""

    @abstractmethod
    def <export_1>(self, args: <ArgsVO>) -> ExitCode:
        """Export 1: <what the surface runs>. Return exit code."""
        ...

    @abstractmethod
    def <export_2>(self, query: <QueryVO>) -> <ResultVO>:
        """Export 2: <what the surface runs>. Return result VO/exit code."""
        ...

    @abstractmethod
    def <export_n>(self, ...) -> ExitCode:
        """Export n: … — add one method per new surface verb."""
        ...
```

**The aggregate is what gets exported to consumers.** Every public operation the
surface/root needs is its own method here; the agent implements them by dispatching to
one-method protocol capabilities. Adding a consumer verb = a new aggregate method +
the matching one-method protocol feature — not a flag on a mega `execute()`.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                               | Why it belongs here                                                            |
| ------------------------------------- | ------------------------------------------------------------------------------ |
| Module docstring (required)           | Names the contract's role: capability ABC or export/aggregate ABC.             |
| Suffix in file + class name           | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.               |
| Protocol: 1 file = 1 class = 1 method | Fan-out stays uniform; extra class/method → new protocol file, never this one. |
| Aggregate: one method / export        | Surface/root exports stay typed and discoverable; no dump-all entry.           |
| Abstract methods only                 | Outer layers depend on promises, not behaviour.                                |
| Shared VOs in signatures              | Domain values stay opaque across layers; no primitive leakage.                 |
| No impl-layer imports                 | Keeps the dependency arrow (capabilities → contract ← agent).                  |
| `__all__` + `_layer_symbols`          | Harness/loader introspection and explicit public surface.                      |
| Register in shared `__init__`         | Importable without reaching into private modules.                              |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked): protocol file = exactly 1 class + 1 method
# (no second ABC, no second method, no multi-feature execute dispatch);
# aggregate = one method per consumer export (many exports, not a single execute()).
# Fallback compile gate: python -c "import <shared_package>.contract_<concept>_<suffix>".
```


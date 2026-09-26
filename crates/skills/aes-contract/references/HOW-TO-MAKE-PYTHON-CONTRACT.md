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
> **Length**: One file per feature. A `_protocol` file may declare **one class per**  
> **capability seam**; each class is **rich** — one named method per operation, each with a  
> concrete return type. An `_aggregate` file declares **exactly one method**.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.py`.
2. **ABC only —** `@abstractmethod`**, body is** `...` **or** `pass`**.** Never real code, never  
 private-helper signatures, never convenience API (`AES101`/`AES102`).
3. **One file per feature, one class per capability seam, each class rich.** A  
 `_protocol` file for a feature with several capabilities declares one ABC per seam —  
 `I<Injector>Protocol`, `I<Sender>Protocol`, `I<Stream>Protocol` — side by side in the  
 same file. Each class carries **every** method its capability implements, each with its  
 own named signature and **one concrete return type**. Never `execute(op, …)` dispatch,  
 never a `**kwargs` bag, never a union return.
4. **A capability implements exactly one class, and all of it.** Because each class is  
 scoped to one capability's own operations, a class is never partially implemented: it  
 declares only the methods that capability owns, and that capability defines every one  
 of them. This is why a single file per feature is enough — no per-method splitting, and  
 no `NotImplementedError` stubs.
5. **Aggregate = exactly one method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol classes. A dump-all `execute(op, …)` aggregate is the violation here.
6. **Signatures use shared VOs.** Domain values must not be `str`, `int`, `float`,
   `list[str]`, or `dict` — wrap them in a taxonomy-defined VO before they appear in a
   signature. `bool` is permitted only for semantic toggles or predicates (e.g.
   `enabled: bool`), never for domain quantities. Enums are not banned; the ban targets
   primitive leakage. A multi-variant response enum on the aggregate is correct and
   expected — each variant must hold VO-wrapped values, not raw primitives. Protocol
   methods keep one concrete VO return type; the aggregate response enum is the
   sanctioned way to carry heterogeneous results across one `execute()` entry point.
   All methods fully type-annotated.
7. **Register in shared** `__init__.py` with `__all__` + `_layer_symbols` for harness  
 introspection.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich classes) or `_aggregate` (outward, one  
 method).
2. **List the feature's capability seams** — group the feature's operations by the  
 capability that owns them. One group becomes one class.
3. **Create file** → `contract_<concept>_<suffix>.py`, one class per seam.
4. **Draft each class** — every method its capability implements, one concrete return type  
 each.
5. **Register** in shared `__init__.py` with `__all__` + `_layer_symbols`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol file — one class per capability seam, each rich

```python
"""<Feature>-domain capability contracts (AES102 `_protocol`).

One file for the <feature> feature. Each class below is one capability
seam: a class carries every method that capability implements, with one
concrete return type each, so a capability implements its class outright
and never carries stubs.
"""

from abc import ABC, abstractmethod

from shared.<domain>.taxonomy_<domain>_vo import ResultVO, VO


class I<Seam1>Protocol(ABC):
    """<Seam 1> capability: <what it owns>."""

    @abstractmethod
    def <operation_1>(self, param: VO) -> ResultVO:
        """<What this operation does.> Return <what it returns>."""
        ...

    @abstractmethod
    def <operation_2>(self, page: PageVO, timeout: TimeoutVO) -> TextVO:
        """<What this operation does.> Return <what it returns>."""
        ...


class I<Seam2>Protocol(ABC):
    """<Seam 2> capability: <what it owns>."""

    @abstractmethod
    def <operation_1>(self, path: PathVO, content: TextVO) -> PathVO:
        """<What this operation does.> Return <what it returns>."""
        ...


class I<SeamN>Protocol(ABC):
    """<Seam N> capability: <what it owns>."""

    @abstractmethod
    def <operation_1>(self, ...) -> ResultVO:
        """<What this operation does.> Return <what it returns>."""
        ...


__all__ = ["I<Seam1>Protocol", "I<Seam2>Protocol", "I<SeamN>Protocol"]

_layer_symbols = {
    "I<Seam1>Protocol": I<Seam1>Protocol,
    "I<Seam2>Protocol": I<Seam2>Protocol,
    "I<SeamN>Protocol": I<SeamN>Protocol,
}
```

**One file → one feature → one class per capability seam → many named methods per class.**

Violations: an `execute(op, …)` that dispatches several features behind one name, a  
`**kwargs` bag, a union return that papers over differing result shapes, a class that  
declares methods its implementors do not own, or an implementor that leaves a declared  
method unimplemented (which Python rejects with `TypeError` at instantiation).

### Aggregate file — exactly one method

```python
"""<Feature>-domain aggregate contract (AES101 `_aggregate`).

The single entry point over the <feature> feature. Consumers pass a
request; the agent behind the aggregate dispatches to the rich protocol
classes in `contract_<feature>_protocol.py`.
"""

from abc import ABC, abstractmethod

from shared.<domain>.taxonomy_<domain>_vo import RequestVO, ResponseVO


class I<Feature>Aggregate(ABC):
    """Single entry point over the <feature> feature."""

    @abstractmethod
    def execute(self, request: RequestVO) -> ResponseVO:
        """Run the requested operation; return its response."""
        ...


__all__ = ["I<Feature>Aggregate"]

_layer_symbols = {"I<Feature>Aggregate": I<Feature>Aggregate}
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol  
classes' method lists; the agent behind the aggregate routes the request to the right  
capability. Adding a consumer verb = a new variant on `RequestVO` + the agent's dispatch  
branch, not a new aggregate method.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                                  | Why it belongs here                                                                |
| ---------------------------------------- | ---------------------------------------------------------------------------------- |
| Module docstring (required)              | Names the feature and the seams in the file.                                       |
| Suffix in file + class names             | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.                   |
| One file per feature                     | All seams of a feature live together; consumers import from one module.            |
| One class per capability seam            | A class lists only what its capability owns, so implementation is always complete. |
| Rich named methods, one return type each | Protocol classes: each method has one VO return; no
   dispatch bag. Aggregate: one `execute()` method whose response is a taxonomy-defined
   enum of VOs.     |
| Abstract methods only                    | Outer layers depend on promises, not behaviour.                                    |
| Aggregate: exactly one method            | Consumers depend on one stable entry point, not a shifting method list.            |
| Shared VOs in signatures                 | Domain values stay opaque across layers; no primitive leakage.                     |
| No impl-layer imports                    | Keeps the dependency arrow (capabilities → contract ← agent).                      |
| `__all__` + `_layer_symbols`             | Harness/loader introspection and explicit public surface.                          |
| Register in shared `__init__`            | Importable without reaching into private modules.                                  |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked):
#   - every method in every protocol class is named and individually typed
#     (no `execute(op, …)`, no `**kwargs`, no union return spanning differing shapes);
#   - each capability implements one class from this file, and implements all of it
#     (a partial implementation raises TypeError at instantiation — instantiate each
#     capability once to prove it);
#   - the aggregate declares exactly one `execute()` method; its response type is a
#     taxonomy-defined VO enum (each variant holds VO-wrapped values, not primitives).
# Fallback compile gate: python -c "import <shared_package>.contract_<concept>_<suffix>".
```


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
> **Length**: `_protocol` = every method the capabilities expose; `_aggregate` = one entry  
> point. Only methods outer layers actually call.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.py`.
2. **ABC only —** `@abstractmethod`**, body is** `...` **or** `pass`**.** Never real code, never  
 private-helper signatures, never convenience API (`AES101`/`AES102`).
3. **Protocol = rich, one method per capability operation.** The protocol declares every  
 operation its capabilities implement, each with its own named method and its own typed  
 signature. A capability implements the whole protocol. No `execute(op, …)` dispatch,  
 no `**kwargs` bag, no union return — every method has one concrete return type.
4. **Aggregate = ONE method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol. A dump-all `execute(op, …)` aggregate is the violation here, not a  
 rich one.
5. **Allowed imports: taxonomy types and other contract types only.** Capabilities,  
 agents, surface, root invert the dependency arrow (`AES201`/`AES205`).
6. **Signatures use shared VOs** — no `str`/`int`/`float`/`list[str]`/`dict` for domain  
 values. `bool` allowed for semantic toggles only. No union return types spanning  
 several value shapes; when operations genuinely differ in return type, they are  
 separate methods, not a union. All methods fully type-annotated; inherit `abc.ABC`.
7. **Register in shared** `__init__.py` with `__all__` + `_layer_symbols` for harness  
 introspection.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich) or `_aggregate` (outward, one method).
2. **Create file** → `contract_<concept>_<suffix>.py`.
3. **Draft protocol** — one class, one `@abstractmethod` per capability operation, each  
 with a concrete return type.
4. **Draft aggregate** — one class, one `@abstractmethod`, body is `...`.
5. **Register** in shared `__init__.py` with `__all__` + `_layer_symbols`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol ABC — rich, one named method per operation

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<domain>_vo import <VO>, <ResultVO>

class I<Name>Protocol(ABC):
    """Capability contract for <domain>: every operation the capabilities expose."""

    @abstractmethod
    def <operation_1>(self, param: <VO>) -> <ResultVO>:
        """<What this operation does.> Return <what it returns>."""
        ...

    @abstractmethod
    def <operation_2>(self, page: <PageVO>, timeout: <TimeoutVO>) -> <TextVO>:
        """<What this operation does.> Return <what it returns>."""
        ...

    @abstractmethod
    def <operation_n>(self, ...) -> <ResultVO>:
        """<Add one method per new capability operation.>"""
        ...
```

**One file → one class → many named methods.** Each method is a real, typed operation.  
Violations: an `execute(op, …)` that dispatches several features behind one name, a  
`**kwargs` bag, or a union return that papers over differing result shapes. A  
capability implements the whole protocol, so it never carries `NotImplementedError` stubs  
for operations it does not own — if it does not own them, it does not implement this  
protocol.

### Aggregate ABC — one method

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<domain>_vo import <RequestVO>, <ResponseVO>

class I<Name>Aggregate(ABC):
    """Single entry point over <domain>; the agent dispatches internally."""

    @abstractmethod
    def execute(self, request: <RequestVO>) -> <ResponseVO>:
        """Run the request the surface/root/CLI/MCP asked for; return the response."""
        ...
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol's  
method list; the agent behind the aggregate routes the request to the right capability  
operation. Adding a consumer verb = a new value on `<RequestVO>` + the agent's dispatch  
branch, not a new aggregate method.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                            | Why it belongs here                                                            |
| ---------------------------------- | ------------------------------------------------------------------------------ |
| Module docstring (required)        | Names the contract's role: capability ABC or entry-point ABC.                  |
| Suffix in file + class name        | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.               |
| Protocol: rich, named methods      | Each operation stays typed and discoverable; no dispatch bag, no union return. |
| Protocol: one concrete return type | Callers know the result shape without narrowing a union.                       |
| Aggregate: exactly one method      | Consumers depend on one stable entry point, not a shifting method list.        |
| Abstract methods only              | Outer layers depend on promises, not behaviour.                                |
| Shared VOs in signatures           | Domain values stay opaque across layers; no primitive leakage.                 |
| No impl-layer imports              | Keeps the dependency arrow (capabilities → contract ← agent).                  |
| `__all__` + `_layer_symbols`       | Harness/loader introspection and explicit public surface.                      |
| Register in shared `__init__`      | Importable without reaching into private modules.                              |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked): protocol methods are all named and individually typed
# (no `execute(op, …)`, no `**kwargs`, no union return spanning differing result shapes);
# aggregate declares exactly one method.
# Fallback compile gate: python -c "import <shared_package>.contract_<concept>_<suffix>".
```


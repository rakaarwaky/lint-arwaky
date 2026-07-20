# The 3-Block Structure

Every implementation file MUST follow this order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Domain Protocol Method Implementation**
3. **Block 3 — Dunder Methods, Factories, and Private Helpers**

## Block 1 — Class Definition & Constructor

```python
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __init__(self) -> None:
        pass
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol methods ONLY.

```python
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def execute(
        self,
        input: <DomainVO>,
        output: list[<ResultVO>],
    ) -> None:
        # domain behavior
        ...
```

Do NOT put these in Block 2: `__repr__`, `__str__`, `__eq__`, `__hash__`, `@classmethod create_default()`, `@staticmethod helpers`.

## Block 3 — Dunder Methods, Factories, and Helpers

```python
class Capabilities<NameCapability>(I<NameCapability>Protocol):
    def __repr__(self) -> str:
        return "Capabilities<NameCapability>()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, Capabilities<NameCapability>)

    @classmethod
    def create_default(cls) -> "Capabilities<NameCapability>":
        return cls()

    def _resolve_threshold(self, input: <DomainVO>) -> int:
        # private helper
        ...
```

Block 3 MUST NOT:

- define domain models (Entities, Value Objects) — that is **No Domain Definition** (ARCHITECTURE §8); consume them from Taxonomy instead.
- perform orchestration — no flow control across capabilities, no error-escalation policy (**No Orchestration**, ARCHITECTURE §8).
- duplicate technical mechanics that belong in a Utility standalone function (**DRY**, ARCHITECTURE §8).

## Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in capabilities)
  │
  ├─ Is it defined as @abstractmethod in the protocol ABC?
  │   └─ YES → Block 2
  │
  ├─ Is it a dunder method? (__repr__, __str__, __eq__, __hash__, __copy__)
  │   └─ YES → Block 3
  │
  ├─ Is it a factory classmethod? (create_default, from_config, from_dict)
  │   └─ YES → Block 3
  │
  ├─ Is it @staticmethod / @classmethod?
  │   ├─ Uses cls or class-level state?
  │   │   └─ YES → Block 3 (keep as @classmethod)
  │   ├─ Tightly coupled to class semantics?
  │   │   └─ YES → Block 3 (keep as @staticmethod)
  │   └─ Pure logic, no class dependency?
  │       └─ YES → EXTRACT to *_utility.py
  │
  └─ Is it a private helper using self?
      └─ YES → Block 3
```

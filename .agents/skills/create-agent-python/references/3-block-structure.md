# The 3-Block Structure

Every implementation file MUST follow this order with mandatory block markers:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Aggregate ABC Method Implementation**
3. **Block 3 — Dunder Methods, Factories, and Private Helpers**

Each block MUST be preceded by a block marker comment:

```python
# ─── Block 1: Class Definition & Constructor ───────────────
```

```python
# ─── Block 2: Aggregate Method Implementation ──────────────
```

```python
# ─── Block 3: Dunder Methods, Factories, Helpers ───────────
```

## Block 1 — Class Definition & Constructor

```python
# ─── Block 1: Class Definition & Constructor ───────────────
class <NameOrchestrator>(I<NameOrchestrator>Aggregate):
    def __init__(self, analyzer: I<NameAnalyzer>Protocol):
        self._analyzer = analyzer
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate ABC methods ONLY.

```python
# ─── Block 2: Aggregate Method Implementation ──────────────
class <NameOrchestrator>(I<NameOrchestrator>Aggregate):
    def execute(self, request: <ScanRequest>VO) -> list[<ResultVO>]:
        # orchestration only
        ...
```

Do NOT put `__repr__`, `__str__`, `__eq__`, `@classmethod create_default()`, `@staticmethod helpers` in Block 2.

## Block 3 — Dunder Methods, Factories, and Helpers

```python
# ─── Block 3: Dunder Methods, Factories, Helpers ───────────
class <NameOrchestrator>(I<NameOrchestrator>Aggregate):
    def __repr__(self) -> str:
        return "<NameOrchestrator>()"

    @classmethod
    def create_default(cls) -> "<NameOrchestrator>":
        return cls(analyzer=Capabilities<NameCapability>())

    def _should_skip_file(self, file: FilePath) -> bool:
        # private helper
        ...
```

## Method Placement Decision Rule

```text
Method / function found in an agent file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in agent)
  │
  ├─ Is it defined as @abstractmethod in the aggregate ABC?
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

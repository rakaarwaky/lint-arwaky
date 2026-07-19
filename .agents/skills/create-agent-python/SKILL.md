---
name: create-agent-python
description: "Create and validate agent layer files following AES rules: 3-block structure, one class per file, aggregate contracts, zero computation/I/O/business logic."
version: 1.1.0
category: refactoring
tags:
  [
    python,
    aes,
    agent,
    aggregate,
    structure,
    3-block-structure,
    di,
    orchestration,
    utility-extraction,
  ]
triggers:
  - "create agent python"
  - "add agent python"
  - "fix agent structure python"
  - "create aggregate python"
  - "agent missing aggregate python"
  - "validate agent logic python"
  - "extract utility python"
  - "free function python"
dependencies: []
related:
  - create-capabilities-python
  - create-infrastructure-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-agent-di-python
---

# create-agent-python

## Purpose

Create and validate Python **agent layer** files following clean architecture rules. Ensures agents contain zero computation, zero I/O, and zero business logic — they are orchestration/pipeline execution only. Agents inherit aggregate ABCs, follow the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Agent Layer (`agent_*.py`)**

| Allowed                                          | Forbidden                                        |
| ------------------------------------------------ | ------------------------------------------------ |
| `for`, `while`, `async for` (orchestration flow) | Computation (`sum()`, `len()`, arithmetic)       |
| `if/else`, `elif`, `match` (control flow)        | Business rules, domain logic                     |
| `try/except`, `raise` (error propagation)        | File I/O (`open()`, `Path()`, `os.`)             |
| `asyncio.wait_for`, `.sleep` (async)             | Network (`requests.`, `httpx.`)                  |
| Sequential statements (orchestration)            | Database (`sqlite3.`, `asyncpg.`)                |
| Protocol/ABC implementation                      | Domain model definition (`@dataclass`)           |
|                                                  | Direct import from `capabilities_*`              |
|                                                  | Direct import from `infrastructure_*`            |

### Structural Rules (All Layers)

- **1 file = 1 class** — each agent file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive aggregate interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`, no `cls`) MUST be extracted to `*_utility.py` modules in shared/taxonomy
- **No module-level `def` in agent files** — free functions outside the class are forbidden; extract to `*_utility.py`

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `class <Type>(I<Name>Aggregate):` declaration
   - `__init__` with DI fields (aggregate interfaces)

2. **Block 2 — Aggregate Methods** (Public Contract)
   - Methods that implement the aggregate ABC's `@abstractmethod` signatures.
   - Contains **ONLY** the domain aggregate methods.
   - **NO** dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`, etc.) here.
   - **NO** factory classmethods (`create_default`, `from_config`) here.
   - **NO** `@staticmethod` helpers here.

3. **Block 3 — Dunder Methods, Factories & Helpers**
   - Dunder/utility methods: `__repr__`, `__str__`, `__eq__`, `__hash__`, `__copy__`, etc.
   - Factory classmethods: `create_default()`, `from_config()`, `from_dict()`
   - `@staticmethod` and `@classmethod` helpers **that depend on class semantics**
   - Private helper methods (`_helper_name`) that use `self`

**CRITICAL:** Block 2 is **RESERVED** for domain aggregate methods ONLY. Dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`) and factory classmethods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `self`, no `cls`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in agent files.

#### Method Placement Decision Rule

```
Method / function found in an agent file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in agent)
  │
  ├─ Is it defined as @abstractmethod in the aggregate ABC?
  │   └─ YES → Block 2 (Aggregate Methods)
  │
  ├─ Is it a dunder method? (__repr__, __str__, __eq__, __hash__, __copy__)
  │   └─ YES → Block 3 (Dunder Methods & Helpers)
  │
  ├─ Is it a factory classmethod? (create_default, from_config, from_dict)
  │   └─ YES → Block 3 (Dunder Methods & Helpers)
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
      └─ YES → Block 3 (Private Helpers)
```

#### Example: Correct 3-Block Order

```python
from shared.orphan_detector.contract_orphan_protocol import ICapabilitiesOrphanProtocol
from shared.orphan_detector.contract_orphan_aggregate import IOrphanOrchestratorAggregate
from shared.code_analysis.taxonomy_result_vo import LintResult


# ─── Block 1: Class Definition & Constructor ──────────────
class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    """Orchestrates orphan detection across a set of files."""

    def __init__(self, analyzer: ICapabilitiesOrphanProtocol) -> None:
        self._analyzer = analyzer


    # ─── Block 2: Aggregate Methods (domain contract ONLY) ─
    def execute(self, files: list[str]) -> list[LintResult]:
        violations: list[LintResult] = []
        for file in files:
            try:
                result = self._analyzer.analyze(file)
                violations.append(result)
            except Exception as e:
                violations.append(LintResult.new_arch(
                    file, 0, "ANALYZE_ERROR", str(e),
                ))
        return violations


    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "OrphanOrchestrator()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, OrphanOrchestrator)

    @classmethod
    def create_default(cls) -> "OrphanOrchestrator":
        from shared.orphan_detector.capabilities_orphan_analyzer import CapabilitiesOrphanAnalyzer
        return cls(analyzer=CapabilitiesOrphanAnalyzer())
```

#### Example: Extracted Utility Module

```python
# shared/orphan_detector/taxonomy_orphan_utility.py
"""Stateless utility functions for orphan detection logic."""

ORPHAN_MARKERS: tuple[str, ...] = ("orphan", "deprecated", "unused")


def is_orphan_candidate(filename: str) -> bool:
    """Check if a filename matches orphan naming patterns."""
    lower = filename.lower()
    return any(marker in lower for marker in ORPHAN_MARKERS)


def format_violation_message(code: str, detail: str) -> str:
    """Format a standardized violation message."""
    return f"[{code}] {detail}"
```

### Aggregate Rules

- **Every agent class MUST inherit from an aggregate ABC**
- **Aggregate MUST define abstract methods for all public methods**
- **Aggregate contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod`, private `_methods`)
- **Constructors in Block 1** — `__init__` receives aggregate interfaces
- **Dunder methods (`__repr__`, `__str__`, `__eq__`, etc.) in Block 3**
- **Factory classmethods (`create_default`, `from_config`) in Block 3**
- **Stateless `@staticmethod` (no class dependency) → extract to `*_utility.py`**

## The Fundamental Question

> **"Is this file orchestration/pipeline execution only?"**

If yes → **`agent_*.py` + inherit aggregate ABC**
If no (has computation, I/O, or business logic) → **split into appropriate layer**

> **"Does this function need the class?"**

If no (`self`/`cls` unused) → **extract to `*_utility.py` in shared/taxonomy**
If yes → **keep in Block 3**

## Naming Convention

| Layer                    | File Pattern            | Protocol File                    | Protocol Name        |
| ------------------------ | ----------------------- | -------------------------------- | -------------------- |
| **Capabilities**   | `capabilities_*.py`   | `contract_<name>_protocol.py`  | `I<Name>Protocol`  |
| **Infrastructure** | `infrastructure_*.py` | `contract_<name>_port.py`      | `I<Name>Port`      |
| **Agents**         | `agent_*.py`          | `contract_<name>_aggregate.py` | `I<Name>Aggregate` |
| **Utility**        | `taxonomy_<name>_utility.py` | —                         | — (free functions) |

## Agent Layer Purpose

Agents are the **orchestration layer** — they coordinate between capabilities and infrastructure but contain:

- **NO computation** (no arithmetic, no data transformation)
- **NO business logic** (no domain rules, no validation)
- **NO I/O** (no file reads, no network calls, no database queries)

Their sole purpose is to orchestrate pipeline execution by calling into capabilities and infrastructure.

## Detection Patterns

### BAD: Computation in Agent

```python
# BAD: Computation in agent layer
class OrphanOrchestrator:
    def process(self):
        total = len(self.files)  # ← COMPUTATION — should be in capabilities
        sum_val = sum(f.size for f in self.files)  # ← FORBIDDEN
```

### BAD: Business Logic in Agent

```python
# BAD: Domain logic in agent layer
class OrphanOrchestrator:
    def analyze(self, content: str) -> bool:
        return "orphan" in content  # ← BUSINESS RULE — should be in capabilities
```

### BAD: Dataclass in Layer File

```python
# BAD: Domain data defined in agent layer
@dataclass
class OrphanReport:  # ← DATA CLASS — should be in shared/taxonomy
    results: list[str]
    timestamp: int

class OrphanOrchestrator:
    report: OrphanReport  # ← concrete type, not DI
```

### BAD: Dunder Methods in Block 2

```python
# BAD: __repr__ / __eq__ mixed in with aggregate methods
class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    def __init__(self, analyzer): ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT an aggregate method
        return "OrphanOrchestrator()"

    def execute(self, files): ...        # ← pushed down

    def __eq__(self, other) -> bool:     # ← also in Block 2 position
        return isinstance(other, OrphanOrchestrator)
```

### BAD: Module-Level Free Function in Agent File

```python
# BAD: Free function outside class in agent file
# agent_orphan_orchestrator.py

def is_orphan_candidate(filename: str) -> bool:  # ← FREE FUNCTION — extract to utility
    return "orphan" in filename.lower()

def format_violation_message(code: str, detail: str) -> str:  # ← FREE FUNCTION
    return f"[{code}] {detail}"

class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    def execute(self, files):
        if is_orphan_candidate(files[0]):  # ← could be imported from utility
            ...
```

### BAD: Stateless @staticmethod That Should Be Extracted

```python
# BAD: @staticmethod with zero class dependency — belongs in utility
class OrphanOrchestrator(IOrphanOrchestratorAggregate):

    @staticmethod
    def is_orphan_candidate(filename: str) -> bool:  # ← no self, no cls, pure logic
        return "orphan" in filename.lower()

    @staticmethod
    def format_message(code: str, detail: str) -> str:  # ← no self, no cls, pure logic
        return f"[{code}] {detail}"

    def execute(self, files):
        if self.is_orphan_candidate(files[0]):  # ← could be a free function
            ...
```

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use aggregates
from contract_orphan_protocol import ICapabilitiesOrphanProtocol

class OrphanOrchestrator:
    def __init__(self, analyzer: ICapabilitiesOrphanProtocol):
        self._analyzer = analyzer  # ← DI via aggregate
```

### GOOD: Correct 3-Block with Dunder Methods

```python
# GOOD: Aggregate methods in Block 2, dunders + factories in Block 3
class OrphanOrchestrator(IOrphanOrchestratorAggregate):

    def __init__(self, analyzer: ICapabilitiesOrphanProtocol) -> None:  # Block 1: constructor
        self._analyzer = analyzer

    def execute(self, files: list[str]) -> list[LintResult]:  # Block 2: aggregate method ONLY
        ...

    def __repr__(self) -> str:               # Block 3: dunder = utility
        return "OrphanOrchestrator()"

    @classmethod
    def create_default(cls) -> "OrphanOrchestrator":  # Block 3: factory
        ...
```

### GOOD: Extracted to Taxonomy Utility

```python
# GOOD: shared/orphan_detector/taxonomy_orphan_utility.py

ORPHAN_MARKERS: tuple[str, ...] = ("orphan", "deprecated", "unused")

def is_orphan_candidate(filename: str) -> bool:
    """Check if a filename matches orphan naming patterns."""
    lower = filename.lower()
    return any(marker in lower for marker in ORPHAN_MARKERS)
```

```python
# GOOD: agent_orphan_orchestrator.py (consumer)

from shared.orphan_detector.taxonomy_orphan_utility import is_orphan_candidate

class OrphanOrchestrator(IOrphanOrchestratorAggregate):

    def execute(self, files: list[str]) -> list[LintResult]:
        if is_orphan_candidate(files[0]):    # ← imported from utility
            ...
```

## Workflow

### Step 1: Analyze File

Read file and check for prohibited content. Ask: **"Is this orchestration only?"**

- If it has computation → **MOVE to Capabilities**
- If it has I/O or business logic → **split into appropriate layer**
- If pure orchestration → continue to Step 2

### Step 2: Check for Missing Aggregate

Does the agent class inherit from an aggregate ABC? If no → create one.

```bash
# Find agents without aggregate inheritance
grep -rn "^class " modules/*/src/agent_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done
```

### Step 3: Create Aggregate File (if missing)

Create `contract_<name>_aggregate.py` in the shared module with abstract methods.

**Aggregate location:**

| Module     | Aggregate Path                                            |
| ---------- | --------------------------------------------------------- |
| compositor | `modules/shared/src/compositor/contract_*_aggregate.py` |
| animator   | `modules/shared/src/animator/contract_*_aggregate.py`   |
| scripting  | `modules/shared/src/scripting/contract_*_aggregate.py`  |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `class <Type>(I<Name>Aggregate):` + `__init__` (class definition with DI fields)
2. Aggregate `@abstractmethod` implementations (**domain aggregate methods ONLY**)
3. Dunder methods (`__repr__`, `__str__`, `__eq__`), factory classmethods (`create_default`, `from_config`), `@staticmethod`/`@classmethod` helpers that use class state, and private `_helpers`

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use aggregates** — constructor receives aggregate interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `self`/`cls` dependency**:

```bash
# Find module-level functions (outside class) — ALWAYS forbidden
grep -n "^def " modules/*/src/agent_*.py

# Find @staticmethod inside class (no self, no cls)
grep -B1 -A5 "@staticmethod" modules/*/src/agent_*.py

# Find @classmethod that doesn't use cls
grep -B1 -A5 "@classmethod" modules/*/src/agent_*.py
```

For each candidate, ask:

| Question | YES → | NO → |
|----------|-------|------|
| Uses `self` or instance state? | Keep in Block 3 | Continue ↓ |
| Uses `cls` or class-level state? | Keep as `@classmethod` in Block 3 | Continue ↓ |
| Tightly coupled to class semantics (e.g., references class constants)? | Keep as `@staticmethod` in Block 3 | Continue ↓ |
| Pure logic, deterministic, no side effects? | **Extract to `*_utility.py`** | Keep in Block 3 |
| Domain-agnostic (not specific to this class)? | **Extract to `*_utility.py`** | Keep in Block 3 |

**Extraction process:**

1. Create `modules/shared/src/<domain>/taxonomy_<name>_utility.py`
2. Move function(s) to utility file with docstrings
3. Extract magic constants to `taxonomy_<name>_constant.py` if needed
4. Add import in agent file: `from shared.<domain>.taxonomy_<name>_utility import func_name`
5. Remove original function from agent file
6. Register utility module in `__init__.py` if needed
7. Verify: `python -c "from shared.<domain>.taxonomy_<name>_utility import *"`

### Step 7: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for computation in agents
grep -n "sum(\|len(\|\.iter\(\)|\.map(" modules/*/src/agent_*.py

# Check for forbidden imports
grep -n "capabilities_\|infrastructure_" modules/*/src/agent_*.py
```

### Step 8: Verify

Run syntax check to confirm no violations.

```bash
python -c "import <module>"
```

## Import Strategy

When deciding where a function belongs:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use when the code is **stateless, pure logic** with no side effects:

| Condition                                     | Example                                       |
| --------------------------------------------- | --------------------------------------------- |
| No `self`, no `cls`, no class state           | `is_orphan_candidate(name: str) -> bool`      |
| All data via parameters                       | `format_violation_message(code, detail) -> str` |
| Deterministic, no side effects                | `normalize_filename(name: str) -> str`        |

```python
# taxonomy_orphan_utility.py (SHARED / TAXONOMY)
def is_orphan_candidate(filename: str) -> bool:
    return "orphan" in filename.lower()

# agent_orphan_orchestrator.py (CONSUMER)
from shared.orphan_detector.taxonomy_orphan_utility import is_orphan_candidate
```

### Option B: Keep as Instance/Class Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `self` / instance fields | `self._analyzer.analyze(file)`                  |
| Uses `cls` / class-level config | `cls._registry[name]`                         |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level constants or types  |

```python
# agent_orphan_orchestrator.py (STAYS IN CLASS — Block 3)
class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    _MAX_RETRIES = 3  # class-level constant

    def __init__(self, analyzer: ICapabilitiesOrphanProtocol):
        self._analyzer = analyzer

    def _should_retry(self, attempt: int) -> bool:  # uses self → stays
        return attempt < self._MAX_RETRIES

    @classmethod
    def from_config(cls, config: IOrchestratorConfigProtocol) -> "OrphanOrchestrator":  # uses cls → stays
        return cls(analyzer=config.create_analyzer())
```

### Decision Tree

```
Function found in agent file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in agent)
  │
  ├─ @staticmethod inside class?
  │   ├─ Pure logic, no class dependency?
  │   │   └─ YES → EXTRACT to *_utility.py
  │   └─ Tightly coupled to class semantics?
  │       └─ YES → Keep as @staticmethod in Block 3
  │
  ├─ @classmethod?
  │   ├─ Factory (create_default, from_config)?
  │   │   └─ YES → Keep in Block 3
  │   ├─ Uses cls or class-level state?
  │   │   └─ YES → Keep in Block 3
  │   └─ Pure logic, no cls dependency?
  │       └─ YES → Convert to free fn → EXTRACT to *_utility.py
  │
  ├─ Instance method (self)?
  │   ├─ Defined in aggregate ABC?
  │   │   └─ YES → Block 2
  │   └─ Private helper?
  │       └─ YES → Block 3
  │
  └─ Dunder method (__repr__, __eq__, etc.)?
      └─ YES → Block 3
```

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class + `__init__` → Aggregate Methods → Dunders/Factories/Helpers).
- [ ] **Block 2 contains ONLY aggregate `@abstractmethod` implementations**. No dunder methods, no `@staticmethod`, no factory classmethods in Block 2.
- [ ] **Dunder methods** (`__repr__`, `__str__`, `__eq__`, `__hash__`) and **factory classmethods** (`create_default`, `from_config`) are in **Block 3**.
- [ ] Agent class inherits an aggregate ABC.
- [ ] Aggregate contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive aggregate interfaces via `__init__`.
- [ ] **No module-level `def` functions** exist outside the class in agent files.
- [ ] **No stateless `@staticmethod`** (zero class dependency) remains in class — extracted to `*_utility.py`.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use aggregate interfaces, not concrete types.
- [ ] **Zero computation** in agent layer (no sum(), no len(), no iteration transforms).
- [ ] **Zero I/O** in agent layer (no open(), no network, no database).
- [ ] **Zero business logic** in agent layer (no domain rules, no validation).
- [ ] No forbidden imports (no capabilities\_\_, no infrastructure\_\_).
- [ ] Aggregate module is registered in the shared module's `__init__.py`.
- [ ] Utility module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    def \|^    @" modules/<module>/src/agent_*.py

# Find agents without aggregate inheritance
grep -rn "^class " modules/*/src/agent_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done

# Ensure aggregate does NOT contain helper methods
grep -E "def (helper|util|private|_)" modules/shared/src/contract_*_aggregate.py || echo "Clean: No helpers in aggregate"

# Check for computation in agents
grep -n "sum(\|len(\|\.iter\(\)|\.map(" modules/*/src/agent_*.py

# Check for I/O in agents
grep -n "open(\|Path(\|os\." modules/*/src/agent_*.py

# Check for business logic in agents
grep -n "is_orphan\|analyze\|validate" modules/*/src/agent_*.py

# Check for dataclasses defined in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | grep agent

# Check for concrete type fields (non-aggregate)
grep -n "__init__" modules/*/src/agent_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-AGGREGATE FIELD: $file"
done

# Find module-level free functions in agent files (ALWAYS forbidden)
grep -n "^def [a-z_]*(" modules/*/src/agent_*.py

# Find @staticmethod that may need extraction (no self, no cls)
grep -B1 -A10 "@staticmethod" modules/*/src/agent_*.py

# Detect dunder methods appearing BEFORE aggregate methods (wrong block order)
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_agg = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_agg is None:
            first_agg = i + 1
    if first_dunder and first_agg and first_dunder < first_agg:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before aggregate method (line {first_agg})')
" modules/*/src/agent_*.py

# Find standalone functions in class files (should be extracted to utility)
grep -n "^def [a-z_]*(\s*[^self])" modules/*/src/agent_*.py

# Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Putting computation in agents**: Arithmetic, `sum()`, `len()`, and data transformation MUST be in capabilities layer.
- ❌ **Putting I/O in agents**: File reads, network calls, and database queries MUST be in infrastructure layer.
- ❌ **Putting business logic in agents**: Domain rules, validation, and computation MUST be in capabilities layer.
- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive aggregate interfaces, not concrete implementations.
- ❌ **Putting helper methods in the aggregate**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave aggregate methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions (no `self`) MUST be extracted to standalone `*_utility.py` modules.
- ❌ **Creating "God Aggregates"**: If an aggregate has >10 methods or mixes unrelated concerns, split it into multiple aggregates.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-python` if merging multiple files.
- ❌ **Placing dunder methods (`__repr__`, `__str__`, `__eq__`) in Block 2**: Block 2 is RESERVED for aggregate method implementations ONLY. Dunders are utilities and belong in Block 3.
- ❌ **Placing factory classmethods (`create_default`, `from_config`) before aggregate methods**: Factories are constructors and belong in Block 3, after aggregate methods.
- ❌ **Mixing `__init__` into Block 2**: `__init__` is part of Block 1 (class definition & constructor), not an aggregate method.
- ❌ **Leaving module-level `def` in agent files**: Free functions outside the class MUST be extracted to `*_utility.py` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `@staticmethod` in class**: If a `@staticmethod` uses no `self`, no `cls`, and no class-level state, it belongs in `*_utility.py`, not in the class body.

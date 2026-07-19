---
name: create-capabilities-python
description: "Create and validate capabilities layer files following AES rules: 3-block structure, one class per file, protocol contracts, zero I/O."
version: 1.2.0
category: refactoring
tags:
  [
    python,
    aes,
    capability,
    protocol,
    structure,
    aes403,
    aes404,
    3-block-structure,
    di,
    utility-extraction,
  ]
triggers:
  - "create capability python"
  - "add capability python"
  - "fix capability structure python"
  - "create protocol python"
  - "capability missing protocol python"
  - "check capabilities python"
  - "extract utility python"
  - "free function python"
dependencies: []
related:
  - create-infrastructure-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-capability-structure-python
  - create-missing-protocols-python
---
# create-capabilities-python

## Purpose

Create and validate Python **capabilities layer** files following clean architecture rules. Ensures capabilities contain zero I/O, inherit protocol ABCs, follow the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Capabilities Layer (`capabilities_*.py`)**

| Allowed                               | Forbidden                                        |
| ------------------------------------- | ------------------------------------------------ |
| Computation, validation, calculation  | File I/O (`open()`, `Path()`, `os.`)       |
| Data transformation, business rules   | Network calls (`requests.`, `httpx.`)        |
| Domain logic, domain model definition | Database operations (`sqlite3.`, `asyncpg.`) |
| Protocol/ABC implementation           | Direct import from `infrastructure_*`          |
|                                       | Direct import from `agent_*`                   |
|                                       | Direct import from `capabilities_*` (self)     |

### Structural Rules (All Layers)

- **1 file = 1 class** — each capabilities file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`, no `cls`) MUST be extracted to `*_utility.py` modules in shared/taxonomy
- **No module-level `def` in capabilities files** — free functions outside the class are forbidden; extract to `*_utility.py`

### Helper vs Utility Decision (The Ultimate Boundary)

The boundary is not just about `self`/`cls`. It is about **Domain Knowledge (The Rules) vs. Dumb Tools (The Mechanics)**.

> **The Litmus Test:** "Does this function know about specific business rules (e.g., AES violations, layer mappings), or is it just a blind data manipulator?"

#### 🟢 Keep as Private Helper in Capabilities (Block 3)
Keep if the function contains **Domain Knowledge** or meets ANY of these:
1. **Contains Business Rules**: Knows about specific system rules (e.g., knows that `_port` suffix means `infrastructure_` layer, or knows specific domain violation codes).
2. **Needs Instance State**: Accesses `self.field` or class variables.
3. **Tightly Coupled**: Logic is specific to this checker only and doesn't make sense elsewhere (e.g., formatting error messages that reference this class name).
4. **Factory Method**: `create_default()`, `from_config()`, `from_dict()` — specific to instantiating this class.
5. **Single-Domain Only**: Function only serves ONE feature/domain and won't be reused elsewhere.

#### 🔴 Extract to Utility (`*_utility.py`)
Extract ONLY if the function is a **Dumb Tool** and meets ALL of these:
1. **Stateless**: No `self`, no `cls`, no class-level state access.
2. **Pure Function**: Input A always produces output B. No side effects (no I/O, no random, no global state mutation).
3. **Domain-Agnostic / Reusable**: Logic is general enough that *multiple* checkers/features could use it (e.g., regex matching, string normalization, AST parsing). It doesn't know *what* it's checking, only *how* to check.
4. **Multi-Domain Reusable**: Function serves multiple features/domains, not just one.

#### I/O Blocker (CRITICAL)

A function can be stateless but STILL **cannot** be extracted to taxonomy if it has I/O:

- `open()`, `read()`, `write()`, `pathlib.Path.read_text()`
- `requests`, `urllib`, `httpx` (network)
- `sqlite3`, `psycopg2`, `sqlalchemy` (database)

**Rule:** Stateless + I/O = Keep in layer (or move to infrastructure), **NOT** taxonomy utility.

```python
def has_crate_self_import(file_path: str) -> bool:
    # Stateless ✓ (no self, no cls)
    # But uses open() / read_dir ✗ (I/O)
    # → CANNOT extract to taxonomy utility
    # → Keep in capabilities layer
```

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `class <Type>(I<Name>Protocol):` declaration
   - `__init__` with DI fields (protocol interfaces)

2. **Block 2 — Protocol Methods** (Public Contract)
   - Methods that implement the protocol ABC's `@abstractmethod` signatures.
   - Contains **ONLY** the domain protocol methods.
   - **NO** dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`, etc.) here.
   - **NO** factory classmethods (`create_default`, `from_config`) here.
   - **NO** `@staticmethod` helpers here.

3. **Block 3 — Dunder Methods, Factories & Helpers**
   - Dunder/utility methods: `__repr__`, `__str__`, `__eq__`, `__hash__`, `__copy__`, etc.
   - Factory classmethods: `create_default()`, `from_config()`, `from_dict()`
   - `@staticmethod` and `@classmethod` helpers **that depend on class semantics**
   - Private helper methods (`_helper_name`) that use `self`

**CRITICAL:** Block 2 is **RESERVED** for domain protocol methods ONLY. Dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`) and factory classmethods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `self`, no `cls`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in capabilities files.

#### Method Placement Decision Rule

```
Method / function found in a capabilities file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in capabilities)
  │
  ├─ Is it defined as @abstractmethod in the protocol ABC?
  │   └─ YES → Block 2 (Protocol Methods)
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
from shared.code_analysis.taxonomy_result_vo import LintResult
from shared.code_analysis.taxonomy_severity_vo import Severity
from shared.code_analysis.contract_line_protocol import ILineCheckerProtocol
from shared.code_analysis.taxonomy_line_checker_utility import (
    is_barrel_file,
    count_lines,
)
from shared.taxonomy_definition_vo import LayerDefinition


# ─── Block 1: Class Definition & Constructor ──────────────
class ArchLineChecker(ILineCheckerProtocol):
    """Checks file line counts against layer-defined min/max thresholds."""

    def __init__(self) -> None:
        pass  # stateless — no DI fields needed


    # ─── Block 2: Protocol Methods (domain contract ONLY) ─
    def check_line_counts(
        self,
        file: str,
        definition: LayerDefinition | None,
        content: str,
        violations: list[LintResult],
    ) -> None:
        basename = Path(file).name

        if is_barrel_file(basename):
            return

        if definition is None:
            return

        if basename in definition.exceptions.values:
            return

        count = count_lines(content)

        if definition.code_analysis.min_lines.value > 0 and count < definition.code_analysis.min_lines.value:
            violations.append(LintResult.new_arch(
                file, 0, "AES302", Severity.HIGH,
                f"File too short (min: {definition.code_analysis.min_lines.value}).",
            ))

        if definition.code_analysis.max_lines.value > 0 and count > definition.code_analysis.max_lines.value:
            violations.append(LintResult.new_arch(
                file, 0, "AES301", Severity.HIGH,
                f"File too large (max: {definition.code_analysis.max_lines.value}).",
            ))


    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "ArchLineChecker()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, ArchLineChecker)

    @classmethod
    def create_default(cls) -> "ArchLineChecker":
        return cls()
```

#### Example: Extracted Utility Module

```python
# shared/code_analysis/taxonomy_line_checker_utility.py
"""Stateless utility functions for line-count checking logic."""

BARREL_FILES: tuple[str, ...] = ("__init__.py", "mod.rs")


def is_barrel_file(basename: str) -> bool:
    """Check if a filename is a barrel/module index file."""
    return basename in BARREL_FILES


def count_lines(content: str) -> int:
    """Count the number of lines in content string."""
    return len(content.splitlines())


def normalize_path(path: str) -> str:
    """Normalize a file path for comparison."""
    return path.strip().lower()
```

### Protocol Rules

- **Every capability class MUST inherit from a protocol ABC** (AES403)
- **Protocol MUST define abstract methods for all public methods**
- **Protocol contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod`, private `_methods`)
- **Constructors in Block 1** — `__init__` receives protocol interfaces
- **Dunder methods (`__repr__`, `__str__`, `__eq__`, etc.) in Block 3**
- **Factory classmethods (`create_default`, `from_config`) in Block 3**
- **Stateless `@staticmethod` (no class dependency) → extract to `*_utility.py`**

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes → **`capabilities_*.py` + inherit protocol ABC**
If no (has I/O) → **split into infrastructure layer instead**

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

## Detection Patterns

### BAD: Capability Without Protocol (AES403)

```python
# BAD: No protocol inheritance
class FrameComposer:
    def compose_frame(self): ...
```

### BAD: Mixed Logic in Capabilities

```python
# BAD: I/O in capabilities layer
class MyCapability:
    def process(self):
        with open("file.txt") as f:  # ← FORBIDDEN
            content = f.read()
```

### BAD: Dataclass in Layer File

```python
# BAD: Domain data defined in capabilities layer
@dataclass
class OrphanResult:  # ← DATA CLASS — should be in shared/taxonomy
    is_orphan: bool
    reason: str
    severity: str

class CapabilitiesOrphanAnalyzer:
    result: OrphanResult  # ← concrete type, not DI
```

### BAD: Dunder Methods in Block 2

```python
# BAD: __repr__ / __eq__ mixed in with protocol methods
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self): ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a protocol method
        return "ArchLineChecker()"

    def check_line_counts(self, ...) -> None:  # ← pushed down
        ...

    def __eq__(self, other) -> bool:     # ← also in Block 2 position
        return isinstance(other, ArchLineChecker)
```

### BAD: Module-Level Free Function in Capabilities File

```python
# BAD: Free function outside class in capabilities file
# capabilities_line_checker.py

def normalize_path(path: str) -> str:       # ← FREE FUNCTION — extract to utility
    return path.strip().lower()

def count_lines(content: str) -> int:       # ← FREE FUNCTION — extract to utility
    return len(content.splitlines())

class ArchLineChecker(ILineCheckerProtocol):
    def check_line_counts(self, ...):
        normalized = normalize_path(file)
        ...
```

### BAD: Stateless @staticmethod That Should Be Extracted

```python
# BAD: @staticmethod with zero class dependency — belongs in utility
class ArchLineChecker(ILineCheckerProtocol):

    @staticmethod
    def normalize_path(path: str) -> str:   # ← no self, no cls, pure logic
        return path.strip().lower()

    @staticmethod
    def is_barrel_file(name: str) -> bool:  # ← no self, no cls, pure logic
        return name in ("__init__.py", "mod.rs")

    def check_line_counts(self, ...):
        if self.is_barrel_file(basename):   # ← could be a free function
            return
```

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use protocols
from shared.code_analysis.taxonomy_analysis import OrphanIndicatorResult
from contract_orphan_protocol import IOrphanFilenameExtractorProtocol

class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor  # ← DI via protocol
```

### GOOD: Correct 3-Block with Dunder Methods

```python
# GOOD: Protocol methods in Block 2, dunders + factories in Block 3
class ArchLineChecker(ILineCheckerProtocol):

    def __init__(self) -> None:              # Block 1: constructor
        pass

    def check_line_counts(self, ...) -> None:  # Block 2: protocol method ONLY
        ...

    def __repr__(self) -> str:               # Block 3: dunder = utility
        return "ArchLineChecker()"

    @classmethod
    def create_default(cls) -> "ArchLineChecker":  # Block 3: factory
        return cls()
```

### GOOD: Extracted to Taxonomy Utility

```python
# GOOD: shared/code_analysis/taxonomy_line_checker_utility.py

BARREL_FILES: tuple[str, ...] = ("__init__.py", "mod.rs")

def is_barrel_file(basename: str) -> bool:
    """Check if a filename is a barrel/module index file."""
    return basename in BARREL_FILES

def count_lines(content: str) -> int:
    """Count the number of lines in content string."""
    return len(content.splitlines())
```

```python
# GOOD: capabilities_line_checker.py (consumer)

from shared.code_analysis.taxonomy_line_checker_utility import (
    is_barrel_file,
    count_lines,
)

class ArchLineChecker(ILineCheckerProtocol):

    def check_line_counts(self, ...):
        if is_barrel_file(basename):        # ← imported from utility
            return
        count = count_lines(content)        # ← imported from utility
        ...
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has I/O → **MOVE to Infrastructure** (AES404)
- If pure business logic → continue to Step 2

### Step 2: Check for Missing Protocol (AES403)

Does the capability class inherit from a protocol ABC? If no → create one.

```bash
# Find capabilities without protocol inheritance
grep -rn "^class " modules/*/src/capabilities_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Protocol" "$file" || echo "MISSING: $file has $class without protocol"
done
```

### Step 3: Create Protocol File (if missing)

Create `contract_<name>_protocol.py` in the shared module with abstract methods.

**Protocol location:**

| Module     | Protocol Path                                            |
| ---------- | -------------------------------------------------------- |
| compositor | `modules/shared/src/compositor/contract_*_protocol.py` |
| animator   | `modules/shared/src/animator/contract_*_protocol.py`   |
| scripting  | `modules/shared/src/scripting/contract_*_protocol.py`  |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `class <Type>(I<Name>Protocol):` + `__init__` (class definition with DI fields)
2. Protocol `@abstractmethod` implementations (**domain protocol methods ONLY**)
3. Dunder methods (`__repr__`, `__str__`, `__eq__`), factory classmethods (`create_default`, `from_config`), `@staticmethod`/`@classmethod` helpers that use class state, and private `_helpers`

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use protocols** — constructor receives protocol interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `self`/`cls` dependency**:

```bash
# Find module-level functions (outside class) — ALWAYS forbidden
grep -n "^def " modules/*/src/capabilities_*.py

# Find @staticmethod inside class (no self, no cls)
grep -B1 -A5 "@staticmethod" modules/*/src/capabilities_*.py

# Find @classmethod that doesn't use cls
grep -B1 -A5 "@classmethod" modules/*/src/capabilities_*.py
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
4. Add import in capabilities file: `from shared.<domain>.taxonomy_<name>_utility import func_name`
5. Remove original function from capabilities file
6. Register utility module in `__init__.py` if needed
7. Verify: `python -c "from shared.<domain>.taxonomy_<name>_utility import *"`

### Step 7: Verify Layer Compliance

Check forbidden imports and I/O patterns:

```bash
# Check for I/O in capabilities
grep -n "open(\|Path(\|os\." modules/*/src/capabilities_*.py

# Check for forbidden imports
grep -n "infrastructure_\|agent_" modules/*/src/capabilities_*.py
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
| No `self`, no `cls`, no class state           | `normalize_path(path: str) -> str`            |
| All data via parameters                       | `count_lines(content: str) -> int`            |
| Deterministic, no side effects                | `is_barrel_file(name: str) -> bool`           |

```python
# taxonomy_line_checker_utility.py (SHARED / TAXONOMY)
def is_barrel_file(basename: str) -> bool:
    return basename in ("__init__.py", "mod.rs")

# capabilities_line_checker.py (CONSUMER)
from shared.code_analysis.taxonomy_line_checker_utility import is_barrel_file
```

### Option B: Keep as Instance/Class Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `self` / instance fields | `self._cache.get(key)`                          |
| Uses `cls` / class-level config | `cls._registry[name]`                         |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level constants or types  |

```python
# capabilities_line_checker.py (STAYS IN CLASS — Block 3)
class ArchLineChecker(ILineCheckerProtocol):
    _THRESHOLD_KEY = "line_count"  # class-level constant

    def __init__(self, config: ICheckerConfigProtocol):
        self._config = config

    def _resolve_threshold(self, layer: str) -> int:  # uses self → stays
        return self._config.get_threshold(layer)

    @classmethod
    def from_registry(cls, name: str) -> "ArchLineChecker":  # uses cls → stays
        return cls._registry[name]
```

### Decision Tree

```
Function found in capabilities file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in capabilities)
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
  │   ├─ Defined in protocol ABC?
  │   │   └─ YES → Block 2
  │   └─ Private helper?
  │       └─ YES → Block 3
  │
  └─ Dunder method (__repr__, __eq__, etc.)?
      └─ YES → Block 3
```

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class + `__init__` → Protocol Methods → Dunders/Factories/Helpers).
- [ ] **Block 2 contains ONLY protocol `@abstractmethod` implementations**. No dunder methods, no `@staticmethod`, no factory classmethods in Block 2.
- [ ] **Dunder methods** (`__repr__`, `__str__`, `__eq__`, `__hash__`) and **factory classmethods** (`create_default`, `from_config`) are in **Block 3**.
- [ ] Capability class inherits a protocol ABC (AES403 resolved).
- [ ] Protocol contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive protocol interfaces via `__init__`.
- [ ] **No module-level `def` functions** exist outside the class in capabilities files.
- [ ] **No stateless `@staticmethod`** (zero class dependency) remains in class — extracted to `*_utility.py`.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use protocol interfaces, not concrete types.
- [ ] **Zero I/O** in capabilities layer (no open(), no Path(), no os.).
- [ ] No forbidden imports (no infrastructure\_\_, no agent\_\_).
- [ ] Protocol module is registered in the shared module's `__init__.py`.
- [ ] Utility module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    def \|^    @" modules/<module>/src/capabilities_*.py

# Find capabilities without protocol inheritance
grep -rn "^class " modules/*/src/capabilities_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Protocol" "$file" || echo "MISSING: $file has $class without protocol"
done

# Ensure protocol does NOT contain helper methods
grep -E "def (helper|util|private|_)" modules/shared/src/contract_*_protocol.py || echo "Clean: No helpers in protocol"

# Check for I/O in capabilities (AES404)
grep -n "open(\|Path(\|os\." modules/*/src/capabilities_*.py

# Check for dataclasses defined in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | grep capabilities

# Check for concrete type fields (non-protocol)
grep -n "__init__" modules/*/src/capabilities_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-PROTOCOL FIELD: $file"
done

# Find module-level free functions in capabilities files (ALWAYS forbidden)
grep -n "^def [a-z_]*(" modules/*/src/capabilities_*.py

# Find @staticmethod that may need extraction (no self, no cls)
grep -B1 -A10 "@staticmethod" modules/*/src/capabilities_*.py

# Detect dunder methods appearing BEFORE protocol methods (wrong block order)
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_proto = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_proto is None:
            first_proto = i + 1
    if first_dunder and first_proto and first_dunder < first_proto:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before protocol method (line {first_proto})')
" modules/*/src/capabilities_*.py

# Find standalone functions in class files (should be extracted to utility)
grep -n "^def [a-z_]*(\s*[^self])" modules/*/src/capabilities_*.py

# Check syntax
python -c "import <module>"

# Find error swallowing patterns (or "", or 0)
grep -n "or ''\|or \"\"\|or 0\|or ''\|or \"\"" modules/*/src/capabilities_*.py

# Find magic constants (hardcoded literals)
grep -n "[0-9]\+\.[0-9]\+" modules/*/src/capabilities_*.py | grep -v "#\|const\|import" | head -20

# Detect dunder methods appearing BEFORE protocol methods (wrong block order)
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_proto = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_proto is None:
            first_proto = i + 1
    if first_dunder and first_proto and first_dunder < first_proto:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before protocol method (line {first_proto})')
" modules/*/src/capabilities_*.py
```

## Error Handling (from fix-error-handling)

**Capabilities Layer Error Rules:**
- **Never silently discard errors** with `or ""` or `or 0` in capabilities layer.
- All public methods MUST return descriptive error types or raise meaningful exceptions.
- IO errors (file read, network) → propagate with `raise` or return error result.
- Logic errors (validation, parsing) → propagate with custom exception type.

### Silent Swallowing (Fix)

```python
# [FORBIDDEN] Error silently discarded
def filepath_or_default(result: FilePath | None) -> FilePath:
    return result or FilePath("")  # Error thrown away

# [FORBIDDEN] Error detail lost
try:
    cycle_check()
except Exception as e:
    print(e)  # Debug printing loses context
```

### Proper Patterns (Use)

```python
# [OK] Explicit error propagation
def parse_file(path: FilePath) -> Result[Content, ParseError]:
    try:
        return Ok(path.value().read_text())
    except Exception as e:
        return Err(ParseError(f"Cannot read: {e}"))

# [OK] LintResult for check failures (not IO failures)
def check_imports(...) -> list[LintResult]:
    try:
        content = path.read_text()
    except Exception as e:
        return [LintResult.new_arch(
            "PARSE_ERROR", f"Cannot read: {e}", str(path)
        )]
    # Import check failure -> LintResult (expected outcome)
```

## Primitive-to-VO Replacement (from fix-primitive-to-vo)

**Capabilities Layer VO Rules:**
- Entity fields MUST use VOs, not primitives (`str`, `int`, `float`, `bool`).
- Contract signatures MUST use VOs.
- VOs MUST validate on construction.

```python
# BEFORE (primitive)
@dataclass
class LintResult:
    file_path: str
    line: int
    severity: str

# AFTER (VO)
@dataclass
class LintResult:
    file_path: FilePath
    line: LineNumber
    severity: SeverityVO
```

## Magic Constant Extraction (from fix-magic-constant)

**Capabilities Layer Constant Rules:**
- NO hardcoded literals in capabilities layer.
- All domain values MUST be named constants.
- Constants MUST live in `taxonomy_*_constant.py`.

```python
# [FORBIDDEN] BEFORE
def calculate_duration(self) -> float:
    return 0.5  # magic

# [OK] AFTER
from shared.animator.taxonomy_animator_constant import MIN_REVEAL_SECONDS
def calculate_duration(self) -> float:
    return MIN_REVEAL_SECONDS
```

## Import Strategy (from fix-imports)

When fixing cross-import violations in capabilities, choose the right approach:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)
Use when the code is **stateless, pure logic** with no side effects:

| Condition                                     | Example                                       |
| --------------------------------------------- | --------------------------------------------- |
| Pure function — no `self`, no `cls`           | `parse_path()`, `normalize_name()`            |
| Stateless — all data via parameters           | `compute_distance(a: Point, b: Point)`        |
| No side effects — deterministic output        | `sanitize_string(input: str) -> str`          |

```python
# taxonomy_path_utility.py (TAXONOMY LAYER)
def parse_path(path: str) -> str | None:
    return path[1:] if path.startswith('/') else None

# capabilities_timeline_processor.py (CONSUMER)
from shared.taxonomy_path_utility import parse_path, normalize_name  # ALLOWED: taxonomy import
```

### Option B: Dependency Injection via Protocols (Port/Protocol Pattern)
Use when the code requires **state, side effects, or layer-specific behavior**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Needs `self` / instance state | Class with fields for data/mutation              |
| Has side effects / I/O        | File operations, network calls, DB queries       |
| Layer-specific implementation | Adapter that depends on concrete infrastructure |

```python
# 1. Define protocol in CONTRACT layer
# contract_frame_exporter_protocol.py
class IFrameExporterProtocol(Protocol):
    def export(self, frame: Frame) -> str: ...

# 2. Capability implements the protocol
# capabilities_frame_exporter.py
class FrameExporter(IFrameExporterProtocol):
    def __init__(self, output_dir: str):
        self._output_dir = output_dir
    def export(self, frame: Frame) -> str:
        return f"{self._output_dir}/{frame.id}.png"

# 3. Consumer receives via DI (knows only the protocol)
# capabilities_timeline_processor.py
class TimelineProcessor:
    def __init__(self, exporter: IFrameExporterProtocol):  # via DI, not direct import
        self._exporter = exporter
```

## Decision Tree: Which Option to Choose?

```text
Encountered cross-import violation in capabilities?
  │
  ├─ Does the code need self / class state?
  │   └─ YES → Option B: Dependency Injection
  │
  ├─ Does the code have side effects (I/O, file, network)?
  │   └─ YES → Option B: Dependency Injection
  │
  └─ Is it pure, stateless, no self?
      └─ YES → Option A: Extract to Taxonomy Utility
```

## Common Mistakes (AVOID)

- ❌ **Putting I/O in capabilities**: File I/O, network calls, and database operations MUST be in infrastructure layer.
- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive protocol interfaces, not concrete implementations.
- ❌ **Putting helper methods in the protocol**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave protocol methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions (no `self`) MUST be extracted to standalone `*_utility.py` modules.
- ❌ **Creating "God Protocols"**: If a protocol has >10 methods or mixes unrelated concerns, split it into multiple protocols.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-python` if merging multiple files.
- ❌ **Placing dunder methods (`__repr__`, `__str__`, `__eq__`) in Block 2**: Block 2 is RESERVED for protocol method implementations ONLY. Dunders are utilities and belong in Block 3.
- ❌ **Placing factory classmethods (`create_default`, `from_config`) before protocol methods**: Factories are constructors and belong in Block 3, after protocol methods.
- ❌ **Mixing `__init__` into Block 2**: `__init__` is part of Block 1 (class definition & constructor), not a protocol method.
- ❌ **Leaving module-level `def` in capabilities files**: Free functions outside the class MUST be extracted to `*_utility.py` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `@staticmethod` in class**: If a `@staticmethod` uses no `self`, no `cls`, and no class-level state, it belongs in `*_utility.py`, not in the class body.
- ❌ **Placing `__init__` in Block 2**: Constructor is part of Block 1 (class definition), not a protocol method.
- ❌ **Placing `__repr__`/`__str__` before protocol methods**: Dunder methods belong in Block 3, after protocol methods.

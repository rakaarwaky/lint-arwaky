---
name: create-capabilities-python
description: "Create and validate capabilities layer files following AES rules: 3-block structure, one class per file, protocol contracts, zero I/O."
version: 1.0.0
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
  ]
triggers:
  - "create capability python"
  - "add capability python"
  - "fix capability structure python"
  - "create protocol python"
  - "capability missing protocol python"
  - "check capabilities python"
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

Create and validate Python **capabilities layer** files following clean architecture rules. Ensures capabilities contain zero I/O, inherit protocol ABCs, follow the 3-Block Structure, and use DI for all fields.

**This skill consolidates rules from:** `fix-capability-structure`, `create-missing-protocols`, `module_logic_validator`, `enforce-1-class-per-file`, and `method_classifier` — applied specifically to the capabilities layer.

## Rules

### Layer Boundaries (AES)

**Capabilities Layer (`capabilities_*.py`)**

| Allowed                              | Forbidden                                    |
| ------------------------------------ | -------------------------------------------- |
| Computation, validation, calculation | File I/O (`open()`, `Path()`, `os.`)         |
| Data transformation, business rules  | Network calls (`requests.`, `httpx.`)        |
| Domain logic, domain model definition | Database operations (`sqlite3.`, `asyncpg.`) |
| Protocol/ABC implementation          | Direct import from `infrastructure_*`        |
|                                      | Direct import from `agent_*`                 |
|                                      | Direct import from `capabilities_*` (self)   |

### Structural Rules (All Layers)

- **1 file = 1 class** — each capabilities file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`) should be extracted to `*_utility.py` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Protocol (Public Contract)
3. `@staticmethod` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions (no `self`) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3.

### Protocol Rules

- **Every capability class MUST inherit from a protocol ABC** (AES403)
- **Protocol MUST define abstract methods for all public methods**
- **Protocol contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod` or instance methods)
- **Constructors in class body** — `__init__` receives protocol interfaces

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes → **`capabilities_*.py` + inherit protocol ABC**
If no (has I/O) → **split into infrastructure layer instead**

## Naming Convention

| Layer          | File Pattern             | Protocol File                          | Protocol Name         |
| -------------- | ------------------------ | -------------------------------------- | --------------------- |
| **Capabilities** | `capabilities_*.py`      | `contract_<name>_protocol.py`          | `I<Name>Protocol`     |
| **Infrastructure** | `infrastructure_*.py`  | `contract_<name>_port.py`              | `I<Name>Port`         |
| **Agents**       | `agent_*.py`             | `contract_<name>_aggregate.py`         | `I<Name>Aggregate`    |

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

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use protocols
from shared.code_analysis.taxonomy_analysis import OrphanIndicatorResult
from contract_orphan_protocol import IOrphanFilenameExtractorProtocol

class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor  # ← DI via protocol
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

| Module       | Protocol Path                                          |
| ------------ | ------------------------------------------------------ |
| compositor   | `modules/shared/src/compositor/contract_*_protocol.py` |
| animator     | `modules/shared/src/animator/contract_*_protocol.py`   |
| scripting    | `modules/shared/src/scripting/contract_*_protocol.py`  |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. `@abstractmethod` methods implementing Protocol (public contract)
3. `@staticmethod` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use protocols** — constructor receives protocol interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and I/O patterns:

```bash
# Check for I/O in capabilities
grep -n "open(\|Path(\|os\." modules/*/src/capabilities_*.py

# Check for forbidden imports
grep -n "infrastructure_\|agent_" modules/*/src/capabilities_*.py
```

### Step 7: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Protocol Methods -> Helpers).
- [ ] Capability class inherits a protocol ABC (AES403 resolved).
- [ ] Protocol contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive protocol interfaces via `__init__`.
- [ ] No standalone functions (no `self`) remain in class — extracted to `*_utility.py` modules.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use protocol interfaces, not concrete types.
- [ ] **Zero I/O** in capabilities layer (no open(), no Path(), no os.).
- [ ] No forbidden imports (no infrastructure_*, no agent_*).
- [ ] Protocol module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^def\|^@staticmethod" modules/<module>/src/capabilities_*.py

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

# Find standalone functions in class files (should be extracted to utility)
grep -n "^def [a-z_]*(\s*[^self])" modules/*/src/capabilities_*.py

# Check syntax
python -c "import <module>"
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

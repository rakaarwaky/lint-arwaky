---
name: create-infrastructure-python
description: "Create and validate infrastructure layer files following AES rules: 3-block structure, one class per file, port contracts, zero business logic."
version: 1.0.0
category: refactoring
tags:
  [
    python,
    aes,
    infrastructure,
    port,
    structure,
    aes404,
    3-block-structure,
    di,
  ]
triggers:
  - "create infrastructure python"
  - "add infrastructure python"
  - "fix infrastructure structure python"
  - "create port python"
  - "infrastructure missing port python"
  - "verify infrastructure python"
dependencies: []
related:
  - create-capabilities-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-capability-structure-python
  - create-missing-protocols-python
---

# create-infrastructure-python

## Purpose

Create and validate Python **infrastructure layer** files following clean architecture rules. Ensures infrastructure contains zero business logic, inherits port ABCs, follows the 3-Block Structure, and uses DI for all fields.

## Rules

### Layer Boundaries (AES)

**Infrastructure Layer (`infrastructure_*.py`)**

| Allowed                              | Forbidden                                    |
| ------------------------------------ | -------------------------------------------- |
| File I/O (`open()`, `Path()`)        | Business rules                               |
| Network calls (`requests.`, `httpx.`)| Domain logic                                 |
| Database operations (`sqlite3.`, `asyncpg.`) | Calculations (should be in capabilities)  |
| External API calls                   | Direct import from `agent_*`                 |
| Protocol/ABC implementation          | Direct import from `capabilities_*`          |

### Structural Rules (All Layers)

- **1 file = 1 class** — each infrastructure file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`) should be extracted to `*_utility.py` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Port (Public Contract)
3. `@staticmethod` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions (no `self`) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3.

### Port Rules

- **Every infrastructure class MUST inherit from a port ABC**
- **Port MUST define abstract methods for all public methods**
- **Port contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod` or instance methods)
- **Constructors in class body** — `__init__` receives port interfaces

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.py` + inherit port ABC**
If no (has business logic) → **split into capabilities layer instead**

## Naming Convention

| Layer          | File Pattern             | Protocol File                         | Protocol Name         |
| -------------- | ------------------------ | ------------------------------------- | --------------------- |
| **Capabilities** | `capabilities_*.py`      | `contract_<name>_protocol.py`         | `I<Name>Protocol`     |
| **Infrastructure** | `infrastructure_*.py`  | `contract_<name>_port.py`             | `I<Name>Port`         |
| **Agents**       | `agent_*.py`             | `contract_<name>_aggregate.py`        | `I<Name>Aggregate`    |

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```python
# BAD: No port inheritance
class FileCache:
    def read(self): ...
```

### BAD: Business Logic in Infrastructure

```python
# BAD: Business logic in infrastructure layer
class OrphanFileCache:
    def analyze(self, content: str) -> bool:
        # ← DOMAIN LOGIC — should be in capabilities
        is_orphan = "orphan" in content
        return is_orphan
```

### BAD: Dataclass in Layer File

```python
# BAD: Domain data defined in infrastructure layer
@dataclass
class CacheEntry:  # ← DATA CLASS — should be in shared/taxonomy
    key: str
    value: str
    timestamp: int

class OrphanFileCache:
    entry: CacheEntry  # ← concrete type, not DI
```

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use protocols
from shared.common.taxonomy_path import FilePath

class OrphanFileCache:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor  # ← DI via protocol
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has business logic → **MOVE to Capabilities** (AES404)
- If pure I/O/external integration → continue to Step 2

### Step 2: Check for Missing Port

Does the infrastructure class inherit from a port ABC? If no → create one.

```bash
# Find infrastructure without port inheritance
grep -rn "^class " modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done
```

### Step 3: Create Port File (if missing)

Create `contract_<name>_port.py` in the shared module with abstract methods.

**Port location:**

| Module       | Port Path                                              |
| ------------ | ------------------------------------------------------ |
| compositor   | `modules/shared/src/compositor/contract_*_port.py`     |
| animator     | `modules/shared/src/animator/contract_*_port.py`       |
| scripting    | `modules/shared/src/scripting/contract_*_port.py`      |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. `@abstractmethod` methods implementing Port (public contract)
3. `@staticmethod` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use protocols** — constructor receives port interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and business logic patterns:

```bash
# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate" modules/*/src/infrastructure_*.py

# Check for forbidden imports
grep -n "capabilities_\|agent_" modules/*/src/infrastructure_*.py
```

### Step 7: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Port Methods -> Helpers).
- [ ] Infrastructure class inherits a port ABC.
- [ ] Port contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive port interfaces via `__init__`.
- [ ] No standalone functions (no `self`) remain in class — extracted to `*_utility.py` modules.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use port interfaces, not concrete types.
- [ ] **Zero business logic** in infrastructure layer (no domain rules, no calculations).
- [ ] No forbidden imports (no capabilities_*, no agent_*).
- [ ] Port module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^def\|^@staticmethod" modules/<module>/src/infrastructure_*.py

# Find infrastructure without port inheritance
grep -rn "^class " modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done

# Ensure port does NOT contain helper methods
grep -E "def (helper|util|private|_)" modules/shared/src/contract_*_port.py || echo "Clean: No helpers in port"

# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate\|business" modules/*/src/infrastructure_*.py

# Check for dataclasses defined in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | grep infrastructure

# Check for concrete type fields (non-protocol)
grep -n "__init__" modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-PROTOCOL FIELD: $file"
done

# Find standalone functions in class files (should be extracted to utility)
grep -n "^def [a-z_]*(\s*[^self])" modules/*/src/infrastructure_*.py

# Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Putting business logic in infrastructure**: Domain rules, calculations, and validation MUST be in capabilities layer.
- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive port interfaces, not concrete implementations.
- ❌ **Putting helper methods in the port**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave port methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions (no `self`) MUST be extracted to standalone `*_utility.py` modules.
- ❌ **Creating "God Ports"**: If a port has >10 methods or mixes unrelated concerns, split it into multiple ports.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-python` if merging multiple files.

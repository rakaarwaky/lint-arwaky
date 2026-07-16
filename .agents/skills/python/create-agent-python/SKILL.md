---
name: create-agent-python
description: "Create and validate agent layer files following AES rules: 3-block structure, one class per file, aggregate contracts, zero computation/I/O/business logic."
version: 1.0.0
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
  ]
triggers:
  - "create agent python"
  - "add agent python"
  - "fix agent structure python"
  - "create aggregate python"
  - "agent missing aggregate python"
  - "validate agent logic python"
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

Create and validate Python **agent layer** files following clean architecture rules. Ensures agents contain zero computation, zero I/O, and zero business logic — they are orchestration/pipeline execution only. Agents inherit aggregate ABCs, follow the 3-Block Structure, and use DI for all fields.

**This skill consolidates rules from:** `module_logic_validator`, `enforce-1-class-per-file`, `method_classifier`, and `fix-agent-di` — applied specifically to the agent layer.

## Rules

### Layer Boundaries (AES)

**Agent Layer (`agent_*.py`)**

| Allowed                                          | Forbidden                                            |
| ------------------------------------------------ | ---------------------------------------------------- |
| `for`, `while`, `async for` (orchestration flow) | Computation (`sum()`, `len()`, arithmetic)           |
| `if/else`, `elif`, `match` (control flow)        | Business rules, domain logic                         |
| `try/except`, `raise` (error propagation)        | File I/O (`open()`, `Path()`, `os.`)                 |
| `asyncio.wait_for`, `.sleep` (async)             | Network (`requests.`, `httpx.`)                      |
| Sequential statements (orchestration)            | Database (`sqlite3.`, `asyncpg.`)                    |
| Protocol/ABC implementation                      | Domain model definition (`@dataclass`)               |
|                                                  | Direct import from `capabilities_*`                  |
|                                                  | Direct import from `infrastructure_*`                |

### Structural Rules (All Layers)

- **1 file = 1 class** — each agent file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`) should be extracted to `*_utility.py` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Aggregate (Public Contract)
3. `@staticmethod` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions (no `self`) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3.

### Aggregate Rules

- **Every agent class MUST inherit from an aggregate ABC**
- **Aggregate MUST define abstract methods for all public methods**
- **Aggregate contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod` or instance methods)
- **Constructors in class body** — `__init__` receives aggregate interfaces

## The Fundamental Question

> **"Is this file orchestration/pipeline execution only?"**

If yes → **`agent_*.py` + inherit aggregate ABC**
If no (has computation, I/O, or business logic) → **split into appropriate layer**

## Naming Convention

| Layer          | File Pattern             | Protocol File                         | Protocol Name         |
| -------------- | ------------------------ | ------------------------------------- | --------------------- |
| **Capabilities** | `capabilities_*.py`      | `contract_<name>_protocol.py`         | `I<Name>Protocol`     |
| **Infrastructure** | `infrastructure_*.py`  | `contract_<name>_port.py`             | `I<Name>Port`         |
| **Agents**       | `agent_*.py`             | `contract_<name>_aggregate.py`        | `I<Name>Aggregate`    |

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

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use protocols
from contract_orphan_protocol import ICapabilitiesOrphanProtocol

class OrphanOrchestrator:
    def __init__(self, analyzer: ICapabilitiesOrphanProtocol):
        self._analyzer = analyzer  # ← DI via protocol
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

| Module       | Aggregate Path                                         |
| ------------ | ------------------------------------------------------ |
| compositor   | `modules/shared/src/compositor/contract_*_aggregate.py`|
| animator     | `modules/shared/src/animator/contract_*_aggregate.py`  |
| scripting    | `modules/shared/src/scripting/contract_*_aggregate.py` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. `@abstractmethod` methods implementing Aggregate (public contract)
3. `@staticmethod` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use protocols** — constructor receives aggregate interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for computation in agents
grep -n "sum(\|len(\|\.iter\(\)|\.map(" modules/*/src/agent_*.py

# Check for forbidden imports
grep -n "capabilities_\|infrastructure_" modules/*/src/agent_*.py
```

### Step 7: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Aggregate Methods -> Helpers).
- [ ] Agent class inherits an aggregate ABC.
- [ ] Aggregate contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive aggregate interfaces via `__init__`.
- [ ] No standalone functions (no `self`) remain in class — extracted to `*_utility.py` modules.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use aggregate interfaces, not concrete types.
- [ ] **Zero computation** in agent layer (no sum(), no len(), no iteration transforms).
- [ ] **Zero I/O** in agent layer (no open(), no network, no database).
- [ ] **Zero business logic** in agent layer (no domain rules, no validation).
- [ ] No forbidden imports (no capabilities_*, no infrastructure_*).
- [ ] Aggregate module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^def\|^@staticmethod" modules/<module>/src/agent_*.py

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

# Check for concrete type fields (non-protocol)
grep -n "__init__" modules/*/src/agent_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-PROTOCOL FIELD: $file"
done

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

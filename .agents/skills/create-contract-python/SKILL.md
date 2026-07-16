---
name: create-contract-python
description: "Create and validate contract layer files (contract_*.py) — port, protocol, aggregate ABCs that decouple layers without implementing any logic."
version: 1.0.0
category: refactoring
tags:
  [
    python,
    aes,
    contract,
    protocol,
    port,
    aggregate,
    interface,
    shared,
    structure,
  ]
triggers:
  - "create contract python"
  - "add contract python"
  - "create protocol python"
  - "create port python"
  - "create protocol python"
  - "create aggregate python"
  - "missing contract python"
dependencies: []
related:
  - create-capabilities-python
  - create-infrastructure-python
  - create-agent-python
  - create-taxonomy-python
  - trait-consolidation-python
  - enforce-1-class-per-file-python
  - create-missing-protocols-python
---

# create-contract-python

## Purpose

Create and validate Python **contract layer** files in `modules/shared/src/<domain>/`. Contracts are pure ABC definitions — they decouple layers by defining interfaces without implementing any logic. Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

## Rules

### The Fundamental Question

> **"Is this a pure ABC definition or does it contain implementation?"**

- **Contract (ABC only)** → **MUST be in shared/taxonomy as `contract_*.py`**. No method bodies, no logic.
- **Class** (that implements protocol) → belongs in layer file (`capabilities_*.py`, `infrastructure_*.py`, `agent_*.py`).

### Contract Layer Structure

```
modules/shared/src/<domain>/
├── __init__.py                 # Module exports for this domain
├── contract_*_port.py          # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.py      # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.py     # Composition facades — implemented by Agents
```

### Three Suffix Types and Their Roles

| Suffix | Role | Implemented By | Example |
|--------|------|-----------------|---------|
| `_port` | Outbound interface | Infrastructure layer | `contract_system_port.py`, `contract_import_parser_port.py` |
| `_protocol` | Inbound interface | Capabilities layer | `contract_import_forbidden_protocol.py`, `contract_naming_checker_protocol.py` |
| `_aggregate` | Composition facade | Agent layer | `contract_import_runner_aggregate.py`, `contract_tui_aggregate.py` |

**CRITICAL:** These suffixes are **strict** — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Naming Convention

`contract_<concept_word(s)>_<role_suffix>.py`

| Concept | File Name | Protocol Name | Implemented By |
|---|---|---|---|
| System operations | `contract_system_port.py` | `ISystemPort` | Infrastructure adapters |
| Forbidden import checking | `contract_import_forbidden_protocol.py` | `IImportForbiddenProtocol` | Capabilities checkers |
| Import runner orchestration | `contract_import_runner_aggregate.py` | `IImportRunnerAggregate` | Agent orchestrators |

### Import Restrictions (AES201)

Contract files must remain **completely pure**:

| Can Import From | Cannot Import From |
| --- | --- |
| `taxonomy_*` files | capabilities, infrastructure, agents, surfaces |
| Other `contract_*` files | Any layer files (*.py without contract_ or taxonomy_ prefix) |

**Contracts define interfaces only — zero implementation logic.**

### Protocol Structure

Every contract ABC follows the protocol pattern:

```python
# contract_system_port.py
from abc import ABC, abstractmethod

class IFileSystemPort(ABC):
    """Outbound interface for file system operations."""

    @abstractmethod
    def read_file(self, path: str) -> str:
        """Read file contents."""
        ...

    @abstractmethod
    async def write_file(self, path: str, content: str) -> None:
        """Write content to file."""
        ...

    @abstractmethod
    def glob_files(self, pattern: str, callback: callable) -> int:
        """Glob files matching pattern."""
        ...
```

## Detection Patterns

### BAD: Contract Contains Implementation

```python
# BAD: Contract file contains method bodies with logic
class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: str) -> str:
        # ← IMPLEMENTATION belongs in infrastructure_*.py
        return open(path).read()  # ← I/O in contract!
```

### BAD: Contract Imports Non-Taxonomy Types

```python
# BAD: Contract imports capability types
from capabilities_my_checker import MyChecker  # ← FORBIDDEN

class IMyProtocol(ABC):
    def check(self, checker: MyChecker) -> None:  # ← Should use taxonomy types only
        ...
```

### GOOD: Pure Protocol ABC

```python
# contract_system_port.py — pure ABC definition
from shared.common.taxonomy_path import FilePath

class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> str:
        """Read file contents."""
        ...

    @abstractmethod
    async def write_file(self, path: FilePath, content: str) -> None:
        """Write content to file."""
        ...

# Implementation belongs in infrastructure_adapter.py — NOT here
```

## Workflow

### Step 1: Determine the Contract Role

Ask: **"Which layer will implement this interface?"**

- Infrastructure implements → `_port` (outbound)
- Capabilities implements → `_protocol` (inbound)
- Agent implements → `_aggregate` (composition facade)

### Step 2: Identify Public Methods

List all methods that other layers need to call. These become abstract method signatures.

```bash
# Find methods used across layers
grep -rn "def " modules/*/src/ | grep -v "shared/" | head -50
```

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.py` in the appropriate domain under `modules/shared/src/<domain>/`.

**Rules:**
- ABC must be imported from `abc` module
- Methods must use `@abstractmethod` decorator
- Use `...` (ellipsis) for method bodies — no implementation logic
- Import only `taxonomy_*` and other `contract_*` files

```python
# contract_<name>_<suffix>.py
from abc import ABC, abstractmethod
from shared.common.taxonomy_path import FilePath

class I<Name><Suffix>(ABC):
    @abstractmethod
    def public_method(self, input: FilePath) -> str:
        """Public method description."""
        ...

    @abstractmethod
    async def async_method(self, id: int) -> None:
        """Async method description."""
        ...
```

### Step 4: Register Module

Update the domain's `__init__.py` to export the new contract module:

```python
# shared/src/<domain>/__init__.py
from .contract_<name>_<suffix> import I<Name><Suffix>  # ← Add this line
from .taxonomy_<name>_vo import SomeVO
```

### Step 5: Implement in Layer File

The implementing layer file imports and inherits from the ABC:

```python
# Infrastructure layer implements _port
from shared.<domain>.contract_system_port import IFileSystemPort

class FileAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> str:
        with open(path.value()) as f:
            return f.read()

    async def write_file(self, path: FilePath, content: str) -> None:
        with open(path.value(), 'w') as f:
            f.write(content)
```

### Step 6: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only ABC definitions** — no method bodies, no implementation logic.
- [ ] ABC imports `ABC` from `abc` module.
- [ ] Methods use `@abstractmethod` decorator.
- [ ] Contract imports only `taxonomy_*` and other `contract_*` files.
- [ ] No capabilities, infrastructure, agents, or surface imports in contract files.
- [ ] Domain's `__init__.py` exports new contract module — `from .contract_<name>_<suffix> import ...`.
- [ ] Layer file inherits the ABC (infrastructure for _port, capabilities for _protocol, agent for _aggregate).
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Find contracts without implementations
grep -rn "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*$class\|inherits.*$class" modules/*/src/*.py || echo "UNIMPLEMENTED: $class in $file"
done

# Check for forbidden imports in contract files
grep -n "from capabilities_\|from infrastructure_\|from agent_" modules/shared/src/*/contract_*.py

# Find contracts that don't use @abstractmethod
grep -rn "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -q "@abstractmethod" "$file" || echo "NO ABSTRACT: $file"
done

# Verify contract module exports are registered
grep -n "^from \.contract_" modules/shared/src/*/ __init__.py

# Check for unregistered contract files (exist on disk but not in __init__.py)
find modules/shared/src/<domain>/ -name "contract_*.py" | while read f; do
    name=$(basename "$f" .py)
    grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py || echo "UNREGISTERED: $name"
done

# Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY ABC definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed. No other suffixes.
- ❌ **Forgetting to register new contract modules in __init__.py**: Every `contract_*.py` file must have a corresponding `from .contract_<name>_<suffix> import ...` in the domain's `__init__.py`.
- ❌ **Missing @abstractmethod decorators on methods**: All contract methods MUST use `@abstractmethod` for proper interface enforcement.
- ❌ **Forgetting to import ABC from abc module**: All contract files must `from abc import ABC, abstractmethod`.
- ❌ **Placing method bodies in contract files**: Even thin wrapper methods belong in layer files, not contracts.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.

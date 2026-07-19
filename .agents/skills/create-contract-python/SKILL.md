---
name: create-contract-python
description: "Create and validate contract layer files (contract_*.py) — pure ABC definitions that decouple layers without leaking implementation details."
version: 1.1.0
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
  - "create aggregate python"
  - "missing contract python"
  - "fix god interface python"
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

Create and validate Python **contract layer** files in `modules/shared/src/<domain>/`. Contracts are **pure ABC definitions** — they decouple layers by defining the "WHAT" (public promise) without implementing any "HOW" (logic) or leaking internal stepping stones. 

Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

## Rules

### The Fundamental Question (The Golden Rule)

> **"Is this a public promise needed by an outer layer, or just an internal stepping stone?"**
> 
> - **Public Promise (WHAT)**: Outer layers need to call this, or it requires polymorphism (multiple implementations). → **Put in Contract (`contract_*.py`)**.
> - **Internal Stepping Stone (HOW)**: Helper methods, highly specific algorithms (e.g., specific regex), or logic that only serves other methods in the same class. → **Keep as Private Helper (`_method`) in Implementation Class**. **NEVER put this in the contract.**

### Contract Layer Structure

```text
modules/shared/src/<domain>/
├── __init__.py                     # Module exports for this domain
├── contract_*_port.py              # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.py          # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.py         # Composition facades — implemented by Agents
```

**CRITICAL:** These suffixes are **strict** — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Three Suffix Types and Their Roles

| Suffix       | Role               | Implemented By       | Example                                                                        |
| ------------ | ------------------ | -------------------- | ------------------------------------------------------------------------------ |
| `_port`      | Outbound interface | Infrastructure layer | `contract_system_port.py`, `contract_import_parser_port.py`                    |
| `_protocol`  | Inbound interface  | Capabilities layer   | `contract_import_forbidden_protocol.py`, `contract_naming_checker_protocol.py` |
| `_aggregate` | Composition facade | Agent layer          | `contract_import_runner_aggregate.py`, `contract_tui_aggregate.py`             |

### Naming Convention

Pattern: `contract_<concept_word(s)>_<role_suffix>.py`

| Concept                     | File Name                               | Protocol Name              | Implemented By          |
| --------------------------- | --------------------------------------- | -------------------------- | ----------------------- |
| System operations           | `contract_system_port.py`               | `ISystemPort`              | Infrastructure adapters |
| Forbidden import checking   | `contract_import_forbidden_protocol.py` | `IImportForbiddenProtocol` | Capabilities checkers   |
| Import runner orchestration | `contract_import_runner_aggregate.py`   | `IImportRunnerAggregate`   | Agent orchestrators     |

### Import Restrictions (AES201)

Contract files must remain **completely pure**:

| Can Import From          | Cannot Import From                                           |
| ------------------------ | ------------------------------------------------------------ |
| `taxonomy_*` files       | `capabilities_*`, `infrastructure_*`, `agent_*`, `surface_*` |
| Other `contract_*` files | Any layer files (`*.py` without `contract_` or `taxonomy_` prefix) |

**Contracts define interfaces only — zero implementation logic.**

### Protocol Structure Rules

Every contract ABC must follow these structural rules:

1. **Inheritance**: Must inherit from `ABC` (imported from `abc` module).
2. **Decorators**: All methods MUST use the `@abstractmethod` decorator.
3. **Bodies**: Use `...` (ellipsis) for method bodies — absolutely no implementation logic.
4. **No Helpers**: Do NOT include private helper signatures (e.g., `_extract_regex`) or highly specific algorithmic steps in the ABC.

```python
# contract_system_port.py — Complete ABC structure example
from abc import ABC, abstractmethod
from shared.common.taxonomy_path import FilePath

class IFileSystemPort(ABC):
    """Outbound interface for file system operations."""

    @abstractmethod
    def read_file(self, path: FilePath) -> str:
        """Read file contents."""
        ...

    @abstractmethod
    async def write_file(self, path: FilePath, content: str) -> None:
        """Write content to file."""
        ...

    @abstractmethod
    def glob_files(self, pattern: str, callback: callable) -> int:
        """Glob files matching pattern."""
        ...

# NOTE: Implementation belongs in infrastructure_adapter.py — NOT here.
```

## Detection Patterns

### BAD: Contract Contains Implementation

```python
# BAD: Contract file contains method bodies with logic
class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: str) -> str:
        # ← IMPLEMENTATION belongs in infrastructure_*.py
        with open(path) as f:  # ← I/O in contract!
            return f.read()
```

### BAD: Contract Imports Non-Taxonomy Types

```python
# BAD: Contract imports capability types
from capabilities_my_checker import MyChecker  # ← FORBIDDEN

class IMyProtocol(ABC):
    @abstractmethod
    def check(self, checker: MyChecker) -> None:  # ← Should use taxonomy VOs only
        ...
```

### BAD: Leaking Implementation Details (God Interface)

```python
# BAD: Contract contains highly specific helper methods that force all implementors to write boilerplate
class IFileParserPort(ABC):
    @abstractmethod
    def parse_file(self, path: FilePath) -> ParsedData:
        """Public promise."""
        ...
    
    @abstractmethod
    def _extract_rust_specific_regex(self, content: str) -> list[str]:
        """BAD: LEAKING IMPLEMENTATION DETAIL. 
        A Python parser doesn't need Rust regex. This belongs in the Rust parser class as a private _method."""
        ...
```

### GOOD: Pure Protocol ABC

```python
# contract_system_port.py — pure ABC definition
from abc import ABC, abstractmethod
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

### Step 2: Identify Public Methods (The Filter)
List all methods. Apply the Golden Rule:
- Does an outer layer call this? → **Keep in Contract**.
- Is it just a stepping stone / internal helper? → **Discard from Contract** (it will be a private `_method` in the impl class).

### Step 3: Create Contract File
Create `contract_<concept>_<suffix>.py` in the appropriate domain under `modules/shared/src/<domain>/`.
- Inherit from `ABC`.
- Use `@abstractmethod` on all methods.
- Use `...` for method bodies.
- Import **only** `taxonomy_*` and other `contract_*` files.

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
from shared.common.taxonomy_path import FilePath

class FileAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> str:
        with open(path.value()) as f:
            return f.read()

    async def write_file(self, path: FilePath, content: str) -> None:
        with open(path.value(), 'w') as f:
            f.write(content)
            
    # Private helpers stay in the class, NOT in the ABC above.
    def _sanitize_path(self, path: str) -> str:
        return path.strip()
```

### Step 6: Verify
Run syntax check to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only ABC definitions** — no method bodies, no implementation logic.
- [ ] **No leaking implementation details**: Contract does not contain highly specific helper methods (e.g., specific regex, internal parsing steps) that belong in the impl class.
- [ ] ABC inherits from `ABC` (imported from `abc` module).
- [ ] All methods use `@abstractmethod` decorator.
- [ ] Contract imports **only** `taxonomy_*` and other `contract_*` files.
- [ ] No `capabilities_*`, `infrastructure_*`, `agent_*`, or `surface_*` imports in contract files.
- [ ] Domain's `__init__.py` exports new contract module (`from .contract_<name>_<suffix> import ...`).
- [ ] Layer file correctly inherits and implements the ABC.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# 1. Find contracts without implementations
grep -rn "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "class.*($class)" modules/*/src/*.py || echo "UNIMPLEMENTED: $class in $file"
done

# 2. Check for forbidden imports in contract files
grep -rn "from capabilities_\|from infrastructure_\|from agent_\|from surface_" modules/shared/src/*/contract_*.py

# 3. Find contracts that don't use @abstractmethod
grep -rn "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -q "@abstractmethod" "$file" || echo "NO ABSTRACT: $file"
done

# 4. Detect potential "God Interfaces" (ABCs with > 10 abstract methods — likely leaking helpers)
find modules/shared/src -name "contract_*.py" -exec sh -c 'count=$(grep -c "@abstractmethod" "$1"); if [ "$count" -gt 10 ]; then echo "WARNING: Potential God Interface? $1 has $count abstract methods"; fi' _ {} \;

# 5. Verify contract module exports are registered
find modules/shared/src/<domain>/ -name "contract_*.py" | while read f; do
    name=$(basename "$f" .py)
    grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py || echo "UNREGISTERED: $name in __init__.py"
done

# 6. Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY ABC definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed. No other suffixes.
- ❌ **Leaking implementation details (God Interface)**: Do not put private helpers, specific regex logic, or internal stepping stones in the contract. They belong in the implementation class as `_private_methods`.
- ❌ **Forgetting to register new contract modules in `__init__.py`**: Every `contract_*.py` file must have a corresponding `from .contract_<name>_<suffix> import ...` in the domain's `__init__.py`.
- ❌ **Missing `@abstractmethod` decorators on methods**: All contract methods MUST use `@abstractmethod` for proper interface enforcement.
- ❌ **Forgetting to inherit from `ABC`**: All contract files must `from abc import ABC`.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.
```

---
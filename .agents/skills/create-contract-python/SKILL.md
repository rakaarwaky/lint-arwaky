---
name: create-contract-python
description: "Create and validate Python contract layer files in shared domain: pure ABC definitions for ports, protocols, and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
version: 1.3.0
category: refactoring
tags:
  [
    python,
    aes,
    contract,
    port,
    protocol,
    aggregate,
    abc,
    shared,
    aes201,
    di,
    vo,
  ]
triggers:
  - "create contract python"
  - "add contract python"
  - "create port python"
  - "create protocol python"
  - "create aggregate python"
  - "fix contract python"
  - "check contract python"
  - "audit contract python"
dependencies: []
related:
  - create-taxonomy-python
  - create-capabilities-python
  - create-infrastructure-python
  - create-agent-python
  - trait-consolidation-python
  - fix-primitive-to-vo
---

# create-contract-python

## Purpose

Create and validate Python **contract layer** files inside:

```text
modules/shared/src/<domain>/
```

Contracts are pure ABC definitions.

They define the **WHAT**:

- public promises,
- stable interfaces,
- polymorphism boundaries,
- DI boundaries.

They MUST NOT define the **HOW**:

- no implementation,
- no private helpers,
- no internal stepping stones,
- no I/O,
- no business logic,
- no layer imports.

Three contract suffixes serve different roles:

- `_port` → implemented by infrastructure
- `_protocol` → implemented by capabilities
- `_aggregate` → implemented by agents

---

## Definition of Done

A contract file is considered valid when:

1. It uses one of the allowed suffixes: `_port`, `_protocol`, `_aggregate`.
2. It contains only ABC definitions.
3. It contains no method implementations.
4. It contains no default method bodies.
5. It contains no helper methods or internal stepping stones.
6. It imports only taxonomy types and other contract types.
7. It does not import from capabilities, infrastructure, agent, or surface layers.
8. All methods use `@abstractmethod` decorator.
9. Public contract signatures use shared VOs for domain data.
10. New contract modules are registered in `__init__.py`.
11. `python -c "import <module>"` passes.

---

## The Fundamental Question

> **"Is this a public promise needed by an outer layer, or just an internal stepping stone?"**

### Public Promise

Put it in the contract when:

- outer layers need to call it,
- it defines a stable public interface,
- it requires polymorphism,
- it is injected via protocol interface.

Example:

```python
def check(self, source: SourceContentVO) -> list[LintResult]: ...
```

### Internal Stepping Stone

Keep it as a private helper in the implementation class when:

- it only supports other methods in the same class,
- it is algorithm-specific,
- it is implementation-specific,
- it is not part of the public promise.

Example:

```python
def _extract_rust_trait_name(self, line: str) -> SymbolName | None:
    # internal helper, not contract material
    ...
```

Internal stepping stones MUST NOT appear in contract ABCs.

---

## Contract Layer Structure

```text
modules/shared/src/<domain>/
├── __init__.py
├── contract_*_port.py
├── contract_*_protocol.py
├── contract_*_aggregate.py
├── taxonomy_*_vo.py
├── taxonomy_*_entity.py
├── taxonomy_*_error.py
├── taxonomy_*_event.py
├── taxonomy_*_constant.py
└── taxonomy_*_utility.py
```

Important:

- Contract files define ABCs only.
- Taxonomy files define data types only.
- Layer files define implementations only.

---

## Three Suffix Types and Their Roles

| Suffix         | Role                                               | Implemented By | Example                                   |
| -------------- | -------------------------------------------------- | -------------- | ----------------------------------------- |
| `_port`      | Outbound interface needing I/O or external systems | Infrastructure | `contract_file_system_port.py`          |
| `_protocol`  | Inbound interface for pure domain behavior         | Capabilities   | `contract_import_forbidden_protocol.py` |
| `_aggregate` | Composition facade for orchestration               | Agents         | `contract_import_runner_aggregate.py`   |

---

## Naming Convention

Pattern:

```text
contract_<concept>_<role_suffix>.py
```

Examples:

| Concept                     | File Name                                 | ABC Name                     | Implemented By |
| --------------------------- | ----------------------------------------- | ---------------------------- | -------------- |
| File system operations      | `contract_file_system_port.py`          | `IFileSystemPort`          | Infrastructure |
| Forbidden import checking   | `contract_import_forbidden_protocol.py` | `IImportForbiddenProtocol` | Capabilities   |
| Import runner orchestration | `contract_import_runner_aggregate.py`   | `IImportRunnerAggregate`   | Agents         |

ABC names MUST use:

```text
I<Name>Port
I<Name>Protocol
I<Name>Aggregate
```

---

## Purity and Import Restrictions (AES201)

Contract files must remain pure.

### Allowed Imports

| Contract File               | May Import From                          |
| --------------------------- | ---------------------------------------- |
| `contract_*_port.py`      | taxonomy types, other contract types     |
| `contract_*_protocol.py`  | taxonomy types, other contract types     |
| `contract_*_aggregate.py` | taxonomy types, other contract types     |

### Forbidden Imports

Contract files MUST NOT import from:

- `capabilities_*`
- `infrastructure_*`
- `agent_*`
- `surface_*`
- concrete implementation classes

Bad:

```python
from capabilities_my_checker import MyChecker  # BAD
```

Good:

```python
from shared.code_analysis.taxonomy_source_vo import SourceContentVO
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
```

---

## ABC Structure Rules

### 1. Contracts contain ABC definitions only

Good:

```python
class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[LintResult]: ...
```

Bad:

```python
class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[LintResult]: ...

class MyChecker(IImportForbiddenProtocol):
    # implementation belongs in capabilities layer
    ...
```

---

### 2. No default method bodies

Default methods are implementation logic.

Bad:

```python
class ICheckerProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[LintResult]: ...

    def check_all(self, sources: list[SourceContentVO]) -> list[LintResult]:
        # BAD: default implementation in contract
        return []
```

If shared behavior is needed, put it in:

- capabilities helper,
- taxonomy utility if pure/domain-agnostic.

---

### 3. No private helpers or internal stepping stones

Bad:

```python
class IFileParserPort(ABC):
    @abstractmethod
    def parse_file(self, path: FilePath) -> ParsedData: ...

    @abstractmethod
    def _extract_rust_specific_regex(self, content: FileContent) -> list[SymbolName]:  # BAD
        ...
```

The second method is implementation-specific.

It belongs in the implementor:

```python
class RustFileParser(IFileParserPort):
    def parse_file(self, path: FilePath) -> ParsedData:
        ...

    def _extract_rust_specific_regex(self, content: FileContent) -> list[SymbolName]:
        # private helper
        ...
```

---

### 4. All methods MUST use `@abstractmethod`

Good:

```python
class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[LintResult]: ...
```

---

### 5. Contracts MUST inherit from `ABC`

```python
from abc import ABC, abstractmethod

class IFileSystemPort(ABC):
    ...
```

---

### 6. Error strategy

Prefer shared taxonomy error types in contract signatures.

Good:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

---

## Primitive and VO Rules

Contract signatures should use shared taxonomy VOs for domain data.

### Forbidden for domain values

| Primitive | Rule                                                                    |
| --------- | ----------------------------------------------------------------------- |
| `str`     | Forbidden for domain fields and public contract values. Use VO.         |
| `int`     | Forbidden for domain values. Use VO.                                    |
| `float`   | Forbidden for domain values. Use VO.                                    |
| `list[str]` | Forbidden for domain collections. Use list VO.                        |
| `dict`    | Forbidden for domain data. Use VO.                                      |

### Allowed with care

| Type   | Rule                                                                                |
| ------ | ----------------------------------------------------------------------------------- |
| `bool` | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for:

- file paths, symbol names, messages, line numbers,
- counts, severity, requests, results, identifiers, policies.

---

## Examples

### GOOD: Port Contract

```python
# contract_file_system_port.py

from abc import ABC, abstractmethod
from shared.file_system.taxonomy_file_content_vo import FileContent
from shared.file_system.taxonomy_file_path_vo import FilePath
from shared.file_system.taxonomy_file_read_error import FileReadError
from shared.file_system.taxonomy_file_write_error import FileWriteError

class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> Result[FileContent, FileReadError]: ...

    @abstractmethod
    def write_file(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]: ...
```

Implemented by infrastructure.

---

### GOOD: Protocol Contract

```python
# contract_import_forbidden_protocol.py

from abc import ABC, abstractmethod
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.code_analysis.taxonomy_source_vo import SourceContentVO
from shared.import_rules.taxonomy_import_rule_list_vo import ImportRuleList

class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO, rules: ImportRuleList) -> list[LintResult]: ...
```

Implemented by capabilities.

---

### GOOD: Aggregate Contract

```python
# contract_import_runner_aggregate.py

from abc import ABC, abstractmethod
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.import_rules.taxonomy_import_scan_request_vo import ImportScanRequest

class IImportRunnerAggregate(ABC):
    @abstractmethod
    def run(self, request: ImportScanRequest) -> list[LintResult]: ...
```

Implemented by agents.

---

## Detection Patterns

### BAD: Contract Contains Implementation

```python
class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> FileContent: ...

class FileAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> FileContent:
        # BAD: implementation belongs in infrastructure_*.py
        with open(path.value()) as f:
            return FileContent(f.read())
```

Fix: Move implementation to infrastructure layer.

---

### BAD: Contract Imports Non-Taxonomy Types

```python
from capabilities_my_checker import MyChecker  # BAD

class IMyProtocol(ABC):
    @abstractmethod
    def check(self, checker: MyChecker) -> None: ...
```

Fix: Use taxonomy VOs and contract types only.

---

### BAD: Leaking Implementation Details

```python
class IFileParserPort(ABC):
    @abstractmethod
    def parse_file(self, path: FilePath) -> ParsedData: ...

    @abstractmethod
    def _extract_rust_specific_regex(self, content: FileContent) -> list[SymbolName]:  # BAD
        ...
```

Fix: Remove internal helper from contract.

---

### BAD: Raw Primitives for Domain Values

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: str) -> str: ...
```

Fix:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

---

## Workflow

### Step 1: Determine the Contract Role

Ask:

> "Which layer will implement this interface?"

| Implemented By | Suffix         |
| -------------- | -------------- |
| Infrastructure | `_port`      |
| Capabilities   | `_protocol`  |
| Agent          | `_aggregate` |

---

### Step 2: Identify Public Methods

Apply the Golden Rule:

```text
Is this method called by outer layers?
├─ YES → keep in contract
└─ NO → make it a private helper in implementation class
```

Remove:

- internal parsing steps,
- helper methods,
- algorithm-specific methods,
- implementation stepping stones.

---

### Step 3: Create Contract File

Create:

```text
modules/shared/src/<domain>/contract_<concept>_<suffix>.py
```

Ensure:

- ABC name uses `I<Name>Port`, `I<Name>Protocol`, or `I<Name>Aggregate`,
- inherits from `ABC`,
- all methods use `@abstractmethod`,
- signatures use taxonomy VOs,
- no implementation exists.

---

### Step 4: Register Module

Update:

```text
modules/shared/src/<domain>/__init__.py
```

Example:

```python
from .contract_file_system_port import IFileSystemPort
from .contract_import_forbidden_protocol import IImportForbiddenProtocol
from .contract_import_runner_aggregate import IImportRunnerAggregate
```

---

### Step 5: Implement in Layer File

Infrastructure:

```python
from shared.file_system.contract_file_system_port import IFileSystemPort

class FileSystemAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> Result[FileContent, FileReadError]:
        # infrastructure implementation
        ...

    def write_file(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]:
        # infrastructure implementation
        ...
```

Private helpers stay in the implementor:

```python
class FileSystemAdapter:
    def _normalize_path(self, path: FilePath) -> FilePath:
        # private helper
        ...
```

---

### Step 6: Verify

```bash
python -c "import <module>"
```

---

## Verification Checklist

- [ ] Contract file uses correct suffix: `_port`, `_protocol`, or `_aggregate`.
- [ ] Contract contains only ABC definitions.
- [ ] Contract contains no method implementations.
- [ ] Contract contains no default method bodies.
- [ ] Contract contains no private helper signatures.
- [ ] Contract contains no implementation-specific stepping stones.
- [ ] ABC inherits from `ABC`.
- [ ] All methods use `@abstractmethod` decorator.
- [ ] Contract imports only taxonomy and contract types.
- [ ] Contract does not import from capabilities.
- [ ] Contract does not import from infrastructure.
- [ ] Contract does not import from agents.
- [ ] Contract does not import from surface.
- [ ] Contract signatures use shared VOs for domain data.
- [ ] Primitive types are not used for domain values.
- [ ] Error types come from shared taxonomy.
- [ ] New contract module is registered in `__init__.py`.
- [ ] `python -c "import <module>"` passes.

---

## Quick Commands

```bash
# List contract ABCs
grep -n "^class I[A-Za-z0-9_]*Port\|^class I[A-Za-z0-9_]*Protocol\|^class I[A-Za-z0-9_]*Aggregate" modules/shared/src/**/contract_*.py

# Check forbidden imports in contract files
grep -n "from capabilities_\|from infrastructure_\|from agent_\|from surface_" modules/shared/src/*/contract_*.py

# Check possible raw primitive signatures
grep -n "def .*(str\|int\|float\|list\[str\]\|dict)" modules/shared/src/**/contract_*.py

# Check methods without @abstractmethod
grep -n "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -q "@abstractmethod" "$file" || echo "NO ABSTRACT: $file"
done

# Check unregistered contract files
for file in modules/shared/src/<domain>/contract_*.py; do
  name=$(basename "$file" .py)
  grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py \
    || echo "UNREGISTERED: $name"
done
```

---

## Common Mistakes

- Putting implementation logic in contract files.
- Adding default method bodies to contract ABCs.
- Importing concrete layer types into contracts.
- Importing capabilities, infrastructure, agents, or surface modules into contracts.
- Using wrong suffix for contract files.
- Leaking implementation details into contract traits.
- Putting internal stepping stones into contract ABCs.
- Creating god interfaces with too many unrelated methods.
- Forgetting `@abstractmethod` decorators on methods.
- Forgetting to inherit from `ABC`.
- Using raw `str` for domain values in contract signatures.
- Using numeric primitives for domain values in contract signatures.
- Using `list[str]` instead of domain list VOs.
- Forgetting to register contract modules in `__init__.py`.
- Duplicating contract definitions across domains instead of placing shared contracts in `common/`.

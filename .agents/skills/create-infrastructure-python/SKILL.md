---
name: create-infrastructure-python
description: "Create and validate Python infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one class per file, port ABC contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
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
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create infrastructure python"
  - "add infrastructure python"
  - "fix infrastructure structure python"
  - "create port python"
  - "infrastructure missing port python"
  - "check infrastructure python"
  - "audit infrastructure python"
dependencies: []
related:
  - create-capabilities-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-infrastructure-structure-python
  - create-missing-ports-python
---

# create-infrastructure-python

## Purpose

Create and validate Python **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access,
- network calls,
- database access,
- external API calls,
- environment/system integration,
- technical mapping,
- serialization/deserialization,
- error mapping,
- adapter implementation for port ABCs.

Infrastructure MUST NOT contain business logic.

---

## Definition of Done

An infrastructure file is considered valid when:

1. It contains exactly **ONE implementation class**.
2. The class inherits exactly **ONE domain port ABC** in the class declaration.
3. Block 2 contains **ONLY** the port ABC method implementations.
4. Dunder methods, factory classmethods, and private helpers are placed in Block 3.
5. The file contains **zero business logic**.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.py`.
10. Adapter-specific helpers may remain inside the implementation file.
11. I/O errors are propagated explicitly.
12. `python -c "import <module>"` passes.

---

## Rules

### Layer Boundaries (AES)

#### Infrastructure Layer (`infrastructure_*.py`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| File I/O (`open()`, `Path()`, `os.`)                | Business rules                                        |
| Network calls (`requests.`, `httpx.`)               | Domain logic                                          |
| Database operations (`sqlite3.`, `asyncpg.`)        | Domain calculations                                   |
| External API calls                                  | Domain validation that decides business correctness   |
| Environment/system access via controlled adapter    | Direct import from concrete `agent_*` modules         |
| Serialization/deserialization                       | Direct import from concrete `capabilities_*` modules  |
| Technical mapping (DTO ↔ VO)                        | Locally defined domain data structures                |
| Error mapping from external libraries               | Raw primitives for domain values in public contracts  |
| Port ABC implementation                             | Silent error swallowing                               |
| Private helpers supporting the adapter              |                                                       |

Infrastructure may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- port ABCs
- protocol ABCs defined in shared, when required by the adapter contract

Infrastructure must not depend on concrete capabilities or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation class per file

Each infrastructure file contains exactly ONE main implementation class.

```python
class FileSystemSourceReader:
    # ...
```

Do not define multiple service classes in the same file.

---

#### 2. Only the implementation class may be defined in the layer file

An infrastructure file may define the implementation class only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in infrastructure files:

```python
@dataclass
class CacheEntry:
    key: str
    value: str
```

Allowed:

```python
from shared.cache.taxonomy_cache_entry_vo import CacheEntry
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, adapters, clients, repositories, or ports MUST use protocol interfaces.

```python
class OrphanFileCache:
    def __init__(self, store: IKeyValueStorePort):
        self._store = store
```

Do not use concrete service types:

```python
class OrphanFileCache:
    def __init__(self, store: RedisKeyValueStore):  # BAD: concrete dependency
        self._store = store
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, timeouts, thresholds, etc. should use shared VOs.

```python
class HttpManifestClient:
    def __init__(self, base_url: BaseUrl, timeout: TimeoutSeconds):
        self._base_url = base_url
        self._timeout = timeout
```

Avoid raw primitives for domain values:

```python
class HttpManifestClient:
    def __init__(self, base_url: str, timeout: int):  # BAD
        self._base_url = base_url
        self._timeout = timeout
```

---

### Helper vs Utility Decision

The boundary is not only about `self`/`cls`.

The real question is:

> Does this function know about adapter-specific or domain-specific rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

---

### When to Keep as Private Helper (Block 3)

Keep the function inside the infrastructure file if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It accesses adapter-specific static/state.
3. It performs adapter-specific mapping.
4. It maps external errors into port-specific errors.
5. It knows infrastructure-specific configuration.
6. It is tightly coupled to this adapter only.
7. It is a factory method such as `create_default()` or `from_config()`.
8. It is stateless but adapter-specific and only used by this class.

Example:

```python
class FileSystemSourceReader:
    def _map_io_error(self, path: FilePath, err: Exception) -> FileReadError:
        return FileReadError.io(path, err)
```

This helper is infrastructure-specific and may remain in Block 3.

---

### When to Extract to Utility (`*_utility.py`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or adapter rules.
5. Reusable: useful for multiple infrastructure/capabilities/modules.

Example:

```python
# shared/common/taxonomy_string_utility.py
def normalize_whitespace(input: str) -> str:
    return " ".join(input.split())
```

---

### I/O Blocker (CRITICAL)

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It belongs in infrastructure.

```python
def read_file_content(path: FilePath) -> Result[FileContent, FileReadError]:
    try:
        raw = path.value().read_text()
    except Exception as err:
        return Err(FileReadError.io(path, err))
    return FileContent.new(raw).map_err(FileReadError.validation)
```

Rule:

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT capabilities layer
```

---

## The 3-Block Structure

Every implementation file MUST follow this order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Port ABC Method Implementation**
3. **Block 3 — Dunder Methods, Factories, and Private Helpers**

---

### Block 1 — Class Definition & Constructor

```python
class FileSystemSourceReader(IFileReaderPort):
    def __init__(self) -> None:
        pass
```

Or with dependencies:

```python
class OrphanFileCache(IOrphanFileCachePort):
    def __init__(self, store: IKeyValueStorePort, policy: CachePolicy):
        self._store = store
        self._policy = policy
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain port ABC methods ONLY.

```python
class FileSystemSourceReader(IFileReaderPort):
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
        # port implementation
        ...
```

Do NOT put these in Block 2:

```python
def __repr__(self) -> str
def __str__(self) -> str
def __eq__(self, other: object) -> bool
def __hash__(self) -> int
@classmethod
def create_default(cls) -> "FileSystemSourceReader"
@staticmethod
def some_helper(...) -> ...
```

Those belong in Block 3.

---

### Block 3 — Dunder Methods, Factories, and Helpers

Block 3 contains:

- `__repr__`, `__str__`, `__eq__`, `__hash__`, `__copy__`, etc.
- Factory classmethods: `create_default()`, `from_config()`, `from_dict()`
- `@staticmethod` and `@classmethod` helpers that depend on class semantics
- Private helper methods (`_helper_name`) that use `self`

```python
class FileSystemSourceReader(IFileReaderPort):
    def __repr__(self) -> str:
        return "FileSystemSourceReader()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, FileSystemSourceReader)

    @classmethod
    def create_default(cls) -> "FileSystemSourceReader":
        return cls()

    def _ensure_parent_dir(self, path: FilePath) -> Result[None, FileWriteError]:
        # private helper
        ...
```

---

### Utility Functions Do Not Belong in Block 3

If a function is:

- stateless,
- pure,
- domain-agnostic,
- and reusable across multiple modules,

then extract it to shared utility.

```python
from shared.common.taxonomy_path_utility import normalize_relative_path
```

But if the function is adapter-specific or infrastructure-specific, it may remain in Block 3.

---

## Method Placement Decision Rule

```text
Method / function found in an infrastructure file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in infrastructure)
  │
  ├─ Is it defined as @abstractmethod in the port ABC?
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

---

## Example: Correct 3-Block Structure

```python
from shared.file_system.taxonomy_file_content_vo import FileContent
from shared.file_system.taxonomy_file_path_vo import FilePath
from shared.file_system.taxonomy_file_read_error import FileReadError
from shared.file_system.contract_file_reader_port import IFileReaderPort


# ─── Block 1: Class Definition & Constructor ──────────────
class FileSystemSourceReader(IFileReaderPort):
    def __init__(self) -> None:
        pass


    # ─── Block 2: Public Contract (domain port ONLY) ──────
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
        try:
            raw = path.value().read_text()
        except Exception as err:
            return Err(FileReadError.io(path, err))
        return FileContent.new(raw).map_err(FileReadError.validation)


    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "FileSystemSourceReader()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, FileSystemSourceReader)

    @classmethod
    def create_default(cls) -> "FileSystemSourceReader":
        return cls()

    def _is_not_found(self, err: Exception) -> bool:
        return isinstance(err, FileNotFoundError)
```

---

## Port Rules

### AES404 — Infrastructure Must Implement Port ABC

Every infrastructure class MUST inherit from a port ABC.

```python
class FileSystemSourceReader(IFileReaderPort):
    # public contract
    ...
```

---

### Port file naming

| Layer            | File Pattern            | Port File                         | Port Name             |
| ---------------- | ----------------------- | --------------------------------- | --------------------- |
| Capabilities     | `capabilities_*.py`   | `contract_<name>_protocol.py`   | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.py` | `contract_<name>_port.py`       | `I<Name>Port`       |
| Agents           | `agent_*.py`          | `contract_<name>_aggregate.py`  | `I<Name>Aggregate`  |

---

### Port content rules

The port ABC MUST contain only public contract methods.

Good:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

Bad:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...

    def private_helper(self) -> None: ...  # BAD: helper in port
```

---

### Constructors are not port methods

`__init__` and factory classmethods MUST stay in Block 1 / Block 3.

Bad:

```python
class IFileReaderPort(ABC):
    @classmethod
    @abstractmethod
    def new(cls) -> "IFileReaderPort":  # BAD
        ...
```

Good:

```python
class FileSystemSourceReader(IFileReaderPort):
    @classmethod
    def create_default(cls) -> "FileSystemSourceReader":
        return cls()
```

---

### Port methods should use shared VOs

Port contracts should avoid raw primitives for domain values.

Bad:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: str) -> str: ...
```

Good:

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

---

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.py` + inherit port ABC**

If no, and it contains business logic → **move to capabilities layer**

---

## Naming Convention

| Layer            | File Pattern            | Port File                         | Port Name             |
| ---------------- | ----------------------- | --------------------------------- | --------------------- |
| Capabilities     | `capabilities_*.py`   | `contract_<name>_protocol.py`   | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.py` | `contract_<name>_port.py`       | `I<Name>Port`       |
| Agents           | `agent_*.py`          | `contract_<name>_aggregate.py`  | `I<Name>Aggregate`  |

---

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```python
class FileCache:
    def read(self) -> str:
        # public behavior without port ABC
        ...
```

Fix:

```python
class FileCache(IFileCachePort):
    def read(self) -> str:
        # contract implementation
        ...
```

---

### BAD: Business Logic in Infrastructure

```python
class OrphanFileCache:
    def analyze(self, content: FileContent) -> bool:
        # BAD: domain logic
        return "orphan" in content.value
```

Fix:

Move analysis to capabilities.

```python
# capabilities_orphan_analyzer.py
class OrphanAnalyzer(IOrphanAnalyzerProtocol):
    def analyze(self, content: FileContent) -> OrphanAnalysisResult:
        # domain logic here
        ...
```

Infrastructure should only load/save/cache data.

---

### BAD: Data Class Defined in Layer File

```python
@dataclass
class CacheEntry:
    key: str
    value: str
```

Fix:

Move to shared taxonomy:

```python
# shared/cache/taxonomy_cache_entry_vo.py
@dataclass
class CacheEntry:
    key: CacheKey
    value: CacheValue
```

Then import it:

```python
from shared.cache.taxonomy_cache_entry_vo import CacheEntry
```

---

### BAD: Concrete Service Field

```python
class OrphanFileCache:
    def __init__(self, store: RedisKeyValueStore):  # BAD
        self._store = store
```

Fix:

```python
class OrphanFileCache:
    def __init__(self, store: IKeyValueStorePort):
        self._store = store
```

---

### BAD: Dunder Methods in Block 2

```python
class FileCacheAdapter(IFileReaderPort):
    def __init__(self, cache_dir: FilePath): ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a port method
        return "FileCacheAdapter()"

    def read(self, path: FilePath) -> str:  # ← pushed down
        ...

    def __eq__(self, other) -> bool:     # ← also in Block 2 position
        return isinstance(other, FileCacheAdapter)
```

Fix:

```python
class FileCacheAdapter(IFileReaderPort):
    def __init__(self, cache_dir: FilePath): ...

    def read(self, path: FilePath) -> str:  # ← Block 2: port method
        ...

    def __repr__(self) -> str:           # ← Block 3: dunder = utility
        return "FileCacheAdapter()"

    def __eq__(self, other) -> bool:     # ← Block 3
        return isinstance(other, FileCacheAdapter)
```

---

### GOOD: Implementor with Shared Data and DI

```python
from shared.cache.taxonomy_cache_policy_vo import CachePolicy
from shared.cache.contract_key_value_store_port import IKeyValueStorePort
from shared.orphan_detector.contract_orphan_file_cache_port import IOrphanFileCachePort

class OrphanFileCache(IOrphanFileCachePort):
    def __init__(self, store: IKeyValueStorePort, policy: CachePolicy):
        self._store = store
        self._policy = policy
```

---

### GOOD: Correct 3-Block with Dunder Methods

```python
class FileCacheAdapter(IFileReaderPort):

    def __init__(self, cache_dir: FilePath) -> None:  # Block 1: constructor
        self._cache_dir = cache_dir

    def read(self, path: FilePath) -> str:  # Block 2: port method ONLY
        ...

    def __repr__(self) -> str:               # Block 3: dunder = utility
        return f"FileCacheAdapter(cache_dir={self._cache_dir!r})"

    @classmethod
    def create_default(cls) -> "FileCacheAdapter":  # Block 3: factory
        return cls(cache_dir=FilePath(".cache"))
```

---

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask:

> Is this code pure I/O or external system integration?

If yes → keep as infrastructure.

If it contains business logic → move to capabilities.

Examples of business logic that must move out of infrastructure:

- deciding whether a file is orphan
- calculating domain severity
- validating business rules
- computing domain metrics
- interpreting domain meaning from content

Technical mapping is still allowed:

- DTO to VO mapping
- serialization
- deserialization
- external error mapping
- connection handling
- retry mechanics
- transport-level normalization

---

### Step 2: Check for Missing Port

Does the infrastructure class inherit from a port ABC?

If no:

1. create `contract_<name>_port.py`
2. define `I<Name>Port`
3. move public method signatures into the port
4. make the class inherit the port

---

### Step 3: Create Port File if Missing

Create port file in the appropriate shared domain folder.

Examples:

| Module           | Port Path                                                |
| ---------------- | -------------------------------------------------------- |
| import-rules     | `modules/shared/src/import_rules/contract_*_port.py`    |
| code-analysis    | `modules/shared/src/code_analysis/contract_*_port.py`   |
| orphan-detector  | `modules/shared/src/orphan_detector/contract_*_port.py` |

Register the module in the relevant `__init__.py`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. class definition + `__init__`
2. port ABC method implementations
3. dunder methods, factory classmethods, private helpers

---

### Step 5: Verify Class Discipline

Check:

- exactly one implementation class
- no local domain data classes
- no local enums/VOs/DTOs/constants
- service fields use protocol interfaces
- value fields use shared VOs

---

### Step 6: Verify Helper vs Utility Boundary

For each helper/function:

```text
Does it know adapter-specific or infrastructure-specific details?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.py
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure:

- no forbidden imports from concrete capabilities
- no forbidden imports from concrete agents
- no business logic
- no domain calculations
- no local domain data definitions

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `or ""` or `or 0` error swallowing
- fallible port methods return descriptive error types or raise meaningful exceptions
- I/O errors are propagated
- public contracts use shared VOs
- no magic constants for domain values

---

### Step 9: Verify Compilation

Run:

```bash
python -c "import <module>"
```

---

## Verification Checklist

- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation class + `__init__`.
- [ ] Block 2 contains ONLY the port ABC method implementations.
- [ ] Block 3 contains dunder methods, factories, and private helpers.
- [ ] Infrastructure class inherits a port ABC (AES404).
- [ ] Port contains only public contract methods.
- [ ] Private helpers are not declared in the port.
- [ ] Constructors are not declared in the port.
- [ ] Dunder methods are in Block 3.
- [ ] Adapter-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.py`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] One file contains exactly one implementation class.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use protocol interfaces via DI.
- [ ] Value/configuration fields use shared VOs.
- [ ] Infrastructure contains zero business logic.
- [ ] No forbidden imports from concrete `capabilities_*`.
- [ ] No forbidden imports from concrete `agent_*`.
- [ ] Port module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes.

---

## Error Handling Rules

Infrastructure error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```python
content = open(path.value()).read() or ""
```

Forbidden:

```python
value = result or 0
```

Unless the value is genuinely optional and the default is an explicit domain/technical decision.

---

### Rule 2: Fallible port methods should return `Result` or raise

If a port method can fail due to I/O, network, database, parsing, or validation, return a result type or raise a meaningful exception.

```python
def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

---

### Rule 3: Use descriptive error types

Prefer custom error types from shared taxonomy.

```python
class FileReadError(Enum):
    IO = "io"
    VALIDATION = "validation"
```

Avoid losing context:

```python
except Exception as e:
    return str(e)  # BAD: context lost
```

---

### Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs.

Lint violations are usually domain/analysis outcomes and belong to capabilities.

Bad:

```python
def read(self, path: FilePath) -> list[LintResult]:
    # BAD: infrastructure deciding lint outcomes
    ...
```

Good:

```python
def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
    # infrastructure returns data or error
    ...
```

Capabilities then decides whether an error becomes a lint violation.

---

### Proper Patterns

```python
# OK: explicit I/O error propagation
def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
    try:
        raw = path.value().read_text()
    except Exception as err:
        return Err(FileReadError.io(path, err))
    return FileContent.new(raw).map_err(FileReadError.validation)
```

```python
# OK: optional config with explicit default constant
def timeout(self) -> TimeoutSeconds:
    return self.config.timeout() or DEFAULT_TIMEOUT_SECONDS
```

---

## Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```python
class IFileWriterPort(ABC):
    @abstractmethod
    def write(self, path: str, content: str) -> None: ...
```

Good:

```python
class IFileWriterPort(ABC):
    @abstractmethod
    def write(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]: ...
```

### Primitive Policy

| Primitive | Rule |
| --------- | ---- |
| `str`     | Forbidden for domain fields and public contract values. Use VO. |
| `int`     | Forbidden for domain values. Use VO. |
| `float`   | Forbidden for domain values. Use VO. |
| `bool`    | Allowed for technical toggles when no richer VO is needed. |

Prefer VOs for:

- file paths
- URLs
- timeouts
- durations
- cache keys
- cache values
- query results
- identifiers
- messages

---

## Magic Constant Extraction Rules

No hardcoded domain literals in infrastructure.

Bad:

```python
def save(self) -> Result[None, FileWriteError]:
    Path("manifest.json").write_text(data)  # BAD: magic string
    return Ok(None)
```

Good:

```python
from shared.manifest.taxonomy_manifest_constant import MANIFEST_FILENAME

def save(self) -> Result[None, FileWriteError]:
    Path(MANIFEST_FILENAME.value).write_text(data)
    return Ok(None)
```

Constants MUST live in:

```text
taxonomy_*_constant.py
```

Technical defaults should also be named constants or come from configuration VOs.

---

## Import Strategy

When fixing cross-import violations in infrastructure, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```python
# shared/common/taxonomy_path_utility.py
def normalize_relative_path(path: str) -> str | None:
    return path[1:] if path.startswith('/') else None
```

Consumer:

```python
from shared.common.taxonomy_path_utility import normalize_relative_path
```

---

### Option B: Dependency Injection via Port ABC

Use when the code needs:

- state,
- collaborators,
- side effects,
- I/O,
- layer-specific implementation.

Example:

```python
# contract_file_writer_port.py
class IFileWriterPort(ABC):
    @abstractmethod
    def write(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]: ...
```

```python
# infrastructure_file_writer_adapter.py
class FileWriterAdapter(IFileWriterPort):
    def write(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]:
        try:
            path.value().write_text(content.value())
        except Exception as err:
            return Err(FileWriteError.io(path, err))
        return Ok(None)
```

```python
# consumer
class ReportPublisher:
    def __init__(self, writer: IFileWriterPort):
        self._writer = writer
```

The consumer depends only on the port trait, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in infrastructure?
  │
  ├─ Does it know adapter-specific or infrastructure-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need self or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → keep in infrastructure, not utility
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `python -c "import <module>"` or AST-based tooling.

```bash
# List classes in infrastructure files
grep -n "^class " modules/*/src/infrastructure_*.py

# List port ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Port" modules/*/src/infrastructure_*.py

# Check possible business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" modules/*/src/infrastructure_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(capabilities_|agent_)" modules/*/src/infrastructure_*.py

# Find error swallowing patterns
grep -n "or ''\|or \"\"\|or 0" modules/*/src/infrastructure_*.py

# Find possible magic numbers
grep -n "[0-9]\+\.[0-9]\+" modules/*/src/infrastructure_*.py | grep -v "#\|const\|import" | head -20

# Check syntax
python -c "import <module>"
```

---

### Check Wrong Block Order

```bash
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_port = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_port is None:
            first_port = i + 1
    if first_dunder and first_port and first_dunder < first_port:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before port method (line {first_port})')
" modules/*/src/infrastructure_*.py
```

---

## Common Mistakes

- Putting business logic in infrastructure.
- Putting domain calculations in infrastructure.
- Putting domain validation in infrastructure.
- Defining domain data classes in infrastructure files.
- Using concrete service types as constructor fields.
- Using raw primitives for domain value fields.
- Exposing raw primitives in public port contracts when a VO exists.
- Putting private helpers in the port ABC.
- Putting constructors in the port ABC.
- Placing dunder methods before the port ABC methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Extracting adapter-specific helpers to shared utility too early.
- Creating god ports with too many unrelated methods.
- Multiple implementation classes in one file.
- Direct dependency on concrete capabilities implementations.
- Direct dependency on concrete agent implementations.
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in infrastructure logic.
- Infrastructure returning lint results directly instead of returning data/errors to capabilities.

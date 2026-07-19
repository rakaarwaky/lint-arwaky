---
name: create-capabilities-python
description: "Create and validate Python capabilities layer files following AES rules: pure domain behavior, zero I/O, 3-block structure, one class per file, protocol ABC contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    python,
    aes,
    capability,
    protocol,
    structure,
    aes402,
    aes403,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create capability python"
  - "add capability python"
  - "fix capability structure python"
  - "create protocol python"
  - "capability missing protocol python"
  - "check capabilities python"
  - "audit capabilities python"
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

Create and validate Python **capabilities layer** files following clean architecture / AES rules.

A capabilities file must contain **pure domain behavior**:

- no I/O,
- no infrastructure detail,
- no agent detail,
- no locally defined domain data structures,
- one implementation class per file,
- one domain protocol ABC as the public contract,
- strict 3-block structure,
- dependency injection for service collaborators,
- shared VOs for domain data.

---

## Definition of Done

A capabilities file is considered valid when:

1. It contains exactly **ONE implementation class**.
2. The class inherits exactly **ONE domain protocol ABC** in the class declaration.
3. Block 2 contains **ONLY** the domain protocol method implementations.
4. Dunder methods, factory classmethods, and private helpers are placed in Block 3.
5. The file contains **zero I/O** and zero side-effecting infrastructure calls.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.py`.
10. Domain-specific helpers may remain inside the implementation file.
11. `python -c "import <module>"` passes.

---

## Rules

### Layer Boundaries (AES)

#### Capabilities Layer (`capabilities_*.py`)

| Allowed                                   | Forbidden                                               |
| ----------------------------------------- | ------------------------------------------------------- |
| Computation, validation, calculation      | File I/O (`open()`, `Path()`, `os.`)              |
| Data transformation, business rules       | Network calls (`requests.`, `httpx.`)               |
| Domain behavior using shared models       | Database operations (`sqlite3.`, `asyncpg.`)        |
| Protocol/ABC implementation               | Direct stdout/stderr printing                           |
| Private helpers supporting the impl class | Direct environment/system-clock/global-state mutation   |
| Calling injected port/protocol traits     | Direct import from`infrastructure_*`                  |
|                                           | Direct import from`agent_*`                           |
|                                           | Direct dependency on concrete`capabilities_*` modules |
|                                           | Locally defined domain data structures                  |

Capabilities may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol/port ABCs

Capabilities must not depend on concrete infrastructure or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation class per file

Each capabilities file contains exactly ONE main implementation class.

```python
class CapabilitiesOrphanAnalyzer:
    # ...
```

Do not define multiple service classes in the same file.

---

#### 2. Only the implementation class may be defined in the layer file

A capabilities file may define the implementation class only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in capabilities files:

```python
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

Allowed:

```python
from shared.orphan_detector.taxonomy_orphan_result_vo import OrphanResult
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, services, adapters, or ports MUST use protocol interfaces.

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(
        self,
        extractor: IOrphanFilenameExtractorProtocol,
        cache: IOrphanFileCachePort,
    ):
        self._extractor = extractor
        self._cache = cache
```

Do not use concrete service types:

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: FilenameExtractor):  # BAD: concrete dependency
        self._extractor = extractor
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, names, thresholds, etc. should use shared VOs.

```python
class FrameExporter:
    def __init__(self, output_dir: OutputDirectory):  # shared VO
        self._output_dir = output_dir
```

Avoid raw primitives for domain values:

```python
class FrameExporter:
    def __init__(self, output_dir: str):  # BAD: primitive domain value
        self._output_dir = output_dir
```

---

### Helper vs Utility Decision

The boundary is not only about `self`/`cls`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

---

#### Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It knows AES-specific patterns, layer names, suffixes, violation codes, or taxonomy conventions.
3. It accesses `self.field` or instance state.
4. It is tightly coupled to this capability only.
5. It is a factory method such as `create_default()` or `from_config()`.
6. It is stateless but only used by this one class and is domain-specific.

Example:

```python
class ContractRoleChecker:
    def _resolve_scope(self, scope: str) -> tuple[str, list[str]]:
        # Domain-specific parsing logic.
        # Even without `self`, this can remain a private helper
        # if only this checker uses it.
        ...
```

---

#### Extract to Utility (`*_utility.py`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/infrastructures/modules.

Example:

```python
# shared/code_analysis/taxonomy_string_utility.py
def match_whole_token(haystack: str, needle: str) -> bool:
    # generic token matching
    ...
```

---

#### I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in capabilities.

```python
# BAD in capabilities layer
def read_config(file_path: str) -> str | None:
    with open(file_path) as f:  # I/O
        return f.read()
```

Correct placement:

```python
# infrastructure_config_reader.py
class FileSystemConfigReader(IConfigReaderPort):
    def read(self, file_path: FilePath) -> Result[ConfigContent, ConfigReadError]:
        try:
            raw = file_path.value().read_text()
        except Exception as e:
            return Err(ConfigReadError.Io(e))
        return ConfigContent.new(raw).map_err(ConfigReadError.Validation)
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
2. **Block 2 — Domain Protocol Method Implementation**
3. **Block 3 — Dunder Methods, Factories, and Private Helpers**

---

### Block 1 — Class Definition & Constructor

```python
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None:
        pass
```

Or with dependencies:

```python
class CapabilitiesOrphanAnalyzer(ILineCheckerProtocol):
    def __init__(
        self,
        extractor: IOrphanFilenameExtractorProtocol,
        cache: IOrphanFileCachePort,
        policy: OrphanAnalysisPolicy,
    ):
        self._extractor = extractor
        self._cache = cache
        self._policy = policy
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol methods ONLY.

```python
class ArchLineChecker(ILineCheckerProtocol):
    def check_line_counts(
        self,
        file: str,
        definition: LayerDefinition | None,
        content: str,
        violations: list[LintResult],
    ) -> None:
        # domain behavior
        ...
```

Do NOT put these in Block 2:

```python
def __repr__(self) -> str
def __str__(self) -> str
def __eq__(self, other: object) -> bool
def __hash__(self) -> int
@classmethod
def create_default(cls) -> "ArchLineChecker"
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
class ArchLineChecker(ILineCheckerProtocol):
    def __repr__(self) -> str:
        return "ArchLineChecker()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, ArchLineChecker)

    @classmethod
    def create_default(cls) -> "ArchLineChecker":
        return cls()

    def _resolve_threshold(self, layer: str) -> int:
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
from shared.code_analysis.taxonomy_line_checker_utility import is_barrel_file
```

But if the function is domain-specific and only used by this class, it may remain in Block 3.

---

### Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in capabilities)
  │
  ├─ Is it defined as @abstractmethod in the protocol ABC?
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
from shared.code_analysis.taxonomy_file_path_vo import FilePath
from shared.code_analysis.taxonomy_layer_definition_vo import LayerDefinition
from shared.code_analysis.taxonomy_line_checker_protocol import ILineCheckerProtocol
from shared.code_analysis.taxonomy_line_checker_utility import is_barrel_file
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.code_analysis.taxonomy_source_vo import SourceContentVO


# ─── Block 1: Class Definition & Constructor ──────────────
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None:
        pass


    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def check_line_counts(
        self,
        file: FilePath,
        definition: LayerDefinition | None,
        source: SourceContentVO,
        violations: list[LintResult],
    ) -> None:
        basename = file.basename()

        if is_barrel_file(basename):
            return

        # Remaining domain logic...


    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "ArchLineChecker()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, ArchLineChecker)

    @classmethod
    def create_default(cls) -> "ArchLineChecker":
        return cls()

    def _is_layer_relevant(self, definition: LayerDefinition) -> bool:
        # Private helper specific to this checker.
        return True
```

---

## Protocol Rules

### AES403 — Capability Must Implement Protocol ABC

Every capability class MUST inherit from a domain protocol ABC.

```python
class CapabilitiesOrphanAnalyzer(IOrphanCheckerProtocol):
    # public contract
    ...
```

---

### Protocol file naming

| Layer          | File Pattern            | Protocol File                    | Protocol Name        |
| -------------- | ----------------------- | -------------------------------- | -------------------- |
| Capabilities   | `capabilities_*.py`   | `contract_<name>_protocol.py`  | `I<Name>Protocol`  |
| Infrastructure | `infrastructure_*.py` | `contract_<name>_port.py`      | `I<Name>Port`      |
| Agents         | `agent_*.py`          | `contract_<name>_aggregate.py` | `I<Name>Aggregate` |

---

### Protocol content rules

The protocol ABC MUST contain only public domain contract methods.

Good:

```python
class ILineCheckerProtocol(ABC):
    @abstractmethod
    def check_line_counts(
        self,
        file: FilePath,
        definition: LayerDefinition | None,
        source: SourceContentVO,
        violations: list[LintResult],
    ) -> None: ...
```

Bad:

```python
class ILineCheckerProtocol(ABC):
    @abstractmethod
    def check_line_counts(self, ...) -> None: ...

    def private_helper(self) -> None: ...  # BAD: helper in protocol
```

---

### Constructors are not protocol methods

`__init__` and factory classmethods MUST stay in Block 1 / Block 3.

Bad:

```python
class ILineCheckerProtocol(ABC):
    @classmethod
    @abstractmethod
    def new(cls) -> "ILineCheckerProtocol":  # BAD
        ...
```

Good:

```python
class ArchLineChecker(ILineCheckerProtocol):
    @classmethod
    def create_default(cls) -> "ArchLineChecker":
        return cls()
```

---

## Detection Patterns

### BAD: Capability Without Protocol (AES403)

```python
class FrameComposer:
    def compose_frame(self) -> None:
        # public behavior without protocol ABC
        ...
```

Fix:

```python
class FrameComposer(IFrameComposerProtocol):
    def compose_frame(self) -> None:
        # contract implementation
        ...
```

---

### BAD: I/O in Capabilities (AES404)

```python
class MyCapability:
    def process(self) -> None:
        content = open("file.txt").read()  # FORBIDDEN
```

Fix:

Move I/O to infrastructure or port implementation.

```python
# infrastructure_source_reader.py
class FileSystemSourceReader(ISourceReaderPort):
    def read(self, path: FilePath) -> Result[SourceContentVO, SourceReadError]:
        try:
            raw = path.value().read_text()
        except Exception as e:
            return Err(SourceReadError.Io(e))
        return SourceContentVO.new(path, raw).map_err(SourceReadError.Validation)
```

Capabilities receives already-loaded data:

```python
class ImportChecker(IImportCheckerProtocol):
    def check(self, source: SourceContentVO) -> list[LintResult]:
        # pure analysis
        return []
```

---

### BAD: Data Class Defined in Layer File

```python
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

Fix:

Move to shared taxonomy:

```python
# shared/orphan_detector/taxonomy_orphan_result_vo.py
@dataclass
class OrphanResult:
    is_orphan: OrphanFlag
    reason: OrphanReason
```

Then import it:

```python
from shared.orphan_detector.taxonomy_orphan_result_vo import OrphanResult
```

---

### BAD: Concrete Service Field

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: FilenameExtractor):  # BAD
        self._extractor = extractor
```

Fix:

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor
```

---

### BAD: Dunder Methods in Block 2

```python
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None: ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a protocol method
        return "ArchLineChecker()"

    def check_line_counts(self, ...) -> None:  # ← pushed down
        ...

    def __eq__(self, other) -> bool:     # ← also in Block 2 position
        return isinstance(other, ArchLineChecker)
```

Fix:

```python
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None: ...

    def check_line_counts(self, ...) -> None:  # ← Block 2: protocol method
        ...

    def __repr__(self) -> str:           # ← Block 3: dunder = utility
        return "ArchLineChecker()"

    def __eq__(self, other) -> bool:     # ← Block 3
        return isinstance(other, ArchLineChecker)
```

---

### GOOD: Capability with DI and Shared VO

```python
from shared.orphan_detector.taxonomy_orphan_analysis_policy_vo import OrphanAnalysisPolicy
from shared.orphan_detector.contract_orphan_file_cache_port import IOrphanFileCachePort
from shared.orphan_detector.contract_orphan_filename_extractor_protocol import IOrphanFilenameExtractorProtocol
from shared.orphan_detector.contract_capabilities_orphan_protocol import ICapabilitiesOrphanProtocol

class CapabilitiesOrphanAnalyzer(ICapabilitiesOrphanProtocol):
    def __init__(
        self,
        extractor: IOrphanFilenameExtractorProtocol,
        cache: IOrphanFileCachePort,
        policy: OrphanAnalysisPolicy,
    ):
        self._extractor = extractor
        self._cache = cache
        self._policy = policy
```

---

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask:

> Is this pure domain behavior?

If yes → keep as capabilities.

If no → move I/O or side-effecting code to infrastructure.

Examples of code that must move out of capabilities:

- `open()`, `Path()`, `os.*`
- `requests`, `httpx`
- `sqlite3`, `asyncpg`
- direct printing
- environment mutation
- system clock access
- global state mutation

---

### Step 2: Check Missing Protocol (AES403)

Does the capability class inherit from a protocol ABC?

If no:

1. create `contract_<name>_protocol.py`
2. define `I<Name>Protocol`
3. move public domain method signatures into the ABC
4. make the class inherit the ABC

---

### Step 3: Create Protocol File if Missing

Create protocol file in the appropriate shared domain folder.

Examples:

| Module          | Protocol Path                                                 |
| --------------- | ------------------------------------------------------------- |
| import-rules    | `modules/shared/src/import_rules/contract_*_protocol.py`    |
| code-analysis   | `modules/shared/src/code_analysis/contract_*_protocol.py`   |
| orphan-detector | `modules/shared/src/orphan_detector/contract_*_protocol.py` |

Register the module in the relevant `__init__.py`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. class definition + `__init__`
2. domain protocol method implementations
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
Does it know domain rules?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.py
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure no forbidden imports or I/O patterns.

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `or ""` or `or 0` error swallowing
- fallible operations return descriptive error types or raise meaningful exceptions
- check/analysis methods may return `list[LintResult]`
- domain data uses VOs
- no magic constants

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
- [ ] Block 2 contains ONLY the domain protocol method implementations.
- [ ] Block 3 contains dunder methods, factories, and private helpers.
- [ ] Capability class inherits a protocol ABC (AES403).
- [ ] Protocol contains only public domain contract methods.
- [ ] Private helpers are not declared in the protocol.
- [ ] Constructors are not declared in the protocol.
- [ ] Dunder methods are in Block 3.
- [ ] Domain-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.py`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] One file contains exactly one implementation class.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use protocol interfaces via DI.
- [ ] Value/configuration fields use shared VOs.
- [ ] Zero I/O in capabilities layer (AES404).
- [ ] No forbidden imports from `infrastructure_*`.
- [ ] No forbidden imports from `agent_*`.
- [ ] No direct dependency on concrete `capabilities_*` implementations.
- [ ] Protocol module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes.

---

## Error Handling Rules

Capabilities error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```python
value = result or ""
```

Forbidden:

```python
value = result or 0
```

---

### Rule 2: Fallible operations should return `Result` or raise

If a method represents an operation that can fail unexpectedly, return a result type or raise a meaningful exception.

```python
def parse_manifest(content: ManifestContent) -> Result[Manifest, ManifestParseError]:
    # ...
    ...
```

---

### Rule 3: Check/analysis methods may return `list[LintResult]`

For linting/analysis use cases, violations are expected domain outcomes.

```python
def check_imports(source: SourceContentVO) -> list[LintResult]:
    violations: list[LintResult] = []

    # analysis logic

    return violations
```

This is allowed.

---

### Rule 4: I/O errors belong to infrastructure/port implementations

Bad in capabilities:

```python
def check_file(path: FilePath) -> list[LintResult]:
    content = open(path.value()).read()  # BAD: I/O in capabilities
    return []
```

Good:

```python
# infrastructure_source_reader.py
class FileSystemSourceReader(ISourceReaderPort):
    def read(self, path: FilePath) -> Result[SourceContentVO, SourceReadError]:
        try:
            raw = path.value().read_text()
        except Exception as e:
            return Err(SourceReadError.Io(e))
        return SourceContentVO.new(path, raw).map_err(SourceReadError.Validation)
```

```python
# capabilities_import_checker.py
class ImportChecker(IImportCheckerProtocol):
    def check(self, source: SourceContentVO) -> list[LintResult]:
        # pure analysis using already-read source
        return []
```

---

## Primitive-to-VO Replacement Rules (AES402)

### General Rule

Domain data MUST use shared VOs, not raw primitives.

Bad:

```python
@dataclass
class LintResult:
    file_path: str
    line: int
    severity: str
```

Good:

```python
@dataclass
class LintResult:
    file_path: FilePath
    line: LineNumber
    severity: Severity
```

---

### Primitive Policy

| Primitive | Rule                                                            |
| --------- | --------------------------------------------------------------- |
| `str`   | Forbidden for domain fields and contract return values. Use VO. |
| `int`   | Forbidden. Use domain VO.                                       |
| `float` | Forbidden. Use domain VO.                                       |
| `bool`  | Allowed for semantic toggles when no richer VO is needed.       |

Prefer VOs for:

- file paths
- symbol names
- messages
- line numbers
- column numbers
- severity
- durations
- counts
- thresholds
- identifiers

---

## Magic Constant Extraction Rules

No hardcoded domain literals in capabilities.

Bad:

```python
def calculate_duration(self) -> float:
    return 0.5
```

Good:

```python
from shared.animator.taxonomy_animator_constant import MIN_REVEAL_SECONDS

def calculate_duration(self) -> float:
    return MIN_REVEAL_SECONDS
```

Constants MUST live in:

```text
taxonomy_*_constant.py
```

---

## Import Strategy

When fixing cross-import violations in capabilities, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```python
# shared/code_analysis/taxonomy_path_utility.py
def normalize_relative_path(path: str) -> str | None:
    return path[1:] if path.startswith('/') else None
```

Consumer:

```python
from shared.code_analysis.taxonomy_path_utility import normalize_relative_path
```

---

### Option B: Dependency Injection via Port/Protocol Trait

Use when the code needs:

- state,
- collaborators,
- side effects,
- infrastructure behavior,
- layer-specific implementation.

Example:

```python
# contract_output_path_builder_protocol.py
class IOutputPathBuilderProtocol(ABC):
    @abstractmethod
    def build_frame_path(self, frame: Frame) -> FrameOutputPath: ...
```

```python
# capabilities_frame_exporter.py
class FrameExporter(IFrameExporterProtocol):
    def __init__(self, path_builder: IOutputPathBuilderProtocol):
        self._path_builder = path_builder

    def export(self, frame: Frame) -> FrameOutputPath:
        return self._path_builder.build_frame_path(frame)
```

The capability depends only on the protocol, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need self or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `python -c "import <module>"` or AST-based tooling.

```bash
# Check possible I/O in capabilities (AES404)
grep -n "open(\|Path(\|os\.\|requests\.\|httpx\.\|sqlite3\.\|asyncpg\." modules/*/src/capabilities_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(infrastructure_|agent_)" modules/*/src/capabilities_*.py

# List classes in capabilities files
grep -n "^class " modules/*/src/capabilities_*.py

# List protocol ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Protocol" modules/*/src/capabilities_*.py

# Find error swallowing patterns
grep -n "or ''\|or \"\"\|or 0" modules/*/src/capabilities_*.py

# Find possible magic numbers
grep -n "[0-9]\+\.[0-9]\+" modules/*/src/capabilities_*.py | grep -v "#\|const\|import" | head -20

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

---

## Common Mistakes

- Putting I/O in capabilities.
- Defining domain data classes in capabilities files.
- Using concrete service types as constructor fields.
- Using raw primitives for domain value fields.
- Putting private helpers in the protocol ABC.
- Putting constructors in the protocol ABC.
- Placing dunder methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Extracting domain-specific single-consumer helpers to shared utility too early.
- Creating god protocols with too many unrelated methods.
- Multiple implementation classes in one file.
- Direct dependency on concrete capabilities implementations.
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in capabilities logic.

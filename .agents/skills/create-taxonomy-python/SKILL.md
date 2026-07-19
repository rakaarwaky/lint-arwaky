---
name: create-taxonomy-python
description: "Create and validate Python taxonomy layer files in shared taxonomy: VOs, entities, errors, events, constants, and pure reusable utilities. Ensures domain data lives only in shared taxonomy and remains pure."
version: 1.3.0
category: refactoring
tags:
  [
    python,
    aes,
    taxonomy,
    shared,
    vo,
    entity,
    error,
    event,
    constant,
    utility,
    aes201,
    primitive-to-vo,
  ]
triggers:
  - "create taxonomy python"
  - "add taxonomy python"
  - "move dataclass to taxonomy python"
  - "create vo python"
  - "create error taxonomy python"
  - "create constant taxonomy python"
  - "check taxonomy python"
  - "audit taxonomy python"
dependencies: []
related:
  - create-capabilities-python
  - create-infrastructure-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - fix-primitive-to-vo
  - fix-magic-constant
---

# create-taxonomy-python

## Purpose

Create and validate Python **taxonomy layer** files inside `modules/shared/src/<domain>/`.

Taxonomy is the single source of truth for:

- value objects,
- entities,
- domain errors,
- domain events,
- constants,
- pure reusable utility functions.

No domain data structures may be defined in:

- capabilities,
- infrastructure,
- agents,
- surface,
- root/container layers.

Those layers must import domain data from shared taxonomy.

---

## Definition of Done

A taxonomy change is considered valid when:

1. Domain data structures live in `shared/taxonomy`.
2. Taxonomy file naming uses the allowed strict suffixes.
3. Taxonomy files do not import from capability, infrastructure, agent, surface, or root layers.
4. Taxonomy files contain no I/O and no side effects.
5. Utility functions in `*_utility.py` are stateless, pure, domain-agnostic, and reusable.
6. Value objects validate on construction.
7. Public domain contracts use VOs instead of raw primitives.
8. New taxonomy modules are registered in the relevant `__init__.py`.
9. `python -c "import <module>"` passes.

---

## The Fundamental Question

> **"Is this a dataclass or an implementor?"**

### Dataclass

A dataclass is a type that carries domain data.

Examples:

- value objects,
- DTOs,
- result objects,
- domain entities,
- domain errors,
- domain events,
- enums representing domain values.

These MUST live in shared taxonomy.

```python
@dataclass(frozen=True)
class OrphanAnalysisResult:
    is_orphan: bool
    reason: str
```

### Implementor

An implementor is a class that inherits a protocol and contains behavior, often with injected dependencies.

Examples:

- `capabilities_*.py`
- `infrastructure_*.py`
- `agent_*.py`

These stay in their layer files.

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor
```

---

## Taxonomy Layer Structure

Use snake_case module directories.

```text
modules/shared/src/
├── __init__.py
├── common/
│   ├── __init__.py
│   ├── taxonomy_*_vo.py
│   ├── taxonomy_*_error.py
│   ├── taxonomy_*_constant.py
│   └── taxonomy_*_utility.py
│
├── <domain>/
│   ├── __init__.py
│   ├── contract_*_protocol.py
│   ├── contract_*_port.py
│   ├── contract_*_aggregate.py
│   ├── taxonomy_*_vo.py
│   ├── taxonomy_*_entity.py
│   ├── taxonomy_*_error.py
│   ├── taxonomy_*_event.py
│   ├── taxonomy_*_constant.py
│   └── taxonomy_*_utility.py
```

Important:

- `contract_*.py` files are NOT taxonomy files.
- Contract ABCs may import taxonomy types.
- Taxonomy files MUST NOT import contract ABCs.

---

## File Naming Convention

Taxonomy files MUST use strict suffixes.

| Suffix        | Purpose                            | Example                                |
| ------------- | ---------------------------------- | -------------------------------------- |
| `_vo`       | Value objects and value-like enums | `taxonomy_file_path_vo.py`           |
| `_entity`   | Entities with identity             | `taxonomy_analysis_entity.py`        |
| `_error`    | Error types                        | `taxonomy_config_error.py`           |
| `_event`    | Event/message types                | `taxonomy_scan_event.py`             |
| `_constant` | Static compile-time constants      | `taxonomy_layer_names_constant.py`   |
| `_utility`  | Stateless pure reusable functions  | `taxonomy_symbol_renamer_utility.py` |

Allowed taxonomy prefixes:

```text
taxonomy_*_vo.py
taxonomy_*_entity.py
taxonomy_*_error.py
taxonomy_*_event.py
taxonomy_*_constant.py
taxonomy_*_utility.py
```

No other taxonomy suffixes are allowed.

---

## Purity and Import Restrictions (AES201)

Taxonomy must remain pure and stable.

### Allowed Dependencies

| Taxonomy Type | May Import From                              | Must Not Import From                                                |
| ------------- | -------------------------------------------- | ------------------------------------------------------------------- |
| `_vo`       | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_entity`   | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_error`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_event`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values                      | external layer imports, I/O, functions                              |
| `_utility`  | taxonomy types, pure stdlib helpers         | capabilities, infrastructure, agents, surface, root, contracts, I/O |

Taxonomy may contain:

- value validation,
- domain invariants inside constructors,
- pure transformations between taxonomy types.

Taxonomy must not contain:

- file I/O,
- network calls,
- database access,
- environment mutation,
- side effects,
- business orchestration,
- use-case logic,
- layer-specific behavior.

---

## Dataclass Patterns

### Value Objects (`_vo.py`)

A value object should wrap domain values with type safety and validation.

Prefer frozen dataclasses.

Bad:

```python
@dataclass
class FilePath:
    value: str
```

Good:

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class FilePath:
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("FilePath cannot be empty")

    @property
    def value(self) -> str:
        return self._value

    def __str__(self) -> str:
        return self._value
```

For simple wrappers, dataclass decorators may be used:

```python
@dataclass(frozen=True)
class FieldNameVO:
    value: str

@dataclass(frozen=True)
class BooleanVO:
    value: bool

@dataclass(frozen=True)
class SeverityVO:
    value: int
```

---

### Composite Value Objects

Composite VOs should use other VOs as fields, not raw primitives.

Bad:

```python
@dataclass(frozen=True)
class ImportRuleVO:
    pattern: str
    message: str
```

Good:

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class ImportRuleVO:
    pattern: RulePattern
    message: RuleMessage

    def __post_init__(self) -> None:
        if not self.pattern.value.strip():
            raise ValueError("RulePattern cannot be empty")

    @property
    def pattern_value(self) -> RulePattern:
        return self.pattern

    @property
    def message_value(self) -> RuleMessage:
        return self.message
```

---

### Entities (`_entity.py`)

Entities represent domain objects with identity.

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class SymbolEntity:
    id: SymbolId
    name: SymbolName

    @property
    def id_value(self) -> SymbolId:
        return self.id

    @property
    def name_value(self) -> SymbolName:
        return self.name
```

---

### Error Types (`_error.py`)

Use Python exceptions.

Prefer VO fields instead of raw public strings.

Bad:

```python
class ConfigError(Exception):
    def __init__(self, key: str, message: str):
        self.key = key
        self.message = message
        super().__init__(f"Config error: {key} - {message}")
```

Good:

```python
class ConfigError(Exception):
    def __init__(self, key: ConfigKey, message: ErrorMessage):
        self._key = key
        self._message = message
        super().__init__(f"Config error for {key.value}: {message.value}")

    @property
    def key(self) -> ConfigKey:
        return self._key

    @property
    def message(self) -> ErrorMessage:
        return self._message
```

If an error wraps lower-level errors:

```python
class FileReadError(Exception):
    def __init__(self, path: FilePath, cause: Exception):
        self._path = path
        self._cause = cause
        super().__init__(f"Failed to read file {path.value}: {cause}")

    @property
    def path(self) -> FilePath:
        return self._path

    @property
    def cause(self) -> Exception:
        return self._cause
```

---

### Event Types (`_event.py`)

Events represent something that happened in the domain.

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class ScanCompletedEvent:
    scan_id: ScanId

    @property
    def scan_id_value(self) -> ScanId:
        return self.scan_id
```

---

### Constants (`_constant.py`)

Constants are pure static values.

```python
# Default frames per second for animation.
FPS_DEFAULT: float = 24.0

# Minimum reveal time in seconds.
MIN_REVEAL_SECONDS: float = 0.5

# Manifest filename.
MANIFEST_FILENAME: str = "manifest.json"
```

Rules:

- no functions,
- no I/O,
- no external layer imports,
- no mutable state.

Constants may be primitive scalars. Consumers should wrap domain-meaningful primitives into VOs when exposing them in public domain contracts.

---

## Utility Functions (`_utility.py`)

Utility files contain pure reusable tools.

### The Ultimate Boundary

A function belongs in `*_utility.py` ONLY if ALL of these are true:

1. Stateless: no `self`, no `cls`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no randomness, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Multi-consumer reusable: useful for multiple modules/layers.

---

### Good Utility Example

```python
# taxonomy_token_utility.py

def match_whole_token(haystack: str, needle: str) -> bool:
    if not needle:
        return False

    import re
    pattern = rf'(?<!\w){re.escape(needle)}(?!\w)'
    return bool(re.search(pattern, haystack))
```

This is a dumb reusable tool.

---

### Bad Utility: Domain Knowledge

```python
# BAD: knows AES layer mapping rules
def get_target_layer_from_suffix(suffix: str) -> str:
    if suffix == "port":
        return "infrastructure"
    elif suffix == "protocol":
        return "capabilities"
    else:
        return "unknown"
```

This belongs in capabilities as a private helper.

---

### Bad Utility: Single Consumer Only

```python
# BAD: only used by one checker
def format_import_violation(rule: ImportRuleVO) -> str:
    return f"Import rule violation: {rule.pattern_value}"
```

If only one capability uses it, keep it as a private helper in that capability.

---

## Primitive-to-VO Rules

Taxonomy is the layer that provides VO replacements for primitives.

### General Rule

Domain data MUST use VOs, not raw primitives.

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
@dataclass(frozen=True)
class LintResult:
    file_path: FilePath
    line: LineNumber
    severity: Severity
```

---

### Primitive Policy

This policy must stay consistent with capabilities and infrastructure skills.

| Primitive | Rule                                                                                |
| --------- | ----------------------------------------------------------------------------------- |
| `str`     | Forbidden for domain fields and public contract return values. Use VO.              |
| `int`     | Forbidden for domain values. Use VO.                                                |
| `float`   | Forbidden for domain values. Use VO.                                                |
| `bool`    | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for:

- file paths,
- symbol names,
- messages,
- line numbers,
- column numbers,
- severity levels,
- durations,
- counts,
- thresholds,
- identifiers.

---

### VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

Good:

```python
@dataclass(frozen=True)
class LineNumber:
    _value: int

    def __post_init__(self) -> None:
        if self._value == 0:
            raise ValueError("LineNumber must be positive")

    @property
    def value(self) -> int:
        return self._value
```

If validation cannot fail, a simpler constructor may be used.

---

### Optional and Collection Primitives

Bad:

```python
@dataclass(frozen=True)
class RuleSet:
    patterns: list[str]
    description: str | None
```

Good:

```python
@dataclass(frozen=True)
class RuleSet:
    patterns: PatternList
    description: RuleDescription | None
```

Use:

- list VOs for collections,
- optional VOs or `Optional[VO]` when semantically optional.

---

## Detection Patterns

### BAD: Dataclass Defined in Capabilities

```python
# capabilities_orphan_analyzer.py

@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

Fix:

Move to taxonomy.

```python
# shared/orphan_detector/taxonomy_orphan_result_vo.py
@dataclass(frozen=True)
class OrphanResult:
    is_orphan: OrphanFlag
    reason: OrphanReason
```

Then import:

```python
from shared.orphan_detector.taxonomy_orphan_result_vo import OrphanResult
```

---

### BAD: Dataclass Defined in Infrastructure

```python
# infrastructure_file_cache.py

@dataclass
class CacheEntry:
    key: str
    value: str
```

Fix:

```python
# shared/cache/taxonomy_cache_entry_vo.py
@dataclass(frozen=True)
class CacheEntry:
    key: CacheKey
    value: CacheValue
```

---

### BAD: Raw Primitive Fields in Taxonomy VO

```python
@dataclass(frozen=True)
class ImportRuleVO:
    pattern: str
    message: str
```

Fix:

```python
@dataclass(frozen=True)
class ImportRuleVO:
    pattern: RulePattern
    message: RuleMessage
```

---

### BAD: Taxonomy Importing Layer Code

```python
# taxonomy_orphan_vo.py

from capabilities_orphan_analyzer import OrphanAnalyzer  # BAD
```

Taxonomy must not import from layers.

---

### BAD: Domain Rule Inside Utility

```python
# taxonomy_layer_utility.py

def is_port_trait_name(name: str) -> bool:
    return name.endswith("Port")
```

If this knows AES naming conventions or layer rules, it is domain knowledge.

It belongs in capabilities as a helper, not taxonomy utility.

---

### GOOD: Dataclass in Taxonomy + Implementor with DI

```python
# shared/orphan_detector/taxonomy_orphan_analysis_result_vo.py
@dataclass(frozen=True)
class OrphanAnalysisResult:
    is_orphan: OrphanFlag
    reason: OrphanReason
```

```python
# capabilities_orphan_analyzer.py
from shared.orphan_detector.taxonomy_orphan_analysis_result_vo import OrphanAnalysisResult
from shared.orphan_detector.contract_orphan_filename_extractor_protocol import IOrphanFilenameExtractorProtocol
from shared.orphan_detector.contract_orphan_file_cache_port import IOrphanFileCachePort

class CapabilitiesOrphanAnalyzer:
    def __init__(
        self,
        extractor: IOrphanFilenameExtractorProtocol,
        cache: IOrphanFileCachePort,
    ):
        self._extractor = extractor
        self._cache = cache
```

Service dependencies use DI.

Value/result data comes from taxonomy.

---

## Workflow

### Step 1: Identify the Dataclass

When you find a class or dataclass in a layer file, ask:

> Is this a dataclass or an implementor?

If it carries domain data:

- result object,
- DTO,
- VO,
- entity,
- error,
- event,
- enum,
- constant,

then move it to taxonomy.

If it inherits a protocol and uses DI, keep it in the layer file.

---

### Step 2: Determine Taxonomy Domain

Choose the correct domain directory under:

```text
modules/shared/src/<domain>/
```

Examples:

| Domain          | Directory                       | Example Types                         |
| --------------- | ------------------------------- | ------------------------------------- |
| common          | `shared/src/common/`          | cross-domain VOs, errors, utilities   |
| orphan_detector | `shared/src/orphan_detector/` | orphan results, reasons, flags        |
| code_analysis   | `shared/src/code_analysis/`   | analysis results, symbols, violations |
| import_rules    | `shared/src/import_rules/`    | import rules, patterns, messages      |
| naming_rules    | `shared/src/naming_rules/`    | naming violations, patterns           |

If a type is used by multiple domains, put it in `common/`.

---

### Step 3: Create or Update Taxonomy File

Use the correct suffix:

```text
taxonomy_*_vo.py
taxonomy_*_entity.py
taxonomy_*_error.py
taxonomy_*_event.py
taxonomy_*_constant.py
taxonomy_*_utility.py
```

Example:

```bash
mkdir -p modules/shared/src/orphan_detector
touch modules/shared/src/orphan_detector/taxonomy_orphan_result_vo.py
```

---

### Step 4: Register Module

Update the domain `__init__.py`.

```python
# shared/src/orphan_detector/__init__.py

from .taxonomy_orphan_result_vo import OrphanResult
from .taxonomy_orphan_reason_vo import OrphanReason
from .contract_orphan_protocol import IOrphanProtocol
from .contract_orphan_file_cache_port import IOrphanFileCachePort
```

---

### Step 5: Update Imports in Layer Files

Before:

```python
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

After:

```python
from shared.orphan_detector.taxonomy_orphan_result_vo import OrphanResult
```

---

### Step 6: Verify Purity

Check that taxonomy files do not import from:

- capabilities,
- infrastructure,
- agents,
- surface,
- root containers,
- contract ABCs.

Also check that taxonomy utilities do not perform I/O.

---

### Step 7: Verify Primitive-to-VO Compliance

Ensure:

- no public raw `str` domain fields,
- no numeric primitive domain fields,
- VOs validate on construction,
- contract ABCs use taxonomy VOs.

---

### Step 8: Verify Compilation

```bash
python -c "import <module>"
```

---

## Verification Checklist

- [ ] All domain dataclasses live in shared/taxonomy.
- [ ] No domain classes with data are defined in layer files.
- [ ] Taxonomy file naming uses allowed suffixes only.
- [ ] Taxonomy files do not import from capabilities.
- [ ] Taxonomy files do not import from infrastructure.
- [ ] Taxonomy files do not import from agents.
- [ ] Taxonomy files do not import from surface.
- [ ] Taxonomy files do not import from root containers.
- [ ] Taxonomy files do not import contract ABCs.
- [ ] Taxonomy files contain no I/O.
- [ ] Taxonomy utilities are stateless, pure, domain-agnostic, and multi-consumer.
- [ ] Domain-specific stateless helpers are NOT forced into taxonomy utility.
- [ ] Single-consumer helpers remain in their consuming layer.
- [ ] Value objects validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types inherit from `Exception`.
- [ ] Constants are pure static values.
- [ ] New taxonomy modules are registered in `__init__.py`.
- [ ] `python -c "import <module>"` passes.

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `python -c "import <module>"` or AST-based tooling.

```bash
# Find possible dataclasses in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from capabilities_\|from infrastructure_\|from agent_\|from surface_" modules/shared/src/*/taxonomy_*.py

# Check possible I/O in taxonomy files
grep -n "open(\|Path(\|os\.\|requests\.\|httpx\.\|sqlite3\.\|asyncpg\." modules/shared/src/*/taxonomy_*.py

# List registered taxonomy modules
grep -n "^from \.taxonomy_" modules/shared/src/*/mod.rs 2>/dev/null || grep -n "^from \.taxonomy_" modules/shared/src/*/__init__.py

# Find magic constants in layer files
grep -n "[0-9]\+\.[0-9]\+" modules/*/src/ --exclude-dir=shared | grep -v "#\|const\|import" | head -20
```

---

### Check Unregistered Taxonomy Files

```bash
for file in modules/shared/src/<domain>/taxonomy_*.py; do
  name=$(basename "$file" .py)
  grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py \
    || echo "UNREGISTERED: $name"
done
```

---

## Naming Convention

| Layer          | File Pattern                | Suffix                        |
| -------------- | --------------------------- | ----------------------------- |
| root           | `root_*_container.py`     | `_container`                |
| taxonomy       | `taxonomy_*_vo.py`        | `_vo`                       |
| taxonomy       | `taxonomy_*_entity.py`    | `_entity`                   |
| taxonomy       | `taxonomy_*_error.py`     | `_error`                    |
| taxonomy       | `taxonomy_*_event.py`     | `_event`                    |
| taxonomy       | `taxonomy_*_constant.py`  | `_constant`                 |
| taxonomy       | `taxonomy_*_utility.py`   | `_utility`                  |
| contract       | `contract_*_protocol.py`  | `_protocol`                 |
| contract       | `contract_*_port.py`      | `_port`                     |
| contract       | `contract_*_aggregate.py` | `_aggregate`                |
| capabilities   | `capabilities_*.py`       | flexible                      |
| infrastructure | `infrastructure_*.py`     | flexible                      |
| agent          | `agent_*.py`              | `_orchestrator`             |
| surface        | `surface_*.py`            | `_command`, `_controller` |

---

## Magic Constant Definitions

All domain constants MUST live in taxonomy constant files.

```python
# modules/shared/src/animator/taxonomy_animator_constant.py

# Default frames per second for animation.
FPS_DEFAULT: float = 24.0

# Minimum reveal time in seconds.
MIN_REVEAL_SECONDS: float = 0.5

# Manifest filename.
MANIFEST_FILENAME: str = "manifest.json"
```

Layer consumption:

```python
from shared.animator.taxonomy_animator_constant import FPS_DEFAULT
```

```python
from shared.animator.taxonomy_animator_constant import MIN_REVEAL_SECONDS
```

```python
from shared.animator.taxonomy_animator_constant import MANIFEST_FILENAME
```

If a constant represents a domain value, wrap it in a VO at the consuming boundary when exposing it through public domain contracts.

---

## Common Mistakes

- Defining dataclasses in layer files.
- Defining domain enums in layer files.
- Importing non-taxonomy layer types into taxonomy files.
- Importing contract ABCs into taxonomy files.
- Using wrong suffix for taxonomy files.
- Forgetting to register taxonomy modules in `__init__.py`.
- Putting domain knowledge into `*_utility.py`.
- Putting single-consumer helpers into `*_utility.py`.
- Keeping reusable domain-agnostic utilities inside layer files.
- Exposing public raw `str` fields in VOs.
- Exposing public numeric primitive fields in domain types.
- Creating VOs without validation when domain invariants exist.
- Duplicating taxonomy types across domains.
- Putting cross-domain types in a specific domain instead of `common/`.
- Creating taxonomy utility functions with I/O.
- Treating every stateless function as utility.
- Treating every concrete field as DI violation.
- Forgetting that value fields may be shared VOs, while service fields must use DI.

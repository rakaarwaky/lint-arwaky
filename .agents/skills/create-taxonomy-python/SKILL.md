---
name: create-taxonomy-python
description: "Create and validate taxonomy layer files (shared/taxonomy) — all data classes, VOs, errors, constants, and utilities must live here following strict naming conventions."
version: 1.1.0
category: refactoring
tags:
  [
    python,
    aes,
    taxonomy,
    shared,
    dataclass,
    vo,
    entity,
    utility,
    structure,
  ]
triggers:
  - "create taxonomy python"
  - "add taxonomy python"
  - "move to taxonomy python"
  - "dataclass in shared python"
  - "create value object python"
  - "create taxonomy entity python"
dependencies: []
related:
  - create-capabilities-python
  - create-infrastructure-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - method_classifier-python
---

# create-taxonomy-python

## Purpose

Create and validate Python **taxonomy layer** files in `modules/shared/src/<domain>/`. This is where ALL data classes, value objects, errors, constants, and stateless utility functions MUST live. No domain types may be defined in capabilities, infrastructure, agents, or surface layers.

## Rules

### The Fundamental Question

> **"Is this a dataclass?"**

- **Dataclass** (with domain data, DTOs, results, VOs) → **MUST be in shared/taxonomy**. Never in capabilities/infrastructure/agents/surface.
- **Class** (that inherits a protocol, uses DI) → belongs in the layer file (`capabilities_*.py`, `infrastructure_*.py`, `agent_*.py`).

### Taxonomy Layer Structure

```
modules/shared/src/
├── __init__.py                 # Top-level module declarations
├── common/                     # Cross-domain shared types
│   ├── __init__.py
│   └── taxonomy_*.py
├── <domain>/                   # Domain-specific taxonomy
│   ├── __init__.py             # Module exports for this domain
│   ├── contract_*.py           # Contract protocols (port, protocol, aggregate)
│   ├── taxonomy_*_vo.py        # Value Objects
│   ├── taxonomy_*_entity.py    # Entity types
│   ├── taxonomy_*_error.py     # Error types
│   └── taxonomy_*_utility.py   # Stateless utility functions
```

### File Naming Convention

Taxonomy files follow strict naming patterns:

| Suffix      | Purpose                              | Allowed? | Example                              |
| ----------- | ------------------------------------ | -------- | ------------------------------------ |
| `_vo`       | Value Objects (wraps a single value) | ✅ YES   | `taxonomy_import_rule_vo.py`         |
| `_entity`   | Domain entities with identity        | ✅ YES   | `taxonomy_analysis_entity.py`        |
| `_error`    | Error types (`Exception`)            | ✅ YES   | `taxonomy_config_error.py`           |
| `_event`    | Event/message types                  | ✅ YES   | `taxonomy_scan_event.py`             |
| `_constant` | Static compile-time constants        | ✅ YES   | `taxonomy_layer_names_constant.py`   |
| `_utility`  | Stateless functions                  | ✅ YES   | `taxonomy_symbol_renamer_utility.py` |

**CRITICAL:** These suffixes are **strict** — only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed for `taxonomy_` prefixed files. No other suffixes.

### Import Restrictions (AES201)

Taxonomy files must remain **completely pure**:

| Taxonomy Type                                          | Can Import From              | Cannot Import From                                              |
| ------------------------------------------------------ | ---------------------------- | --------------------------------------------------------------- |
| **taxonomy(vo)**                                       | Other taxonomy types         | agents, infrastructure, surfaces, contracts, capabilities, root |
| **taxonomy(entity), taxonomy(error), taxonomy(event)** | taxonomy VOs/constants       | agents, infrastructure, surfaces, contracts, capabilities       |
| **taxonomy(constant)**                                 | Nothing (pure static values) | Any external imports                                            |
| **taxonomy(utility)**                                  | taxonomy types               | Non-taxonomy layers                                             |

### Dataclass Patterns

#### Value Objects (`_vo.py`)

Wrap a single value with type safety:

```python
# taxonomy_import_rule_vo.py
from dataclasses import dataclass

@dataclass(frozen=True)
class ImportRuleVO:
    pattern: str
    message: str

    def value(self) -> str:
        return self.pattern
```

#### Macro-Generated Value Objects

For simple wrappers, use dataclass decorators:

```python
# taxonomy_common_vo.py
from dataclasses import dataclass

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

#### Error Types (`_error.py`)

Use Python exceptions:

```python
# taxonomy_config_error.py
class ConfigError(Exception):
    def __init__(self, key: str, message: str):
        self.key = key
        self.message = message
        super().__init__(f"Config error: {key} - {message}")
```

#### Utility Functions (`_utility.py`)

Stateless functions (no `self`, no side effects) that act as **Dumb Tools**.

**🚨 CRITICAL: The Ultimate Boundary for Utilities**
A function belongs in `*_utility.py` ONLY if it meets ALL of these:

1. **Stateless**: No `self`, no `cls`, no class field access.
2. **Pure Function**: Input A always produces output B. No side effects (no I/O).
3. **Domain-Agnostic / Reusable**: It does NOT know about specific business rules or domain-specific validation logic. It is a blind data manipulator (e.g., regex matching, string normalization, AST parsing).
4. **Multi-Consumer Reusable**: Function serves multiple capabilities/infrastructures (could be same domain or cross-domain), not just one class.

If a stateless function contains **Domain Knowledge** OR only serves **ONE capability/infrastructure class**, it MUST stay in the capabilities layer as a **Private Helper**, NOT extracted to taxonomy utility.

```python
# ✅ GOOD: Dumb Tool (Domain-Agnostic, Multi-Consumer Reusable)
def extract_trait_name(content: str) -> str | None:
    # Just regex, doesn't know what a "trait" means in domain context
    # Multiple capabilities/infrastructures can use this
    # ...
    pass

# ❌ BAD: Domain Knowledge masquerading as utility
def get_target_layer_from_suffix(suffix: str) -> str:
    # KNOWS business rules: port = infrastructure.
    # This belongs in capabilities as a private helper!
    if suffix == "port":
        return "infrastructure"
    elif suffix == "protocol":
        return "capabilities"
    else:
        return "unknown"

# ❌ BAD: Single Consumer Only
def format_import_violation(rule: ImportRule) -> str:
    # Only used by one capability class, not reusable by others
    # This belongs in capabilities as a private helper!
    return f"Import rule violation: {rule.pattern}"
```

## Detection Patterns

### BAD: Dataclass Defined in Layer File

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

### BAD: Dataclass Defined in Infrastructure

```python
# BAD: Domain data defined in infrastructure layer
@dataclass
class CacheEntry:  # ← DATA CLASS — should be in shared/taxonomy
    key: str
    value: str
    timestamp: int
```

### GOOD: Dataclass in Taxonomy + Class with DI

```python
# GOOD: Dataclass in taxonomy
# modules/shared/src/orphan-detector/taxonomy_analysis_vo.py
@dataclass(frozen=True)
class OrphanIndicatorResult:
    is_orphan: bool
    reason: str
    severity: str

# GOOD: Class imports from taxonomy
# modules/orphan-detector/src/capabilities_orphan_analyzer.py
from shared.orphan_detector.taxonomy_analysis import OrphanIndicatorResult
from contract_orphan_protocol import IOrphanFilenameExtractorProtocol

class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor  # ← DI
```

## Workflow

### Step 1: Identify the Dataclass

When you find a dataclass in a layer file (capabilities/infrastructure/agent/surface), ask: **"Is this a dataclass or a class?"**

- If it contains domain data, DTOs, results, or value wrappers → **dataclass → move to taxonomy**
- If it inherits a protocol and uses DI → **class → stays in layer file**

### Step 2: Determine Taxonomy Domain

Find the correct domain directory under `modules/shared/src/<domain>/`:

| Domain            | Directory                     | Example Types                              |
| ----------------- | ----------------------------- | ------------------------------------------ |
| `common`          | `shared/src/common/`          | Cross-domain types (PathVO, BooleanVO)     |
| `orphan-detector` | `shared/src/orphan-detector/` | Orphan results, severity, violations       |
| `code-analysis`   | `shared/src/code-analysis/`   | Analysis results, reachability, violations |
| `import-rules`    | `shared/src/import-rules/`    | Import rules, violations, language types   |
| `naming-rules`    | `shared/src/naming-rules/`    | Naming violations, patterns                |

### Step 3: Create or Update Taxonomy File

**Option A: New taxonomy domain** — Create `<domain>/` directory with `__init__.py`, then add taxonomy files.

**Option B: Existing domain** — Add new file to existing domain directory.

**Naming:** Use the correct suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).

```bash
# Example: Create orphan result dataclass in taxonomy
mkdir -p modules/shared/src/orphan-detector/
# Create taxonomy_orphan_vo.py
```

### Step 4: Register Module

Update the domain's `__init__.py` to export the new taxonomy module:

```python
# shared/src/orphan-detector/__init__.py
from .taxonomy_orphan_vo import OrphanResult  # ← Add this line
from .taxonomy_analysis import OrphanIndicatorResult
```

### Step 5: Update Imports in Layer Files

Replace local dataclass definitions with imports from taxonomy:

```python
# BEFORE (BAD): Local dataclass
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str

# AFTER (GOOD): Import from taxonomy
from shared.orphan_detector.taxonomy_orphan_vo import OrphanResult
```

### Step 6: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] **All dataclasses in shared/taxonomy** — no dataclasses/Enums defined in layer files.
- [ ] **Taxonomy file naming follows strict suffixes** — `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`.
- [ ] **Taxonomy files import only from taxonomy** — no imports from capabilities, infrastructure, agents, contracts, or surface.
- [ ] **Utility functions in `*_utility.py` are purely domain-agnostic AND serve MULTIPLE capabilities/infrastructures** — functions containing business rules OR serving only ONE class stay in capabilities as private helpers.
- [ ] **Layer files import dataclasses from taxonomy** — not defined locally.
- [ ] **Domain's `__init__.py` exports new taxonomy modules** — `from .taxonomy_<name> import ...`.
- [ ] **Value Objects use `@dataclass(frozen=True)`** — immutable by default.
- [ ] **Error types inherit from `Exception`** — with proper error messages.
- [ ] **Constants are pure static values** — no imports, no functions.
- [ ] **Contract signatures use VOs, not primitives** — ALL primitives are FORBIDDEN in contract method signatures:
  - `str` → use domain-specific VO (e.g., `FilePath`, `SymbolName`)
  - `int` → use domain-specific VO (e.g., `LineNumber`, `Count`)
  - `bool` → use `BooleanVO`
  - `float` → use domain-specific VO (e.g., `Score`)
  - `list[str]` → use domain-specific list VO (e.g., `PatternList`)
  - `dict` → use domain-specific VO
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Find dataclasses defined in layer files (not in shared/taxonomy)
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | grep -v "__init__"

# Check for forbidden imports in taxonomy files
grep -n "from capabilities_\|from infrastructure_\|from agent_" modules/shared/src/*/taxonomy_*.py

# Find layer files with concrete type fields (non-protocol) that might need taxonomy dataclasses
grep -n "__init__" modules/*/src/ | grep -v "Protocol\|Port\|Aggregate" | grep -v "shared/"

# Verify taxonomy module exports are registered
grep -n "^from \.taxonomy_" modules/shared/src/*/ __init__.py

# Check for unregistered taxonomy files (exist on disk but not in __init__.py)
find modules/shared/src/<domain>/ -name "taxonomy_*.py" | while read f; do
    name=$(basename "$f" .py)
    grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py || echo "UNREGISTERED: $name"
done

# Check for dataclasses in layer files that should be moved to taxonomy
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP '(?<=@dataclass\n|^class )\K[a-zA-Z_]+')
    echo "POSSIBLE DATACLASS: $file has $class"
done
```

## Naming Convention (from fix-naming)

**All Layer File Naming:**

| Layer                    | Pattern                    | Suffix                                     |
| ------------------------ | -------------------------- | ------------------------------------------ |
| **root**           | `root_*_container.py`    | `_container`                             |
| **taxonomy**       | `taxonomy_*_vo.py`       | `_vo`, `_constant`, `_utility`, etc. |
| **contract**       | `contract_*_protocol.py` | `_protocol`, `_port`, `_aggregate`   |
| **capabilities**   | `capabilities_*.py`      | flexible                                   |
| **infrastructure** | `infrastructure_*.py`    | flexible                                   |
| **agent**          | `agent_*.py`             | `_orchestrator`                          |
| **surface**        | `surface_*.py`           | `_command`, `_controller`              |

## Primitive-to-VO Patterns (from fix-primitive-to-vo)

**Taxonomy Layer VO Creation Rules:**

- Entity fields MUST use VOs, not primitives (`str`, `int`, `float`, `bool`).
- **Contract signatures MUST use VOs** — ALL primitives are FORBIDDEN in contract method signatures. The VOs created here are the mandatory replacements:
  - `str` → use domain-specific VO (e.g., `FilePath`, `SymbolName`)
  - `int` → use domain-specific VO (e.g., `LineNumber`, `Count`, `Score`)
  - `bool` → use `BooleanVO`
  - `float` → use domain-specific VO (e.g., `Score`)
  - `list[str]` → use domain-specific list VO (e.g., `PatternList`)
  - `dict` → use domain-specific VO
- VOs MUST validate on construction.

```python
# BEFORE (primitive in layer file)
@dataclass
class LintResult:
    file_path: str   # ← primitive
    line: int        # ← primitive
    severity: str    # ← primitive

# AFTER (VO in taxonomy)
# modules/shared/src/import-rules/taxonomy_file_path_vo.py
@dataclass(frozen=True)
class FilePath:
    value: str

# modules/shared/src/import-rules/taxonomy_line_number_vo.py
@dataclass(frozen=True)
class LineNumber:
    value: int

# modules/shared/src/import-rules/taxonomy_severity_vo.py
@dataclass(frozen=True)
class SeverityVO:
    value: str

@dataclass
class LintResult:
    file_path: FilePath   # ← VO
    line: LineNumber      # ← VO
    severity: SeverityVO  # ← VO
```

## Magic Constant Definitions (from fix-magic-constant)

**Taxonomy Layer Constant Rules:**

- All domain values live in `taxonomy_*_constant.py` files.
- Constants are static compile-time values — no functions, no imports.
- Used by agent, capabilities, and infrastructure layers.

```python
# modules/shared/src/animator/taxonomy_animator_constant.py
"""Default frames per second for animation"""
FPS_DEFAULT = 24.0

"""Minimum reveal time in seconds"""
MIN_REVEAL_SECONDS = 0.5

"""Manifest filename constant"""
MANIFEST_FILENAME = "manifest.json"
```

**Layer consumption:**

```python
# Agent layer
from shared.animator.taxonomy_animator_constant import FPS_DEFAULT
result = self.process(FPS_DEFAULT)

# Capabilities layer
from shared.animator.taxonomy_animator_constant import MIN_REVEAL_SECONDS
def calculate_duration(self) -> float:
    return MIN_REVEAL_SECONDS

# Infrastructure layer
from shared.animator.taxonomy_animator_constant import MANIFEST_FILENAME
file = open(MANIFEST_FILENAME, "w")
```

## Common Mistakes (AVOID)

- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Importing non-taxonomy types into taxonomy files**: Taxonomy must remain completely pure — no imports from capabilities, infrastructure, agents, contracts, or surface.
- ❌ **Using wrong suffix for taxonomy files**: Only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed. No other suffixes.
- ❌ **Forgetting to register new taxonomy modules in `__init__.py`**: Every `taxonomy_*.py` file must have a corresponding `from .taxonomy_<name> import ...` in the domain's `__init__.py`.
- ❌ **Placing Domain Knowledge in Utility files**: If a stateless function contains business-specific rules or domain logic (e.g., layer mappings, validation rules tied to a specific domain), it belongs in capabilities as a private helper, NOT in `*_utility.py`.
- ❌ **Placing Single-Consumer functions in Utility files**: If a function only serves ONE capability/infrastructure class, it belongs in capabilities as a private helper, NOT in `*_utility.py`.
- ❌ **Placing utility functions in layer files**: Stateless, domain-agnostic free functions (no `self`, no `cls`) that serve MULTIPLE capabilities/infrastructures MUST be extracted to `*_utility.py` modules in shared/taxonomy.
- ❌ **Creating multiple dataclasses with different names for the same concept**: Consolidate into a single taxonomy file.
- ❌ **Duplicating taxonomy types across domains**: If a type belongs to multiple domains, put it in `common/` and import from there.

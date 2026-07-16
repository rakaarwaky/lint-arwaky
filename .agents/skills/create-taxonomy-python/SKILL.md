---
name: create-taxonomy-python
description: "Create and validate taxonomy layer files (shared/taxonomy) — all data classes, VOs, errors, and utilities must live here following strict naming conventions."
version: 1.0.0
category: refactoring
tags: [python, aes, taxonomy, shared, dataclass, vo, entity, utility, structure]
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

Stateless functions (no `self`, no side effects):

```python
# taxonomy_symbol_renamer_utility.py

def format_bytes(bytes: int) -> str:
    """Stateless formatting utility — no self needed"""
    return f"{bytes / 1024:.1f}KB"

def clamp(value: float, min_val: float, max_val: float) -> float:
    """Stateless math utility — no self needed"""
    return max(min_val, min(value, max_val))
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
- [ ] **Utility functions in `*_utility.py`** — standalone functions (no `self`) extracted to modules.
- [ ] **Layer files import dataclasses from taxonomy** — not defined locally.
- [ ] **Domain's `__init__.py` exports new taxonomy modules** — `from .taxonomy_<name> import ...`.
- [ ] **Value Objects use `@dataclass(frozen=True)`** — immutable by default.
- [ ] **Error types inherit from `Exception`** — with proper error messages.
- [ ] **Constants are pure static values** — no imports, no functions.
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

## Common Mistakes (AVOID)

- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Importing non-taxonomy types into taxonomy files**: Taxonomy must remain completely pure — no imports from capabilities, infrastructure, agents, contracts, or surface.
- ❌ **Using wrong suffix for taxonomy files**: Only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed. No other suffixes.
- ❌ **Forgetting to register new taxonomy modules in **init**.py**: Every `taxonomy_*.py` file must have a corresponding `from .taxonomy_<name> import ...` in the domain's `__init__.py`.
- ❌ **Placing utility functions in layer files**: Standalone functions (no `self`) MUST be extracted to `*_utility.py` modules in shared/taxonomy.
- ❌ **Creating multiple dataclasses with different names for the same concept**: Consolidate into a single taxonomy file.
- ❌ **Duplicating taxonomy types across domains**: If a type belongs to multiple domains, put it in `common/` and import from there.

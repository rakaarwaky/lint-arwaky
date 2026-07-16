---
name: clean-bloat
version: 3.1.0
category: validation
tags:
  [
    aes,
    cleanup,
    bloat,
    stubs,
    thin-wrappers,
    mvp,
    boilerplate,
    facade,
    re-export,
  ]
triggers:
  - "clean bloat"
  - "remove stubs"
  - "remove thin wrappers"
  - "clean capabilities"
  - "mvp cleanup"
  - "check boilerplate facades"
  - "validate boilerplate"
  - "find facade boilerplate"
  - "clean re-export"
  - "remove re-export hub"
dependencies: []
related:
  - check-module-logic
---

# clean-bloat

## Rules

- Never remove real logic
- Always update protocol (remove abstract methods)
- Always update **init** (remove exports)
- Always run lint after changes

## Purpose

Scan capability files for code that is NOT relevant to the FRD MVP scope.

## When to Use

- After refactoring capability modules
- Before committing capability changes
- When user asks to clean bloat from a module

## The Fundamental Question

Before keeping any function, ask:

> **"Why does this function need to exist?"**

If the answer is:

- "Because it was always there" -> **REMOVE**
- "Because it might be useful someday" -> **REMOVE**
- "Because it handles edge cases we don't have" -> **REMOVE**
- "Because it's required by FRD MVP" -> **KEEP**
- "Because it's called by a method that's required by FRD MVP" -> **KEEP**

## Detection Patterns

### Thin Wrappers (Remove)

```python
# Simple attribute return
def get_something(self, obj) -> float:
    return obj.attribute
# WHY: Direct attribute access is simpler. No logic added.

# isinstance check
def is_something(obj: object) -> bool:
    return isinstance(obj, SomeClass)
# WHY: isinstance() is already simple. Wrapper adds no value.

# Simple enum comparison
def should_force_X(hint: ActionHint) -> bool:
    return hint == ActionHint.X
# WHY: Comparison is already simple. Wrapper adds no value.

# Constant return
def get_constant() -> float:
    return SOME_CONSTANT
# WHY: Constant is already accessible. Wrapper adds no value.
```

### Stubs (Remove)

```python
def method() -> None: ...
def method() -> None: pass
def method() -> object: return None
def method() -> str: return ""
# WHY: Empty implementations provide no value.
```

### Duplicate Methods (Remove)

Same method in multiple capability files -- keep in the file that owns the logic.
WHY: Duplicates create maintenance burden. Single source of truth.

### Redundant Aliases (Remove)

```python
method_name = ClassName.method_name  # REMOVE
# WHY: Direct import is clearer. Aliases add indirection.
```

### Overengineered Patterns (Remove)

```python
# Temporal enforcer, circular dependency detection, etc.
# if NOT in FRD MVP -> REMOVE
# WHY: Complexity without clear MVP requirement is waste.
```

### Unimplemented Contract Abstractions (Remove)

```python
# Protocols with no concrete implementations
# WHY: Empty abstractions add no value.
```

### Re-Export Hubs (Remove/Slim Down)

Massive files that ONLY re-export types from taxonomy or shared modules — no business logic.

```python
# BAD: 500+ line file that just re-exports taxonomy types
# contract_rendering_protocol.py
from src.core._01_taxonomy._01_value_objects._00_primitives.file_path import FilePath
from src.core._01_taxonomy._01_value_objects._00_primitives.identifiers import WorldId
from src.core._01_taxonomy._01_value_objects._02_visual import BlendMode
# ... 70 more imports ...
class RenderingProtocol:  # Empty class, no logic
    pass
# WHY: This is NOT a contract. Contract = abstract interface (Port/Protocol/Aggregate).
#      This is a re-export hub. Types should be imported directly by consumers.
```

**Detection pattern:**

- File has 50+ import statements but 0-1 class definitions
- File name contains `protocol` but has no abstract methods
- File re-exports types that consumers could import directly

**Fix:**

1. Delete the re-export hub file
2. Update all consumers to import directly from taxonomy/shared module
3. If a proper contract is needed, define it with abstract methods in shared module

### Standalone Re-Export Files (Remove)

Single files that only re-export from contract layer without adding logic.

```python
# BAD: capabilities_layer_service.py — 10 lines, just re-exports
from modules.shared.src.compositor.contract_layer_service import LayerRetrievalService
__all__ = ["LayerRetrievalService"]
# WHY: Consumers should import directly from contract layer.
#      This file adds zero value — just indirection.
```

**Fix:**

1. Delete the re-export file
2. Update consumers to import directly from contract/shared module
3. Update `__init__.py` if it re-exports from this file

## Workflow

### Step 1: Read FRD

Read the FRD to understand MVP scope.

### Step 2: List Files

List all capability/protocol files.

### Step 3: Analyze Each File

For each file, ask "Why does each function need to exist?"

### Step 4: Mark for Removal

If answer is not "required by FRD MVP" -> mark for removal.

### Step 5: Report

Report per file -- show what to keep/remove.

### Step 6: Get Approval

Get approval per file.

### Step 7: Execute Cleanup

Remove bloat, update protocol, update **init**.

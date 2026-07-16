---
name: enforce-1-class-per-file
version: 1.0.0
category: refactoring
tags: [aes, class, structure, single-responsibility]
triggers:
  - "enforce 1 class per file"
  - "merge classes"
  - "one class per file"
dependencies: []
related:
  - fix-class-wrapping
  - fix-capability-structure
---

# enforce-1-class-per-file

## Rules

- 1 file = 1 class
- Data classes, enums, constants → move to shared taxonomy
- Helper classes → merge into main class as methods
- Protocol classes stay in shared contract

## Purpose

Ensure each capability/infrastructure/agent file contains exactly ONE class.

## When to Use

- File has multiple classes
- Data class or enum defined locally in infrastructure
- Helper class that supports the main class

## The Fundamental Question

> **"Does this file have more than 1 class?"**

If yes → **Merge into 1 class or move to taxonomy**

## Detection Pattern

```python
# BAD: 2 classes in 1 file
class CoordinateTransforms:  # → MOVE to CoordinateMapper as static methods
    ...
class CoordinateMapper:
    ...

# GOOD: 1 class per file
class CoordinateMapper:
    @staticmethod
    def world_to_camera(...):  # was in CoordinateTransforms
        ...
```

## Types to Move to Taxonomy

| Type               | Move To                  |
| ------------------ | ------------------------ |
| `@dataclass`       | `taxonomy_*_vo.py`       |
| `class *Enum`      | `taxonomy_*_vo.py`       |
| `class *Constants` | `taxonomy_*_constant.py` |
| `class *Info`      | `taxonomy_*_vo.py`       |

## Types to Merge into Main Class

| Type          | Merge Into                    |
| ------------- | ----------------------------- |
| Helper class  | Main class as `@staticmethod` |
| Factory class | Main class methods            |
| Builder class | Main class methods            |

## Workflow

### Step 1: Count Classes

Find files with >1 class.

### Step 2: Classify Each Class

- Data/enum → move to taxonomy
- Helper → merge into main class
- Protocol → move to shared contract

### Step 3: Execute

Move or merge each class.

### Step 4: Update Imports

Fix all references.

### Step 5: Verify

Run syntax check.

---
name: fix-magic-constant
version: 1.0.0
category: refactoring
tags: [aes, magic, constant, aes405]
triggers:
  - "fix magic constant"
  - "replace hardcoded value"
dependencies: []
related:
  - fix-agent-di
---

# fix-magic-constant

## Rules

- NO hardcoded literals in ANY layer
- All domain values MUST be named constants
- Constants MUST live in taxonomy_*_constant.py

## Purpose

Remove hardcoded literals from ALL layers (agent, capabilities, infrastructure) and replace with named constants.

## When to Use

- Agent file has hardcoded literals
- Capabilities file has hardcoded literals
- Infrastructure file has hardcoded literals
- Magic numbers or strings in business logic

## The Fundamental Question

> **"Is there a hardcoded literal?"**

If yes -> **Replace with named constant**

## Workflow

### Step 1: Find Magic Constants

Read code and find hardcoded literals.

### Step 2: Create/Find Constant

Create or find named constant in taxonomy.

### Step 3: Replace

Replace magic with named constant.

## Layer-Specific Examples

### Agent

```python
# [FORBIDDEN] BEFORE
result = self.process(fps=24)

# [OK] AFTER
from modules.shared.src.animator.taxonomy_animator_constant import FPS_DEFAULT
result = self.process(fps=FPS_DEFAULT)
```

### Capabilities

```python
# [FORBIDDEN] BEFORE
def calculate_duration(self) -> float:
    return 0.5  # magic

# [OK] AFTER
from modules.shared.src.animator.taxonomy_animator_constant import MIN_REVEAL_SECONDS
def calculate_duration(self) -> float:
    return MIN_REVEAL_SECONDS
```

### Infrastructure

```python
# [FORBIDDEN] BEFORE
def save(self):
    with open("manifest.json", "w") as f:  # magic path
        ...

# [OK] AFTER
from modules.shared.src.animator.taxonomy_animator_constant import MANIFEST_FILENAME
def save(self):
    with open(MANIFEST_FILENAME, "w") as f:
        ...
```

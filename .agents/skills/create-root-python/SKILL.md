---
name: create-root-python
description: "Create and validate Python root layer files: composition root that wires Capabilities to Contract protocols/aggregates and bootstraps the application. Container connects implementations, Entry starts the system."
metadata:
    tags: [python, aes, root, container, entry, composition, di, wiring]
    triggers:
        - "create root python"
        - "add root python"
        - "create container python"
        - "create entry python"
        - "wire dependencies python"
        - "check root python"
        - "audit root python"
    dependencies: []
    related:
        - create-capabilities-python
        - create-agent-python
        - create-contract-python
        - create-taxonomy-python
---

# create-root-python

## Purpose

Create and validate Python **root layer** files — the composition layer that assembles the system.

Root connects concrete implementations to contracts and starts the application. It may depend on all layers.

Two root roles exist:

- `container` → Wires one feature by connecting Capabilities to Contract protocols and aggregates
- `entry` → Bootstraps the application and composes feature containers

## Definition of Done

1. Root file uses correct suffix: `_container` or `_entry`.
2. Container wires Capabilities to Contract protocols/aggregates.
3. Entry bootstraps the application and composes feature containers.
4. Root may instantiate and wire components.
5. Root must not contain business logic.
6. Root must not contain orchestration policy.
7. Root must not contain technical parsing or user interface behavior.
8. `python -c "import <module>"` passes.

## Workflow

### Step 1: Determine Root Role

Ask: **"What does this file do?"**

| Role | Suffix | Responsibility |
|------|--------|----------------|
| Container | `_container` | Wire one feature's Capabilities to Contracts |
| Entry | `_entry` | Bootstrap application, compose feature containers |

### Step 2: Create Root File

Create `root_<concept>_<suffix>.py` in the appropriate location.

### Step 3: Wire Dependencies

Connect Capabilities implementations to their Contract protocols/aggregates.

### Step 4: Verify

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# Find root files
grep -rn "root_\|container\|entry" modules/*/src/

# Check for business logic in root
grep -n "def calculate\|def validate\|def analyze" modules/*/src/root_*.py
```

## Common Mistakes

- Putting business logic in root files.
- Putting orchestration policy in root files.
- Root containing UI behavior.
- Forgetting to wire Capabilities to Contracts.
- Creating circular dependencies between containers.

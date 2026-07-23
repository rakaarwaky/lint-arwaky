---
name: create-taxonomy-python
description: "Create and validate Python taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
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
    - create-agent-python
    - create-contract-python
---

# create-taxonomy-python

## Purpose

Create and validate Python **taxonomy layer** files inside `modules/shared/src/<domain>/`.

Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns.

Taxonomy is the single source of truth for:

- value objects, entities, domain errors, domain events,
- constants (compile-time literal values).

No domain data structures may be defined in capabilities, agent, surface, or root layers.

## Definition of Done

1. Domain data structures live in `shared/taxonomy`.
2. Taxonomy file naming uses allowed strict suffixes.
3. Taxonomy files do not import from capability, agent, surface, or root layers.
4. Taxonomy files contain no I/O and no side effects.
5. Value objects validate on construction.
6. Public domain contracts use VOs instead of raw primitives.
7. New taxonomy modules are registered in `__init__.py`.
8. `python -c "import <module>"` passes.

## References

| File                               | Content                                                    |
| ---------------------------------- | ---------------------------------------------------------- |
| `references/purity-imports.md`     | AES201 import restrictions, allowed/forbidden dependencies |
| `references/dataclass-patterns.md` | VOs, entities, errors, events, constants patterns          |
| `references/primitive-vo-rules.md` | Primitive policy table, VO construction rules              |
| `references/examples.md`           | All BAD/GOOD code examples                                 |
| `references/commands.md`           | Quick heuristic check commands                             |
| `references/checklist.md`          | Verification checklist                                     |

## Templates

| File                                  | Purpose               |
| ------------------------------------- | --------------------- |
| `templates/taxonomy_name_vo.py`       | New value object file |
| `templates/taxonomy_name_error.py`    | New error type file   |
| `templates/taxonomy_name_constant.py` | New constants file    |

## Workflow

### Step 1: Identify the Dataclass

When you find a class in a layer file, ask: **"Is this a dataclass or an implementor?"**

If it carries domain data → move to taxonomy. If it inherits a protocol and uses DI → keep in layer file.

### Step 2: Determine Taxonomy Domain

Choose the correct domain directory under `modules/shared/src/<domain>/`.

### Step 3: Create or Update Taxonomy File

Use the correct suffix: `_vo`, `_entity`, `_error`, `_event`, `_constant`.

### Step 4: Register Module

Update the domain `__init__.py`.

### Step 5: Update Imports in Layer Files

Replace local definitions with imports from taxonomy.

### Step 6: Verify Purity

No imports from layers, no I/O in taxonomy files.

### Step 7: Verify Primitive-to-VO Compliance

No public raw `str` domain fields, VOs validate on construction.

### Step 8: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# Find possible dataclasses in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from.*capabilities_|from.*agent_|from.*surface_" modules/shared/src/*/taxonomy_*.py

# Check possible I/O in taxonomy files
grep -n "open(\|Path(\|os\." modules/shared/src/*/taxonomy_*.py
```

## Common Mistakes

- Defining dataclasses in layer files.
- Importing non-taxonomy layer types into taxonomy files.
- Importing contract ABCs into taxonomy files.
- Using wrong suffix for taxonomy files.
- Forgetting to register taxonomy modules in `__init__.py`.
- Exposing public raw `str` fields in VOs.
- Creating VOs without validation when domain invariants exist.
- Duplicating taxonomy types across domains.

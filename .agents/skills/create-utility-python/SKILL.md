---
name: create-utility-python
description: "Create and validate Python utility layer files following AES rules: stateless standalone functions, no class, no protocol impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags:
    [
      python,
      aes,
      utility,
      shared,
      stateless,
      pure-function,
      domain-agnostic,
    ]
  triggers:
    - "create utility python"
    - "add utility python"
    - "extract utility python"
    - "create helper function python"
    - "check utility python"
    - "audit utility python"
  dependencies: []
  related:
    - create-taxonomy-python
    - create-capabilities-python
    - create-agent-python
---

# create-utility-python

## Purpose

Create and validate Python **utility layer** files inside `modules/shared/src/<domain>/`.

A utility file contains **stateless standalone functions**. It exists so that Capabilities, Agents, and Surfaces can remain clean and expressive by delegating low-level technical mechanics to reusable helpers.

A utility file must:

- contain ONLY module-level functions (no class, no `self` parameter),
- be completely stateless (no instance state, no class attributes),
- be pure (input A always produces output B),
- be domain-agnostic (no business rules, no domain knowledge),
- be reusable across multiple modules.

## Role Naming (ARCHITECTURE §7)

Utility role suffixes describe the technical responsibility:

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

File: `utility_<domain>_<role>.py`

## Dependencies (ARCHITECTURE §7)

- **May depend on:** Taxonomy only.
- **Must NOT depend on / import:** Capabilities, Agent, Surface, Contract, other Utility (except shared taxonomy utilities).

## Special Rules (ARCHITECTURE §7)

- **Stateless Only:** no class definitions, no `self`, no instance state.
- **Pure Functions:** input A always produces output B. No randomness, no global state mutation, no I/O side effects (unless domain-agnostic + reusable).
- **No Business Decisions:** utility does not know business rules, domain constraints, or architecture policies.
- **No Protocol Implementation:** utility never implements an ABC protocol or aggregate.
- **I/O Allowed:** stateless + I/O + domain-agnostic + reusable = valid utility (e.g., `walk_source_files`, `read_file_content`).
- **Standalone Functions Only:** no classes, no methods. Just `def` declarations at module level.
- **No Magic Constants:** extract reusable constants into `taxonomy_<domain>_constant.py` in shared.

## Definition of Done

1. NO class definition — only module-level functions.
2. All functions are stateless (no `self`, no instance state).
3. Functions are pure: input A always produces output B.
4. Functions are domain-agnostic: no business rules, no architecture knowledge.
5. Functions are reusable across multiple modules.
6. No magic constants — use shared taxonomy constants.
7. Only depends on Taxonomy layer.
8. `python -c "import <module>"` passes.

## References

Read these files for detailed rules:

| File                             | Content                                         |
| -------------------------------- | ----------------------------------------------- |
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies      |
| `references/stateless-rules.md`  | Stateless, pure, domain-agnostic decision rules |
| `references/examples.md`         | All BAD/GOOD code examples                      |
| `references/commands.md`         | Quick heuristic check commands                  |

## Templates

Use these templates when creating new files:

| File                        | Purpose                         |
| --------------------------- | ------------------------------- |
| `templates/utility_name.py` | New utility implementation file |

## Workflow

### Step 1: Analyze Code Responsibility

Read the code and ask: **"Is this a stateless, pure, domain-agnostic function?"**

If yes → extract to utility. If no → check if it's business logic (→ capabilities), orchestration (→ agent), or domain data (→ taxonomy).

### Step 2: Check Reusability

Is the function used by multiple modules? Or will it be useful in the future?

- **Single-use + domain-specific** → keep as private helper in the layer file
- **Reusable + domain-agnostic** → extract to utility

### Step 3: Verify Stateless Purity

Does the function have ANY of these?

- `self` parameter
- Access to class attributes
- Random number generation
- System clock access
- Global state mutation
- Business rule knowledge

If YES → NOT a utility. Keep as private helper.

### Step 4: Verify Domain Agnosticism

Does the function know about:

- Architecture layer names?
- Business domain rules?
- Specific capability logic?

If YES → NOT a utility. Domain-specific code belongs in capabilities.

### Step 5: Create Utility File

Write the module-level functions following the template. No classes.

### Step 6: Update Module Registration

Add import to the appropriate shared domain `__init__.py`.

### Step 7: Verify Compilation

```bash
python -c "import modules.shared.src.<domain>.utility_<name>"
```

## Quick Commands

```bash
# Check for forbidden patterns (class, self)
grep -rn "^class \|def.*self" modules/shared/src/<domain>/utility_*.py

# List all utility functions
grep -rn "^def " modules/shared/src/<domain>/utility_*.py

# Check imports in utilities (should only use taxonomy)
grep -rn "^from\|^import" modules/shared/src/<domain>/utility_*.py
```

## Common Mistakes

- Adding class definitions to utility files.
- Implementing protocol ABCs in utility files.
- Using `self` or accessing class attributes.
- Including business logic or domain rules.
- Using magic constants instead of shared taxonomy constants.
- Importing Capabilities, Agent, or Surface modules.
- Creating functions that are only used by one module (keep as private helper).
- Mixing pure functions with stateful operations.
- Adding I/O to domain-specific functions (must be domain-agnostic + reusable).

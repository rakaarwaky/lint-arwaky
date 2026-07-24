---
name: create-surface-python
description: "Create and validate Python surface layer files following AES406: smart/utility/passive surfaces, strict import rules, delegate to aggregates, zero direct lower-layer imports, zero business logic, VO-based state, and explicit error handling."
metadata:
  tags:
    [
      python,
      aes,
      surface,
      command,
      controller,
      component,
      hook,
      store,
      action,
      view,
      aes406,
      vo,
    ]
  triggers:
    - "create surface python"
    - "add surface python"
    - "fix surface python"
    - "create command python"
    - "create component python"
    - "create hook python"
    - "create store python"
    - "surface role violation python"
    - "audit surface python"
  dependencies: []
  related:
    - create-capabilities-python
    - create-agent-python
    - create-contract-python
    - create-taxonomy-python
---

# create-surface-python

## Purpose

Create and validate Python **surface layer** files in feature modules.

The surface layer is the outermost boundary of the application.

It is responsible for:

- receiving user input,
- mapping input events to shared action/event VOs,
- delegating execution to aggregates,
- rendering/displaying state from shared VOs.

The surface layer MUST NOT:

- import capabilities directly,
- import concrete agent classes directly,
- contain business logic,
- contain domain computation,
- perform I/O directly.

## AES406 — Surface Role Rules

### Surface Classification
- **Smart** (`_command`, `_controller`, `_page`, `_entry`): may contain orchestration logic
- **Utility** (`_hook`, `_store`, `_action`, `_screen`, `_router`): supports smart surfaces
- **Passive** (all other surface suffixes): presentation-only

### Global Check (All Surfaces)
- **Max 15 `def` definitions per file.** Exceeding → violation.

### Passive + Utility Checks
- **Max 10 public methods per class.** Exceeding → violation.
- **Max 80 lines per method body.** Exceeding → violation.
- **Max 3 levels of if-nesting depth.** Exceeding → violation.
- **Max 3 control-flow statements** (`if`, `elif`, `else`, `for`, `while`, `try`, `except`). Exceeding → domain logic violation.

Smart surfaces (`_command`, `_controller`, `_page`, `_entry`) are exempted from passive checks but still subject to the 15-function global limit.

## Definition of Done

1. Surface file uses a valid suffix.
2. Surface role is clear: smart, utility, or passive.
3. Smart surface imports only taxonomy and aggregate contracts.
4. Utility surface imports only taxonomy and passive surfaces.
5. Passive surface imports only taxonomy.
6. No surface file imports capabilities or concrete agents.
7. Smart surface delegates to aggregates via `I<Name>Aggregate`.
8. Utility surface does not import concrete smart surfaces.
9. Passive surface contains only rendering/display logic.
10. Surface state fields use shared VOs.
11. Service dependencies use protocol interfaces via DI.
12. Errors are handled explicitly.
13. `python -c "import <module>"` passes.
14. **AES406:** File has max 15 `def` definitions.
15. **AES406 (passive/utility):** Each class has max 10 public methods.
16. **AES406 (passive/utility):** Method bodies max 80 lines.
17. **AES406 (passive/utility):** Nesting depth max 3 levels.
18. **AES406 (passive/utility):** Max 3 control-flow statements.

## References

| File                                | Content                                         |
| ----------------------------------- | ----------------------------------------------- |
| `references/layer-boundaries.md`    | Allowed/Forbidden imports for each surface type |
| `references/surface-types.md`       | Smart/Utility/Passive surface definitions       |
| `references/helper-vs-utility.md`   | Helper vs utility decision                      |
| `references/error-handling.md`      | Error handling rules                            |
| `references/primitive-vo-policy.md` | Primitive policy table                          |
| `references/examples.md`            | All BAD/GOOD code examples                      |
| `references/commands.md`            | Quick heuristic check commands                  |
| `references/checklist.md`           | 20-item verification checklist                  |

## Templates

| File                        | Purpose                         |
| --------------------------- | ------------------------------- |
| `templates/surface_name.py` | New surface implementation file |

## Workflow

### Step 1: Determine Surface Type

Ask: **"What role does this surface serve?"**

| Role                               | Suffixes                                     |
| ---------------------------------- | -------------------------------------------- |
| Entry point / command / controller | `_command`, `_controller`, `_page`, `_entry` |
| Event/action/store/screen adapter  | `_hook`, `_store`, `_action`, `_screen`      |
| Rendering component/view/layout    | `_component`, `_view`, `_layout`             |

### Step 2: Check Import Rules

Verify imports follow the correct pattern for the surface type.

### Step 3: Create Surface File

Create `surface_<concept>_<suffix>.py` in the appropriate feature module.

### Step 4: Verify Role Compliance

No capabilities imports, no concrete agent imports, no business logic, no domain computation, no I/O.

### Step 5: Verify DI and VO Usage

Service fields use protocol interfaces, state fields use shared VOs.

### Step 6: Verify Error Handling

No silent error swallowing.

### Step 7: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# Check forbidden lower-layer imports
grep -n "^\s*from.*capabilities_\|from.*agent_\|from.*surface_" modules/*/src/surface_*.py

# Check syntax
python -c "import <module>"

# AES406: Count def definitions per file (global limit: 15)
grep -c "^\s*def \|^\s*async def " modules/*/src/surface_*.py

# AES406: Count public methods per class (limit: 10)
grep -n "^\s*def [a-z]" modules/*/src/surface_*.py

# AES406: Check for deep nesting (limit: 3 levels)
grep -n "^\s{12,}if \|^\s{12,}for \|^\s{12,}while " modules/*/src/surface_*.py

# AES406: Count control-flow in passive/utility surfaces (limit: 3)
grep -c "if \|elif \|else:\|for \|while \|try:\|except " modules/*/src/surface_*_component.py modules/*/src/surface_*_view.py modules/*/src/surface_*_layout.py modules/*/src/surface_*_hook.py modules/*/src/surface_*_store.py modules/*/src/surface_*_action.py modules/*/src/surface_*_screen.py
```

## Common Mistakes

- Importing capabilities directly in surface files.
- Importing concrete agent classes in surface files.
- Smart surface calling capabilities directly.
- Utility surface importing concrete smart surface.
- Passive surface containing business logic.
- Defining domain data classes in surface files.
- Using concrete service types as smart surface fields.
- Silently discarding errors.

---
name: create-surface-python
description: "Create and validate surface layer files (surface_*.py) — entry points, controllers, hooks, stores, and passive UI components following AES406 role rules."
version: 1.0.0
category: refactoring
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
    entry,
    structure,
    aes406,
  ]
triggers:
  - "create surface python"
  - "add surface python"
  - "fix surface python"
  - "create command python"
  - "create component python"
  - "create hook python"
  - "surface role violation python"
dependencies: []
related:
  - create-capabilities-python
  - create-infrastructure-python
  - create-agent-python
  - create-contract-python
  - create-taxonomy-python
  - enforce-1-class-per-file-python
  - module_logic_validator-python
---

# create-surface-python

## Purpose

Create and validate Python **surface layer** files in feature modules. The surface layer is the outermost boundary — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly. Three surface types serve different roles with strict import rules.

**This skill consolidates rules from:** `module_logic_validator`, `enforce-1-class-per-file`, `method_classifier`, and AES406 surface role checks — applied specifically to the surface layer.

## Rules

### The Fundamental Question

> **"What type of surface is this?"**

- **Smart Surface** (`_command`, `_controller`, `_page`, `_entry`) — Entry points; maps events, orchestrates via aggregates
- **Utility Surface** (`_hook`, `_store`, `_action`, `_screen`) — Thin wrappers around smart surfaces; passive logic
- **Passive Surface** (`_component`, `_view`, `_layout`) — Pure rendering/display; no logic or orchestration

### Three Surface Types

| Type | Suffixes | Can Import From | Forbidden | Description |
|------|----------|-----------------|-----------|-------------|
| **Smart Surface** | `_command`, `_controller`, `_page`, `_entry` | `taxonomy_*` + `contract_aggregate_*` only | capabilities, infrastructure, agents, smart surfaces | Entry points; owns DI container, orchestrates via aggregates |
| **Utility Surface** | `_hook`, `_store`, `_action`, `_screen` | `taxonomy_*` + passive surfaces only | smart surfaces, capabilities, infrastructure, agents | Thin wrappers; maps events/actions to smart surfaces |
| **Passive Surface** | `_component`, `_view`, `_layout` | `taxonomy_*` only | everything else (no logic, no orchestration) | Pure rendering/display; zero business logic |

### Import Restrictions (AES406)

Surface layer follows strict role-based import rules:

```python
# Smart Surface — CAN import taxonomy + contract_aggregate
from shared.common.taxonomy_path import FilePath
from shared.cli_commands.contract_import_runner import IImportRunnerAggregate  # ✅ ALLOWED

# Smart Surface — CANNOT import capabilities/infrastructure directly
from capabilities_my_checker import MyChecker  # ❌ FORBIDDEN (AES406)
from infrastructure_adapter import FileAdapter  # ❌ FORBIDDEN (AES406)

# Utility Surface — CAN import taxonomy + passive surfaces only
from shared.tui.surface_shortcut_component import ShortcutComponent  # ✅ ALLOWED (passive)
from surface_smart_command import SmartCommand  # ❌ FORBIDDEN (AES406 - smart surface)

# Passive Surface — CAN import taxonomy only
from shared.common.taxonomy_common import AppState  # ✅ ALLOWED
```

### Data Flow Pattern

```
User presses "c" (check) in TUI
 ↓
surface_tui_command.py (Smart Surface) — maps key to TuiEvent.ActionCheck
 ↓
agent_tui_orchestrator.py — receives event, delegates to lint executor
 ↓
capabilities_lint_executor.py — runs check logic, calls code analysis
 ↓
contract_code_analysis_port.py — interface for code analysis
 ↓
infrastructure_code_analysis_adapter.py — actual file scanning
```

The surface layer is the **outermost boundary** — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly.

### Structural Rules (All Layers)

- **1 file = 1 class** for smart surfaces that hold state
- **All data classes in shared/taxonomy** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`) should be extracted to `*_utility.py` modules in shared/taxonomy

## Detection Patterns

### BAD: Smart Surface Imports Capabilities Directly

```python
# BAD: Smart surface imports capabilities directly
from capabilities_my_checker import MyChecker  # ← FORBIDDEN (AES406)

class CheckCommand:
    def __init__(self):
        self._checker = MyChecker()  # ← Should use IMyProtocol via contract_aggregate
```

### BAD: Passive Surface Contains Business Logic

```python
# BAD: Passive surface contains business logic
class MyComponent:
    def render(self) -> str:
        # ← BUSINESS LOGIC — passive surfaces should only render
        output = f"Result: {self.result}"
        return output
```

### BAD: Utility Surface Imports Smart Surface

```python
# BAD: Utility surface imports smart surface (AES406)
from surface_check_command import CheckCommand  # ← FORBIDDEN

class MyAction:
    def __init__(self):
        self._command = CheckCommand()  # ← Should only depend on passive surfaces or taxonomy
```

### GOOD: Smart Surface Uses Aggregates Only

```python
# GOOD: Smart surface imports only taxonomy + contract aggregates
from shared.cli_commands.contract_import_runner import IImportRunnerAggregate
from shared.common.taxonomy_path import FilePath

class CheckCommand:
    def __init__(self, runner: IImportRunnerAggregate):
        self._runner = runner  # ← DI via contract

    def scan(self) -> None:
        # Delegates to aggregate — never imports capabilities/infrastructure directly
        self._runner.run_check()
```

### GOOD: Passive Surface is Pure Rendering

```python
# GOOD: Passive surface only renders, no logic
from shared.tui.taxonomy_tui_vo import AppState

class StatusComponent:
    def __init__(self, state: AppState):
        self._state = state

    def render(self) -> str:
        # Pure rendering — no business logic, no computation
        return f"Status: {self._state.status}"
```

## Workflow

### Step 1: Determine Surface Type

Ask: **"What role does this surface serve?"**

- Entry point / CLI command / TUI entry → **Smart Surface** (`_command`, `_controller`, `_page`, `_entry`)
- Event handler / store / action / screen → **Utility Surface** (`_hook`, `_store`, `_action`, `_screen`)
- UI component / view / layout → **Passive Surface** (`_component`, `_view`, `_layout`)

### Step 2: Check Import Rules

Verify imports follow the correct pattern for the surface type:

```bash
# Check smart surfaces for forbidden imports
grep -n "capabilities_\|infrastructure_\|agent_" modules/*/src/surface_*_command.py

# Check passive surfaces for business logic
grep -n "\.len(\|\.map(\|\.sum(\|if.*contains" modules/*/src/surface_*_component.py

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" modules/*/src/surface_*_action.py
```

### Step 3: Create Surface File

Create `surface_<concept>_<suffix>.py` in the appropriate feature module.

**Smart Surface rules:**
- Can import `taxonomy_*` + `contract_aggregate_*` only
- Owns DI container via aggregate interfaces (`I<Name>Aggregate`)
- Orchestrates via aggregates — never imports capabilities/infrastructure directly

```python
# surface_check_command.py (Smart Surface)
from shared.cli_commands.contract_import_runner import IImportRunnerAggregate
from shared.common.taxonomy_path import FilePath

class CheckCommand:
    def __init__(self, runner: IImportRunnerAggregate):
        self._runner = runner

    def scan(self) -> None:
        self._runner.run_check()  # ← Delegates to aggregate
```

**Passive Surface rules:**
- Can import `taxonomy_*` only
- Pure rendering/display — zero business logic, zero computation, zero orchestration

```python
# surface_status_component.py (Passive Surface)
from shared.tui.taxonomy_tui_vo import AppState

class StatusComponent:
    def __init__(self, state: AppState):
        self._state = state

    def render(self) -> str:
        return f"Status: {self._state.status}"  # ← Pure rendering only
```

### Step 4: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for capabilities/infrastructure imports in surface files
grep -rn "capabilities_\|infrastructure_\|agent_" modules/*/src/surface_*.py

# Check passive surfaces for business logic
grep -n "\.len(\|\.map(\|\.sum(\|if.*contains" modules/*/src/surface_*_component.py modules/*/src/surface_*_view.py

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" modules/*/src/surface_*_action.py modules/*/src/surface_*_hook.py
```

### Step 5: Verify

Run syntax check to confirm no violations.

## Verification Checklist

- [ ] Surface file uses correct suffix (`_command`, `_controller`, `_page`, `_entry`, `_hook`, `_store`, `_action`, `_screen`, `_component`, `_view`, `_layout`).
- [ ] **Smart Surface** imports only `taxonomy_*` + `contract_aggregate_*` — no capabilities, infrastructure, agents.
- [ ] **Utility Surface** imports only `taxonomy_*` + passive surfaces — no smart surfaces, capabilities, infrastructure.
- [ ] **Passive Surface** imports only `taxonomy_*` — zero business logic, zero computation, zero orchestration.
- [ ] **Zero direct imports** of capabilities, infrastructure, or agents in any surface file.
- [ ] Smart surfaces delegate to aggregates via `I<Name>Aggregate` — never call capabilities/infrastructure directly.
- [ ] Passive surfaces contain only rendering/display logic — no computation, no data transformation.
- [ ] Utility surfaces are thin wrappers — no business logic, no orchestration.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Check smart surfaces for forbidden imports (AES406)
grep -n "capabilities_\|infrastructure_\|agent_" modules/*/src/surface_*_command.py modules/*/src/surface_*_controller.py

# Check passive surfaces for business logic (AES406)
grep -n "\.len(\|\.map(\|\.sum(\|if.*contains\|for.*in" modules/*/src/surface_*_component.py modules/*/src/surface_*_view.py

# Check utility surfaces for smart surface imports (AES406)
grep -n "surface_.*_command\|surface_.*_controller" modules/*/src/surface_*_action.py modules/*/src/surface_*_hook.py

# Find surface files that import non-taxonomy/non-contract types
find modules/*/src/ -name "surface_*.py" | while read f; do
    grep -n "^from capabilities_\|^from infrastructure_\|^from agent_" "$f" || true
done

# Check for dataclasses defined in surface files (should be in taxonomy)
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/surface_*.py | grep -v "shared/" | grep -v "__init__"

# Check for concrete type fields (non-protocol) in smart surfaces
grep -n "__init__" modules/*/src/surface_*_command.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-PROTOCOL FIELD: $file"
done

# Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Importing capabilities/infrastructure directly in surface files**: Smart surfaces must use `contract_aggregate_*` interfaces via `I<Name>Aggregate`. Never import capabilities or infrastructure directly.
- ❌ **Putting business logic in passive surfaces**: Passive surfaces (`_component`, `_view`, `_layout`) must contain only rendering/display logic — zero computation, zero data transformation.
- ❌ **Utility surfaces importing smart surfaces**: Utility surfaces (`_hook`, `_store`, `_action`, `_screen`) can only import `taxonomy_*` + passive surfaces. Importing smart surfaces violates AES406.
- ❌ **Defining dataclasses in surface files**: Domain data must be in shared/taxonomy. Only the class belongs in surface files.
- ❌ **Using concrete types as constructor fields in smart surfaces**: Smart surface fields should always receive `I<Name>Aggregate` (DI via aggregates), not concrete implementations.
- ❌ **Orchestrating directly from smart surfaces**: Smart surfaces delegate to agents via aggregates — they don't call capabilities or infrastructure directly.
- ❌ **Duplicating surface definitions across features**: If a surface belongs to multiple features, put it in a shared location and import from there.

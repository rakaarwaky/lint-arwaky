---
name: create-surface-typescript
description: "Create and validate surface layer files (surface_*.ts) — entry points, controllers, hooks, stores, and passive UI components following AES406 role rules."
version: 1.0.0
category: refactoring
tags:
  [
    typescript,
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
  - "create surface typescript"
  - "add surface typescript"
  - "fix surface typescript"
  - "create command typescript"
  - "create component typescript"
  - "create hook typescript"
  - "surface role violation typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - create-agent-typescript
  - create-contract-typescript
  - create-taxonomy-typescript
  - enforce-1-class-per-file-typescript
  - module_logic_validator-typescript
---

# create-surface-typescript

## Purpose

Create and validate TypeScript **surface layer** files in feature packages. The surface layer is the outermost boundary — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly. Three surface types serve different roles with strict import rules.

## Rules

### The Fundamental Question

> **"What type of surface is this?"**

- **Smart Surface** (`_command`, `_controller`, `_page`, `_entry`) — Entry points; maps events, orchestrates via aggregates
- **Utility Surface** (`_hook`, `_store`, `_action`, `_screen`) — Thin wrappers around smart surfaces; passive logic
- **Passive Surface** (`_component`, `_view`, `_layout`) — Pure rendering/display; no logic or orchestration

### Three Surface Types

| Type                | Suffixes                                     | Can Import From                            | Forbidden                                            | Description                                                  |
| ------------------- | -------------------------------------------- | ------------------------------------------ | ---------------------------------------------------- | ------------------------------------------------------------ |
| **Smart Surface**   | `_command`, `_controller`, `_page`, `_entry` | `taxonomy_*` + `contract_aggregate_*` only | capabilities, infrastructure, agents, smart surfaces | Entry points; owns DI container, orchestrates via aggregates |
| **Utility Surface** | `_hook`, `_store`, `_action`, `_screen`      | `taxonomy_*` + passive surfaces only       | smart surfaces, capabilities, infrastructure, agents | Thin wrappers; maps events/actions to smart surfaces         |
| **Passive Surface** | `_component`, `_view`, `_layout`             | `taxonomy_*` only                          | everything else (no logic, no orchestration)         | Pure rendering/display; zero business logic                  |

### Import Restrictions (AES406)

Surface layer follows strict role-based import rules:

```typescript
// Smart Surface — CAN import taxonomy + contract_aggregate
import { FilePath } from '../shared/common/taxonomy_path';
import { IImportRunnerAggregate } from '../shared/cli_commands/contract_import_runner';  // ✅ ALLOWED

// Smart Surface — CANNOT import capabilities/infrastructure directly
import { MyChecker } from '../capabilities/my_checker';  // ❌ FORBIDDEN (AES406)
import { FileAdapter } from '../infrastructure/adapter';  // ❌ FORBIDDEN (AES406)

// Utility Surface — CAN import taxonomy + passive surfaces only
import { ShortcutComponent } from '../tui/surface_shortcut_component';  // ✅ ALLOWED (passive)
import { SmartCommand } from './surface_smart_command';  // ❌ FORBIDDEN (AES406 - smart surface)

// Passive Surface — CAN import taxonomy only
import { AppState } from '../shared/common/taxonomy_common';  // ✅ ALLOWED
```

### Data Flow Pattern

```
User presses "c" (check) in TUI
 ↓
surface_tui_command.ts (Smart Surface) — maps key to TuiEvent.ActionCheck
 ↓
agent_tui_orchestrator.ts — receives event, delegates to lint executor
 ↓
capabilities_lint_executor.ts — runs check logic, calls code analysis
 ↓
contract_code_analysis_port.ts — interface for code analysis
 ↓
infrastructure_code_analysis_adapter.ts — actual file scanning
```

The surface layer is the **outermost boundary** — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly.

### Structural Rules (All Layers)

- **1 file = 1 class** for smart surfaces that hold state
- **All data types in shared/taxonomy** — no interfaces/types may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions should be extracted to `*_utility.ts` modules in shared/taxonomy

## Detection Patterns

### BAD: Smart Surface Imports Capabilities Directly

```typescript
// BAD: Smart surface imports capabilities directly
import { MyChecker } from '../capabilities/my_checker';  // ← FORBIDDEN (AES406)

class CheckCommand {
    constructor() {
        this._checker = new MyChecker();  // ← Should use IMyProtocol via contract_aggregate
    }
}
```

### BAD: Passive Surface Contains Business Logic

```typescript
// BAD: Passive surface contains business logic
class MyComponent {
    render(): string {
        // ← BUSINESS LOGIC — passive surfaces should only render
        const output = `Result: ${this.result}`;
        return output;
    }
}
```

### BAD: Utility Surface Imports Smart Surface

```typescript
// BAD: Utility surface imports smart surface (AES406)
import { CheckCommand } from './surface_check_command';  // ← FORBIDDEN

class MyAction {
    constructor() {
        this._command = new CheckCommand();  // ← Should only depend on passive surfaces or taxonomy
    }
}
```

### GOOD: Smart Surface Uses Aggregates Only

```typescript
// GOOD: Smart surface imports only taxonomy + contract aggregates
import { IImportRunnerAggregate } from '../shared/cli_commands/contract_import_runner';
import { FilePath } from '../shared/common/taxonomy_path';

class CheckCommand {
    constructor(private runner: IImportRunnerAggregate) {}  // ← DI via contract

    scan(): void {
        // Delegates to aggregate — never imports capabilities/infrastructure directly
        this.runner.runCheck();
    }
}
```

### GOOD: Passive Surface is Pure Rendering

```typescript
// GOOD: Passive surface only renders, no logic
import { AppState } from '../shared/tui/taxonomy_tui_vo';

class StatusComponent {
    constructor(private state: AppState) {}

    render(): string {
        // Pure rendering — no business logic, no computation
        return `Status: ${this.state.status}`;
    }
}
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
grep -n "capabilities_\|infrastructure_\|agent_" packages/*/src/surface_*_command.ts

# Check passive surfaces for business logic
grep -n "\.length\|\.map\|\.reduce\|\.includes" packages/*/src/surface_*_component.ts

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" packages/*/src/surface_*_action.ts
```

### Step 3: Create Surface File

Create `surface_<concept>_<suffix>.ts` in the appropriate feature package.

**Smart Surface rules:**

- Can import `taxonomy_*` + `contract_aggregate_*` only
- Owns DI container via aggregate interfaces (`I<Name>Aggregate`)
- Orchestrates via aggregates — never imports capabilities/infrastructure directly

```typescript
// surface_check_command.ts (Smart Surface)
import { IImportRunnerAggregate } from '../shared/cli_commands/contract_import_runner';
import { FilePath } from '../shared/common/taxonomy_path';

class CheckCommand {
    constructor(private runner: IImportRunnerAggregate) {}

    scan(): void {
        this.runner.runCheck();  // ← Delegates to aggregate
    }
}
```

**Passive Surface rules:**

- Can import `taxonomy_*` only
- Pure rendering/display — zero business logic, zero computation, zero orchestration

```typescript
// surface_status_component.ts (Passive Surface)
import { AppState } from '../shared/tui/taxonomy_tui_vo';

class StatusComponent {
    constructor(private state: AppState) {}

    render(): string {
        return `Status: ${this.state.status}`;  // ← Pure rendering only
    }
}
```

### Step 4: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for capabilities/infrastructure imports in surface files
grep -rn "capabilities_\|infrastructure_\|agent_" packages/*/src/surface_*.ts

# Check passive surfaces for business logic
grep -n "\.length\|\.map\|\.reduce\|\.includes\|for.*of" packages/*/src/surface_*_component.ts packages/*/src/surface_*_view.ts

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" packages/*/src/surface_*_action.ts packages/*/src/surface_*_hook.ts
```

### Step 5: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] Surface file uses correct suffix (`_command`, `_controller`, `_page`, `_entry`, `_hook`, `_store`, `_action`, `_screen`, `_component`, `_view`, `_layout`).
- [ ] **Smart Surface** imports only `taxonomy_*` + `contract_aggregate_*` — no capabilities, infrastructure, agents.
- [ ] **Utility Surface** imports only `taxonomy_*` + passive surfaces — no smart surfaces, capabilities, infrastructure.
- [ ] **Passive Surface** imports only `taxonomy_*` — zero business logic, zero computation, zero orchestration.
- [ ] **Zero direct imports** of capabilities, infrastructure, or agents in any surface file.
- [ ] Smart surfaces delegate to aggregates via `I<Name>Aggregate` — never call capabilities/infrastructure directly.
- [ ] Passive surfaces contain only rendering/display logic — no computation, no data transformation.
- [ ] Utility surfaces are thin wrappers — no business logic, no orchestration.
- [ ] All interfaces imported from shared/taxonomy (none defined locally).
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Check smart surfaces for forbidden imports (AES406)
grep -n "capabilities_\|infrastructure_\|agent_" packages/*/src/surface_*_command.ts packages/*/src/surface_*_controller.ts

# Check passive surfaces for business logic (AES406)
grep -n "\.length\|\.map\|\.reduce\|\.includes\|for.*of" packages/*/src/surface_*_component.ts packages/*/src/surface_*_view.ts

# Check utility surfaces for smart surface imports (AES406)
grep -n "surface_.*_command\|surface_.*_controller" packages/*/src/surface_*_action.ts packages/*/src/surface_*_hook.ts

# Find surface files that import non-taxonomy/non-contract types
find packages/*/src/ -name "surface_*.ts" | while read f; do
    grep -n "^import.*capabilities_\|^import.*infrastructure_\|^import.*agent_" "$f" || true
done

# Check for interfaces defined in surface files (should be in taxonomy)
grep -rn "^interface\|^type " packages/*/src/surface_*.ts | grep -v "shared/" | grep -v "index.ts"

# Check for concrete type fields (non-interface) in smart surfaces
grep -n "constructor" packages/*/src/surface_*_command.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Importing capabilities/infrastructure directly in surface files**: Smart surfaces must use `contract_aggregate_*` interfaces via `I<Name>Aggregate`. Never import capabilities or infrastructure directly.
- ❌ **Putting business logic in passive surfaces**: Passive surfaces (`_component`, `_view`, `_layout`) must contain only rendering/display logic — zero computation, zero data transformation.
- ❌ **Utility surfaces importing smart surfaces**: Utility surfaces (`_hook`, `_store`, `_action`, `_screen`) can only import `taxonomy_*` + passive surfaces. Importing smart surfaces violates AES406.
- ❌ **Defining interfaces in surface files**: Domain data must be in shared/taxonomy. Only the class belongs in surface files.
- ❌ **Using concrete types as constructor fields in smart surfaces**: Smart surface fields should always receive `I<Name>Aggregate` (DI via aggregates), not concrete implementations.
- ❌ **Orchestrating directly from smart surfaces**: Smart surfaces delegate to agents via aggregates — they don't call capabilities or infrastructure directly.
- ❌ **Duplicating surface definitions across features**: If a surface belongs to multiple features, put it in a shared location and import from there.

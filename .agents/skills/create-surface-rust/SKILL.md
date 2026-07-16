---
name: create-surface-rust
description: "Create and validate surface layer files (surface_*.rs) — entry points, controllers, hooks, stores, and passive UI components following AES406 role rules."
version: 1.0.0
category: refactoring
tags:
  [
    rust,
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
  - "create surface rust"
  - "add surface rust"
  - "fix surface rust"
  - "create command rust"
  - "create component rust"
  - "create hook rust"
  - "surface role violation rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-agent-rust
  - create-contract-rust
  - create-taxonomy-rust
  - enforce-1-struct-per-file-rust
  - module_logic_validator-rust
---

# create-surface-rust

## Purpose

Create and validate Rust **surface layer** files in feature crates. The surface layer is the outermost boundary — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly. Three surface types serve different roles with strict import rules.

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

```rust
// Smart Surface — CAN import taxonomy + contract_aggregate
use shared::common::taxonomy_path_vo::FilePath;
use shared::cli_commands::contract_import_runner_aggregate::IImportRunnerAggregate;  // ✅ ALLOWED

// Smart Surface — CANNOT import capabilities/infrastructure directly
use crate::capabilities_my_checker::MyChecker;  // ❌ FORBIDDEN (AES406)
use crate::infrastructure_adapter::FileAdapter;  // ❌ FORBIDDEN (AES406)

// Utility Surface — CAN import taxonomy + passive surfaces only
use shared::tui::surface_shortcut_component::ShortcutComponent;  // ✅ ALLOWED (passive)
use crate::surface_smart_command::SmartCommand;  // ❌ FORBIDDEN (AES406 - smart surface)

// Passive Surface — CAN import taxonomy only
use shared::common::taxonomy_common_vo::AppState;  // ✅ ALLOWED
```

### Data Flow Pattern

```
User presses "c" (check) in TUI
 ↓
surface_tui_command.rs (Smart Surface) — maps key to TuiEvent::ActionCheck
 ↓
agent_tui_orchestrator.rs — receives event, delegates to lint executor
 ↓
capabilities_lint_executor.rs — runs check logic, calls code analysis
 ↓
contract_code_analysis_port.rs — interface for code analysis
 ↓
infrastructure_code_analysis_adapter.rs — actual file scanning
```

The surface layer is the **outermost boundary** — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly.

### Structural Rules (All Layers)

- **1 file = 1 impl struct** for smart surfaces that hold state
- **All data classes in shared/taxonomy** — no structs/enums with data may be defined outside shared/taxonomy
- **Fields must use DI** — impl struct fields should be `Arc<dyn Trait>` objects, not concrete types
- **Helper functions stay in layer** — helper methods that support the struct remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic free functions (no `&self`) should be extracted to `*_utility.rs` modules in shared/taxonomy

## Detection Patterns

### BAD: Smart Surface Imports Capabilities Directly

```rust
// BAD: Smart surface imports capabilities directly
use crate::capabilities_my_checker::MyChecker;  // ← FORBIDDEN (AES406)

pub struct CheckCommand {
    checker: MyChecker,  // ← Should use Arc<dyn IMyProtocol> via contract_aggregate
}
```

### BAD: Passive Surface Contains Business Logic

```rust
// BAD: Passive surface contains business logic
pub struct MyComponent {
    result: String,
}

impl MyComponent {
    pub fn render(&self) -> String {
        // ← BUSINESS LOGIC — passive surfaces should only render
        let output = format!("Result: {}", self.result);
        return output;
    }
}
```

### BAD: Utility Surface Imports Smart Surface

```rust
// BAD: Utility surface imports smart surface (AES406)
use crate::surface_check_command::CheckCommand;  // ← FORBIDDEN

pub struct MyAction {
    command: CheckCommand,  // ← Should only depend on passive surfaces or taxonomy
}
```

### GOOD: Smart Surface Uses Aggregates Only

```rust
// GOOD: Smart surface imports only taxonomy + contract aggregates
use shared::cli_commands::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::common::taxonomy_path_vo::FilePath;

pub struct CheckCommand {
    // Owns DI container via aggregate interfaces
    runner: Arc<dyn IImportRunnerAggregate>,  // ← DI via contract
}

impl CheckCommand {
    pub fn scan(&self) {
        // Delegates to aggregate — never imports capabilities/infrastructure directly
        self.runner.run_check().unwrap();
    }
}
```

### GOOD: Passive Surface is Pure Rendering

```rust
// GOOD: Passive surface only renders, no logic
use shared::tui::taxonomy_tui_vo::AppState;

pub struct StatusComponent {
    state: AppState,
}

impl StatusComponent {
    pub fn render(&self) -> String {
        // Pure rendering — no business logic, no computation
        format!("Status: {}", self.state.status)
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
grep -n "capabilities_\|infrastructure_\|agent_" crates/*/src/surface_*_command.rs

# Check passive surfaces for business logic
grep -n "\.len()\|\.map(\|\.sum()\|if.*contains" crates/*/src/surface_*_component.rs

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" crates/*/src/surface_*_action.rs
```

### Step 3: Create Surface File

Create `surface_<concept>_<suffix>.rs` in the appropriate feature crate.

**Smart Surface rules:**

- Can import `taxonomy_*` + `contract_aggregate_*` only
- Owns DI container via aggregate interfaces (`Arc<dyn IAggregate>`)
- Orchestrates via aggregates — never imports capabilities/infrastructure directly

```rust
// surface_check_command.rs (Smart Surface)
use shared::cli_commands::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::common::taxonomy_path_vo::FilePath;

pub struct CheckCommand {
    runner: Arc<dyn IImportRunnerAggregate>,
}

impl CheckCommand {
    pub fn new(runner: Arc<dyn IImportRunnerAggregate>) -> Self {
        Self { runner }
    }

    pub fn scan(&self) {
        self.runner.run_check().unwrap();  // ← Delegates to aggregate
    }
}
```

**Passive Surface rules:**

- Can import `taxonomy_*` only
- Pure rendering/display — zero business logic, zero computation, zero orchestration

```rust
// surface_status_component.rs (Passive Surface)
use shared::tui::taxonomy_tui_vo::AppState;

pub struct StatusComponent {
    state: AppState,
}

impl StatusComponent {
    pub fn render(&self) -> String {
        format!("Status: {}", self.state.status)  // ← Pure rendering only
    }
}
```

### Step 4: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for capabilities/infrastructure imports in surface files
grep -rn "capabilities_\|infrastructure_\|agent_" crates/*/src/surface_*.rs

# Check passive surfaces for business logic
grep -n "\.len()\|\.map(\|\.sum()\|if.*contains" crates/*/src/surface_*_component.rs

# Check utility surfaces for smart surface imports
grep -n "surface_.*_command\|surface_.*_controller" crates/*/src/surface_*_action.rs
```

### Step 5: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] Surface file uses correct suffix (`_command`, `_controller`, `_page`, `_entry`, `_hook`, `_store`, `_action`, `_screen`, `_component`, `_view`, `_layout`).
- [ ] **Smart Surface** imports only `taxonomy_*` + `contract_aggregate_*` — no capabilities, infrastructure, agents.
- [ ] **Utility Surface** imports only `taxonomy_*` + passive surfaces — no smart surfaces, capabilities, infrastructure.
- [ ] **Passive Surface** imports only `taxonomy_*` — zero business logic, zero computation, zero orchestration.
- [ ] **Zero direct imports** of capabilities, infrastructure, or agents in any surface file.
- [ ] Smart surfaces delegate to aggregates via `Arc<dyn IAggregate>` — never call capabilities/infrastructure directly.
- [ ] Passive surfaces contain only rendering/display logic — no computation, no data transformation.
- [ ] Utility surfaces are thin wrappers — no business logic, no orchestration.
- [ ] All data classes imported from shared/taxonomy (none defined locally).
- [ ] `cargo check -p <crate-name>` passes without warnings or errors.

## Quick Commands

```bash
# Check smart surfaces for forbidden imports (AES406)
grep -n "capabilities_\|infrastructure_\|agent_" crates/*/src/surface_*_command.rs crates/*/src/surface_*_controller.rs

# Check passive surfaces for business logic (AES406)
grep -n "\.len()\|\.map(\|\.sum()\|if.*contains\|for.*in" crates/*/src/surface_*_component.rs crates/*/src/surface_*_view.rs

# Check utility surfaces for smart surface imports (AES406)
grep -n "surface_.*_command\|surface_.*_controller" crates/*/src/surface_*_action.rs crates/*/src/surface_*_hook.rs

# Find surface files that import non-taxonomy/non-contract types
find crates/*/src/ -name "surface_*.rs" | while read f; do
    grep -n "^use crate::capabilities_\|^use crate::infrastructure_\|^use crate::agent_" "$f" || true
done

# Check for dataclasses defined in surface files (should be in taxonomy)
grep -rn "^pub struct" crates/*/src/surface_*.rs | grep -v "shared/" | grep -v "impl\|trait\|fn "

# Check for concrete type fields (non-DI) in smart surfaces
grep -rn "^\s*[a-z_]*:" crates/*/src/surface_*_command.rs | grep -v "Arc<dyn"

# Find surface files that import non-taxonomy/non-contract types
find crates/*/src/ -name "surface_*.rs" | while read f; do
    grep -n "^use crate::capabilities_\|^use crate::infrastructure_\|^use crate::agent_" "$f" || true
done
```

## Surface Role Types (from fix-naming)

**All Layer File Naming:**

| Layer              | Pattern                  | Suffix                             |
| ------------------ | ------------------------ | ---------------------------------- |
| **root**           | `root_*_container.rs`    | `_container`                       |
| **taxonomy**       | `taxonomy_*_vo.rs`       | `_vo`, `_constant`                 |
| **contract**       | `contract_*_protocol.rs` | `_protocol`, `_port`, `_aggregate` |
| **capabilities**   | `capabilities_*.rs`      | flexible                           |
| **infrastructure** | `infrastructure_*.rs`    | flexible                           |
| **agent**          | `agent_*.rs`             | `_orchestrator`                    |
| **surface**        | `surface_*.rs`           | `_command`, `_controller`          |

## AES406 Violations (from module_logic_validator)

**Surface Layer Import Rules:**

```
Smart Surface:  CAN import taxonomy_* + contract_aggregate_* only
Utility Surface: CAN import taxonomy_* + passive surfaces only
Passive Surface: CAN import taxonomy_* only
```

### Forbidden Import Patterns (AES406)

```rust
// Agent → Surface [FORBIDDEN]
use crate::surface_cli_command::*

// Capabilities → Surface [FORBIDDEN]
use crate::surface_tui_screen::*

// Infrastructure → Surface [FORBIDDEN]
use crate::surface_status_component::*

// Surface → Capabilities [FORBIDDEN]
use crate::capabilities_lint_executor::*

// Surface → Infrastructure [FORBIDDEN]
use crate::infrastructure_file_adapter::*

// Surface → Agent [FORBIDDEN]
use crate::agent_orchestrator::*
```

## Data Flow Pattern (from module_logic_validator)

**Surface Layer Data Flow:**

```
User presses "c" (check) in TUI
 ↓
surface_tui_command.rs (Smart Surface) — maps key to TuiEvent::ActionCheck
 ↓
agent_tui_orchestrator.rs — receives event, delegates to lint executor
 ↓
capabilities_lint_executor.rs — runs check logic, calls code analysis
 ↓
contract_code_analysis_port.rs — interface for code analysis
 ↓
infrastructure_code_analysis_adapter.rs — actual file scanning
```

The surface layer is the **outermost boundary** — it receives user input, maps events, and delegates to aggregates. It never imports capabilities or infrastructure directly.

## Import Strategy (from fix-imports)

**Surface Layer Import Rules:**

```
ALLOWED:    contract_* (aggregate traits only), taxonomy_*
FORBIDDEN:  capabilities_*, infrastructure_*, agent_* — NEVER import concrete structs
```

### Surface Pattern (DI via Aggregate)

```rust
// WRONG:
// surface_cli_command.rs
use crate::capabilities_lint_executor::LintExecutor;  // FORBIDDEN

// CORRECT:
// surface_cli_command.rs
use crate::contract_lint_aggregate::ILintAggregate;

pub struct CliCommand {
    lint: Arc<dyn ILintAggregate>,
}

impl CliCommand {
    pub fn new(lint: Arc<dyn ILintAggregate>) -> Self {
        Self { lint }
    }
}
```

## Common Mistakes (AVOID)

- ❌ **Importing capabilities/infrastructure directly in surface files**: Smart surfaces must use `contract_aggregate_*` interfaces via `Arc<dyn IAggregate>`. Never import capabilities or infrastructure directly.
- ❌ **Putting business logic in passive surfaces**: Passive surfaces (`_component`, `_view`, `_layout`) must contain only rendering/display logic — zero computation, zero data transformation.
- ❌ **Utility surfaces importing smart surfaces**: Utility surfaces (`_hook`, `_store`, `_action`, `_screen`) can only import `taxonomy_*` + passive surfaces. Importing smart surfaces violates AES406.
- ❌ **Defining dataclasses in surface files**: Domain data must be in shared/taxonomy. Only the impl struct belongs in surface files.
- ❌ **Using concrete types as fields in smart surfaces**: Smart surface fields should always be `Arc<dyn Trait>` (DI via aggregates), never concrete implementations.
- ❌ **Orchestrating directly from smart surfaces**: Smart surfaces delegate to agents via aggregates — they don't call capabilities or infrastructure directly.
- ❌ **Duplicating surface definitions across features**: If a surface belongs to multiple features, put it in a shared location and import from there.

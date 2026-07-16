---
name: module_logic_validator-rust
version: 1.0.0
category: validation
tags: [aes, compliance, validation, agent, capabilities, infrastructure, rust]
triggers:
  - "check module compliance rust"
  - "validate agent logic rust"
  - "check capabilities rust"
  - "verify infrastructure rust"
  - "aes compliance check rust"
  - "validate module logic rust"
dependencies: []
related:
  - fix-cross-import
  - fix-agent-di
---

# module_logic_validator-rust

## Rules

- Agent: NO computation, NO I/O, NO business logic
- Capabilities: NO I/O, NO network calls
- Infrastructure: NO business rules, NO domain logic

## Purpose

Validate AES layer compliance for ALL crate files.

## When to Use

- **AFTER** modifying any `agent_*`, `capabilities_*`, or `infrastructure_*` file
- **AFTER** refactoring capability modules
- **BEFORE** committing changes
- When user asks to verify compliance

## The Fundamental Question

Before keeping any code, ask:

> **"Is this code in the right layer?"**

If the answer is:

- "Agent has business logic" → **MOVE to Capabilities**
- "Capabilities has I/O" → **MOVE to Infrastructure**
- "Infrastructure has business logic" → **MOVE to Capabilities**
- "Agent imports capabilities" → **REMOVE import**
- "Code is in correct layer" → **KEEP**

## Workflow

### Step 1: Modify Target File

Modify the target file (agent__, capabilities__, or infrastructure_*).

### Step 2: Ask Questions

Ask "Is this code in the right layer?"

### Step 3: Re-fix if needed

If violations found, go back to Step 1.

## Layer Rules

### Agent Layer (`agent_*.rs`)

**Purpose:** Orchestration and pipeline execution.

| Allowed                                | Forbidden                                  |
| -------------------------------------- | ------------------------------------------ |
| `for`, `while`, `loop`                 | Computation (arithmetic, `sum()`, `len()`) |
| `if/else`, `match`                     | Business rules, domain logic               |
| `?`, `if let`                          | File I/O (`std::fs`, `File::open`)         |
| `tokio::select!`, `tokio::time::sleep` | Network (`reqwest`, `hyper`)               |
| Sequential statements                  | Database (`sqlx`, `rusqlite`)              |
|                                        | Domain model definition (`struct`)         |
|                                        | Direct import from `capabilities_*`        |
|                                        | Direct import from `infrastructure_*`      |

### Capabilities Layer (`capabilities_*.rs`)

**Purpose:** Business logic and domain rules.

| Allowed                 | Forbidden                             |
| ----------------------- | ------------------------------------- |
| Business rules          | File I/O (`std::fs`, `File::open`)    |
| Domain logic            | Network (`reqwest`, `hyper`)          |
| Calculations            | Database (`sqlx`, `rusqlite`)         |
| Data transformation     | Direct import from `infrastructure_*` |
| Validation logic        | Direct import from `agent_*`          |
| Domain model definition |                                       |
| Trait implementation    |                                       |

### Infrastructure Layer (`infrastructure_*.rs`)

**Purpose:** External system integration.

| Allowed                            | Forbidden                                |
| ---------------------------------- | ---------------------------------------- |
| File I/O (`std::fs`, `File::open`) | Business rules                           |
| Network calls                      | Domain logic                             |
| Database operations                | Calculations (should be in capabilities) |
| External API calls                 | Direct import from `agent_*`             |
| Trait implementation               | Direct import from `capabilities_*`      |

## Forbidden Import Patterns

```rust
// Agent → Capabilities [FORBIDDEN]
use crate::capabilities_*::*

// Agent → Infrastructure [FORBIDDEN]
use crate::infrastructure_*::*

// Capabilities → Infrastructure [FORBIDDEN]
use crate::infrastructure_*::*

// Capabilities → Agent [FORBIDDEN]
use crate::agent_*::*

// Infrastructure → Agent [FORBIDDEN]
use crate::agent_*::*

// Infrastructure → Capabilities [FORBIDDEN]
use crate::capabilities_*::*
```

## Quick Reference

| Layer          | Can Import From              | Cannot Import From                                           |
| -------------- | ---------------------------- | ------------------------------------------------------------ |
| taxonomy       | taxonomy                     | contract, capabilities, infrastructure, agent, surface, root |
| contract       | taxonomy, contract           | capabilities, infrastructure, agent, surface, root           |
| capabilities   | taxonomy, contract           | **infrastructure**, surface, agent, **capabilities**, root   |
| infrastructure | taxonomy, contract           | surface, **capabilities**, agent, **infrastructure**, root   |
| agent          | taxonomy, contract           | capabilities, infrastructure, surface, root                  |
| surface        | taxonomy, contract (limited) | capabilities, infrastructure, agent, root                    |
| root           | ALL layers                   | (none)                                                       |

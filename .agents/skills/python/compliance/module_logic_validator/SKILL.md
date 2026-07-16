---
name: module_logic_validator
version: 3.0.0
category: validation
tags: [aes, compliance, validation, agent, capabilities, infrastructure]
triggers:
  - "check module compliance"
  - "validate agent logic"
  - "check capabilities"
  - "verify infrastructure"
  - "aes compliance check"
  - "validate module logic"
dependencies: []
related:
  - fix_aes201_aes405_agent_contract
  - fix_aes201_capability_cross_import
  - fix_aes404_capabilities_infrastructure_mismatch
---

# module_logic_validator

## Rules

- Agent: NO computation, NO I/O, NO business logic
- Capabilities: NO I/O, NO network calls
- Infrastructure: NO business rules, NO domain logic

## Purpose

Validate AES layer compliance for ALL module files.

Validate AES layer compliance for ALL module files using AST-based analysis.

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

### Agent Layer (`agent_*.py`)

**Purpose:** Orchestration and pipeline execution.

| Allowed                      | Forbidden                                  |
| ---------------------------- | ------------------------------------------ |
| `for`, `while`, `async for`  | Computation (`sum()`, `len()`, arithmetic) |
| `if/else`, `elif`, `match`   | Business rules, domain logic               |
| `try/except`, `raise`        | File I/O (`open()`, `Path()`, `os.`)       |
| `asyncio.wait_for`, `.sleep` | Network (`requests.`, `httpx.`)            |
| Sequential statements        | Database (`sqlite3.`, `asyncpg.`)          |
|                              | Domain model definition (`@dataclass`)     |
|                              | Direct import from`capabilities_*`         |
|                              | Direct import from`infrastructure_*`       |

### Capabilities Layer (`capabilities_*.py`)

**Purpose:** Business logic and domain rules.

| Allowed                     | Forbidden                            |
| --------------------------- | ------------------------------------ |
| Business rules              | File I/O (`open()`, `Path()`, `os.`) |
| Domain logic                | Network (`requests.`, `httpx.`)      |
| Calculations                | Database (`sqlite3.`, `asyncpg.`)    |
| Data transformation         | Direct import from`infrastructure_*` |
| Validation logic            | Direct import from`agent_*`          |
| Domain model definition     |                                      |
| Protocol/ABC implementation |                                      |

### Infrastructure Layer (`infrastructure_*.py`)

**Purpose:** External system integration.

| Allowed                       | Forbidden                                |
| ----------------------------- | ---------------------------------------- |
| File I/O (`open()`, `Path()`) | Business rules                           |
| Network calls                 | Domain logic                             |
| Database operations           | Calculations (should be in capabilities) |
| External API calls            | Direct import from`agent_*`              |
| Protocol implementation       | Direct import from`capabilities_*`       |

## Forbidden Import Patterns

```python
# Agent → Capabilities [FORBIDDEN]
from modules.\w+.capabilities_\w+ import

# Agent → Infrastructure [FORBIDDEN]
from modules.\w+.infrastructure_\w+ import

# Capabilities → Infrastructure [FORBIDDEN]
from modules.\w+.infrastructure_\w+ import

# Capabilities → Agent [FORBIDDEN]
from modules.\w+.agent_\w+ import

# Infrastructure → Agent [FORBIDDEN]
from modules.\w+.agent_\w+ import

# Infrastructure → Capabilities [FORBIDDEN]
from modules.\w+.capabilities_\w+ import
```

## Pattern Detection

### Computation (Forbidden in Agent)

```python
sum\(|len\(|min\(|max\(|abs\(|round\(|pow\(|sqrt\(
```

### I/O (Forbidden in Agent & Capabilities)

```python
open\(|\.read\(|\.write\(|\.exists\(|\.mkdir\(|\.unlink\(
Path\(|\.path\.
os\.path|os\.makedirs|os\.remove|os\.listdir|os\.environ
requests\.|httpx\.|aiohttp\.|urllib\.|socket\.
sqlite3\.|asyncpg\.|sqlalchemy\.|\.execute\(|\.fetchone\(|\.fetchall\(
```

### Business Logic (Forbidden in Agent & Infrastructure)

```python
if.*compliance|if.*validation|if.*business|if.*rule
```

## See Also

- [INDEX.md](../INDEX.md) — Skills library overview
- [RULES_AES.md](../../RULES_AES.md) — AES rule definitions
- [ARCHITECTURE.md](../../ARCHITECTURE.md) — 7-layer hierarchy

---
name: fix-agent-di-rust
version: 1.0.0
category: refactoring
tags: [aes, agent, dependency-injection, di, aes201, aes405, rust]
triggers:
  - "fix agent import rust"
  - "wire agent rust"
  - "agent dependency injection rust"
dependencies: []
related:
  - module_logic_validator
---

# fix-agent-di-rust

## Rules

- Agent MUST implement aggregate trait
- Agent MUST NOT import capabilities or infrastructure directly
- Dependencies MUST be injected via constructor
- Container MUST wire all dependencies

## Purpose

Fix violations where agents import capabilities/infrastructure directly (AES201) or instantiate structs directly (AES405).

## When to Use

- Agent file has `use crate::capabilities_*`
- Agent file has `use crate::infrastructure_*`
- Agent instantiates structs directly

## The Fundamental Question

> **"Does this agent import concrete structs?"**

If yes -> **Use DI via constructor**

## Workflow

### Step 1: Find Aggregate Trait

Read file in `crates/shared/src/<domain>/contract_*_aggregate.rs`

### Step 2: Check Agent Implementation

Read agent file, ask: Does agent implement aggregate?

### Step 3: Remove Forbidden Imports

Remove imports from capabilities/infrastructure.

### Step 4: Add Constructor DI

Add constructor that receives dependencies via trait objects.

### Step 5: Update Container

Update container to pass dependencies.

## Example

```rust
// WRONG:
// agent_import_orchestrator.rs
use crate::capabilities_mandatory_checker::MandatoryChecker;  // FORBIDDEN
use crate::infrastructure_parser_adapter::ParserAdapter;      // FORBIDDEN

// CORRECT:
// agent_import_orchestrator.rs
use crate::contract_mandatory_checker_protocol::IMandatoryCheckerProtocol;
use crate::contract_parser_port::IParserPort;

pub struct ImportOrchestrator {
    checker: Arc<dyn IMandatoryCheckerProtocol>,
    parser: Arc<dyn IParserPort>,
}

impl ImportOrchestrator {
    pub fn new(
        checker: Arc<dyn IMandatoryCheckerProtocol>,
        parser: Arc<dyn IParserPort>,
    ) -> Self {
        Self { checker, parser }
    }
}
```

---
name: fix-agent-di
version: 1.0.0
category: refactoring
tags: [aes, agent, dependency-injection, di, aes201, aes405]
triggers:
  - "fix agent import"
  - "wire agent"
  - "agent dependency injection"
dependencies: []
related:
  - module_logic_validator
---

# fix-agent-di

## Rules

- Agent MUST inherit from aggregate contract
- Agent MUST NOT import capabilities or infrastructure directly
- Dependencies MUST be injected via `wire()` method
- Container MUST wire all dependencies

## Purpose

Fix violations where agents import capabilities/infrastructure directly (AES201) or instantiate classes directly (AES405).

## When to Use

- Agent file has `from modules.*.capabilities_* import`
- Agent file has `from modules.*.infrastructure_* import`
- Agent instantiates classes directly

## The Fundamental Question

> **"Does this agent import concrete classes?"**

If yes -> **Use DI via wire() method**

## Workflow

### Step 1: Find Aggregate Contract

Read file in `modules/shared/src/<domain>/contract_*_aggregate.py`

### Step 2: Check Agent Inheritance

Read agent file, ask: Does agent inherit aggregate?

### Step 3: Remove Forbidden Imports

Remove imports from capabilities/infrastructure.

### Step 4: Add wire() Method

Add method for dependency injection.

### Step 5: Update Container

Update container to pass dependencies.

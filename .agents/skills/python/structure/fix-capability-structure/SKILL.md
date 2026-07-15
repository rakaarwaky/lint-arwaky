---
name: fix-capability-structure
version: 1.0.0
category: refactoring
tags: [aes, capability, protocol, structure, aes403, aes404]
triggers:
  - "fix capability structure"
  - "create protocol"
  - "capability missing protocol"
dependencies: []
related:
  - clean-bloat
  - module_logic_validator
---

# fix-capability-structure

## Rules

- Capabilities: ZERO I/O, must inherit protocol, ALL methods in protocol
- Infrastructure: ZERO business logic, must inherit port
- One capability class = one protocol file

## Purpose

Fix violations where capability class doesn't inherit protocol ABC (AES403) or file contains mixed business logic + I/O (AES404).

## When to Use

- Adding new capability file
- Capability class has no protocol
- File contains both business logic and I/O

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes -> **capabilities_*.py + inherit protocol**
If no (has I/O) -> **split into infrastructure_*.py**

## Workflow

### Step 1: Analyze File 

Read file and check for mixed responsibilities.

### Step 2: Create/Find Protocol 

If pure business logic -> create/find protocol, inherit it.

### Step 3: Split if Mixed 

If mixed -> split into capabilities + infrastructure.

### Step 4: Ensure All Methods in Protocol 

Check that ALL methods are in protocol.

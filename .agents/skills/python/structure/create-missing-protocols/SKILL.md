---
name: create-missing-protocols
version: 1.0.0
category: refactoring
tags: [aes, protocol, contract, capability, aes403]
triggers:
  - "create protocol"
  - "add protocol"
  - "capability missing protocol"
dependencies: []
related:
  - fix-capability-structure
  - module_logic_validator
---

# create-missing-protocols

## Rules

- Every capability class MUST inherit from a protocol
- Protocol MUST define abstract methods for all public methods
- Protocol lives in `modules/shared/src/<domain>/contract_*_protocol.py`
- 1 capability class = 1 protocol file

## Purpose

Create missing protocol files for capabilities that don't inherit from any protocol (AES403 fix).

## When to Use

- Capability class has no protocol inheritance
- Lint reports AES403 violations
- New capability file added without protocol

## The Fundamental Question

> **"Does this capability have a protocol?"**

If no → **Create protocol and make capability inherit it**

## Detection Pattern

```python
# BAD: No protocol
class FrameComposer:
    def compose_frame(self): ...

# GOOD: Inherits protocol
class FrameComposer(FrameComposerProtocol):
    def compose_frame(self): ...  # implements abstract method
```

## Protocol Location

| Module | Protocol Path |
|--------|--------------|
| compositor | `modules/shared/src/compositor/contract_*_protocol.py` |
| animator | `modules/shared/src/animator/contract_*_protocol.py` |
| scripting | `modules/shared/src/scripting/contract_*_protocol.py` |

## Workflow

### Step 1: Find Capabilities Without Protocols
Check each capability file for protocol inheritance.

### Step 2: List Public Methods
Identify all public methods that should be in the protocol.

### Step 3: Create Protocol File
Create `contract_*_protocol.py` in shared module with ABC and abstract methods.

### Step 4: Update Capability
Make capability class inherit from the new protocol.

### Step 5: Verify
Run lint to confirm AES403 is resolved.

---
name: create-module-test-suite
version: 1.0.0
category: creation
tags: [aes, test, pytest, contract, unit, integration]
triggers:
  - "create tests"
  - "add tests"
  - "create test suite"
  - "module tests"
dependencies: []
related:
  - module_logic_validator
---

## Rules

- Contract tests MUST verify protocol inheritance
- Unit tests MUST cover happy path, edge cases, errors
- Integration tests MUST use real DI container
- Test IDs MUST match FRD section 10

## Coverage Targets

| Layer          | Minimum |
| -------------- | ------- |
| Capabilities   | 90%     |
| Infrastructure | 70%     |
| Agent          | 80%     |

## Purpose

Generate contract tests, unit tests, and integration tests for module capabilities.

## When to Use

- Adding new capability module
- Increasing test coverage
- Before major releases

## The Fundamental Question

> **"Does this capability have tests?"**

If no -> **Create test suite**

### Step 1: Analyze Module

- List files in `modules/<name>/src/`
- Identify layers (capabilities, infrastructure, agent)
- Check existing tests

### Step 2: Create Fixtures

- Create `tests/conftest.py`
- Add sample data, empty data, invalid data fixtures

### Step 3: Contract Tests

- Create `tests/contract/test_contract_<name>.py`
- Verify protocol compliance
- Test all required methods exist

### Step 4: Unit Tests

- Create `tests/unit/test_capabilities_<name>.py`
- Test happy path, edge cases, errors
- Target: 90% coverage for capabilities

### Step 5: Integration Tests

- Create `tests/integration/test_integration_<name>.py`
- Test cross-capability interaction
- Test DI container wiring

```
modules/<name>/tests/
  conftest.py
  contract/
    test_contract_<name>.py
  unit/
    test_capabilities_<name>.py
  integration/
    test_integration_<name>.py
```

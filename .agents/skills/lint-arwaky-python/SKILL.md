---
name: lint-arwaky-python
description: "Run lint-arwaky CLI scanner and MCP server for Python projects — validate AES compliance, check layer violations, and fix architecture issues."
version: 1.0.0
category: tooling
tags:
  [
    python,
    lint,
    aes,
    compliance,
    scanning,
    mcp,
  ]
triggers:
  - "lint arwaky python"
  - "scan python project"
  - "verify aes compliance python"
dependencies: []
related:
  - module_logic_validator-python
  - fix-cross-import-python
---

# lint-arwaky-python

## Purpose

Run lint-arwaky CLI scanner and MCP server for Python projects. Validates AES (Architecture Error Standards) compliance, checks layer violations, and helps fix architecture issues.

## When to Use

- Before committing changes
- After refactoring modules
- When verifying AES compliance
- When user asks to scan Python project

## The Fundamental Question

> **"Is this Python project AES compliant?"**

If no → **Run lint-arwaky scanner and fix violations**

## Workflow

### Step 1: Run CLI Scanner

```bash
# Scan Python project for AES violations
python -m lint_arwaky scan <project-path> --language python

# Scan specific module
python -m lint_arwaky scan modules/animator --language python

# Check specific rule
python -m lint_arwaky check aes201 --language python
```

### Step 2: Review Violations

Analyze scan results for:

- AES201 import violations (cross-layer imports)
- AES403 missing protocol inheritance
- AES404 mixed layer responsibilities
- AES405 magic constants
- AES406 surface role violations

### Step 3: Fix Violations

Use appropriate skills to fix violations:

```bash
# For import violations
python -m lint_arwaky fix cross-import modules/

# For missing protocols
python -m lint_arwaky fix protocol modules/

# For layer violations
python -m lint_arwaky fix layer modules/
```

### Step 4: Verify Fixes

Run scanner again to confirm violations resolved:

```bash
# Re-scan after fixes
python -m lint_arwaky scan <project-path> --language python

# Verify specific rule
python -m lint_arwaky check aes201 --language python
```

## AES Rules for Python

### Layer Import Rules (AES201)

```
ALLOWED:    taxonomy_*, contract_*
FORBIDDEN:  capabilities_*, infrastructure_*, agent_* (peer layers)
```

### Protocol Requirements (AES403)

- Every capability class MUST inherit from protocol ABC
- Every infrastructure class MUST inherit from port ABC
- Every agent class MUST inherit from aggregate ABC

### Layer Boundaries (AES404)

| Layer          | Can Contain                  | Cannot Contain              |
| -------------- | ---------------------------- | --------------------------- |
| capabilities   | Pure computation, validation | I/O, network, database      |
| infrastructure | I/O, network, database       | Business logic, computation |
| agent          | Orchestration flow           | Computation, I/O, business  |

## Quick Commands

```bash
# Scan entire project
python -m lint_arwaky scan modules/ --language python

# Check specific rule
python -m lint_arwaky check aes201 --language python

# Fix violations automatically
python -m lint_arwaky fix all modules/ --language python

# Run MCP server for IDE integration
python -m lint_arwaky mcp --language python
```

## Verification Checklist

- [ ] All layer imports follow AES201 rules
- [ ] All classes inherit appropriate protocol ABCs (AES403)
- [ ] No mixed responsibilities in layers (AES404)
- [ ] No magic constants in layers (AES405)
- [ ] Surface files follow role-based imports (AES406)

## Common Issues (FIX)

| Issue | Fix Strategy |
| --- | --- |
| Cross-layer imports | Use contract layer protocols via DI |
| Missing protocol inheritance | Create protocol ABC and inherit |
| Mixed layer responsibilities | Move code to appropriate layer |
| Magic constants | Extract to taxonomy constants |
| Surface importing capabilities | Use aggregate contracts instead |

---
name: lint-arwaky-typescript
description: "Run lint-arwaky CLI scanner and MCP server for TypeScript projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [typescript, lint, aes, compliance, scanning, mcp]
  triggers:
    - "lint arwaky typescript"
    - "scan typescript project"
    - "verify aes compliance typescript"
  dependencies: []
  related:
    - cleanup-files-typescript
    - create-capabilities-typescript
---

---

# lint-arwaky-typescript

## Purpose

Run lint-arwaky CLI scanner and MCP server for TypeScript projects. Validates AES (Architecture Error Standards) compliance, checks layer violations, and helps fix architecture issues.

## When to Use

- Before committing changes
- After refactoring modules
- When verifying AES compliance
- When user asks to scan TypeScript project

## The Fundamental Question

> **"Is this TypeScript project AES compliant?"**

If no → **Run lint-arwaky scanner and fix violations**

## Workflow

### Step 1: Run CLI Scanner

```bash
# Scan TypeScript project for AES violations
cargo run --bin lint-arwaky-cli -- scan <project-path> for typescript

# Scan specific package
cargo run --bin lint-arwaky-cli -- scan packages/animator for typescript

# Check specific rule
cargo run --bin lint-arwaky-cli -- check aes201 --language typescript
```

### Step 2: Review Violations

Analyze scan results for:

- AES201 import violations (cross-layer imports)
- AES403 missing interface inheritance
- AES404 mixed layer responsibilities
- AES405 magic constants
- AES406 surface role violations

### Step 3: Fix Violations

Use appropriate skills to fix violations:

```bash
# For import violations
cargo run --bin lint-arwaky-cli -- fix cross-import packages/

# For missing interfaces
cargo run --bin lint-arwaky-cli -- fix protocol packages/

# For layer violations
cargo run --bin lint-arwaky-cli -- fix layer packages/
```

### Step 4: Verify Fixes

Run scanner again to confirm violations resolved:

```bash
# Re-scan after fixes
cargo run --bin lint-arwaky-cli -- scan <project-path> for typescript

# Verify specific rule
cargo run --bin lint-arwaky-cli -- check aes201 --language typescript
```

## AES Rules for TypeScript

### Layer Import Rules (AES201)

```
ALLOWED:    taxonomy_*, contract_*
FORBIDDEN:  capabilities_*, infrastructure_*, agent_* (peer layers)
```

### Interface Requirements (AES403)

- Every capability class MUST implement a protocol interface
- Every infrastructure class MUST implement a port interface
- Every agent class MUST implement an aggregate interface

### Layer Boundaries (AES404)

| Layer          | Can Contain                  | Cannot Contain              |
| -------------- | ---------------------------- | --------------------------- |
| capabilities   | Pure computation, validation | I/O, network, database      |
| infrastructure | I/O, network, database       | Business logic, computation |
| agent          | Orchestration flow           | Computation, I/O, business  |

## Quick Commands

```bash
# Scan entire project
cargo run --bin lint-arwaky-cli -- scan packages/ for typescript

# Check specific rule
cargo run --bin lint-arwaky-cli -- check aes201 --language typescript

# Fix violations automatically
cargo run --bin lint-arwaky-cli -- fix all packages/ --language typescript

# Run MCP server for IDE integration
cargo run --bin lint-arwaky-mcp
```

## Verification Checklist

- [ ] All layer imports follow AES201 rules
- [ ] All classes implement appropriate protocol interfaces (AES403)
- [ ] No mixed responsibilities in layers (AES404)
- [ ] No magic constants in layers (AES405)
- [ ] Surface files follow role-based imports (AES406)

## Common Issues (FIX)

| Issue                          | Fix Strategy                            |
| ------------------------------ | --------------------------------------- |
| Cross-layer imports            | Use contract layer interfaces via DI    |
| Missing interface inheritance  | Create protocol interface and implement |
| Mixed layer responsibilities   | Move code to appropriate layer          |
| Magic constants                | Extract to taxonomy constants           |
| Surface importing capabilities | Use aggregate interfaces instead        |

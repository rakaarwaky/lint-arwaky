# Skills Index

> Rust AES (Agentic Engineering System) Skills Library
> Version: 1.0.0 | Last Updated: 2026-07-15

---

## Quick Navigation

| Category | Purpose | Skills |
|----------|---------|--------|
| [Imports](#imports) | Import rules & DI | 1 |
| [Refactoring](#refactoring) | Interface & trait consolidation | 1 |

---

## Imports

Skills for import rules and dependency injection.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [fix-cross-import-rust](fix-cross-import/SKILL.md) | Fix cross-import violations (AES201) | Capabilities import from capabilities or infrastructure from infrastructure |

---

## Refactoring

Skills for interface design and trait consolidation.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [trait-consolidation-rust](trait-consolidation/SKILL.md) | Consolidate all fn into trait protocol | Capability has methods not in trait contract |

---

## Workflow

```
1. Code Change
   ↓
2. fix-cross-import-rust (check AES201 compliance)
   ↓
3. trait-consolidation-rust (merge fn into trait)
   ↓
4. cargo check (verify compilation)
   ↓
5. Commit
```

---

## AES Rule Reference

| Rule | Category | Description |
|------|----------|-------------|
| AES101 | Naming | File naming conventions |
| AES102 | Naming | Suffix restrictions |
| AES201 | Import | Forbidden cross-layer imports |
| AES202 | Import | Missing mandatory imports |
| AES204 | Dummy | Dummy import/function detection |
| AES303 | Structure | Class wrapping requirements |
| AES403 | Protocol | Capability protocol implementation |

---

*Rust Skills Library — AES Architecture Compliance*
*Project: lint-arwaky*

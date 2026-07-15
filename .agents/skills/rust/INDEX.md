# Skills Index

> Rust AES (Agentic Engineering System) Skills Library
> Version: 2.0.0 | Last Updated: 2026-07-16

---

## Quick Navigation

| Category | Purpose | Skills |
|----------|---------|--------|
| [Structure](#structure) | File & struct organization | 6 |
| [Imports](#imports) | Import rules & DI | 3 |
| [Quality](#quality) | Code cleanup & best practices | 5 |
| [Compliance](#compliance) | Validation & linting | 3 |
| [Testing](#testing) | Test creation | 1 |

---

## Structure

Skills for file organization and struct layout.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [fix-capability-structure](structure/fix-capability-structure/SKILL.md) | Ensure 1 struct = 1 trait, no I/O | Adding new capability file |
| [fix-naming](structure/fix-naming/SKILL.md) | Fix file naming conventions | File name doesn't follow pattern |
| [merge-overlap](structure/merge-overlap/SKILL.md) | Merge overlapping files | Multiple files do same thing |
| [merge-files](structure/merge-files/SKILL.md) | Merge two files into one | Two impl files share same domain |
| [enforce-1-struct-per-file](structure/enforce-1-struct-per-file/SKILL.md) | Ensure 1 struct per file | File has >1 struct |
| [create-missing-protocols](structure/create-missing-protocols/SKILL.md) | Create traits for capabilities | Capability has no trait |
| [trait-consolidation](structure/trait-consolidation/SKILL.md) | Consolidate all fn into trait | Capability has methods not in trait |

---

## Imports

Skills for import rules and dependency injection.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [fix-cross-import](imports/fix-cross-import/SKILL.md) | Fix cross-import violations (AES201) | Capabilities import from capabilities or infrastructure from infrastructure |
| [fix-agent-di](imports/fix-agent-di/SKILL.md) | Fix agent to use DI | Agent imports concrete structs |
| [fix-surface-import](imports/fix-surface-import/SKILL.md) | Fix surface imports | Surface imports capabilities |

---

## Quality

Skills for code cleanup and best practices.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [clean-bloat](quality/clean-bloat/SKILL.md) | Remove stubs, thin wrappers | After refactoring |
| [fix-bypass-comments](quality/fix-bypass-comments/SKILL.md) | Remove allow, unwrap, expect | File has bypass comments |
| [fix-magic-constant](quality/fix-magic-constant/SKILL.md) | Replace hardcoded values | Magic constants in code |
| [fix-primitive-to-vo](quality/fix-primitive-to-vo/SKILL.md) | Replace primitives with VOs | Raw types in signatures |
| [find-unused-files](quality/find-unused-files/SKILL.md) | Find dead/orphan files | After refactoring |

---

## Compliance

Skills for validation and linting.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [module_logic_validator](compliance/module_logic_validator/SKILL.md) | Validate AES layer compliance | After modifying any file |
| [method_classifier](compliance/method_classifier/SKILL.md) | Validate Public/Helper/Utility | After adding methods |
| [lint-arwaky-cli](compliance/lint-arwaky-cli/SKILL.md) | Rust linting (clippy, rustfmt) | Before committing |

---

## Testing

Skills for test creation.

| Skill | Purpose | When to Use |
|-------|---------|-------------|
| [create-crate-test-suite](testing/create-crate-test-suite/SKILL.md) | Create contract, unit, integration tests | Adding tests to crate |

---

## Workflow

```
1. Code Change
   ↓
2. compliance/module_logic_validator (check layer compliance)
   ↓
3. Any fix-* skill (fix violations)
   ↓
4. quality/clean-bloat (remove bloat)
   ↓
5. quality/find-unused-files (find dead code)
   ↓
6. compliance/lint-arwaky-cli (final lint)
   ↓
7. Commit
```

---

## AES Rule Reference

| Rule | Category | Description |
|------|----------|-------------|
| AES101 | Naming | File naming conventions |
| AES102 | Naming | Suffix restrictions |
| AES201 | Import | Forbidden cross-layer imports |
| AES202 | Import | Missing mandatory imports |
| AES301 | Structure | File size limits |
| AES304 | Quality | Bypass comment prohibition |
| AES401 | Type | Primitive to VO conversion |
| AES402 | Type | VO validation requirements |
| AES403 | Protocol | Capability trait implementation |
| AES404 | Layer | Capabilities/Infrastructure mismatch |
| AES405 | Agent | Agent role violations |
| AES501 | Taxonomy | Domain model definition location |

---

*Rust Skills Library — AES Architecture Compliance*
*Project: lint-arwaky*

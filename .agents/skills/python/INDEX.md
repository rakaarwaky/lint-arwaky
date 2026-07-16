# Skills Index

> AES (Agentic Engineering System) Skills Library
> Version: 3.1.0 | Last Updated: 2026-07-14

---

## Quick Navigation

| Category                  | Purpose                       | Skills |
| ------------------------- | ----------------------------- | ------ |
| [Structure](#structure)   | File & class structure        | 6      |
| [Imports](#imports)       | Import rules & DI             | 3      |
| [Quality](#quality)       | Code cleanup & best practices | 5      |
| [Compliance](#compliance) | Validation & linting          | 3      |
| [Testing](#testing)       | Test creation                 | 1      |

---

## Structure

Skills for file organization and class structure.

| Skill                                                                   | Purpose                             | When to Use                      |
| ----------------------------------------------------------------------- | ----------------------------------- | -------------------------------- |
| [fix-capability-structure](structure/fix-capability-structure/SKILL.md) | Ensure 1 class = 1 protocol, no I/O | Adding new capability file       |
| [fix-class-wrapping](structure/fix-class-wrapping/SKILL.md)             | Wrap functions in classes           | File has module-level functions  |
| [fix-naming](structure/fix-naming/SKILL.md)                             | Fix file naming conventions         | File name doesn't follow pattern |
| [merge-overlap](structure/merge-overlap/SKILL.md)                       | Merge overlapping files             | Multiple files do same thing     |
| [enforce-1-class-per-file](structure/enforce-1-class-per-file/SKILL.md) | Ensure 1 class per file             | File has >1 class                |
| [create-missing-protocols](structure/create-missing-protocols/SKILL.md) | Create protocols for capabilities   | Capability has no protocol       |

---

## Imports

Skills for import rules and dependency injection.

| Skill                                                     | Purpose                     | When to Use                                                                 |
| --------------------------------------------------------- | --------------------------- | --------------------------------------------------------------------------- |
| [fix-cross-import](imports/fix-cross-import/SKILL.md)     | Fix cross-import violations | Capabilities import from capabilities or infrastructure from infrastructure |
| [fix-agent-di](imports/fix-agent-di/SKILL.md)             | Fix agent to use DI         | Agent imports concrete classes                                              |
| [fix-surface-import](imports/fix-surface-import/SKILL.md) | Fix surface imports         | Surface imports capabilities                                                |

---

## Quality

Skills for code cleanup and best practices.

| Skill                                                       | Purpose                                 | When to Use              |
| ----------------------------------------------------------- | --------------------------------------- | ------------------------ |
| [clean-bloat](quality/clean-bloat/SKILL.md)                 | Remove stubs, thin wrappers, re-exports | After refactoring        |
| [fix-bypass-comments](quality/fix-bypass-comments/SKILL.md) | Remove noqa, type: ignore               | File has bypass comments |
| [fix-magic-constant](quality/fix-magic-constant/SKILL.md)   | Replace hardcoded values                | Magic constants in code  |
| [fix-primitive-to-vo](quality/fix-primitive-to-vo/SKILL.md) | Replace primitives with VOs             | Raw types in signatures  |
| [find-unused-files](quality/find-unused-files/SKILL.md)     | Find dead/orphan files                  | After refactoring        |

---

## Compliance

Skills for validation and linting.

| Skill                                                                | Purpose                        | When to Use              |
| -------------------------------------------------------------------- | ------------------------------ | ------------------------ |
| [module_logic_validator](compliance/module_logic_validator/SKILL.md) | Validate AES layer compliance  | After modifying any file |
| [method_classifier](compliance/method_classifier/SKILL.md)           | Validate Public/Helper/Utility | After adding methods     |
| [lint-arwaky-cli](compliance/lint-arwaky-cli/SKILL.md)               | Multi-language linting         | Before committing        |

---

## Testing

Skills for test creation.

| Skill                                                                 | Purpose                                  | When to Use            |
| --------------------------------------------------------------------- | ---------------------------------------- | ---------------------- |
| [create-module-test-suite](testing/create-module-test-suite/SKILL.md) | Create contract, unit, integration tests | Adding tests to module |

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

| Rule   | Category  | Description                          |
| ------ | --------- | ------------------------------------ |
| AES101 | Naming    | File naming conventions              |
| AES102 | Naming    | Suffix restrictions                  |
| AES201 | Import    | Forbidden cross-layer imports        |
| AES202 | Import    | Missing mandatory imports            |
| AES303 | Structure | Class wrapping requirements          |
| AES304 | Quality   | Bypass comment prohibition           |
| AES401 | Type      | Primitive to VO conversion           |
| AES402 | Type      | VO validation requirements           |
| AES403 | Protocol  | Capability protocol implementation   |
| AES404 | Layer     | Capabilities/Infrastructure mismatch |
| AES405 | Agent     | Agent role violations                |
| AES501 | Taxonomy  | Domain model definition location     |

---

_Skills Library — AES Architecture Compliance_
_Project: PSD Timelapse_

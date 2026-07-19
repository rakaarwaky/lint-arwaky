---
name: add-docs-typescript
description: "Add proper JSDoc comments and type annotations to TypeScript files following project conventions."
version: 1.0.0
category: documentation
tags: [typescript, docs, jsdoc, type-hints, refactoring]
triggers:
  - "add docs typescript"
  - "add jsdoc typescript"
  - "add type hints typescript"
dependencies: []
related:
  - cleanup-files-typescript
  - consolidate-files-typescript
---

# add-docs-typescript

## Purpose

Add proper JSDoc comments and type annotations to TypeScript files following project conventions. Ensures all modules, classes, and functions have clear documentation.

## Rules

### JSDoc Style

- **Module docstrings**: One-liner at top of file describing module purpose
- **Class docstrings**: One-liner describing class purpose and behavior
- **Method docstrings**: Describe purpose, parameters, return values, and exceptions

### Type Annotations

- Use TypeScript type annotations for all function parameters and return types
- Use interfaces for object shapes
- Use type aliases for unions and intersections

## When to Use

- New files without docstrings
- Files with incomplete type annotations
- Before committing changes to shared/taxonomy or contract layers

## The Fundamental Question

> **"Is this code documented and typed?"**

If no → **Add JSDoc and type annotations**

## Workflow

### Step 1: Analyze File

Read file and identify undocumented modules, classes, and functions.

### Step 2: Add Module Docstring

Add one-liner at top of file:

```typescript
/** Taxonomy value objects for import rules. */
```

### Step 3: Add Class Docstrings

Add class-level documentation:

```typescript
/** Value object representing an import rule with pattern and message. */
class ImportRuleVO {
    // ...
}
```

### Step 4: Add Method Docstrings

Add method documentation with parameters, returns, and throws:

```typescript
/**
 * Check if path matches the import rule.
 * @param path - File path to check
 * @returns True if path matches the rule
 * @throws ValueError - If path is empty
 */
check(path: string): boolean {
    // ...
}
```

### Step 5: Add Type Annotations

Add type annotations to all function signatures:

```typescript
validate(data: Record<string, unknown>): [boolean, string] {
    // ...
}
```

## Verification Checklist

- [ ] All modules have one-liner docstrings
- [ ] All classes have descriptive docstrings
- [ ] All public methods have parameter/return documentation
- [ ] All function signatures use type annotations
- [ ] Complex types use interfaces or type aliases

## Quick Commands

```bash
# Check files without docstrings
find packages/ -name "*.ts" | while read f; do
    head -1 "$f" | grep -q "^/\*\*" || echo "NO DOCSTRING: $f"
done

# Check for missing type annotations
grep -rn "function " packages/*/src/ | grep -v ": " | head -20

# Run TypeScript type checker
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Missing module docstrings**: Every file needs a one-liner at the top
- ❌ **Incomplete parameter documentation**: All parameters must be documented
- ❌ **Using @ts-ignore without reason**: Fix the root cause instead of suppressing errors
- ❌ **Over-documenting obvious code**: Keep docstrings concise and meaningful

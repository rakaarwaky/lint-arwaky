---
name: fix-bypass-typescript
description: "Fix TypeScript bypass comments (@ts-ignore, @ts-expect-error) by addressing root causes instead of suppressing errors."
metadata:
    tags: [typescript, bypass, comments, type-hints, refactoring, ts-ignore]
    triggers:
        - "fix bypass typescript"
        - "remove ts-ignore typescript"
        - "remove ts-expect-error typescript"
    dependencies: []
    related:
        - cleanup-files-typescript
---

# fix-bypass-typescript

## Rules

- NO `@ts-ignore` allowed without justification
- NO `@ts-expect-error` allowed without justification
- NO `// eslint-disable` allowed without justification
- Fix the root cause instead of suppressing errors

## Purpose

Remove `@ts-ignore`, `@ts-expect-error`, `// eslint-disable` comments and fix the underlying type/error issues.

## When to Use

- File has bypass comments
- Type checker reports errors that are suppressed
- Linter violations hidden by eslint-disable

## The Fundamental Question

> **"Why is there a bypass comment?"**

If yes → **Fix root cause and remove comment**

## Workflow

### Step 1: Find Bypass Comments

Read code and find bypass comments:

```bash
# Find ts-ignore comments
grep -rn "@ts-ignore" packages/*/src/

# Find ts-expect-error comments
grep -rn "@ts-expect-error" packages/*/src/

# Find eslint-disable comments
grep -rn "eslint-disable" packages/*/src/
```

### Step 2: Fix Root Cause

Fix underlying type error or lint violation:

- For `@ts-ignore` → Add proper type annotations
- For `@ts-expect-error` → Fix the type error
- For `eslint-disable` → Fix the linting issue (formatting, naming, etc.)

### Step 3: Remove Comment

Remove the bypass comment once root cause is fixed.

## Detection Patterns

### Ts-Ignore Comments

```typescript
// BAD: Suppressing type errors
function process(data: any) {  // @ts-ignore
    // Fix: Add proper type annotations
}
```

### Ts-Expect-Error Comments

```typescript
// BAD: Suppressing type errors
const result = someFunction() as any;  // @ts-expect-error
// Fix: Add proper type assertions or fix the function signature
```

### Eslint-Disable Comments

```typescript
// BAD: Suppressing linting errors
import fs from 'fs';  // eslint-disable-line no-unused-vars

// Fix: Remove unused imports or address the violation
```

## Verification Checklist

- [ ] All `@ts-ignore` comments removed (or justified)
- [ ] All `@ts-expect-error` comments removed (or justified)
- [ ] All `// eslint-disable` comments removed (or justified)
- [ ] Type checker passes without errors
- [ ] Linter passes without violations

## Quick Commands

```bash
# Find ts-ignore comments
grep -rn "@ts-ignore" packages/*/src/

# Find ts-expect-error comments
grep -rn "@ts-expect-error" packages/*/src/

# Find eslint-disable comments
grep -rn "eslint-disable" packages/*/src/

# Run TypeScript type checker
npx tsc --noEmit

# Run ESLint
npx eslint packages/ --max-warnings 0
```

## Common Mistakes (AVOID)

- ❌ **Keeping bypass comments without fixing**: Always fix the root cause
- ❌ **Adding @ts-ignore for wrong reasons**: Only use when type system can't express the truth
- ❌ **Suppressing legitimate errors**: Fix formatting/naming issues instead of hiding them

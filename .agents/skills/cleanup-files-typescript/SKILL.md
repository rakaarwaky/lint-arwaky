---
name: cleanup-files-typescript
description: "Clean up TypeScript files by removing unused imports, fixing formatting, and ensuring ESLint/Prettier compliance."
version: 1.0.0
category: cleanup
tags: [typescript, cleanup, eslint, prettier, formatting, imports, refactoring]
triggers:
  - "cleanup typescript"
  - "fix formatting typescript"
  - "remove unused imports typescript"
dependencies: []
related:
  - add-docs-typescript
  - consolidate-files-typescript
---

# cleanup-files-typescript

## Purpose

Clean up TypeScript files by removing unused imports, fixing formatting issues, and ensuring ESLint/Prettier compliance. Prepares files for production use.

## Rules

### ESLint/Prettier Compliance

- Maximum line length: 100 characters (Prettier default)
- Use 2 spaces for indentation (no tabs)
- Semicolons required (Prettier default)
- Single quotes for strings (Prettier default)

### Import Ordering

1. Node.js built-in modules
2. Third-party packages
3. Local application imports
4. All imports must be alphabetical within each group

### Unused Code

- Remove unused imports (`import X` but never use `X`)
- Remove unused variables and functions
- Remove commented-out code blocks

## When to Use

- After refactoring files
- Before committing changes
- When cleaning up merged branches

## The Fundamental Question

> **"Is this file clean and formatted?"**

If no → **Run cleanup tools**

## Workflow

### Step 1: Remove Unused Imports

```bash
# Using ESLint to remove unused imports
npx eslint packages/ --fix --rule 'no-unused-vars: error'

# Using ts-prune to find unused exports
npx ts-prune packages/
```

### Step 2: Format Code

```bash
# Using Prettier to format code
npx prettier --write packages/ --single-quote --trailing-comma es5

# Using ESLint to auto-fix
npx eslint packages/ --fix
```

### Step 3: Check ESLint Compliance

```bash
# Using ESLint to check compliance
npx eslint packages/ --max-warnings 0
```

### Step 4: Remove Commented Code

Review files for commented-out code blocks and remove them.

## Verification Checklist

- [ ] All imports are sorted alphabetically by group
- [ ] No unused imports remain
- [ ] No unused variables or functions
- [ ] All lines under 100 characters
- [ ] Proper semicolons and quotes
- [ ] No commented-out code blocks

## Quick Commands

```bash
# Remove unused imports
npx eslint packages/ --fix --rule 'no-unused-vars: error'

# Format with Prettier
npx prettier --write packages/ --single-quote --trailing-comma es5

# Check ESLint compliance
npx eslint packages/ --max-warnings 0

# Find commented code blocks
grep -rn "^//\s.*function\|^//\s.*class\|^//\s.*const" packages/*/src/
```

## Common Mistakes (AVOID)

- ❌ **Keeping commented-out code**: Remove or commit properly instead of leaving comments
- ❌ **Mixing import groups**: Node.js built-in, third-party, and local imports must be separate
- ❌ **Ignoring line length limits**: Keep lines under 100 characters for Prettier compatibility

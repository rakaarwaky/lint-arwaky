---
name: consolidate-files-typescript
description: "Consolidate multiple TypeScript files into single cohesive modules following single responsibility principle."
version: 1.0.0
category: refactoring
tags: [typescript, consolidation, single-responsibility, refactoring, structure]
triggers:
  - "consolidate typescript"
  - "merge files typescript"
  - "combine modules typescript"
dependencies: []
related:
  - add-docs-typescript
  - cleanup-files-typescript
  - create-capabilities-typescript
---

# consolidate-files-typescript

## Purpose

Consolidate multiple TypeScript files into single cohesive modules following single responsibility principle. Ensures each module has one clear purpose and all related code lives together.

## Rules

### Single Responsibility

- Each file should have ONE clear purpose
- Related classes/functions belong in the same file
- Unrelated code must be split into separate files

### File Organization

- Place related functionality together in modules
- Use `index.ts` for module exports and re-exports
- Keep public API clear through named exports

## When to Use

- Files with scattered responsibilities
- Multiple small files that belong together
- After refactoring that split code across files

## The Fundamental Question

> **"Do these files serve the same purpose?"**

If yes → **Consolidate into single module**

## Workflow

### Step 1: Analyze File Responsibilities

Read files and identify related functionality:

```bash
# List classes/functions in files
grep -rn "^class \|^function \|^export " packages/*/src/ | sort
```

### Step 2: Identify Consolidation Candidates

Files that should be merged:

- Multiple files with related classes (e.g., `parser.ts`, `parser_utils.ts`)
- Files that only import from each other
- Split functionality across multiple small files

### Step 3: Merge Related Code

Move classes/functions to target file:

```typescript
// Before: parser.ts and parser_utils.ts
// After: Single parser.ts with all related code
```

### Step 4: Update Imports

Fix all imports across the codebase:

```bash
# Find files importing from removed modules
grep -rn "from.*parser_utils" packages/
```

### Step 5: Verify

Run TypeScript compiler and tests:

```bash
npx tsc --noEmit
npx vitest run
```

## Verification Checklist

- [ ] Consolidated file has single clear purpose
- [ ] All related classes/functions are in same file
- [ ] No scattered functionality across multiple files
- [ ] All imports updated to reflect new structure
- [ ] `index.ts` exports consolidated module correctly
- [ ] Tests pass after consolidation

## Quick Commands

```bash
# Find files with related functionality
grep -rn "^class " packages/*/src/ | sort | uniq -f1

# Check for files that only import from each other
grep -rn "^from.*import\|^import.*from" packages/*/src/index.ts

# Verify imports after consolidation
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Merging unrelated files**: Only consolidate files with clear shared purpose
- ❌ **Forgetting to update imports**: All references must be updated after consolidation
- ❌ **Breaking module exports**: Ensure `index.ts` exports are maintained

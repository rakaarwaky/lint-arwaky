---
name: add-docs-typescript
description: "Add proper JSDoc comments, type annotations, and crate-level FRD.md/README.md to TypeScript packages following project conventions."
version: 1.1.0
category: documentation
tags: [typescript, docs, jsdoc, type-hints, frd, readme]
triggers:
  - "add docs typescript"
  - "add jsdoc typescript"
  - "add type hints typescript"
  - "add frd typescript"
  - "add package readme typescript"
dependencies: []
related:
  - cleanup-files-typescript
  - consolidate-files-typescript
---

# add-docs-typescript

## Rules

- Every TypeScript package directory (e.g. `packages/<name>/`) MUST contain TWO crate-level docs: `FRD.md` and `README.md`.
- `FRD.md` is STATELESS — it describes the IDEAL TARGET only. It MUST NOT record progress, status, current-state notes, or "what's done so far". If reality diverges, fix `README.md`, never pollute `FRD.md` with state.
- `README.md` describes the REAL CURRENT STATE — what actually exists today. It is allowed (and expected) to diverge from the ideal target in `FRD.md`.
- Relationship: **FRD = ideal target, README = current reality.** README should call out gaps vs FRD; FRD must stay clean of any "as-built" noise.
- All public modules, classes, and functions MUST have JSDoc comments and type annotations.
- Doc comments MUST explain "what" and "why", not "how" (code shows how).

## Purpose

Add package-level documentation and JSDoc/type annotations:
- `FRD.md` — stateless ideal target (Feature Goal / Requirements & Scope / Success Indicators).
- `README.md` — real current state (what exists, public API surface, known gaps vs FRD).
- JSDoc comments + type annotations on all public items.

## When to Use

- New package has no `FRD.md` or no `README.md`.
- `FRD.md` contains state/progress notes (violates stateless rule) — clean it.
- README and FRD are conflated (state leaking into FRD) — split them.
- Public modules/classes/functions lack JSDoc or type annotations.
- User asks to document the package or add docs.

## The Fundamental Question

> **"Can a newcomer understand this package's purpose in 30 seconds?"**

If no -> **Add FRD.md (ideal target) + README.md (reality).**

> **"Is this code documented and typed?"**

If no -> **Add JSDoc and type annotations.**

## Detection Patterns

### Missing FRD.md / README.md (Create)

```
packages/<name-folder>/
├── src/
│   ├── index.ts
│   └── ...
├── tests/
├── FRD.md        # stateless ideal target
└── README.md     # real current state
```

### Missing JSDoc / Type Annotations (Add)

```typescript
// PURPOSE explain file in one sentence
class ImportRuleVO {
    // ...
}

// [OK] JSDoc + type annotations
/** Value object representing an import rule with pattern and message. */
class ImportRuleVO {
    // ...
}
```

## FRD.md Template (STATELESS — ideal target only)

```markdown
# FRD — <package-name>

> Stateless document. Describes the IDEAL TARGET only. Never record progress,
> status, or current-state notes. If reality diverges from this, update
> README.md — do NOT add state to this file.

## Feature Goal
<One paragraph: what this package is supposed to accomplish when complete.>

## Requirements & Scope
- In scope: <...>
- Out of scope: <...>

## Success Indicators
- [ ] <measurable ideal outcome>
- [ ] <measurable ideal outcome>
```

## README.md Template (REAL current state)

```markdown
# <package-name>

> Current real state — what actually exists today. May diverge from FRD.md
> (the ideal target). Keep this honest; gaps belong here, not in FRD.

## What exists now
- <real modules / features implemented>
- <real behavior>

## Public API surface
- `<Class>` — <one-line reality of what it does>
- `<function>` — <...>

## Known gaps vs FRD
- <deviation from ideal target — what's missing or different>
```

## Workflow

### Step 1: Analyze Package

- List files in `packages/<name>/src/`
- Identify public modules, classes, and functions
- Check existing docs (README.md / FRD.md / JSDoc / type annotations)

### Step 2: Create / Fix FRD.md (ideal target, stateless)

Write package-level FRD.md following the FRD template. It MUST contain only:

1. Feature Goal
2. Requirements & Scope
3. Success Indicators

Strip any state, progress, or "as-built" notes. FRD is the ideal target — it never changes because code isn't done yet.

### Step 3: Create / Update README.md (reality)

Write README.md reflecting the ACTUAL current state:

1. What exists now (real modules, real behavior)
2. Public API surface (real items)
3. Known gaps vs FRD (where reality diverges from the ideal target)

README is the current reality — it changes as the package evolves.

### Step 4: Add JSDoc Comments

- **Module docstrings**: One-liner at top of file describing module purpose
- **Class docstrings**: One-liner describing class purpose and behavior
- **Method docstrings**: Describe purpose, parameters, return values, and exceptions

```typescript
/** Taxonomy value objects for import rules. */

/** Value object representing an import rule with pattern and message. */
class ImportRuleVO {
    // ...
}

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

- Use TypeScript type annotations for all function parameters and return types
- Use interfaces for object shapes
- Use type aliases for unions and intersections

```typescript
validate(data: Record<string, unknown>): [boolean, string] {
    // ...
}
```

## Verification Checklist

- [ ] FRD.md exists and is stateless (no progress/state notes)
- [ ] README.md exists and reflects real current state with gaps vs FRD
- [ ] All modules have one-liner docstrings
- [ ] All classes have descriptive docstrings
- [ ] All public methods have parameter/return documentation
- [ ] All function signatures use type annotations
- [ ] Complex types use interfaces or type aliases

## Quick Commands

```bash
# Check files without docstrings
find packages/ -name "*.ts" | while read f; do
    head -1 "$f" | grep -q '^/\*\*' || echo "NO DOCSTRING: $f"
done

# Run TypeScript type checker
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Missing module docstrings**: Every file needs a one-liner at the top
- ❌ **Incomplete parameter documentation**: All parameters must be documented
- ❌ **Using @ts-ignore without reason**: Fix the root cause instead of suppressing errors
- ❌ **State leaking into FRD.md**: FRD is stateless — put reality/gaps in README
- ❌ **Over-documenting obvious code**: Keep docstrings concise and meaningful

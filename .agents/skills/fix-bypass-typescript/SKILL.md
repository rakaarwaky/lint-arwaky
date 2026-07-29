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
    - cleanup-consolidate-typescript
---

# fix-bypass-typescript

**Rule:** Fix the root cause instead of suppressing errors. No `@ts-ignore`, `@ts-expect-error`, or `// eslint-disable` without justification.

## Workflow

1. **Find** bypass comments:
   ```bash
   grep -rn "@ts-ignore" packages/*/src/
   grep -rn "@ts-expect-error" packages/*/src/
   grep -rn "eslint-disable" packages/*/src/
   ```
2. **Diagnose** — Why is there a bypass comment? What error is it hiding?
3. **Fix root cause:**
   - `@ts-ignore` → Add proper type annotations.
   - `@ts-expect-error` → Fix the type error or update the signature.
   - `eslint-disable` → Fix the lint violation (unused imports, naming, etc.).
4. **Remove** the bypass comment.
5. **Verify:**
   ```bash
   npx tsc --noEmit
   npx eslint packages/ --max-warnings 0
   ```

## Verification Checklist

- [ ] All `@ts-ignore` removed (or justified with explanation).
- [ ] All `@ts-expect-error` removed (or justified with explanation).
- [ ] All `// eslint-disable` removed (or justified with explanation).
- [ ] Type checker passes without errors.
- [ ] Linter passes without violations.

---
name: fix-bypass
description: Fixes bypass suppressions at root cause. Use when cleaning type ignore, noqa, allow, unwrap.
metadata:
  tags:
    [
      python,
      rust,
      typescript,
      bypass,
      comments,
      type-hints,
      refactoring,
      noqa,
      aes304,
      allow,
      unwrap,
      ts-ignore,
    ]
  triggers:
    - "fix bypass python"
    - "remove noqa python"
    - "remove type ignore python"
    - "fix bypass rust"
    - "fix bypass comments rust"
    - "remove allow rust"
    - "remove unwrap rust"
    - "fix bypass typescript"
    - "remove ts-ignore typescript"
    - "remove ts-expect-error typescript"
  dependencies: []
  related:
    - cleanup-consolidate
    - add-docs
    - lint-arwaky
---
# fix-bypass

**Rule:** Fix the root cause instead of suppressing errors. No bypass comment or unsafe call without justification.

## The Fundamental Question

> **"Is there a bypass comment or unsafe call?"**

If yes → **Fix root cause and remove.**

## Workflow

1. **Find** bypass comments and unsafe calls (patterns + commands per language below).
2. **Diagnose** — Why is there a bypass comment? What error is it hiding?
3. **Fix root cause** — use the per-language fix mapping below.
4. **Remove** the bypass comment or unsafe call.
5. **Verify** — run the per-language type checker and linter; both must pass clean.

Pick the language section matching the codebase: [Python](#python) · [Rust](#rust) · [TypeScript](#typescript).

---

## Python

Bypass forms: `# type: ignore`, `# noqa`.

**Find:**

```bash
grep -rn "type: ignore" modules/*/src/
grep -rn "noqa" modules/*/src/
```

**Fix root cause:**

- `type: ignore` → Add proper type annotations.
- `noqa` → Fix the lint violation (formatting, naming, unused imports, etc.).

**Verify:**

```bash
python -m mypy modules/ --ignore-missing-imports
pycodestyle modules/ --max-line-length=88
```

### Verification Checklist

- [ ] All `# type: ignore` removed (or justified with explanation).
- [ ] All `# noqa` removed (or justified with explanation).
- [ ] Type checker passes without errors.
- [ ] Linter passes without violations.

---

## Rust

### Rules

- NO `#[allow(...)]` allowed (except in config exceptions)
- NO `unwrap()` allowed
- NO `expect()` allowed
- NO `panic!()` allowed
- Fix the root cause instead

### When to Use

- File has bypass comments
- File uses unwrap/expect/panic

### Workflow

**Step 1: Find Bypass Comments** — Read code and find bypass comments and unsafe calls.

**Step 2: Fix Root Cause** — Fix underlying type/error.

**Step 3: Remove Comment/Call** — Remove the bypass comment or unsafe call.

### Common Violations

| Violation               | Fix                                            |
| ----------------------- | ---------------------------------------------- |
| `#[allow(dead_code)]`   | Remove unused code or add to config exceptions |
| `#[allow(clippy::...)]` | Fix the clippy warning                         |
| `unwrap()`              | Use `?` or `match` for error handling          |
| `expect("msg")`         | Use `?` or `match` for error handling          |
| `panic!("msg")`         | Return `Result::Err` instead                   |

---

## TypeScript

Bypass forms: `@ts-ignore`, `@ts-expect-error`, `// eslint-disable`.

**Find:**

```bash
grep -rn "@ts-ignore" packages/*/src/
grep -rn "@ts-expect-error" packages/*/src/
grep -rn "eslint-disable" packages/*/src/
```

**Fix root cause:**

- `@ts-ignore` → Add proper type annotations.
- `@ts-expect-error` → Fix the type error or update the signature.
- `eslint-disable` → Fix the lint violation (unused imports, naming, etc.).

**Verify:**

```bash
npx tsc --noEmit
npx eslint packages/ --max-warnings 0
```

### Verification Checklist

- [ ] All `@ts-ignore` removed (or justified with explanation).
- [ ] All `@ts-expect-error` removed (or justified with explanation).
- [ ] All `// eslint-disable` removed (or justified with explanation).
- [ ] Type checker passes without errors.
- [ ] Linter passes without violations.

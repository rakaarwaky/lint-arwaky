# Lint Arwaky — TypeScript

The shared command reference, global options, MCP tools, workflow, exit codes and the full
AES101–AES506 fix table live in `../SKILL.md` and `rule-routing.md`. This file keeps only what
differs for TypeScript / JavaScript.

## Adapters & external tooling

`lint-arwaky-cli install` installs the external adapters; `lint-arwaky-cli adapters` lists the
active ones (ESLint, TSC, etc.); `lint-arwaky-cli external packages/` runs only those linters.
`get_config` accepts `"language": "javascript"`; scan examples use `"language": "typescript"`.
Covered extensions: `.ts`, `.tsx`, `.js`, `.jsx`.

```bash
# Type check without emitting
npx tsc --noEmit

# Lint (if configured)
npx eslint src/ --ext .ts,.tsx,.js,.jsx

# Tests (if configured)
npx vitest run    # or npm test
```

## Workspace layout & `--member`

TypeScript workspaces are laid out as `workspaces-bad/packages/<package>/src/…`, so path
examples use `packages/`, and `--member` targets a single **package** name.

```bash
lint-arwaky-cli scan workspaces-bad/packages                   # basic scan (text)
lint-arwaky-cli scan workspaces-bad/packages --format json
lint-arwaky-cli scan workspaces-bad/packages --member animator # single package member
lint-arwaky-cli scan workspaces-bad/packages --filter AES201   # by rule code (e.g. AES401)
lint-arwaky-cli fix packages/ --dry-run --filter AES101
lint-arwaky-cli ci packages/ --threshold 80 --format junit
lint-arwaky-cli orphan packages/ --format json
lint-arwaky-cli role packages/ --filter AES403
lint-arwaky-cli security packages/                             # ESLint security + dep CVEs
lint-arwaky-cli dependencies packages/
lint-arwaky-cli scan packages/ --format json > ~/.local/share/lint-arwaky/reports/scan_typescript.json
lint-arwaky-cli scan packages/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_typescript.sarif
```

## TypeScript-specific fix routing

- AES101 rename → after `git mv`, update the `index.ts` barrel exports (a file missing from the
  barrel is unreachable and reports as an orphan, AES501–506).
- AES201 forbidden import → remove the cross-layer `import { … } from "…"` and depend on the
  contract interface injected through DI (`create-contract`).
- AES303 mandatory definition → add the missing `class` / `interface` / `enum`.
- AES304 bypass → remove `@ts-ignore`, `@ts-expect-error`, `@ts-nocheck`, `eslint-disable`.
- AES403: every capability class MUST implement its protocol interface; AES405 (Agent Role):
  every agent class MUST implement its aggregate interface and use concrete types instead of
  `any`.
- AES404 (Utility Role): `utility_*` modules export pure functions only and may import
  `taxonomy` and nothing else.

## Role boundaries

| Layer        | Can Contain                  | Cannot Contain             |
| :----------- | :--------------------------- | :------------------------- |
| capabilities | Pure computation, validation | I/O, network, database     |
| utility      | Stateless exported functions | State, classes, non-taxonomy imports |
| agent        | Orchestration flow           | Computation, I/O, business |

## Pre-flight (TypeScript)

```bash
# Verify Node.js environment
node --version && npm --version

# Install dependencies if needed
npm install

# Check TypeScript compiles
npx tsc --noEmit
```

## Verify pipeline (TypeScript)

```bash
# 1. Re-scan — should show 0 violations
lint-arwaky-cli scan <target-path>

# 2. TypeScript compiles
npx tsc --noEmit

# 3. ESLint passes (if configured)
npx eslint src/ --ext .ts,.tsx,.js,.jsx

# 4. Tests pass (if configured)
npx vitest run  # or npm test
```

## Quick fix recipes (TypeScript)

### Rename file (AES101/102)

```bash
# Before: capabilities_scanner.ts (wrong — missing concern + suffix mismatch)
# After:  capabilities_file_scanner.ts
git mv src/capabilities_scanner.ts src/capabilities_file_scanner.ts
# Update index.ts barrel exports
```

### Remove unused import (AES203)

```bash
lint-arwaky-cli fix src/file.ts --filter AES203
# Or manually: remove the unused import line
```

### Fix bypass comments (AES304)

```bash
# Find all bypass patterns
grep -rn '@ts-ignore\|@ts-expect-error\|@ts-nocheck\|eslint-disable\|FIXME\|TODO' src/

# Fix each one:
# @ts-ignore → fix the type error properly
# @ts-expect-error → add proper type assertion or narrowing
# eslint-disable → fix the underlying lint rule
```

### Remove dead code (AES501–506)

```bash
# Find orphan files
lint-arwaky-cli orphan packages/ --format json

# If orphan is truly dead → delete it
# If orphan should be wired → add to container or import chain
```

### Fix layer role violation (AES401–406)

```bash
# Identify which role rule is violated
lint-arwaky-cli role packages/ --filter AES403

# Common fixes:
# - Move I/O code from capabilities to utility
# - Move business logic from surface to capabilities
# - Move orchestration from capabilities to agent
# - Use aggregate interfaces from contract instead of importing capabilities directly
```

## Verification Checklist (TypeScript)

- [ ] All layer imports follow AES201 rules.
- [ ] All capability classes implement their protocol interface (AES403).
- [ ] Utility modules are stateless exported functions importing `taxonomy` only (AES404).
- [ ] Agent classes implement their aggregate interface and use concrete types, not `any` (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `npx tsc --noEmit` and `npx eslint src/ --ext .ts,.tsx,.js,.jsx` clean.
- [ ] `lint-arwaky-cli scan packages/` reports 0 violations.

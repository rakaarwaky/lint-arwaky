# HOW TO USE LINT TYPESCRIPT

> **Purpose**: Scan, diagnose, and fix AES architecture violations in a TypeScript/JavaScript
> codebase with `lint-arwaky-cli`, then re-verify to zero findings.
>
> **Audience**: Agents and engineers running or remediating aes-lint-arwaky on TS/JS packages.
>
> **Scope**: TypeScript workspace paths (`packages/`, `workspaces-bad/packages/`), adapters
> (ESLint, TSC), TypeScript-specific fix routing and verify pipeline. Extensions: `.ts`,
> `.tsx`, `.js`, `.jsx`.
>
> **Location**: Run from the repo root (or the lint target root); reports default under
> `${XDG_DATA_HOME:-~/.local/share}/aes-lint-arwaky/reports/`.
>
> **Length**: One scan session = pre-flight → scan → triage → fix → re-scan to 0. Shared
> commands, MCP tools, exit codes, and AES101 naming live in
> `HOW-TO-USE-LINT-COMMANDS.md`; the full AES fix table lives in
> `HOW-TO-USE-LINT-ROUTING.md`.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Pre-flight first.** Violations on a non-compiling package are unreliable — `npx tsc
   --noEmit` must pass before you trust a scan.
2. **Shared command surface is not restated here.** Global flags, aliases (`lac`/`lat`/`lam`),
   MCP tools, exit codes, and AES101 naming live in `HOW-TO-USE-LINT-COMMANDS.md`;
   triage order and the fix table live in `HOW-TO-USE-LINT-ROUTING.md`. This file only holds TypeScript deltas.
3. **Path and `--member` follow the package layout.** Examples use `packages/`; `--member`
   targets a single **package** name. Covered extensions: `.ts`, `.tsx`, `.js`, `.jsx`.
   `get_config` accepts `"language": "javascript"`; scan examples may use `"typescript"`.
4. **Triage CRITICAL before HIGH/MEDIUM.** 🔴 AES201, AES205, AES304 → 🟡 AES101–102, AES202,
   AES301–303, AES401–403, AES406, AES505–506 → 🟢 AES203–204, AES305, AES404–405, AES501–504.
5. **Auto-fix only what is auto-fixable.** AES101 rename, AES203 unused import, AES304 bypass
   → `fix` (try `--dry-run` first). AES201 wrong dependency → manual; depend on the contract
   interface via DI (`aes-contract`).
6. **After AES101 rename, update the barrel.** `git mv` then re-export in `index.ts` — a file
   missing from the barrel is unreachable and reports as an orphan (AES501–506).
7. **Re-scan to 0 before declaring done.** Then run the TypeScript verify pipeline below;
   language checks are part of Verify, not a substitute for the AES scan.

---

## Template

Copy, fill, delete nothing.

### Adapters & external tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the
active ones (ESLint, TSC, …); `lint-arwaky-cli external packages/` runs only those linters.

```bash
npx tsc --noEmit
npx eslint src/ --ext .ts,.tsx,.js,.jsx
npx vitest run    # or npm test
```

### Scan session

```bash
# Pre-flight — package must typecheck
node --version && npm --version
npm install
npx tsc --noEmit

# Scan (text / json / by member / by rule)
lint-arwaky-cli scan workspaces-bad/packages
lint-arwaky-cli scan workspaces-bad/packages --format json
lint-arwaky-cli scan workspaces-bad/packages --member animator
lint-arwaky-cli scan workspaces-bad/packages --filter AES201
lint-arwaky-cli fix packages/ --dry-run --filter AES101
lint-arwaky-cli ci packages/ --threshold 80 --format junit
lint-arwaky-cli orphan packages/ --format json
lint-arwaky-cli role packages/ --filter AES403
lint-arwaky-cli security packages/
lint-arwaky-cli dependencies packages/

# Reports → XDG data dir
lint-arwaky-cli scan packages/ --format json \
  > ~/.local/share/aes-lint-arwaky/reports/scan_typescript.json
lint-arwaky-cli scan packages/ --format sarif \
  > ~/.local/share/aes-lint-arwaky/reports/scan_typescript.sarif
```

### TypeScript-specific fix routing

- **AES101 rename** → after `git mv`, update `index.ts` barrel exports (missing from barrel =
  orphan AES501–506).
- **AES201 forbidden import** → remove the cross-layer `import { … } from "…"`; depend on the
  contract interface injected through DI (`aes-contract`).
- **AES303** → add the missing `class` / `interface` / `enum`.
- **AES304** → remove `@ts-ignore`, `@ts-expect-error`, `@ts-nocheck`, `eslint-disable`
  (`fix-bypass`).
- **AES403** → every capability class MUST implement its protocol interface.
- **AES405** → every agent class MUST implement its aggregate interface; use concrete types
  instead of `any`.
- **AES404** → `utility_*` modules export pure functions only; import `taxonomy` and nothing
  else.

### Role boundaries

| Layer | Can contain | Cannot contain |
| ----- | ----------- | -------------- |
| capabilities | Pure computation, validation | I/O, network, database |
| utility | Stateless exported functions | State, classes, non-taxonomy imports |
| agent | Orchestration flow | Computation, I/O, business |

### Quick fix recipes

```bash
# AES101/102 rename
git mv src/capabilities_scanner.ts src/capabilities_file_scanner.ts
# then update index.ts barrel exports

# AES203 unused import
lint-arwaky-cli fix src/file.ts --filter AES203

# AES304 bypass
grep -rn '@ts-ignore\|@ts-expect-error\|@ts-nocheck\|eslint-disable\|FIXME\|TODO' src/
# fix root cause, delete suppression

# AES501–506 orphan
lint-arwaky-cli orphan packages/ --format json
# truly dead → delete; should be wired → container / barrel chain

# AES401–406 role
lint-arwaky-cli role packages/ --filter AES403
# I/O out of capabilities → utility; business out of surface → capabilities;
# orchestration out of capabilities → agent; surface uses aggregate interfaces only
```

---

## Section Contract

Every TypeScript lint session is required to carry the rows that apply. Each exists for one reason.

| Section | Why it belongs here |
| ------- | ------------------- |
| Pre-flight typecheck | A scan on a non-compiling package is noise, not signal. |
| Scan with path + member + format | Findings must be machine-triageable and package-scoped. |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via `HOW-TO-USE-LINT-ROUTING.md`. |
| Barrel update after rename | AES101 without `index.ts` re-export becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | `tsc` / eslint / tests still pass — AES clean ≠ package healthy. |

---

## Verify

```bash
# 1. Re-scan — must be 0 AES violations
lint-arwaky-cli scan <target-path>

# 2. Typecheck
npx tsc --noEmit

# 3. ESLint (if configured)
npx eslint src/ --ext .ts,.tsx,.js,.jsx

# 4. Tests (if configured)
npx vitest run  # or npm test
```

Exit codes (shared, `HOW-TO-USE-LINT-COMMANDS.md`): `0` pass · `1` violations · `2` config/parse error.

Checklist:

- [ ] Layer imports follow AES201.
- [ ] Capability classes implement their protocol interface (AES403).
- [ ] Utility modules are stateless exported functions importing `taxonomy` only (AES404).
- [ ] Agent classes implement their aggregate interface; no `any` (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `npx tsc --noEmit` and `npx eslint src/ --ext .ts,.tsx,.js,.jsx` clean.
- [ ] `lint-arwaky-cli scan packages/` reports 0 violations.

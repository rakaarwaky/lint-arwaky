---
name: aes-lint-arwaky
description: "AES architecture linter for Python Rust TS. Use when scanning, fixing AES101-AES506 violations."
metadata:
  tags:
    - python
    - rust
    - typescript
    - lint
    - aes
    - compliance
    - scanning
    - architecture
    - mcp
    - ci
    - fix
  related_skills:
    - aes-taxonomy
    - aes-utility
    - aes-contract
    - aes-capabilities
    - aes-agent
    - aes-surface
    - aes-root
    - aes-testing-suite
    - cleanup-consolidate
    - fix-bypass
  triggers:
    - lint arwaky
    - lint arwaky python
    - lint arwaky rust
    - lint arwaky typescript
    - lint code
    - scan project
    - scan python project
    - scan rust project
    - scan typescript project
    - verify aes compliance
    - check compliance
    - aes violations
    - fix architecture violations
    - fix violations
    - scan and fix
    - audit codebase
    - architecture health check
    - ci quality gate
---

# aes-lint-arwaky

> **Purpose**: Scan, diagnose, and fix AES101–AES506 violations, then re-verify to zero —
> using `lint-arwaky-cli` as the primary architecture gate (language compile/lint is fallback).
>
> **Audience**: Agents and engineers enforcing the AES 7-layer architecture in Python, Rust, or
> TypeScript codebases.
>
> **Scope**: Shared CLI/MCP surface, per-language HOW-TOs, AES fix routing, verification to 0.
> Not: writing new layers (use `aes-*`), test/bench naming (use `aes-testing-suite`).

## Language routing

| Language | HOW-TO | Use when |
| -------- | ------ | -------- |
| All (shared CLI, MCP, exit codes, AES101 naming) | [`references/HOW-TO-USE-LINT-COMMANDS.md`](references/HOW-TO-USE-LINT-COMMANDS.md) | Invoking scan/fix/ci, aliases, flags, MCP tools, layer matrix |
| Python | [`references/HOW-TO-USE-LINT-PYTHON.md`](references/HOW-TO-USE-LINT-PYTHON.md) | `modules/`, `py_compile` / Ruff / Mypy verify, `__init__.py` barrel |
| Rust | [`references/HOW-TO-USE-LINT-RUST.md`](references/HOW-TO-USE-LINT-RUST.md) | `crates/`, `cargo check` / clippy / nextest verify, `mod.rs` barrel |
| TypeScript | [`references/HOW-TO-USE-LINT-TYPESCRIPT.md`](references/HOW-TO-USE-LINT-TYPESCRIPT.md) | `packages/`, `tsc` / ESLint / vitest verify, `index.ts` barrel |
| Fix routing (every AES code) | [`references/HOW-TO-USE-LINT-ROUTING.md`](references/HOW-TO-USE-LINT-ROUTING.md) | Mapping a finding to its fix + owning `aes-*` skill |

Read the HOW-TO for the active language before fixing. Do not restate flags, subcommands, or
the fix table here — they live in the HOW-TOs.

## Layer chain

Bottom-up; a layer may only depend on layers below it:

`taxonomy_` → `utility_` → `contract_` → `capabilities_` → `agent_` → `surface_` → `root_`

Naming: `<layer>_<concern>_<role>.<ext>` (AES101). Legal suffixes per layer: AES102 matrix in
the routing HOW-TO. `tests/` / `benches/` are **not** AES layers (see `aes-testing-suite`).

## Invariants

| Invariant | Rule | Read |
| --------- | ---- | ---- |
| Primary gate | `lint-arwaky-cli scan` (AES101–506) → 0; language compile is fallback only | Commands + language HOW-TO |
| Direction | Upper layers import downward only; never upward (AES201 CRITICAL) | Routing HOW-TO |
| Role purity | Protocol = one method per feature; aggregate = many exports, one per feature (not dump-all) | Routing + `aes-contract` |
| Auto-fix set | Only AES101 / AES203 / AES304 via `fix` (`--dry-run` first); AES201/205 manual | Routing HOW-TO |
| Barrel after rename | AES101 rename → update `__init__.py` / `mod.rs` / `index.ts` or it becomes AES501–506 | Language HOW-TO |
| Done | Re-scan to **0** + language verify pass; exit code is the gate (`0`/`1`/`2`) | Commands HOW-TO |

## Diagnostic Tree

```text
Scan failed or findings non-zero?
├─ exit 2 → config/parse → lint_arwaky.config.yaml / path → fix config, re-run
├─ exit 1, findings present
│  ├─ CRITICAL 🔴 AES201 / AES205 / AES304 → structural; route via HOW-TO-USE-LINT-ROUTING.md (aes-contract / fix-bypass)
│  ├─ HIGH 🟡 AES101–102, AES202, AES301–303, AES401–403, AES406, AES505–506 → fix / aes-{layer}
│  ├─ MEDIUM/LOW 🟢 AES203–204, AES305, AES404–405, AES501–504 → fix or cleanup-consolidate
│  └─ After each AES101 rename → barrel update (language HOW-TO) or orphan reappears
└─ exit 0 but code unhealthy → language verify failed → HOW-TO-USE-LINT-<LANG>.md pipeline

```text

## Workflow

1. **Pre-flight** — codebase must build/parse in its own language (language HOW-TO). Violations
   on broken code are unreliable.
2. **Scan** — `lint-arwaky-cli scan <target-path> --format json` (shared flags:
   `HOW-TO-USE-LINT-COMMANDS.md`). Triage CRITICAL → HIGH → MEDIUM/LOW.
3. **Diagnose** — per finding: AES code, layer prefix, auto-fixable?, root cause. Full table:
   `references/HOW-TO-USE-LINT-ROUTING.md`.
4. **Fix** — auto: `lint-arwaky-cli fix <path>` (`--dry-run`, optional `--filter`). Manual:
   route via HOW-TO-USE-LINT-ROUTING.md to the owning `aes-*` skill.
5. **Verify** — re-scan to 0, then language verify pipeline (HOW-TO-USE-LINT-<LANG>.md).
6. **Commit** — only when asked: `fix: resolve <N> AES violations (<rules>)`.

## Verification

### Machine Checks

```bash
# Primary AES gate — must exit 0 with no findings
lint-arwaky-cli scan <target-path> --format json
echo "exit=$?"   # 0 pass · 1 violations · 2 config/parse error

# Family isolation while iterating
lint-arwaky-cli import <path> --filter AES201
lint-arwaky-cli role <path> --filter AES403
lint-arwaky-cli orphan <path> --format json

```text

### Human Checks

- [ ] Pre-flight build/import/typecheck passed **before** trusting the scan.
- [ ] CRITICAL (AES201/205/304) fixed before cosmetics.
- [ ] Every fix applied from the routing HOW-TO row (or auto-fix set), not a hand-rolled edit.
- [ ] Barrel/`mod.rs`/`index.ts` updated after every AES101 rename.
- [ ] Re-scan reports **0** AES violations (exit 0).
- [ ] Language verify pipeline (HOW-TO-USE-LINT-<LANG>.md) also green.

## Pre-flight Checklist

- [ ] Target path correct (`modules/` | `crates/` | `packages/`).
- [ ] Language build/parse clean (see language HOW-TO).
- [ ] Shared invocation chosen (`aa tool run lint …` or `lint-arwaky-cli` / `lac`).
- [ ] `--format` explicit if output is consumed by a pipeline or report file.

## Common Mistakes (Anti-Patterns)

| Mistake | Correct approach |
| ------- | ---------------- |
| Skipping pre-flight; trusting findings on broken code | Build/import first — violations on red builds are noise. |
| Auto-fixing AES201/205 with `fix` | Only AES101/203/304 auto-fix; AES201/205 manual via `aes-contract`. |
| AES101 rename without barrel/`mod.rs`/`index.ts` update | Re-export immediately — otherwise AES501–506 orphan. |
| Declaring done at "mostly clean" | Done = re-scan **0** + language verify; exit code is the gate. |
| Parsing stdout instead of exit code for CI | Gate on `0`/`1`/`2`; stdout is not the contract. |
| Re-guessing fix rows from memory | Consult `HOW-TO-USE-LINT-ROUTING.md` every time. |
| Treating `tests/`/`benches/` as AES layers | They follow `aes-testing-suite` naming, not AES101–102. |

## Related Skills

`aes-taxonomy` · `aes-utility` · `aes-contract` · `aes-capabilities` ·
`aes-agent` · `aes-surface` · `aes-root` · `aes-testing-suite` · `cleanup-consolidate` ·
`fix-bypass`

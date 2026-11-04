---
name: aes-lint-arwaky
description: "AES architecture linter for Python Rust TS. Use when scanning, fixing AES101-AES506 violations."
when-to-use: "scan a Python, Rust, or TypeScript codebase for AES violations; diagnose and fix AES101–AES506 findings; gate CI on architecture compliance; verify a fix to zero findings."
argument-hint: '<python | rust | typescript> <target-path> [--format json]'
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
---

# aes-lint-arwaky

Scan, diagnose, and fix AES101–AES506 violations, then re-verify to zero — `lint-arwaky-cli` is the primary architecture gate; language compile and linters are fallback only. The audience is the agent or engineer enforcing the AES 7-layer architecture in a Python, Rust, or TypeScript codebase; the **language** decides the verify pipeline, and the **AES code** decides the fix and its owning `aes-*` skill. Findings on broken code are unreliable — the target must build or parse first. Invariants read by more than one step live in Global Rules below.

## Global Rules

- Layer chain: `taxonomy_` → `utility_` → `contract_` → `capabilities_` → `agent_` → `surface_` → `root_`; a layer imports downward only (AES201 CRITICAL). Naming `<layer>_<concern>_<role>.<ext>` (AES101), legal suffixes per layer per the AES102 matrix in the routing HOW-TO.
- Scope: `tests/` / `benches/` are not AES layers — they follow `aes-testing-suite` naming, not AES101–102.
- Primary gate: `lint-arwaky-cli scan` exits `0` with zero findings; exit codes are the contract — `0` pass, `1` violations, `2` config/parse error. stdout is not the contract.
- Auto-fix set: only AES101 / AES203 / AES304 via `fix` (`--dry-run` first); AES201/205 are structural and manual via `aes-contract`.
- Barrel after rename: every AES101 rename updates `__init__.py` / `mod.rs` / `index.ts` in the same change, or orphans reappear as AES501–506.
- Triage order: CRITICAL 🔴 AES201/AES205 → HIGH 🟡 AES101–102, AES202, AES301–303, AES401–403, AES406, AES505–506 → MEDIUM/LOW 🟢 AES203–204, AES305, AES404–405, AES501–504; each finding routes through [references/lint-routing.md](references/lint-routing.md).
- Done: re-scan reports **0** + language verify pipeline green; commit `fix: resolve <N> AES violations (<rules>)` only when asked.
- One fact, one home: flags, subcommands, MCP tools, and the per-code fix table live in the HOW-TOs — read them, do not restate here. Cite the linter, not this file, when pointing at a rule. Each bullet states a fact once; references point at the bullet.

## Routing

| File | Purpose |
| --- | --- |
| [references/lint-commands.md](references/lint-commands.md) | Shared CLI, MCP, exit codes — scan/fix/ci invocation, aliases, flags, layer matrix |
| [references/lint-python.md](references/lint-python.md) | Python verify pipeline — `modules/`, `py_compile` / Ruff / Mypy, `__init__.py` barrel |
| [references/lint-rust.md](references/lint-rust.md) | Rust verify pipeline — `crates/`, `cargo check` / clippy / nextest, `mod.rs` barrel |
| [references/lint-typescript.md](references/lint-typescript.md) | TypeScript verify pipeline — `packages/`, `tsc` / ESLint / vitest, `index.ts` barrel |
| [references/lint-routing.md](references/lint-routing.md) | Fix routing — every AES code mapped to its fix and owning `aes-*` skill |
| Related skills | aes-taxonomy, aes-utility, aes-contract, aes-capabilities, aes-agent, aes-surface, aes-root, aes-testing-suite |

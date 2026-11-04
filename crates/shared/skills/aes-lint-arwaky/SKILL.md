---
name: aes-lint-arwaky
description: "AES architecture linter for Python Rust TS. Use when scanning, fixing AES101-AES506 violations."
when-to-use: "Use when scanning a Python, Rust, or TypeScript codebase for AES violations, diagnosing and fixing AES101–AES506 findings, or gating CI on architecture compliance."
argument-hint: "<target-path> [--format json] [--filter AESxxx]"
---

# aes-lint-arwaky

Scan, diagnose, and fix AES101–AES506 violations, then re-verify to zero — `lint-arwaky-cli` is the primary architecture gate; language compile and linters are fallback only. Findings on broken code are unreliable — the target must build or parse first.

## Global rules

- **Layer chain**: `taxonomy_` → `utility_` → `contract_` → `capabilities_` → `agent_` → `surface_` → `root_`; a layer imports downward only (AES201 CRITICAL). Naming `<layer>_<concern>_<role>.<ext>` (AES101); legal suffixes per layer per the AES102 matrix in the routing HOW-TO.
- **Scope**: `tests/` / `benches/` are not AES layers — they follow `aes-testing-suite` naming, not AES101–102.
- **Primary gate**: `lint-arwaky-cli scan` exits `0` with zero findings; exit codes are the contract — `0` pass, `1` violations, `2` config/parse error. stdout is not the contract.
- **Auto-fix set**: only AES101 / AES203 / AES304 via `fix` (`--dry-run` first); AES201/205 are structural and manual via `aes-contract`.
- **Barrel after rename**: every AES101 rename updates `__init__.py` / `mod.rs` / `index.ts` in the same change, or orphans reappear as AES501–506.
- **Triage order**: CRITICAL 🔴 AES201/AES205 → HIGH 🟡 AES101–102, AES202, AES301–303, AES401–403, AES406, AES505–506 → MEDIUM/LOW 🟢 AES203–204, AES305, AES404–405, AES501–504; each finding routes through the routing HOW-TO.
- **Done**: re-scan reports **0** + language verify pipeline green; commit `fix: resolve <N> AES violations (<rules>)` only when asked.
- **One fact, one home**: flags, subcommands, MCP tools, and the per-code fix table live in the HOW-TOs — read them, do not restate here. Cite the linter, not this file, when pointing at a rule.
- **Related**: aes-taxonomy, aes-utility, aes-contract, aes-capabilities, aes-agent, aes-surface, aes-root, aes-testing-suite — each owns its layer's fix.

## Routing

| Section | Reference | Purpose |
| ------- | --------- | ------- |
| Language routing | [references/language-routing.md](references/language-routing.md) | Which HOW-TO to load per language and fix routing |
| Shared CLI | [references/lint-commands.md](references/lint-commands.md) | scan/fix/ci invocation, aliases, flags, MCP tools, layer matrix |
| Python verify | [references/lint-python.md](references/lint-python.md) | `modules/`, `py_compile` / Ruff / Mypy, `__init__.py` barrel |
| Rust verify | [references/lint-rust.md](references/lint-rust.md) | `crates/`, `cargo check` / clippy / nextest, `mod.rs` barrel |
| TypeScript verify | [references/lint-typescript.md](references/lint-typescript.md) | `packages/`, `tsc` / ESLint / vitest, `index.ts` barrel |
| Fix routing | [references/lint-routing.md](references/lint-routing.md) | Every AES code mapped to its fix and owning `aes-*` skill |
| Diagnostic tree | [references/diagnostic-tree.md](references/diagnostic-tree.md) | Exit-code and severity routing for a failed scan |
| Workflow | [references/workflow.md](references/workflow.md) | Pre-flight → scan → diagnose → fix → verify → commit |
| Verification | [references/verification.md](references/verification.md) | Primary gate + family isolation + language fallback |
| Pre-flight | [references/pre-flight.md](references/pre-flight.md) | Target path, build state, invocation, format |
| Anti-patterns | [references/anti-patterns.md](references/anti-patterns.md) | Common scan/fix mistakes |

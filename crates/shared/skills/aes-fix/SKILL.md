---
name: aes-fix
description: "Reference fixes for every AES code rule (AES1xx naming, AES2xx import graph, AES3xx quality, AES4xx role/layer, AES5xx orphan, AES6xx/7xx structure/doc). Use when a scan reports violations and you need concrete how-to-fix steps."
when-to-use: "Run after `lint-arwaky-cli scan` reports violations. Each code rule maps to its own `references/AESxxx.md` file that states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command."
argument-hint: '<AES101 | AES102 | AES103 | AES2xx | AES3xx | AES4xx | AES5xx | AES6xx | AES7xx>'
---

# aes-fix

Purpose: turn a `lint-arwaky-cli scan` finding into a concrete edit. Each AES code rule family has one reference file under `references/`; read the file matching the rule code on your violation, apply the fix steps, then re-run the scan to confirm it clears.

Audience: Agents and engineers remediating lint violations in AES projects (Python, Rust, TypeScript).

Scope: all code-rule families the scanner enforces:
- AES1xx — naming (naming-rules crate)
- AES2xx — import graph (import-rules crate)
- AES3xx — quality (quality-rules crate)
- AES4xx — role/layer (role-rules crate)
- AES5xx — orphan (orphan-rules crate)
- AES6xx/7xx — structure/doc (structure-rules, doc-rules crates)

Global rules:
- One fact, one home: each `references/AESxxx.md` carries the how-to-fix for its rule family; do not restate the same fix in two files.
- Cite the checker name (e.g. `NamingConventionChecker` for AES101), not just the rule code, when pointing at implementation.
- Every reference ends with a Verify step naming the exact command.

## Routing

| File | Purpose |
| --- | --- |
| [references/AES101.md](references/AES101.md) | AES101 naming convention — stem shape (lowercase, underscore, min words) |
| [references/AES102.md](references/AES102.md) | AES102 suffix/prefix policy — layer suffix, forbidden suffixes, cross-layer |
| [references/AES103.md](references/AES103.md) | AES103 test/bench file-prefix discipline — flat prefix naming |
| [references/AES2xx.md](references/AES2xx.md) | AES2xx import graph validation — forbidden/mandatory/unused/dummy/cycle imports |
| [references/AES3xx.md](references/AES3xx.md) | AES3xx quality checks — file size, dead inheritance, bypass, mandatory definition, duplication |
| [references/AES4xx.md](references/AES4xx.md) | AES4xx role/layer enforcement — file must match its layer role |
| [references/AES5xx.md](references/AES5xx.md) | AES5xx orphan detection — unreachable/unwired files |
| [references/AES6xx.md](references/AES6xx.md) | AES6xx document-structure checks — README, AGENTS, ARCHITECTURE, FRD, PRD, DATA, CONTRIBUTING, BACKLOG, DESIGN, ROADMAP |
| [references/AES7xx.md](references/AES7xx.md) | AES7xx structure checks — shared purity, feature health, surface purity, test-suite coverage, member-root placement, PR-LA folder mapping |

## Section contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Every reference maps exactly one AES rule family | A violation's rule code resolves to one file, no ambiguity |
| Each file states what the rule checks, common causes, fix steps, before/after, verify | Gives the agent a complete remediation playbook |
| Fix steps are concrete (rename, move, add import, remove import, add definition, split file) | The agent can apply them without guessing |

## Verify

Re-run `lint-arwaky-cli scan .` and confirm the violation is gone. Then re-run the full scan to confirm no new violations were introduced by the edit.

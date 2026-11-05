---
name: aes-fix
description: "Reference fixes for every AES code rule (AES1xx naming, AES2xx import graph, AES3xx quality, AES4xx role/layer, AES5xx orphan, AES6xx document structure, AES7xx project structure). Use when a scan reports violations and you need concrete how-to-fix steps."
when-to-use: "Run after `lint-arwaky-cli scan` reports violations. Each code rule maps to its own `references/AESxxx.md` file that states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command."
argument-hint: '<AES101 | AES102 | AES103 | AES201 | AES202 | AES203 | AES204 | AES205 | AES301 | AES302 | AES303 | AES304 | AES305 | AES401 | AES402 | AES403 | AES404 | AES405 | AES406 | AES501 | AES502 | AES503 | AES504 | AES505 | AES506 | AES601 | AES602 | AES603 | AES604 | AES605 | AES606 | AES607 | AES608 | AES609 | AES610 | AES701 | AES702 | AES703 | AES704 | AES705 | AES706>'
---

# aes-fix

Purpose: turn a `lint-arwaky-cli scan` finding into a concrete edit. Each AES code rule has its own `references/AESxxx.md` file that states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command. Read the file matching the rule code on your violation, apply the fix steps, then re-run the scan to confirm it clears.

Audience: Agents and engineers remediating lint violations in AES projects (Python, Rust, TypeScript).

Scope: every code-rule family the scanner enforces, one reference file per rule code:
- AES1xx — naming (naming-rules crate): AES101, AES102, AES103
- AES2xx — import graph (import-rules crate): AES201–AES205
- AES3xx — quality (quality-rules crate): AES301–AES305
- AES4xx — role/layer (role-rules crate): AES401–AES406
- AES5xx — orphan (orphan-rules crate): AES501–AES506
- AES6xx — document structure (doc-rules crate): AES601–AES610
- AES7xx — project structure (structure-rules crate): AES701–AES706

Global rules:
- One fact, one home: each `references/AESxxx.md` carries the how-to-fix for its rule code; do not restate the same fix in two files.
- Cite the checker name (e.g. `NamingConventionChecker` for AES101), not just the rule code, when pointing at implementation.
- Every reference ends with a Verify step naming the exact command.

## Routing

| Rule | File | Purpose |
| --- | --- | --- |
| AES101 | references/AES101.md | Naming convention — stem shape (lowercase, underscore, min words) |
| AES102 | references/AES102.md | Suffix/prefix policy — layer suffix, forbidden suffixes, cross-layer |
| AES103 | references/AES103.md | Test/bench file-prefix discipline — flat layout, legal prefixes |
| AES201 | references/AES201.md | Import graph — forbidden layer import |
| AES202 | references/AES202.md | Import graph — mandatory layer import |
| AES203 | references/AES203.md | Import graph — unused import |
| AES204 | references/AES204.md | Import graph — dummy/stub import |
| AES205 | references/AES205.md | Import graph — circular dependency |
| AES301 | references/AES301.md | Quality — file too large |
| AES302 | references/AES302.md | Quality — file too short |
| AES303 | references/AES303.md | Quality — mandatory definition missing |
| AES304 | references/AES304.md | Quality — bypass annotation |
| AES305 | references/AES305.md | Quality — code duplication |
| AES401 | references/AES401.md | Role/layer — taxonomy primitive purity |
| AES402 | references/AES402.md | Role/layer — contract ABC-only |
| AES403 | references/AES403.md | Role/layer — capabilities protocol implementation |
| AES404 | references/AES404.md | Role/layer — utility purity |
| AES405 | references/AES405.md | Role/layer — agent orchestration only |
| AES406 | references/AES406.md | Role/layer — surface event mapping |
| AES501 | references/AES501.md | Orphan — taxonomy orphan |
| AES502 | references/AES502.md | Orphan — contract orphan |
| AES503 | references/AES503.md | Orphan — capabilities orphan |
| AES504 | references/AES504.md | Orphan — utility orphan |
| AES505 | references/AES505.md | Orphan — agent orphan |
| AES506 | references/AES506.md | Orphan — surface orphan |
| AES601 | references/AES601.md | Document structure — AGENTS.md conformance |
| AES602 | references/AES602.md | Document structure — README.md conformance |
| AES603 | references/AES603.md | Document structure — ARCHITECTURE.md conformance |
| AES604 | references/AES604.md | Document structure — CONTRIBUTING.md conformance |
| AES605 | references/AES605.md | Document structure — PRD.md conformance |
| AES606 | references/AES606.md | Document structure — ROADMAP.md conformance |
| AES607 | references/AES607.md | Document structure — BACKLOG.md conformance |
| AES608 | references/AES608.md | Document structure — DATA.md conformance |
| AES609 | references/AES609.md | Document structure — FRD.md conformance |
| AES610 | references/AES610.md | Document structure — DESIGN.md conformance |
| AES701 | references/AES701.md | Project structure — shared/layer folder purity |
| AES702 | references/AES702.md | Project structure — feature folder health |
| AES703 | references/AES703.md | Project structure — surface folder purity |
| AES704 | references/AES704.md | Project structure — test-suite coverage |
| AES705 | references/AES705.md | Project structure — member-root placement |
| AES706 | references/AES706.md | Project structure — PR-LA folder mapping | AES103 test/bench file-prefix discipline — flat prefix naming |
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

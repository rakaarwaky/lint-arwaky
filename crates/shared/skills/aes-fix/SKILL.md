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
| AES101 | [AES101.md](references/AES101.md) | Naming convention — stem shape |
| AES102 | [AES102.md](references/AES102.md) | Suffix/prefix policy |
| AES103 | [AES103.md](references/AES103.md) | Test/bench prefix |
| AES201 | [AES201.md](references/AES201.md) | Forbidden import |
| AES202 | [AES202.md](references/AES202.md) | Mandatory import |
| AES203 | [AES203.md](references/AES203.md) | Unused import |
| AES204 | [AES204.md](references/AES204.md) | Dummy import |
| AES205 | [AES205.md](references/AES205.md) | Circular import |
| AES301 | [AES301.md](references/AES301.md) | File too large |
| AES302 | [AES302.md](references/AES302.md) | File too short |
| AES303 | [AES303.md](references/AES303.md) | Definition missing |
| AES304 | [AES304.md](references/AES304.md) | Bypass annotation |
| AES305 | [AES305.md](references/AES305.md) | Code duplication |
| AES401 | [AES401.md](references/AES401.md) | Taxonomy purity |
| AES402 | [AES402.md](references/AES402.md) | Contract ABC-only |
| AES403 | [AES403.md](references/AES403.md) | Capability protocol |
| AES404 | [AES404.md](references/AES404.md) | Utility purity |
| AES405 | [AES405.md](references/AES405.md) | Agent orchestration |
| AES406 | [AES406.md](references/AES406.md) | Surface event mapping |
| AES501 | [AES501.md](references/AES501.md) | Taxonomy orphan |
| AES502 | [AES502.md](references/AES502.md) | Contract orphan |
| AES503 | [AES503.md](references/AES503.md) | Capability orphan |
| AES504 | [AES504.md](references/AES504.md) | Utility orphan |
| AES505 | [AES505.md](references/AES505.md) | Agent orphan |
| AES506 | [AES506.md](references/AES506.md) | Surface orphan |
| AES601 | [AES601.md](references/AES601.md) | AGENTS.md shape |
| AES602 | [AES602.md](references/AES602.md) | README.md shape |
| AES603 | [AES603.md](references/AES603.md) | ARCHITECTURE.md shape |
| AES604 | [AES604.md](references/AES604.md) | CONTRIBUTING.md shape |
| AES605 | [AES605.md](references/AES605.md) | PRD.md shape |
| AES606 | [AES606.md](references/AES606.md) | ROADMAP.md shape |
| AES607 | [AES607.md](references/AES607.md) | BACKLOG.md shape |
| AES608 | [AES608.md](references/AES608.md) | DATA.md shape |
| AES609 | [AES609.md](references/AES609.md) | FRD.md shape |
| AES610 | [AES610.md](references/AES610.md) | DESIGN.md shape |
| AES701 | [AES701.md](references/AES701.md) | Shared purity |
| AES702 | [AES702.md](references/AES702.md) | Feature health |
| AES703 | [AES703.md](references/AES703.md) | Surface purity |
| AES704 | [AES704.md](references/AES704.md) | Test-suite coverage |
| AES705 | [AES705.md](references/AES705.md) | Member-root placement |
| AES706 | [AES706.md](references/AES706.md) | PR-LA folder mapping |

## Section contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Every reference maps exactly one AES rule code | A violation's rule code resolves to one file, no ambiguity |
| Each file states what the rule checks, common causes, fix steps, before/after, verify | Gives the agent a complete remediation playbook |
| Fix steps are concrete (rename, move, add import, remove import, add definition, split file) | The agent can apply them without guessing |

## Verify

Re-run `lint-arwaky-cli scan .` and confirm the violation is gone. Then re-run the full scan to confirm no new violations were introduced by the edit.

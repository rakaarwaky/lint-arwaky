---
name: aes-fix
description: "AES rule fix reference (AES1xx–AES7xx)"
when-to-use: "Run after `lint-arwaky-cli scan` reports violations. Each code rule maps to its own `references/AESxxx.md` file that states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command."
argument-hint: '<AES101 | AES102 | AES103 | AES201 | AES202 | AES203 | AES204 | AES205 | AES301 | AES302 | AES303 | AES304 | AES305 | AES401 | AES402 | AES403 | AES404 | AES405 | AES406 | AES501 | AES502 | AES503 | AES504 | AES505 | AES506 | AES601 | AES602 | AES603 | AES604 | AES605 | AES606 | AES607 | AES608 | AES609 | AES610 | AES701 | AES702 | AES703 | AES704 | AES705 | AES706>'
---

# aes-fix

Turn a `lint-arwaky-cli scan` finding into a concrete edit. Agents and engineers remediating AES violations (Python, Rust, TypeScript) read the reference matching the rule code, apply its fix steps, then re-run the scan to confirm it clears. Scope: every code-rule family the scanner enforces, one reference per rule code.

## Global rules

- **One fact, one home.** Each `references/AESxxx.md` carries the how-to-fix for its rule code; do not restate the same fix in two files.
- **Cite the checker name**, not just the rule code, when pointing at implementation (e.g. `NamingConventionChecker` for AES101).
- **Every reference ends with a Verify step** naming the exact command.
- **Turn progress gates.** `lint-arwaky-cli report` records a per-target scan snapshot. Run it once at the start of the turn (baseline) and once at the end (delta against baseline). A positive delta means the fix run added problems; the turn is not done.
- **Section contract.** Every reference maps exactly one AES rule code and states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command. Fix steps must be concrete (rename, move, add import, remove import, add definition, split file) so the agent can apply them without guessing.

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
| AES605 | [AES605.md](references/AES605.md) | ROADMAP.md shape |
| AES606 | [AES606.md](references/AES606.md) | PRD.md shape |
| AES607 | [AES607.md](references/AES607.md) | DOC shape |
| AES608 | [AES608.md](references/AES608.md) | DATA shape |
| AES609 | [AES609.md](references/AES609.md) | BACKLOG shape |
| AES610 | [AES610.md](references/AES610.md) | DESIGN shape |
| AES701 | [AES701.md](references/AES701.md) | Workspace root |
| AES702 | [AES702.md](references/AES702.md) | Crates folder |
| AES703 | [AES703.md](references/AES703.md) | Crate structure |
| AES704 | [AES704.md](references/AES704.md) | Module structure |
| AES705 | [AES705.md](references/AES705.md) | Source folder |
| AES706 | [AES706.md](references/AES706.md) | PR-LA folder mapping |

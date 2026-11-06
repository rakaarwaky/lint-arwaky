---
name: aes-fix
description: "AES rule fix references for AES1xx-AES7xx: one step-N file per rule code with concrete fix steps and a verify command."
when-to-use: "Run after `lint-arwaky-cli scan` or `lint-arwaky-cli report` flags AES violations; fix and ship the fixes."
argument-hint: '<target-path> [rule-code]'
---

# aes-fix

## Rules

- **One fact, one home.** Each `references/step-N-AESxxx.md` carries the how-to-fix for its rule code; do not restate the same fix in two files.
- **Cite the checker name**, not just the rule code, when pointing at implementation (e.g. `NamingConventionChecker` for AES101).
- **Every reference ends with a Verify step** naming the exact command.
- **Turn progress gates.** Run `lint-arwaky-cli report` once at the start of the turn (baseline) and once at the end (delta). A positive delta means the run added problems; the turn is not done.
- **Orchestration and ship** are part of `step-0-setup.md`: group violations by owning member, spawn one subagent per member with its own violations and rule reference links, verify the delta, then open a PR and merge to main.
- **Completion.** The end-of-turn report shows zero or negative delta for every touched rule and the PR is merged to main.

## Routing


| Rule code | Reference | What it checks |
| --------- | --------- | -------------- |
| setup | [`step-0-setup.md`](references/step-0-setup.md) | Baseline, group by member, spawn subagents, verify delta, open PR, merge to main |
| AES101 | [`step-1-AES101.md`](references/step-1-AES101.md) | Naming: Variable |
| AES102 | [`step-2-AES102.md`](references/step-2-AES102.md) | Naming: Function / Method |
| AES103 | [`step-3-AES103.md`](references/step-3-AES103.md) | Naming: Class / Struct |
| AES201 | [`step-4-AES201.md`](references/step-4-AES201.md) | Orphan: Dead Function / Method |
| AES202 | [`step-5-AES202.md`](references/step-5-AES202.md) | Orphan: Dead Class / Struct |
| AES203 | [`step-6-AES203.md`](references/step-6-AES203.md) | Orphan: Dead File |
| AES204 | [`step-7-AES204.md`](references/step-7-AES204.md) | Orphan: Dead Module |
| AES205 | [`step-8-AES205.md`](references/step-8-AES205.md) | Orphan: Dead Package |
| AES301 | [`step-9-AES301.md`](references/step-9-AES301.md) | Role: Agent Layer |
| AES302 | [`step-10-AES302.md`](references/step-10-AES302.md) | Role: Capabilities Layer |
| AES303 | [`step-11-AES303.md`](references/step-11-AES303.md) | Role: Contract Layer |
| AES304 | [`step-12-AES304.md`](references/step-12-AES304.md) | Role: Root Layer |
| AES305 | [`step-13-AES305.md`](references/step-13-AES305.md) | Role: Surface Layer |
| AES401 | [`step-14-AES401.md`](references/step-14-AES401.md) | Import: Wildcard |
| AES402 | [`step-15-AES402.md`](references/step-15-AES402.md) | Import: Unused |
| AES403 | [`step-16-AES403.md`](references/step-16-AES403.md) | Import: Cycle |
| AES404 | [`step-17-AES404.md`](references/step-17-AES404.md) | Import: Cross-Layer Upward |
| AES405 | [`step-18-AES405.md`](references/step-18-AES405.md) | Import: Cross-Sibling |
| AES406 | [`step-19-AES406.md`](references/step-19-AES406.md) | Import: External |
| AES501 | [`step-20-AES501.md`](references/step-20-AES501.md) | Quality: Error Handling |
| AES502 | [`step-21-AES502.md`](references/step-21-AES502.md) | Quality: Test Coverage |
| AES503 | [`step-22-AES503.md`](references/step-22-AES503.md) | Quality: Type Safety |
| AES504 | [`step-23-AES504.md`](references/step-23-AES504.md) | Quality: Doc Coverage |
| AES505 | [`step-24-AES505.md`](references/step-24-AES505.md) | Quality: Logging |
| AES506 | [`step-25-AES506.md`](references/step-25-AES506.md) | Quality: Performance |
| AES601 | [`step-26-AES601.md`](references/step-26-AES601.md) | Structure: File Size |
| AES602 | [`step-27-AES602.md`](references/step-27-AES602.md) | Structure: Module Size |
| AES603 | [`step-28-AES603.md`](references/step-28-AES603.md) | Structure: Package Size |
| AES604 | [`step-29-AES604.md`](references/step-29-AES604.md) | Structure: Function Length |
| AES605 | [`step-30-AES605.md`](references/step-30-AES605.md) | Document Structure: PRD.md Conformance |
| AES606 | [`step-31-AES606.md`](references/step-31-AES606.md) | Document Structure: ROADMAP.md Conformance |
| AES607 | [`step-32-AES607.md`](references/step-32-AES607.md) | Document Structure: CONTRIBUTING.md Conformance |
| AES608 | [`step-33-AES608.md`](references/step-33-AES608.md) | Document Structure: ARCHITECTURE.md Conformance |
| AES609 | [`step-34-AES609.md`](references/step-34-AES609.md) | Document Structure: README.md Conformance |
| AES610 | [`step-35-AES610.md`](references/step-35-AES610.md) | Document Structure: AGENTS.md Conformance |
| AES701 | [`step-36-AES701.md`](references/step-36-AES701.md) | Project Structure: Root Layout |
| AES702 | [`step-37-AES702.md`](references/step-37-AES702.md) | Project Structure: Crates / Modules |
| AES703 | [`step-38-AES703.md`](references/step-38-AES703.md) | Project Structure: Scripts |
| AES704 | [`step-39-AES704.md`](references/step-39-AES704.md) | Project Structure: Tests |
| AES705 | [`step-40-AES705.md`](references/step-40-AES705.md) | Project Structure: Docs |
| AES706 | [`step-41-AES706.md`](references/step-41-AES706.md) | Project Structure: PR-LA Folder Mapping |

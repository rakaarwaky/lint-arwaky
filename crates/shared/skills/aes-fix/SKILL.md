---
name: aes-fix
description: "AES rule fix reference (AES1xx–AES7xx)"
when-to-use: "Run after `lint-arwaky-cli scan` reports violations. Each code rule maps to its own `references/step-N-AESxxx.md` file that states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command."
argument-hint: '<AES101 | AES102 | AES103 | AES201 | AES202 | AES203 | AES204 | AES205 | AES301 | AES302 | AES303 | AES304 | AES305 | AES401 | AES402 | AES403 | AES404 | AES405 | AES406 | AES501 | AES502 | AES503 | AES504 | AES505 | AES506 | AES601 | AES602 | AES603 | AES604 | AES605 | AES606 | AES607 | AES608 | AES609 | AES610 | AES701 | AES702 | AES703 | AES704 | AES705 | AES706>'
---

# aes-fix

Turn a `lint-arwaky-cli scan` finding into a concrete edit. Agents and engineers remediating AES violations (Python, Rust, TypeScript) read the reference matching the rule code, apply its fix steps, then re-run the scan to confirm it clears. Scope: every code-rule family the scanner enforces, one reference per rule code.

## Global rules

- **One fact, one home.** Each `references/step-N-AESxxx.md` carries the how-to-fix for its rule code; do not restate the same fix in two files.
- **Cite the checker name**, not just the rule code, when pointing at implementation (e.g. `NamingConventionChecker` for AES101).
- **Every reference ends with a Verify step** naming the exact command.
- **Turn progress gates.** `lint-arwaky-cli report` records a per-target scan snapshot. Run it once at the start of the turn (baseline) and once at the end (delta against baseline). A positive delta means the fix run added problems; the turn is not done.
- **Section contract.** Every reference maps exactly one AES rule code and states what the rule checks, common causes, concrete fix steps, a before/after example, and the verify command. Fix steps must be concrete (rename, move, add import, remove import, add definition, split file) so the agent can apply them without guessing.

## Routing

| Step | Reference | Purpose |
| ---- | --------- | ------- |
| Setup | [`step-0-setup.md`](references/step-0-setup.md) | Record baseline report before any fix |
| AES101 | [`step-1-AES101.md`](references/step-1-AES101.md) | Naming Convention (Stem Shape) |
| AES102 | [`step-2-AES102.md`](references/step-2-AES102.md) | Suffix/Prefix Policy |
| AES103 | [`step-3-AES103.md`](references/step-3-AES103.md) | Test/Bench File-Prefix Discipline |
| AES201 | [`step-4-AES201.md`](references/step-4-AES201.md) | Import Graph: Forbidden Layer Import |
| AES202 | [`step-5-AES202.md`](references/step-5-AES202.md) | Import Graph: Mandatory Layer Import |
| AES203 | [`step-6-AES203.md`](references/step-6-AES203.md) | Import Graph: Unused Import |
| AES204 | [`step-7-AES204.md`](references/step-7-AES204.md) | Import Graph: Dummy/Stub Import |
| AES205 | [`step-8-AES205.md`](references/step-8-AES205.md) | Import Graph: Circular Dependency |
| AES301 | [`step-9-AES301.md`](references/step-9-AES301.md) | Quality: File Too Large |
| AES302 | [`step-10-AES302.md`](references/step-10-AES302.md) | Quality: File Too Short |
| AES303 | [`step-11-AES303.md`](references/step-11-AES303.md) | Quality: Mandatory Definition Missing |
| AES304 | [`step-12-AES304.md`](references/step-12-AES304.md) | Quality: Bypass Annotation |
| AES305 | [`step-13-AES305.md`](references/step-13-AES305.md) | Quality: Code Duplication |
| AES401 | [`step-14-AES401.md`](references/step-14-AES401.md) | Role/Layer: Taxonomy Primitive Purity |
| AES402 | [`step-15-AES402.md`](references/step-15-AES402.md) | Role/Layer: Contract ABC-Only |
| AES403 | [`step-16-AES403.md`](references/step-16-AES403.md) | Role/Layer: Capabilities Protocol Implementation |
| AES404 | [`step-17-AES404.md`](references/step-17-AES404.md) | Role/Layer: Utility Purity |
| AES405 | [`step-18-AES405.md`](references/step-18-AES405.md) | Role/Layer: Agent Orchestration Only |
| AES406 | [`step-19-AES406.md`](references/step-19-AES406.md) | Role/Layer: Surface Event Mapping |
| AES501 | [`step-20-AES501.md`](references/step-20-AES501.md) | Orphan: Taxonomy Orphan |
| AES502 | [`step-21-AES502.md`](references/step-21-AES502.md) | Orphan: Contract Orphan |
| AES503 | [`step-22-AES503.md`](references/step-22-AES503.md) | Orphan: Capabilities Orphan |
| AES504 | [`step-23-AES504.md`](references/step-23-AES504.md) | Orphan: Utility Orphan |
| AES505 | [`step-24-AES505.md`](references/step-24-AES505.md) | Orphan: Agent Orphan |
| AES506 | [`step-25-AES506.md`](references/step-25-AES506.md) | Orphan: Surface Orphan |
| AES601 | [`step-26-AES601.md`](references/step-26-AES601.md) | Document Structure: AGENTS.md Conformance |
| AES602 | [`step-27-AES602.md`](references/step-27-AES602.md) | Document Structure: README.md Conformance |
| AES603 | [`step-28-AES603.md`](references/step-28-AES603.md) | Document Structure: ARCHITECTURE.md Conformance |
| AES604 | [`step-29-AES604.md`](references/step-29-AES604.md) | Document Structure: CONTRIBUTING.md Conformance |
| AES605 | [`step-30-AES605.md`](references/step-30-AES605.md) | Document Structure: PRD.md Conformance |
| AES606 | [`step-31-AES606.md`](references/step-31-AES606.md) | Document Structure: ROADMAP.md Conformance |
| AES607 | [`step-32-AES607.md`](references/step-32-AES607.md) | Document Structure: BACKLOG.md Conformance |
| AES608 | [`step-33-AES608.md`](references/step-33-AES608.md) | Document Structure: DATA.md Conformance |
| AES609 | [`step-34-AES609.md`](references/step-34-AES609.md) | Document Structure: FRD.md Conformance |
| AES610 | [`step-35-AES610.md`](references/step-35-AES610.md) | Document Structure: DESIGN.md Conformance |
| AES701 | [`step-36-AES701.md`](references/step-36-AES701.md) | Project Structure: Shared/Layer Folder Purity |
| AES702 | [`step-37-AES702.md`](references/step-37-AES702.md) | Project Structure: Feature Folder Health |
| AES703 | [`step-38-AES703.md`](references/step-38-AES703.md) | Project Structure: Surface Folder Purity |
| AES704 | [`step-39-AES704.md`](references/step-39-AES704.md) | Project Structure: Crates Folder Purity |
| AES705 | [`step-40-AES705.md`](references/step-40-AES705.md) | Project Structure: Config File Purity |
| AES706 | [`step-41-AES706.md`](references/step-41-AES706.md) | Project Structure: PR-LA Folder Mapping |

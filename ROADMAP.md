# ROADMAP — Lint Arwaky

## Current Condition

- Todo: None
- In Progress: None
- Blocked: FR-DISP, FR-CONF, FR-GITH, FR-TUIC, FR-MCPP — downgraded to At Risk pending closure of CRITICAL/WARNING findings opened by the BA/SA/UX/ARCH/BE/FE/PE audit (#522-#631); see Feature Roll-up and Risk Register below.
- Release Status: `Cargo.toml` and `CHANGELOG.md` version/changelog the workspace as 3.7.1, but no `v3.7.1` GitHub tag or Release has been published (confirmed via `gh release view v3.7.1` and `gh api .../tags`) — treat 3.7.1 as not-yet-released until the Risk Register item is closed. The `DEPLOY.md` Release Sign-off Checklist records per-domain evidence at commit `8d4342a`; it is a pipeline-health snapshot, not authorization to tag the ambiguous 3.7.1 candidate.
- UAT Status: Persona-level acceptance scenarios (Developer, DevOps, AI Agent) verified clean (see `TEST.md` Section 4).
- Document Gate: `lint-arwaky-cli docs .` reports 0 document invariant violations across all crates and root documents.
- Audit Tracking: Open BA/SA/UX/ARCH/BE/FE/PE findings are grouped under GitHub Milestone "Audit Remediation (v3.7.1)" (created 2026-10-01); per-issue assignment is partially manual — see Risk Register for the tooling gap.

## Feature Roll-up

One unified table: every feature + cross-cutting items, sorted by priority.

| ID | Priority | Feature | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|---|
| FR-SHAR | P0 | `crates/shared` | Done | On Track | — | — | 2026-09-30 |
| FR-CONF | P0 | `crates/config-system` | Done | At Risk | BE config-wipe CRITICAL finding | Re-verify after single-bad-field config-wipe finding closes | 2026-10-01 |
| FR-FILE | P0 | `crates/filesystem` | Done | On Track | — | — | 2026-09-30 |
| FR-NAMI | P0 | `crates/naming-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-IMPO | P0 | `crates/import-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-QUAL | P0 | `crates/quality-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-ROLE | P0 | `crates/role-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-ORPH | P0 | `crates/orphan-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-STRU | P0 | `crates/structure-rules` | Done | On Track | — | — | 2026-09-30 |
| FR-DISP | P0 | `crates/dispatcher` | Done | At Risk | #606, #611 | Re-verify after main-thread-freeze and clipboard-hang CRITICAL findings close | 2026-10-01 |
| FR-CLIC | P0 | `crates/cli-commands` | Done | On Track | — | — | 2026-09-30 |
| FR-EXTE | P1 | `crates/external-lint` | Done | On Track | — | — | 2026-09-30 |
| FR-AUTO | P1 | `crates/auto-fix` | Done | On Track | — | — | 2026-09-30 |
| FR-REPO | P1 | `crates/report-formatter` | Done | On Track | — | — | 2026-09-30 |
| FR-MCPP | P1 | `crates/mcp-server` | Done | At Risk | BE path-validation CRITICAL finding | Re-verify after shared-crate path-validation fix (shared with FR-TUIC, #608) | 2026-10-01 |
| FR-GITH | P1 | `crates/git-hooks` | Done | At Risk | BE hook-overwrite finding | Re-verify after install-hook backup/overwrite-guard fix | 2026-10-01 |
| FR-FILW | P1 | `crates/file-watch` | Done | On Track | — | — | 2026-09-30 |
| FR-PRJS | P1 | `crates/project-setup` | Done | On Track | — | — | 2026-09-30 |
| FR-MAINT | P1 | `crates/maintenance` | Done | On Track | — | — | 2026-09-30 |
| FR-TUIC | P2 | `crates/tui` | Done | At Risk | #552, #606, #611, #608 | Re-verify after UX confirm-gate and FE main-thread/path-validation CRITICAL findings close | 2026-10-01 |
| FR-SECR | P3 | Hardcoded-secret detection | Deferred | On Track | Dedicated scanners are more suitable | Keep out of AES rules; recommend gitleaks/TruffleHog in README | 2026-10-01 |

## Status Policy

- Status is **verified, not self-reported**. A row reaches `Done` only after someone re-ran the evidence command and read its output.
- A recorded verification names a **commit hash**, not "today".
- A PR that merges a fix updates **every** row that fix invalidates, in the same PR.
- Business outcome metrics (PRD.md Section 2) are audited per release cycle against recorded git commits.

## State Definitions

| State | Meaning |
|---|---|
| Idea | Captured, not yet examined; no spec exists for it. |
| Refinement | Being specced; a spec or product decision is needed first. |
| Ready | Specified enough to start; nobody has started it. |
| In Progress | Someone is in it now. |
| Blocked | Cannot proceed; name the blocker in `Actual Condition`. |
| In Review | PR open, awaiting review/CI. |
| QA | Implemented; awaiting a verification pass against evidence. |
| Done | Evidenced complete — cites the command/commit/PR. |
| Released | Done and shipped in a release. |
| Deferred | Intentionally out of current scope; reason in Next Action. |

## Health Definitions

| Health | Meaning |
|---|---|
| On Track | Nothing threatens the tier's scope. |
| At Risk | Open gaps could compromise the tier's gate. |
| Blocked | Work cannot proceed; name the blocker. |
| Ready for QA | No open rows; a verification sweep is outstanding. |
| Ready for Release | All evidence for the feature is recorded. |
| Released | Shipped. |

## Dependencies

Cross-crate and cross-cutting blocking relationships (dependency map), so cross-team sequencing does not require reconstructing it from individual issue bodies.

| Dependent | Depends On | Why |
|---|---|---|
| FR-TUIC (`tui`) | FR-DISP (`dispatcher`) | Needs a progress-callback hook on the scan path before the TUI progress bar can show real data instead of static chrome (#607) |
| FR-MCPP (`mcp-server`) | `crates/shared` path validation | Accepts a client-supplied path with the same boundary/traversal gap the TUI hits via `unwrap_or_default` (#608); a single shared-crate fix should resolve both consumers |
| FR-TUIC (`tui`) | `crates/shared` path validation | Same root-cause gap as the FR-MCPP row above (#608); verify both surfaces once `crates/shared` is fixed, rather than patching each independently |

External, out-of-repo dependents (not workspace crates, tracked separately since this repo does not own them):

| External Client | Depends On | Why |
|---|---|---|
| Claude Desktop | `crates/mcp-server` tool surface (5 tools, protocol `2024-11-05`) | Consumes `mcp-config --client claude`; no compatibility matrix exists yet (see `DEPLOY.md` External Client Compatibility) |
| VS Code MCP extension | `crates/mcp-server` tool surface | Consumes `mcp-config --client vscode`; same gap |
| Hermes Agent | `crates/mcp-server` tool surface | Consumes `mcp-config --client hermes`; same gap |

## Risk Register

- **Risk (re-verified 2026-09-30, commit `8d4342a` — closed):** Document invariant audit via `lint-arwaky-cli docs .` reports 0 violations across all `FRD.md` and workspace document chain files (doc-length overruns and doc-thin checks resolved in PR #500 / PR #520). Definition of Done document gate passes clean.
- **Risk (closed 2026-09-17):** Broken doc tests after documentation upgrade. **Resolved by** `bb715442`: all doc tests pass; self-lint clean.
- **Risk (historical gate waiver — closed 2026-09-29):** AES607 counting bug in PR #335 false-failed legitimate PRs (#344, #350, #351). A temporary branch-protection waiver was granted by the repository admin and resolved via PR #500 / PR #520 commit `8d4342a`. The Gate Waiver Process codified in `AGENTS.md` was written retroactively from this incident and governs future waivers only.
- **Risk (opened 2026-10-01, PE-1-01 / #617):** Findings from the BA/SA/UX/ARCH/BE/FE audit cycles (#522-#614), including 11+ CRITICAL-severity issues, sit against Feature Roll-up rows that were still marked Done/On Track a day after the engagement opened. **Mitigation:** FR-DISP, FR-CONF, FR-GITH, FR-TUIC, FR-MCPP are downgraded to At Risk above with cited issue numbers; the role that most recently audited a crate re-verifies and returns its row to On Track once the cited issues close (open question: whether a dedicated release manager should own this instead — tracked here until decided).
- **Risk (opened 2026-10-01, PE-3-01 / #623):** `Cargo.toml` (root + 22 member crates) and `CHANGELOG.md` both declare `3.7.1`, but `gh release view v3.7.1` returns not-found and `gh api repos/rakaarwaky/lint-arwaky/tags` lists no `v3.7.1` tag. **Determination:** this repository's history does not contain evidence either way of an out-of-band publish; treat 3.7.1 as **not released** pending the real release workflow. **Mitigation:** either run the tag → `release.yml` → GitHub Release pipeline for 3.7.1 once the Release Sign-off Checklist in `DEPLOY.md` is re-collected against the actual release commit, or retarget the version and correct `CHANGELOG.md`/`Cargo.toml`. Do not stack a 3.7.2/3.8.0 bump on this ambiguity in the meantime (the `Unreleased` section above `## 3.7.1` already has one pending change).
- **Risk (opened 2026-10-01, PE-1-02 / #618 and PE-1-03 / #619):** The issue-filing integration can create issues and (confirmed this session) create a GitHub Milestone, but every attempted label mutation (`gh issue edit --add-label`) and issue update — including milestone assignment via `gh issue edit --milestone` and `gh api -X PATCH .../issues/<n>` — returns `Resource not accessible by integration`. Only a small minority of the audit backlog carries a severity label, and the "Audit Remediation (v3.7.1)" milestone (created 2026-10-01, see `https://github.com/rakaarwaky/lint-arwaky/milestone/1`) could not be bulk-assigned to existing issues for the same reason. **Mitigation:** a repository admin grants the GitHub App/token `issues: write` scope sufficient for label and milestone mutation on *existing* issues, not just creation; until then, treat each issue body's `[ROLE][SEVERITY]` title prefix as authoritative over the Label chip, and assign the milestone by hand through the GitHub UI.
- **Risk (opened 2026-10-01, PE-5-03 / #631):** `.github/workflows/release.yml` generates and lists `lint-arwaky-cli.sha256` as a release asset, but the live `v3.7.0` Release has only the two binaries attached — no `.sha256` asset. **Determination:** this repository's single-commit history cannot establish whether the checksum step predates `v3.7.0` or was added afterward; `DEPLOY.md`'s existing "no checksum file published" caveat remains accurate for the current live release and must be re-verified (`gh release view <tag> --json assets`) the next time a tag is actually cut, rather than assumed fixed because the workflow source changed. **Mitigation:** `release.yml` now fails the `release` job if any expected asset (including the checksum) is missing from the downloaded artifact set, so a repeat of this gap cannot ship silently.

## Change Log

| Date | Change |
|---|---|
| 2026-09-17 | Initial master backlog created; 19 feature backlogs added |
| 2026-09-29 | Consolidated: single table with FR-ID format |
| 2026-09-29 | Streamlined columns: removed Feature/Spec/Backlog; added shared DATA.md + surface BACKLOG requirements |
| 2026-09-29 | Resolved AGENTS.md section contract conflict (#493 vs #494): adopted 12 strict H2 headings (restoring Related Documents) in PR #500 / PR #509 with AES605 test enforcement |
| 2026-09-30 | Addressed 15 Business Analyst audit issues (BA-1-01 through BA-5-03): added outcome metrics, evidence citations, UAT plan, deterministic bad-workspace floors, gate waiver policy, and release checklist |
| 2026-09-30 | Fixed `fix` exit-code aggregation contract mismatch: added `FailReason`/`SkipReason` Display impls, wired `FixResult.error` → dispatcher `FixReport.has_failed` → CLI/MCP exit 2, clarified PRD `fix` rule (post-fix remaining violations → exit 1, not exit 0), added `fix_result_exit_code_contract_failed_outcomes_trigger_error` regression test. Corrected historical gate-waiver attribution in Risk Register. |
| 2026-10-01 | Addressed 15 Product Engineering (PE) audit issues (PE-1-01 through PE-5-03, #617-#631): downgraded FR-DISP/FR-CONF/FR-GITH/FR-TUIC/FR-MCPP to At Risk with cited issues, added the Dependencies (dependency map) section, recorded the 3.7.1 tag/Release ambiguity and the label/milestone integration-permission gap in the Risk Register, created the "Audit Remediation (v3.7.1)" GitHub Milestone, added `DEPLOY.md` External Client Compatibility table and Rollback Record, reconciled the Release Sign-off Checklist with the Product/Engineering/QA/Documentation/Operations domains the Deploy checklist names, added `release.yml` asset-verification hardening, and added `TEST.md` Demo Walkthrough/Artifact Manifest/Role Sign-offs/Dependency Map subsections plus a demo-environment caveat citing #552 and #606. |
| 2026-10-01 | Shrunk `crates/shared/src/common` from 53 to 37 modules (#576): audited every module against the two-part criterion (used by `common` itself **and** by >1 non-shared crate), moved all 16 failing modules out of `common` — 15 to sibling shared packages (`auto_fix`, `role_rules`, `import_rules`, `filesystem`, `cli_commands`, `quality_rules`, `external_lint`, `orphan_rules`), plus dead `taxonomy_filesystem_error` deleted, and relocated their tests to feature-crate `tests/` targets. Editing one moved module now rebuilds 8 packages instead of 37. `utility_value_object_generator` deliberately stays in `common`: it defines `#[macro_export]` macros consumed by `common` itself, and every sibling shared package depends on `common`, so relocating it would create a dependency cycle. Evidence: test count unchanged (1995 = 1995), `cargo fmt --check`, `clippy -D warnings`, self-lint 0, `docs .` 0. |

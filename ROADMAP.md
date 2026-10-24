# ROADMAP — Lint Arwaky

## Current Condition

- Todo: None
- In Progress: None
- Blocked: FR-DISP, FR-CONF, FR-GITH, FR-TUIC, FR-MCPP — open findings from the BA/SA/UX/ARCH/BE/FE/PE audit (#522-#631) gate re-verification of these rows; see Feature Roll-up and Risk Register below.
- Release Status: RELEASED as v3.8.0. It supersedes 3.7.1 and 3.7.2, which were declared in `Cargo.toml` and `CHANGELOG.md` but never tagged (PE-3-01 / #623, now closed); the last published 3.7 release was `v3.7.0`, and everything since first shipped as 3.8.0. The `DEPLOY.md` Release Sign-off Checklist records per-domain evidence; it is a pipeline-health snapshot, not authorization to tag a future version.
- UAT Status: Persona-level acceptance scenarios (Developer, DevOps, AI Agent) verified clean (see `TEST.md` Section 4).
- Document Gate: `lint-arwaky-cli docs .` reports 0 document invariant violations across all crates and root documents.
- Audit Tracking: Open BA/SA/UX/ARCH/BE/FE/PE findings are grouped under GitHub Milestone "Audit Remediation (v3.7.1)" (created 2026-10-01); per-issue assignment is partially manual — see Risk Register for the tooling gap.

## Feature Roll-up

One unified table: every feature + cross-cutting items, sorted by priority.


| ID | Feature | Priority | State | Dependencies | Next Action | Updated |
|---|---|---:|---|---|---|---|
| FR-SHAR | `crates/shared` | P0 | Done | — | — | 2026-10-02 |
| FR-CONF | `crates/config-system` | P0 | Done | BE config-wipe CRITICAL finding | Re-verify after single-bad-field config-wipe finding closes | 2026-10-02 |
| FR-FILE | `crates/filesystem` | P0 | Done | — | — | 2026-10-02 |
| FR-NAMI | `crates/naming-rules` | P0 | Done | — | — | 2026-09-30 |
| FR-IMPO | `crates/import-rules` | P0 | Done | — | — | 2026-09-30 |
| FR-QUAL | `crates/quality-rules` | P0 | Done | — | — | 2026-09-30 |
| FR-ROLE | `crates/role-rules` | P0 | Done | — | — | 2026-10-02 |
| FR-ORPH | `crates/orphan-rules` | P0 | Done | — | — | 2026-10-02 |
| FR-STRU | `crates/structure-rules` | P0 | Done | — | — | 2026-09-30 |
| FR-DISP | `crates/dispatcher` | P0 | Done | #606, #611 | Re-verify after main-thread-freeze and clipboard-hang CRITICAL findings close | 2026-10-01 |
| FR-CLIC | `crates/cli-commands` | P0 | Done | — | — | 2026-09-30 |
| FR-EXTE | `crates/external-lint` | P1 | Done | — | — | 2026-10-02 |
| FR-AUTO | `crates/auto-fix` | P1 | Done | — | — | 2026-09-30 |
| FR-REPO | `crates/report-formatter` | P1 | Done | — | — | 2026-09-30 |
| FR-MCPP | `crates/mcp-server` | P1 | Done | BE path-validation CRITICAL finding | Re-verify after shared-crate path-validation fix (shared with FR-TUIC, #608) | 2026-10-01 |
| FR-GITH | `crates/git-hooks` | P1 | Done | BE hook-overwrite finding | Re-verify after install-hook backup/overwrite-guard fix | 2026-10-01 |
| FR-FILW | `crates/file-watch` | P1 | Done | — | — | 2026-09-30 |
| FR-PRJS | `crates/project-setup` | P1 | Done | — | — | 2026-09-30 |
| FR-MAINT | `crates/maintenance` | P1 | Done | — | — | 2026-09-30 |
| FR-TUIC | `crates/tui` | P2 | Done | #552, #606, #611, #608 | Re-verify after UX confirm-gate and FE main-thread/path-validation CRITICAL findings close | 2026-10-01 |
| FR-SECR | Hardcoded-secret detection | P3 | Done | Dedicated scanners are more suitable | — | 2026-10-23 |


## State Definitions


| State       | Meaning                                                     |
| ----------- | ----------------------------------------------------------- |
| Ready       | Specified enough to start; nobody has started it.           |
| In Progress | Someone is in it now.                                       |
| In Review   | PR open, awaiting review/CI.                                |
| Done        | Evidenced complete — cites the command/commit/PR.           |


## Risk Register

- **Risk (re-verified 2026-09-30, commit `8d4342a` — closed):** Document invariant audit via `lint-arwaky-cli docs .` reports 0 violations across all `FRD.md` and workspace document chain files (doc-length overruns and doc-thin checks resolved in PR #500 / PR #520). Definition of Done document gate passes clean.
- **Risk (closed 2026-09-17):** Broken doc tests after documentation upgrade. **Resolved by** `bb715442`: all doc tests pass; self-lint clean.
- **Risk (open, raised 2026-10-01, SA-4-02 / #547, SA-1-02 / #538):** A single MCP `execute_command` call has no timeout and no cancellation path, and the external-lint adapter phase has no overall budget — only per-adapter ceilings that sum to ~14 minutes worst case. The contract is now written down (`crates/mcp-server/DESIGN.md`, `crates/external-lint/FRD.md`, `PRD.md` NFR table) so agent clients can set their own timeout; bounding the adapter phase as one budget, or parallelizing independent-language adapters, remains a technical follow-up with no owner assigned.
- **Risk (open, raised 2026-10-01, SA-5-01 / #549):** The performance NFR now states two scopes with one number each (indexing: 1,000 files &lt; 2s, 10,000 &lt; 10s; full pipeline: 1,000 &lt; 5s, 10,000 &lt; 15s) in `PRD.md`, `README.md`, and `crates/filesystem/FRD.md`. Per the Status Policy these numbers are reconciled, not yet re-measured: a Criterion benchmark run over 1,000- and 10,000-file workspaces is owed as the evidence behind them.
- **Risk (historical gate waiver — closed 2026-09-29):** AES607 counting bug in PR #335 false-failed legitimate PRs (#344, #350, #351). A temporary branch-protection waiver was granted by the repository admin and resolved via PR #500 / PR #520 commit `8d4342a`. The Gate Waiver Process codified in `AGENTS.md` was written retroactively from this incident and governs future waivers only.
- **Risk (opened 2026-10-01, PE-1-01 / #617):** Findings from the BA/SA/UX/ARCH/BE/FE audit cycles (#522-#614), including 11+ CRITICAL-severity issues, sit against Feature Roll-up rows that were still marked Done a day after the engagement opened. **Mitigation:** FR-DISP, FR-CONF, FR-GITH, FR-TUIC, FR-MCPP are Blocked above with cited issue numbers; the role that most recently audited a crate re-verifies and returns its row to Done once the cited issues close (open question: whether a dedicated release manager should own this instead — tracked here until decided).
- **Risk (opened 2026-10-01, PE-3-01 / #623 — RESOLVED 2026-10-04, CLOSED 2026-10-05):** `Cargo.toml` (root) and `CHANGELOG.md` declared `3.7.1`, but no `v3.7.1` GitHub tag or Release exists; the 18 member crates were already at `3.7.2`. **Resolution:** the root version was aligned to `3.7.2` to match the members, the `Unreleased` changelog section was promoted to `## 3.7.2 (2026-10-04)`, and the `3.7.1` entry was marked "not tagged" with a note that its features shipped with 3.7.2. **Closure (2026-10-05):** rather than back-tagging the two versions that were declared but never published, `v3.8.0` supersedes both — everything since the last real tag (`v3.7.0`) first ships as 3.8.0. `SECURITY.md` no longer claims a maintained 3.7 line, and the CHANGELOG records the supersession so the tag history explains itself.
- **Risk (opened 2026-10-01, PE-1-02 / #618 and PE-1-03 / #619):** The issue-filing integration can create issues and (confirmed this session) create a GitHub Milestone, but every attempted label mutation (`gh issue edit --add-label`) and issue update — including milestone assignment via `gh issue edit --milestone` and `gh api -X PATCH .../issues/<n>` — returns `Resource not accessible by integration`. Only a small minority of the audit backlog carries a severity label, and the "Audit Remediation (v3.7.1)" milestone (created 2026-10-01, see `https://github.com/rakaarwaky/lint-arwaky/milestone/1`) could not be bulk-assigned to existing issues for the same reason. **Mitigation:** a repository admin grants the GitHub App/token `issues: write` scope sufficient for label and milestone mutation on *existing* issues, not just creation; until then, treat each issue body's `[ROLE][SEVERITY]` title prefix as authoritative over the Label chip, and assign the milestone by hand through the GitHub UI.
- **Risk (opened 2026-10-01, PE-5-03 / #631):** `.github/workflows/release.yml` generates and lists `lint-arwaky-cli.sha256` as a release asset, but the live `v3.7.0` Release has only the two binaries attached — no `.sha256` asset. **Determination:** this repository's single-commit history cannot establish whether the checksum step predates `v3.7.0` or was added afterward; `DEPLOY.md`'s existing "no checksum file published" caveat remains accurate for the current live release and must be re-verified (`gh release view <tag> --json assets`) the next time a tag is actually cut, rather than assumed fixed because the workflow source changed. **Mitigation:** `release.yml` now fails the `release` job if any expected asset (including the checksum) is missing from the downloaded artifact set, so a repeat of this gap cannot ship silently.
- **Risk (opened 2026-10-02):** AES704 requires every feature folder to carry all seven test types plus a benchmark, and the 20 dogfood tests added for it are generated skip-unless-present guards: each resolves the linter binary from `PATH` and returns early when it is absent, so on a machine with no installed binary they assert nothing. A dogfood test that always skips is a category satisfied in name only. **Mitigation:** the skip is deliberate (a dogfood test must never break CI, which is the skill's own contract), but the value lands only where the binary is installed — `lint-arwaky tool install lint-arwaky` before a local run. The seven `unit_`, `contract_`, `integration_`, `smoke_`, `e2e_`, and `acceptance_` files in each crate are real assertions against the crate's own seams and do not depend on the binary.


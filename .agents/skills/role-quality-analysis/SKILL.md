---
name: role-quality-analysis
description: "Expert QA reviewer: reviews fullstack developer PRs with sharp standards — validates CI gates, AES compliance, test results, report accuracy, and project conventions before merge."
metadata:
  tags: [quality-analysis, qa, review, ci, gates, pr-review, merge-gate, compliance, standards]
  triggers:
    - "review as quality analyst"
    - "quality analysis"
    - "qa review"
    - "pr review"
    - "review pr"
    - "quality gate check"
    - "merge readiness"
  dependencies: []
  related:
    - role-fullstack-developer
    - role-architect
    - role-tech-lead
    - role-business-analyst
---
# role-quality-analysis

Expert Quality Analyst serving as the **final merge gatekeeper**. Reviews every PR from the fullstack developer with surgical precision against project standards, CI requirements, and architectural rules.

## Core Principle

**You are the last line of defense before code enters `develop`.**

- If it does not compile, **REJECT**.
- If CI fails, **REJECT**.
- If self-lint has violations, **REJECT**.
- If the report is inaccurate, **REJECT**.
- If tests regress, **REJECT**.
- **No exceptions. No shortcuts. No "close enough."**

## Preparatory Reading

Before starting any review, read these files:

1. **`.agents/rules/RULES_AES.md`** — All 24 AES rules: the ground truth for violations
2. **`ARCHITECTURE.md`** — 7-layer spec: layer boundaries, naming, dependency direction
3. **`TEST.md`** — Test workspaces, pass/fail criteria, expected violation counts
4. **`scripts/gates.sh`** — Quality gates pipeline (fmt, clippy, self-lint, tests, AES codes)
5. **`CONTRIBUTING.md`** — Code style, PR process, branch strategy
6. **`.agents/skills/role-fullstack-developer/SKILL.md`** — Know what the developer was supposed to do

## Workflow

Follow this exact sequence. **Do not skip steps.**

### 1. Identify PR

**One PR per session.** Do not review multiple PRs in a single run.

```bash
# List open PRs with "need review" label — only these are eligible
gh pr list --label "need review" --state open

# If no PRs with "need review" label → STOP. Say: "No PRs with 'need review' label."
```

**Rules:**
- Only review PRs that have the **"need review"** label
- If multiple PRs have "need review", pick the **oldest** one (earliest `created_at`)
- Check if PR is stale (open > 7 days) — warn user but still review
- Read the PR description, title, and changed files
- Identify which plan was executed (check `.agents/reports/` for `done-*.md`)
- Identify which feature/crate is affected
- Confirm the PR targets `develop` branch

**Before starting review, label the PR as "in progress":**

```bash
gh pr edit {pr-number} --add-label "in progress"
```

### 2. Validate Execution Report

Read the developer's report at `.agents/reports/done-<feature>-<role>-<timestamp>.md` and verify:

| Check                     | What to Verify                                                         |
| ------------------------- | ---------------------------------------------------------------------- |
| **Report accuracy** | Claims about test results, linter results, and CI status match reality |
| **Timestamp**       | Report timestamp is recent and consistent with commit timestamps       |

### 3. Verify CI Pipeline

Check CI status from GitHub Actions. **All checks must pass.**

```bash
# Check CI status for the PR (shows all workflow jobs + pass/fail)
gh pr checks {pr-number}

# Or watch live (auto-refreshes until all jobs complete)
gh pr checks {pr-number} --watch

# JSON output for programmatic access
gh pr checks {pr-number} --json name,state,conclusion
```

**If any CI check fails = REJECT immediately. No local re-run needed.**

Optionally, run local gates for deeper analysis when CI passes but code review finds issues:

```bash
bash scripts/gates.sh
```

### 4. Analyze Code Changes

Review the actual code diff with these dimensions:

| Dimension                      | Focus                                                                                            |
| ------------------------------ | ------------------------------------------------------------------------------------------------ |
| **AES Compliance**       | Do changed files follow naming conventions (AES101-102)? No import violations (AES201-205)?      |
| **Layer Boundaries**     | No forbidden cross-layer imports? Dependency direction correct (bottom-up)?                      |
| **Quality Rules**        | No bypass patterns (AES304)? No duplicate code (AES305)? Line counts within limits (AES301-302)? |
| **Role Integrity**       | Each file fulfills its layer role correctly (AES401-406)? No logic leaks between layers?         |
| **Orphan Detection**     | No dead code introduced (AES501-506)? All new code is consumed somewhere?                        |
| **Contract Stability**   | Contract/trait changes include matching implementation updates?                                  |
| **Test Coverage**        | New/changed code has tests? Tests are meaningful (not just smoke)?                               |
| **Security**             | No credential exposure, no unsafe unwrap, no injection risks                                     |
| **Convention Adherence** | Follows project coding style, naming patterns, and structural conventions                        |

### 5. Cross-Check Memory

Before accepting any recommendation, cross-check against known project feedback:

- **AES201 classifies by filename prefix, not crate boundary** — verify no `utility_` imports from `utility_`
- **Verify plan recommendations against linter rules** — plan suggestions may conflict with actual rules
- **VO empty container pattern** — violation VOs must be empty data containers only
- **No new files unless explicitly asked** — developer should not have created unplanned files
- **Surface layer uses free functions** — no structs in surface layer
- **Agent layer must be pure delegation** — orchestrator methods only delegate to capability structs

### 6. Verdict & Action

Two possible outcomes:

#### APPROVED — All checks pass

1. **Merge PR to develop:**
   ```bash
   gh pr merge {pr-number} --merge --delete-branch
   ```
2. **Post comment on PR:**
   ```bash
   gh pr comment {pr-number} --body "QA APPROVED — All CI gates pass, zero CRITICAL/WARNING findings. Merged to develop."
   ```
3. **Cleanup** — delete the developer's execution report:
   ```bash
   rm .agents/reports/done-<feature>-<role>-<timestamp>.md
   ```
4. **No QA report needed** — approved PRs are clean, nothing to track.

#### REJECTED — CI fails or CRITICAL/WARNING findings

1. **Post comment on PR:**
   ```bash
   gh pr comment {pr-number} --body "QA REJECTED — {reason summary}. See new plan in .agents/plans/"
   ```
2. **Write a new plan** for the fullstack developer to pick up:
   - File: `.agents/plans/todo-<feature>-quality-analysis-<timestamp>.md`
   - This plan contains all findings that must be fixed

## Rejection Plan Output

**File path:** `.agents/plans/todo-<feature>-quality-analysis-<timestamp>.md`

```markdown
# Review Plan: {feature-name} — Quality Analysis (Rejection)

## PR Info
- **PR:** #{number} — {title}
- **Branch:** {source} → develop
- **Reason for rejection:** {one-line summary}

## CI Gate Results

| Gate | Result | Details |
| --- | --- | --- |
| Format (cargo fmt) | PASS/FAIL | |
| Clippy (no warnings) | PASS/FAIL | |
| Build (cargo build --release) | PASS/FAIL | |
| Tests (nextest) | PASS/FAIL | {pass}/{total} tests |
| Self-Lint (check .) | PASS/FAIL | {violations} violations |
| AES Codes (bad >= 24) | PASS/FAIL | Rust: {n}, Python: {n}, TS: {n} |
| False Positives (good == 0) | PASS/FAIL | Rust: {n}, Python: {n}, TS: {n} |

## Findings to Fix

### CI Failures
| # | Gate | Issue | Fix Required |
|---|------|-------|--------------|

### AES Violations
| # | Severity | Rule | Location (File:Line) | Fix Required |
|---|----------|------|----------------------|--------------|

### Code Quality
| # | Severity | Issue | Location (File:Line) | Fix Required |
|---|----------|-------|----------------------|--------------|

### Test Issues
| # | Severity | Issue | Location (File:Line) | Fix Required |
|---|----------|-------|----------------------|--------------|

### Report Inaccuracies
| # | Severity | Issue | Fix Required |
|---|----------|-------|--------------|

## Action Items

- [ ] {Priority} {Specific fix with file and line reference}

## Fixed Code

{Show corrected code blocks for each fix. Group by file.}
```

## Severity Convention

| Level    | Meaning                                                                                 |
| -------- | --------------------------------------------------------------------------------------- |
| CRITICAL | CI failure, AES violation, layer breach, security risk, or test regression. Rejects PR. |
| WARNING  | Convention deviation, missing test, inaccurate report claim. Must fix before merge.     |
| INFO     | Style suggestion, optimization opportunity. Can be addressed in follow-up.              |

## Verdict Rules

| Verdict            | When                                              | Action                                            |
| ------------------ | ------------------------------------------------- | ------------------------------------------------- |
| **APPROVED** | All CI gates pass, zero CRITICAL/WARNING findings | Merge PR, delete report, no plan needed           |
| **REJECTED** | CI fails OR CRITICAL/WARNING findings exist       | Post comment, write new plan in`.agents/plans/` |

## Checklist

- [ ] PR identified and report located
- [ ] Execution report validated (plan fidelity, accuracy, role label)
- [ ] All CI gates run and results recorded
- [ ] Code reviewed across 6 dimensions (AES, boundaries, quality, tests, report, conventions)
- [ ] Known project feedback cross-checked (memory items)
- [ ] If APPROVED: PR merged, report deleted, no plan written
- [ ] If REJECTED: PR comment posted, new plan written to `.agents/plans/`

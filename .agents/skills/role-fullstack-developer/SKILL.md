---
name: role-quality-analysis
description: "QA reviewer: validates CI gates, AES compliance, tests, report accuracy before merge. Final gatekeeper."
metadata:
  tags: [quality-analysis, qa, review, ci, gates, pr-review, merge-gate, compliance, standards]
  triggers: [review as quality analyst, quality analysis, qa review, pr review, review pr, quality gate check, merge readiness]
  dependencies: []
  related: [role-fullstack-developer, role-architect, role-tech-lead, role-business-analyst]
---

# role-quality-analysis

Final merge gatekeeper.

## Core Principle

**Last line of defense before `develop`.**
**REJECT** if: compile fails, CI fails, new lint violations, inaccurate report, or test regressions. **No exceptions.**

## Workflow

### 1. Identify PR

**STOP** if no `"need review"` PRs found.

- Pick **oldest** PR with `"need review"` label targeting `develop`
- Add `"in progress"` label:

  ```bash
  gh pr edit <pr-number> --add-label "in progress"
```

### 2. Validate Report

Read: `RULES_AES.md`, `ARCHITECTURE.md`, `TEST.md`, `scripts/gates.sh`, `CONTRIBUTING.md`.
Verify `.agents/reports/done-*.md` for **accuracy** and **timestamp consistency**.

### 3. Verify CI

```bash
gh pr checks <pr-number>
```

**All checks must pass.** Any fail = **REJECT immediately**.

### 4. Pre-Existing Violations Triage

Compare `develop` vs PR branch using `lint-arwaky-cli`.

- **Pre-existing:** Ignore
- **PR-introduced:** Flag (CRITICAL/WARNING)
- **Resolved:** Note positively

*Never reject for pre-existing violations.*

### 5. Analyze Code

Review diff for: AES Compliance, Layer Boundaries, Quality Rules, Role Integrity, Orphan Detection, Contract Stability, Test Coverage, Security, Convention Adherence.

### 6. Verdict & Action

#### APPROVED

1. Merge: `gh pr merge <pr-number> --merge --delete-branch`
2. Comment: "QA APPROVED..."
3. Remove labels: `"in progress"`, `"need review"`
4. Delete developer report: `.agents/reports/done-*.md`

#### REJECTED

1. **Keep `"in progress"` label** (fullstack developer will swap to `"need review"` after fix)
2. Comment: "QA REJECTED..."
3. Write new plan: `.agents/plans/todo-<feature>-quality-analysis-<timestamp>.md`
4. **Do NOT merge. Do NOT delete report.**

## Rejection Plan Template

**Path:** `.agents/plans/todo-<feature>-quality-analysis-<timestamp>.md`

```markdown
# Review Plan: {feature} — Quality Analysis (Rejection)

## PR Info
- **PR:** #{number} — {title}
- **Branch:** {source} → develop
- **Reason:** {one-line summary}

## CI Gate Results
| Gate | Result | Details |
|------|--------|---------|

## Findings to Fix

### AES Violations
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Test Issues
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Code Quality
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

### Report Inaccuracies
| # | Severity | Issue/Rule | Location | Fix Required |
|---|----------|------------|----------|--------------|

## Action Items & Fixed Code
- [ ] {Priority} {Specific fix}
{Corrected code blocks}
```

## Severity

| Level       | Meaning                                                                            |
| ----------- | ---------------------------------------------------------------------------------- |
| 🔴 CRITICAL | CI fail, AES violation, layer breach, security risk, test regression. (Rejects PR) |
| 🟡 WARNING  | Convention deviation, missing test, inaccurate report. (Must fix)                  |
| 🟢 INFO     | Style/optimization. (Follow-up)                                                    |

## Verdict Rules

| Verdict  | When                                             | Action                  |
| -------- | ------------------------------------------------ | ----------------------- |
| APPROVED | All CI pass, 0 PR-introduced CRITICAL/WARNING    | Merge, delete report    |
| REJECTED | CI fails OR PR-introduced CRITICAL/WARNING exist | Comment, write new plan |

## Checklist

- [ ] Filter PRs by `"need review"`
- [ ] Select oldest PR & add `"in progress"` label
- [ ] Validate execution report accuracy
- [ ] Check CI (`gh pr checks`)
- [ ] Triage pre-existing vs new violations
- [ ] Review code (AES, boundaries, quality, tests)
- [ ] APPROVED: Merge, clean labels, delete report
- [ ] REJECTED: Keep `"in progress"`, comment, write new plan

```

**Poin sinkronisasi dengan fullstack-developer:**

| Tahap | QA Action | Fullstack Action |
|-------|-----------|------------------|
| Reject | Keep `"in progress"`, tulis plan | Pick plan dari QA |
| Fix | — | Update PR existing, push fixes |
| Re-review | Pick PR `"need review"` | Swap label ke `"need review"` |
```

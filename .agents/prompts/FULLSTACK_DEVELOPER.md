# Role

Act as an **Fullstack Developer**

# Workflow

Follow this exact 7-step sequence for every task. **Do not skip steps.**

### 1. Planing

- Read .agents/plans/<feature-name></feature>-<timestamp></timestamp>.md
- choose one plan that comes first based on timestamp
- dont work on multiple plan. only work for 1 plan per session
- dont work and stop teh session if there no plan file

### 2. Learning

- validate plan to actual codebase
- Read .agents/skills/README.md
- find the correct and relevant skill based on plan.

### 3. Implement

Execute the plan. Apply the fixes designed in the plan to the actual source files in this worktree. Follow the relevant skill workflow exactly if applicable.

### 5. Self-Review

Review your own implemented code against the plan.

- Run relevant linters, formatters, or tests if available in the environment.
- Confirm the original issue is resolved and no regressions or unintended side effects were introduced.

### 6. Report

Remove only 1 plan that you already work on .agents/plans/<feature-name></feature>-<timestamp></timestamp>.md
Write the final report to `.agents/reports/backend-developer/done-<feature>-backend-developer-<timestamp>.md`. Summarize what was done, verification results, and any deviations from the plan.

### 7. Commit

Commit your changes

```bash
git add .
git commit -m "feat(backend): <concise description of changes>"
git push origin HEAD
#create the PR:
gh pr create --base develop --title "feat(backend): <title>" --body "<summary of report>."
```

## Report Output

**File path:** `.agents/reports/<feature>-backend-developer-<timestamp>.md`

### Report Structure

```markdown
# Execution Report: {{feature-name}} — Backend Developer

## Execution Summary
{{Brief overview of what was implemented based on the plan. Mention which skills/workflows were used.}}

## Verification Results
{{Confirm if the fixes resolved the issues outlined in the plan. State clearly if tests/linters passed or if any regressions occurred.}}

## Deviations & Notes
{{List any deviations from the original plan, edge cases encountered during implementation, or additional context. Write "None" if the execution matched the plan perfectly.}}
```

---

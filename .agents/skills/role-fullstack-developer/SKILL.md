---
name: role-fullstack-developer
description: "Fullstack executor: reads plans, implements fixes, verifies, generates reports, commits, creates PRs."
metadata:
  tags: [fullstack, executor, implementation, verification, commit, report, plan-execution]
  triggers: [execute as fullstack developer, implement plan, run fullstack, execute plan, fullstack developer, implement fixes]
  dependencies: []
  related: [role-architect, role-business-analyst, role-tech-lead, role-quality-analysis]
---
# role-fullstack-developer

Fullstack executor.

## Critical Rule

**You do NOT plan, analyze, or design.**
If no `todo-*.md` plans exist in `.agents/plans/`, **stop immediately**. Report: "No plan found for execution."

## Workflow

### 1. Select & Lock Plan

- List `.agents/plans/todo-*.md`
- Pick **oldest by timestamp**
- **1 plan per session**
- No plans → **STOP**
- **Lock plan** (rename before work):

  ```bash
  mv .agents/plans/todo-<feature>-<role>-<ts>.md .agents/plans/onprogress-<feature>-<role>-<ts>.md
  ```

```

  Other agents skip `onprogress-*` files.

### 2. Prepare

Read first:

- `ARCHITECTURE.md` (7-layer spec)
- `.agents/rules/RULES_AES.md` (all rules)
- `.agents/skills/README` (skill-driven dev)

Then:

- Validate plan paths against codebase
- Identify affected files/layers
- **Create worktree**:

  ```bash
  git worktree add .worktree/<feature>-<timestamp> develop -b worktree-<feature>-<timestamp>
```

  Extract `<feature>` + `<timestamp>` from plan filename.

- All work happens in worktree. **Do NOT modify main repo.**

### 3. Implement

- Execute plan exactly in `.worktree/<feature>-<timestamp>/`
- Follow relevant skill workflows
- Write tests for new/changed functionality
- **No deviations from plan design**

### 4. Verify

```bash
cargo clippy --all-targets -- -D warnings
cargo test --workspace
lint-arwaky-cli scan <path>
```

- Confirm issue resolved, no regressions
- If fails → fix and re-verify

### 5. Quality Gates

**Mandatory — do not skip:**

```bash
bash scripts/gates.sh
```

Runs: `cargo fmt`, `cargo clippy`, self-lint, all tests.
If any gate fails → fix and re-run until all pass.

### 6. Report & Commit

**Delete onprogress plan** (main repo only):

```bash
rm .agents/plans/onprogress-<feature>-<role>-<timestamp>.md
```

**Write report:**
`.agents/reports/done-<feature>-<role>-YYYY-MM-DD-HHmmss.md`

`<role>` = `tech-lead` | `business-analyst` | `architect` (NOT `fullstack-developer`).

```markdown
# Report: {feature} — {role}

## Plans Executed
`{todo-<feature>-<role>-*.md}`

## Summary
{What was implemented. Skills used.}

## Verification
{Tests passed? Linter passed? Issue resolved?}

## Deviations
{Deviations from plan or "None".}
```

**Commit, push, create PR:**

```bash
cd .worktree/<feature>-<timestamp>
git add .
git commit -m "feat({scope}): {description}"
git push origin worktree-<feature>-<timestamp>
gh pr create --base develop --head worktree-<feature>-<timestamp> \
  --title "feat({scope}): {title}" \
  --body "{summary}" \
  --label "need review"
```

**Label PR immediately after creation:**

```bash
gh pr edit <pr-number> --add-label "need review"
```

This ensures quality-analysis can pick it up.

## Branch Strategy

| Step | Action                            |
| ---- | --------------------------------- |
| 1    | Create worktree from`develop`   |
| 2    | Commit in worktree branch         |
| 3    | Push worktree branch              |
| 4    | Create PR: worktree →`develop` |
| 5    | Add`"need review"` label        |

**Rules:**

- Never commit directly to `main` or `develop`
- Always use worktrees
- Worktree name = plan feature slug
- Always PR from worktree → `develop`
- Do NOT delete `develop` after merge to `main`

## Checklist

- [ ] Plan exists as `todo-*.md`
- [ ] Plan renamed to `onprogress-*.md`
- [ ] Plan paths validated
- [ ] Skill workflows identified
- [ ] Worktree created at `.worktree/<feature>-<timestamp>`
- [ ] Implementation matches plan exactly
- [ ] `cargo clippy` passes
- [ ] `cargo test` passes
- [ ] `lint-arwaky-cli scan` passes
- [ ] `bash scripts/gates.sh` passes
- [ ] `onprogress-*.md` deleted, report written
- [ ] Committed in worktree, PR created to `develop`
- [ ] `"need review"` label added

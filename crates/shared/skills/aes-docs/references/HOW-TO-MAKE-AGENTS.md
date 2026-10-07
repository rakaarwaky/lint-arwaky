# HOW TO MAKE [AGENTS.md](http://AGENTS.md)

> **Purpose**: Tell the AI agent how to work safely, what commands to run,
> and what counts as done.
>
> **Audience**: The AI agent in the next session
>
> **Scope**: Operational constraints, commands, security, and git
> workflows.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines
>

---

## Rules

1. **Commands must match CI.** Every printed command must be
copy-pasteable and identical to the CI gate.
1. **No absolute personal paths.** Use `$HOME`, `${workspaceFolder}`,
or repo-relative paths (`absolute-path`).
1. **No secrets or credentials.** Never put tokens, passwords, or
keys in this file.
1. **Do not restate to other documents.** Use link instead.
1. **Mark optional sections clearly.** Do not force fake sections
like pipeline diagrams just to fill a template.
1. **Use** `main`, `.worktrees/`.
1. **Respect the length budget.** Target 50–500 lines.

---

## Workflow

1. **Determine context** — Agent config for single tool or multi-agent system.

1. **Write frontmatter** — name, description, persona, tools.
1. **Write AGENTS.md** — behavior rules, guardrails, response format.
1. **Verify** → validate with command 'lint-arwaky docs'

## Template

Copy, fill, delete nothing.

````markdown
---
trigger: always
description: "{Project} operational guide."
---
# {Project Name}

## User Context

- Preferences: {response style, e.g., concise, technical, direct}

### Autonomy

Operate as a YOLO-reversible agent. Allowed without asking: read code,
logs, status, diffs, test output, CI output; create or update isolated
branches and worktrees; install or sync dependencies from lockfiles;
run tests, linters, type checkers, builds, documentation checks; apply
reversible fixes to code, tests, docs, formatting, imports, comments;
commit, push to feature branches, open or update PRs when required
checks pass; poll PR status, CI status, merge queue, and failed check
logs; fix CI failures caused by the change, push follow-ups, repeat
validation; merge the PR after required checks pass.

### Scope and Precedence

- Local project guide controls project-specific conventions.
- Global guide (this `User Context` block, mirrored in
  {global agent guide path, e.g., ~/.qwen/QWEN.md}) controls
  autonomy, safety, and general behavior.
- Destructive or out-of-scope actions — force-push, `git reset
  --hard`, `rm -rf` on something outside the working tree, dropping
  a database, messaging a third party — still need explicit approval.

### Work Loop

1. Read the local project guide.
2. Identify the smallest correct change.
3. Work in an isolated worktree under {worktree-dir}/, never on
   {default branch}.
4. Run the project validation commands from `Commands`.
5. Fix failures caused by the change: read error output, reproduce
   locally when possible, apply the smallest fix, rerun the failed
   command, run related checks before pushing.
6. Commit with the convention defined by `Git Workflow`.
7. Push and open or update the PR.
8. Poll PR and CI status until terminal state. Treat pending checks
   as active work.
9. If CI fails: read failed output, fix, commit, push, poll again.
   Repeat until checks pass or a blocker requires human action.
10. If a merge conflict appears: resolve it in the isolated worktree
    when reversible and within scope.
11. If the repository uses a merge queue: poll the queue and fix
    rejections the same way as a CI failure.
12. When required checks pass, merge using the method from `Git
    Workflow`. If undefined and squash is allowed, use squash.
13. Verify the merged state from GitHub.

### Command Policy

Read-only inspection and PR/CI polling commands are always allowed:

```bash
git diff
git log --oneline -5
gh pr status
gh pr checks
gh pr checks --watch
gh pr view --json state,mergeStateStatus,mergeable,statusCheckRollup
gh run list --limit 5
```

## Session Start

Check state:

```bash
git status
git branch --show-current
git worktree list
git fetch origin {default branch}
```

Continue only from the correct {worktree-dir}/<branch-name>. If state
is missing or stale, ask before destructive changes. If the active
worktree is dirty, preserve relevant changes and stash unrelated
ones with `git stash push -u -m "agent-auto-stash"`.

## Runtime

- Language: {Language + pinned version}.
- Environment: {Where it lives and how it is created}.
- Artifacts: {Where they go. Never use /tmp for build output a
  reviewer must find}.

```bash
{version probe, e.g., python --version}
{env setup, e.g., export UV_PROJECT_ENVIRONMENT="$HOME/.local/share/<project>/venv" && uv sync}
```

## Quick Facts

INPUT  = {artifact + what it carries}
OUTPUT = {artifact + locked spec values, e.g., format, size, rate}

## Pipeline

{A} → {B} → {C} → {D}
{one word per stage}

## Git Workflow

Every change must use a worktree under
{worktree-dir}/<branch-name>. Do not work directly on {default
branch}. Do not switch branches (no `git checkout`/`git switch`);
use `git worktree add` so each branch lives in its own directory. PR
to {default branch} when done. Exceptions require explicit user
approval.

Branch prefixes: `<type>/`, ...

```bash
git worktree add -b {branch-name} {worktree-dir}/{branch-name} origin/{default branch}
cd {worktree-dir}/{branch-name}

# Run the checks under Commands, then:
git add .
git commit -m "{type}: {short description}"
git push -u origin {branch-name}

gh pr create --base {default branch} --head {branch-name} \
  --title "{type}: {short description}" \
  --body "$(cat <<'PRBODY'
What changed:
PRBODY
)"
```

After merge:

```bash
cd ../..
git worktree remove {worktree-dir}/{branch-name}
git branch -d {branch-name}
```

Merge strategy: {which prefixes squash, which rebase onto }.

Merge and verification commands:

```bash
gh pr merge --{merge method}
gh pr view --json state,mergedAt,mergeCommit
```

Do not bypass branch protection, required checks, review requirements,
or merge restrictions. Do not force-push to protected branches. Do
not switch branches between active tasks; use separate worktrees.

## Commands

```bash
# Tests
{whole-workspace test command}                      # what it covers
{single-package test command}                       # one unit
{single-file test command}                          # one file

# Lint / types / architecture —
{formatter/linter}                                  # matches ci.yml {job name}
{type checker, exact config-file flags}
{architecture scanner}
{dry-run variant, if the fixer is destructive}
```

## Guided Skills

Use `.agents/skills/` when a task matches a guided workflow. Read the
matching skill before generating structural code.

## Definition of Done

A change is done when:

- Work happened inside the correct {worktree-dir}/<branch-name>.
- Tests pass for touched units.
- Linter, type checker, and architecture scanner pass for touched paths.
- PR title and body follow conventions.
- A PR that merges a fix updates every invalidated backlog row in the
  same PR, then merges to {default branch}.
- Required CI checks pass and the PR is merged into {default branch}.
- Merged state is verified from GitHub (e.g. `gh pr view --json
  state,mergedAt,mergeCommit`).

A task is not complete at commit, local checks, or PR creation.

When a failure is unrelated to the current task, note it and continue
if the change remains safe. When a failure requires destructive
cleanup, secret access, production access, or out-of-scope behavior,
stop and ask.

## Writing Style

Use this section when editing prose, docs, PR descriptions, or release
notes. Do not apply it to code identifiers, commands, or config keys.

- Lead with the point. Use plain, active verbs. Keep concrete facts
  (names, dates, numbers, mechanisms).
- No invented claims, weasel attribution, throat-clearing openers,
  binary contrasts, or dramatic endings. Name the source or cut the claim.
- Use complete sentences, no emoji by default, code formatting for
  commands and variables. Vary rhythm only when it helps.

## Related Documents

- {link plus one line on what that document answers; repeat this bullet once per related document}.

---
````

## Section Contract

Every section is required unless marked optional. Each exists for one
reason. The H2 set is closed by AES605: global agent behavior
(Autonomy, Scope and Precedence, Work Loop, Command Policy) is
carried as H3 subsections under `User Context`, not as new H2s.

| Section             | Why it belongs here                                                                                          |
| ------------------- | ------------------------------------------------------------------------------------------------------------ |
| Frontmatter         | Loads the file unconditionally in rule-style harnesses. Skip when harness discovers plain `AGENTS.md`.       |
| User Context        | Sets response style, global autonomy rules, and the work loop without the user restating them. Skip when linters, not prose, enforce style here. |
| Session Start       | Fixes "agent edited the wrong worktree" at the door. Skip in single-file throwaway repo.                     |
| Runtime             | Version pin + env isolation. Skip when nothing here is version-pinned.                                       |
| Quick Facts         | One I/O contract with locked values. Skip when no fixed input/output artifact.                               |
| Pipeline            | Stage order and controller in one glance. Skip when not pipeline-shaped.                                     |
| Git Workflow        | Where work lands, which branch, how a PR is opened, and how the merge is verified. Skip when not a git-hosted repo. |
| Commands            | The gates, verbatim as CI runs them. Highest-value section. Never skip.                                      |
| Guided Skills       | Points at repo-shipped skills. Skip when repo ships no skills.                                               |
| Definition of Done  | Converts "finished" into a checkable list, ending at verified merged state, not at PR open. Never skip.        |
| Writing Style       | Prose rules for docs/PRs. Compress inline rather than split. Skip when prose is out of scope.                |
| Documentation Split | The spec/status boundary in one place. Skip when repo has no spec/backlog split.                             |
| Related Documents   | One line per doc: what it answers. Never skip.                                                               |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: agents-section-missing, ci-command-drift, absolute-path, secret-in-docs, dead-link, doc-length.

```

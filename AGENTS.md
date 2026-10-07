---
trigger: always
description: "Lint Arwaky operational guide. ARCHITECTURE.md wins on ambiguity."
---
# Lint Arwaky

## User Context

- Preferences: concise, technical, direct.

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
  `~/.qwen/QWEN.md`) controls
  autonomy, safety, and general behavior.
- Destructive or out-of-scope actions — force-push, `git reset
  --hard`, `rm -rf` on something outside the working tree, dropping
  a database, messaging a third party — still need explicit approval.

### Work Loop

1. Read the local project guide.
2. Identify the smallest correct change.
3. Work in an isolated worktree under `.worktrees/`, never on
   main.
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
git fetch origin main
```

Continue only from the correct .worktrees/feature-x. If state
is missing or stale, ask before destructive changes. If the active
worktree is dirty, preserve relevant changes and stash unrelated
ones with `git stash push -u -m "agent-auto-stash"`.

## Runtime

- Language: Rust (edition 2024, pinned via rust-toolchain.toml).
- Environment: cargo workspace under crates/, built on Linux/macOS with CARGO_INCREMENTAL=0.
- Artifacts: target/ and $HOME/.local/bin/ (install path). Never use /tmp for
  build output a reviewer must find.

```bash
rustc --version
CARGO_INCREMENTAL=0 cargo build --release
```

## Quick Facts

INPUT  = Rust workspace under crates/ with AES 7-layer naming
OUTPUT = 0 violations from lint-arwaky-cli check ., docs ., and scripts/gates.sh

## Pipeline

scan → check → gates → publish
lint, audit, verify, ship
`orchestrated by <controller>`

## Git Workflow

Every change must use a worktree under
.worktrees/feature-x. Do not work directly on main.
Do not switch branches (no `git checkout`/`git switch`); use `git worktree add`
so each branch lives in its own directory. PR to main when done.
Exceptions require explicit user approval.

Branch prefixes: `<type>/`, ...

```bash
git worktree add -b feature-x .worktrees/feature-x origin/main
cd .worktrees/feature-x

# Run the checks under Commands, then:
git add .
git commit -m "feature: summary"
git push -u origin feature-x

gh pr create --base main --head feature-x \
  --title "feature: summary" \
  --body "$(cat <<'PRBODY'
What changed:
PRBODY
)"
```

After merge:

```bash
cd ../..
git worktree remove .worktrees/feature-x
git branch -d feature-x
```

Merge strategy: squash all PR prefixes onto main; rebase long-lived branches onto main.

Merge and verification commands:

```bash
gh pr merge --squash
gh pr view --json state,mergedAt,mergeCommit
```

Do not bypass branch protection, required checks, review requirements,
or merge restrictions. Do not force-push to protected branches. Do
not switch branches between active tasks; use separate worktrees.

## Commands

```bash
# Tests
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2   # all tests (3x faster than cargo test)
CARGO_INCREMENTAL=0 cargo nextest run -p <crate>                      # one unit
CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>        # one file

# Lint / types / architecture —
CARGO_INCREMENTAL=0 cargo fmt --all -- --check                         # matches ci.yml "Format" job
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings          # matches ci.yml "Clippy" job
lint-arwaky-cli check .                                                # architecture scanner (self-lint)
bash scripts/gates.sh                                                  # dry-run variant of full CI gate

# Doc / structure audits
lint-arwaky-cli docs .                                               # document invariants (AES60x)
lint-arwaky-cli structure .                                           # folder layout (AES70x)

# Mergify — queue and stack
mergify config validate                    # validate .mergify.yml after edits
mergify config simulate <PR_URL>           # preview what rules would do
mergify queue status                       # see what's in the queue
mergify queue show <PR_NUMBER>             # inspect a specific PR in the queue
mergify stack new <branch-name>            # create a new stacked PR branch
mergify stack push                         # push + create/update stacked PRs
mergify stack list                         # show commit ↔ PR mapping
mergify events --pr <PR_NUMBER> --since 7d # audit what Mergify did to a PR
mergify freeze create --reason "..." --timezone UTC  # pause all merges
```

## Guided Skills

Use `.agents/skills/` when a task matches a guided workflow. Read the
matching skill before generating structural code.

## Definition of Done

A change is done when:

- Work happened inside the correct .worktrees/feature-x.
- Tests pass for touched units.
- Linter, type checker, and architecture scanner pass for touched paths.
- PR title and body follow conventions.
- A PR that merges a fix updates every invalidated backlog row in the
  same PR, then merges to main.
- Required CI checks pass and the PR is merged into main.
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

- [README.md](README.md): What the tool does, every command, and how to install it.
- [PRD.md](PRD.md): What the product does and why — feature tiers, exit codes, non-functional goals.
- [ARCHITECTURE.md](ARCHITECTURE.md): Layer contract for every crate and the rules that keep the boundaries.
- [RULES_AES.md](RULES_AES.md): The 32 rules, what each one forbids, and why.
- [ROADMAP.md](ROADMAP.md): What ships next, in order.
- [TEST.md](TEST.md): How to run the suite and what a green run proves.
- [CONTRIBUTING.md](CONTRIBUTING.md): Branch, commit, and PR conventions.
- [DEPLOY.md](DEPLOY.md): Release path and the checks it gates on.
- [SECURITY.md](SECURITY.md): Reporting a vulnerability and what counts as one.

---

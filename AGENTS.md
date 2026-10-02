# AGENTS.md — Lint Arwaky

Read before making any changes to the codebase.
Make sure to read [TEST.md](TEST.md) for pass/fail criteria before committing any changes.

---

## Project Overview

**Lint Arwaky** is an architecture linter for Rust, Python, and TypeScript that enforces the [Agentic Engineering System (AES)](ARCHITECTURE.md) — a 7-layer architecture with 34 rules across 7 groups (naming, import, quality, role, orphan, doc, structure). The project itself is written in Rust and is self-auditing (it passes its own lint rules).

---

## Precedence

1. Safety rules in this file.
2. Explicit user approval in the current session.
3. Spec documents: PRD.md, ARCHITECTURE.md, crate FRD.md files, shared-folder DATA.md, DESIGN.md.
4. `AGENTS.md` defaults.

If two documents conflict, follow the higher-ranked source. If still unclear, ask.

## Build & dev

```bash
CARGO_INCREMENTAL=0 cargo build --release           # full build
CARGO_INCREMENTAL=0 cargo check -p <crate>          # type-check only
CARGO_INCREMENTAL=0 cargo clippy -p <crate>         # lint only
cargo nextest run -p <crate>                         # tests (3× faster than cargo test)
```

### Format & lint

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

### Self-lint

Binary path: `$HOME/.cargo/bin/lint-arwaky-cli`

```bash
lint-arwaky-cli scan .   # runs ALL 7 code linters on own codebase
```

### Scan test projects

Test workspaces contain intentional violations (`workspaces-bad/`) and clean files (`workspaces-good/`). Language is auto-detected from file extensions — no flag needed.

```bash
# Bad workspaces (should find violations)
lint-arwaky-cli scan workspaces-bad/crates
lint-arwaky-cli scan workspaces-bad/modules
lint-arwaky-cli scan workspaces-bad/packages

# Good workspaces (should find 0 violations)
lint-arwaky-cli scan workspaces-good/crates
lint-arwaky-cli scan workspaces-good/modules
lint-arwaky-cli scan workspaces-good/packages
```

### MCP server & TUI

```bash
lint-arwaky-mcp   # MCP server (stdin/stdout JSON-RPC 2.0)
lint-arwaky-tui   # TUI file browser
```

## Security

See [SECURITY.md](SECURITY.md) for full details.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for full details.

## Commands

```bash
# Tests (matches CI "Tests" job)
cargo nextest run --workspace --lib --tests -j 2

# Lint / types (matches CI "Clippy" job)
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings

# Format (matches CI "Format" job)
cargo fmt --all -- --check

# Self-lint (matches CI "Self-Lint" job)
lint-arwaky-cli check .

# Build (matches CI "Build" job)
CARGO_INCREMENTAL=0 cargo build --release
```

## Git Workflow

`main` is protected by the "Protect main - quality gates" ruleset: 6 required status checks (Format, Clippy, Build, Tests, Self-Lint, Codacy) must pass before any commit lands. **Direct pushes to `main` are rejected** — always go through a PR.

**Mergify merge queue** (configured in `.mergify.yml`): every non-draft, conflict-free PR targeting `main` is auto-queued. The queue updates the PR branch with the latest `main` (merge commit), runs all 6 quality-gate checks on the integration commit, and squashes to `main` once everything passes. You never need to comment `@mergifyio queue` or click merge manually.

Mergify CLI is installed (`mergify --version`). Useful commands:

```bash
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

Every change:

```bash
git worktree add -b <branch-name> .worktree/<branch-name> origin/main
cd .worktree/<branch-name>

# Run the Commands above, then:
git add .
git commit -m "<type>: <short description>"
git push -u origin <branch-name>

gh pr create --base main --head <branch-name> \
  --title "<type>: <short description>"
# Mergify auto-queues it — no further action needed.
```

After merge (squash, `--delete-branch`):

```bash
cd <repo-root>
git worktree remove .worktree/<branch-name>
git branch -d <branch-name>
```

### Gate Waiver Process

If a required CI check is proven defective (e.g. a shipped rule false-fails legitimate PRs, as occurred with AES607 in PR #335):

1. **Approver role**: A repository admin (`@raka`) may grant a temporary, time-boxed override via branch protection settings.
2. **Mandatory tracking**: The override requires a linked GitHub issue documenting the defect root cause, reproduction steps, and an assigned fix owner with a target ETA.
3. **Time-box limit**: The waiver expires automatically after a maximum of 5 business days.
4. **Audit trail**: Every active waiver must be logged in `ROADMAP.md`'s Risk Register until resolved and verified by a subsequent clean run.

## Branch Management

Allowed branch naming: `main`, `develop`

When merging a PR to develop:

- **use `--delete-branch`** — for feature/fix branches after merge
- **do NOT delete `develop`** branch after merge to `main`

**Worktree policy (important):**

- When working on a feature/fix branch, **use a git worktree** under `.worktree/` (e.g. `<repo-root>/.worktree/feature-name`) instead of switching branches in the current checkout with `git checkout`.

## Skills

`crates/shared/skills/` is the source of truth for the distributed skill pack; each is one directory with a `SKILL.md` and an optional `references/HOW-TO-*.md` set of per-language playbooks. Layer creation (`aes-taxonomy`, `aes-contract`, `aes-utility`, `aes-capabilities`, `aes-agent`, `aes-surface`, `aes-root`), maintenance (`aes-lint-arwaky`, `aes-migration`), and documentation (`aes-docs`, `aes-testing-suite`) skills are triggered by keyword.

`lint-arwaky init` installs every `SKILL.md` plus only the `references/` files matching the target's detected languages. `crates/shared/src/project_setup/taxonomy_project_setup_constant.rs` is generated — run `python3 tools/regenerate_skills.py` after adding, removing, or renaming a skill file. The `catalog_matches_the_skills_directory` test fails if a skill file exists without being embedded, so a missed regeneration cannot reach `main`.

**Role pipeline:** `Architect` → `Business Analyst` → `Tech Lead` → `Fullstack Developer` (review then execute). Plan files go to `.agents/plans/`.

**Pipeline gate:**

- A `severity-critical` issue raised by `Architect` or `Business Analyst` is a hard block: `Tech Lead` cannot begin implementation until the issue has a recorded triage state (`accepted`, `deferred`, or `rejected` with rationale) in the plan file.
- `severity-warning` and `severity-info` findings are advisory only and do not block downstream role progression.
- Triage authority belongs to the Tech Lead / Repository Maintainer (`@raka`). In case of competing specifications, the Tech Lead adjudicates before worktrees or PRs are created.

### Mergify Stacks (dependent PRs)

When a feature spans multiple commits that each need their own PR, use `mergify stack`:

```bash
git stash -u                     # always stash before stack ops
mergify stack new feat/my-stack  # creates a new branch from main
# ... make commit(s), then:
mergify stack push               # creates/updates stacked PRs automatically
mergify stack list               # show the current stack
mergify stack reorder A B C      # reorder commits in the stack
mergify stack sync               # rebase onto latest main
mergify stack note -m "why"      # attach an explanation before amending
git commit --amend
mergify stack push               # pushes the amended stack
```

Stacks manage their own `Depends-On:` headers and GitHub-native stacking.
Never use `git rebase -i` on a stack branch — use `mergify stack {edit,fixup,squash,reorder,move,drop}` instead. See the [Mergify stack documentation](https://docs.mergify.com/stacks/) for the full reference.

## Quality gates

```bash
bash scripts/gates.sh                       # fmt + clippy + self-lint + tests
cargo nextest run --workspace --lib --tests # all tests, 3× faster
```

## Definition of Done

A change is done when all of the following hold:

- Work happened inside the correct `.worktree/<branch-name>`, never directly on `main`.
- `bash scripts/gates.sh` passes: format, clippy, self-lint, and the full test suite.
- `lint-arwaky-cli check .` reports 0 violations.
- `lint-arwaky-cli scan workspaces-good/crates` still reports 0 violations.
- `lint-arwaky-cli docs .` reports 0 document invariant violations.
- Pass/fail criteria in [TEST.md](TEST.md) hold for the touched paths.
- A new AES rule adds a trigger file to all 3 test workspaces and a row in the TEST.md per-rule matrix.
- A PR that fixes behavior updates the invalidated ROADMAP.md backlog rows in the same change.

## Related Documents

- [PRD.md](PRD.md): What the product does and why — feature tiers, exit codes, non-functional goals.
- [ARCHITECTURE.md](ARCHITECTURE.md): The full 7-layer AES specification and naming rules.
- [ROADMAP.md](ROADMAP.md): Real condition — what is done, what is in flight, with re-runnable evidence.
- [TEST.md](TEST.md): Test workspaces and pass/fail criteria.
- [CONTRIBUTING.md](CONTRIBUTING.md): Setup, code style, and PR process.
- [DEPLOY.md](DEPLOY.md): MCP client setup and release deployment.

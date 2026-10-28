# CONTRIBUTING — Calculator Workspace

## Contribution Overview

Contributors add feature modules, bug fixes, and docs-only updates. Every change must clear the format, clippy, self-lint, and docs gates before the PR merges.

## Contribution Guidelines

1. Respect layer boundaries and the AES naming convention.
2. Persistent data belongs under `target/` or `$HOME/.local/bin/`; never `/tmp`.
3. Update `Cargo.toml`, `pyproject.toml`, or `package.json` with every add or delete.
4. Do not leak absolute paths, secrets, or machine-specific values into the diff.

## Workflow

| Step | Action | Detail |
|------|--------|--------|
| 1 | Fork / Branch | `<branch-naming-convention>` |
| 2 | Implement | Follow FRD spec, write tests |
| 3 | Self-review | Check against acceptance criteria in PRD |
| 4 | Submit PR | Link to BACKLOG ID, describe what changed |
| 5 | Review | <reviewer / approval criteria> |
| 6 | Merge | <merge strategy: squash / rebase / merge> |

## Commit Convention

| Type | Usage | Example |
|------|-------|---------|
| `feat` | New feature | `feat(auth): add login endpoint` |
| `fix` | Bug fix | `fix(auth): handle expired token` |
| `docs` | Documentation only | `docs(frd): update FR-AUTH-001` |
| `test` | Adding/updating tests | `test(auth): add edge case for timeout` |
| `refactor` | Code refactor, no behaviour change | `refactor(auth): extract middleware` |
| `chore` | Maintenance, deps, config | `chore: bump dependencies` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](PRD.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |

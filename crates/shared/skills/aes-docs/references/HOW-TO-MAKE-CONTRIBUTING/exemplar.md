# CONTRIBUTING Exemplar

A filled CONTRIBUTING against the Ecommerce Application project, matching the Template shape exactly.

```markdown
# CONTRIBUTING — Ecommerce Application

## Contribution Overview

Fork the repo, branch from `main`, implement against the FRD spec, write tests, self-review against the PRD acceptance criteria, then submit a PR linked to the BACKLOG ID. A change is ready to merge when it passes Test, Format, Clippy, and Self-Lint CI gates.

## Contribution Guidelines

Commands are verbatim and copy-pasteable. Steps are numbered and sequential; prerequisites come before dependents. Paths are relative to the repo root. No secrets or credentials appear in any configuration shown. Templates carry comments that explain what to fill and why.

## Workflow

| Step | Action | Detail |
|------|--------|--------|
| 1 | Fork / Branch | Branch from `main` using `git checkout -b feat/<short-name>` |
| 2 | Implement | Follow FRD spec, write tests |
| 3 | Self-review | Check against acceptance criteria in PRD |
| 4 | Submit PR | Link to BACKLOG ID, describe what changed |
| 5 | Review | Maintainer approval after CI passes |
| 6 | Merge | Squash merge into `main` |

## Commit Convention

| Type | Usage | Example |
|------|-------|---------|
| `feat` | New feature | `feat(web): add product catalog page` |
| `fix` | Bug fix | `fix(web): handle empty cart on checkout` |
| `docs` | Documentation only | `docs(frd): update FR-ECOMMERCE-001` |
| `test` | Adding/updating tests | `test(web): add edge case for timeout` |
| `refactor` | Code refactor, no behaviour change | `refactor(web): extract button component` |
| `chore` | Maintenance, deps, config | `chore: bump dependencies` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](PRD.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |

```

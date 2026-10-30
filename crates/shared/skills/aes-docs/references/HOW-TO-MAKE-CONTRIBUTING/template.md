# CONTRIBUTING Template

Copy, fill, delete what does not apply to the project.

```markdown
# CONTRIBUTING — <project-name>

## Contribution Overview

<One paragraph: how to contribute, what mindset contributors should have, and what "ready to merge" looks like.>

## Contribution Guidelines

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
| PRD | <Business goals, user stories, and scope> | [PRD.md](PRD.md) |
| Architecture | <System boundaries, tech stack, and infra> | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

## Section Contract

Every section is required unless marked optional. Each exists for one reason.

| Section              | Why it belongs here                                                     |
| -------------------- | ----------------------------------------------------------------------- |
| Contribution Overview| Sets contributor mindset and "ready to merge" definition.               |
| Contribution Guidelines | The contribution rules every change must respect.                      |
| Workflow             | `Step &#124; Action &#124; Detail` contribution pipeline.                             |
| Commit Convention    | `Type &#124; Usage &#124; Example` conventional-commit prefixes.                         |
| Reference            | Connects contribution policy to PRD and Architecture.                   |

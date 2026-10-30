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
| `feat` | New feature | `feat(feature-name): <new-description>` |
| `fix` | Bug fix | `fix(feature-name): <fix-description>` |
| `docs` | Documentation only | `docs(doc-name): <update-description>` |
| `test` | Adding/updating tests | `test(feature-name): <test-description>` |
| `refactor` | Code refactor, no behaviour change | `refactor(feature-name): <refactor-description>` |
| `chore` | Maintenance, deps, config | `chore: <bump-description>` |

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

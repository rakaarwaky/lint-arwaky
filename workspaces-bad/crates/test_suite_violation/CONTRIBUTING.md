## Contribution Overview

Contribute to Lint Arwaky by keeping 7-layer architecture discipline, no-bypass fixes, and test-first changes. "Ready to merge" means: all local gates pass (format, clippy, self-lint, test suite), one regression test per FR, conventional commit, and a PR description that names the contribution path and links the issue.

## Contribution Guidelines

- **7-layer architecture**: every file belongs to a layer, named `layer_concern_role`, and obeys the layer's dependency rules. Read [ARCHITECTURE.md](ARCHITECTURE.md) before placing a new file.
- **Worktree discipline**: a feature or fix branch is worked in a git worktree, never by switching branches in the main checkout.
- **No async runtime in the core**: the linter core uses `std::thread` and `rayon`; `tokio` is confined to `file-watch` and `mcp-server`.
- **No bypasses**: `#[allow]`, `// eslint-disable`, and `# noqa` are not acceptable fixes. Correct the cause instead.
- **Single source of truth**: `crates/shared/skills/` holds the skill markdown; `crates/shared/src/project_setup/taxonomy_project_setup_constant.rs` embeds it into the binary and is maintained manually — update it when a skill file is added, removed, or renamed. The `catalog_matches_the_skills_directory` test fails when the two disagree, so commit the regenerated constant with the skill change.

## Workflow

| Step | Action                              | Detail                                                                                  |
|------|------------------------------------|----------------------------------------------------------------------------------------|
| 1    | Fork / Branch                     | `<branch-name>` following `<prefix>-<scope>`; branch into `develop`, not `main`       |
| 2    | Implement                         | Follow FRD spec, write tests first in `crates/<crate>/tests/` named after the FR ID   |
| 3    | Self-review                       | Check against acceptance criteria in `PRD.md` and run `bash scripts/gates.sh`          |
| 4    | Submit PR                         | Link the `BACKLOG` ID, describe what changed, name the contribution path               |
| 5    | Review                            | QA/PE sign-off on CRITICAL/WARNING findings, CI green, no merge conflicts             |
| 6    | Merge                             | Squash into `develop`, then promote to `main` with `--delete-branch`                  |

## Commit Convention

| Type     | Usage                                  | Example                                  |
|----------|----------------------------------------|------------------------------------------|
| `feat`   | New feature or capability              | `feat(auth): add login endpoint`         |
| `fix`    | Bug fix                                 | `fix(auth): handle expired token`        |
| `docs`   | Documentation only                     | `docs(frd): update FR-AUTH-001`         |
| `test`   | Adding/updating tests                 | `test(auth): add edge case for timeout`  |
| `refactor` | Code refactor, no behaviour change  | `refactor(auth): extract middleware`     |
| `chore`  | Maintenance, deps, config             | `chore: bump dependencies`              |

## Reference

| Artifact | Purpose                                        | Link |
|----------|------------------------------------------------|------|
| PRD      | Business goals, user stories, and scope        | [PRD.md](PRD.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](ARCHITECTURE.md) |

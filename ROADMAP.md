# ROADMAP — Lint Arwaky

## Progress Overview

RELEASED as v3.16.0 (tag `v3.16.0`, 2026-11-08). `VERSION` and `Cargo.toml` are at 3.16.1, the staged next patch. The v3.7.1/3.7.2 declarations (PE-3-01 / #623) stay superseded by v3.8.0, the first release cut after them. Release sign-off is recorded in `DEPLOY.md` and persona acceptance in `TEST.md` Section 4; both are pending re-collection for 3.16.1 and block the tag until filled. Open audit findings are read from the issue tracker rather than restated here, and the former "Audit Remediation (v3.7.1)" milestone is retired — every issue filed against it is closed. naming-rules and orphan-rules shipped in the 3.9-3.16 line and are recorded as Done, with their evidence in each crate's BACKLOG.md.

## Roadmap Status

### PR-LA-001: auto-fix

| Aspect       | Detail                            |
|--------------|----------------------------------|
| Priority     | P2                                |
| State        | Done                              |
| Backlog      | [BACKLOG.md](crates/auto-fix/BACKLOG.md) |

### PR-LA-002: cli-commands

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/cli-commands/BACKLOG.md) |

### PR-LA-003: config-system

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/config-system/BACKLOG.md) |

### PR-LA-004: dispatcher

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/dispatcher/BACKLOG.md) |

### PR-LA-005: doc-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/doc-rules/BACKLOG.md) |

### PR-LA-012: maintenance

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P3                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/maintenance/BACKLOG.md) |

### PR-LA-006: external-lint

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/external-lint/BACKLOG.md) |

### PR-LA-007: file-watch

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P3                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/file-watch/BACKLOG.md) |

### PR-LA-008: filesystem

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/filesystem/BACKLOG.md) |

### PR-LA-009: git-hooks

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P3                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/git-hooks/BACKLOG.md) |

### PR-LA-010: import-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/import-rules/BACKLOG.md) |

### PR-LA-013: mcp-server

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P3                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/mcp-server/BACKLOG.md) |

### PR-LA-011: logging

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/logging/BACKLOG.md) |

### PR-LA-014: naming-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/naming-rules/BACKLOG.md) |

### PR-LA-015: orphan-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/orphan-rules/BACKLOG.md) |

### PR-LA-016: project-setup

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/project-setup/BACKLOG.md) |

### PR-LA-017: quality-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/quality-rules/BACKLOG.md) |

### PR-LA-018: report-formatter

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/report-formatter/BACKLOG.md) |

### PR-LA-019: role-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/role-rules/BACKLOG.md) |

### PR-LA-020: shared

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/shared/BACKLOG.md) |

### PR-LA-021: structure-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/structure-rules/BACKLOG.md) |

### PR-LA-022: tui

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/tui/BACKLOG.md) |

## Reference

| Artifact     | Purpose                                          | Link|
|--------------|--------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope          | [PRD.md](PRD.md)|
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md)|

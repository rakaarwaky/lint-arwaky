# ROADMAP — Lint Arwaky

## Progress Overview

RELEASED as v3.8.0. It supersedes 3.7.1 and 3.7.2, which were declared in `Cargo.toml` and `CHANGELOG.md` but never tagged (PE-3-01 / #623, now closed); the last published 3.7 release was `v3.7.0`, and everything since first shipped as 3.8.0. The `DEPLOY.md` Release Sign-off Checklist records per-domain evidence; it is a pipeline-health snapshot, not authorization to tag a future version. Persona-level acceptance scenarios (Developer, DevOps, AI Agent) verified clean (see `TEST.md` Section 4). Document gate `lint-arwaky-cli docs .` reports 0 document invariant violations across all crates and root documents. Open BA/SA/UX/ARCH/BE/FE/PE findings sit against GitHub Milestone "Audit Remediation (v3.7.1)" (created 2026-10-01); per-issue assignment is partially manual.

## Roadmap Status

### PR-LA-001: filesystem

| Aspect       | Detail                            |
|--------------|----------------------------------|
| Priority     | P0                                |
| State        | Done                              |
| Backlog      | [BACKLOG.md](crates/filesystem/BACKLOG.md) |

### PR-LA-002: auto-fix

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/auto-fix/BACKLOG.md) |

### PR-LA-003: cli-commands

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/cli-commands/BACKLOG.md) |

### PR-LA-004: config-system

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/config-system/BACKLOG.md) |

### PR-LA-005: dispatcher

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/dispatcher/BACKLOG.md) |

### PR-LA-012: maintenance

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P0                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/maintenance/BACKLOG.md) |

### PR-LA-006: doc-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/doc-rules/BACKLOG.md) |

### PR-LA-007: external-lint

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/external-lint/BACKLOG.md) |

### PR-LA-008: file-watch

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
| State        | Done                                  |
| Backlog      | [BACKLOG.md](crates/file-watch/BACKLOG.md) |

### PR-LA-009: git-hooks

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P1                                    |
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
| Priority     | P1                                    |
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
| Priority     | P2                                    |
| State        | In Review                             |
| Backlog      | [BACKLOG.md](crates/naming-rules/BACKLOG.md) |

### PR-LA-015: orphan-rules

| Aspect       | Detail                              |
|--------------|-------------------------------------|
| Priority     | P2                                    |
| State        | In Review                             |
| Backlog      | [BACKLOG.md](crates/orphan-rules/BACKLOG.md) |

## Reference

| Artifact     | Purpose                                          | Link|
|--------------|--------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope          | [PRD.md](PRD.md)|
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md)|

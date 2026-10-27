## Progress Overview

RELEASED as v3.8.0. It supersedes 3.7.1 and 3.7.2, which were declared in `Cargo.toml` and `CHANGELOG.md` but never tagged (PE-3-01 / #623, now closed); the last published 3.7 release was `v3.7.0`, and everything since first shipped as 3.8.0. The `DEPLOY.md` Release Sign-off Checklist records per-domain evidence; it is a pipeline-health snapshot, not authorization to tag a future version. Persona-level acceptance scenarios (Developer, DevOps, AI Agent) verified clean (see `TEST.md` Section 4). Document gate `lint-arwaky-cli docs .` reports 0 document invariant violations across all crates and root documents. Open BA/SA/UX/ARCH/BE/FE/PE findings sit against GitHub Milestone "Audit Remediation (v3.7.1)" (created 2026-10-01); per-issue assignment is partially manual.

## Roadmap Status

### PR-LA-001: Multi-language scanning (Rust, Python, JS/TS)

| Aspect       | Detail                              |
|--------------|-----------------------------------|
| Priority     | P0                                 |
| State        | Done                               |
| Dependencies | —                                 |
| Updated      | 2026-10-02                        |

### PR-LA-002: 34 AES rules enforcement across 7 groups

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P0                                       |
| State        | Done                                     |
| Dependencies | BE config-wipe CRITICAL finding (#522)  |
| Updated      | 2026-10-02                              |

### PR-LA-003: CLI core commands (`check`, `scan`, `fix`, `ci`)

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P0                                       |
| State        | Done                                     |
| Dependencies | #606, #611 (re-verification pending)    |
| Updated      | 2026-10-01                              |

### PR-LA-004: MCP server with 5 tools, full CLI parity

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P0                                       |
| State        | Done                                     |
| Dependencies | BE path-validation CRITICAL finding (#608) |
| Updated      | 2026-10-01                              |

### PR-LA-005: Self-auditing capability

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P0                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-10-02                              |

### PR-LA-006: External linter adapters (Clippy, Ruff, ESLint)

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-10-02                              |

### PR-LA-007: SARIF 2.1.0, JUnit XML, JSON reports

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-09-30                              |

### PR-LA-008: Git hooks integration

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | BE hook-overwrite finding (#522)        |
| Updated      | 2026-10-01                              |

### PR-LA-009: Auto-fix capabilities (remove + replace + rename)

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-09-30                              |

### PR-LA-010: Watch mode for continuous linting

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-09-30                              |

### PR-LA-011: TUI file browser

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P2                                       |
| State        | Done                                     |
| Dependencies | #552, #606, #611, #608 (re-verification pending) |
| Updated      | 2026-10-01                              |

### PR-LA-012: Workspace exit-code contract enforced everywhere

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P0                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-09-30                              |

### PR-LA-013: Acceptance tests standardized (one test file per FR)

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P1                                       |
| State        | Done                                     |
| Dependencies | —                                       |
| Updated      | 2026-10-02                              |

### PR-LA-014: Windows support

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P2                                       |
| State        | In Review                                |
| Dependencies | Dedicated Windows CI runner (deferred)  |
| Updated      | 2026-10-02                              |

### PR-LA-015: Deeper monorepo performance optimizations

| Aspect       | Detail                                  |
|--------------|----------------------------------------|
| Priority     | P2                                       |
| State        | In Review                                |
| Dependencies | —                                       |
| Updated      | 2026-10-02                              |

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope          | [PRD.md](PRD.md) |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |

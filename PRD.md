# PRD — Lint Arwaky

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust + Python + JavaScript/TypeScript together, enforces architectural rules, and works for both human developers (CLI) and AI agents (MCP tools).

## Goals & Success Metrics

- Goal 1: Multi-language linting in a single pass (Rust, Python, JS/TS)
- Goal 2: 24 AES rules enforced across 5 groups (Naming, Import, Quality, Role, Orphan)
- Goal 3: MCP server with 5 tools for autonomous AI-agent integration
- Goal 4: Zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` flagged)
- Goal 5: Self-auditing — project lints itself under its own rule engine

## User Personas

- **AI Agent**: Autonomous linting, self-healing, code review via MCP tools
- **Developer**: Lint codebases, enforce architecture during local development
- **DevOps / CI**: Quality gates, trend reports, dependency scans
- **Contributor**: Extend adapters, add CLI commands
- **Reviewer**: Architecture audit, code quality analysis

## Scope

- In scope:
  - CLI binary (`lint-arwaky-cli`) for human developers
  - MCP server (`lint-arwaky-mcp`) for AI agents
  - TUI file browser (`lint-arwaky-tui`)
  - 24 AES rules across 5 groups
  - External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
  - SARIF 2.1.0, JUnit XML, JSON reports
  - Git hooks integration
  - Auto-fix capabilities

- Out of scope:
  - IDE plugins (VS Code, IntelliJ)
  - Web dashboard
  - Cloud-hosted SaaS
  - Non-Rust implementation

## Feature Requirements (Prioritized)

### P0 — Must Have

- [ ] Multi-language scanning (Rust, Python, JS/TS)
- [ ] 24 AES rules enforcement
- [ ] CLI with `check`, `scan`, `fix`, `ci` commands
- [ ] MCP server with 5 tools
- [ ] Self-auditing capability

### P1 — Should Have

- [ ] External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
- [ ] SARIF 2.1.0, JUnit XML, JSON reports
- [ ] Git hooks integration
- [ ] Auto-fix capabilities
- [ ] Watch mode for continuous linting

### P2 — Nice to Have

- [ ] TUI file browser
- [ ] Orphan code detection
- [ ] Duplicate code detection
- [ ] Dependency vulnerability scanning

## Non-functional Requirements (High-level)

- Performance: Scan 1000 files in < 5 seconds
- Security: No network calls required for core functionality
- Scalability: Handle monorepos with 10,000+ files
- Platform: Linux (primary), macOS (secondary)
- Binary: Static release via `cargo build --release`

## Open Questions / Risks

- Windows support timeline
- Performance optimization for very large codebases
- Integration with CI/CD platforms (GitHub Actions, GitLab CI)

# FRD — external-lint

## Feature Goal
The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Ruff, Mypy, ESLint, Prettier, and others on Rust, Python, and JS/TS files, normalizes their stdout/JSON reports, and integrates them into the Lint Arwaky compliance report.

## Requirements & Scope
- Rust ecosystem — cargo clippy, rustfmt, cargo-audit.
- Python ecosystem — ruff, mypy, bandit.
- JavaScript/TypeScript ecosystem — eslint, prettier, tsc.
- Normalize external tool reports into the unified Lint Arwaky format.
- Map tool-specific severity levels (error/warning/info/refactor) to Lint Arwaky Severity (CRITICAL/HIGH/MEDIUM/LOW).
- Run subprocesses asynchronously or parallelized where possible.

## Success Indicators
- [ ] Tool discovery and fallback — missing tools are safely ignored or warned about without crashing the run.
- [ ] Seamless report unification — AES and external violations combined in a single unified report or MCP response.
- [ ] Error level translation — tool severities are correctly mapped to Lint Arwaky Severity.
- [ ] Performance control — subprocesses run async/parallel to prevent blocking CLI feedback.

# FRD — external-lint

## Feature Goal

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Ruff, Mypy, ESLint, Prettier, and others on Rust, Python, and JS/TS files, normalizes their stdout/JSON reports, and integrates them into the Lint Arwaky compliance report.

## Requirements & Scope

- Supported Linters & Tools
  - Rust Ecosystem
    - `cargo clippy`: Catches idioms, performance bugs, and style issues.
    - `rustfmt`: Verifies codebase-wide formatting guidelines.
    - `cargo-audit`: Audits dependencies listed in `Cargo.lock` for known vulnerabilities.
  - Python Ecosystem
    - `ruff`: Extremely fast linter replacing flake8/autoflake/isort.
    - `mypy`: Performs static type checking on Python source code.
    - `bandit`: Scans Python code for common security vulnerabilities (e.g. SQLi, unsafe imports).
  - JavaScript / TypeScript Ecosystem
    - `eslint`: Checks JS/TS styling and syntax rules.
    - `prettier`: Ensures consistent formatting rules.
    - `tsc`: Checks TypeScript compiler/typing errors.
- Report Normalization
  - Normalize external tool reports (stdout/JSON) into the unified Lint Arwaky format.
  - Map tool-specific severity levels (error, warning, info, refactor) to Lint Arwaky Severity (CRITICAL, HIGH, MEDIUM, LOW).
  - Combine local AES rule violations with external linter violations in a single unified terminal report or MCP response.
- Execution
  - Run subprocesses asynchronously or parallelized where possible to prevent blocking CLI feedback.
  - Safely ignore or warn about missing tools without crashing the run process.

## Success Indicators

- [ ] Tool discovery and fallback — missing tools are safely ignored or warned about without crashing the run.
- [ ] Seamless report unification — AES and external violations combined in a single unified report or MCP response.
- [ ] Error level translation — tool severities are correctly mapped to Lint Arwaky Severity.
- [ ] Performance control — subprocesses run async/parallel to prevent blocking CLI feedback.

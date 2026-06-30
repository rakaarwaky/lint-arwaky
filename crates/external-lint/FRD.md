# Feature Requirement Document (FRD) - External Lint

See [README.md](../../../README.md) for adapter overview and [SKILL.md](../../../.agents/skills/build-verify/SKILL.md) for verification steps.

## 1. Feature Goal

The goal of the `external-lint` module is to serve as an aggregate bridge for external, industry-standard linter and formatting tools. It coordinates and executes tools like Cargo Clippy, Ruff, Mypy, ESLint, and Prettier on Rust, Python, and JS/TS files, normalizes their stdout/JSON reports, and integrates them seamlessly into the Lint Arwaky compliance report.

## 2. Requirements & Scope

The `external-lint` module manages and wraps external processes based on the following specifications:

### Supported Linters & Tools

- **Rust Ecosystem**
  - `cargo clippy`: Catches idioms, performance bugs, and style issues.
  - `rustfmt`: Verifies codebase-wide formatting guidelines.
  - `cargo-audit`: Audits dependencies listed in `Cargo.lock` for known vulnerabilities.

- **Python Ecosystem**
  - `ruff`: Extremely fast linter replacing flake8/autoflake/isort.
  - `mypy`: Performs static type checking on Python source code.
  - `bandit`: Scans Python code for common security vulnerabilities (e.g. SQLi, unsafe imports).

- **JavaScript / TypeScript Ecosystem**
  - `eslint`: Checks JS/TS styling and syntax rules.
  - `prettier`: Ensures consistent formatting rules.
  - `tsc`: Checks TypeScript compiler/typing errors.

### Output Normalization

- **Path Mapping**: Normalizes paths returned by external tools to match project-root relative or absolute formats.
- **Unified Diagnostic Schema**: Maps external diagnostics (line, column, code, message, severity) into the `LintResult` Value Object used by Lint Arwaky.

---

## 3. Success Indicators

The success of the `external-lint` module is measured by:

- **Tool Discovery and Fallback**: Safely ignores or warns about missing tools without crashing the run process.
- **Seamless Report Unification**: Combines local AES rules violations with external linter violations in a single unified terminal report or MCP response.
- **Error Level Translation**: Correctly maps tool-specific severity levels (e.g., error, warning, info, refactor) to Lint Arwaky's `Severity` levels (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`).
- **Performance Control**: Runs sub-processes asynchronously or parallelized where possible to prevent blocking CLI feedback.

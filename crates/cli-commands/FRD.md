# Feature Requirement Document (FRD) - CLI Commands

## 1. Feature Goal

The primary purpose of the `cli-commands` module is to provide a unified command-line interface (CLI) for users to run the entire linting pipeline. This module implements surfaces for main commands such as `check`, `scan`, `fix`, `git`, `config`, `setup`, `tui`, and `watch`.

## 2. Requirements & Scope

The `cli-commands` module is responsible for providing command surfaces based on the following specifications:

### Command Specifications

- **check**: Check a single file or directory against AES rules.
- **scan**: Scan the entire workspace and generate a comprehensive report.
- **fix**: Apply automatic fixes to files that violate rules.
- **git**: Implement git hooks and diff checks for pre-commit.
- **config**: Manage lint_arwaky configuration (initialization, validation, updates).
- **setup**: Set up a new project with AES directory structure.
- **tui**: Launch the terminal user interface (TUI) for interactive linting.
- **watch**: Monitor file changes in real-time and run automatic scans.

### Inputs

- Command-line arguments (`&[String]`) parsed by clap.
- Project configuration from various sources (YAML, environment variables).

### Outputs

- Linter reports in terminal format (colored output via the console crate).
- Appropriate exit code (0 for success, non-zero for violations).

---

## 3. Success Indicators

The success of the `cli-commands` module is measured by:

- **UX Consistency**: All commands follow a consistent input/output pattern.
- **Performance**: CLI is responsive with buffered output for large workspaces.
- **Help Documentation**: Every command has clear `--help` documentation.
- **Self-Audit Conformity**: The module itself passes AES rule checks.

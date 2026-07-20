# FRD — cli-commands

## Feature Goal
The cli-commands crate provides a unified command-line interface (CLI) that drives the entire linting pipeline. It implements surfaces for the main commands: check, scan, fix, git, config, setup, tui, and watch.

## Requirements & Scope
- check — check a single file or directory against AES rules.
- scan — scan the entire workspace and generate a comprehensive report.
- fix — apply automatic fixes to files that violate rules.
- git — implement git hooks and diff checks for pre-commit.
- config — manage lint_arwaky configuration (initialization, validation, updates).
- setup — set up a new project with the AES directory structure.
- watch — monitor file changes in real time and run automatic scans.
- tui — launch the interactive terminal UI.
- Consistent input/output patterns and clear --help documentation for every command.

## Success Indicators
- [ ] UX consistency — all commands follow a consistent input/output pattern.
- [ ] Performance — CLI is responsive with buffered output for large workspaces.
- [ ] Help documentation — every command has clear --help documentation.
- [ ] Rule conformance — the crate itself passes AES rule checks when complete.

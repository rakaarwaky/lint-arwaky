# FRD — git-hooks

## Feature Goal
The git-hooks crate implements a git hooks system for lint checking at the pre-commit stage. It detects file changes through git diffs and runs the linter only on modified files, ensuring code entering the repository complies with AES rules.

## Requirements & Scope
- In scope:
  - Installation of pre-commit hooks across Linux, macOS, and Windows.
  - Git diff-based detection so only changed files are scanned.
  - Blocking commits that violate AES rules.
  - Optional diff checks for staged changes.
- Out of scope:
  - Continuous filesystem watching — only git state (diff/staged) drives the hook, not a watcher.
  - Evaluating the rules — the hook invokes the checker; the CHECK logic lives in the analysis crates.

## Success Indicators
- [ ] Hook installation — hooks correctly installed on all supported system types (Linux, macOS, Windows).
- [ ] Diff accuracy — only files that have actually changed are scanned.
- [ ] Commit blocking — commits that violate AES rules are successfully blocked.
- [ ] Rule conformance — the crate's own source complies with AES rules when complete.

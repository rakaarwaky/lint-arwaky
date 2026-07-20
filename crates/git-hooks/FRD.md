# Feature Requirement Document (FRD) - Git Hooks

See [README.md](../../../README.md) for usage and [TEST.md](../../../TEST.md) for verification criteria.

## 1. Feature Goal

The primary purpose of the `git-hooks` module is to implement a git hooks system for lint checking at the pre-commit stage. This module detects file changes through git diffs and runs the linter only on modified files, ensuring that code entering the repository complies with AES rules.

## 2. Requirements & Scope

The `git-hooks` module is responsible for git hooks based on the following specifications:


---

## 3. Success Indicators

The success of the `git-hooks` module is measured by:

- **Hook Installation**: Hooks are correctly installed on all system types (Linux, macOS, Windows).
- **Diff Accuracy**: Only files that have actually been changed are scanned.
- **Commit Blocking**: Commits that violate AES rules are successfully blocked.
- **Rule Conformance**: When complete, the module's own source complies with AES rules in its source code.

# Feature Requirement Document (FRD) - Git Hooks

See [README.md](../../../README.md) for usage and [TEST.md](../../../TEST.md) for verification criteria.

## 1. Feature Goal

The primary purpose of the `git-hooks` module is to implement a git hooks system for lint checking at the pre-commit stage. This module detects file changes through git diffs and runs the linter only on modified files, ensuring that code entering the repository complies with AES rules.

## 2. Requirements & Scope

The `git-hooks` module is responsible for git hooks based on the following specifications:

### Component Specifications

- **GitHooksOrchestrator**: Coordinates the pre-commit hook execution process.
- **DiffChecker**: Analyzes git diffs to determine changed files.
- **HookManager**: Manages git hook installation and configuration.
- **GitHookAdapter**: Adapter for interaction with the git system (subprocess calls).

### Inputs

- Git staging status (staged files).
- Hook configuration defined in YAML.

### Outputs

- Lint results on modified files.
- Non-zero exit code if violations are found, preventing the commit.

---

## 3. Success Indicators

The success of the `git-hooks` module is measured by:

- **Hook Installation**: Hooks are correctly installed on all system types (Linux, macOS, Windows).
- **Diff Accuracy**: Only files that have actually been changed are scanned.
- **Commit Blocking**: Commits that violate AES rules are successfully blocked.
- **Self-Audit Conformity**: The module itself complies with AES rules in its source code.

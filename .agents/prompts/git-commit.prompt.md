---
name: git-commit-lint-arwaky
description: "Generate conventional commit message for Lint Arwaky changes. Use when committing code changes to this project."
version: 1.0.0
---

# Git Commit — Lint Arwaky

## Purpose

Generate a conventional commit message following the Lint Arwaky project conventions.

## Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | When to Use |
|------|-------------|
| `feat` | New feature, new file, new capability |
| `fix` | Bug fix, error correction |
| `refactor` | Code restructuring, merging files, renaming |
| `docs` | Documentation changes (README, ARCHITECTURE, etc.) |
| `test` | Adding or modifying tests |
| `chore` | Build, CI/CD changes, dependency updates |
| `style` | Formatting, whitespace, naming conventions |

### Scopes

| Scope | When to Use |
|-------|-------------|
| `import-rules` | Changes in import_rules crate |
| `code-analysis` | Changes in code_analysis crate |
| `config-system` | Changes in config_system crate |
| `shared` | Changes in shared crate |
| `cli` | CLI command changes |
| `mcp` | MCP server changes |
| `tui` | TUI launcher changes |
| `rust` | Rust skill/prompt/instruction changes |
| `all` | Changes across multiple crates |

## Rules

1. **Use imperative mood**: "add feature" not "added feature" or "adds feature"
2. **Lowercase description**: "add cycle import analyzer" not "Add cycle import analyzer"
3. **No period at end**: "merge files" not "merge files."
4. **Max 72 characters** for the subject line
5. **Separate subject from body** with blank line
6. **Reference AES codes** in body if applicable (e.g., "AES205: circular dependency detection")

## Examples

### Simple Feature

```
feat(import-rules): add cycle import analyzer
```

### Merge + Refactor

```
refactor(import-rules): merge cycle_analyzer into cycle_import_analyzer

Combine DependencyCycleAnalyzer and CycleAnalyzer into single
CycleImportAnalyzer implementing ICycleImportProtocol.

- Merged capabilities_cycle_import_analyzer.rs and capabilities_cycle_analyzer.rs
- Created unified contract_cycle_import_protocol.rs
- Updated root container wiring
- Deleted old capabilities_cycle_analyzer.rs
```

### Bug Fix

```
fix(shared): correct trait method signature in ICycleAnalyzerPort

Add &self parameter to pure_normalize_to_layer method.
```

### Documentation

```
docs(ARCHITECTURE): update 7-layer specification for merged modules
```

## Before Commit Checklist

- [ ] Run `cargo fmt --all`
- [ ] Run `cargo clippy --all-targets -- -D warnings`
- [ ] Run `cargo test --workspace`
- [ ] Verify no `[allow(...)]` bypasses in new code
- [ ] Check branch naming follows rules (main, develop, features/<name>, fix/<name>)

## Auto-Commit (AI Executes Directly)

When invoked, the AI agent MUST automatically execute these commands via terminal — do NOT ask user for permission:

```bash
cd /home/raka/mcp-arwaky/lint-arwaky

# Stage all changes
git add .

# Commit with generated message
git commit -m "<type>(<scope>): <description>" <<'COMMIT_EOF'
<optional body>
COMMIT_EOF

# Push to remote (only if user explicitly requests)
# git push origin <branch-name>
```

## Execution Rules

1. **Always run `git status --short` first** to identify changed files
2. **Generate commit message** based on changed files and context
3. **Execute `git add .`** to stage all changes
4. **Execute `git commit -m "..."`** with the generated message
5. **Do NOT ask for user confirmation** — execute directly
6. **Report back** the commit hash and success status

## Example Invocation

When user types `/git-commit-lint-arwaky`:

1. AI reads `git status --short` output
2. AI generates appropriate commit message following conventions above
3. AI runs `git add . && git commit -m "..."` automatically
4. AI reports: `Commit created: <hash>`

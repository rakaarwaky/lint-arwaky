# Contributing to Lint Arwaky

> This guide covers the contribution paths this project supports: shipping a
> change, from setup to merge.

- [Principles](#principles)
- [Development Setup](#development-setup)
- [Feature Change](#feature-change)
- [Documentation Change](#documentation-change)
- [Quality Verification & PR Process](#quality-verification--pr-process)

---

## Principles

Before making changes, observe these non-negotiable rules:

1. **7-layer architecture**: every file belongs to a layer, named
   `layer_concern_role`, and obeys the layer's dependency rules. Read
   [ARCHITECTURE.md](ARCHITECTURE.md) before placing a new file.
2. **Worktree discipline**: a feature or fix branch is worked in a git worktree
   under `.worktree/<branch>`, never by switching branches in the main checkout.
3. **No async runtime in the core**: the linter core uses `std::thread` and
   `rayon`; `tokio` is confined to `file-watch` and `mcp-server`.
4. **No bypasses**: `#[allow]`, `// eslint-disable`, and `# noqa` are not
   acceptable fixes. Correct the cause instead.
5. **Single source of truth**: `crates/shared/src/project_setup/taxonomy_skills_constant.rs`
   is generated — run `python3 tools/regenerate_skills.py` after any skill file
   is added, removed, or renamed.

---

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/rakaarwaky/lint-arwaky.git
   cd lint-arwaky
   ```
2. Verify host prerequisites:
   ```bash
   cargo --version   # >= 1.85.0, edition 2024
   rustup show active-toolchain
   ```
3. Build the workspace:
   ```bash
   CARGO_INCREMENTAL=0 cargo build --release
   ```
4. Verify the installation:
   ```bash
   ./target/release/lint-arwaky-cli version
   ```

### Prerequisites

- **Rust** >= 1.85.0 (edition 2024, pinned via `rust-toolchain.toml`)
- **Cargo** (bundled with Rust)
- **Git**
- Familiarity with `clap` derive macros, JSON-RPC 2.0 (MCP protocol), and
  `std::thread` / `rayon` concurrency

> Optional: `rustup` for toolchain management, `cargo-watch` for development.

### Running the binaries

```bash
# Run the CLI
./target/release/lint-arwaky-cli version
# Expected: lint-arwaky 3.7.1

# Run the MCP server in a separate terminal
./target/release/lint-arwaky-mcp
# Expected: "Listening on stdin/stdout (JSON-RPC 2.0)"

# Self-lint the project (must report 0 violations)
./target/release/lint-arwaky-cli check .
# Scans `crates/`, `modules/`, `packages/` under the AES rules this project enforces.
```

For development without the release profile:

```bash
cargo run --bin lint-arwaky-cli -- scan .
cargo run --bin lint-arwaky-mcp
```

---

## Feature Change

A new rule, a fix to an existing rule, or a change to crate behaviour.

1. Read the target crate's `FRD.md` to learn its contract, and `AGENTS.md` for
   session guardrails.
2. Create a worktree:
   ```bash
   git worktree add -b <branch-name> .worktree/<branch-name> origin/main
   cd .worktree/<branch-name>
   ```
3. Write the test first in the owning crate's `tests/` directory, then the
   implementation.
4. Verify the change took effect:
   ```bash
   cargo nextest run -p <crate>
   ```
5. Commit with a conventional prefix (`feat:`, `fix:`, `refactor:`) and open a PR.

## Documentation Change

A change to Markdown only, with no code edit.

1. Create a worktree the same way as above.
2. Edit the document; the AES606 heading contract governs the H1/H2 structure of
   every root document, so run the doc gate before committing:
   ```bash
   ./target/debug/lint-arwaky-cli docs .
   ```
3. If a skill file changed, regenerate the derived constant:
   ```bash
   python3 tools/regenerate_skills.py
   ```
4. Commit with `docs:` and open a PR.

---

## Quality Verification & PR Process

Clear every gate before committing or opening a PR.

### 1. Run the verification commands

```bash
bash scripts/gates.sh
```

This runs format, clippy, self-lint, and the full test suite. CI mirrors each
of these as a separate required status check.

### 2. Code style

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

### 3. Conventional commit guidelines

| Prefix      | Usage                                  |
| ----------- | -------------------------------------- |
| `feat:`     | New feature or capability              |
| `fix:`      | Bug fix                                |
| `chore:`    | Maintenance, dependency bump, cleanup  |
| `docs:`     | Documentation changes                  |
| `refactor:` | Refactoring without behavioral change  |

### 4. Pull request checklist

- All local verification commands pass.
- New files follow the project's naming and directory conventions.
- Any registry or manifest the project keeps is updated.
- No absolute paths, secrets, or machine-specific values leaked into the diff.
- The PR description names the contribution path and links the issue.

---

## Branch Management

- Allowed branch naming: `main`, `develop`. Feature/fix branches are merged into
  `develop` via PR, then promoted to `main`.
- Use `--delete-branch` when merging feature/fix PRs; never delete `develop`
  when merging to `main`.

## Why Contribute

| Aspect                     | Benefit                                                        |
| -------------------------- | -------------------------------------------------------------- |
| **Real-world impact**      | Your code powers the same rule engine that audits this project |
| **Skill development**      | Practice Rust, MCP, and 7-layer architecture                   |
| **Open-source experience** | Build portfolio with a self-auditing codebase                  |
| **Community**              | Join a project where every PR is checked by the rules it adds  |
| **Learning opportunity**   | Study a codebase that passes its own architecture linter       |

## Questions?

Open an issue on GitHub or contact the maintainer.

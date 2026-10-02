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
5. **Single source of truth**: `crates/shared/skills/` holds the skill markdown;
   `crates/shared/src/project_setup/taxonomy_project_setup_constant.rs` embeds
   it into the binary and is generated — run
   `python3 tools/regenerate_skills.py` after any skill file is added, removed,
   or renamed. The `catalog_matches_the_skills_directory` test fails when the
   two disagree, so commit the regenerated constant with the skill change.

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

3. Write the test first in the owning crate's `tests/` directory, named after the
   functional requirement ID (e.g. `tests/acceptance_FR_001.rs`, scoped to the
   crate directory namespace and mapping to `FR-<Feature>-001` in that crate's `FRD.md`),
   then implement the feature. Legacy tests already carrying domain-specific names
   (e.g. `acceptance_cli_commands.rs`, `acceptance_mcp_server.rs`) keep their name
   until a dedicated rename refactor; new tests follow the `acceptance_FR_NNN` pattern.
4. Verify the change took effect:

   ```bash
   cargo nextest run -p <crate>
   ```

5. Sync the contract documents the change touches, in the same commit:
   - A new or changed `FixOutcome` reason → the Reason Code Reference table in
     `crates/auto-fix/FRD.md`.
   - A changed field on a shared value object → the attribute table in
     `crates/shared/DATA.md`.
   - A renumbered, added, or consolidated rule code → `RULES_AES.md` first, then
     every `DESIGN.md`/`FRD.md` range that names it.
   Then run the doc-consistency gate, which fails on exactly these drifts:

   ```bash
   ```

6. Commit with a conventional prefix (`feat:`, `fix:`, `refactor:`) and open a PR.

## Documentation Change

A change to Markdown only, with no code edit.

1. Create a worktree the same way as above.
2. Edit the document; the AES605 heading contract governs the H1/H2 structure of
   every root document, so run the doc gate before committing:

   ```bash
   ./target/debug/lint-arwaky-cli docs .
   ```

3. If a skill file changed, regenerate the derived constant:

   ```bash
   python3 tools/regenerate_skills.py
   ```

4. Run the doc-consistency gate — it validates in-repo Markdown anchor links,
   rule-code ranges, the shared data model's attribute tables, and the auto-fix
   reason table:

   ```bash
   ```

5. Commit with `docs:` and open a PR.

### Claim Before You Build (Shared Contract Changes)

Before restructuring a shared document contract enforced by AES605 (such as `AGENTS.md` required H2 headings, `PRD.md` sections, or `ARCHITECTURE.md` layers):

1. **Open or claim an issue first**: Do not begin implementation without an assigned GitHub issue.
2. **Adjudicate conflicts**: If competing proposals emerge (such as #493 vs #494 regarding 11 vs 12 H2 sections), the Tech Lead / Architect resolves the specification contract before PR creation.
3. **Record rationale**: Document the resolution in `ROADMAP.md` Change Log or `CHANGELOG.md` to prevent duplicate parallel PRs.

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

### Issue Title Convention

Until GitHub Label writes are reliable (see #618), issue titles are the
authoritative structured-classification source: `[ROLE][SEVERITY] {title}`,
where ROLE is one of `BA`/`SA`/`UX`/`ARCH`/`BE`/`FE`/`PE`/`QA` and SEVERITY is
one of `CRITICAL`/`WARNING`/`INFO`. Do not rely on Label chips alone for triage
queries — parse titles instead (QA #647).

### Defect Assignment

Every new defect issue must be assigned to an owner within 2 business days of
filing, or labeled `needs-owner` if no owner is available yet — a silently
empty `assignees` field is not an acceptable tracking state (QA #646).
CRITICAL severity issues should be assigned at creation time whenever
possible. Every assigned defect fix follows the Regression Test Convention in
[TEST.md §2.0](TEST.md): a permanent `regression_*` test guard lands together
with the fix.

---

## Branch Management

- Allowed branch naming: `main`, `develop`. Feature/fix branches are merged into
  `develop` via PR, then promoted to `main`.
- Use `--delete-branch` when merging feature/fix PRs; never delete `develop`
  when merging to `main`.

## Versioning Policy

All lint-arwaky workspace crates are published in **lockstep** under a single
version number (see `[workspace.dependencies]` in the root `Cargo.toml`). This
is a deliberate trade-off, not an accident:

- A version bump does **not** imply every crate's public API changed — consult
  `CHANGELOG.md`'s per-release notes for which crates were actually modified.
- Lockstep keeps the ~20 crates coherent for consumers who install the
  `lint-arwaky-cli` binary (the primary distribution), where cross-crate
  version skew is a bug, not a feature.
- Per-crate independent semver (release-plz / cargo-workspaces style) was
  evaluated and rejected for now: no crate is currently consumed standalone
  outside the workspace binaries. If that changes, revisit this policy and
  record the migration plan here.

When bumping the version, every `[workspace.dependencies]` entry, every member
crate `version`, and `CHANGELOG.md` must move together in the same change.

## Issue Closure Policy

A **defect/bug issue must not be closed without a PR that includes a regression
test** demonstrating the reported behavior no longer reproduces. Link the test
(file + test name) in the closing PR description or comment.

This policy exists because issues #354, #366, and #368 were previously closed
while at least one of their originally-reported defects remained in the code —
confirmed by a later re-audit and re-filed as #552, #564, and #566. "Closed"
must mean "verified fixed", not "worked on".

Feature/enhancement issues are exempt (there is no defect to reproduce), but
their acceptance criteria must still be verifiably met before closure.

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

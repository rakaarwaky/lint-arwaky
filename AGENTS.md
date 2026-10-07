---
trigger: always
description: "Lint Arwaky operational guide. ARCHITECTURE.md wins on ambiguity."
---
# Lint Arwaky

## Runtime

- Language: Rust (edition 2024, pinned via rust-toolchain.toml)
- Environment: cargo workspace under crates/, built on Linux/macOS with CARGO_INCREMENTAL=0
- Artifacts: target/ and $HOME/.local/bin/ (install path). Never use /tmp for
  build output a reviewer must find
- Package manager: cargo, with Cargo.lock

```bash
rustc --version
CARGO_INCREMENTAL=0 cargo build --release
```

## Project Quick Facts

INPUT  = Rust workspace under crates/ with AES 7-layer naming
OUTPUT = 0 violations from lint-arwaky-cli check ., docs ., and scripts/gates.sh

## Pipeline

scan → check → gates → publish

## Git Conventions

- Default branch: main
- Worktree directory: .worktrees/
- Branch pattern: <type>/<feature-x>
- Branch prefixes: docs/, fix/, feat/, chore/, test/

Create the working worktree:

```bash
git fetch origin main
git worktree add -b docs/feature-x .worktrees/docs-feature-x origin/main
cd .worktrees/docs-feature-x
```

After validation:

```bash
git add .
git commit -m "docs: summary"
git push -u origin docs/feature-x
gh pr create --base main --head docs/feature-x \
  --title "docs: summary" \
  --body "$(cat <<'PRBODY'
What changed:
PRBODY
)"
```

After merge:

```bash
cd ../..
git worktree remove .worktrees/docs-feature-x
git branch -d docs/feature-x
```

## Commands

Every command must match the exact CI gate.

```bash
# Tests
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2   # all tests (3x faster than cargo test)
CARGO_INCREMENTAL=0 cargo nextest run -p <crate>                      # one unit
CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>        # one file

# Lint / types / architecture
CARGO_INCREMENTAL=0 cargo fmt --all -- --check                         # matches ci.yml "Format" job
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings          # matches ci.yml "Clippy" job
lint-arwaky-cli check .                                                # architecture scanner (self-lint)
```

## Guided Skills

- Skill directory: .agents/skills/

## Related Documents

- [README.md](README.md): What the tool does, every command, and how to install it
- [PRD.md](PRD.md): What the product does and why — feature tiers, exit codes, non-functional goals
- [ARCHITECTURE.md](ARCHITECTURE.md): Layer contract for every crate and the rules that keep the boundaries
- [RULES_AES.md](RULES_AES.md): The 32 rules, what each one forbids, and why
- [ROADMAP.md](ROADMAP.md): What ships next, in order
- [TEST.md](TEST.md): How to run the suite and what a green run proves
- [CONTRIBUTING.md](CONTRIBUTING.md): Branch, commit, and PR conventions
- [DEPLOY.md](DEPLOY.md): Release path and the checks it gates on
- [SECURITY.md](SECURITY.md): Reporting a vulnerability and what counts as one

---

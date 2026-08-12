---
name: ci-quality-gates
description: Set up a complete CI system with quality gates, architecture enforcement, and AI code review for a Rust multi-crate workspace. Covers GitHub Actions workflows, branch protection rulesets, self-linting, and integrating CodeRabbit, Codacy, cubic, and Repowise. Use when asked to create or replicate CI like this project's.
metadata:
  tags: [ci, github-actions, quality-gates, code-review, coderabbit, codacy, cubic, repowise, rust, self-lint, architecture, aes]
  triggers:
    - "setup ci"
    - "quality gates"
    - "ci pipeline"
    - "code review bot"
    - "architecture enforcement"
    - "replicate ci"
    - "branch protection"
    - "self-lint"
  dependencies: []
  related:
    - lint-arwaky-rust
    - repowise-scan
    - role-quality-analysis
    - role-tech-lead
---

# CI Quality Gates System

Replicate the CI setup used by this repo: quality gates enforced by GitHub
Actions + a branch-protection ruleset, architecture enforcement via the
project's own linter (self-lint), and four AI review layers (CodeRabbit,
Codacy, cubic, Repowise). This is a copy-and-extend blueprint, not a
one-off checklist — adapt names and paths to the target repo.

## Architecture overview

```
PR opened / pushed
   │
   ├─ CI workflow (.github/workflows/ci.yml)
   │    ├─ Format  (cargo fmt --check)
   │    ├─ Clippy  (cargo clippy -D warnings)
   │    ├─ Build   (cargo build --release)
   │    ├─ Tests   (cargo nextest run)
   │    └─ Self-Lint (lint-arwaky-cli check .  == 0 violations)
   │
   ├─ Codacy Static Code Analysis (app.codacy.com)
   ├─ CodeRabbit (app.coderabbit.ai) — change review
   ├─ cubic AI reviewer (www.cubic.dev) — ultrareview
   └─ Repowise bot (.repowise/bot.yaml) — health gate advisory
   │
   └─ Ruleset "Protect main" (GitHub rulesets)
        ├─ required_status_checks: Format, Clippy, Build, Tests,
        │    Self-Lint, Codacy Static Code Analysis
        ├─ strict: true (must be up to date)
        ├─ non_fast_forward + deletion: blocked
        └─ auto-merge workflow (squash) merges when all pass
```

## 1. GitHub Actions workflow (.github/workflows/ci.yml)

Five jobs on `ubuntu-latest`; cache with sccache + Swatinem/rust-cache.

| Job | Command | Notes |
|---|---|---|
| `fmt` | `cargo fmt --all -- --check` | rustfmt component only |
| `clippy` | `cargo clippy --all-targets -- -D warnings` | clippy component; install `mold` linker |
| `build` | `cargo build --release` | needs: build for downstream |
| `test` | `cargo nextest run --workspace --lib --tests -j 2` | needs: build; install nextest via curl |
| `self-lint` | `lint-arwaky-cli check .` == 0 violations | needs: build; also assert ≥24 AES codes on `workspaces-bad` |

Key env: `CARGO_INCREMENTAL=0` (sccache), `RUSTC_WRAPPER=sccache`,
`CARGO_BUILD_JOBS=4`, `RUST_MIN_STACK=33554432`.

### Self-lint job (architecture enforcement)

This is the differentiator: the repo lints **itself** with its own
architecture rules (AES 7-layer system). CI fails if the codebase violates
its own rules:

```bash
output=$(./target/debug/lint-arwaky-cli check . 2>&1)
violations=$(echo "$output" | grep -oP 'Total:\s*\K\d+' || echo "0")
[ "${violations}" = "0" ]
```

## 2. Branch protection: GitHub Ruleset

Create a ruleset "Protect main - quality gates" (Settings → Rules →
Rulesets), `target: branch`, enforcement `active`:

- **required_status_checks**: Format, Clippy, Build, Tests, Self-Lint,
  Codacy Static Code Analysis — with `strict: true` (requires up-to-date).
- **non_fast_forward**: blocked.
- **deletion**: blocked.

Effect: no direct push to `main`; every change must go through a PR that
passes all gates. (This is why all work here uses worktrees + PRs.)

## 3. Auto-merge (.github/workflows/auto-merge.yml)

Squash-merges a PR to `main` once required checks pass:

```yaml
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  enable-auto-merge:
    if: base.ref == 'main' && !draft && !merged
    steps:
      - run: gh pr merge "$PR_NUMBER" --auto --squash --subject "$PR_TITLE"
```

Use env vars (not inline interpolation) to avoid command injection through
PR metadata.

## 4. AI review layers

### CodeRabbit (app.coderabbit.ai)

GitHub App; no repo config file required (config via UI or repo-wide).
Reviews every PR: walkthrough, actionable inline comments, pre-merge
checks (docstring coverage, title/description), "finishing touches".
Re-review runs on every push.

### Codacy Static Code Analysis

- Add repo at app.codacy.com → analysis runs on each PR.
- **Required check**: `Codacy Static Code Analysis` (add to ruleset).
- Exclusions in `.codacy.yaml` (fixture/test code must not skew metrics):

```yaml
exclude_paths:
- "workspaces-bad/**"
- "workspaces-good/**"
- "crates/*/tests/**"
- "scripts/**"
- "tools/**"
```

- Custom AI instructions in `.github/instructions/codacy.instructions.md`
  (org/repo defaults, run `codacy_cli_analyze` after edits, trivy after
  dependency changes).

### cubic AI reviewer (www.cubic.dev)

GitHub App; runs "ultrareview" on each PR with P1/P2/P3 findings.
No repo config file in this project (UI-configured).

### Repowise PR bot (.repowise/bot.yaml)

Health gate on PRs (advisory): code-health of changed files, change risk,
dead-code findings, blast radius. Config:

```yaml
bot:
  enabled: true
  comment_mode: on_signal
  ignore_paths:   # mirror .codacy.yaml + .repowiseIgnore
    - "workspaces-bad/**"
    - "workspaces-good/**"
    - "crates/*/tests/**"
    - "scripts/**"
    - "tools/**"
    - "*.md"
  health_gate:
    mode: advisory
    min_new_file_score: 7.0
    block_on_introduced: false
```

Also configure `.repowiseIgnore` (committed) + `.repowise/config.yaml`
exclude_patterns so fixture code is never indexed; `.repowise/health-rules.json`
tunes scoring policy (see the `repowise-scan` skill).

## 5. Local quality gates (scripts/gates.sh)

Reproduce CI locally before pushing — the pre-push hook runs the same
gates. Run:

```bash
bash scripts/gates.sh          # fmt + clippy + self-lint + AES codes + tests
CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --lib --tests
lint-arwaky-cli scan .         # whole repo, must be 0 violations
```

## 6. Supporting workflows

| Workflow | Purpose |
|---|---|
| `release.yml` | Build release binaries, smoke-test, attest-build-provenance |
| `release-tag.yml` | Tag release PRs |
| `auto-release.yml` | Release on workflow_run after merge |
| `cleanup-branches.yml` | Delete merged branches |
| `labeler.yml` | Auto-label PRs (uses actions/labeler) |

## Checklist — replicate on a new repo

- [ ] `.github/workflows/ci.yml` with fmt/clippy/build/test/self-lint jobs
- [ ] Self-lint job runs the project's own linter, fails on violations
- [ ] Ruleset on `main`: required status checks + strict + no fast-forward
- [ ] Auto-merge workflow (squash) gated on the same checks
- [ ] CodeRabbit + Codacy + cubic apps installed on the repo
- [ ] `.codacy.yaml` excludes fixtures; Codacy in required checks
- [ ] `.repowise/bot.yaml` + `.repowiseIgnore` + `health-rules.json`
- [ ] `scripts/gates.sh` mirrors CI for local run + pre-push hook
- [ ] Release/labeler/cleanup workflows if the release train is needed

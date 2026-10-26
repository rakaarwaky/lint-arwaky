# Contributing to Calculator Workspace

Welcome. This document covers the contribution paths below. Only the paths this project actually supports are present.

- [Development Setup](#development-setup)
- [Quality Verification & PR Process](#quality-verification--pr-process)

---

## Principles to Keep in Mind

Before making changes, observe these non-negotiable rules. Each rule states the constraint, not the tool that enforces it:

1. **Layer boundaries**: every change must respect the `layer_concern_role.<ext>` naming convention and the dependency rules of each architectural layer.
2. **Storage / path compliance**: persistent data and caches belong under `target/` or `$HOME/.local/bin/`; never write to `/tmp` for build output a reviewer must find.
3. **Single source of truth**: `Cargo.toml`, `pyproject.toml`, and `package.json` are the definitive manifests; every add/delete must update them.

---

## Development Setup

1. Clone the repository:

   ```bash
   git clone <repo-url>
   cd <repo>
   ```

   Add `--recurse-submodules` only if the project has submodules.

2. Verify host prerequisites:

   ```bash
   rustc --version
   python3 --version
   node --version
   ```

3. Provision the environment:

   ```bash
   CARGO_INCREMENTAL=0 cargo build --release
   ```

4. Verify installation:

   ```bash
   ./target/release/lint-arwaky-cli check .
   ```

---

## Quality Verification & PR Process

Before committing or opening a PR, clear every gate below.

### 1. Run the Verification Commands

```bash
CARGO_INCREMENTAL=0 cargo fmt --all -- --check
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
lint-arwaky-cli check .
```

<One line naming what this runs: test suite, linter, type check, architecture scanner.> CI mirrors this check on every push via `.github/workflows/ci.yml`.

### 2. Conventional Commit Guidelines

| Prefix      | Usage                                      |
| ----------- | ------------------------------------------ |
| `feat:`     | New feature or capability                  |
| `fix:`      | Bug fix                                    |
| `chore:`    | Maintenance, dependency bump, cleanup      |
| `docs:`     | Documentation changes                      |
| `refactor:` | Refactoring without behavioral change      |

### 3. Pull Request Checklist

- [ ] All local verification commands pass.
- [ ] New files follow the project's naming and directory conventions.
- [ ] Any registry or manifest the project keeps is updated.
- [ ] No absolute paths, secrets, or machine-specific values leaked into the diff.
- [ ] The PR description names the contribution path and links the issue.

# AGENTS.md — Lint Arwaky

## Build & dev

```bash
_# Build everything
cargo build --release

# Self-lint
cargo run --bin lint-arwaky-cli -- check

# Scan —
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Run MCP server
cargo run --bin lint-arwaky-mcp

# Run TUI launcher
cargo run --bin lint-arwaky-tui

# Per-crate build/check/test
cargo build -p import_rules_lint_arwaky
cargo check -p naming_rules_lint_arwaky
cargo test -p code_analysis_lint_arwaky

# Tests
cargo test --workspace        # all
cargo test -p import_rules_lint_arwaky    # single crate
cargo test --lib <name_fragment>  # single test by name

# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo clippy -p import_rules -- -D warnings  # per crate
```

## Architecture (6-layer AES + Vertical Slicing + Multi-Crate Workspace)

The codebase uses **7 architectural layers** as **file prefixes**, organized into **feature module** (vertical slicing) in a **Cargo workspace**.

| Layer (prefix)    | Allowed suffixes                                                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant`                                                                        |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                                                                                       |
| `capabilities_`   | `_checker`, `_analyzer`, `_processor`, etc.                                                                              |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc.                                                                                |
| `agent_`          | `_orchestrator`                                                                                                          |
| `surface_`        | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen` |
| `root_`           | `_container`, `_entry`                                                                                                   |

### Workspace Packages Strcuture

```
crates/
  shared/               — Foundation: ALL taxonomy_ + contract_
  import-rules/         — Import compliance checks
  naming-rules/         — Naming convention enforcement
  role-rules/           — Role-layer violation checks
  orphan-detector/      — Unreachable/dead component detection
  code-analysis/        — Code quality: file limits, bypasses, mandatory defs
  auto-fix/             — Auto-fix processor
  config-system/        — Config loading & parsing
  external-lint/        — Python, JS, Rust external linter adapters
  file-watch/           — File watching
  git-hooks/            — Git hooks management
  maintenance/          — Maintenance operations
  project-setup/        — Project init, doctor, mcp-config
  cli-commands/         — CLI surfaces (_command)
  mcp-server/           — MCP server surfaces
  tui/                  — TUI Interface
  root_cli_main_entry.rs       — CLI binary entry (root_entry)
  root_mcp_main_entry.rs       — MCP binary entry (root_entry)
  root_tui_main_entry.rs       — TUI binary entry (root_entry)
```

## Testing with test projects

```bash
cargo run --bin lint-arwaky-cli -- scan test-workspaces
```

Each contains intentional violations. See `TEST.md` for pass/fail criteria.

### Configuration & Rules

| File                                 | Purpose                                    |
| ------------------------------------ | ------------------------------------------ |
| `Cargo.toml` (root)                  | Workspace manifest — members, deps, bins   |
| `crates/*/Cargo.toml`                | Per-crate manifests                        |
| `lint_arwaky.config.rust.yaml`       | AES rules config for Rust                  |
| `lint_arwaky.config.python.yaml`     | AES rules config for Python                |
| `lint_arwaky.config.javascript.yaml` | AES rules config for JavaScript/TypeScript |

### Documentation

| File              | Purpose                                  |
| ----------------- | ---------------------------------------- |
| `RULES_AES.md`    | Complete 24 AES rules catalog            |
| `RULES_RUFF.md`   | Python Ruff rule mapping                 |
| `RULES_MYPY.md`   | Python MyPy rule mapping                 |
| `RULES_BANDIT.md` | Python Bandit rule mapping               |
| `RULES_RADON.md`  | Python Radon complexity rules            |
| `ARCHITECTURE.md` | AES architecture specification (7 layer) |
| `PRD.md`          | Product Requirements Document            |
| `CHANGELOG.md`    | Release history                          |
| `CONTRIBUTING.md` | Contribution guide                       |
| `DEPLOY.md`       | Deployment guide                         |
| `SKILL.md`        | MCP skill documentation for AI agents    |
| `TEST.md`         | Test project pass/fail criteria          |
| `LICENSE`         | MIT License                              |

### Scripts

| File                | Purpose                                                                                  |
| ------------------- | ---------------------------------------------------------------------------------------- |
| `install.local.sh`  | Bumps patch version + builds release + installs 3 binaries (`cli`, `mcp`, `tui`) locally |
| `install.remote.sh` | Remote/CI install script                                                                 |

## Branch management (CRITICAL — must follow)

Allowed branch naming:

- `main`
- `develop`
- `features/<name>`
- `fix/<name>`

When merging a PR to develop:

- Use `gh pr merge <num> --squash` ONLY
- ** use `--delete-branch`** — for feature branches after merger 
- but do  NOT be deleted `develop` branch after merge to main 
- Branches that were accidentally deleted must be restored immediately via:
  ```bash
  git branch <branch-name> origin/develop
  jj git import
  jj bookmark set <branch-name>
  ```

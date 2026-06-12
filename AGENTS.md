# AGENTS.md — Lint Arwaky

## Build & dev

```bash
# Build everything
cargo build --release

# Self-lint — project audits itself under AES rules (no path needed)
cargo run --bin lint-arwaky-cli -- check .

# Scan — lint external/test project (runs AES + clippy/ruff/eslint/tsc/...)
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Run MCP server (JSON-RPC 2.0 over stdin/stdout)
cargo run --bin lint-arwaky-mcp

# Run TUI launcher (interactive terminal UI)
cargo run --bin lint-arwaky-tui

# Per-crate build/check/test
cargo build -p import_rules
cargo check -p naming_rules
cargo test -p code_analysis

# Tests
cargo test --workspace        # all
cargo test -p import_rules    # single crate
cargo test --lib <name_fragment>  # single test by name

# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo clippy -p import_rules -- -D warnings  # per crate
```

## Architecture (6-layer AES + Vertical Slicing + Multi-Crate Workspace)

The codebase uses **7 architectural layers** as **file prefixes**, organized into **feature crates** (vertical slicing) in a **Cargo workspace**.

**CRITICAL**: Layers are determined by **file prefix** (`taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`, `root_`), NOT by folder location or crate name. Each feature crate contains files from multiple layers internally.

| Layer (prefix)    | Allowed suffixes                                  |
| ----------------- | ------------------------------------------------- |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                |
| `capabilities_`   | `_checker`, `_analyzer`, `_processor`, etc.       |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc.         |
| `agent_`          | `_orchestrator`                                   |
| `surface_`        | `_command`, `_handler`, `_controller`             |
| `root_`           | `_container`, `_entry`                            |

### Workspace Crates (feature folders → workspace members)

```
crates/
  shared/               — Foundation: ALL taxonomy_* + contract_* (NO deps on feature crates)
  import-rules/         — Import compliance (AES001, AES002)
  naming-rules/         — Naming convention (AES010, AES011)
  role-rules/           — Role violations (AES0305, AES0306)
  orphan-detector/      — Orphan code detection (AES030)
  code-analysis/        — Quality: unused (AES023), class/line, auto-fix
  auto-fix/             — Auto-fix processor (AES0303)
  config-system/        — Config loading & parsing
  pipeline-jobs/        — Jobs, dispatcher, execution
  source-parsing/       — Source code parsing (scanners, parsers)
  language-adapters/    — Python, JS, Rust linter adapters
  file-system/          — File system abstraction
  file-watch/           — File watching
  git-hooks/            — Git hooks management
  multi-project/        — Multi-project governance
  project-setup/        — Project init, doctor, mcp-config
  plugin-system/        — Plugin discovery & management
  output-report/        — Output formatting & report generation
  lifecycle-state/      — Agent lifecycle management
  metrics-service/      — Metrics provider
  cli-commands/         — CLI surfaces (_command) + transport
  mcp-server/           — MCP server surfaces
  root_compsotion_container.rs — Root composition (root layer)
  root_cli_main_entry.rs       — CLI binary entry (root_entry)
  root_mcp_main_entry.rs       — MCP binary entry (root_entry)
  root_tui_main_entry.rs       — TUI binary entry (root_entry)
```

**Container Pattern**: Each feature crate owns its own `root_container.rs` at crate root. Containers wire `capabilities_*`, `infrastructure_*`, `agent_*` implementations behind `contract_*` traits. Agent layer contains ONLY orchestrators (`agent_*_orchestrator.rs`). Root layer contains containers and binary entries. **Folder structure ≠ layer assignment.**

### Dependency Graph (enforced by Cargo workspace)

```
shared (taxonomy_*, contract_*)     ◄── foundation, NO deps
       ▲
       │
import-rules, naming-rules, role-rules, code-analysis,
auto-fix, orphan-detector, config-system, source-parsing,
language-adapters, file-system, file-watch, git-hooks,
multi-project, project-setup, plugin-system, output-report,
lifecycle-state, metrics-service, pipeline-jobs
       ▲                         (capabilities_*/infrastructure_* + agent_*)
       │                         deps: shared ONLY
cli-commands, mcp-server          (surface_*)
       ▲                         deps: all feature crates + shared
root_compsotion_container (root_*)
```

Import flow: `surface_` → `agent_` → `capabilities_` / `infrastructure_` → `contract_` → `taxonomy_`.
Surfaces must NOT import infrastructure/capabilities directly — they go through feature crate's `root_container` or `ServiceContainerAggregate` trait (AES001 sub-condition).

AES rules enforced: 27 codes across 4 groups (Layer & Import, Naming & Structure, File & Content, Role Violations). See `RULES_AES.md` for the complete catalog with old-to-new mapping.

## Key conventions

- Filenames: `[layer]_[concept(s)]_[suffix].rs` — flexible word count (AES010)
- Every logic file must define a struct implementing a contract trait (AES011, AES014)
- `#[allow(...)]`, `unwrap()`, `panic!` are forbidden (AES022)
- `_constant` files may only contain `pub const` / `pub static` (AES015)
- No bypass: `noqa`, `type: ignore` flagged
- Score: starts at 100, deducts per finding. Any CRITICAL = fail regardless.

## Testing with test projects

```bash
cargo run --bin lint-arwaky-cli -- scan test-project-rust/
cargo run --bin lint-arwaky-cli -- scan test-project-python/
cargo run --bin lint-arwaky-cli -- scan test-project-javascript/
```

Each contains intentional violations. See `TEST.md` for pass/fail criteria.

## Graph-It-Live — Dependency Graph Monitoring

Graph-It-Live is integrated via MCP (`graph-it serve`) untuk visualisasi dan analisis dependency graph secara real-time. Gunakan untuk memonitor arsitektur sampai production ready.

### Tool MCP yang tersedia

| Tool             | Fungsi                                            | Contoh Use Case                                |
| ---------------- | ------------------------------------------------- | ---------------------------------------------- |
| `path-in <file>` | Cari semua file yang import file tertentu         | Cek siapa saja yang import suatu contract port |
| `trace <sym>`    | Trace execution flow: `path/file.rs#FunctionName` | Lacak alur dari surface sampai infrastructure  |
| `explain <file>` | Analisis intra-file call hierarchy                | Pahami struktur internal suatu file            |
| `path <file>`    | Crawl dependency graph dari entry file            | Graph seluruh project dari entry point         |
| `cycles <file>`  | Deteksi circular dependencies                     | Pastikan tidak ada cycle antar layer           |
| `scan`           | Re-index workspace setelah perubahan              | Update index setelah pull/merge                |

### Production Readiness Checklist (pakai Graph-It-Live)

**Setiap sebelum merge ke `develop`:**

```bash
# 1. Self-lint — pastikan 0 CRITICAL
cargo run --bin lint-arwaky-cli -- check .

# 2. Circular dependency check — random sample dari tiap layer
#    Panggil tool cycles untuk file di tiap layer:
#    - cycles crates/source-parsing/contract_parser_port.rs
#    - cycles crates/naming-rules/contract_naming_runner_aggregate.rs
#    - cycles crates/role-rules/src/role_container.rs

# 3. Verify layer boundary — pastikan surface tidak import infra langsung
#    Panggil tool path-in untuk file infrastructure:
#    - path-in crates/language-adapters/infrastructure_js_naming.rs
#    Harusnya di-import hanya oleh root/container files

# 4. Orphan check — cari file yang tidak di-import siapa pun
#    Panggil tool path-in untuk file yang mencurigakan:
#    - path-in crates/orphan-detector/capabilities_orphan_analyzer.rs
#    Pastikan setiap file punya minimal 1 incoming reference

# 5. Build + test
cargo build --release
cargo test --workspace
cargo clippy --all-targets -- -D warnings

# 6. Scan test projects — pastikan violation count tidak turun drastis
cargo run --bin lint-arwaky-cli -- scan test-project-rust/
cargo run --bin lint-arwaky-cli -- scan test-project-python/
cargo run --bin lint-arwaky-cli -- scan test-project-javascript/
```

### Workflow Harian

1. **Sebelum coding**: `scan` untuk refresh index
2. **Saat coding**: `path-in <file>` untuk cek impact sebelum refactor
3. **Sebelum commit**: `cycles <file>` untuk cek circular deps
4. **Sebelum PR**: jalankan production readiness checklist di atas

### Contoh Resolusi Masalah via Graph-It-Live

| Problem                  | Graph-It-Live Command              | What to check                                                |
| ------------------------ | ---------------------------------- | ------------------------------------------------------------ |
| Surface import violation | `path-in surfaces/...`             | Pastikan surface hanya import taxonomy + contract*aggregate* |
| Missing inheritance      | `trace capabilities/...#IAnalyzer` | Lacak siapa yang implement IAnalyzer protocol                |
| Dead code                | `path-in suspicious_file.rs`       | Jika 0 incoming references, file tidak dipakai               |
| Circular dependency      | `cycles <file>`                    | Identifikasi cycle, extract shared logic ke lower layer      |
| Layer boundary leak      | `path-in infrastructure/...`       | Pastikan hanya di-import oleh container (root layer)        |

## Project Files & Directories

### Configuration & Rules

| File                                 | Purpose                                           |
| ------------------------------------ | ------------------------------------------------- |
| `Cargo.toml` (root)                  | Workspace manifest — members, deps, bins          |
| `crates/*/Cargo.toml`                | Per-crate manifests                               |
| `lint_arwaky.config.rust.yaml`       | AES rules config for Rust                         |
| `lint_arwaky.config.python.yaml`     | AES rules config for Python                       |
| `lint_arwaky.config.javascript.yaml` | AES rules config for JavaScript/TypeScript        |

### Documentation

| File              | Purpose                                  |
| ----------------- | ---------------------------------------- |
| `RULES_AES.md`    | Complete 27 AES rules catalog (v2.0)     |
| `RULES_RUFF.md`   | Python Ruff rule mapping                 |
| `RULES_MYPY.md`   | Python MyPy rule mapping                 |
| `RULES_BANDIT.md` | Python Bandit rule mapping               |
| `RULES_RADON.md`  | Python Radon complexity rules            |
| `ARCHITECTURE.md` | AES architecture specification (6 layer) |
| `PRD.md`          | Product Requirements Document            |
| `CHANGELOG.md`    | Release history                          |
| `CONTRIBUTING.md` | Contribution guide                       |
| `DEPLOY.md`       | Deployment guide                         |
| `SKILL.md`        | MCP skill documentation for AI agents    |
| `TEST.md`         | Test project pass/fail criteria          |
| `LICENSE`         | MIT License                              |

### Scripts

| File                              | Purpose                                                                                  |
| --------------------------------- | ---------------------------------------------------------------------------------------- |
| `install.local.sh`                | Bumps patch version + builds release + installs 3 binaries (`cli`, `mcp`, `tui`) locally |
| `install.remote.sh`               | Remote/CI install script                                                                 |
| `scripts/install_graphit_live.sh` | Build + install Graph-It-Live extension                                                  |

### Project Directories

| Directory                  | Purpose                                             |
| -------------------------- | --------------------------------------------------- |
| `crates/`                  | Source code — 26 workspace crates, 6 layers         |
| `test-project-rust/`       | Test project with intentional violations (Rust)     |
| `test-project-python/`     | Test project with intentional violations (Python)   |
| `test-project-javascript/` | Test project with intentional violations (JS/TS)    |
| `Graph-It-Live-arwaky/`    | Graph-It-Live fork — dependency graph visualization |
| `scripts/`                 | Build, install, and utility scripts                 |

## VCS: jj (Jujutsu) — always use instead of git

This repo uses `jj` (Jujutsu) as its VCS. Do NOT use `git` directly.

```bash
jj status            # working copy state
jj log               # commit history
jj describe          # edit commit message
jj new               # create new change on top
jj squash            # merge changes into parent
jj bookmark list     # list bookmarks (branches)
jj bookmark set <name>  # create/update a bookmark
jj git push          # push bookmarks to remote
jj git fetch         # fetch from remote
```

`jj` interoperates with the git remote — the `.jj/` directory replaces `.git/` for local operations, but `jj git push` pushes to the configured git remote.

## Branch management (CRITICAL — must follow)

Allowed branch naming:

- `main`
- `develop`
- `features/<name>` (plural `features/`, NOT `feature/`)

When merging a PR to develop:

- Use `gh pr merge <num> --squash` ONLY
- **NEVER use `--delete-branch`** — feature branches must NOT be deleted after merge
- Branches that were accidentally deleted must be restored immediately via:
  ```bash
  git branch <branch-name> origin/develop
  jj git import
  jj bookmark set <branch-name>
  ```

<h1 align="center">Lint Arwaky</h1>
  <p align="center">
    <strong>Architecture linter enforcement for Rust, Python, and TypeScript.</strong>
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-3.6.0-blue" alt="version" />
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange" alt="rust" />
  <img src="https://img.shields.io/badge/license-MIT-green" alt="license" />
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macOS-lightgrey" alt="platform" />
  <img src="https://img.shields.io/badge/languages-rust%20%7C%20python%20%7C%20typescript-blueviolet" alt="languages" />
</p>

---

Your codebase is not a draft. It is the product your team ships, the system
your users depend on, and the asset your company builds on. Yet every day,
silent architecture decay erodes it — dead imports, broken layer boundaries,
orphan files, naming inconsistencies, and role violations that no one notices
until they cause an outage or block a release.

**Lint Arwaky stops that decay before it starts.**

Built in Rust for speed, structured by the
[Agentic Engineering System](ARCHITECTURE.md) specification, and designed to
enforce architecture the way a compiler enforces syntax — Lint Arwaky does not
ask you to care about code quality. It makes it impossible to ignore.

---

## Table of Contents

- [Why This Exists](#why-this-exists)
- [Features](#features)
- [Supported Languages](#supported-languages)
- [Quick Start](#quick-start)
- [Integrate into Your Project (CI Gates)](#integrate-into-your-project-ci-gates)
- [Commands](#commands)
- [AES Rules (24)](#aes-rules-24)
- [Exit Codes](#exit-codes)
- [Configuration](#configuration)
- [MCP Server](#mcp-server)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Performance](#performance)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

---

## Why This Exists

Most linters catch syntax errors or style mistakes. They do not catch:


| Problem                | What happens                                                 | What Lint Arwaky does                                            |
| ------------------------ | -------------------------------------------------------------- | ------------------------------------------------------------------ |
| **Architecture drift** | Files import from layers they should never touch             | Flags every forbidden cross-layer import (AES201)                |
| **Orphan code**        | Dead files silently grow in your repository                  | Traces every import, flags unreachable files (AES501–506)       |
| **Role confusion**     | Agents write business logic, surfaces depend on lower layers | Enforces per-layer responsibility rules (AES401–406)            |
| **Import chaos**       | Circular dependencies, unused modules, dummy imports         | Detects cycles, unused imports, AI-generated stubs (AES203–205) |
| **Naming decay**       | Inconsistent naming breaks layer detection                   | Enforces`prefix_concept_suffix` convention (AES101–102)         |
| **Bypass culture**     | `unwrap()`, `#[allow(...)]`, `noqa`, `FIXME` accumulate      | Zero tolerance — every suppression is flagged (AES304)          |

---

## Features

- **24 AES rules** across 5 groups — naming, import, quality, role, orphan
- **Multi-language** — Rust, Python, TypeScript/JavaScript in a single scan
- **Full AST parsing** — tree-sitter for all languages, zero regex
- **External linter bridge** — Clippy, Rustfmt, cargo-audit, Ruff, MyPy, Bandit, ESLint, Prettier, tsc unified into one report
- **Auto-fix** — remove + replace + rename with `--dry-run` preview
- **MCP server** — 5 tools for AI agent integration with full CLI parity
- **CI gates** — stable exit codes, threshold enforcement, SARIF/JUnit/JSON reports
- **Git hooks** — pre-commit architecture enforcement
- **Watch mode** — continuous linting on file changes
- **Self-auditing** — the project lints itself under its own rules

---

## Supported Languages


| Language   | Extensions    | AST Parser             | External Linters             |
| ------------ | --------------- | ------------------------ | ------------------------------ |
| Rust       | `.rs`         | tree-sitter-rust       | Clippy, Rustfmt, cargo-audit |
| Python     | `.py`         | tree-sitter-python     | Ruff, MyPy, Bandit           |
| TypeScript | `.ts`, `.tsx` | tree-sitter-typescript | ESLint, Prettier, tsc        |
| JavaScript | `.js`, `.jsx` | tree-sitter-javascript | ESLint, Prettier             |

---

## Quick Start

### Option 1: Remote Install (pre-built binary)

```bash
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/scripts/install.remote.sh | bash
```

### Option 2: Build from Source

```bash
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky
bash scripts/install.local.sh
```

Requires Rust 1.85.0 and Cargo.

### Verify

```bash
lint-arwaky-cli version
# Lint Arwaky v3.6.0
```

### First Scan

```bash
# Scan entire workspace
lint-arwaky-cli scan .

# Scan with CI exit codes
lint-arwaky-cli ci . --threshold 0

# Preview auto-fixes without applying
lint-arwaky-cli fix . --dry-run
```

---

## Integrate into Your Project (CI Gates)

Lint Arwaky is designed to be dropped into **any** Rust, Python, or
TypeScript project as an architecture-enforcement CI gate.

### 1. Project layout

The scanner only analyzes code inside the standard workspace member
directories. Structure your project accordingly (create them if missing):

```text
your-project/
├── crates/     # Rust members
├── packages/   # TypeScript / JavaScript members
├── modules/    # Python members
├── lint_arwaky.config.yaml   # generated by `init`
└── ...
```

### 2. Install the binary

```bash
# Pre-built binary (Linux x86_64)
curl -fsSL https://github.com/rakaarwaky/lint-arwaky/releases/latest/download/lint-arwaky-latest-linux-x86_64.tar.gz -o /tmp/lint-arwaky.tar.gz
sudo tar -xzf /tmp/lint-arwaky.tar.gz -C /usr/local/bin lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui
sudo chmod +x /usr/local/bin/lint-arwaky-*

# Or install from source
cargo install --git https://github.com/rakaarwaky/lint-arwaky.git
```

### 3. Initialize config + adapters

```bash
lint-arwaky-cli init      # creates lint_arwaky.config.yaml
lint-arwaky-cli install   # installs external linter deps (clippy, ruff, bandit, eslint, ...)
lint-arwaky-cli doctor    # verify toolchain health
```

### 4. Add the CI gate (GitHub Actions)

Create `.github/workflows/ci.yml`:

```yaml
name: CI
on:
  push: { branches: [main, develop] }
  pull_request: { branches: [main, develop] }

jobs:
  lint-arwaky:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - name: Install lint-arwaky (latest release)
        run: |
          curl -fsSL https://github.com/rakaarwaky/lint-arwaky/releases/latest/download/lint-arwaky-latest-linux-x86_64.tar.gz -o /tmp/lint-arwaky.tar.gz
          sudo tar -xzf /tmp/lint-arwaky.tar.gz -C /usr/local/bin lint-arwaky-cli lint-arwaky-mcp lint-arwaky-tui
          sudo chmod +x /usr/local/bin/lint-arwaky-*
          lint-arwaky-cli version
      - name: Architecture scan
        run: lint-arwaky-cli check .          # exit 1 on any violation
      - name: Rule engine still fires
        run: |
          codes=$(lint-arwaky-cli scan workspaces-bad 2>&1 \
            | grep -oP "AES\d+" | sort -u | wc -l)
          [ "${codes:-0}" -ge 24 ]            # guard against silent no-ops
```

Add the required status check `lint-arwaky` to your branch protection
ruleset, and add `lint-arwaky-cli ci . --threshold <score>` to your
release pipeline for score-based gating.

See [DEPLOY.md](DEPLOY.md) for the full deployment checklist and
[.agents/skills/setup-ci-quality-gates/SKILL.md](.agents/skills/setup-ci-quality-gates/SKILL.md)
for the complete quality-gate blueprint (rulesets, auto-merge, AI review).

---

## Commands

### Core Analysis


| Command                       | Description                                 |
| ------------------------------- | --------------------------------------------- |
| `lint-arwaky-cli scan [path]` | Run all linters (alias:`check`)             |
| `lint-arwaky-cli fix [path]`  | Apply safe fixes (`--dry-run` to preview)   |
| `lint-arwaky-cli ci [path]`   | CI mode with exit codes (`--threshold <n>`) |

### Individual Linter Surfaces


| Command                           | Rules       | Description                                         |
| ----------------------------------- | ------------- | ----------------------------------------------------- |
| `lint-arwaky-cli naming [path]`   | AES101–102 | Naming convention checks                            |
| `lint-arwaky-cli import [path]`   | AES201–205 | Import boundary checks                              |
| `lint-arwaky-cli quality [path]`  | AES301–305 | Code quality checks                                 |
| `lint-arwaky-cli role [path]`     | AES401–406 | Layer role checks                                   |
| `lint-arwaky-cli orphan <path>`   | AES501–506 | Orphan / unreachable code detection                 |
| `lint-arwaky-cli external [path]` | tool-native | External linter checks (Clippy, Ruff, ESLint, etc.) |

### File Operations


| Command                        | Description                              |
| -------------------------------- | ------------------------------------------ |
| `lint-arwaky-cli watch [path]` | Watch directory and lint on file changes |

### Maintenance


| Command                               | Description                                |
| --------------------------------------- | -------------------------------------------- |
| `lint-arwaky-cli doctor`              | Environment diagnostics (toolchain health) |
| `lint-arwaky-cli security [path]`     | Security vulnerability scan (cargo-audit)  |
| `lint-arwaky-cli dependencies [path]` | Dependency report from manifests           |

### Git Hooks


| Command                          | Description                 |
| ---------------------------------- | ----------------------------- |
| `lint-arwaky-cli install-hook`   | Install git pre-commit hook |
| `lint-arwaky-cli uninstall-hook` | Remove git pre-commit hook  |

### Setup and Config


| Command                       | Description                                    |
| ------------------------------- | ------------------------------------------------ |
| `lint-arwaky-cli init`        | Create default config files                    |
| `lint-arwaky-cli install`     | Install linter adapter dependencies (`--sudo`) |
| `lint-arwaky-cli mcp-config`  | Print MCP server config (`--client <type>`)    |
| `lint-arwaky-cli config-show` | Display active configuration                   |

### Info


| Command                    | Description                      |
| ---------------------------- | ---------------------------------- |
| `lint-arwaky-cli version`  | Display version                  |
| `lint-arwaky-cli adapters` | List active linters and adapters |

---

## AES Rules (24)


| Group       | Rules                                                                                                                                                                        | Count |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------- |
| **Naming**  | AES101 naming convention · AES102 suffix/prefix validation                                                                                                                  | 2     |
| **Import**  | AES201 layer dependency · AES202 mandatory imports · AES203 unused imports · AES204 dummy imports · AES205 circular dependencies                                         | 5     |
| **Quality** | AES301 max lines · AES302 min lines · AES303 mandatory definitions · AES304 bypass detection · AES305 duplicate code                                                     | 5     |
| **Role**    | AES401 taxonomy purity · AES402 contract primitives · AES403 capability implementation · AES404 utility purity · AES405 agent composition · AES406 surface passive role | 6     |
| **Orphan**  | AES501 taxonomy · AES502 contract · AES503 capabilities · AES504 utility · AES505 agent · AES506 surface                                                                | 6     |

External linter results use **tool-native codes** (e.g., `clippy::needless_return`,
`ruff::E501`, `eslint::no-unused-vars`) and are reported alongside AES rules
but are not part of the 24-rule count.

---

## Exit Codes


| Code | Name                 | When                                                             |
| ------ | ---------------------- | ------------------------------------------------------------------ |
| `0`  | Ok                   | Clean scan · doctor finished · dry-run completed               |
| `1`  | Policy fail          | Violations found · CI threshold failed · vulnerabilities found |
| `2`  | Runtime error        | Path missing · pipeline crash · invalid args                   |
| `3`  | Prerequisite missing | Required external tool not installed                             |

---

## Configuration

Configuration is loaded from YAML files with a 5-level priority chain:

```
1. Project root       lint_arwaky.config.yaml
2. Parent dirs        up to 3 levels up
3. XDG user           ~/.config/lint-arwaky/
4. XDG system         /etc/xdg/lint-arwaky/
5. Embedded defaults  compiled into binary
```

### Generate and Inspect

```bash
# Create default config
lint-arwaky-cli init

# Show active config
lint-arwaky-cli config-show
```

---

## MCP Server

Lint Arwaky exposes a [Model Context Protocol](https://modelcontextprotocol.io)
server for AI agent integration with **full CLI parity**.

### 5 Tools


| Tool              | Description                          |
| ------------------- | -------------------------------------- |
| `execute_command` | Run any CLI command with full parity |
| `list_commands`   | List all available commands          |
| `read_skill`      | Read skill/documentation files       |
| `health_check`    | Server and toolchain health          |
| `get_config`      | Retrieve active configuration        |

### Start Server

```bash
cargo run --bin lint-arwaky-mcp
```

### Client Configuration

```bash
# Print config for your MCP client
lint-arwaky-cli mcp-config --client claude
lint-arwaky-cli mcp-config --client cursor
```

See [DEPLOY.md](DEPLOY.md) for full client setup and tool reference.

---

## Architecture

Lint Arwaky follows its own AES (Agentic Engineering System) specification —
a strict 7-layer architecture enforced by its own rules:

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification.

---

## Project Structure

```
lint-arwaky/
├── crates/
│   ├── shared/             # Taxonomy VOs, contracts, utilities
│   ├── config-system/      # Config loading, merging, validation
│   ├── filesystem/         # File walking, AST parsing, graph construction
│   ├── naming-rules/       # AES101–102 naming conventions
│   ├── import-rules/       # AES201–205 import boundaries
│   ├── code-analysis/      # AES301–305 code quality
│   ├── role-rules/         # AES401–406 layer roles
│   ├── orphan-detector/    # AES501–506 orphan detection
│   ├── external-lint/      # External linter adapters (Clippy, Ruff, ESLint, etc.)
│   ├── auto-fix/           # Mechanical fixes (remove + replace + rename)
│   ├── report-formatter/   # text / JSON / SARIF / JUnit output
│   ├── cli-commands/       # CLI surface
│   ├── mcp-server/         # MCP server (5 tools, full CLI parity)
│   ├── git-hooks/          # Pre-commit / git-diff hooks
│   ├── file-watch/         # Continuous lint on file changes
│   ├── project-setup/      # init / install / mcp-config
│   ├── maintenance/        # doctor / security / deps
│   └── tui/                # Interactive terminal UI
├── scripts/
│   ├── install.remote.sh   # Pre-built binary installer
│   └── install.local.sh    # Build-from-source installer
├── PRD.md                  # Product requirements
├── ARCHITECTURE.md         # AES specification
├── DEPLOY.md               # MCP deployment guide
├── CONTRIBUTING.md         # Contribution guidelines
└── README.md               # This file
```

---

## Performance


| Metric                              | Target       |
| ------------------------------------- | -------------- |
| 1,000 files (full pipeline)         | < 5 seconds  |
| 10,000 files (full pipeline)        | < 15 seconds |
| File discovery (1,660 files)        | < 500 ms     |
| AST parsing (1,660 files, parallel) | < 2 s        |
| Graph construction                  | < 200 ms     |

All parsing uses tree-sitter (full AST, all languages). File-level checks
are parallelized via rayon. No async runtime dependency.

---

## Testing

```bash
# Run all tests
cargo test --workspace

# Self-lint (the project lints itself)
cargo run --bin lint-arwaky-cli -- check .

# Run MCP server
cargo run --bin lint-arwaky-mcp

# Acceptance tests follow naming convention:
# tests/acceptance_FR_00N.rs
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Key conventions:

- All code follows the AES 7-layer architecture (enforced by this tool)
- Acceptance tests named `acceptance_FR_00N.rs` per functional requirement
- No `unwrap()`, `#[allow(...)]`, `todo!()`, `FIXME`, or `HACK` in production code
- Full AST parsing only — no regex-based code analysis

---

## License

[MIT](LICENSE)

```

```

---
name: lint-arwaky-rust
description: "Run lint-arwaky CLI scanner and MCP server for Rust projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [rust, lint, aes, compliance, scanning, mcp, clippy]
  triggers:
    - "lint arwaky rust"
    - "lint code rust"
    - "check compliance rust"
    - "scan rust project"
  dependencies: []
  related:
    - cleanup-consolidate-rust
    - build-verify-all
---

# lint-arwaky-rust — Complete Command & Argument Reference

Run linters (`clippy`, `rustfmt`, `lint-arwaky-cli`) and enforce 7-layer Architecture Enforcement System (AES) compliance rules for Rust crates and workspaces.

---

## Shell Aliases

Shortcut aliases are available for fast terminal access (automatically added to `~/.bashrc` / `~/.zshrc`):

| Alias | Target Binary | Description | Example Usage |
| :--- | :--- | :--- | :--- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner | `lac scan .`, `lac fix crates/`, `lac ci` |
| `lat` | `lint-arwaky-tui` | Terminal User Interface (TUI) dashboard | `lat` |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) | Configured in Claude / Cursor / Windsurf |

---

## 1. Global CLI Options

These options apply globally across all `lint-arwaky-cli` subcommands:

| Option | Long Flag | Description |
| :--- | :--- | :--- |
| `-v` | `--verbose` | Enable debug logging and detailed diagnostic traces. |
| `-q` | `--quiet` | Minimize console output (suppress non-error messages). |
| `-o` | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration). |
| | `--filter <CODE>` | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h` | `--help` | Print help information for the CLI or specific subcommand. |
| `-V` | `--version` | Print CLI binary version. |

---

## 2. Complete Commands & Subcommands Reference

### `scan` / `check`
Scans target Rust workspace, discovers workspace members, and runs all linters.

```bash
# Basic scan (defaults to text format)
lint-arwaky-cli scan workspaces-bad/crates

# Scan with specific output format (text | json | sarif | junit)
lint-arwaky-cli scan workspaces-bad/crates --format json

# Scan single workspace member by name
lint-arwaky-cli scan workspaces-bad/crates --member shared

# Filter results by specific AES rule ID
lint-arwaky-cli scan workspaces-bad/crates --filter AES401

# Save reports to custom directory
lint-arwaky-cli scan workspaces-bad/crates --format json --output-dir ~/.local/share/lint-arwaky/reports
```

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to current directory `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: Target single workspace member by package name.
* `--filter <CODE>`: Filter violations by AES rule ID.
* `-o, --output-dir <DIR>`: Output directory path to save report files.

---

### `fix`
Applies safe automatic fixes to compliance violations across the codebase.

```bash
# Apply automatic fixes
lint-arwaky-cli fix crates/

# Preview changes without modifying files (Dry Run)
lint-arwaky-cli fix crates/ --dry-run

# Preview fixes for specific rule code
lint-arwaky-cli fix crates/ --dry-run --filter AES101
```

**Arguments & Flags**:
* `[PATH]`: Target path to fix (defaults to `.`).
* `--dry-run`: Perform a dry run showing diffs without modifying files.
* `--filter <CODE>`: Apply fixes only for a specific AES rule ID.

---

### `ci`
Continuous Integration quality gate mode. Evaluates compliance score against a threshold.

```bash
# CI mode with default threshold
lint-arwaky-cli ci crates/

# CI mode with custom score threshold (exits with status 1 if score < 80)
lint-arwaky-cli ci crates/ --threshold 80 --format junit
```

**Arguments & Flags**:
* `[PATH]`: Target path (defaults to `.`).
* `--threshold <SCORE>`: Minimum acceptable quality score (0–100, default: 80).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).

---

### `quality`, `import`, `naming`, `role`, `orphan`, `external`
Run a single linter independently for targeted analysis.

```bash
# Run only naming rules
lint-arwaky-cli naming crates/

# Run only orphan detection with JSON output
lint-arwaky-cli orphan crates/ --format json

# Run orphan on a specific member
lint-arwaky-cli orphan crates/ --member shared_common

# Run only import rules on a specific path
lint-arwaky-cli import crates/code_analysis

# Run only role rules
lint-arwaky-cli role crates/

# Run only external linters (clippy)
lint-arwaky-cli external crates/

# Run only quality analysis
lint-arwaky-cli quality crates/
```

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: (orphan only) Target specific workspace member.

---

### `security` & `dependencies`
Scans for security vulnerabilities and library dependency CVEs.

```bash
# Scan code for security issues (Bandit, Cargo Audit, ESLint Security)
lint-arwaky-cli security crates/

# Scan Rust library dependencies for vulnerabilities
lint-arwaky-cli dependencies crates/
```

---

### `watch`
Monitors file system changes and re-runs linting automatically upon file save.

```bash
# Watch directory and re-lint on changes
lint-arwaky-cli watch crates/
```

---

### `install-hook` & `uninstall-hook`
Manages Git pre-commit hook integration.

```bash
# Install git pre-commit hook
lint-arwaky-cli install-hook

# Uninstall git pre-commit hook
lint-arwaky-cli uninstall-hook
```

---

### `init` & `install`
Initializes workspace configuration and installs linter adapter dependencies.

```bash
# Create default lint_arwaky.config.yaml in workspace
lint-arwaky-cli init

# Install required external linter tools (clippy, rustfmt, etc.)
lint-arwaky-cli install
```

---

### `config-show`, `adapters`, & `mcp-config`
Displays workspace configuration and active integrations.

```bash
# Show active configuration tokens and rules
lint-arwaky-cli config-show

# List all active linter adapters (Clippy, Rustfmt, etc.)
lint-arwaky-cli adapters

# Print MCP server configuration JSON for AI client integration
lint-arwaky-cli mcp-config
```

---

### `doctor` & `version`
Environment diagnostic tools.

```bash
# Health check for Rust tooling and environment
lint-arwaky-cli doctor

# Display binary version information
lint-arwaky-cli version
```

---

## MCP Server Tools Reference (`lint-arwaky-mcp`)

`lint-arwaky-mcp` exposes 5 JSON-RPC 2.0 tools over STDIO for AI clients (Claude Code, Cursor, Windsurf, Hermes):

| Tool Name | Description | Arguments / Parameters |
| :--- | :--- | :--- |
| `execute_command` | Execute any CLI command action | `action` (required: `"scan"`, `"check"`, `"fix"`, `"security"`, `"doctor"`, etc.), `args` (optional JSON object, e.g. `{"path": "/abs/path"}`) |
| `list_commands` | List available CLI commands catalog | `domain` (optional: filter by domain string, e.g. `"setup"`, `"check"`) |
| `read_skill` | Read `SKILL.md` documentation by section | `section` (optional: header name to extract) |
| `health_check` | Check MCP server & adapter health | None (0 parameters) |
| `get_config` | Get active architecture config | `path` (optional project path), `language` (optional: `"rust"`, `"python"`, `"javascript"`) |

### Example MCP JSON-RPC Payload

```json
// execute_command: run Rust scan
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"execute_command","arguments":{"action":"scan","args":{"path":"workspaces-bad/crates"}}}}

// health_check: check Rust adapters (clippy)
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"health_check","arguments":{}}}

// get_config: retrieve Rust architecture configuration
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_config","arguments":{"language":"rust"}}}
```

---

## 3. Native Rust Tooling Commands

```bash
# Auto-format Rust code
cargo fmt --all

# Check Clippy lints
cargo clippy --all-targets -- -D warnings

# Per-crate build/check/test
cargo check -p <crate-name>
cargo test -p <crate-name>
cargo test --workspace
```

---

## 4. Report Redirection & XDG Storage

Output can be saved directly to the XDG `reports` directory (`~/.local/share/lint-arwaky/reports/`):

```bash
# Save JSON report
lint-arwaky-cli scan crates/ --format json > ~/.local/share/lint-arwaky/reports/scan_rust.json

# Save SARIF report for GitHub Code Scanning
lint-arwaky-cli scan crates/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_rust.sarif
```

---

## 5. Verification Checklist

- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test --workspace` passes
- [ ] `lint-arwaky-cli scan .` reports 0 violations

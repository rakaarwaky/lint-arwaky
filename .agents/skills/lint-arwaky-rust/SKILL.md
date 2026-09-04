---
name: lint-arwaky-rust
description: "Run lint-arwaky CLI scanner and MCP server for Rust projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [rust, lint, aes, compliance, scanning, mcp, clippy, fix]
  triggers:
    - "lint arwaky rust"
    - "lint code rust"
    - "check compliance rust"
    - "scan rust project"
    - "fix architecture violations rust"
    - "fix violations rust"
    - "scan and fix rust"
    - "audit rust codebase"
  dependencies: []
  related:
    - cleanup-consolidate-rust
    - fix-bypass-rust
    - create-taxonomy-rust
    - create-contract-rust
    - create-utility-rust
    - create-capabilities-rust
    - create-agent-rust
    - create-surface-rust
    - create-root-rust
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

---

## 6. Scan → Diagnose → Fix → Verify Workflow

This is the **end-to-end workflow** for an AI agent when asked to fix architecture violations in a Rust codebase.

### 6.1 Pre-flight

```bash
# Ensure project builds
CARGO_INCREMENTAL=0 cargo build -p <crate> 2>&1 | tail -20
```

If build fails, fix compilation errors first. Violations on broken code are unreliable.

### 6.2 Scan (detect all violations)

```bash
# Full scan — get total count and breakdown by rule
lint-arwaky-cli scan <target-path> --format json > /tmp/arwaky-scan.json

# Count total violations
cat /tmp/arwaky-scan.json | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"Total: {d.get('total_violations', len(d.get('violations', [])))}\")"

# Filter by severity — focus on CRITICAL first (AES201, AES205, AES304)
lint-arwaky-cli scan <target-path> --filter AES201
lint-arwaky-cli scan <target-path> --filter AES304
```

**Priority order for fixes:**
1. 🔴 **CRITICAL**: AES201 (forbidden import), AES205 (circular import), AES304 (bypass comment)
2. 🟡 **HIGH**: AES101–102 (naming), AES202 (mandatory import), AES301–303 (quality), AES401–403, AES406, AES505–506
3. 🟢 **MEDIUM/LOW**: AES203–204, AES305, AES404–405, AES501–504

### 6.3 Diagnose (understand each violation)

For each violation, read the affected file and determine:

| Question | What to look for |
|---|---|
| **Which rule?** | AES code from scan output (e.g. AES201) |
| **Which layer?** | File prefix: `taxonomy_`, `contract_`, `capabilities_`, etc. |
| **What's wrong?** | Read the violation message and the source file |
| **Can it be auto-fixed?** | AES101 (rename), AES203 (unused import), AES304 (bypass) → yes. AES201 (wrong dependency) → manual. |
| **Root cause?** | Is it a naming issue, a wrong import, a missing implementation, or dead code? |

### 6.4 Fix (apply changes)

**Auto-fixable violations** (use the CLI):

```bash
# Preview first
lint-arwaky-cli fix <target-path> --dry-run

# Fix all auto-fixable violations
lint-arwaky-cli fix <target-path>

# Fix specific rule
lint-arwaky-cli fix <target-path> --filter AES101
lint-arwaky-cli fix <target-path> --filter AES304
```

**Manual fixes** (by violation type):

| Violation | Fix approach | Skill to use |
|---|---|---|
| AES101 (naming) | Rename file to `layer_concern_role.rs` | `create-{taxonomy,contract,capabilities,...}-rust` |
| AES102 (suffix) | Change suffix to match layer rule | `create-{layer}-rust` |
| AES201 (forbidden import) | Remove cross-layer import; use DI via contract | `create-contract-rust` |
| AES202 (mandatory import) | Add the required import | — |
| AES203 (unused import) | Remove unused import line | — |
| AES204 (dummy import) | Remove dummy import + stub usage | `fix-bypass-rust` |
| AES205 (circular import) | Break cycle by extracting to lower layer | `create-contract-rust` |
| AES301 (max lines) | Split file by responsibility | `cleanup-consolidate-rust` |
| AES302 (min lines) | Merge thin file into parent or delete | `cleanup-consolidate-rust` |
| AES303 (mandatory def) | Add struct/enum/trait definition | — |
| AES304 (bypass) | Fix root cause, remove `#[allow]`/`unwrap()` | `fix-bypass-rust` |
| AES305 (duplication) | Extract shared logic to utility | `create-utility-rust` |
| AES401–406 (role) | Move code to correct layer | `create-{layer}-rust` |
| AES501–506 (orphan) | Wire into container or delete dead code | `cleanup-consolidate-rust` |

### 6.5 Verify (confirm all clean)

```bash
# 1. Re-scan — should show 0 violations
lint-arwaky-cli scan <target-path>
lint-arwaky-cli scan .  # self-lint for the lint-arwaky project itself

# 2. Build check
CARGO_INCREMENTAL=0 cargo check -p <crate>

# 3. Tests pass
cargo nextest run -p <crate> --lib --tests

# 4. Format + clippy
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

### 6.6 Commit

```bash
# Stage and commit with descriptive message
git add -A
git commit -m "fix: resolve <N> AES violations (<list of rules>)"
```

---

## 7. Quick Fix Recipes

### Rename file (AES101/102)

```bash
# Before: capabilities_scanner.rs (wrong — no underscore prefix for capabilities)
# After:  capabilities_file_scanner.rs
git mv src/capabilities_scanner.rs src/capabilities_file_scanner.rs
# Update mod.rs references
```

### Remove unused import (AES203)

```bash
# Auto-fixable
lint-arwaky-cli fix src/file.rs --filter AES203
```

### Fix bypass comments (AES304)

```bash
# Find all bypass patterns
grep -rn 'unwrap\|expect\|panic!\|#\[allow' src/

# Fix each one:
# unwrap() → use ? or match
# expect("msg") → use ? with context
# panic!("msg") → return Result::Err
# #[allow(...)] → fix the underlying warning
```

### Remove dead code (AES501–506)

```bash
# Find orphan files
lint-arwaky-cli orphan crates/ --format json

# If orphan is truly dead → delete it
# If orphan should be wired → add to container or import chain
```

### Break circular dependency (AES205)

```bash
# Identify the cycle
lint-arwaky-cli import <target-path> --filter AES205

# Solution: extract shared types/traits to a lower layer (taxonomy or contract)
# Move the shared interface to contract_<concern>_protocol.rs
# Both sides import from contract instead of importing each other
```

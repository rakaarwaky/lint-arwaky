---
name: lint-arwaky
description: Comprehensive terminal commands and argument reference for lint-arwaky-cli
---

# Lint Arwaky CLI — Complete Command & Argument Reference

`lint-arwaky-cli` is the autonomous code quality and 7-layer Architecture Enforcement System (AES) gatekeeper for Rust, Python, and TypeScript projects.

---

## 1. Global Options

These options apply globally across all subcommands:

| Option | Long Flag            | Description                                                                        |
| :----- | :------------------- | :--------------------------------------------------------------------------------- |
| `-v`   | `--verbose`          | Enable debug logging and detailed diagnostic traces.                               |
| `-q`   | `--quiet`            | Minimize console output (suppress non-error messages).                             |
| `-o`   | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration).              |
|        | `--filter <CODE>`    | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h`   | `--help`             | Print help information for the CLI or specific subcommand.                         |
| `-V`   | `--version`          | Print CLI binary version.                                                          |

---

## 2. Commands & Subcommands Reference

### `scan` / `check`

Scans target path or workspace, discovers workspace members, and runs all AES linters.

```bash
# Basic scan (defaults to text output)
lint-arwaky-cli scan [PATH]

# Scan with specific output format (text | json | sarif | junit)
lint-arwaky-cli scan [PATH] --format <FORMAT>

# Scan only a specific workspace member (e.g. shared, import-rules)
lint-arwaky-cli scan [PATH] --member <MEMBER_NAME>

# Filter results by specific rule code
lint-arwaky-cli scan [PATH] --filter AES401

# Save reports to custom directory
lint-arwaky-cli scan [PATH] --format json --output-dir ~/.local/share/lint-arwaky/reports
```

**Arguments & Flags**:

- `[PATH]`: Target path to scan (defaults to current directory `.`).
- `--format <FORMAT>`: Output format. Options: `text` (default), `json`, `sarif`, `junit`.
- `--member <NAME>`: Target single workspace member by package name.
- `--filter <CODE>`: Filter violations by AES rule ID (e.g., `AES101`, `AES401`, `AES504`).
- `-o, --output-dir <DIR>`: Output directory path to save report files.

---

### `fix`

Applies safe automatic fixes to compliance violations across the codebase.

```bash
# Apply automatic fixes
lint-arwaky-cli fix [PATH]

# Preview changes without mutating files (Dry Run)
lint-arwaky-cli fix [PATH] --dry-run

# Preview fixes for specific rule code
lint-arwaky-cli fix [PATH] --dry-run --filter AES101
```

**Arguments & Flags**:

- `[PATH]`: Target path to fix (defaults to `.`).
- `--dry-run`: Perform a dry run showing diffs without modifying files.
- `--filter <CODE>`: Apply fixes only for a specific AES rule ID.

---

### `ci`

Continuous Integration quality gate mode. Evaluates compliance score against a threshold.

```bash
# CI mode with default threshold
lint-arwaky-cli ci [PATH]

# CI mode with custom score threshold (exits with status 1 if score < 80)
lint-arwaky-cli ci [PATH] --threshold 80 --format junit
```

**Arguments & Flags**:

- `[PATH]`: Target path (defaults to `.`).
- `--threshold <SCORE>`: Minimum acceptable quality score (0–100, default: 80). Exits with code 1 if score is below threshold.
- `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).

---

### `orphan`

Checks if a target file is an orphan (AES501–AES506) unreachable from entry points.

```bash
# Check single file for orphan status
lint-arwaky-cli orphan <FILE_PATH>
```

**Arguments & Flags**:

- `<FILE_PATH>`: Relative or absolute path to the target source file.

---

### `security` & `dependencies`

Scans for security vulnerabilities and library dependency CVEs.

```bash
# Scan code for security vulnerabilities (Bandit, Cargo Audit, ESLint Security)
lint-arwaky-cli security [PATH]

# Scan library dependencies for vulnerabilities
lint-arwaky-cli dependencies [PATH]
```

---

### `watch`

Monitors file system changes and re-runs linting automatically upon file save.

```bash
# Watch directory and re-lint on changes
lint-arwaky-cli watch [PATH]
```

---

### `install-hook` & `uninstall-hook`

Manages Git pre-commit hook integration.

```bash
# Install git pre-commit hook to block non-compliant commits
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

# Install required external linter tools (ruff, mypy, clippy, eslint, etc.)
lint-arwaky-cli install
```

---

### `config-show`, `adapters`, & `mcp-config`

Displays workspace configuration and active integrations.

```bash
# Show active configuration tokens and rules
lint-arwaky-cli config-show

# List all active linter adapters (Clippy, Ruff, Mypy, ESLint, etc.)
lint-arwaky-cli adapters

# Print MCP server configuration JSON for AI client integration
lint-arwaky-cli mcp-config
```

---

### `doctor` & `version`

Environment diagnostic tools.

```bash
# Health check for external linter executables and environment
lint-arwaky-cli doctor

# Display binary version information
lint-arwaky-cli version
```

---

## 3. Report Output Formats & Redirection

`lint-arwaky-cli` supports 4 output formats produced by the `report-formatter` layer:

1. **`text`**: Human-readable terminal output.
2. **`json`**: Structured JSON array for programmatic consumption.
3. **`sarif`**: SARIF 2.1.0 JSON format for GitHub Code Scanning / IDE annotations.
4. **`junit`**: JUnit XML format for CI/CD test runners (Jenkins, GitLab CI, GitHub Actions).

### Saving Reports to File

Reports can be saved directly to the XDG `reports` directory (`~/.local/share/lint-arwaky/reports/`) using standard Unix output redirection (`>`) or the `-o` option:

```bash
# Save JSON report to XDG reports directory
lint-arwaky-cli scan --format json > ~/.local/share/lint-arwaky/reports/scan_report.json

# Save SARIF report for GitHub Code Scanning
lint-arwaky-cli scan --format sarif > ~/.local/share/lint-arwaky/reports/scan_report.sarif

# Save JUnit XML report for CI test execution
lint-arwaky-cli scan --format junit > ~/.local/share/lint-arwaky/reports/junit_report.xml

# Using --output-dir option
lint-arwaky-cli scan --format json --output-dir ~/.local/share/lint-arwaky/reports
```

---

## 4. Installation Scripts Reference

Two XDG installation scripts are available in the `scripts/` directory:

### Local Installation (User-level)

Cleans and installs binaries to `~/.cargo/bin` and XDG layout to `~/.config/lint-arwaky` and `~/.local/share/lint-arwaky/reports`:

```bash
bash scripts/install.local.sh
```

### Global Installation (System-wide)

Installs binaries to `/usr/local/bin`, configuration to `/etc/lint-arwaky`, and report data to `/var/lib/lint-arwaky/reports`:

```bash
sudo bash scripts/install.global.sh
```

### Release Pipeline Automation

Executes release build, self-linting, clippy/fmt checks, version bump, Jujutsu (JJ) commit/bookmarking, and crates.io publishing:

```bash
# Full release pipeline with interactive commit & patch bump
bash scripts/release.sh --bump patch

# CI verification checks only (Dry Run / No Commit)
bash scripts/release.sh --ci-only
```

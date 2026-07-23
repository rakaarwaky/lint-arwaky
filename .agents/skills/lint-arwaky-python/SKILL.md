---
name: lint-arwaky-python
description: "Run lint-arwaky CLI scanner and MCP server for Python projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [python, lint, aes, compliance, scanning, mcp]
  triggers:
    - "lint arwaky python"
    - "scan python project"
    - "verify aes compliance python"
  dependencies: []
  related:
    - cleanup-files-python
    - create-capabilities-python
---

# lint-arwaky-python — Complete Command & Argument Reference

Run `lint-arwaky-cli` scanner and MCP server for Python projects. Validates AES (Architecture Error Standards) compliance, checks layer violations, and helps fix architecture issues.

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
Scans target Python workspace, discovers modules, and runs all linters.

```bash
# Basic scan (defaults to text format)
lint-arwaky-cli scan test-workspaces/modules

# Scan with specific output format (text | json | sarif | junit)
lint-arwaky-cli scan test-workspaces/modules --format json

# Filter scan results by rule code (e.g. AES201, AES401)
lint-arwaky-cli scan test-workspaces/modules --filter AES201

# Save reports to custom directory
lint-arwaky-cli scan test-workspaces/modules --format json --output-dir ~/.local/share/lint-arwaky/reports
```

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to current directory `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: Target single workspace member by module name.
* `--filter <CODE>`: Filter violations by AES rule ID.
* `-o, --output-dir <DIR>`: Output directory path to save report files.

---

### `fix`
Applies safe automatic fixes to compliance violations across the codebase.

```bash
# Apply automatic fixes
lint-arwaky-cli fix modules/

# Preview changes without modifying files (Dry Run)
lint-arwaky-cli fix modules/ --dry-run

# Preview fixes for specific rule code
lint-arwaky-cli fix modules/ --dry-run --filter AES101
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
lint-arwaky-cli ci modules/

# CI mode with custom score threshold (exits with status 1 if score < 80)
lint-arwaky-cli ci modules/ --threshold 80 --format junit
```

**Arguments & Flags**:
* `[PATH]`: Target path (defaults to `.`).
* `--threshold <SCORE>`: Minimum acceptable quality score (0–100, default: 80).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).

---

### `orphan`
Checks if a target Python source file is an orphan (AES501–AES506) unreachable from entry points.

```bash
# Check single file for orphan status
lint-arwaky-cli orphan modules/animator/utility_helper.py
```

**Arguments & Flags**:
* `<FILE_PATH>`: Relative or absolute path to the target source file.

---

### `security` & `dependencies`
Scans for security vulnerabilities and library dependency CVEs.

```bash
# Scan code for security issues (Bandit, Cargo Audit, ESLint Security)
lint-arwaky-cli security modules/

# Scan Python library dependencies for vulnerabilities
lint-arwaky-cli dependencies modules/
```

---

### `watch`
Monitors file system changes and re-runs linting automatically upon file save.

```bash
# Watch directory and re-lint on changes
lint-arwaky-cli watch modules/
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

# Install required external linter tools (ruff, mypy, bandit, etc.)
lint-arwaky-cli install
```

---

### `config-show`, `adapters`, & `mcp-config`
Displays workspace configuration and active integrations.

```bash
# Show active configuration tokens and rules
lint-arwaky-cli config-show

# List all active linter adapters (Ruff, Mypy, Radon, Bandit, etc.)
lint-arwaky-cli adapters

# Print MCP server configuration JSON for AI client integration
lint-arwaky-cli mcp-config
```

---

### `doctor` & `version`
Environment diagnostic tools.

```bash
# Health check for Python tooling and environment
lint-arwaky-cli doctor

# Display binary version information
lint-arwaky-cli version
```

---

## 3. Report Redirection & XDG Storage

Output can be saved directly to the XDG `reports` directory (`~/.local/share/lint-arwaky/reports/`):

```bash
# Save JSON report
lint-arwaky-cli scan modules/ --format json > ~/.local/share/lint-arwaky/reports/scan_python.json

# Save SARIF report for GitHub Code Scanning
lint-arwaky-cli scan modules/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_python.sarif
```

---

## 4. AES Rules for Python

### Layer Import Rules (AES201)

```
ALLOWED:    taxonomy_*, contract_*
FORBIDDEN:  capabilities_*, agent_* (peer layers)
```

### Protocol Requirements (AES403)

- Every capability class MUST inherit from protocol ABC
- Every agent class MUST inherit from aggregate ABC

### Layer Boundaries (AES404)

| Layer | Can Contain | Cannot Contain |
| :--- | :--- | :--- |
| capabilities | Pure computation, validation | I/O, network, database |
| agent | Orchestration flow | Computation, I/O, business |

---

## 5. Verification Checklist

- [ ] All layer imports follow AES201 rules
- [ ] All classes inherit appropriate protocol ABCs (AES403)
- [ ] No mixed responsibilities in layers (AES404)
- [ ] No magic constants in layers (AES405)
- [ ] Surface files follow role-based imports (AES406)

---

## 6. Common Issues & Fix Strategies

| Issue | Fix Strategy |
| :--- | :--- |
| Cross-layer imports | Use contract layer protocols via DI |
| Missing protocol inheritance | Create protocol ABC and inherit |
| Mixed layer responsibilities | Move code to appropriate layer |
| Magic constants | Extract to taxonomy constants |
| Surface importing capabilities | Use aggregate contracts instead |

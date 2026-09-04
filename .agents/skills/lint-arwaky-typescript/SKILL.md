---
name: lint-arwaky-typescript
description: "Run lint-arwaky CLI scanner and MCP server for TypeScript projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [typescript, lint, aes, compliance, scanning, mcp, fix]
  triggers:
    - "lint arwaky typescript"
    - "scan typescript project"
    - "verify aes compliance typescript"
    - "fix architecture violations typescript"
    - "fix violations typescript"
    - "scan and fix typescript"
    - "audit typescript codebase"
    - "lint arwaky javascript"
    - "fix violations javascript"
  dependencies: []
  related:
    - cleanup-consolidate-typescript
    - fix-bypass-typescript
    - create-taxonomy-typescript
    - create-contract-typescript
    - create-utility-typescript
    - create-capabilities-typescript
    - create-agent-typescript
    - create-surface-typescript
    - create-root-typescript
---

# lint-arwaky-typescript — Complete Command & Argument Reference

Run `lint-arwaky-cli` scanner and MCP server for TypeScript projects. Validates AES (Architecture Error Standards) compliance, checks layer violations, and helps fix architecture issues.

---

## Shell Aliases

Shortcut aliases are available for fast terminal access (automatically added to `~/.bashrc` / `~/.zshrc`):

| Alias | Target Binary | Description | Example Usage |
| :--- | :--- | :--- | :--- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner | `lac scan .`, `lac fix`, `lac doctor` |
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
Scans target TypeScript workspace, discovers packages, and runs all linters.

```bash
# Basic scan (defaults to text format)
lint-arwaky-cli scan workspaces-bad/packages

# Scan with specific output format (text | json | sarif | junit)
lint-arwaky-cli scan workspaces-bad/packages --format json

# Scan specific package member
lint-arwaky-cli scan workspaces-bad/packages --member animator

# Filter scan results by rule code (e.g. AES201, AES401)
lint-arwaky-cli scan workspaces-bad/packages --filter AES201

# Save reports to custom directory
lint-arwaky-cli scan workspaces-bad/packages --format json --output-dir ~/.local/share/lint-arwaky/reports
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
lint-arwaky-cli fix packages/

# Preview changes without modifying files (Dry Run)
lint-arwaky-cli fix packages/ --dry-run

# Preview fixes for specific rule code
lint-arwaky-cli fix packages/ --dry-run --filter AES101
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
lint-arwaky-cli ci packages/

# CI mode with custom score threshold (exits with status 1 if score < 80)
lint-arwaky-cli ci packages/ --threshold 80 --format junit
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
lint-arwaky-cli naming packages/

# Run only orphan detection with JSON output
lint-arwaky-cli orphan packages/ --format json

# Run orphan on a specific member
lint-arwaky-cli orphan packages/ --member animator

# Run only import rules
lint-arwaky-cli import packages/

# Run only role rules
lint-arwaky-cli role packages/

# Run only external linters (eslint)
lint-arwaky-cli external packages/

# Run only quality analysis
lint-arwaky-cli quality packages/
```

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: (orphan only) Target specific workspace member.

---

### `security` & `dependencies`
Scans for security vulnerabilities and library dependency CVEs.

```bash
# Scan code for security issues (ESLint Security, Bandit, Cargo Audit)
lint-arwaky-cli security packages/

# Scan TypeScript library dependencies for vulnerabilities
lint-arwaky-cli dependencies packages/
```

---

### `watch`
Monitors file system changes and re-runs linting automatically upon file save.

```bash
# Watch directory and re-lint on changes
lint-arwaky-cli watch packages/
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

# Install required external linter tools (eslint, tsc, etc.)
lint-arwaky-cli install
```

---

### `config-show`, `adapters`, & `mcp-config`
Displays workspace configuration and active integrations.

```bash
# Show active configuration tokens and rules
lint-arwaky-cli config-show

# List all active linter adapters (ESLint, TSC, etc.)
lint-arwaky-cli adapters

# Print MCP server configuration JSON for AI client integration
lint-arwaky-cli mcp-config
```

---

### `doctor` & `version`
Environment diagnostic tools.

```bash
# Health check for TypeScript tooling and environment
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
// execute_command: run TypeScript scan
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"execute_command","arguments":{"action":"scan","args":{"path":"workspaces-bad/packages"}}}}

// health_check: check TypeScript adapters (eslint)
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"health_check","arguments":{}}}

// get_config: retrieve TypeScript architecture configuration
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_config","arguments":{"language":"typescript"}}}
```

---

## 3. Report Redirection & XDG Storage

Output can be saved directly to the XDG `reports` directory (`~/.local/share/lint-arwaky/reports/`):

```bash
# Save JSON report
lint-arwaky-cli scan packages/ --format json > ~/.local/share/lint-arwaky/reports/scan_ts.json

# Save SARIF report for GitHub Code Scanning
lint-arwaky-cli scan packages/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_ts.sarif
```

---

## 4. AES Rules for TypeScript

### Layer Import Rules (AES201)

```
ALLOWED:    taxonomy_*, contract_*
FORBIDDEN:  capabilities_*, agent_* (peer layers)
```

### Interface Requirements (AES403)

- Every capability class MUST implement a protocol interface
- Every agent class MUST implement an aggregate interface

### Layer Boundaries (AES404)

| Layer | Can Contain | Cannot Contain |
| :--- | :--- | :--- |
| capabilities | Pure computation, validation | I/O, network, database |
| agent | Orchestration flow | Computation, I/O, business |

---

## 5. Verification Checklist

- [ ] All layer imports follow AES201 rules
- [ ] All classes implement appropriate protocol interfaces (AES403)
- [ ] No mixed responsibilities in layers (AES404)
- [ ] No magic constants in layers (AES405)
- [ ] Surface files follow role-based imports (AES406)

---

## 6. Scan → Diagnose → Fix → Verify Workflow

End-to-end workflow for an AI agent when asked to fix architecture violations in a TypeScript/JavaScript codebase.

### 6.1 Pre-flight

```bash
# Verify Node.js environment
node --version && npm --version

# Install dependencies if needed
npm install

# Check TypeScript compiles
npx tsc --noEmit
```

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
| AES101 (naming) | Rename file to `layer_concern_role.ts` | `create-{taxonomy,contract,capabilities,...}-typescript` |
| AES102 (suffix) | Change suffix to match layer rule | `create-{layer}-typescript` |
| AES201 (forbidden import) | Remove cross-layer import; use DI via contract | `create-contract-typescript` |
| AES202 (mandatory import) | Add the required import | — |
| AES203 (unused import) | Remove unused import line | — |
| AES204 (dummy import) | Remove dummy import + stub usage | `fix-bypass-typescript` |
| AES205 (circular import) | Break cycle by extracting to lower layer | `create-contract-typescript` |
| AES301 (max lines) | Split file by responsibility | `cleanup-consolidate-typescript` |
| AES302 (min lines) | Merge thin file into parent or delete | `cleanup-consolidate-typescript` |
| AES303 (mandatory def) | Add class/interface/function definition | — |
| AES304 (bypass) | Fix root cause, remove `@ts-ignore`/`@ts-expect-error` | `fix-bypass-typescript` |
| AES305 (duplication) | Extract shared logic to utility | `create-utility-typescript` |
| AES401–406 (role) | Move code to correct layer | `create-{layer}-typescript` |
| AES501–506 (orphan) | Wire into container or delete dead code | `cleanup-consolidate-typescript` |

### 6.5 Verify (confirm all clean)

```bash
# 1. Re-scan — should show 0 violations
lint-arwaky-cli scan <target-path>

# 2. TypeScript compiles
npx tsc --noEmit

# 3. ESLint passes (if configured)
npx eslint src/ --ext .ts,.tsx,.js,.jsx

# 4. Tests pass (if configured)
npx vitest run  # or npm test
```

### 6.6 Commit

```bash
git add -A
git commit -m "fix: resolve <N> AES violations (<list of rules>)"
```

---

## 7. Quick Fix Recipes

### Rename file (AES101/102)

```bash
# Before: capabilities_scanner.ts (wrong — missing concern + suffix mismatch)
# After:  capabilities_file_scanner.ts
git mv src/capabilities_scanner.ts src/capabilities_file_scanner.ts
# Update index.ts barrel exports
```

### Remove unused import (AES203)

```bash
lint-arwaky-cli fix src/file.ts --filter AES203
# Or manually: remove the unused import line
```

### Fix bypass comments (AES304)

```bash
# Find all bypass patterns
grep -rn '@ts-ignore\|@ts-expect-error\|@ts-nocheck\|eslint-disable\|FIXME\|TODO' src/

# Fix each one:
# @ts-ignore → fix the type error properly
# @ts-expect-error → add proper type assertion or narrowing
# eslint-disable → fix the underlying lint rule
```

### Remove dead code (AES501–506)

```bash
# Find orphan files
lint-arwaky-cli orphan packages/ --format json

# If orphan is truly dead → delete it
# If orphan should be wired → add to container or import chain
```

### Fix layer role violation (AES401–406)

```bash
# Identify which role rule is violated
lint-arwaky-cli role packages/ --filter AES403

# Common fixes:
# - Move I/O code from capabilities to utility
# - Move business logic from surface to capabilities
# - Move orchestration from capabilities to agent
# - Use aggregate interfaces from contract instead of importing capabilities directly
```

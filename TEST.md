1. Test Projects

There are 3 test workspaces with 2 variants each:

- **`workspaces-bad/`** — files with intentional violations (linter SHOULD detect them)
- **`workspaces-good/`** — clean files (linter should NOT flag them = false positive test)

| Category         | Bad Path                     | Good Path                     | Purpose                                     |
| ---------------- | ---------------------------- | ----------------------------- | ------------------------------------------- |
| Rust (crates)    | `workspaces-bad/crates/`   | `workspaces-good/crates/`   | AES Rust rules + Clippy/Rustfmt/cargo-audit |
| Python (modules) | `workspaces-bad/modules/`  | `workspaces-good/modules/`  | AES Python rules + Ruff/MyPy/Bandit         |
| JS/TS (packages) | `workspaces-bad/packages/` | `workspaces-good/packages/` | AES JS/TS rules + ESLint/Prettier/tsc       |

### Workspace Structure

```
workspaces-bad/                      # Files with violations (should trigger AES rules)
├── crates/                          # Rust: 142 files, 382 violations, 24 AES codes
│   ├── shared_common/src/           # Orphan files, bad naming, etc.
│   ├── naming_violations/src/       # AES101 naming violations
│   ├── code_analysis/src/           # AES503 capabilities orphans
│   └── ...
├── modules/                         # Python: 154 files, 384 violations, 24 AES codes
├── packages/                        # JS/TS: 144 files, 526 violations, 24 AES codes
├── Cargo.toml, pyproject.toml, package.json, ...

workspaces-good/                     # Clean files (0 violations expected)
├── crates/                          # Rust: 29 clean files
│   └── ...
├── modules/                         # Python: 69 clean files
├── packages/                        # JS/TS: 50 clean files
├── Cargo.toml, pyproject.toml, package.json, ...
```

### Expected Violation Counts

| Workspace | Language | Files | Violations | False Positives | Expected AES Codes |
| --------- | -------- | ----- | ---------- | --------------- | ------------------ |
| bad       | Rust     | 100+  | 100+       | —              | 24 unique codes    |
| bad       | Python   | 100+  | 100+       | —              | 24 unique codes    |
| bad       | JS/TS    | 100+  | 100+       | —              | 24 unique codes    |
| good      | Rust     | 29    | 0          | 0               | —                 |
| good      | Python   | 69    | 0          | 0               | —                 |
| good      | JS/TS    | 50    | 0          | 0               | —                 |

> **Note**: workspaces-good should produce 0 violations. If any violation
> appears, it's a false positive that must be fixed.

See [README.md](README.md) for CLI reference and
[ARCHITECTURE.md](ARCHITECTURE.md) for AES background.

---

## 2. How to Run Tests

### 2.1 Self-Lint (must be clean)

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- check .
```

> `check .` scans the lint-arwaky codebase itself. Expected: **0 violations**.

### 2.2 Scan Test Projects

```bash
# Scan bad workspace (should find violations)
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/crates
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/modules
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/packages

# Scan good workspace (should find 0 violations = false positive test)
cargo run --bin lint-arwaky-cli -- scan workspaces-good/crates
cargo run --bin lint-arwaky-cli -- scan workspaces-good/modules
cargo run --bin lint-arwaky-cli -- scan workspaces-good/packages
```

> Language is auto-detected from file extensions. No language flag needed.
> Python & JS/TS scans require external tools installed (ruff, mypy, bandit,
> eslint, prettier, tsc) for external lint violations.

### 2.3 Individual Rule Surface

```bash
# Run only naming rules on Rust test workspace
cargo run --bin lint-arwaky-cli -- naming workspaces-bad/crates

# Run only import rules
cargo run --bin lint-arwaky-cli -- import workspaces-bad/crates

# Run only orphan detection
cargo run --bin lint-arwaky-cli -- orphan workspaces-bad/crates
```

---

## 3. Pass / Fail Criteria

### 3.1 Thresholds

| Criteria                       | PASS   | FAIL       |
| ------------------------------ | ------ | ---------- |
| Total violations (Rust scan)   | >= 300 | < 300 or 0 |
| Total violations (Python scan) | >= 300 | < 300 or 0 |
| Total violations (JS/TS scan)  | >= 300 | < 300 or 0 |
| Unique AES codes (Rust)        | >= 24  | < 24       |
| Unique AES codes (Python)      | >= 24  | < 24       |
| Unique AES codes (JS/TS)       | >= 24  | < 24       |
| Self-lint violations           | 0      | > 0        |

### 3.2 Per-Rule Detection Matrix

Every AES rule MUST produce at least 1 violation in the test workspaces.
If any rule produces 0 violations, the test project is missing a trigger file.

| Rule   | Description                            | Rust | Python | JS/TS |
| ------ | -------------------------------------- | ---- | ------ | ----- |
| AES101 | Naming convention                      | ✓   | ✓     | ✓    |
| AES102 | Suffix/prefix validation               | ✓   | ✓     | ✓    |
| AES201 | Layer dependency violation             | ✓   | ✓     | ✓    |
| AES202 | Mandatory import missing               | ✓   | ✓     | ✓    |
| AES203 | Unused import                          | ✓   | ✓     | ✓    |
| AES204 | Dummy import / function                | ✓   | ✓     | ✓    |
| AES205 | Circular dependency                    | ✓   | ✓     | ✓    |
| AES301 | Max line count exceeded                | ✓   | ✓     | ✓    |
| AES302 | Min line count below                   | ✓   | ✓     | ✓    |
| AES303 | Missing definitions / dead inheritance | ✓   | ✓     | ✓    |
| AES304 | Bypass detection                       | ✓   | ✓     | ✓    |
| AES305 | Duplicate code                         | ✓   | ✓     | ✓    |
| AES401 | Taxonomy purity                        | ✓   | ✓     | ✓    |
| AES402 | Contract primitives                    | ✓   | ✓     | ✓    |
| AES403 | Capability implementation              | ✓   | ✓     | ✓    |
| AES404 | Utility purity                         | ✓   | ✓     | ✓    |
| AES405 | Agent composition                      | ✓   | ✓     | ✓    |
| AES406 | Surface passive role                   | ✓   | ✓     | ✓    |
| AES501 | Taxonomy orphan                        | ✓   | ✓     | ✓    |
| AES502 | Contract orphan                        | ✓   | ✓     | ✓    |
| AES503 | Capabilities orphan                    | ✓   | ✓     | ✓    |
| AES504 | Utility orphan                         | ✓   | ✓     | ✓    |
| AES505 | Agent orphan                           | ✓   | ✓     | ✓    |
| AES506 | Surface orphan                         | ✓   | ✓     | ✓    |

### 3.3 Negative Tests (must produce 0 violations)

| #  | Scenario                                                | Expected                                        |
| -- | ------------------------------------------------------- | ----------------------------------------------- |
| 1  | Barrel file (`mod.rs`, `__init__.py`, `index.ts`) | 0 violations (skipped)                          |
| 2  | File in`exceptions` list                              | 0 violations (skipped)                          |
| 3  | Config`architecture.enabled: false`                   | 0 violations (all rules disabled)               |
| 4  | Rule`AES201.enabled: false`                           | 0 AES201 violations (other rules still run)     |
| 5  | Clean, well-structured file                             | 0 violations                                    |
| 6  | `pub use` re-export in barrel file                    | 0 violations (not flagged as unused/dummy)      |
| 7  | `unwrap_or_default()` usage                           | 0 AES304 violations (safe variant)              |
| 8  | Import inside`#[cfg(test)]` block                     | 0 violations (conditional skip)                 |
| 9  | Root layer file (`root_*`)                            | 0 role-rule violations (skipped)                |
| 10 | File with`parse_ok = false`                           | PARSE_WARN emitted, file skipped for AES checks |

### 3.4 Exit Code Tests

| #  | Scenario                                      | Expected Exit Code               |
| -- | --------------------------------------------- | -------------------------------- |
| 1  | `scan` on clean project                     | 0                                |
| 2  | `scan` on workspaces-bad (violations found) | 1                                |
| 3  | `scan` on nonexistent path                  | 2                                |
| 4  | `scan` with invalid arguments               | 2                                |
| 5  | `security` without cargo-audit installed    | 3                                |
| 6  | `ci --threshold 0` with violations          | 1                                |
| 7  | `ci --threshold 100` with few violations    | 0                                |
| 8  | `fix --dry-run` with violations             | 0 (preview only)                 |
| 9  | `doctor` with all tools installed           | 0                                |
| 10 | `doctor` with missing tools                 | 0 (missing tools listed in body) |

---

## 4. Release Eligibility Checklist

Before releasing the binary to production or deploying to a client,
complete all verification tasks below.

### 4.1 Architecture Compliance (Self-Lint)

The base codebase must be clean of internal architecture rule violations.

- [ ] Run self-lint audit:

  ```bash
  cargo run --bin lint-arwaky-cli -- check .
  ```
- [ ] **Criteria**: Output must show **`Total violations: 0`**.
- [ ] **Safety net**: No inline bypasses (`#[allow(...)]`, `unwrap()`, `todo!()`,
  `FIXME`, `HACK`). If an external module strictly requires an exception,
  register it in `lint_arwaky.config.yaml` under the `exceptions`
  block — never use inline bypass comments.

### 4.2 Cross-Language Functional Verification

- [ ] Build a clean release:

  ```bash
  bash scripts/install.local.sh
  ```
- [ ] Run scan on bad workspaces (should find violations):

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates
  lint-arwaky-cli scan workspaces-bad/modules
  lint-arwaky-cli scan workspaces-bad/packages
  ```
- [ ] Run scan on good workspaces (should find 0 violations):

  ```bash
  lint-arwaky-cli scan workspaces-good/crates
  lint-arwaky-cli scan workspaces-good/modules
  lint-arwaky-cli scan workspaces-good/packages
  ```
- [ ] **Criteria**: Bad workspaces meet aggregate thresholds (Section 3.1).
- [ ] **Criteria**: Good workspaces produce 0 violations (false positive test).
- [ ] **Criteria**: All 24 AES codes detected per language (Section 3.2).
- [ ] **Criteria**: All negative tests pass (Section 3.3).
- [ ] **Criteria**: All exit code tests pass (Section 3.4).

### 4.3 System & MCP Protocol Verification

- [ ] Run workspace unit tests:

  ```bash
  cargo test --workspace
  ```
- [ ] Run binary health diagnostics:

  ```bash
  lint-arwaky-cli doctor
  ```
- [ ] Run MCP protocol smoke test:

  ```bash
  echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp
  ```

  **Criteria**: Responds in < 2 seconds with complete list of 5 registered
  MCP tools (`execute_command`, `list_commands`, `read_skill`, `health_check`,
  `get_config`).

### 4.4 Report Format Verification

- [ ] JSON output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format json
  ```
- [ ] SARIF output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format sarif
  ```
- [ ] JUnit XML output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format junit
  ```

  **Criteria**: All 3 formats produce valid, parseable output with correct
  rule codes, file paths, line numbers, and severity levels.

---

## 5. Instructions for AI Agents

1. **Automated verification**: Every time you modify code, rebuild with
   `scripts/install.local.sh` and run `check .` locally.
2. **Fix the root cause, do not bypass**: Never use inline bypasses
   (`unwrap`, `expect`, `panic!`, `noqa`, `#[allow(...)]`, `FIXME`, `HACK`)
   to suppress architecture warnings.
3. **Readiness report**: Upon completing work, report the status of every
   item in the Section 4 checklist transparently to the user.
4. **Test project maintenance**: If a new AES rule is added, add
   corresponding trigger files to all 3 test workspaces and update the
   per-rule detection matrix (Section 3.2).
5. **No false positives**: If a clean file produces a violation, it is a
   bug in the rule implementation — fix the rule, do not modify the test
   project to accommodate the bug.

---

## Reference

- PRD: [PRD.md](PRD.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- CLI Reference: [README.md](README.md)
- MCP Deployment: [DEPLOY.md](DEPLOY.md)

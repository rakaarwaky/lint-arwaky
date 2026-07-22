# Test Plan — Test Project Methodology

> **Principle**: The application is only declared **PASSED** if it successfully detects **many violations** in the test project folder.

## 1. Test Projects

There are 3 test workspace

| Category         | Path                          | Purpose                               |
| ---------------- | ----------------------------- | ------------------------------------- |
| Rust (crates)    | `test-workspaces/crates/`   | AES Rust + general quality checks     |
| Python (modules) | `test-workspaces/modules/`  | AES Python + Ruff/MyPy/Bandit scans   |
| JS/TS (packages) | `test-workspaces/packages/` | AES JS/TS + ESLint/Prettier/TSC scans |

See [README.md](README.md) for CLI reference and [ARCHITECTURE.md](ARCHITECTURE.md) for AES background.

> `check` is ONLY for `cargo run --bin lint-arwaky-cli -- check .` (self-lint). Test projects use `scan`.
> Python & JavaScript tests require external tools installed (ruff, mypy, bandit, eslint, etc.) for additional violations.

## 2. How to Run Tests

### 2.1 Running a Scan

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates for rust
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules for python
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages for typescript
```

## 3. Pass / Fail Criteria

| Criteria                  | PASS                   | FAIL        |
| ------------------------- | ---------------------- | ----------- |
| Total violations (scan)   | >= 2000 violations     | < 2000 or 0 |
| Unique AES codes (Rust)   | >= 24 unique AES codes | < 24        |
| Unique AES codes (Python) | >= 24 unique AES codes | < 24        |
| Unique AES codes (JS/TS)  | >= 24 unique AES codes | < 24        |

### 4.1 Release Eligibility Checklist (Production Ready)

Before releasing the binary to a production environment or deploying it to a client, the AI Agent must complete the following verification tasks:

#### A. Architecture Compliance (Self-Lint)

The base codebase must be clean of any internal architecture rule violations.

- [ ] Run self-lint audit:

  ```bash
  cargo run --bin lint-arwaky-cli -- check .
  ```
- [ ] _Criteria:_ Output must show **`Total violations: 0`**.
- [ ] _Safety Net:_ Ensure there are no arbitrary bypasses using `#[allow(...)]` or `unwrap()`. If an external module strictly requires an exception, register that module in the configuration file [lint_arwaky.config.rust.yaml](file:///home/raka/mcp-arwaky/lint-arwaky/lint_arwaky.config.rust.yaml) under the `exceptions` block, rather than using inline bypass comments.

  #### B. Cross-Language Functional Verification (Scan Test Projects)
- [ ] Build a clean release and copy the binary:

  ```bash
  bash scripts/install.local.sh
  ```
- [ ] Run scan on the test-workspaces folder:

  ```bash
  cd test-workspaces
  cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates for rust
  cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules for python
  cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages for typescript
  ```
- [ ] _Criteria:_ Findings count matches target baselines (mandatory 24 unique AES violation codes detected)

#### C. System & MCP Protocol Verification

- [ ] Run workspace unit tests:

  ```bash
  cargo test --workspace
  ```
- [ ] Run binary health diagnostics:

  ```bash
  lint-arwaky-cli doctor
  ```
- [ ] Run JSON-RPC MCP protocol smoke-test:

  ```bash
  echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp
  ```

  _Criteria:_ Binary responds in **< 2 seconds** with the complete list of 5 registered MCP tools.

### 4.2 Specific Instructions for AI Agents Working Here

1. **Automated Verification:** Every time you modify the code, you must rebuild the binary using `scripts/install.local.sh` and run the `check .` audit locally.
2. **Fix the Root Cause, Do Not Bypass:** Never use inline bypasses (`unwrap`, `expect`, `panic!`, `noqa`) to bypass architecture warnings.
3. **Readiness Report:** Upon completing work, report the status of every item in the checklist above transparently to the user.

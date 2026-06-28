# Test Plan — Test Project Methodology

> **Principle**: The application is only declared **PASSED** if it successfully detects **many violations** in the test project folder.

## 1. Test Projects

There are 23 test workspace members with intentional violations grouped under a single multi-project directory:

| Category         | Workspace Members | Path                              | Purpose                               |
| ---------------- | ----------------- | --------------------------------- | ------------------------------------- |
| Rust (crates)    | 7                 | `test-workspaces/crates/<name>`   | AES Rust + general quality checks     |
| Python (modules) | 8                 | `test-workspaces/modules/<name>`  | AES Python + Ruff/MyPy/Bandit scans   |
| JS/TS (packages) | 8                 | `test-workspaces/packages/<name>` | AES JS/TS + ESLint/Prettier/TSC scans |

> **Note**: `check` = AES self-lint — only for the `crates/` project itself. `scan` = multi-adapter — for ALL target projects (Rust, Python, JavaScript).
> `check` is ONLY for `cargo run --bin lint-arwaky-cli -- check .` (self-lint). Test projects use `scan`.
> Python & JavaScript tests require external tools installed (ruff, mypy, bandit, eslint, etc.) for additional violations.

## 2. How to Run Tests

### 2.1 Running a Multi-Workspace Scan (All Projects)

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-workspaces/
```

### 2.2 Running a Scan on a Specific Workspace Member

```bash
# Example: scanning one Rust workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates/cli_commands

# Example: scanning one Python workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules/cli_commands

# Example: scanning one JS/TS workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages/cli_commands
```

## 3. Pass / Fail Criteria

| Criteria                  | PASS                   | FAIL        |
| ------------------------- | ---------------------- | ----------- |
| Total violations (scan)   | >= 2000 violations     | < 2000 or 0 |
| Unique AES codes (Rust)   | >= 18 unique AES codes | < 18        |
| Unique AES codes (Python) | >= 18 unique AES codes | < 18        |
| Unique AES codes (JS/TS)  | >= 18 unique AES codes | < 18        |

**Baseline v1.10.29** (22 June 2026):

| Project                 | Command                         | Total Violations | Unique AES Codes | Status  |
| ----------------------- | ------------------------------- | ---------------- | ---------------- | ------- |
| Self-lint (lint-arwaky) | `check .`                       | 0                | 0                | ✅ PASS |
| Rust (crates)           | `scan test-workspaces/crates`   | 262              | 19               | ❌      |
| Python (modules)        | `scan test-workspaces/modules`  | 454              | 11               | ❌      |
| JS/TS (packages)        | `scan test-workspaces/packages` | 1357             | 7                | ❌      |

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

#### B. Structural & Dependency Integrity (Graph-It-Live)

Ensure that layer boundaries are maintained and no dead code (_orphan code_) remains in the active workspace.

- [ ] **Circular Dependency Detection:**
      Verify random samples of files in each layer using the `cycles` tool:

  ```bash
  graph-it serve cycles crates/shared/src/common/contract_parser_port.rs
  graph-it serve cycles crates/naming-rules/src/contract_naming_runner_aggregate.rs
  ```

  _Criteria:_ Output shows **0 dependency cycles**.

- [ ] **Layer Boundary Protection:**
      Ensure UI/Surfaces components do not import technical infrastructure or capabilities directly (must go through `ServiceContainerAggregate`):

  ```bash
  graph-it serve path-in crates/external-lint/src/infrastructure_js_eslint_adapter.rs
  ```

  _Criteria:_ Infrastructure files are only imported by `root/` (di-container) files.

- [ ] **Orphan Code (Dead Code) Verification:**
      Ensure that no active logic files are isolated or unreferenced:

  ```bash
  graph-it serve path-in crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs
  ```

  _Criteria:_ Every logic file has at least 1 incoming reference from an entry point or higher layer.

#### C. Cross-Language Functional Verification (Scan Test Projects)

The multi-adapter scanner must be proven to successfully detect at least 18 unique violation types in the test projects.

- [ ] Build a clean release and copy the binary:
  ```bash
  bash build.local.sh
  ```
- [ ] Run scan on the test-workspaces folder:
  ```bash
  lint-arwaky-cli scan test-workspaces/
  ```
- [ ] _Criteria:_ Findings count matches target baselines (minimum 20 unique AES violation codes detected cumulatively, at least 1 CRITICAL violation found, and zero false positives on valid files).

#### D. System & MCP Protocol Verification

- [ ] Run workspace unit tests:

  ```bash
  cargo test --workspace
  ```

- [ ] Run binary health diagnostics:

  ```bash
  lint-arwaky-cli maintenance doctor
  ```

- [ ] Run JSON-RPC MCP protocol smoke-test:

  ```bash
  echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp
  ```

  _Criteria:_ Binary responds in **< 2 seconds** with the complete list of 5 registered MCP tools.

### 4.2 Specific Instructions for AI Agents Working Here

1. **Automated Verification:** Every time you modify the code, you must rebuild the binary using `build.local.sh` and run the `check .` audit locally.
2. **Fix the Root Cause, Do Not Bypass:** Never use inline bypasses (`unwrap`, `expect`, `panic!`, `noqa`) to bypass architecture warnings. You must design code according to contracts or register the module under YAML exceptions if it is genuinely safe.
3. **Readiness Report:** Upon completing work, report the status of every item in the checklist above transparently to the user.

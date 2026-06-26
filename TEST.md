# Test Plan — Test Project Methodology

> **Prinsip**: Aplikasi hanya dinyatakan **LULUS** jika berhasil mendeteksi **banyak violations** pada test project folder.

## 1. Test Projects

Ada 23 test workspace member dengan intentional violations yang dikelompokkan di bawah satu direktori multi-project:

| Category         | Workspace Members | Path                                | Purpose                               |
| ---------------- | ----------------- | ----------------------------------- | ------------------------------------- |
| Rust (crates)    | 7                 | `test-workspaces/crates/<name>`   | AES Rust + general quality checks     |
| Python (modules) | 8                 | `test-workspaces/modules/<name>`  | AES Python + Ruff/MyPy/Bandit scans   |
| JS/TS (packages) | 8                 | `test-workspaces/packages/<name>` | AES JS/TS + ESLint/Prettier/TSC scans |

> **Catatan**: `check` = AES self-lint — hanya untuk `crates/` project sendiri. `scan` = multi-adapter — untuk SEMUA target project (Rust, Python, JavaScript).
> `check` HANYA untuk `cargo run --bin lint-arwaky-cli -- check .` (self-lint). Test project menggunakan `scan`.
> Test Python & JavaScript membutuhkan external tools terinstall (ruff, mypy, bandit, eslint, dll) untuk violations tambahan.

## 2. Cara Menjalankan Test

### 2.1 Menjalankan Scan Multi-Workspace (Semua Project)

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-workspaces/
```

### 2.2 Menjalankan Scan pada Specific Workspace Member

```bash
# Contoh menscan salah satu Rust workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates/cli_commands

# Contoh menscan salah satu Python workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules/cli_commands

# Contoh menscan salah satu JS/TS workspace member
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages/cli_commands
```

## 3. Kriteria LULUS / GAGAL

| Kriteria                           | LULUS                  | GAGAL         |
| ---------------------------------- | ---------------------- | ------------- |
| Total discovered workspace members | 23 workspace members   | < 23          |
| Total violations (scan)            | >= 2000 violations     | < 2000 atau 0 |
| Unique AES codes (Rust)            | >= 24 unique AES codes | < 24          |
| Unique AES codes (Python)          | >= 24 unique AES codes | < 24          |
| Unique AES codes (JS/TS)           | >= 24 unique AES codes | < 24          |
| Total unique AES codes (all)       | >= 24 unique AES codes | < 24          |

## 4. Violations yang Diharapkan

### 4.1 Rust (AES Self-Lint) — 155 violations detected ✅

| AES Code | Type                | Contoh File                                                |
| -------- | ------------------- | ---------------------------------------------------------- |
| AES201   | Forbidden import    | surface_direct_infra_handler, taxonomy_forbidden_import    |
| AES202   | Mandatory import    | capabilities files missing taxonomy import                 |
| AES101   | Naming convention   | badname.rs                                                 |
| AES102   | Suffix mismatch     | surface_mod, contract_wrong_name_port, taxonomy_mod        |
| AES301   | File too large      | extremely_large_vo                                         |
| AES302   | File too short      | surface_mod, capabilities_mcp_tool_processor, agent_mod    |
| AES304   | Bypass/unwrap       | agent_unsafe_bypass_orchestrator, taxonomy_bypass_comment  |
| AES203   | Unused import       | surface_complex_view_handler, infrastructure_broad_import  |
| AES303   | Mandatory def       | taxonomy_bare_entity (dead inheritance)                    |
| AES501   | Orphan taxonomy     | taxonomy orphan files                                      |
| AES502   | Orphan contract     | contract_wrong_name_port                                   |
| AES503   | Orphan capabilities | capabilities orphan files                                  |
| AES504   | Orphan infra        | infrastructure orphan files                                |
| AES505   | Orphan agent        | agent orphan files                                         |
| AES506   | Orphan surface      | surface orphan files                                       |
| AES401   | Taxonomy role       | taxonomy_primitive_entity, taxonomy_impure_system_constant |
| AES402   | Contract primitive  | contract_wrong_name_port                                   |
| AES403   | Capabilities role   | capabilities_unmatched_struct_processor                    |
| AES405   | Agent role          | agent_stateful_violations                                  |
| AES406   | Surface role        | surface_complex_busy_handler, surface_many_functions       |

### 4.2 Python (Multi-Adapter — requires ruff, mypy, bandit installed)

| Tool   | Expected Issues             |
| ------ | --------------------------- |
| Ruff   | Style/formatting violations |
| MyPy   | Type annotation violations  |
| Bandit | Security violations         |



| Tool     | Expected Issues          |
| -------- | ------------------------ |
| ESLint   | Code quality violations  |
| Prettier | Formatting violations    |
| TSC      | Type checking violations |

```bash
cargo run --bin lint-arwaky-cli -- scan test-workspaces/
```

**Baseline v1.10.29** (22 Juni 2026):

| Project                 | Command                           | Total Violations | Unique AES Codes | Status  |
| ----------------------- | --------------------------------- | ---------------- | ---------------- | ------- |
| Self-lint (lint-arwaky) | `check .`                       | 0                | 0                | ✅ PASS |
| Rust (crates)           | `scan test-workspaces/crates`   | 262              | 19               | ❌      |
| Python (modules)        | `scan test-workspaces/modules`  | 454              | 11               | ❌      |
| JS/TS (packages)        | `scan test-workspaces/packages` | 1357             | 7                | ❌      |
| **Combined**      | `scan test-workspaces/`         | 2073             | 81               | ❌      |

**Target: 24 unique AES codes per language.**

### Per-language AES Coverage (Current)

**Rust (19/24):** AES102, AES201, AES202, AES203, AES204, AES205, AES303, AES304, AES401, AES402, AES403, AES404, AES405, AES406, AES501, AES502, AES503, AES504, AES506
**Missing:** AES101, AES301, AES302, AES305, AES505

**Python (11/24):** AES102, AES201, AES202, AES203, AES204, AES303, AES304, AES405, AES501, AES503, AES506
**Missing:** AES101, AES205, AES301, AES302, AES305, AES401, AES402, AES403, AES404, AES406, AES502, AES504, AES505

**JS/TS (7/24):** AES102, AES201, AES202, AES203, AES204, AES303, AES405
**Missing:** AES101, AES205, AES301, AES302, AES304, AES305, AES401, AES402, AES403, AES404, AES406, AES501, AES502, AES503, AES504, AES505, AES506

### All 24 AES Codes Target

| Code   | Group                             | Rust | Python | JS/TS |
| ------ | --------------------------------- | ---- | ------ | ----- |
| AES101 | Naming — layer prefix            | ❌   | ❌     | ❌    |
| AES102 | Naming — suffix convention       | ✅   | ✅     | ✅    |
| AES201 | Import — forbidden layer         | ✅   | ✅     | ✅    |
| AES202 | Import — mandatory import        | ✅   | ✅     | ✅    |
| AES203 | Import — unused import           | ✅   | ✅     | ✅    |
| AES204 | Import — dummy/todo import       | ✅   | ✅     | ✅    |
| AES205 | Import — barrel re-export        | ✅   | ❌     | ❌    |
| AES301 | Quality — file max lines         | ❌   | ❌     | ❌    |
| AES302 | Quality — fn max lines           | ❌   | ❌     | ❌    |
| AES303 | Quality — bypass suppression     | ✅   | ✅     | ✅    |
| AES304 | Quality — mandatory def          | ✅   | ✅     | ❌    |
| AES305 | Quality — todo in non-test       | ❌   | ❌     | ❌    |
| AES401 | Role — layer-role suffix         | ✅   | ❌     | ❌    |
| AES402 | Role — bypass aggregate          | ✅   | ❌     | ❌    |
| AES403 | Role — capability bypasses agent | ✅   | ❌     | ❌    |
| AES404 | Role — surface calls capability  | ✅   | ❌     | ❌    |
| AES405 | Role — infra no aggregate        | ✅   | ✅     | ✅    |
| AES406 | Role — duplicate container       | ✅   | ❌     | ❌    |
| AES501 | Orphan — unreachable file        | ✅   | ✅     | ❌    |
| AES502 | Orphan — unused contract         | ✅   | ❌     | ❌    |
| AES503 | Orphan — unused capability       | ✅   | ✅     | ❌    |
| AES504 | Orphan — dead dependency         | ✅   | ❌     | ❌    |
| AES505 | Orphan — circular dependency     | ❌   | ❌     | ❌    |
| AES506 | Orphan — barrel all-unused       | ✅   | ✅     | ❌    |

---

## 6. Production Readiness Checklist & AI Agent Instructions

> [!IMPORTANT]
> Since this project is developed using **Agentic Coding (AI Agent-driven)** where you (the user) do not manually read the code, this checklist acts as an **automation contract**. The AI Agent **must** run and verify all steps below before declaring a feature/task complete or proposing a merge to the main branch.

### 6.1 Release Eligibility Checklist (Production Ready)

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
  graph-it serve cycles crates/source-parsing/src/contract_parser_port.rs
  graph-it serve cycles crates/naming-rules/src/contract_naming_runner_aggregate.rs
  ```

  _Criteria:_ Output shows **0 dependency cycles**.
- [ ] **Layer Boundary Protection:**
  Ensure UI/Surfaces components do not import technical infrastructure or capabilities directly (must go through `ServiceContainerAggregate`):

  ```bash
  graph-it serve path-in crates/external-lint/src/infrastructure_js_naming.rs
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

### 6.2 Specific Instructions for AI Agents Working Here

1. **Automated Verification:** Every time you modify the code, you must rebuild the binary using `build.local.sh` and run the `check .` audit locally.
2. **Fix the Root Cause, Do Not Bypass:** Never use inline bypasses (`unwrap`, `expect`, `panic!`, `noqa`) to bypass architecture warnings. You must design code according to contracts or register the module under YAML exceptions if it is genuinely safe.
3. **Readiness Report:** Upon completing work, report the status of every item in the checklist above transparently to the user.

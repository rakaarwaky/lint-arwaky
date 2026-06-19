# Test Plan — Test Project Methodology

> **Prinsip**: Aplikasi hanya dinyatakan **LULUS** jika berhasil mendeteksi **banyak violations** pada test project folder.

## 1. Test Projects

Ada 3 test project dengan intentional violations:

| Project    | Language | Command | Path                       | File Count |
| ---------- | -------- | ------- | -------------------------- | ---------- |
| Rust       | Rust     | `scan`  | `test-project-rust/`       | ~30 files  |
| Python     | Python   | `scan`  | `test-project-python/`     | ~60 files  |
| JavaScript | JS/TS    | `scan`  | `test-project-javascript/` | ~130 files |

> **Catatan**: `check` = AES self-lint — hanya untuk `crates/` project sendiri. `scan` = multi-adapter — untuk SEMUA target project (Rust, Python, JavaScript).
> `check` HANYA untuk `cargo run --bin lint-arwaky-cli -- check .` (self-lint). Test project menggunakan `scan`.
> Test Python & JavaScript membutuhkan external tools terinstall (ruff, mypy, bandit, eslint, dll) untuk violations tambahan.

## 2. Cara Menjalankan Test

### 2.1 Test Rust Project

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-project-rust/
```

### 2.2 Test Python Project

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-project-python/
```

### 2.3 Test JavaScript Project

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-project-javascript/
```

## 3. Kriteria LULUS / GAGAL

| Kriteria                           | LULUS                             | GAGAL                       |
| ---------------------------------- | --------------------------------- | --------------------------- |
| Total violations Rust (scan)       | >= 100 violations                 | < 100 atau 0                |
| Total violations Python (scan)     | >= 200 violations                 | < 200 atau 0                |
| Total violations JavaScript (scan) | >= 500 violations                 | < 500 atau 0                |
| Unique AES codes per project       | >= 10 unique codes each           | < 10                        |
| Combined unique AES codes          | >= 15 unique codes                | < 15                        |
| Severity CRITICAL ditemukan        | Minimal 1 di setiap project       | Tidak ada                   |
| Zero false positive                | Tidak ada violation di file benar | Ada violation di file benar |

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

### 4.3 JavaScript (Multi-Adapter — requires eslint, prettier, tsc installed)

| Tool     | Expected Issues          |
| -------- | ------------------------ |
| ESLint   | Code quality violations  |
| Prettier | Formatting violations    |
| TSC      | Type checking violations |

## 5. Baseline

```bash
echo "=== RUST ===" && cargo run --bin lint-arwaky-cli -- scan test-project-rust/ 2>&1 | grep "Violations:"
echo "=== PYTHON ===" && cargo run --bin lint-arwaky-cli -- scan test-project-python/ 2>&1 | grep "Violations:"
echo "=== JAVASCRIPT ===" && cargo run --bin lint-arwaky-cli -- scan test-project-javascript/ 2>&1 | grep "Violations:"
```

**Baseline v1.10.14** (15 Juni 2026):

| Project                 | Command                         | Total Violations | Unique AES Codes | Status  |
| ----------------------- | ------------------------------- | ---------------- | ---------------- | ------- |
| Self-lint (lint-arwaky) | `check .`                       | 0                | 0                | ✅ PASS |
| Rust test project       | `scan test-project-rust/`       | 155              | 17               | ✅ PASS |
| Python test project     | `scan test-project-python/`     | 232              | 15               | ✅ PASS |
| JavaScript test project | `scan test-project-javascript/` | 844              | 13               | ✅ PASS |
| **Combined**            |                                 | **1231**         | **18**           | ✅ PASS |

**18+ Unique AES Codes (v3.0) target after source migration:**
AES101, AES102, AES201, AES202, AES203, AES205,
AES301, AES302, AES303, AES304, AES305,
AES401, AES402, AES403, AES404, AES405, AES406,
AES501, AES502, AES503, AES504, AES505, AES506

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
- [ ] Run scan on the three test projects:
  ```bash
  lint-arwaky-cli scan test-project-rust/
  lint-arwaky-cli scan test-project-python/
  lint-arwaky-cli scan test-project-javascript/
  ```
- [ ] _Criteria:_ Findings count matches target baselines (minimum 18 unique AES violation codes detected cumulatively, at least 1 CRITICAL violation found in each test project, and zero false positives on valid files).

#### D. System & MCP Protocol Verification

- [ ] Run workspace unit tests:
  ```bash
  cargo test --workspace
  ```
- [ ] Run binary health diagnostics:
  ```bash
  lint-arwaky-cli setup doctor
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

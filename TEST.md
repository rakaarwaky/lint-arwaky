# Test Plan — Test Project Methodology

> **Prinsip**: Aplikasi hanya dinyatakan **LULUS** jika berhasil mendeteksi **banyak violations** pada test project folder.

## 1. Test Projects

Ada 3 test project dengan intentional violations:

| Project    | Language | Command | Path                       | File Count |
| ---------- | -------- | ------- | -------------------------- | ---------- |
| Rust       | Rust     | `scan`  | `test-project-rust/`       | ~30 files  |
| Python     | Python   | `scan`  | `test-project-python/`     | ~60 files  |
| JavaScript | JS/TS    | `scan`  | `test-project-javascript/` | ~130 files |

> **Catatan**: `check` = AES self-lint — hanya untuk `src-rust/` project sendiri. `scan` = multi-adapter — untuk SEMUA target project (Rust, Python, JavaScript).
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
| Total violations Rust (check)      | >= 21 violation types berbeda     | < 21 atau 0                 |
| Total violations Python (scan)     | >= 21 violation types berbeda     | < 21 atau 0                 |
| Total violations JavaScript (scan) | >= 21 violation types berbeda     | < 21 atau 0                 |
| Severity CRITICAL ditemukan        | Minimal 1 di setiap project       | Tidak ada                   |
| Zero false positive                | Tidak ada violation di file benar | Ada violation di file benar |

## 4. Violations yang Diharapkan

### 4.1 Rust (AES Self-Lint) — 27violations detected ✅

| AES Code | Type              | Contoh File                                               |
| -------- | ----------------- | --------------------------------------------------------- |
| AES003   | Naming convention | wrong_suffix, dummy_port, stateful_orchestrator           |
| AES004   | File too large    | extremely_large_vo                                        |
| AES005   | File too short    | invalid_import_vo, removal_types, missing_import_analyzer |

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
echo "=== RUST ===" && cargo run --bin lint-arwaky-cli -- scan test-project-rust/ 2>&1 | grep "Total AES Violations"
echo "=== PYTHON ===" && cargo run --bin lint-arwaky-cli -- scan test-project-python/ 2>&1 | grep "Total AES Violations"
echo "=== JAVASCRIPT ===" && cargo run --bin lint-arwaky-cli -- scan test-project-javascript/ 2>&1 | grep "Total AES Violations"
```

**Baseline v1.10.9** (11 Juni 2026):

| Project                 | Command                         | Total Violations | Unique AES Codes | Status |
| ----------------------- | ------------------------------- | ---------------- | ---------------- | ------ |
| Self-lint (lint-arwaky) | `check .`                       | 153              | 15               |        |
| Rust test project       | `scan test-project-rust/`       | 34               | 14               |        |
| Python test project     | `scan test-project-python/`     | 238              | 9                |        |
| JavaScript test project | `scan test-project-javascript/` | 323              | 12               |        |
| **Combined**            |                                 |                  | **21**           |        |

**21 Unique AES Codes (v2.0):**
AES001, AES002,
AES010, AES011, AES012, AES013, AES014, AES015, AES016,
AES020, AES021, AES022, AES023, AES024,
AES030, AES0301, AES0302, AES0303, AES0305, AES0306

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
  graph-it serve cycles src-rust/source-parsing/contract_parser_port.rs
  graph-it serve cycles src-rust/di-containers/contract_service_aggregate.rs
  ```
  _Criteria:_ Output shows **0 dependency cycles**.
- [ ] **Layer Boundary Protection:**
      Ensure UI/Surfaces components do not import technical infrastructure or capabilities directly (must go through `ServiceContainerAggregate`):
  ```bash
  graph-it serve path-in src-rust/language-adapters/infrastructure_js_naming.rs
  ```
  _Criteria:_ Infrastructure files are only imported by `agent/` or `di-container/` files.
- [ ] **Orphan Code (Dead Code) Verification:**
      Ensure that no active logic files are isolated or unreferenced:
  ```bash
  graph-it serve path-in src-rust/orphan-detector/capabilities_orphan_analyzer.rs
  ```
  _Criteria:_ Every logic file has at least 1 incoming reference from an entry point or higher layer.

#### C. Cross-Language Functional Verification (Scan Test Projects)

The multi-adapter scanner must be proven to successfully detect at least 21 unique violation types in the test projects.

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
- [ ] _Criteria:_ Findings count matches target baselines (minimum 21 unique AES violation codes detected cumulatively, at least 1 CRITICAL violation found in each test project, and zero false positives on valid files).

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

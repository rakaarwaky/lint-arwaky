# Test Plan — Test Project Methodology

> **Prinsip**: Aplikasi hanya dinyatakan **LULUS** jika berhasil mendeteksi **banyak violations** pada test project folder.

## 1. Test Projects

Ada 3 test project dengan intentional violations:

| Project    | Language | Command  | Path                         | File Count |
| ---------- | -------- | -------- | ---------------------------- | ---------- |
| Rust       | Rust     | `scan` | `test-project-rust/`       | ~30 files  |
| Python     | Python   | `scan` | `test-project-python/`     | ~60 files  |
| JavaScript | JS/TS    | `scan` | `test-project-javascript/` | ~130 files |

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

**Baseline v1.10.2** (7 Juni 2026):

| Project                 | Command                           | Total Violations | Unique AES Codes | Status |
| ----------------------- | --------------------------------- | ---------------- | ---------------- | ------ |
| Self-lint (lint-arwaky) | `check .`                       | 153              | 15               |        |
| Rust test project       | `scan test-project-rust/`       | 34               | 14               |        |
| Python test project     | `scan test-project-python/`     | 238              | 9                |        |
| JavaScript test project | `scan test-project-javascript/` | 323              | 12               |        |
| **Combined**      |                                   |                  | **21**     |        |

**21 Unique AES Codes (v2.0):**
AES001, AES002,
AES010, AES011, AES012, AES013, AES014, AES015, AES016,
AES020, AES021, AES022, AES023, AES024,
AES030, AES0301, AES0302, AES0303, AES0304, AES0305, AES0306

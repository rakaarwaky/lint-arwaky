 1. Test Projects

There are 3 test workspaces, each containing files **intentionally designed**
to trigger AES violations:


| Category         | Path                        | Purpose                                     |
| ------------------ | ----------------------------- | --------------------------------------------- |
| Rust (crates)    | `test-workspaces/crates/`   | AES Rust rules + Clippy/Rustfmt/cargo-audit |
| Python (modules) | `test-workspaces/modules/`  | AES Python rules + Ruff/MyPy/Bandit         |
| JS/TS (packages) | `test-workspaces/packages/` | AES JS/TS rules + ESLint/Prettier/tsc       |

### Test Project Structure

```
test-workspaces/
├── crates/                          # Rust test workspace
│   ├── taxonomy_bad_naming.rs       # AES101: uppercase, hyphens, too few words
│   ├── taxonomy_user_vo.rs          # AES401: raw String/i32 fields (should use VOs)
│   ├── taxonomy_constant.rs         # AES401: contains fn/struct (should be const only)
│   ├── contract_bad_protocol.rs     # AES402: primitives in method signatures
│   ├── capabilities_no_impl.rs      # AES403: no protocol implementor
│   ├── capabilities_too_many.rs     # AES403: >3 type declarations
│   ├── utility_with_struct.rs       # AES404: struct in utility file
│   ├── agent_no_aggregate.rs        # AES405: no aggregate implementor
│   ├── surface_passive_logic.rs     # AES406: domain logic in passive surface
│   ├── surface_too_many_fns.rs      # AES406: >15 functions
│   ├── forbidden_import.rs          # AES201: taxonomy imports capabilities
│   ├── unused_imports.rs            # AES203: declared but never used
│   ├── dummy_functions.rs           # AES204: _use_* / dummy_* functions
│   ├── circular_a.rs                # AES205: circular dependency (a → b)
│   ├── circular_b.rs                # AES205: circular dependency (b → a)
│   ├── orphan_file.rs               # AES501-506: not imported by anything
│   ├── bypass_unwrap.rs             # AES304: unwrap(), expect(), panic!
│   ├── bypass_allow.rs              # AES304: #[allow(...)]
│   ├── bypass_comments.rs           # AES304: FIXME, HACK, XXX
│   ├── bloated_file.rs              # AES301: >1000 lines
│   ├── empty_file.rs                # AES302: <10 lines
│   ├── no_definitions.rs            # AES303: no struct/enum/trait
│   ├── duplicate_a.rs               # AES305: >50% overlap with duplicate_b
│   ├── duplicate_b.rs               # AES305: >50% overlap with duplicate_a
│   └── mod.rs                       # Barrel file — must be SKIPPED
│
├── modules/                         # Python test workspace
│   ├── taxonomy_bad_naming.py       # AES101: invalid naming
│   ├── utility_with_class.py        # AES404: class in utility file
│   ├── bypass_noqa.py               # AES304: # noqa, # type: ignore
│   ├── bypass_not_implemented.py    # AES304: raise NotImplementedError
│   ├── orphan_module.py             # AES501-506: unreachable
│   ├── __init__.py                  # Barrel file — must be SKIPPED
│   └── ...
│
└── packages/                        # JS/TS test workspace
    ├── taxonomy_bad_naming.ts       # AES101: invalid naming
    ├── surface_component_logic.tsx  # AES406: domain logic in passive component
    ├── bypass_ts_ignore.ts          # AES304: @ts-ignore, @ts-expect-error
    ├── bypass_eslint_disable.js     # AES304: eslint-disable
    ├── orphan_component.tsx         # AES501-506: unreachable
    ├── index.ts                     # Barrel file — must be SKIPPED
    └── ...
```

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
# Rust test workspace
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates

# Python test workspace
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules

# JS/TS test workspace
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages
```

> Language is auto-detected from file extensions. No language flag needed.
> Python & JS/TS scans require external tools installed (ruff, mypy, bandit,
> eslint, prettier, tsc) for external lint violations.

### 2.3 Individual Rule Surface

```bash
# Run only naming rules on Rust test workspace
cargo run --bin lint-arwaky-cli -- naming test-workspaces/crates

# Run only import rules
cargo run --bin lint-arwaky-cli -- import test-workspaces/crates

# Run only orphan detection
cargo run --bin lint-arwaky-cli -- orphan test-workspaces/crates
```

---

## 3. Pass / Fail Criteria

### 3.1 Thresholds


| Criteria                       | PASS    | FAIL        |
| -------------------------------- | --------- | ------------- |
| Total violations (Rust scan)   | >= 2000 | < 2000 or 0 |
| Total violations (Python scan) | >= 2000 | < 2000 or 0 |
| Total violations (JS/TS scan)  | >= 2000 | < 2000 or 0 |
| Unique AES codes (Rust)        | >= 24   | < 24        |
| Unique AES codes (Python)      | >= 24   | < 24        |
| Unique AES codes (JS/TS)       | >= 24   | < 24        |
| Self-lint violations           | 0       | > 0         |

### 3.2 Per-Rule Detection Matrix

Every AES rule MUST produce at least 1 violation in the test workspaces.
If any rule produces 0 violations, the test project is missing a trigger file.


| Rule   | Description                            | Rust | Python | JS/TS |
| -------- | ---------------------------------------- | ------ | -------- | ------- |
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


| #  | Scenario                                          | Expected                                        |
| ---- | --------------------------------------------------- | ------------------------------------------------- |
| 1  | Barrel file (`mod.rs`, `__init__.py`, `index.ts`) | 0 violations (skipped)                          |
| 2  | File in`exceptions` list                          | 0 violations (skipped)                          |
| 3  | Config`architecture.enabled: false`               | 0 violations (all rules disabled)               |
| 4  | Rule`AES201.enabled: false`                       | 0 AES201 violations (other rules still run)     |
| 5  | Clean, well-structured file                       | 0 violations                                    |
| 6  | `pub use` re-export in barrel file                | 0 violations (not flagged as unused/dummy)      |
| 7  | `unwrap_or_default()` usage                       | 0 AES304 violations (safe variant)              |
| 8  | Import inside`#[cfg(test)]` block                 | 0 violations (conditional skip)                 |
| 9  | Root layer file (`root_*`)                        | 0 role-rule violations (skipped)                |
| 10 | File with`parse_ok = false`                       | PARSE_WARN emitted, file skipped for AES checks |

### 3.4 Exit Code Tests


| #  | Scenario                                     | Expected Exit Code               |
| ---- | ---------------------------------------------- | ---------------------------------- |
| 1  | `scan` on clean project                      | 0                                |
| 2  | `scan` on test-workspaces (violations found) | 1                                |
| 3  | `scan` on nonexistent path                   | 2                                |
| 4  | `scan` with invalid arguments                | 2                                |
| 5  | `security` without cargo-audit installed     | 3                                |
| 6  | `ci --threshold 0` with violations           | 1                                |
| 7  | `ci --threshold 100` with few violations     | 0                                |
| 8  | `fix --dry-run` with violations              | 0 (preview only)                 |
| 9  | `doctor` with all tools installed            | 0                                |
| 10 | `doctor` with missing tools                  | 0 (missing tools listed in body) |

---

## 4. Release Eligibility Checklist

Before releasing the binary to production or deploying to a client,
complete all verification tasks below.

### 4.1 Architecture Compliance (Self-Lint)

The base codebase must be clean of internal architecture rule violations.

- [ ]  Run self-lint audit:

  ```bash
  cargo run --bin lint-arwaky-cli -- check .
  ```
- [ ]  **Criteria**: Output must show **`Total violations: 0`**.
- [ ]  **Safety net**: No inline bypasses (`#[allow(...)]`, `unwrap()`, `todo!()`,
  `FIXME`, `HACK`). If an external module strictly requires an exception,
  register it in `lint_arwaky.config.rust.yaml` under the `exceptions`
  block — never use inline bypass comments.

### 4.2 Cross-Language Functional Verification

- [ ]  Build a clean release:

  ```bash
  bash scripts/install.local.sh
  ```
- [ ]  Run scan on all 3 test workspaces:

  ```bash
  lint-arwaky-cli scan test-workspaces/crates
  lint-arwaky-cli scan test-workspaces/modules
  lint-arwaky-cli scan test-workspaces/packages
  ```
- [ ]  **Criteria**: Each scan meets aggregate thresholds (Section 3.1).
- [ ]  **Criteria**: All 24 AES codes detected per language (Section 3.2).
- [ ]  **Criteria**: All negative tests pass (Section 3.3).
- [ ]  **Criteria**: All exit code tests pass (Section 3.4).

### 4.3 System & MCP Protocol Verification

- [ ]  Run workspace unit tests:

  ```bash
  cargo test --workspace
  ```
- [ ]  Run binary health diagnostics:

  ```bash
  lint-arwaky-cli doctor
  ```
- [ ]  Run MCP protocol smoke test:

  ```bash
  echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp
  ```

  **Criteria**: Responds in < 2 seconds with complete list of 5 registered
  MCP tools (`execute_command`, `list_commands`, `read_skill`, `health_check`,
  `get_config`).

### 4.4 Report Format Verification

- [ ]  JSON output:

  ```bash
  lint-arwaky-cli scan test-workspaces/crates --format json
  ```
- [ ]  SARIF output:

  ```bash
  lint-arwaky-cli scan test-workspaces/crates --format sarif
  ```
- [ ]  JUnit XML output:

  ```bash
  lint-arwaky-cli scan test-workspaces/crates --format junit
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

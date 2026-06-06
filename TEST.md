# Verification Suite — Lint Arwaky v1.10.2

---

## 1. MCP Tool Verification

### 1.1 `execute_command(action, args)`

Primary dispatch tool. Verify CLI integration through the MCP boundary.

| Test Case | Action | Arguments | Expected Output |
| --- | --- | --- | --- |
| T-EXEC-01 | `check` | `{"path": "."}` | JSON response with `quality_score` (int) and `violations` (list) |
| T-EXEC-02 | `fix` | `{"path": "."}` | `remediation_summary` containing file count and fixed issue IDs |
| T-EXEC-03 | `report` | `{"path": ".", "format": "json"}` | Valid JSON payload conforming to `ReportFormatterProcessor.report_to_dict` |
| T-EXEC-04 | `security` | `{"path": "."}` | List of Bandit-style violations with `severity` and `confidence` fields |

### 1.2 `list_commands(domain)`

Verify tool discovery and command mapping.

| Domain | Expected Commands |
| --- | --- |
| `core` | `check`, `scan`, `fix`, `report`, `ci`, `version`, `adapters`, `config`, `security`, `cancel` |
| `analysis` | `complexity`, `duplicates`, `trends`, `dependencies` |
| `setup` | `setup init`, `setup doctor`, `setup mcp-config`, `setup hermes` |
| `dev` | `diff`, `suggest`, `import`, `export`, `watch`, `install-hook`, `uninstall-hook` |
| `git` | `git-diff`, `multi-project` |

The single source of truth for this catalog is `src-rust/taxonomy/command_catalog_constant.rs::COMMAND_CATALOG`.

### 1.3 `commands_schema(tool_name)`

Verify JSON Schema retrieval.

- **Requirement**: `commands_schema("execute_command")` must return the same `inputSchema` advertised in `tools/list`.
- **Requirement**: Unknown `tool_name` returns a JSON error object, not a panic.

### 1.4 `read_skill_context(section)`

Verify documentation retrieval.

- **Requirement**: Section `directives` returns rules for agentic behavior.
- **Requirement**: Section `mcp-tools` returns technical signatures for all 5 tools.
- **Requirement**: Empty / missing `section` returns the full SKILL.md.

### 1.5 `health_check()`

Verify infrastructure connectivity.

- **Requirement**: Reports adapter liveness for: `rust_linter_adapter`, `python_ruff_adapter`, `python_mypy_adapter`, `python_bandit_adapter`, `python_metrics_adapter`, `javascript_linter_adapter`.
- **Requirement**: Reports `cargo` toolchain status (parsed from `cargo --version`).

---

## 2. Agentic Engineering System (AES) Validation

Verify the governance engine's penalty logic and rule detection. The current rule catalog spans **AES001 through AES033** (AES028 and AES029 are reserved for future use).

| Rule ID | Violation Type | Test Action | Expected Result Code | Severity | Penalty |
| --- | --- | --- | --- | --- | --- |
| AES001 | Layering | `surfaces` → `infrastructure` import | `import-layer-violation` | CRITICAL | -5 |
| AES003 | Naming | Create `bad_name.rs` | `naming-convention` | HIGH | -3 |
| AES004 | File size | Create file > 500 lines | `file-too-large` | MEDIUM | -2 |
| AES005 | File size | Create file < 10 lines | `file-too-short` | LOW | -1 |
| AES006 | Primitives (contract) | Add raw `String` to port signature | `primitive-usage` | HIGH | -3 |
| AES006 | Primitives (taxonomy entity) | Add `String` field to entity | `primitive-usage` | HIGH | -3 |
| AES006 | Primitives (infra — allowed) | Add `String` param to infra adapter | *No violation* (`no_primitives: false`) | — | 0 |
| AES009 | Structure | Module without a struct | `mandatory-class-definition` | HIGH | -3 |
| AES014 | Bypass | `#[allow(...)]` on lint rule | `bypass-comment-violation` | CRITICAL | -5 |
| AES015 | Unused mandatory | Import contract, never use it | `unused-mandatory-import` | MEDIUM | -2 |
| AES016 | Dead inheritance | Empty struct inheriting from contract | `dead-inheritance-bypass` | CRITICAL | -5 |
| AES017 | Orphan | Taxonomy file unreachable from any surface | `orphan-code-detection` | MEDIUM | -2 |
| AES024 | Agent `Any` | `let x: Box<dyn Any>` in agent module | `agent-any-bypass` | CRITICAL | -5 |
| AES025 | MCP schema | Tool with empty `description` | `mcp-tool-schema-violation` | HIGH | -3 |
| AES027 | Mandatory inheritance | File imports contract, no struct inherits | `mandatory-inheritance` | HIGH | -3 |
| AES030 | Dispatch method | `COMMAND_CATALOG` references missing method | `capability-method-not-found` | HIGH | -3 |
| AES033 | Constant purity | `struct` or `fn` inside `_constant.rs` | `constant-purity` | HIGH | -3 |

---

## 3. Core Linter Integrations

Verify binary bridge and output parsing for each adapter.

| Adapter | Language | Integration Type | Output Format |
| --- | --- | --- | --- |
| `rust_linter_adapter` | Rust | `cargo clippy --message-format=json` | JSON |
| `python_ruff_adapter` | Python | `ruff check --output-format=json` | JSON |
| `python_mypy_adapter` | Python | `mypy --output=json` | JSON |
| `python_bandit_adapter` | Python | `bandit -f json` | JSON |
| `python_metrics_adapter` | Python | Library (cyclomatic scoring) | A-F grades |
| `javascript_linter_adapter` | JS/TS | `eslint --format=json` | JSON |
| `ast_rust_scanner` | Rust | `syn` parse (in-process) | LintResultList |
| `ast_py_scanner` | Python | `rustpython-parser` (in-process) | LintResultList |
| `ast_js_scanner` | JS/TS | `tree-sitter-javascript` (in-process) | LintResultList |

---

## 4. Quality Score Calculation

The scoring algorithm is `Score = 100 - SUM(Severity.score_impact())` (no lower bound).

1. Run `check` on a directory with 1 Naming Violation (AES003, HIGH = -3) and 1 Layering Violation (AES001, CRITICAL = -5).
2. **Success Criteria**: Quality Score must be `92` and `has_critical` must be `true` (run exits with code 1).
3. Run `check` on a directory with 1 Naming Violation (AES003, -3) and 1 Bypass Violation (AES014, -5).
4. **Success Criteria**: Quality Score must be `92` and compliance fails (CRITICAL found).

Reference implementation: `compute_score` in `src-rust/cli_main_entry.rs:455`.

---

## 5. Report Integrity

Verify output format consistency.

- **Text** (default): Human-readable report from `ReportFormatterProcessor.format()`.
- **JSON**: Dictionary with `meta`, `summary`, and `details` keys (matches `report_to_dict`).
- **SARIF 2.1.0**: Conformance to `https://json.schemastore.org/sarif-2.1.0.json`. Output includes `$schema`, `version: "2.1.0"`, `runs[].tool.driver.name = "lint-arwaky"`, and `runs[].tool.driver.version`.
- **JUnit XML**: Valid XML with `testsuite` and `testcase` / `failure` tags for CI gates.

---

## 6. Self-Lint Target

The project audits itself under the same AES rule engine it exposes.

```bash
cargo build --release
./target/release/lint-arwaky-cli check .
# Expected: 0 CRITICAL findings; score equals 100 - sum(HIGH) - sum(MEDIUM) - sum(LOW)
```

Any regression in AES rule enforcement MUST be caught by the self-lint target on CI.

# Verification Suite: Lint-Arwaky v1.9.4
---

## 1. MCP Tool Verification

### 1.1 execute_command(action, args)
Verify core CLI integration.

| Test Case | Action | Arguments | Expected Output Criteria |
| :--- | :--- | :--- | :--- |
| T-EXEC-01 | `check` | `{"path": "."}` | JSON response with `quality_score` (int) and `violations` (list). |
| T-EXEC-02 | `fix` | `{"path": "."}` | `remediation_summary` containing file count and fixed issue IDs. |
| T-EXEC-03 | `report`| `{"path": ".", "format": "json"}` | Valid JSON payload conforming to `ReportVO` schema. |
| T-EXEC-04 | `security`| `{"path": "."}` | List of Bandit violations with `severity` and `confidence` fields. |

### 1.2 list_commands(domain)
Verify tool discovery and command mapping.

| Domain | Expected Commands |
| :--- | :--- |
| `core` | `check`, `scan`, `fix`, `report`, `ci`, `version`, `adapters`, `security`, `cancel` |
| `analysis` | `complexity`, `duplicates`, `trends`, `dependencies`, `batch` |
| `setup` | `setup init`, `setup hermes`, `setup doctor`, `setup mcp-config` |

### 1.3 read_skill_context(section)
Verify documentation retrieval.

- **Requirement**: Section `directives` must return rules for agentic behavior.
- **Requirement**: Section `mcp-tools` must return technical signatures for all 5 tools.

### 1.4 health_check()
Verify infrastructure connectivity.

- **Requirement**: Must verify linter binary state.
- **Requirement**: Must list all 12+ linter adapters and their binary status.

---

## 2. Agentic Engineering System (AES) Validation
Verify the governance engine's penalty logic and rule detection.

| Rule ID | Violation Type | Test Action | Expected Result Code | Severity | Penalty |
| :--- | :--- | :--- | :--- | :--- | :--- |
| AES003 | Naming | Create `bad_name.py` | `FILE_NAMING_VIOLATION` | HIGH | -3 |
| AES009 | Structure | Module without a class | `CLASS_MANDATORY_VIOLATION` | HIGH | -3 |
| AES001 | Layering | `surfaces` -> `infrastructure` | `LAYER_BOUNDARY_VIOLATION` | CRITICAL | -5 |
| AES014 | Bypass | Usage of `# noqa` on AES rule | `SECURITY_BYPASS_VIOLATION` | CRITICAL | -5 |
| AES006 | Primitives (contract) | Add `str` param to contract port | `PRIMITIVE_USAGE` | HIGH | -3 |
| AES006 | Primitives (taxonomy entity) | Add `str` field to entity | `PRIMITIVE_USAGE` | HIGH | -3 |
| AES006 | Primitives (infra — allowed) | Add `str` param to infra adapter | *No violation* (no_primitives=false) | — | 0 |

---

## 3. Core Linter Integrations
Verify binary bridge and output parsing for each adapter.

| Linter | Domain | Integration Type | Output Format |
| :--- | :--- | :--- | :--- |
| **Ruff** | Python | Binary CLI | Multi-line text / JSON |
| **MyPy** | Python | Binary CLI | Standard error/out |
| **Bandit** | Security | Binary CLI | Severity-based JSON |
| **ESLint** | JS/TS | Node.js | Standard JSON |
| **Radon** | Complexity| Library | Cyclomatic score (A-F) |
| **pip-audit** | Deps | Binary CLI | CVE list |

---

## 4. Quality Score Calculation
Verify the scoring algorithm: `Score = 100 - SUM(Penalties)`.

1. Run `check` on a directory with 1 Naming Violation (-3 HIGH) and 1 Layering Violation (-5 CRITICAL).
2. **Success Criteria**: Quality Score must be `92`.
3. Run `check` on a directory with 1 Naming Violation (-3 HIGH) and 1 Bypass Violation (-5 CRITICAL).
4. **Success Criteria**: Quality Score must be `92` and compliance fails (CRITICAL found).

---

## 5. Report Integrity
Verify output format consistency.

- **SARIF**: Conformance to static analysis results interchange format (v2.1.0).
- **JUnit**: Valid XML structure with `testcase` and `failure` tags for CI gates.
- **JSON**: Dictionary with `meta`, `summary`, and `details` keys.

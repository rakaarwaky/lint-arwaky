
# Business Analyst Review: `config-system` v1.10.106

## Executive Summary

The `config-system` crate serves as the configuration backbone for the `lint-arwaky` tool — responsible for loading, parsing, validating, and detecting workspace configurations. While the implementation is architecturally sound (0 AES violations detected), the **requirements documentation (FRD.md) is critically underdeveloped** relative to the actual system behavior encoded in the source. This creates significant traceability risk, onboarding friction, and regression vulnerability.

---

## 1. Requirement Clarity Assessment

### 1.1 FRD.md — Current State

| #  | Requirement Statement                                                                                    | Clarity Rating  | Issue                                                                                                                                                              |
| -- | -------------------------------------------------------------------------------------------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R1 | "ConfigLoadingOrchestrator — Coordinates the configuration loading process from various sources."       | ⚠️ Vague      | "Various sources" is undefined. What sources? What priority order? What happens on conflict?                                                                       |
| R2 | "ConfigRulesValidator — Validates loaded configuration rules against the defined schema."               | ⚠️ Vague      | "Defined schema" is never specified. Which fields? What ranges? What constitutes a valid vs. invalid rule?                                                         |
| R3 | "WorkspaceDetector — Detects Rust workspace roots based on Cargo.toml or common project roots."         | ⚠️ Incomplete | Only mentions Rust. The implementation also detects TypeScript (`package.json`) and Python (`pyproject.toml`, `setup.py`, `requirements.txt`).             |
| R4 | "ConfigParserProvider — Provides parsers for YAML, TOML (Cargo.toml), and other configuration formats." | ⚠️ Ambiguous  | "Other configuration formats" is open-ended. What other formats? Is this extensible?                                                                               |
| R5 | "ConfigYamlReader — Reads and parses the main YAML configuration file."                                 | ⚠️ Incomplete | Doesn't mention XDG Base Directory compliance, parent-directory traversal (depth=2), or multi-language filename resolution.                                        |
| R6 | "MultiProjectOrchestrator — Manages configuration for multiple projects/workspaces simultaneously."     | ❌ Missing      | **No corresponding implementation file exists.** `discover_workspaces()` lives inside `ConfigOrchestrator`, not a separate `MultiProjectOrchestrator`. |

### 1.2 Identified Ambiguities

| ID     | Ambiguity                                                                                               | Location                                        | Impact                                           |
| ------ | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------ |
| AMB-01 | Config file naming convention (`lint_arwaky.config.{language}.yaml`) is never stated as a requirement | FRD.md                                          | Developers must reverse-engineer from code       |
| AMB-02 | Fallback behavior (embedded defaults when no file found) is undocumented                                | FRD.md                                          | Stakeholders unaware of silent fallback behavior |
| AMB-03 | "Merge with project-level overrides" — merge strategy (deep? shallow? last-wins?) undefined            | FRD.md                                          | Risk of incorrect override behavior              |
| AMB-04 | Workspace detection depth limit (max 2 parent levels) is an implicit design decision                    | `capabilities_workspace_detector_provider.rs` | Not traceable to any requirement                 |
| AMB-05 | Warning vs. Error semantics — when is a missing config a warning vs. a failure?                        | `agent_config_orchestrator.rs`                | Unclear error-handling contract for consumers    |

---

## 2. Completeness Gap Analysis

### 2.1 Functional Gaps (Implemented but Not Required)

The following behaviors exist in code but have **no corresponding requirement** in the FRD:

| Gap ID | Behavior                                                                                              | Source File                                                        | Risk                                              |
| ------ | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------- |
| GAP-01 | XDG Base Directory Specification compliance for config lookup                                         | `capabilities_yaml_reader.rs` → `read_any()`                  | Regulatory/portability requirement unstated       |
| GAP-02 | `XDG_CONFIG_DIRS` environment variable parsing with `/etc/xdg` fallback                           | `capabilities_yaml_reader.rs`                                    | Deployment behavior undocumented                  |
| GAP-03 | Parent directory traversal (depth ≤ 2) for config discovery                                          | `capabilities_yaml_reader.rs` → `read_config()`               | Search scope undefined in requirements            |
| GAP-04 | Workspace folder context inference (`crates/` → Rust, `packages/` → TS, `modules/` → Python) | `capabilities_workspace_detector_provider.rs`                    | Detection heuristic undocumented                  |
| GAP-05 | Default layer injection when config has empty`layers`                                               | `agent_config_orchestrator.rs` → `load_config_for_language()` | Silent mutation of user config                    |
| GAP-06 | Concurrent workspace discovery via`join_all`                                                        | `agent_config_orchestrator.rs` → `discover_workspaces()`      | Performance/concurrency requirement missing       |
| GAP-07 | TOML`[tool.lint-arwaky]` / `[tool.lint_arwaky]` section extraction                                | `capabilities_parser_provider.rs`                                | Integration contract with Cargo.toml undocumented |
| GAP-08 | `list_config_files()` — multi-language config enumeration                                          | `contract_reader_protocol.rs`                                    | Feature exists but no requirement                 |

### 2.2 Non-Functional Gaps

| Category                 | Missing Requirement                                                                                 |
| ------------------------ | --------------------------------------------------------------------------------------------------- |
| **Performance**    | No latency/throughput targets for config loading (especially`discover_workspaces` with async I/O) |
| **Error Handling** | No error taxonomy or escalation policy defined (warnings printed to stderr vs. returned)            |
| **Security**       | No mention of path traversal protection, symlink handling, or config file permission checks         |
| **Extensibility**  | No plugin/adapter registration mechanism documented for new config formats                          |
| **Observability**  | `eprintln!` used for warnings — no structured logging requirement                                |
| **Compatibility**  | No versioning strategy for config file schema evolution                                             |

### 2.3 Phantom Requirement

| ID         | Issue                                                                                                                                                                                                                                                                                     |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PHANTOM-01 | **"MultiProjectOrchestrator"** is listed in the FRD as a distinct component but does not exist as a separate module. The functionality is embedded in `ConfigOrchestrator::discover_workspaces()`. This creates a false expectation for stakeholders and breaks 1:1 traceability. |

---

## 3. Testability Assessment

### 3.1 Current State

| Requirement       | Testable?      | Reason                                                                     |
| ----------------- | -------------- | -------------------------------------------------------------------------- |
| R1 (Orchestrator) | ❌             | "Various sources" — no acceptance criteria, no input/output specification |
| R2 (Validator)    | ⚠️ Partially | Threshold ranges are in code (0–100, >0) but not in requirements          |
| R3 (Detector)     | ⚠️ Partially | Only Rust mentioned; TS/Python detection untestable from FRD alone         |
| R4 (Parser)       | ❌             | "Other formats" is unbounded — cannot define test completion criteria     |
| R5 (Reader)       | ❌             | No file path pattern, no search depth, no fallback behavior specified      |
| R6 (MultiProject) | ❌             | Component doesn't exist as described                                       |

### 3.2 Missing Acceptance Criteria

None of the 6 requirements have:

- **Given/When/Then** scenarios
- **Boundary conditions** (empty file, malformed YAML, permission denied, symlink loops)
- **Expected outputs** (return types, error codes, warning messages)
- **Performance thresholds** (max discovery time for N workspaces)

---

## 4. Conflict & Inconsistency Detection

| ID     | Conflict                                | Details                                                                                                                                                                                                                                                                                                                                                                           |
| ------ | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CON-01 | **FRD vs. Architecture**          | FRD lists`MultiProjectOrchestrator` as a separate component. AES Architecture §9 mandates Agent role = `orchestrator` (singular). Having two orchestrators in one crate violates single-responsibility at the Agent layer.                                                                                                                                                   |
| CON-02 | **FRD vs. Implementation**        | FRD says "WorkspaceDetector — Detects**Rust** workspace roots." Implementation detects Rust, TypeScript, AND Python. The FRD understates scope by 66%.                                                                                                                                                                                                                     |
| CON-03 | **Naming vs. Architecture §3**   | Architecture mandates`layer_concern_role` naming. FRD uses PascalCase component names (`ConfigLoadingOrchestrator`) that don't map to file names (`agent_config_orchestrator.rs`). No glossary bridges this gap.                                                                                                                                                            |
| CON-04 | **Contract vs. Agent dependency** | Architecture §9: "Agent must depend on Contract, not concrete implementations."`ConfigOrchestrator` calls `parse_config_yaml()` and `default_config_for_language()` — **free functions from Taxonomy** — directly. This is architecturally permitted (Taxonomy dependency) but creates hidden coupling to parsing logic that should arguably be behind a Protocol. |

---

## 5. Traceability Matrix (Current)

| FRD Requirement               | Source File(s)                                  | Contract                         | Test Coverage   |
| ----------------------------- | ----------------------------------------------- | -------------------------------- | --------------- |
| R1: ConfigLoadingOrchestrator | `agent_config_orchestrator.rs`                | `IConfigOrchestratorAggregate` | ❌ None visible |
| R2: ConfigRulesValidator      | `capabilities_rules_validator.rs`             | `IConfigValidatorProtocol`     | ❌ None visible |
| R3: WorkspaceDetector         | `capabilities_workspace_detector_provider.rs` | `IWorkspaceDetectorProtocol`   | ❌ None visible |
| R4: ConfigParserProvider      | `capabilities_parser_provider.rs`             | `IConfigParserProtocol`        | ❌ None visible |
| R5: ConfigYamlReader          | `capabilities_yaml_reader.rs`                 | `IConfigReaderProtocol`        | ❌ None visible |
| R6: MultiProjectOrchestrator  | ⚠️ Embedded in R1's file                      | ⚠️ Same aggregate              | ❌ None visible |

**Traceability Score: ~40%** — Requirements map to files but lack acceptance criteria, test cases, and behavioral specifications.

---

## 6. Recommendations

### 🔴 Critical (Address Before Next Release)

| # | Recommendation                                                                                                                                                                                                             | Rationale                                                                            |
| - | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| 1 | **Rewrite FRD with structured user stories** using format: *"As a [role], I need [capability] so that [business value]. Acceptance: Given/When/Then."*                                                             | Current FRD is a component list, not a requirements document.                        |
| 2 | **Resolve PHANTOM-01**: Either extract `MultiProjectOrchestrator` into its own `agent_multi_project_orchestrator.rs` or remove it from the FRD and document `discover_workspaces()` as a sub-capability of R1. | False requirements erode stakeholder trust.                                          |
| 3 | **Specify the config resolution algorithm** as a numbered priority chain: (1) project-root YAML → (2) parent dir (depth ≤ 2) → (3) XDG user config → (4) XDG system dirs → (5) embedded defaults.               | This is the core business logic and it's entirely implicit.                          |
| 4 | **Define error/warning taxonomy**: Which failures are fatal? Which produce warnings? What is the consumer's contract?                                                                                                | Currently`eprintln!` is used ad hoc — no programmatic error handling for callers. |

### 🟡 High Priority (Next Sprint)

| # | Recommendation                                                                                                                                                    | Rationale                                                                     |
| - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| 5 | **Add a Requirements Glossary** mapping FRD component names → file names → contract traits → public API methods.                                         | Bridges the naming gap between business stakeholders and developers.          |
| 6 | **Document the merge strategy** for `utility_config_merger.rs`: field-level merge rules, conflict resolution, array concatenation vs. replacement.        | "Merge correctness" is a success indicator but merge semantics are undefined. |
| 7 | **Add boundary/edge-case requirements**: empty YAML, YAML with unknown fields, circular symlinks, read-permission denied, 0-byte files, BOM-prefixed files. | Enables comprehensive test design.                                            |
| 8 | **Specify NFR targets**: config load time < Xms for single project; workspace discovery < Yms for N members; memory ceiling for large configs.              | Currently no performance contract exists.                                     |

### 🟢 Medium Priority (Backlog Refinement)

| #  | Recommendation                                                                                                                               | Rationale                                                                                                                                                                                                |
| -- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 9  | **Replace `eprintln!` with structured logging** (e.g., `tracing` crate) and add an observability requirement.                      | Production debuggability; stderr is not machine-parseable.                                                                                                                                               |
| 10 | **Add config schema versioning** (`schema_version: 1`) and a forward-compatibility requirement.                                      | Prevents breaking changes for existing users.                                                                                                                                                            |
| 11 | **Document the `WorkspaceType::Unknown` fallback behavior** — what happens downstream? Is it an error? A no-op?                     | Currently returns`Unknown` and proceeds with `"unknown"` language string → `default_config_for_language` prints a warning and returns empty config. This chain should be an explicit requirement. |
| 12 | **Add a stakeholder sign-off section** to the FRD with roles: Product Owner, Tech Lead, QA Lead, DevOps (for XDG/deployment concerns). | Ensures shared ownership of requirements.                                                                                                                                                                |

---

## 7. Business Value Alignment Check

| Success Indicator (FRD) | Measurable?  | Current Evidence                                                                    |
| ----------------------- | ------------ | ----------------------------------------------------------------------------------- |
| Discovery reliability   | ❌ No metric | No test workspaces, no detection accuracy target                                    |
| Validation accuracy     | ❌ No metric | Only 3 threshold checks implemented; no schema validation for layers/rules          |
| Merge correctness       | ❌ No metric | `utility_config_merger.rs` has 2 unit tests (empty + global rule) — insufficient |
| Rule conformance (AES)  | ✅ Binary    | `lint-arwaky-cli scan` reports 0 violations                                       |

**Verdict**: 3 of 4 success indicators are **not measurable** in their current form. They need quantified targets (e.g., "≥95% detection accuracy across 20 reference project structures").

---

## 8. Summary Scorecard

| Dimension          | Score (1–5)    | Notes                                                    |
| ------------------ | --------------- | -------------------------------------------------------- |
| Clarity            | 2/5             | Component names only; no behavioral specs                |
| Completeness       | 2/5             | ~60% of implemented behavior undocumented                |
| Testability        | 1/5             | Zero acceptance criteria; no GWT scenarios               |
| Traceability       | 2/5             | File-level mapping exists; requirement-level does not    |
| Consistency        | 3/5             | One phantom component; one scope understatement          |
| Business Alignment | 2/5             | Success indicators not measurable                        |
| **Overall**  | **2.0/5** | Implementation quality far exceeds documentation quality |

---

> **Bottom Line**: The `config-system` crate is well-engineered code with clean AES compliance, but it is **flying without a requirements contract**. The FRD reads as a developer's TODO list rather than a stakeholder agreement. Investing 2–3 days in requirements elaboration will pay dividends in regression safety, onboarding speed, and cross-team alignment.

---

*Prepared for: Raka*
*Review Date: July 21, 2026*
*Document Version: config-system v1.10.106*

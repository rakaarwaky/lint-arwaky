As an Expert Business Analyst, I have conducted a comprehensive review of the provided documentation, specifically comparing the **Feature Requirements Document (`FRD.md`)** against the **Architecture Guidelines (`ARCHITECTURE.md`)** and the **Actual Implementation (`taxonomy_cli_vo.rs`, `surface_*.rs`)**.

While the technical implementation is highly sophisticated and strictly adheres to the Agentic Engineering System (AES) layered architecture, **there is a significant disconnect between the written requirements (FRD) and the actual product scope.**

Below is my detailed analysis, gap identification, and strategic recommendations to align business value, ensure testability, and satisfy stakeholders (including AI agents and human developers).

---

### 1. Critical Gaps & Discrepancies (FRD vs. Implementation)

The most pressing issue is that the FRD does not reflect the current state of the product, leading to severe traceability failures.

- **Missing `tui` Command:**
  - _FRD Requirement:_ Explicitly lists `tui — launch the interactive terminal UI`.
  - _Implementation:_ The `tui` command is **completely absent** from the Clap CLI definition (`taxonomy_cli_vo.rs`) and there is no `surface_tui_command.rs`.
  - _Impact:_ Broken promise to stakeholders; missing feature in release notes.
- **Orphaned CLI Definition (`duplicates`):**
  - _Implementation:_ `taxonomy_cli_vo.rs` defines a `Duplicates` subcommand.
  - _Gap:_ There is no corresponding `surface_duplicates_command.rs` or handler in `lib.rs`.
  - _Impact:_ Executing `lint-arwaky duplicates` will likely result in a runtime panic or unhandled match arm, causing a critical user-facing bug.
- **Command Naming & Mapping Mismatch:**
  - _FRD:_ Lists `git`, `config`, and `setup`.
  - _Implementation:_ The code uses `check --git-diff`, `install-hook`/`uninstall-hook`, `config-show`, `init`, and `install`.
  - _Impact:_ Users reading the FRD will be confused by the actual CLI help menu (`--help`).

### 2. Ambiguities & Conflicting Requirements

- **Conflict: `check` vs. `scan` Business Logic:**
  - _FRD Definition:_ `check` is for a "single file or directory". `scan` is for the "entire workspace".
  - _Code Reality (`surface_check_action.rs`):_ `check` is explicitly documented as "self-lint the lint-arwaky project itself", while `scan` is for "external project + external adapters".
  - _BA Recommendation:_ We must redefine these terms. `check` should be the universal command for analyzing a target path, while `scan` should be deprecated or repurposed as an alias for multi-workspace discovery.
- **Ambiguity in "Auto-Fix" Safety:**
  - _FRD:_ States `fix — apply automatic fixes`.
  - _Code Reality:_ Implements a `--dry-run` flag and relies on a factory pattern to simulate fixes.
  - _BA Recommendation:_ The FRD must explicitly state the safety boundaries of the `fix` command. Which rules are auto-fixable? (Code comments indicate AES101, AES203, AES304). What is the rollback mechanism if a fix corrupts a file?

### 3. Testability & Acceptance Criteria Review

The **Success Indicators** in the FRD are currently qualitative and untestable. They need to be converted into strict, measurable Acceptance Criteria (AC).

| Current FRD Success Indicator | BA Critique                                                           | Proposed Measurable Acceptance Criteria                                                                                                                                                          |
| :---------------------------- | :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **UX consistency**            | Too vague. What defines consistency?                                  | **AC1:** All commands must support `--format [text, json, sarif, junit]`. **AC2:** All commands must return standardized POSIX exit codes (0 = success, 1 = violations found, 2 = system error). |
| **Performance**               | "Buffered output" is an implementation detail, not a business metric. | **AC1:** CLI must parse and output results for a 10,000-file workspace in < 3 seconds. **AC2:** Memory consumption must not exceed 500MB during a full workspace `scan`.                         |
| **Help documentation**        | Good, but needs enforcement.                                          | **AC1:** Every subcommand must have a `#[command(about = ...)]` and `#[arg(help = ...)]` annotation in `taxonomy_cli_vo.rs`.                                                                     |
| **Rule conformance**          | Excellent, aligns with AES architecture.                              | **AC1:** The `lint-arwaky` repository must pass its own `ci` command with a threshold of 100 in the CI/CD pipeline.                                                                              |

### 4. Scope Creep & Undocumented Features

The codebase contains several high-value features that are completely missing from the FRD. As a BA, I must flag these as **Undocumented Scope**. If they are intentional, the FRD must be updated. If not, they represent wasted engineering effort.

- `ci` (CI-optimized with threshold comparison)
- `orphan` (Dead code detection)
- `security` (Vulnerability scanning via Bandit/Cargo-Audit)
- `dependencies` (Dependency reporting)
- `mcp-config` (Model Context Protocol integration for AI agents)
- `doctor` (Environment diagnostics)

_Note: The inclusion of `mcp-config` and `orphan` heavily aligns with the "AI-native" and "Agentic" goals mentioned in `ARCHITECTURE.md`, but the FRD fails to capture this strategic business value._

### 5. Missing Non-Functional Requirements (NFRs)

The FRD lacks critical NFRs required for a CLI tool intended for CI/CD and AI Agent consumption:

1. **Interoperability:** The CLI must output SARIF (Static Analysis Results Interchange Format) for GitHub/GitLab integration. _(Note: The code actually supports this via `Format::Sarif`, but the FRD doesn't mandate it)._
2. **Observability:** How are errors logged? The code uses `tracing`, but the FRD doesn't define log levels or structured logging requirements for the `--verbose` flag.
3. **Cross-Platform Compatibility:** The tool supports Rust, Python, and JS. The FRD must explicitly state support for Windows (PowerShell/CMD), macOS, and Linux pathing behaviors.

---

### 6. Strategic Recommendations (Action Plan)

To bridge the gap between business requirements and technical execution, I recommend the following immediate actions:

#### Action 1: Rewrite the FRD to Match the AES Architecture

The current FRD reads like a generic CLI tool. It must be rewritten to reflect the **Agentic Engineering System (AES)**.

- **New Feature Goal:** "Provide a unified, AI-native CLI and MCP (Model Context Protocol) surface that orchestrates AES compliance, security, and structural linting across multi-language workspaces."
- **Update Command Scope:** Group commands logically:
  - _Analysis:_ `check`, `scan`, `ci`, `orphan`
  - _Remediation:_ `fix`
  - _Environment:_ `doctor`, `security`, `dependencies`
  - _Configuration:_ `init`, `config-show`, `mcp-config`
  - _Automation:_ `watch`, `install-hook`

#### Action 2: Fix the Code-Level Defects

- **Remove or Implement `Duplicates`:** Either delete the `Duplicates` enum from `taxonomy_cli_vo.rs` or assign a developer to build `surface_duplicates_command.rs`.
- **Clarify `check` vs `scan`:** Refactor `surface_check_action.rs` so `check` is the universal entry point, and `scan` is explicitly documented as an alias for multi-workspace discovery.

#### Action 3: Establish a Requirements Traceability Matrix (RTM)

Create a mapping between the Clap CLI definitions and the Surface Aggregates to ensure no dead code or missing UI surfaces exist.

| CLI Command (`taxonomy_cli_vo.rs`) | Surface Handler                  | Aggregate Contract             | Status                  |
| :--------------------------------- | :------------------------------- | :----------------------------- | :---------------------- |
| `Check` / `Scan`                   | `surface_check_command.rs`       | `ICodeAnalysisAggregate`       | ✅ Aligned              |
| `Fix`                              | `surface_fix_command.rs`         | `LintFixOrchestratorAggregate` | ✅ Aligned              |
| `Doctor` / `Security`              | `surface_maintenance_command.rs` | `MaintenanceCommandsAggregate` | ⚠️ Missing from FRD     |
| `Duplicates`                       | **MISSING**                      | N/A                            | ❌**Critical Defect**   |
| `Tui`                              | **MISSING**                      | N/A                            | ❌**FRD/Code Mismatch** |

#### Action 4: Define the AI-Agent Handoff (MCP)

Since the architecture explicitly mentions "AI agents" and the code includes `mcp-config`, the FRD must include a section on **Agent Interoperability**.

- _Requirement:_ The CLI must be callable via standard input/output (Stdio) for MCP clients (Claude, Cursor, Windsurf) without requiring terminal TTY allocation.

### Summary

The engineering team has built a robust, highly structured, and architecturally sound system (AES). However, the **FRD is obsolete and fails to capture the true scope and value of the product**. By updating the FRD to include the CI/CD, Security, and MCP features, resolving the `check`/`scan` semantic conflict, and cleaning up orphaned CLI definitions, we will ensure the product is testable, marketable, and perfectly aligned with its AI-native business goals.

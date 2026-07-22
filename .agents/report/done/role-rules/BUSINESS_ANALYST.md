# Business Analysis Review: `role-rules` v1.10.106

## Executive Summary

The `role-rules` crate implements architectural boundary enforcement (AES401–AES406) for the Agentic Engineering System. After a thorough cross-referencing of the **FRD**, **ARCHITECTURE.md**, and **source implementation**, I've identified **critical gaps between stated requirements and delivered behavior**, several ambiguities that would impede QA sign-off, and traceability breakdowns that increase regression risk. Below is a structured findings report with prioritized recommendations.

---

## 1. Requirement Clarity Assessment

### 1.1 Well-Defined Requirements ✅

| Req ID           | Statement                                                | Verdict         |
| ---------------- | -------------------------------------------------------- | --------------- |
| AES401-R1        | Constant files must only contain`pub const`/`pub static` | Clear, testable |
| AES402           | Contract signatures must not use raw primitives          | Clear intent    |
| AES405 (partial) | Agent files must not exceed line limit                   | Measurable      |

### 1.2 Ambiguous Requirements ⚠️

| Req ID        | Issue                                                                                                                                                                                                                                                                                              | Impact                                                  |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| **AES401-R2** | _"must not expose raw primitive types in their public interfaces"_ — What constitutes a "public interface"? The implementation checks struct fields but skips `pub(crate) value:` fields and `fn from()` constructors. No definition of "public" is given.                                         | Developers cannot predict what will be flagged.         |
| **AES403**    | _"must implement at least one defined contract protocol"_ — The implementation checks for `impl I{StructName}` (Rust-specific naming). No guidance for Python (`class Foo(Protocol)`) or JS (`implements`).                                                                                        | Multi-language teams get inconsistent enforcement.      |
| **AES405**    | _"must not use dynamic, generic, or untyped constructs (such as `any` in JS/TS or generic `Object`/`dyn Any` in Rust)"_ — Implementation **only** scans for `: any`, `: Any`, `-> any`, `Any<`, `Any[`, `any[`. It does **not** detect `dyn Any`, `Box<dyn Any>`, `Object`, `unknown`, or `never`. | Requirement overpromises; implementation underdelivers. |
| **AES406**    | _"must remain passive… must not contain core business logic, validation rules, or state mutation logic"_ — No operational definition of "core business logic." Implementation uses a heuristic of >3 control-flow statements or >10 public methods.                                                | Threshold is arbitrary and undocumented in the FRD.     |

### 1.3 Conflicting Requirements 🔴

| Conflict                            | Location A                                                                                       | Location B                                                                                                                | Resolution Needed                                                                  |
| ----------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| **Surface "router" classification** | ARCHITECTURE.md §10 table lists`router` under **Smart surfaces** (may initiate feature behavior) | Orchestrator code classifies`_router` as **utility surface** (`is_utility = filename.contains("_router")`)                | Which is authoritative? Smart surfaces get different checks than utility surfaces. |
| **Utility layer enforcement**       | FRD AES404:_"Utility files must NOT implement any contract\_ protocol"_                          | Orchestrator`match "utility" => {}` — **zero checks performed**                                                           | AES404 is a dead requirement.                                                      |
| **Precision reporting**             | FRD Success Indicator:_"reports violations pointing to the exact line and column numbers"_       | Multiple violations emit`line: 0, column: 0` (e.g., `CapabilityNoProtocol`, `AgentFileSizeLimit`, `SurfaceRoleViolation`) | Contradicts the stated success criterion.                                          |

---

## 2. Completeness Gap Analysis

### 2.1 Functional Gaps (Requirements → Implementation)

| FRD Requirement                                  | Implementation Status                                                          | Severity    |
| ------------------------------------------------ | ------------------------------------------------------------------------------ | ----------- |
| AES401-R2: VO primitive check                    | `check_vo()` returns **empty vec** — no-op                                     | 🔴 Critical |
| AES404: Utility purity enforcement               | **No checker exists**; orchestrator skips utility files entirely               | 🔴 Critical |
| AES405:`dyn Any` / generic `Object` detection    | Not implemented; only`any`/`Any` string matching                               | 🟡 High     |
| AES403: "at least one defined contract protocol" | Checks for protocol*import* existence, not actual trait implementation linkage | 🟡 High     |
| AES406: State mutation detection                 | Not implemented; only method count, line length, and if-nesting checked        | 🟡 Medium   |

### 2.2 Non-Functional Gaps

| Category                  | Missing Requirement                                                                                                                                                                                |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Performance**           | No SLA for scan time per 1,000 files. The orchestrator reads files synchronously with`std::fs::read_to_string` — no async I/O despite `async_trait` on the runner.                                 |
| **Scalability**           | No maximum file count or memory budget defined.                                                                                                                                                    |
| **Error Handling**        | `read_to_string(...).unwrap_or_default()` silently swallows I/O errors. No requirement states whether unreadable files should be reported or skipped.                                              |
| **Configuration**         | `max_lines` is hardcoded to `500` in the orchestrator. No FRD requirement specifies this value or makes it configurable.                                                                           |
| **Multi-language Parity** | FRD implies equal enforcement across Rust/Python/JS. Implementation is Rust-primary with partial Python/JS support (e.g.,`_check_rust_routing` is far more thorough than `_check_python_routing`). |
| **Idempotency**           | No requirement states whether repeated scans produce identical results.                                                                                                                            |

### 2.3 Missing Stakeholder Requirements

- **Developer feedback loop**: No requirement for auto-fix suggestions, IDE integration, or CI gate behavior.
- **Suppression mechanism**: No `#[allow(...)]` or inline-disable requirement documented.
- **Reporting format**: No requirement for JSON/SARIF/human-readable output differentiation.
- **Versioning policy**: No requirement for how rule additions/deprecations are communicated across versions.

---

## 3. Testability Assessment

### 3.1 Current State

The FRD's "Success Indicators" section contains **four unchecked checkboxes** with no:

- Acceptance criteria
- Measurable thresholds
- Test scenarios
- Expected input/output pairs

### 3.2 Testability Verdicts

| Requirement                  | Testable?  | Blocker                                                                                                      |
| ---------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ |
| AES401-R1 (constant purity)  | ✅ Yes     | Clear pass/fail: non-const declaration → violation                                                           |
| AES401-R2 (VO primitives)    | ❌ No      | `check_vo()` is a no-op; cannot test what doesn't exist                                                      |
| AES402 (contract primitives) | ⚠️ Partial | Testable for Rust traits; Python/JS detection uses different heuristics with no documented expected behavior |
| AES403 (capability routing)  | ⚠️ Partial | "≤ 3 structs" threshold is undocumented; >3 structs silently skips the check                                 |
| AES404 (utility purity)      | ❌ No      | No implementation to test                                                                                    |
| AES405 (agent purity)        | ⚠️ Partial | File size limit testable;`any`-type detection has false-positive risk (e.g., comments containing `: any`)    |
| AES406 (surface passive)     | ⚠️ Partial | Thresholds (10 methods, 80 lines, 3 if-depth) are implementation constants, not requirements                 |

### 3.3 Missing Test Artifacts

- No **boundary-value scenarios** (e.g., exactly 500 lines, exactly 15 functions, exactly 3 control-flow statements)
- No **negative test cases** (valid files that must NOT trigger violations)
- No **multi-language test matrix**
- No **regression suite** linked to requirement IDs

---

## 4. Traceability Matrix

| Req ID    | FRD §      | ARCH §          | Implementation File                                                      | Test Case | Status           |
| --------- | ---------- | --------------- | ------------------------------------------------------------------------ | --------- | ---------------- |
| AES401-R1 | FRD §Req 1 | §5 Taxonomy     | `capabilities_taxonomy_role_auditor.rs` → `check_constant_impl`          | ❌ None   | Partial          |
| AES401-R2 | FRD §Req 2 | §5 Taxonomy     | `capabilities_taxonomy_role_auditor.rs` → `check_vo_impl` (NO-OP)        | ❌ None   | **Broken**       |
| AES402    | FRD §Req   | §6 Contract     | `capabilities_contract_role_auditor.rs` → `check_contract_primitive`     | ❌ None   | Partial          |
| AES403    | FRD §Req   | §8 Capabilities | `capabilities_capabilities_role_auditor.rs` → `check_capability_routing` | ❌ None   | Partial          |
| AES404    | FRD §Req   | §7 Utility      | **NO IMPLEMENTATION**                                                    | ❌ None   | **Missing**      |
| AES405    | FRD §Req   | §9 Agent        | `capabilities_agent_role_auditor.rs`                                     | ❌ None   | Partial          |
| AES406    | FRD §Req   | §10 Surface     | `capabilities_surface_role_auditor.rs`                                   | ❌ None   | Partial          |
| AES013    | —          | —               | `capabilities_contract_role_auditor.rs` → `check_aggregate`              | ❌ None   | **Undocumented** |

> **Critical Finding**: AES013 (Forbidden Inheritance) is implemented but **never mentioned** in the FRD. This is an undocumented requirement — a traceability violation in both directions.

---

## 5. Business Value Alignment

### 5.1 Value Delivered

- Prevents architectural drift in AI-agent-modified codebases (core value proposition)
- Multi-language support increases addressable market
- Compile-time/scan-time detection reduces review cost

### 5.2 Value at Risk

| Risk                                    | Business Impact                                                                                                             |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| AES404 (Utility) unenforced             | Developers can place stateful, contract-implementing code in utility files without detection → architecture erodes silently |
| AES401-R2 (VO check) is a no-op         | Primitive obsession in value objects goes undetected → the "stable domain language" promise is broken                       |
| Inconsistent multi-language enforcement | Python/JS teams receive weaker guardrails → uneven quality across polyglot workspaces                                       |
| No suppression mechanism                | Teams will disable the entire linter rather than tolerate false positives → total value loss                                |

---

## 6. Prioritized Recommendations

### P0 — Critical (Block Release)

| #   | Recommendation                                                                                                                               | Rationale                                                                                                           |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| 1   | **Implement AES404 Utility Purity checker** or formally descope it from the FRD with a documented rationale.                                 | A stated requirement with zero enforcement is worse than no requirement — it creates false confidence.              |
| 2   | **Implement or explicitly deprecate `check_vo()`** (AES401-R2). Currently returns empty vec.                                                 | The FRD promises VO primitive detection; the code delivers nothing.                                                 |
| 3   | **Resolve the `router` classification conflict** between ARCHITECTURE.md and orchestrator code. Issue an ADR (Architecture Decision Record). | Smart vs. utility surface triggers different checks; misclassification causes missed violations or false positives. |
| 4   | **Document AES013** in the FRD or remove it from implementation.                                                                             | Undocumented behavior is untestable and creates stakeholder surprise.                                               |

### P1 — High (Next Sprint)

| #   | Recommendation                                                                                                                                                                              | Rationale                                                                    |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| 5   | **Add measurable acceptance criteria** to each AES requirement. Example: _"AES405: Any agent file exceeding 500 lines SHALL emit exactly one HIGH-severity violation at line 1, column 1."_ | Enables QA automation and removes ambiguity.                                 |
| 6   | **Fix precision reporting**: Ensure all violations carry actual line/column. Replace `line: 0` emissions with the first relevant line.                                                      | FRD success indicator explicitly promises "exact line and column numbers."   |
| 7   | **Define the "≤ 3 structs" threshold** for AES403 in the FRD and make it configurable via `ArchitectureConfig`.                                                                             | Currently a magic number in code; stakeholders cannot tune or understand it. |
| 8   | **Expand AES405 detection** to cover `dyn Any`, `Box<dyn Any>`, `Object`, `unknown` as stated in the FRD, or narrow the FRD language to match implementation.                               | Requirement-implementation mismatch erodes trust.                            |

### P2 — Medium (Backlog)

| #   | Recommendation                                                                                                                         | Rationale                                                                                 |
| --- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| 9   | **Add a suppression/allow-list mechanism** (e.g., `// lint-arwaky: allow AES401`).                                                     | Without escape hatches, teams disable the tool entirely.                                  |
| 10  | **Define multi-language parity requirements** explicitly: which rules apply to which languages, and at what fidelity.                  | Prevents implicit assumptions and guides implementation prioritization.                   |
| 11  | **Add performance NFRs**: e.g., "Scan 10,000 files in < 5 seconds on standard CI hardware."                                            | The synchronous`read_to_string` loop will not scale; sets expectation for async refactor. |
| 12  | **Create a requirement-to-test traceability matrix** as a living document (not just checkboxes).                                       | Enables impact analysis when rules change and supports audit compliance.                  |
| 13  | **Document error-handling policy**: Should unreadable files emit a warning violation? Should malformed content be skipped or reported? | Current`unwrap_or_default()` silently produces empty content → false negatives.           |

### P3 — Low (Continuous Improvement)

| #   | Recommendation                                                                                                                            | Rationale                                                                                                     |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| 14  | Introduce**property-based test scenarios** for each AES rule (e.g., "for any valid taxonomy constant file, zero violations are emitted"). | Increases confidence beyond example-based tests.                                                              |
| 15  | Add a**CHANGELOG section** to the FRD tracking requirement additions/removals per version.                                                | Supports stakeholder communication and migration planning.                                                    |
| 16  | Define**severity calibration guidelines** (when is a violation HIGH vs. MEDIUM?).                                                         | Current assignments appear arbitrary (e.g.,`CapabilityNoProtocol` is MEDIUM but `ContractPrimitive` is HIGH). |

---

## 7. Summary Scorecard

| Dimension                 | Score (1–5) | Notes                                                     |
| ------------------------- | :---------: | --------------------------------------------------------- |
| **Clarity**               |     3/5     | Core intent is clear; operational definitions are missing |
| **Completeness**          |     2/5     | Two requirements are entirely unimplemented; NFRs absent  |
| **Testability**           |     2/5     | No acceptance criteria, no test scenarios, magic numbers  |
| **Traceability**          |     2/5     | AES013 undocumented; AES404 unlinked; no test mapping     |
| **Consistency**           |    2.5/5    | Router conflict; precision promise vs. line-0 reality     |
| **Business Alignment**    |    3.5/5    | Core value proposition is strong; gaps undermine trust    |
| **Stakeholder Readiness** |    2.5/5    | FRD reads as developer notes, not a stakeholder contract  |

---

## 8. Closing Statement

The `role-rules` crate addresses a genuine and high-value need — enforcing architectural boundaries in AI-agent-modified codebases. However, the requirements documentation currently functions as **developer intent notes** rather than a **testable stakeholder contract**. The two critical unimplemented requirements (AES404, AES401-R2) and the undocumented AES013 rule create a gap between what is _promised_ and what is _delivered_ that will erode stakeholder confidence if not addressed before the next release milestone.

I recommend a **requirements baseline review workshop** with the architecture owner, QA lead, and a representative developer from each language track (Rust/Python/TS) to ratify the clarified requirements before implementation resumes.

---

_Prepared by: Business Analysis Review | Document Version: role-rules v1.10.106 | Date: 2026-07-21_

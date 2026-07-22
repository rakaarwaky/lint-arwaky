# Business Analyst Review: `orphan-detector` v1.10.106

## Executive Summary

The `orphan-detector` crate is a well-structured static analysis feature within the `lint-arwaky` ecosystem, targeting dead/unreachable code detection across a 7-layer architecture (AES501–AES506). While the implementation is technically mature, the **requirements documentation exhibits significant gaps in clarity, testability, and traceability** that pose risks to stakeholder confidence, regression safety, and long-term maintainability.

---

## 1. Requirement Clarity Assessment

### 1.1 FRD Requirement vs. Implementation Divergence

| Req ID | FRD Statement                                                                                     | Actual Implementation Behavior                                                                                  | Gap Severity                                                                                      |
| ------ | ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| AES501 | "Taxonomy files must be reachable from contracts, capabilities, or orchestrators"                 | Checks`inbound_links` from **any** non-taxonomy file (including surfaces, root, utility)                        | 🟡 Medium — FRD is**narrower** than implementation                                                |
| AES502 | "Contract files must have at least one active implementation in capabilities or utility layers"   | Checks implementation**AND** usage (called by orchestrator/container/surface). Three distinct sub-checks exist. | 🔴 High — FRD omits 2 of 3 validation paths                                                       |
| AES503 | "Capability files must be instantiated or imported by orchestrators**or other capability files**" | Checks reachability from entry points**AND** container wiring. Does **not** check inter-capability imports.     | 🔴 High — FRD contradicts architecture rule ("Capabilities must never import other Capabilities") |
| AES504 | "Utility files must be wired into root containers or imported by capabilities/agents"             | Checks if**any** file in `all_files` imports the module — no layer restriction                                  | 🟡 Medium — Implementation is**broader** than FRD                                                 |
| AES505 | "Agent orchestrator files must be called by surface layer files or binary entry points"           | Checks surfaces**AND** containers (`_container.rs/py/ts/js`)                                                    | 🟡 Medium — FRD omits container as valid caller                                                   |
| AES506 | "Surface files must be registered in the routing system or called from main entries"              | Checks reachability set**AND** identifier-level imports from entry/router files                                 | 🟡 Medium — "routing system" is undefined                                                         |

### 1.2 Ambiguities Identified

| #   | Ambiguity                                                                                                                                                                        | Location          | Impact                                                                            |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- | --------------------------------------------------------------------------------- |
| A1  | **"Reachable"** is never formally defined. Does it mean direct import? Transitive import? Runtime invocation?                                                                    | FRD §Requirements | Engineers interpret differently; graph BFS assumes transitive import reachability |
| A2  | **"Active implementation"** — what makes an implementation "active" vs. dead? Is a `#[cfg(test)]`-gated impl active?                                                             | AES502            | False positives in test-only code                                                 |
| A3  | **"Configurable exceptions and ignored path patterns"** — no schema, no default list, no override semantics documented                                                           | FRD §Requirements | Users cannot self-serve configuration                                             |
| A4  | **"Routing system"** (AES506) — no definition of what constitutes registration in routing                                                                                        | AES506            | Cannot verify compliance without knowing the routing mechanism                    |
| A5  | **"Binary entry points"** — the code recognizes `main.rs`, `lib.rs`, `index.ts`, `__main__.py`, `_entry.*`, `_container.*`, `root_*`. The FRD mentions none of these explicitly. | AES505/506        | Stakeholders cannot validate entry point coverage                                 |

---

## 2. Completeness Assessment

### 2.1 Missing Requirements

| Category                      | Missing Item                                                                                    | Business Risk                            |
| ----------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------- |
| **Error Handling**            | No requirements for behavior when files are unreadable, symlinked, or binary                    | Silent failures mask real orphans        |
| **Performance**               | "Less than a second" — for what project size? 100 files? 10,000? Multi-crate workspace?         | Cannot set SLA or capacity plan          |
| **Multi-Language**            | FRD mentions no language scope. Code supports Rust, Python, JS/TS. What about Go, Java, C#?     | Scope creep or unmet expectations        |
| **Incremental Analysis**      | No requirement for incremental/re-analysis on file change vs. full scan                         | CI/CD pipeline performance               |
| **Reporting**                 | No requirement for output format (JSON, SARIF, human-readable), aggregation, or deduplication   | Integration with CI/CD tools blocked     |
| **Severity Policy**           | Severity levels (HIGH/MEDIUM/LOW) are assigned in code but no business rule defines the mapping | Inconsistent prioritization across teams |
| **False Positive Management** | No suppression mechanism (`// lint-arwaky: ignore`), no allowlist per-file                      | Developer friction, alert fatigue        |
| **Versioning**                | No requirement for backward compatibility of AES codes or config schema                         | Breaking changes in CI pipelines         |

### 2.2 Missing Non-Functional Requirements

```
┌─────────────────────────────────────────────────────────────────┐
│  NFR Category        │  Status     │  Notes                     │
├─────────────────────────────────────────────────────────────────┤
│  Performance         │  ⚠️ Vague   │  "< 1 second" — no scale   │
│  Scalability         │  ❌ Missing  │  No max file count defined  │
│  Reliability         │  ❌ Missing  │  No crash recovery req      │
│  Configurability     │  ⚠️ Partial  │  YAML exists, undocumented  │
│  Observability       │  ❌ Missing  │  No logging/metrics req     │
│  Portability         │  ⚠️ Implicit │  Path normalization exists  │
│  Security            │  ❌ Missing  │  No path traversal guards   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Testability Assessment

### 3.1 Success Indicator Critique

| Indicator                 | Current Wording                                                                   | Testability Issue                                                   | Recommendation                                                                                            |
| ------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Dead code identification  | "100% detection of unused or unreachable architectural files"                     | **Untestable** — requires oracle of all orphans in any codebase     | Define a**reference corpus** (e.g., 50 seeded orphans across 6 layers) with expected detection rate ≥ 98% |
| Zero false warnings       | "valid components transitively reachable from entry points must never be flagged" | **Falsifiable but not measurable** — "never" across infinite inputs | Define a**regression suite** of 200+ known-valid files; assert 0 false positives per release              |
| Configuration flexibility | "correctly respects rule-specific exceptions and ignored path patterns"           | No acceptance criteria for "correctly"                              | Specify: given config X with ignore pattern Y, file Z matching Y must not appear in output                |
| Performance               | "less than a second even for larger multi-crate projects"                         | "Larger" is undefined                                               | Specify: ≤ 1s for workspace with ≤ 5,000 source files, ≤ 20 crates, cold cache                            |
| Workspace cleanliness     | "keeps the production binary lightweight"                                         | Not measurable from orphan-detector alone                           | Remove or reassign to build-system requirements                                                           |

### 3.2 Missing Test Scenarios

- Circular import detection (A imports B, B imports A — neither is orphan)
- Re-export chains (`mod.rs` → `lib.rs` → `main.rs`)
- Conditional compilation (`#[cfg(feature = "x")]`)
- Empty files / files with only comments
- Files with non-ASCII paths
- Workspace with 0 entry points (all files orphan?)
- Cross-crate imports via `shared::` prefix

---

## 4. Conflicting Requirements

### 4.1 Architecture vs. FRD Conflict (Critical)

> **FRD AES503:** "Capability files must be instantiated or imported by orchestrators **or other capability files**."
>
> **ARCHITECTURE.md §8:** "Capabilities must **never** import or call other Capabilities directly. They are standalone execution units."

**Impact:** The FRD explicitly permits a dependency pattern that the architecture **forbids**. This creates:

- Confusion for developers writing capabilities
- Ambiguity for the orphan detector: should a capability imported only by another capability be flagged or not?
- The implementation (`CapabilitiesOrphanAnalyzer`) does **not** check inter-capability imports, aligning with architecture but **violating the FRD**.

**Recommendation:** Amend AES503 to: _"Capability files must be instantiated or imported by orchestrators or wired in root containers."_

### 4.2 Agent Layer Dependency Conflict

> **ARCHITECTURE.md §9:** "Agent may depend only on Taxonomy and Contract."
>
> **Implementation (`agent_orphan_orchestrator.rs`):** Imports `shared::orphan_detector::utility_orphan_io`, `shared::orphan_detector::utility_orphan_filename`, and other utility modules.

**Impact:** The orchestrator (Agent layer) directly calls Utility functions, violating the stated dependency rule. This is either:

- An architecture documentation error (Agent _can_ use Utility), or
- An implementation violation requiring refactoring

**Recommendation:** Clarify whether the Agent layer may call Utility for I/O bridging, or refactor utility calls into the Capabilities layer.

### 4.3 Severity Inconsistency

| Orphan Type           | Severity in Code | Expected Business Priority                               |
| --------------------- | ---------------- | -------------------------------------------------------- |
| Taxonomy (AES501)     | LOW              | Low (domain model, no runtime impact) ✅                 |
| Contract (AES502)     | LOW              | **Should be MEDIUM** — dead contracts mislead architects |
| Capabilities (AES503) | MEDIUM           | Medium ✅                                                |
| Utility (AES504)      | HIGH             | **Should be MEDIUM** — utility is support code           |
| Agent (AES505)        | HIGH             | High ✅                                                  |
| Surface (AES506)      | HIGH             | High ✅                                                  |

No documented rationale exists for severity assignments.

---

## 5. Traceability Assessment

### 5.1 Current Traceability Matrix (Incomplete)

```
FRD Req ──→ AES Code ──→ Protocol Trait ──→ Analyzer Impl ──→ Test Case
  AES501      ✅           ✅                 ✅                ❌
  AES502      ✅           ✅                 ✅                ❌
  AES503      ✅           ✅                 ✅                ❌
  AES504      ✅           ✅                 ✅                ❌
  AES505      ✅           ✅                 ✅                ❌
  AES506      ✅           ✅                 ✅                ❌
  Config      ⚠️           ❌                 ⚠️                ❌
  Perf        ⚠️           ❌                 ❌                ❌
```

### 5.2 Missing Traceability Links

- **No requirement-to-test mapping** — no test file references AES codes
- **No config-to-behavior mapping** — YAML config fields are undocumented
- **No architecture-rule-to-enforcement mapping** — which AES code enforces which architecture rule?
- **No change history** — no changelog linking version bumps to requirement changes

---

## 6. Business Value Alignment

### 6.1 Value Proposition Gaps

The FRD states the goal is _"preventing codebase bloat and keeping the system maintainable"_ but lacks:

| Missing Element              | Why It Matters                                                                                        |
| ---------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Cost of inaction**         | No data on how much dead code costs (build time, cognitive load, onboarding)                          |
| **Prioritization rationale** | Why are Agent/Surface orphans HIGH but Taxonomy LOW? No business impact analysis                      |
| **Adoption metrics**         | No definition of success post-deployment (e.g., "reduce orphan count by 80% in 6 months")             |
| **User personas**            | Who consumes this output? CI pipeline? Developer IDE? Tech lead dashboard?                            |
| **Integration points**       | No requirement for how results flow into developer workflow (PR comments, dashboards, blocking gates) |

### 6.2 Stakeholder Communication Gaps

- **Developers** need: clear fix guidance (partially present in `AesOrphanViolation` Display impl ✅), suppression mechanism (❌)
- **Tech Leads** need: trend reporting, severity-based filtering, exception management (❌)
- **CI/CD Engineers** need: exit code semantics, output format contract, performance budget (❌)
- **Product Owners** need: coverage metrics, false positive rate, ROI tracking (❌)

---

## 7. Recommendations

### Priority 1 — Critical (Block Release Confidence)

| #   | Recommendation                                                                                                                                       | Effort | Impact                                                     |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------------------------------------------- |
| R1  | **Resolve AES503 conflict** — Remove "or other capability files" from FRD to align with architecture's no-inter-capability-dependency rule           | Low    | Eliminates architectural contradiction                     |
| R2  | **Define "reachable" formally** — Add glossary: "A file F is reachable iff there exists a path in the import graph from any entry point to F"        | Low    | Eliminates interpretation variance                         |
| R3  | **Document all entry point patterns** — List the 15 patterns from `get_orphan_entry_points()` in the FRD as the authoritative entry point definition | Low    | Stakeholders can validate coverage                         |
| R4  | **Create a reference test corpus** — 50+ files with known orphan/alive status per layer, checked into the repo                                       | Medium | Enables regression testing and "100% detection" validation |

### Priority 2 — High (Improve Testability & Ops)

| #   | Recommendation                                                                                               | Effort | Impact                          |
| --- | ------------------------------------------------------------------------------------------------------------ | ------ | ------------------------------- |
| R5  | **Specify performance benchmarks** — "≤ 1s for ≤ 5,000 files, ≤ 20 crates, cold cache, single-threaded"      | Low    | Testable NFR                    |
| R6  | **Document configuration schema** — Full YAML reference with field descriptions, defaults, and examples      | Medium | Self-service configuration      |
| R7  | **Add suppression mechanism requirement** — `// lint-arwaky:ignore AES503` inline or `.arwaky-ignore` file   | Medium | Reduces false positive friction |
| R8  | **Define output contract** — JSON schema for `LintResult` array, exit codes (0=clean, 1=violations, 2=error) | Medium | CI/CD integration               |
| R9  | **Document severity rationale** — Business impact analysis justifying HIGH/MEDIUM/LOW per AES code           | Low    | Stakeholder alignment           |

### Priority 3 — Medium (Long-term Quality)

| #   | Recommendation                                                                                            | Effort | Impact                    |
| --- | --------------------------------------------------------------------------------------------------------- | ------ | ------------------------- |
| R10 | **Add traceability matrix** — Living document mapping FRD → AES → Protocol → Impl → Test                  | Medium | Audit readiness           |
| R11 | **Define incremental analysis requirement** — Behavior on single-file change vs. full scan                | Medium | CI performance            |
| R12 | **Add observability requirements** — Structured logging, metrics (files scanned, orphans found, duration) | Medium | Production debugging      |
| R13 | **Clarify Agent-Utility dependency rule** — Amend ARCHITECTURE.md §9 or refactor orchestrator             | Low    | Architectural consistency |
| R14 | **Add multi-language scope statement** — Explicitly list supported languages and extension criteria       | Low    | Scope management          |
| R15 | **Version the config schema** — Add `schema_version` field to YAML, define migration policy               | Low    | Backward compatibility    |

---

## 8. Summary Scorecard

| Dimension              | Score      | Notes                                                                                  |
| ---------------------- | ---------- | -------------------------------------------------------------------------------------- |
| **Clarity**            | 5/10       | Requirements exist but diverge from implementation; key terms undefined                |
| **Completeness**       | 4/10       | Functional requirements present; NFRs, error handling, config largely missing          |
| **Testability**        | 3/10       | Success indicators are aspirational, not measurable; no test corpus                    |
| **Consistency**        | 5/10       | One critical conflict (AES503 vs. Architecture); severity undocumented                 |
| **Traceability**       | 4/10       | AES codes provide partial tracing; no test links, no config mapping                    |
| **Business Alignment** | 4/10       | Technical goal clear; business metrics, personas, and integration undefined            |
| **Overall Maturity**   | **4.2/10** | Implementation is ahead of documentation; requirements need a structured revision pass |

---

## 9. Proposed Next Steps

```
Week 1:  R1 + R2 + R3 + R9  (Quick wins — resolve conflicts, define terms)
Week 2:  R4 + R5 + R8       (Testability foundation)
Week 3:  R6 + R7            (Developer experience)
Week 4:  R10 + R11 + R12    (Operational maturity)
Ongoing: R13–R15            (Architectural hygiene)
```

---

_Prepared by: Expert Business Analyst — Requirements Engineering & Process Optimization_
_Document under review: `orphan-detector` v1.10.106 — FRD, ARCHITECTURE.md, and source implementation_
_Date: July 21, 2026_

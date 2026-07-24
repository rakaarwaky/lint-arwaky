# Review Report: crates (workspace) — Business Analyst

## Summary

The lint-arwaky workspace has **strong structural FRD coverage** (17 crates × FRD with FR IDs, edge cases, NFR, glossary, and root PRD links), but **requirements health is uneven**: product-level status is stale (root `PRD.md` still lists all P0–P2 items unchecked), several **cross-crate contradictions** undermine stakeholder trust (auto-fix “removal-only” vs rename/replace; MCP promises vs stubbed actions; PRD “5 tools” vs FRD “4 tools”), and **traceability from FR → acceptance test is broken or incomplete** for most crates. Highest business risk is shipping dual surfaces (CLI vs MCP) with different functional fidelity, plus untestable error contracts that collapse all failures to `false`/`0`. Fixing ID consistency, acceptance gaps, and product-status alignment will improve stakeholder satisfaction more than writing new FR text.

**Scope reviewed:** all feature crates under `crates/` (17 FRDs + root `PRD.md` + acceptance test inventory).  
**Rules applied:** `.agents/rules/RULES_AES.md`, FRD rules from `.agents/skills/add-docs-rust/references/frd-rules.md`.

---

## Findings by Category

### Requirements Clarity & Completeness

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1 | 🔴 CRITICAL | Overview states *“Only removal operations are automated — no code is added or modified”*, but FR-002 **replaces** `unwrap()` with `expect("safe")` and FR-003 **renames** symbols (`renamed_` prefix). Stakeholders cannot tell the safety boundary of auto-fix. | `crates/auto-fix/FRD.md` overview vs FR-002/FR-003 | Rewrite overview to classify operations: **remove**, **replace**, **rename**. Explicitly list non-goals (no structural refactor, no semantic rename, no multi-file rename). Align NFR “Accuracy” with that taxonomy. |
| 2 | 🔴 CRITICAL | MCP `execute` action set claims broad CLI parity, but several actions are **placeholders** (`fix` “returns placeholder success”; hooks “returns success messages”; orphan/security/dependencies return thin JSON). Conflicts with PRD Goal 3 and P0 “MCP server with tools”. | `crates/mcp-server/FRD.md` FR-001; `PRD.md` Goals/P0 | Publish an explicit **MCP capability matrix**: Action × Status (`full` / `partial` / `stub`). Either implement full parity with CLI or demote stubs to “not supported” with clear error codes. |
| 3 | 🔴 CRITICAL | Root PRD claims **“MCP server with 5 tools”**; MCP FRD System Overview and FR-005 register **four** tools (execute, list, read skill, health). | `PRD.md` Goal 3 / P0; `crates/mcp-server/FRD.md` | Reconcile to a single number. If fifth tool was planned (e.g. `watch` / `config`), either specify FR-00x or update PRD to 4. |
| 4 | 🟡 WARNING | `FR-013` appears only in the QA checklist (“git-diff analyzes only changed files”) but has **no Functional Requirements section**. FR-012 already covers Git Diff Command — ID gap / possible merge error. | `crates/cli-commands/FRD.md` | Either promote FR-013 to a real FR (if distinct from FR-012) or renumber checklist item to FR-012 and drop FR-013. Keep sequential IDs without holes. |
| 5 | 🟡 WARNING | Dual FR IDs `FR-004a` / `FR-004b` break the “unique sequential FR-NNN” convention. Both FR-001 and FR-004b emit **AES201**, so business distinction is unclear (layer hierarchy vs config-forbidden patterns). | `crates/import-rules/FRD.md` | Renumber to FR-004 (dummy/AES204) and FR-005 (forbidden/AES201 config), shift cycle to FR-006. Clarify AES201 as one rule with two detection paths, or split error codes if product needs separate reporting. |
| 6 | 🟡 WARNING | `AES000` file-read diagnostics are specified in code-analysis FRD but **absent** from `RULES_AES.md` and PRD’s “24 AES rules” catalog (AES101–506). | `crates/code-analysis/FRD.md` FR-006; `.agents/rules/RULES_AES.md`; `PRD.md` | Add AES000 to the official catalog (severity, group, fix guidance) or reclassify as non-AES internal diagnostic so counts stay accurate. |
| 7 | 🟡 WARNING | Root PRD feature checkboxes remain all `[ ]` despite substantial implementation and FRD maturity. Product status is not visible to stakeholders. | `PRD.md` Feature Requirements | Mark delivered P0/P1 items complete (or partial with notes). Link each checkbox to crate FRD(s). Maintain a single product-status source of truth. |
| 8 | 🟡 WARNING | Several FRDs embed **implementation-shaped contracts** (trait object types, container factories, method tables) — violates timeless FRD rule (“no class/function/file names”). | e.g. `auto-fix` API Contract; `cli-commands` FR-012 inputs (`Arc<dyn …>`); `mcp-server` overview | Replace with functional operations and domain VOs only. Move DI/wiring detail to ARCHITECTURE or design notes. |
| 9 | 🟡 WARNING | import-rules FR-001 business rules are **incomplete vs AES201 table** (12 sub-conditions in RULES_AES). FRD lists only a subset of layer constraints. | `crates/import-rules/FRD.md` FR-001; `RULES_AES.md` AES201 | Mirror full allowed/mandatory/forbidden matrix per layer (or reference RULES_AES as normative and mark FRD as summary with “see RULES_AES”). |
| 10 | 🟢 INFO | Root PRD is thin (personas, goals) while 17 FRDs carry real detail. No crate-level PRDs exist (acceptable if root PRD decomposes features). | workspace | Add a **Feature Map** section to root PRD: Feature → crate → primary FR IDs → priority. Avoid per-crate PRDs unless product ownership splits. |
| 11 | 🟢 INFO | Glossary term “AES” is inconsistently expanded (Architecture Enforcement Specification vs Arwaky Engineering Standards vs coding standard). | multiple FRDs / PRD | Standardize one expansion in root PRD glossary; FRDs reference it. |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 12 | 🔴 CRITICAL | **shared** has 7 FRs and **zero acceptance tests** (only utility unit-style tests). Foundation VOs/contracts lack business-level pass/fail criteria. | `crates/shared/FRD.md`; `crates/shared/tests/` | Add acceptance tests for VO invariants (empty path, score threshold, severity impact) and contract trait surface stability. Map to FR-001–FR-007. |
| 13 | 🔴 CRITICAL | auto-fix collapses I/O failure, out-of-bounds, and “not an import line” into **`false`/`0`**. QA cannot write distinct acceptance criteria for error *class*. | `crates/auto-fix/FRD.md` FR-001–003 | Introduce typed outcomes: `Applied` / `Skipped(reason)` / `Failed(reason)` (domain VO). Acceptance must assert reason codes, not only bool. |
| 14 | 🟡 WARNING | Acceptance coverage ratio is low for several high-value crates: | see Gap Analysis | Prioritize acceptance for CLI exit-code matrix, MCP stubs, maintenance doctor/security, project-setup, report formats, TUI critical paths. |
|  |  | crate | FRs | acceptance files |
|  |  | maintenance | 7 | 2 |
|  |  | project-setup | 7 | 1 |
|  |  | report-formatter | 7 | 1 |
|  |  | tui | 12 | 1 |
|  |  | cli-commands | 14 | 7 |
|  |  | external-lint | 7 | 3 |
|  |  | orphan-detector | 11 | 6 (AES only) |
| 15 | 🟡 WARNING | **ID scheme mismatch** between FRD and acceptance files: config-system uses `US-1..5` vs FR-001..010; orphan/role use `AES###` not `FR-###`; auto-fix uses `FRD-UNUSED` style; many acceptance files have **no FR comment**. | multiple `tests/acceptance_*` | Standardize: filename `acceptance_FR_00N.rs` + module doc `//! FR-00N`. config-system: rename US→FR or add FR mapping table in FRD. |
| 16 | 🟡 WARNING | FRD QA checklists are unchecked markdown (`- [ ]`) and often not 1:1 with automated acceptance tests — dual sources of truth. | all FRDs “Test Scenarios” | Either auto-generate checklist status from test names or mark checklist items with test file references (`covered by tests/acceptance_FR_001.rs`). |
| 17 | 🟡 WARNING | import-rules NFR claims **“Zero false positives for valid imports”** without measurable definition or golden corpus. | `crates/import-rules/FRD.md` NFR | Define golden fixtures (valid AES layouts) and acceptance that zero AES201–205 fire. Soften NFR to “≤0 FPs on golden suite” if absolute zero is unrealistic. |
| 18 | 🟢 INFO | code-analysis and naming-rules show **best practice**: acceptance files named per FR (`acceptance_FR_001.rs` …). Reuse as template workspace-wide. | `crates/code-analysis/tests/`, `naming-rules/tests/` | Document this pattern in create-test-rust skill / TEST.md. |

### Scope & Dependencies

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 19 | 🔴 CRITICAL | **CLI vs MCP functional gap**: CLI FRD specifies full fix/doctor/security/hooks behavior; MCP FR-001 stubs several of the same actions. AI-agent persona (primary PRD persona) gets a weaker product. | `cli-commands` + `mcp-server` FRDs | Product decision: (A) MCP is thin façade over same aggregates as CLI (parity required), or (B) MCP is intentionally limited (document subset). Update PRD personas accordingly. |
| 20 | 🟡 WARNING | Exit-code semantics are **crate-local and inconsistent** (e.g. security uses exit 3 for missing tool; most commands use 0/1/2 only). CI/DevOps persona needs a single contract. | `cli-commands` FR-001–015; maintenance FR-006 | Publish workspace **Exit Code Contract** in PRD or shared FRD: `0` ok, `1` policy fail, `2` runtime error, `3` prerequisite missing — apply everywhere. |
| 21 | 🟡 WARNING | TUI is **P2 Nice-to-Have** in PRD but has a full 12-FR FRD and implementation investment; acceptance is thin (1 file). Prioritization vs effort misaligned. | `PRD.md` P2; `crates/tui/FRD.md` | Either promote TUI to P1 if it is a supported surface, or freeze FR growth and mark FRD as “aspirational / best-effort”. |
| 22 | 🟡 WARNING | auto-fix depends on “analysis aggregate” for lint input; dry-run construction flag “cannot be changed per invocation” may conflict with CLI per-command `--dry-run`. | `auto-fix` Assumptions; `cli-commands` FR-003 | Specify whether dry-run is **process lifetime** or **per request**. Align container factory contract with CLI flag. |
| 23 | 🟢 INFO | Cross-crate integration points are listed but lack a single **end-to-end value stream** diagram (scan → format → CI gate → fix → re-scan). | workspace | Add one sequence diagram in root PRD or ARCHITECTURE for primary happy path (Developer + CI + AI Agent). |

### Traceability (FRD ↔ Code)

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 24 | 🔴 CRITICAL | Product requirements (PRD checkboxes) are **not traced** to delivered FRDs or crates — all remain open. Release readiness cannot be asserted. | `PRD.md` | Build RTM (Requirements Traceability Matrix): PRD item → crate → FR-IDs → acceptance tests → status. |
| 25 | 🟡 WARNING | orphan-detector acceptance covers AES501–506 only; FR-001–003 (graph, entry, reachability), FR-010 (barrel), FR-011 (config) lack acceptance-level traceability. | `crates/orphan-detector/` | Add acceptance_FR_001..003, _010, _011 or map AES tests to FR IDs in headers. |
| 26 | 🟡 WARNING | role-rules FR-001 (file collection) and FR-008 (config toggle) have no dedicated AES acceptance (AES401–406 only). | `crates/role-rules/` | Add acceptance for ignore/toggle and multi-language file collection. |
| 27 | 🟡 WARNING | maintenance acceptance only `audit` + `dep_update`; FR-001 doctor, FR-002 stats, FR-003 clean, FR-004 update, FR-005 diagnose poorly mapped by name. | `crates/maintenance/` | Rename/extend acceptance files to FR-001…007; ensure doctor exit semantics match CLI FR-004. |
| 28 | 🟡 WARNING | report-formatter has one umbrella acceptance for FR-001–007 (text/json/sarif/junit). Format regressions can pass without per-format criteria. | `crates/report-formatter/` | Split acceptance per format FR or use parameterized cases labeled FR-001…004. |
| 29 | 🟢 INFO | Most FRDs correctly link `PRD: ../../PRD.md`. Structural link exists; content-level RTM does not. | all FRDs Reference | Keep link; add reverse links from PRD Feature Map. |

---

## Violations (if any)

These are **requirements/documentation** violations relative to project FRD rules and AES catalog — not runtime AES findings from a scan.

1. **FRD timeless rule** — Implementation identifiers in FRDs (`fix processor`, `Arc<dyn GitHooksAggregate>`, container factories, layer file prefixes as API surface). Per `frd-rules.md`: FRD must not contain implementation details that break on refactor.
2. **AES catalog drift** — `AES000` specified in code-analysis FRD but missing from `RULES_AES.md` and PRD’s 24-rule count narrative.
3. **AES201 dual-path ambiguity** — import-rules FR-001 + FR-004b both produce AES201 without clarifying diagnostic differentiation for users/CI filters.
4. **PRD/MCP count conflict** — 5 tools (PRD) vs 4 tools (mcp-server FRD).
5. **auto-fix safety narrative conflict** — removal-only overview vs replace/rename FRs (business policy violation in the document itself).
6. **Naming convention for requirements IDs** — `FR-004a/b`, `US-*`, `FRD_*`, `AES*` acceptance names reduce AES101-like consistency for *requirements* artifacts (process debt, not code AES101).

No AES layer-import code audit was executed as part of this BA review (out of BA scope); recommend a follow-up `lint-arwaky` self-scan for code-level AES201–506.

---

## Action Items

- [ ] **P0** Resolve auto-fix operation taxonomy (remove / replace / rename) and update overview + NFR + acceptance criteria.
- [ ] **P0** Publish MCP vs CLI capability matrix; eliminate silent stubs or document them as unsupported with errors.
- [ ] **P0** Reconcile MCP tool count (4 vs 5) in PRD and mcp-server FRD.
- [ ] **P0** Update root PRD checkboxes / Feature Map with delivery status and crate links (RTM v1).
- [ ] **P0** Define workspace Exit Code Contract and align cli-commands + maintenance + CI docs.
- [ ] **P1** Standardize FR IDs (fix import-rules 004a/b; add missing cli FR-013 or remove).
- [ ] **P1** Catalog AES000 officially or reclassify; keep “24 rules” claim accurate.
- [ ] **P1** Add shared crate acceptance tests for VO/contract FRs.
- [ ] **P1** Normalize acceptance file naming to `acceptance_FR_00N.rs` across crates; map US/AES names.
- [ ] **P1** Close acceptance gaps for maintenance, project-setup, report-formatter, tui, orphan FR-001–003/010/011.
- [ ] **P1** Replace bool error collapses in auto-fix with reason-coded outcomes for testability.
- [ ] **P2** Strip implementation names from FRD API contracts (timeless FRD pass).
- [ ] **P2** Expand import-rules FR-001 to full AES201 matrix or normative reference to RULES_AES.
- [ ] **P2** Decide TUI priority (P1 supported vs P2 best-effort) and adjust FRD/acceptance investment.
- [ ] **P2** Link FRD QA checklist items to concrete test paths; avoid dual unchecked lists.
- [ ] **P3** Unify AES glossary expansion across docs.

---

## Gap Analysis Table

| Current State | Issue | Recommendation | Priority |
| ------------- | ----- | -------------- | -------- |
| 17 crate FRDs with FR structure | Good skeleton; uneven depth and contradictions | Keep structure; fix contradictions before adding new FRs | P0 |
| Root PRD all checkboxes open | No product delivery signal | RTM + status update | P0 |
| MCP stubs vs full CLI | AI persona under-served vs PRD | Capability matrix + parity plan | P0 |
| auto-fix “removal only” vs rename/replace | Conflicting safety promise | Rewrite policy language | P0 |
| shared: 7 FR, 0 acceptance | Untested foundation promises | Add acceptance suite | P1 |
| acceptance ID chaos (US/AES/FRD/FR) | Broken FR↔test traceability | Standardize naming + headers | P1 |
| Low acceptance density (tui 1/12, setup 1/7, maint 2/7) | High regression risk on user-facing flows | Priority acceptance backlog | P1 |
| AES000 not in RULES_AES | Catalog / count inconsistency | Officialize or demote | P1 |
| Exit codes vary by command | CI integration friction | Single exit-code contract | P0 |
| FRD API tables use impl types | FRD churn on refactor | Timeless functional API only | P2 |
| TUI P2 but full FRD | Scope/priority mismatch | Promote or freeze | P2 |
| import-rules FR-004a/b + dual AES201 | Ambiguous requirements IDs | Renumber + clarify codes | P1 |
| QA checklists unchecked & unlinked | Dual source of truth | Link checklist ↔ tests | P2 |
| PRD “5 MCP tools” vs FRD “4” | Stakeholder confusion | Single authoritative count | P0 |
| NFR “zero false positives” (import-rules) | Untestable absolute | Golden suite SLAs | P2 |

---

## Coverage Snapshot (FR vs acceptance files)

| Crate | FR sections | Acceptance files | Traceability quality |
| ----- | ----------- | ---------------- | -------------------- |
| auto-fix | 5 | 4 | Medium (custom FRD-* names) |
| cli-commands | 14 (+ hole at 013) | 7 | Medium (feature-named, not FR-ID) |
| code-analysis | 6 | 6 | **High** |
| config-system | 10 | 5 | Low (US-* IDs) |
| external-lint | 7 | 3 | Low–Medium |
| file-watch | 6 | 4 | Medium |
| git-hooks | 6 | 4 | Medium (FRD-00N) |
| import-rules | 6 (004a/b) | 5 | Medium–High |
| maintenance | 7 | 2 | Low |
| mcp-server | 5 | 4 | Medium |
| naming-rules | 2 | 2 | **High** |
| orphan-detector | 11 | 6 | Medium (AES-only) |
| project-setup | 7 | 1 | Low |
| report-formatter | 7 | 1 | Low |
| role-rules | 8 | 6 | Medium (AES-only) |
| shared | 7 | 0 | **None** |
| tui | 12 | 1 | Low |

---

## Stakeholder Impact

| Persona | Pain if gaps remain | Business value if fixed |
| ------- | ------------------- | ----------------------- |
| **AI Agent** | MCP stubs report success without doing work → false confidence / bad agent loops | Reliable MCP parity with CLI actions |
| **Developer** | Unclear auto-fix safety; inconsistent CLI exits | Trust automated fix + predictable local workflow |
| **DevOps / CI** | Exit code 3 vs 2 ambiguity; incomplete format acceptance | Stable quality gates and SARIF/JUnit consumers |
| **Contributor** | FR ID chaos + impl-heavy FRDs | Faster onboarding; safer extensions |
| **Reviewer** | PRD still “all open”; AES000 not in catalog | Auditable architecture compliance narrative |

---

## Recommended Next Steps (sequence)

1. **Product alignment workshop (short):** MCP parity, auto-fix safety policy, exit codes, TUI priority.  
2. **Doc hotfix PR:** PRD status + tool count + auto-fix overview + FR-013/004a cleanup + AES000 decision.  
3. **Traceability PR:** RTM table + acceptance rename/map for shared, maintenance, project-setup, report-formatter.  
4. **Testability PR:** reason-coded auto-fix results; golden import-rules suite.  
5. **Optional AES self-scan** for code-level layer violations (separate from BA).

---

*Report generated as Business Analyst review of `crates/` requirements artifacts. No product code was modified.*

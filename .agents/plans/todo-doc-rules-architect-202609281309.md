# Plan: doc-rules (AES601–AES607, FR/Protocol Parity) — Software Architect

## Summary

This review covers the `doc-rules` feature crate after the landing of **AES607 — FR/Protocol Parity** (`feat(doc-rules): enforce FR/protocol class parity as AES607`, commit `779480d`, PR #335). The feature spans four system boundaries: the shared kernel contracts (`crates/shared/src/doc_rules/*`), the feature crate (`crates/doc-rules/*`), the dispatcher integration (`crates/dispatcher/src/surface_docs_action.rs`), and the governing documents (`FRD.md`, `BACKLOG.md`, `RULES_AES.md`). Cross-module impact is significant: AES607 makes doc-rules the first "Doc" group rule that reads **source code** (`.rs` files under `crates/shared/src`), which silently breaks the feature's own read-scope NFR, hard-couples the capability to the Rust workspace layout, and — most visibly — **fires on this repository itself**: the `doc-rules` FRD declares its requirements as a table (0 countable FR headings) against 2 `I*Protocol` traits (one of which, `IDocAuditProtocol`, is an orphan), and `structure-rules` sits at 0 vs 1. Non-functional requirements (read scope, finding precision, determinism) are partially violated by the new rule. The layering itself (taxonomy → contract → utility → capability → agent → root) is clean and correctly wired; the readiness gaps are contract drift, standard self-application, and the rule-catalog fork between `RULES_AES.md` (396 lines, no AES607) and `.agents/rules/RULES_AES.md` (614 lines, with AES607).

---

### Scope 1: Define system-wide module boundaries
<!-- Module ownership, dependency direction, abstraction layers, circular dependencies, responsibility segregation, service granularity, interface segregation -->

#### Issue ARCH-1-001-202609281309

**Title:** [ARCH][CRITICAL] Orphan contract trait `IDocAuditProtocol` — dead seam in the shared contract layer

**Label:** contract, severity-critical

**Location Path:**
`crates/shared/src/doc_rules/contract_doc_protocol.rs:22`

**Description:**
`contract_doc_protocol.rs` declares two traits: `IDocCheckerProtocol` (implemented by `DocChecker`, consumed by `DocOrchestrator`) and `IDocAuditProtocol`. A workspace-wide search shows `IDocAuditProtocol` is referenced exactly **once** — its own declaration. No capability implements it, no agent consumes it, no test exercises it. This is an interface-segregation failure (a published seam nobody uses, AES501-class orphan in the contract layer) **and** it directly inflates the AES607 protocol count for the doc-rules feature itself: `count_protocol_traits` counts it as a capability seam (`pub trait I…Protocol`, not `Aggregate`), so the feature reports 2 seams while only 1 exists.

**Acceptance Criteria:**
- `grep -rn IDocAuditProtocol crates/` returns zero hits.
- `count_protocol_traits(crates/shared/src/doc_rules)` returns 1.
- Orphan-rules scan (`AES501`) reports no dead contract in `shared/doc_rules`.

**Recommendation:** Delete the trait. The agent's execution seam is already published as `IDocRunnerAggregate`; a second, unimplemented "agent contract" trait duplicates that seam in the wrong role.

**Git Diff:**
```diff
--- a/crates/shared/src/doc_rules/contract_doc_protocol.rs
-/// Agent contract for doc-rules: accepts a request and returns findings.
-/// The agent orchestrates one or more capability protocols; this trait
-/// exposes the execution seam.
-pub trait IDocAuditProtocol: Send + Sync {
-    /// Execute the audit over the documents described in *request*.
-    fn audit(&self, request: DocRequest) -> DocResponse;
-}
```

**Open Questions:** None

#### Issue ARCH-1-002-202609281309

**Title:** [ARCH][WARNING] `DocChecker` capability owns file discovery — Utility-layer concern duplicated inside a capability

**Label:** layer, severity-warning

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs` (`collect_documents` ~L370, `check_feature_folder` L807–852, `has_orchestrator` L855+, `read_source` L906, `collect_feature_docs` L915)

**Description:**
ARCHITECTURE.md §7 assigns "File discovery — walk directories, detect files, apply ignore" to the **Utility** layer, and the Capabilities DRY rule requires shared technical mechanics to be extracted there. `DocChecker` performs its own directory walking (`fs::read_dir` in three separate private helpers) across `crates/`, `modules/`, and `packages/`, duplicating mechanics that already exist in the `filesystem` feature and in dispatcher-side collectors. The capability mixes the *decision* concern (what a finding means) with the *discovery* concern (which files exist), which bloats it to 979 lines and makes the walk untestable in isolation.

**Acceptance Criteria:**
- All `fs::read_dir`/`fs::read_to_string` walking moves to `utility_doc_walker.rs` (or an existing filesystem utility) as stateless functions.
- `capabilities_doc_checker.rs` contains no direct `fs::read_dir` call.
- Existing `contract_doc_rules.rs` tests pass unchanged.

**Recommendation:** Extract `collect_documents`, `collect_feature_docs`, `has_orchestrator`, and `read_source` into `crates/doc-rules/src/utility_doc_walker.rs` (utility layer, taxonomy-only imports), leaving `DocChecker` with pure judgment logic — mirroring what was already done correctly for `utility_protocol_counter.rs`.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/capabilities_doc_checker.rs
-    pub fn collect_documents(&self, root: &Path) -> Vec<DocSource> {
-        ... fs::read_dir walking ...
-    }
+use crate::utility_doc_walker::{collect_documents, has_orchestrator};
--- /dev/null
+++ b/crates/doc-rules/src/utility_doc_walker.rs
+// PURPOSE: stateless workspace walking for the doc chain (discovery only,
+// no judgment) — file discovery is a Utility concern per ARCHITECTURE.md §7.
+pub fn collect_documents(root: &Path) -> Vec<DocSource> { ... }
```

**Open Questions:** Should discovery reuse `crates/filesystem` (cross-crate utility) instead of a local utility module, given `filesystem` already owns ignore handling?

#### Issue ARCH-1-003-202609281309

**Title:** [ARCH][WARNING] AES607 hardcodes `crates/shared/src` — capability coupled to one workspace layout it does not own

**Label:** dependency, severity-warning

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs:368` (`check_fr_protocol_parity`)

**Description:**
`check_fr_protocol_parity` resolves the feature's contract module as `root.join("crates/shared/src").join(&module)`. This bakes three assumptions into a capability: (a) the kernel lives in `crates/` (yet the very same file iterates `["crates", "modules", "packages"]` everywhere else), (b) the kernel folder is named `shared` with an `src/` child, and (c) the module name is the feature name with `-`→`_`. Any Python (`modules/shared/`) or TypeScript (`packages/shared/`) workspace is silently exempted from AES607 (`count_protocol_traits` returns `None` → rule skipped), and any Rust workspace with a different kernel path gets false negatives. This is an uncontrolled cross-module dependency on another member's internal layout.

**Acceptance Criteria:**
- The shared-module path is derived from the same member roots (`crates|modules|packages`) used by `collect_documents`/`check_feature_folder`, or from a `taxonomy_doc_constant` / `lint_arwaky.config.yaml` entry.
- A test with an FRD under `modules/<feature>/` and protocols under `modules/shared/` produces an AES607 finding.

**Recommendation:** Introduce `consts::KERNEL_SRC_SEGMENTS` (or a config key) and resolve the module against the member root the FRD was found under, not a literal Rust path.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/capabilities_doc_checker.rs
-        let Some(protocol_count) =
-            count_protocol_traits(&root.join("crates/shared/src").join(&module))
+        // Resolve the kernel next to the member root the FRD lives under.
+        let member_root = doc.path.ancestors().nth(2).unwrap_or(root);
+        let Some(protocol_count) =
+            count_protocol_traits(&kernel_module_dir(member_root, &module))
```

**Open Questions:** Is a config override (`lint_arwaky.config.yaml`) desired for non-standard kernel locations, or is convention-only acceptable?

---

### Scope 2: Design cross-feature integration patterns
<!-- Sync/async boundaries, event-driven patterns, message contracts, data consistency across modules, pipeline integrity, backpressure -->

#### Issue ARCH-2-001-202609281309

**Title:** [ARCH][CRITICAL] AES607 breaks the doc-rules read-scope contract — the FRD's NFR still promises ".md files only"

**Label:** contract, severity-critical

**Location Path:**
`crates/doc-rules/FRD.md` (Non-functional Requirements table) ↔ `crates/doc-rules/src/utility_protocol_counter.rs:count_protocol_traits`

**Description:**
The doc-rules FRD's NFR table states: *"Read scope — Doc rules read no file beyond the documents under audit — Run a scan and confirm only `.md` files are opened by this group."* AES607's `count_protocol_traits` now opens and reads every `.rs` file in `crates/shared/src/<module>/` during a docs audit. The implementation and its governing contract have diverged inside the same PR that added the rule: the published integration contract (what the docs pipeline touches, which matters for sandboxing, caching, and CI cost accounting downstream in `dispatcher`) is no longer true. Consumers that budgeted the docs group as "markdown-only" (watch mode, incremental scans) now have an invisible dependency on source-tree state — an FRD edit *or* a contract-trait edit can flip AES607, but only `.md` changes are treated as doc-audit triggers.

**Acceptance Criteria:**
- FRD NFR table updated to state the true read scope (documents under audit **plus** the feature's shared contract module), or AES607's source scan is moved behind an index provided by the caller.
- `file-watch`/incremental paths re-trigger the docs group when `crates/shared/src/*/contract_*` changes.
- FRD gains a requirement row covering parity checking (see ARCH-4-001: this also restores the feature's own FR/protocol parity).

**Recommendation:** Update the NFR row and add `FR-DOC-004` for the parity invariant. Longer term, have the dispatcher pass a pre-built protocol-class index (it already indexes source files for the code-rule groups) so doc-rules stays markdown-only.

**Git Diff:**
```diff
--- a/crates/doc-rules/FRD.md
-| Read scope | Doc rules read no file beyond the documents under audit | Run a scan and confirm only `.md` files are opened by this group |
+| Read scope | Doc rules read the documents under audit and, for parity checks, the feature's shared contract module | Run a scan and confirm only `.md` files and `shared/<feature>/contract_*` sources are opened |
```

**Open Questions:** Should AES607 consume the dispatcher's existing file index (keeping doc-rules markdown-pure) instead of widening the contract?

#### Issue ARCH-2-002-202609281309

**Title:** [ARCH][WARNING] Three competing FR-heading grammars — AES601, AES607, and table-style FRDs disagree on what a requirement is

**Label:** contract, severity-warning

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs:115` (`fr_id_wellformed_re`: `FR-[A-Z0-9]+-\d+`) ↔ `crates/doc-rules/src/utility_protocol_counter.rs:count_fr_headings` (`FR-[A-Za-z0-9_]+-\d+`) ↔ `crates/doc-rules/FRD.md` / `crates/structure-rules/FRD.md` (table rows, no headings)

**Description:**
The "requirement" message contract is defined three different ways: AES601's well-formed matcher accepts only UPPERCASE feature segments; AES607's counter accepts mixed case and underscores (its doc-comment even blesses CamelCase `FR-AutoFix-001`, which AES601 would *not* accept as well-formed); and the two newest FRDs (doc-rules, structure-rules) declare requirements as `| FR-DOC-001 | … |` table rows, which **neither** regex sees. Consequences: a CamelCase FR heading is counted by AES607 but skips AES601's six-field check; a table-style FRD bypasses AES601 entirely and is counted as **0** requirements by AES607. One domain concept, three incompatible grammars, drifting independently in two files.

**Acceptance Criteria:**
- A single FR-heading pattern constant lives in `crates/shared/src/doc_rules/taxonomy_doc_constant.rs` and is consumed by both AES601 matchers and `count_fr_headings`.
- A canonical FRD requirement shape (headings vs table) is decided and documented in `HOW-TO-MAKE-FRD.md`; the non-canonical shape produces an AES601/AES602 finding.
- A test proves the same document yields identical requirement counts to AES601 and AES607.

**Recommendation:** Promote the pattern to a taxonomy constant (`FR_HEADING_PATTERN`) with one casing policy, and make AES601 flag table-style Requirements sections so the count-bypass is impossible.

**Git Diff:**
```diff
--- a/crates/shared/src/doc_rules/taxonomy_doc_constant.rs
+/// The single grammar for a requirement heading, shared by AES601 and AES607.
+pub const FR_HEADING_PATTERN: &str = r"(?m)^#{2,4}\s+FR-[A-Z0-9]+-\d+:";
--- a/crates/doc-rules/src/utility_protocol_counter.rs
-    let re = regex::Regex::new(r"(?m)^#{2,4}\s+FR-[A-Za-z0-9_]+-\d+:").ok()?;
+    let re = regex::Regex::new(consts::FR_HEADING_PATTERN).ok()?;
```

**Open Questions:** Which casing is canonical — `FR-AUTOFIX-001` (AES601 today) or `FR-AutoFix-001` (AES607's comment)? The repo's own FRDs use uppercase.

#### Issue ARCH-2-003-202609281309

**Title:** [ARCH][WARNING] AES607 error and precision semantics: silent skips, silent undercounts, and no line number in the finding

**Label:** contract, severity-warning

**Location Path:**
`crates/doc-rules/src/utility_protocol_counter.rs:count_protocol_traits` and `crates/doc-rules/src/capabilities_doc_checker.rs:check_fr_protocol_parity` (finding emission, ~L385)

**Description:**
Three quiet failure paths weaken the rule's message contract: (1) if the shared module directory can't be read, `count_protocol_traits` returns `None` and the rule is **silently skipped** — a permissions error looks identical to compliance; (2) if one `.rs` file inside the module fails to read, the loop `continue`s, silently **undercounting** and potentially emitting a *false* mismatch finding whose numbers depend on transient I/O state — this also threatens the FRD's Determinism NFR ("two runs report the same findings"); (3) the emitted `DocFinding` carries no line reference at all (other doc findings embed `line N` in the message), while the FRD's Finding-precision NFR demands "every finding names a file and a line number", and FR-DOC-003 promises line-level detail.

**Acceptance Criteria:**
- An unreadable module or file yields an explicit INFO-grade finding (e.g. `parity_unverifiable`) instead of a skip/undercount.
- The AES607 mismatch message anchors to the `## Requirements` heading line of the FRD.
- Two consecutive runs over a tree with one unreadable contract file produce identical findings.

**Recommendation:** Return `Result<usize, PathBuf>`-style outcomes from the counters, map read failures to a dedicated violation type, and locate the Requirements section line for the finding message.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/utility_protocol_counter.rs
-        let Ok(text) = fs::read_to_string(&path) else {
-            continue;
-        };
+        let Ok(text) = fs::read_to_string(&path) else {
+            return None; // unverifiable beats a fabricated count
+        };
--- a/crates/doc-rules/src/capabilities_doc_checker.rs
+            // Anchor the finding at the Requirements heading.
+            format!("line {req_line} FRD declares {fr_count} requirements but ...")
```

**Open Questions:** Should "unverifiable" be a reportable violation type or a logged diagnostic? Today the pipeline has no diagnostic channel besides findings.

---

### Scope 3: Architect system scalability
<!-- Stateless design, bottlenecks, resource contention, horizontal scaling readiness, reliability targets, capacity, performance baseline -->

#### Issue ARCH-3-001-202609281309

**Title:** [ARCH][WARNING] AES607 re-reads the shared contract module from disk for every FRD audited — O(features × contract files) repeated I/O

**Label:** design, severity-warning

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs:check_fr_protocol_parity` → `utility_protocol_counter.rs:count_protocol_traits`

**Description:**
`check_fr_protocol_parity` runs once per FRD, and each invocation calls `count_protocol_traits`, which does a fresh `fs::read_dir` + full `fs::read_to_string` of every `.rs` file in the feature's shared module. With 16 feature FRDs today this is 16 directory walks and dozens of full-file reads per docs run — and it grows linearly with feature count on every `docs`, `check`, and watch-mode cycle. Nothing is memoized even though the audit is single-request and the data is immutable within a run. This is the first emerging bottleneck in a group whose baseline was "read a fixed set of markdown files".

**Acceptance Criteria:**
- Contract-module scans are performed at most once per module per `audit()` call (memoized map or pre-pass).
- A benchmark in `crates/doc-rules/benches/` (currently absent — every other feature crate has one) records the docs-audit baseline before and after.

**Recommendation:** Build a `BTreeMap<String, usize>` of module → protocol count in one pre-pass inside `audit()`, pass it to `check_fr_protocol_parity`; add `benches/bench_doc_rules.rs` to lock the baseline.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/capabilities_doc_checker.rs
-        let Some(protocol_count) =
-            count_protocol_traits(&root.join("crates/shared/src").join(&module))
+        // Counted once per audit in a pre-pass; O(modules), not O(FRDs × files).
+        let Some(protocol_count) = protocol_counts.get(&module).copied()
```

**Open Questions:** None

#### Issue ARCH-3-002-202609281309

**Title:** [ARCH][INFO] `count_fr_headings` recompiles its regex on every call — breaks the crate's own `OnceLock` caching pattern

**Label:** design, severity-info

**Location Path:**
`crates/doc-rules/src/utility_protocol_counter.rs:count_fr_headings`

**Description:**
`capabilities_doc_checker.rs` caches all seven of its regexes in `static OnceLock`s; the new utility compiles `regex::Regex::new(...)` inside `count_fr_headings` on every invocation. Regex compilation is orders of magnitude more expensive than matching, and this function runs once per FRD per audit (and per keystroke in watch mode). Minor today, but it silently abandons the established performance convention of the very crate it lives in.

**Acceptance Criteria:**
- The FR-heading regex is compiled once per process (`OnceLock`), matching the checker's existing pattern.
- No behavioral change in counts.

**Recommendation:** Hoist into a `OnceLock` (and, per ARCH-2-002, source the pattern from the taxonomy constant).

**Git Diff:**
```diff
--- a/crates/doc-rules/src/utility_protocol_counter.rs
-pub fn count_fr_headings(text: &str) -> Option<usize> {
-    let re = regex::Regex::new(r"(?m)^#{2,4}\s+FR-[A-Za-z0-9_]+-\d+:").ok()?;
-    Some(re.find_iter(text).count())
-}
+static FR_RE: OnceLock<Option<regex::Regex>> = OnceLock::new();
+pub fn count_fr_headings(text: &str) -> Option<usize> {
+    let re = FR_RE
+        .get_or_init(|| regex::Regex::new(consts::FR_HEADING_PATTERN).ok())
+        .as_ref()?;
+    Some(re.find_iter(text).count())
+}
```

**Open Questions:** None

#### Issue ARCH-3-003-202609281309

**Title:** [ARCH][INFO] Docs audit is fully sequential with whole-file loads — parallelization headroom unclaimed, no performance baseline recorded

**Label:** design, severity-info

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs:audit` (per-document loop) and missing `crates/doc-rules/benches/`

**Description:**
`audit()` walks documents, loads each fully into memory, and audits them one by one; findings are then deduped and sorted (`sorted()`), which correctly guarantees deterministic output. Because `DocChecker` is a zero-state struct and each `audit_document` is independent, the per-document loop is embarrassingly parallel — but no baseline exists to justify (or reject) parallelizing: doc-rules is the only rule crate in the workspace **without** a `benches/` directory. Horizontal-scaling readiness is fine (stateless capability behind `Arc<dyn>`); what's missing is the measurement that capacity planning depends on.

**Acceptance Criteria:**
- `crates/doc-rules/benches/bench_doc_rules.rs` exists and measures `audit()` over a representative tree (this repo).
- A documented decision (in the bench or FRD NFR) on whether per-document parallelism is worth it at the measured baseline.

**Recommendation:** Add the missing bench first; only introduce `rayon`-style parallelism if the baseline shows the docs group materially contributing to `check` latency. Determinism is already protected by the final sort.

**Git Diff:**
```diff
--- /dev/null
+++ b/crates/doc-rules/benches/bench_doc_rules.rs
+// Baseline: full docs audit over the workspace root (parity of runs asserted).
+fn bench_audit_all(c: &mut Criterion) { ... }
```

**Open Questions:** None

---

### Scope 4: Govern technology standards
<!-- Stack selection, naming convention, pattern adherence, framework standardization, dependency policy, security baseline -->

#### Issue ARCH-4-001-202609281309

**Title:** [ARCH][CRITICAL] Dogfooding failure: AES607 fires on the repository that ships it (doc-rules 0↔2, structure-rules 0↔1)

**Label:** design, severity-critical

**Location Path:**
`crates/doc-rules/FRD.md` (Requirements table) · `crates/structure-rules/FRD.md` (Requirements table) · `crates/shared/src/doc_rules/contract_doc_protocol.rs`

**Description:**
Reproducing the shipped counters against this repo: 14 of 16 feature crates are at parity (e.g. auto-fix 6↔6, config-system 10↔10), but the two newest features fail their own standard — `doc-rules` counts **0** FR headings (its FRD uses table-style requirements, invisible to `count_fr_headings`) against **2** protocol traits (one being the orphan `IDocAuditProtocol`), and `structure-rules` counts **0** against **1**. So the first run of `docs` after this feature merge reports the enforcing feature itself as non-compliant. A governance rule that its own author-crate violates on day one erodes the standard's authority and will train teams to ignore AES607 findings.

**Acceptance Criteria:**
- `docs` audit over this repository reports **zero** AES607 findings.
- `crates/doc-rules/FRD.md` declares exactly as many FR headings (canonical format) as `shared/doc_rules` declares `I*Protocol` traits (1 after ARCH-1-001, or 4 if requirements are re-sliced).
- `crates/structure-rules/FRD.md` brought to parity the same way.
- CI gate: the release workflow runs `check .` including the docs group as a merge blocker.

**Recommendation:** Convert both FRDs' Requirements tables to the canonical `### FR-<FEATURE>-NNN:` heading format (which also re-enables AES601's six-field audit over them), delete the orphan trait (ARCH-1-001), and re-slice requirements so counts match honestly rather than cosmetically.

**Git Diff:**
```diff
--- a/crates/doc-rules/FRD.md
-| ID | Requirement |
-|----|-------------|
-| FR-DOC-001 | Every `.md` document satisfies the AES heading and section contract. |
-| FR-DOC-002 | A feature folder carrying a doc pair also carries an orchestrator file. |
-| FR-DOC-003 | Doc findings are reportable per document with line-level detail. |
+### FR-DOC-001: Audit every document against the AES heading and section contract
+...six required fields...
```

**Open Questions:** Should parity for doc-rules land as 1 FR ↔ 1 protocol (single `IDocCheckerProtocol` seam) or should the checker be split into per-group seams (601–604 doc text, 605 folder, 606 headings, 607 parity) with matching FRs?

#### Issue ARCH-4-002-202609281309

**Title:** [ARCH][WARNING] Protocol counting by line-prefix string matching instead of AST — deviates from the workspace's established analysis standard

**Label:** design, severity-warning

**Location Path:**
`crates/doc-rules/src/utility_protocol_counter.rs:count_protocol_traits` (filter: `line.starts_with("pub trait I") && line.contains("Protocol") && !line.contains("Aggregate")`)

**Description:**
The workspace standard for source understanding is AST analysis (AES203 unused-import detection is AST-based across Rust/Python/JS per RULES_AES.md). AES607 instead counts trait declarations by trimmed-line prefix matching, which: misses `pub(crate) trait IFooProtocol` and multi-line/attribute-split declarations; over-counts trait names that merely *contain* "Protocol" (e.g. `IProtocolAggregateExt`-style names are excluded/included by substring accident — `contains("Aggregate")` also excludes a hypothetical `IAggregateSyncProtocol` that *is* a seam); and counts declarations inside `#[cfg(test)]` blocks or doc-example code fences copied into `.rs` files. A governance rule (HIGH severity, per `.agents/rules/RULES_AES.md`) should not rest on grep-grade parsing when the repo already standardizes on syn/AST tooling.

**Acceptance Criteria:**
- Trait counting handles: `pub(crate)`, attributes/doc-comments above the declaration, and suffix-exact matching (`ends_with("Protocol")`, `ends_with("Aggregate")`).
- Unit tests cover the false-positive/false-negative cases above (`unit_doc_rules_protocol_counter.rs`).

**Recommendation:** Minimum: switch to an anchored regex on identifier boundaries with suffix-exact semantics. Preferred: parse with `syn` (already in the dependency tree for AST rules) and count `ItemTrait`s whose ident matches `^I.*Protocol$`.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/utility_protocol_counter.rs
-                line.starts_with("pub trait I")
-                    && line.contains("Protocol")
-                    && !line.contains("Aggregate")
+                trait_decl_re() // r"^pub(?:\(crate\))?\s+trait\s+(I[A-Za-z0-9_]*Protocol)\b"
+                    .captures(line)
+                    .is_some_and(|c| !c[1].ends_with("AggregateProtocol"))
```

**Open Questions:** Is `syn` acceptable in a utility module's dependency budget, or does the flexible-suffix utility policy prefer regex-only?

#### Issue ARCH-4-003-202609281309

**Title:** [ARCH][WARNING] AES607 is Rust-only while the platform standard is multi-language — Python/TypeScript workspaces are silently exempt

**Label:** module, severity-warning

**Location Path:**
`crates/doc-rules/src/utility_protocol_counter.rs` (`.rs` extension filter, `pub trait I` syntax) · `crates/doc-rules/src/capabilities_doc_checker.rs:368` (`crates/shared/src`)

**Description:**
ARCHITECTURE.md §2 declares multi-language workspaces (crates/packages/modules) as a core standard, and the rest of doc-rules honors it (`collect_documents` and `check_feature_folder` iterate all three member roots; RULES_AES describes Python/JS support for other rules). AES607, however, can only ever count Rust traits in `crates/shared/src`: a Python feature's `contract_<domain>_protocol.py` classes or a TypeScript `I*Protocol` interface are never counted, so the parity standard applies to exactly one of the three supported stacks — an inconsistent governance surface. Teams on the other stacks get zero findings and believe they comply.

**Acceptance Criteria:**
- AES607 counts protocol classes for Python (`class I<X>Protocol(Protocol/ABC)`) and TypeScript (`export interface I<X>Protocol`) under `modules/shared/` and `packages/shared/` respectively, or
- RULES_AES.md explicitly scopes AES607 to Rust with a tracked roadmap item for the other stacks.

**Recommendation:** Short term, document the Rust-only scope in `.agents/rules/RULES_AES.md` §AES607 and add a ROADMAP.md entry. Medium term, add per-language counters behind one utility signature (`count_protocol_decls(dir, lang)`), resolved from the member root the FRD lives under (pairs with ARCH-1-003).

**Git Diff:**
```diff
--- a/.agents/rules/RULES_AES.md
 ### AES607 — FR/Protocol Parity
+**Scope (current):** Rust workspaces (`crates/shared/src`). Python/TypeScript
+parity counting is tracked in ROADMAP.md and reports no findings yet.
```

**Open Questions:** None

---

### Scope 5: Manage technical debt strategy
<!-- Debt identification, classification, refactoring priority, deprecation path, backward compatibility, migration plan -->

#### Issue ARCH-5-001-202609281309

**Title:** [ARCH][CRITICAL] Forked rule catalog: root `RULES_AES.md` (396 lines) has no AES607 while `.agents/rules/RULES_AES.md` (614 lines) defines it

**Label:** design, severity-critical

**Location Path:**
`RULES_AES.md` ↔ `.agents/rules/RULES_AES.md`

**Description:**
The workspace carries two divergent copies of the AES rule catalog. The agent-facing copy (`.agents/rules/RULES_AES.md`) documents AES607 (2 mentions); the root copy — the one contributors and external consumers read first, and the one this repo's own architect workflow historically pointed tools at — has **zero** mentions of AES607 and is 218 lines behind. Every new rule now requires a manual dual update that this very PR already missed once. A forked source of truth for the governing standard is classic strategic debt: it compounds with each rule release and guarantees future drift.

**Acceptance Criteria:**
- Exactly one authoritative `RULES_AES.md` exists; the other location is a generated copy, a symlink note, or a short pointer file.
- AES607 (and AES701–705) appear in whatever surface contributors are directed to.
- CI check (or AES604-style crosslink rule) fails when the copies diverge.

**Recommendation:** Declare `.agents/rules/RULES_AES.md` canonical, reduce root `RULES_AES.md` to a pointer (or sync it in CI), and add a drift check to the `ci.yml` docs job.

**Git Diff:**
```diff
--- a/RULES_AES.md
-# (396-line stale catalog, AES601–AES606 era)
+# AES Rules
+Canonical catalog: [.agents/rules/RULES_AES.md](.agents/rules/RULES_AES.md)
+This file is a pointer; do not edit rule text here.
```

**Open Questions:** Is the root copy consumed by any published tooling (e.g. `project-setup` templates) that requires full text rather than a pointer?

#### Issue ARCH-5-002-202609281309

**Title:** [ARCH][WARNING] doc-rules FRD/BACKLOG drift: API table names a nonexistent `DocAuditor`/`IDocAuditProtocol`; BACKLOG has no AES607 task

**Label:** design, severity-warning

**Location Path:**
`crates/doc-rules/FRD.md` (API table) · `crates/doc-rules/BACKLOG.md` (rows end at TASK-DOC-006)

**Description:**
Two document-code drifts shipped with the feature: (1) the FRD's API table documents `DocAuditor::audit` as the `IDocAuditProtocol` implementation — no `DocAuditor` type exists anywhere; the real seam is `DocChecker: IDocCheckerProtocol`. This stale row is also what keeps the orphan trait (ARCH-1-001) looking legitimate. (2) `BACKLOG.md` tracks TASK-DOC-001…006 but has no row for AES607, so the backlog — defined by the FRD itself as "real condition for this feature" — no longer reflects reality one commit after the feature landed. Both are the exact class of drift the doc-rules feature exists to prevent (AES603/AES604 spirit), so they double as debt and as credibility risk.

**Acceptance Criteria:**
- FRD API table rows match real, compiling symbols (`DocChecker::audit` / `IDocCheckerProtocol`).
- `BACKLOG.md` carries `TASK-DOC-007 | AES607 FR/protocol parity | Done`.
- AES604 crosslink audit passes over the updated pair.

**Recommendation:** One-line fixes now, plus (pairing with ARCH-4-001) regenerate the FRD in canonical shape so AES601/602 keep it honest going forward.

**Git Diff:**
```diff
--- a/crates/doc-rules/FRD.md
-| `DocAuditor::audit` | Method | `IDocAuditProtocol` implementation. |
+| `DocChecker::audit` | Method | `IDocCheckerProtocol` implementation. |
--- a/crates/doc-rules/BACKLOG.md
 | TASK-DOC-006 | AES606 agent doc heading structure | Done |
+| TASK-DOC-007 | AES607 FR/protocol parity | Done |
```

**Open Questions:** None

#### Issue ARCH-5-003-202609281309

**Title:** [ARCH][INFO] Stale comments and unresolvable help reference: "five document categories (AES601–AES607)" and a pathless `HOW-TO-MAKE-FRD.md` pointer in user-facing findings

**Label:** design, severity-info

**Location Path:**
`crates/doc-rules/src/capabilities_doc_checker.rs:4` (header comment) · `crates/doc-rules/src/lib.rs:1` · `check_fr_protocol_parity` finding messages ("see the 4-direction table in HOW-TO-MAKE-FRD.md")

**Description:**
Small decay items worth clearing while the feature is fresh: the checker's header still says "five document categories (AES601–AES607)" — seven codes, count never updated; and both AES607 mismatch messages tell users to "see the 4-direction table in HOW-TO-MAKE-FRD.md" without any path — the document actually lives at `crates/skills/aes-docs/references/HOW-TO-MAKE-FRD.md`, which no first-time user will find from the finding text. Low individual cost, but finding messages are the product's UX contract and dangling references age into folklore.

**Acceptance Criteria:**
- Header comments state the correct rule span/category count.
- AES607 messages carry a resolvable relative path (or a stable docs URL) to the 4-direction table.

**Recommendation:** Update the two comments; centralize the help-doc path as a `taxonomy_doc_constant` so future moves update every message at once.

**Git Diff:**
```diff
--- a/crates/doc-rules/src/capabilities_doc_checker.rs
-// each against five document categories (AES601–AES607). Each finding carries
+// each against seven doc invariants (AES601–AES607). Each finding carries
-             up to match, or merge the classes down — see the 4-direction table in \
-             HOW-TO-MAKE-FRD.md"
+             up to match, or merge the classes down — see the 4-direction table in \
+             crates/skills/aes-docs/references/HOW-TO-MAKE-FRD.md"
```

**Open Questions:** None

---

## Violations

1. **AES607 self-violation (live):** `crates/doc-rules/FRD.md` — 0 countable FR headings vs 2 `I*Protocol` traits in `shared/doc_rules`; `crates/structure-rules/FRD.md` — 0 vs 1 (`shared/structure_rules`). (ARCH-4-001)
2. **AES501-class orphan in contract layer:** `IDocAuditProtocol` declared, never implemented, never consumed. (ARCH-1-001)
3. **FRD NFR breach:** doc-rules read-scope NFR ("only `.md` files opened") violated by `count_protocol_traits` reading `.rs` files. (ARCH-2-001)
4. **FRD NFR breach:** Finding-precision NFR ("every finding names a file and a line number") violated by the line-less AES607 finding. (ARCH-2-003)
5. **AES603/604-spirit doc drift:** FRD API table names nonexistent `DocAuditor`; BACKLOG missing the AES607 task row. (ARCH-5-002)
6. **Standards fork:** root `RULES_AES.md` lacks AES607 present in `.agents/rules/RULES_AES.md`. (ARCH-5-001)
7. **Utility-concern leakage:** file discovery implemented inside `capabilities_doc_checker.rs` contrary to ARCHITECTURE.md §7. (ARCH-1-002)

## Action Items

- [ ] P0 Delete orphan `IDocAuditProtocol` from `contract_doc_protocol.rs`; fix FRD API row to `DocChecker::audit` / `IDocCheckerProtocol` (ARCH-1-001, ARCH-5-002)
- [ ] P0 Bring `doc-rules` and `structure-rules` FRDs to canonical FR-heading format and true parity; verify `docs` reports zero AES607 findings on this repo (ARCH-4-001)
- [ ] P0 Reconcile the two `RULES_AES.md` copies into one canonical source with a CI drift check (ARCH-5-001)
- [ ] P0 Update the doc-rules FRD read-scope NFR (or move the source scan behind a dispatcher-provided index) and add `FR-DOC-004` for parity (ARCH-2-001)
- [ ] P1 Unify the FR-heading grammar into one taxonomy constant consumed by AES601 and AES607; flag table-style Requirements sections (ARCH-2-002)
- [ ] P1 Replace silent skip/undercount in `count_protocol_traits` with explicit unverifiable findings; anchor AES607 findings to the Requirements heading line (ARCH-2-003)
- [ ] P1 Derive the kernel module path from the FRD's member root instead of hardcoding `crates/shared/src` (ARCH-1-003)
- [ ] P1 Harden trait counting (anchored regex or `syn` AST) with unit tests for `pub(crate)`, suffix-exact, and cfg(test) cases (ARCH-4-002)
- [ ] P1 Memoize protocol counts per module per audit; add TASK-DOC-007 to BACKLOG (ARCH-3-001, ARCH-5-002)
- [ ] P2 Extract file discovery from `DocChecker` into a utility module (ARCH-1-002)
- [ ] P2 Document AES607's Rust-only scope in RULES_AES and ROADMAP, or add Python/TS counters (ARCH-4-003)
- [ ] P2 Add `benches/bench_doc_rules.rs`; hoist `count_fr_headings` regex into `OnceLock`; fix stale comments and the pathless HOW-TO reference (ARCH-3-002, ARCH-3-003, ARCH-5-003)

## Fixed Document

### Decision A — One contract seam, honestly counted (ARCH-1-001, ARCH-4-001, ARCH-5-002)
`shared/doc_rules` keeps exactly one protocol trait (`IDocCheckerProtocol`) and one aggregate (`IDocRunnerAggregate`). The doc-rules FRD is regenerated in canonical heading form with a requirement set whose cardinality matches the seam count by design, not by accident; `structure-rules` FRD receives the same treatment. BACKLOG gains TASK-DOC-007. Exit test: `docs` over this repository → 0 findings for AES601, AES604, AES607.

### Decision B — One requirement grammar, owned by taxonomy (ARCH-2-002, ARCH-3-002)
`taxonomy_doc_constant::FR_HEADING_PATTERN` becomes the single definition of a requirement heading. AES601's matchers and AES607's counter both compile it (once, via `OnceLock`). Table-style Requirements sections trigger an AES601 finding so no FRD can dodge field checks or zero out its parity count again.

### Decision C — Honest read scope for the docs group (ARCH-2-001, ARCH-1-003, ARCH-4-003)
Either (preferred) the dispatcher supplies a protocol-class index and doc-rules stays markdown-pure, or the FRD's read-scope NFR is rewritten to name the shared contract module explicitly and the kernel path is resolved per member root (`crates|modules|packages`) rather than hardcoded. RULES_AES documents the current Rust-only parity scope until Python/TS counters ship.

### Decision D — Fail loud, count precisely (ARCH-2-003, ARCH-4-002, ARCH-3-001)
Counters return explicit unverifiable outcomes instead of silently skipping or undercounting; AES607 findings carry a line anchor; trait counting moves to anchored, suffix-exact matching (or `syn`); counts are memoized per audit. Determinism NFR gets a regression test: two runs over a tree with an unreadable contract file must emit identical findings.

### Decision E — One rule catalog (ARCH-5-001, ARCH-5-003)
`.agents/rules/RULES_AES.md` is canonical; the root file becomes a pointer, with a CI drift gate. All user-facing finding messages reference resolvable paths sourced from taxonomy constants.

## Severity

| Level | Meaning |
| --- | --- |
| CRITICAL | Layering breach, security architecture gap, scalability blocker, or uncontrolled cross-system dependency. Immediate fix. |
| WARNING | Inconsistent standard, emerging bottleneck, weak integration pattern, or growing technical debt. Fix this cycle. |
| INFO | Architectural optimization, pattern refinement, or long-term improvement. Deferrable. |

# Architecture Compliance Audit — `crates/` vs ARCHITECTURE.md

**Audited commit:** `ce0a219` (branch `arena/974e53aa-lint-arwaky`, version 3.12.2)
**Date:** 2026-11-01
**Scope:** all 41 workspace members + the root package (769 `.rs` files under `crates/`)
**Method:** static re-implementation of the AES rules from ARCHITECTURE.md, cross-checked
line-by-line against the shipped checkers (`crates/*/src/capabilities_*.rs`,
`crates/shared/src/*/utility_*.rs`). No Rust toolchain is available in this
environment, so `cargo test` / `lint-arwaky-cli scan .` were **not** executed —
every finding below cites the code path that would produce it.
**Reproduce:** `python3 scripts/audit_architecture_compliance.py [--json]` (naming/import/doc
surface) and `python3 scripts/audit_module_reachability.py` (AES501–506 wiring graph)

---

## 1. Bottom line

The code base is, on the whole, a faithful implementation of the documented architecture:

- the 7-layer file naming/suffix policy (AES101/AES102) holds for all 450 production source files;
- the import matrix (AES201) and mandatory-import rule (AES202) hold for 404 scoped files,
  with **2 exceptions** (both taxonomy error files);
- every feature crate has capabilities + orchestrator + root container + FRD/BACKLOG +
  `shared/src/<feature>` mirror (AES702), every surface crate has DESIGN/BACKLOG (AES703),
  `shared/` is free of capabilities/agent/surface files (AES701), and no test-suite category
  is missing (AES704);
- no contract trait signature uses a forbidden primitive (AES402); no utility file declares
  state (AES404); no orchestrator implements a protocol or references `Any` (AES405);
- no import cycles inside the crate graph (AES205).

What the naming/import surface cannot show is whether the code is **wired**: the orphan family
(AES501–AES506) has five true positives. One capability pair in `logging` and a second
orchestrator in `filesystem` are built but never connected to their containers, one
`external-lint` capability exists only as a re-export, and two `shared/` utilities plus one
taxonomy VO are reachable only from tests. The `report-formatter` container itself is dead
(A9). Details in §2 A9/A10.

What does **not** hold is the *documentation ↔ implementation* contract. Seven documented
rules are absent from the shipped rule table, one documented rule (AES303) is switched off
while being described as an active HIGH rule, two rules state different things in the doc
than in code (AES304 pattern list, AES401 detection path), and the AES101–AES103 detail
tables are missing three implementation exceptions. Those are listed in §3 and matter most,
because an AI agent reading ARCHITECTURE.md will write code the tool then judges differently.

| Group | Rules | State |
| ----- | ----- | ----- |
| 1 Naming | AES101–103 | doc drift (3 items), 2 files outside the naming pattern |
| 2 Import boundaries | AES201–205 | 2 code violations (taxonomy → utility macro), otherwise clean, no cycles |
| 3 Quality | AES301–305 | AES303 documented-active but disabled; AES304 pattern list ≠ doc; tests excluded from 301/302 (A6 is informational only) |
| 4 Roles | AES401–406 | 1 real AES401 violation; AES402 doc lists 1 of 5 sub-checks; rest clean |
| 5 Orphans | AES501–506 | **5 dead modules** (AES501 ×1, AES503 ×2, AES504 ×2, AES505 ×1) + 1 dead root container (A9) |
| 6 Documents | AES601–610 | 601–607 codes carry two conflicting meanings; 606–610 unregistered |
| 7 Structure | AES701–706 | clean; 705/706 unregistered; doc-internal path conflict |

---

## 2. Implementation that conflicts with ARCHITECTURE.md

### A1 — CRITICAL: two taxonomy error files depend on a **utility** macro (AES201 row 3, AES202)

| File | Line | Evidence |
| ---- | ---- | -------- |
| `crates/shared/src/file_watch/taxonomy_file_watch_error.rs` | 2, 4 | `use shared_common::domain_error_vo;` … `domain_error_vo!(WatchServiceError, …)` |
| `crates/shared/src/git_hooks/taxonomy_git_hooks_error.rs` | 2, 4 | `use shared_common::domain_error_vo;` … `domain_error_vo!(GitHookError, …)` |

The macro is defined in `crates/shared/src/common/utility_value_object_generator.rs:146`,
i.e. a **`utility_*`** file. Per the AES201 matrix:

> `taxonomy(entity,error,event)` — Allowed: `taxonomy`; **Forbidden: agent, surface, contract, utility, capabilities, root**

So a taxonomy error file reaching into the utility layer is a CRITICAL boundary breach, and
the same files also fail the AES202 mandatory import (`taxonomy(vo|constant)` is never imported —
only the macro is).

**Why the linter does not report it:** both checkers resolve a dependency's layer from a
layer-prefixed *path segment* (`crates/shared/src/common/architecture_layer_detector.rs:16-40`,
`crates/shared/src/import_rules/utility_import_resolver.rs:35-100`). The import path
`shared_common::domain_error_vo` contains no `taxonomy_` / `utility_` segment, so the path
resolves to "no layer" and is skipped by AES201. The barrel fallback in AES202
(`capabilities_import_mandatory_checker.rs:269-306`) resolves the path to
`utility_value_object_generator.rs` → layer `utility` ≠ `taxonomy` → the requirement is
considered unsatisfied, so AES202 **should** fire here (HIGH, `MANDATORY_IMPORT`) — that is
the one place the violation is visible, and only as a missing import, not as the boundary
breach it actually is.

This is the highest-value finding: it is simultaneously a real rule violation, a detection
gap, and the reason the two files carry no `struct`/`enum` of their own (see A5).

### A2 — HIGH: a `*_constant.rs` file declares a type alias (AES401)

`crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs:268`

```rust
pub type DocH2Contract = (
```

ARCHITECTURE.md AES401 fires when "a `_constant` file carries a non-const declaration".
`pub type` is in the checker's own forbidden list
(`crates/role-rules/src/capabilities_taxonomy_role_auditor.rs:442`, fallback path), **but the
metadata path does not check Rust type aliases**: `check_constant_with_metadata`
(`…/capabilities_taxonomy_role_auditor.rs:263-327`) iterates struct/enum/trait/function/impl
only, even though `RustMetadata::type_definitions` exists
(`crates/shared/src/filesystem/taxonomy_filesystem_vo.rs:102`) and the TypeScript branch does
check type aliases. So the violation is reported only when tree-sitter metadata is absent —
i.e. the repository's own violation is invisible on the normal code path.

### A3 — HIGH: the same `*_constant.rs` file carries an inline test module (AES401)

`crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs:395-396`

```rust
#[cfg(test)]
mod tests {
```

The AST parser (`crates/shared/src/filesystem/utility_ast_rust.rs`) does not skip
`#[cfg(test)]` items, so every `fn` inside that block is collected into
`RustMetadata::function_definitions` and reported by the metadata path as
`AES401 … Function '…' found in constant file`. It is the only `*_constant.rs` file in the
workspace with inline tests, so the fix is contained.

### A4 — HIGH: two production files carry no AES layer prefix (AES101 pattern)

| File | Stem | Consequence |
| ---- | ---- | ----------- |
| `crates/shared/src/common/architecture_layer_detector.rs` | `architecture_layer_detector` | no layer → AES102, AES201, AES202, AES401/404 all skip it |
| `crates/shared/src/config_system/config_architecture_rules.rs` | `config_architecture_rules` | same; this is also the file that hard-codes the whole AES matrix (690 lines of business rules) |

ARCHITECTURE.md AES101 defines the pattern as `layer_concern_role` with "Layer | Prefix".
The implementation only validates the *shape* (`^[a-z0-9]+(_[a-z0-9]+){2,}$`,
`capabilities_naming_convention_checker.rs:132-176`) and never requires the prefix, so both
files pass silently while being invisible to every layer-scoped rule.

### A5 — AES303: documented as active HIGH, shipped **disabled**; enabling it contradicts AES404

- Gate: `crates/quality-rules/src/capabilities_mandatory_definition_checker.rs:52`
  (`if !def.code_analysis.mandatory_class_definition.value { return; }`).
- Shipped default: `crates/shared/src/config_system/config_architecture_rules.rs:688`
  (`mandatory_class_definition: BooleanVO::new(false)`) and no layer sets it either
  (`CodeAnalysisRuleVO::default()` → `false`).
- ARCHITECTURE.md AES303 says severity HIGH, scope "Every scanned source file" with only
  barrel/entry/`*_constant`/`main` excepted.

If it were enabled, **101 production files** would fire (full list in
`scripts/architecture_audit.json`), and the majority are `utility_*` files — which AES404
*requires* to be stateless standalone functions with **no** struct/trait/impl. AES303 and
AES404 are therefore mutually unsatisfiable for the utility layer, which is presumably why
the rule ships off. The document never says this.

### A6 — INFO: the four AES304 hits are two comments that name the patterns themselves

| File | Line | Token | What it actually is |
| ---- | ---- | ----- | ------------------- |
| `crates/auto-fix/src/capabilities_bypass_fix.rs` | 114 | `FIXME`, `HACK`, `XXX` | a comment describing which markers the fixer strips — one line, three words |
| `crates/shared/src/auto_fix/utility_word_boundary.rs` | 9 | `FIXME` | a doc comment showing the word-boundary rewrite |

These fire only if `FIXME`/`HACK`/`XXX` are active patterns. They are **not** in the shipped
`forbidden_bypass` list (see B6) — they exist only in
`default_forbidden_bypass()` (`crates/quality-rules/src/capabilities_check_bypass_checker.rs:395-420`),
which is used *only* when the configured pattern list is empty. With the shipped architecture the
list is non-empty, so these four words are currently inert: **there is no live violation here.**
The coupling is with B6 — if the team activates those markers, both comments start reporting
CRITICAL and must be reworded to not spell the tokens literally.

### A7 — AES301: two test files exceed the 1000-line ceiling but are never scanned

| File | Lines |
| ---- | ----- |
| `crates/structure-rules/tests/contract_structure_rules.rs` | 1243 |
| `crates/role-rules/tests/unit_role_rules_capabilities_role_auditor.rs` | 1047 |

`DEFAULT_IGNORED_PATHS` contains `tests` and `benches`
(`crates/shared/src/common/taxonomy_default_constant.rs:18-19`) and prunes the production walk
(`crates/filesystem/src/agent_filesystem_orchestrator.rs:501`). ARCHITECTURE.md AES301 says
"Every scanned source file" without ever stating that `tests/` and `benches/` are outside the
scan — see B5.

### A8 — minor: root-package files live at the `crates/` member root

`crates/lib.rs`, `crates/root_cli_main_entry.rs`, `crates/root_mcp_main_entry.rs`,
`crates/root_tui_main_entry.rs`, `crates/root_entry_container.rs`. AES705 only fires for
`capabilities_*`, `agent_*`, `surface_*`, `utility_*` at a member root, so `root_*` is
tolerated here, but ARCHITECTURE.md's "General Layout" does not describe a composition layer
that lives *outside* any crate folder. Cosmetic, but it is the one part of the tree the
document's layout model cannot express.

### A9 — MEDIUM: the `report-formatter` root container is never constructed (AES302 family, uncovered by AES501–506)

| File | Line | Evidence |
| ---- | ---- | -------- |
| `crates/report-formatter/src/root_report_formatter_container.rs` | 17, 21, 39 | `pub struct ReportFormatterContainer`, `impl ReportFormatterContainer`, `impl Default for …` — **no other file in the workspace mentions the type** |

The four formatters are alive, but only because `crates/root_cli_main_entry.rs` builds them
directly (`:3` occurrences) instead of going through the container. So `report-formatter`
ships a composition root that composes nothing: AES702 requires every feature crate to have
a `root_*_container.rs` and that file exists, yet nothing in the rule set notices that it is
dead — AES503/AES505–506 all test *non-root* files, and containers are the seeds of the
liveness walk. Verified with `grep -rn "ReportFormatterContainer" crates --include=*.rs`.

### A10 — MEDIUM: five production modules are never reached from any root container (AES501/503/504/505)

Detected by `scripts/audit_module_reachability.py` (new; see Appendix), which builds the
consumer → producer graph over all 450 production files and walks liveness down from
`root_*`/entry files. Barrel `pub mod` lines are not treated as consumers and barrel
`pub use` re-exports are not treated as wiring — exactly as AES502 states — which is what
makes these visible.

| Rule | File | Why it is dead |
| ---- | ---- | -------------- |
| AES503 | `crates/external-lint/src/capabilities_output_normalizer.rs` | `OutputNormalizer` appears only in its own file plus the `lib.rs:70` **re-export**; `root_external_lint_container.rs` never mentions it and no constructor call exists anywhere. `INormalizeProtocol` itself is still re-exported, so the trait survives with zero live implementors. |
| AES503 | `crates/logging/src/capabilities_filter_build.rs` | `FilterBuilder` / `IFilterBuildProtocol`. `crates/logging/src/root_logging_container.rs:22-33` wires exactly three seams — `SubscriberInit`, `PhaseTimerCapability`, `WalkerReporter` — and never the filter builder. The protocol therefore has no live implementor. |
| AES505 | `crates/filesystem/src/agent_filesystem_pipeline_orchestrator.rs` | `FilesystemPipelineOrchestrator` is implemented and re-exported, but `crates/filesystem/src/root_filesystem_container.rs` wires only `agent_filesystem_orchestrator`, and no other file references the type. Two orchestrators exist for one feature; only one is reachable. |
| AES504 | `crates/shared/src/cli_commands/utility_skill_hint_resolver.rs` | `resolve_skill_hint_for_file[_typed]` is referenced only from `crates/cli-commands/tests/unit_*.rs` (`tests/` is outside the scan per `DEFAULT_IGNORED_PATHS`). No capability/agent/surface/root file imports it. |
| AES504 | `crates/shared/src/maintenance/utility_tool_executor.rs` | `run_tool_in_dir` / `tool_exists` / `get_binary_path` / `run_tool` are referenced only from `crates/maintenance/tests/unit_maintenance_tool_executor.rs`. |
| AES501 | `crates/shared/src/common/taxonomy_skill_hint_vo.rs` | `SkillHint` / `resolve_skill_hint` are consumed by exactly one file — the dead resolver above (`utility_skill_hint_resolver.rs:7-8`). Its only live-ish consumers are the CLI tests, so this taxonomy file has no inbound import from any contract file, which is the letter of AES501. |

Two candidates were checked by hand and **rejected** — record them so the next run does not
re-litigate them:

- `crates/shared/src/common/utility_release_version.rs` — the script flags it because
  `RELEASE_VERSION` is produced by `include!(concat!(env!("OUT_DIR"), "/release_version.rs"))`
  (`:12`), so no definition is visible; six production call sites use
  `shared_common::RELEASE_VERSION` (`cli-commands/src/surface_git_command.rs:25`,
  `dispatcher/src/surface_version_action.rs:14`, …). **False positive.**
- `crates/external-lint/src/capabilities_cargo_dir_resolver.rs` — contains only
  `impl ICargoDirProtocol for ExternalLintExecutor`; the *type* is defined and used in
  `capabilities_command_executor.rs` / `root_external_lint_container.rs`, so the file
  participates in a live impl. Not an orphan (its problem, if any, is placement, not wiring).

**Scorer notes (three bugs fixed while getting here, kept for reviewers):** an earlier pass
reported zero orphans because (i) the BFS walked edges in the *wrong direction*, (ii) `pub mod x;`
declarations in `lib.rs`/`mod.rs` were counted as consumers, and (iii) `referrers()` results were
used as if they were dependencies. A fourth bug — counting `pub fn new` as an exported name —
made `FilterBuilder` and `FilesystemPipelineOrchestrator` look reachable from half the workspace.
Any future re-run should keep all four calibrations.

**Verdict:** the "no orphans" reading of Group 5 is false for this tree. AES501–AES505 all have
at least one true positive; AES502 and AES506 have none, which means the contract layer and the
surface layer are the only two that are fully wired.

---

## 3. Documentation ↔ implementation drift

These are cases where the *tool* and *ARCHITECTURE.md* disagree. They are more dangerous than
§2 for agent-authored code, because the document is declared to be authoritative
(`AGENTS.md`: "ARCHITECTURE.md wins on ambiguity").

### B1 — Rule inventory: 41 documented, 34 registered, 32 in the repo config

| Source | Count |
| ------ | ----- |
| ARCHITECTURE.md Rule Index | 41 |
| `crates/shared/src/config_system/config_architecture_rules.rs` (`static RULES`) | 34 |
| `lint_arwaky.config.yaml` (`architecture.rules`) | 32 |

- Missing from the shipped rule table: **AES606, AES607, AES608, AES609, AES610, AES705, AES706**.
  They still run (unregistered ⇒ "enabled", see `agent_role_orchestrator.rs:272-279`), but they
  can never be switched off or given per-file exceptions through config.
- Missing from `lint_arwaky.config.yaml`: the same seven plus **AES103** and **AES704**.

### B2 — AES601–AES607 carry two conflicting meanings in the same file

`crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs` defines, 300 lines apart:

```rust
pub const RULE_CODE_FR_FORMAT: &str = "AES601";            // :11   legacy "category" meaning
pub const RULE_CODE_SECTION_STRUCTURE: &str = "AES602";    // :59
pub const RULE_CODE_SPEC_PURITY: &str = "AES603";          // :65
pub const RULE_CODE_CROSSLINKS: &str = "AES604";           // :74
pub const RULE_CODE_DOC_STRUCTURE: &str = "AES605";        // :86
pub const RULE_CODE_TEMPLATE_CONFORMANCE: &str = "AES606"; // :110
pub const RULE_CODE_PR_FOLDER_OWNERSHIP: &str = "AES607";  // :313

pub const RULE_CODE_AGENTS_DOC: &str = "AES601";           // :380  documented meaning
pub const RULE_CODE_README_DOC: &str = "AES602";           // :381
…
pub const RULE_CODE_DESIGN_DOC: &str = "AES610";           // :389
```

ARCHITECTURE.md assigns 601–610 **per document** (601 AGENTS, 602 README, 603 ARCHITECTURE,
604 CONTRIBUTING, 605 PRD, 606 ROADMAP, 607 BACKLOG, 608 DATA, 609 FRD, 610 DESIGN). The
category constants reuse the same numbers for *checks*, and the shared contract layer still
documents them that way — `crates/shared/src/doc_rules/contract_doc_protocol.rs:58-92` says
"README.md — **AES604** crosslinks; AES605 heading; AES606 entry tables". So a finding stamped
`AES602` can mean "README document" or "section structure" depending on which capability
produced it. Only `RULE_CODE_TEMPLATE_CONFORMANCE` still has a live caller
(`crates/dispatcher/src/surface_check_scope_action.rs:519`); the rest are dead but continue to
mislead.

### B3 — AES303 is documented as an active HIGH rule and is off by default

See A5. ARCHITECTURE.md should either mark it opt-in (`mandatory_class_definition`) or state
that the utility layer is exempt.

### B4 — AES103: the `tests/common/` support-directory exception is undocumented

`crates/shared/src/naming_rules/taxonomy_naming_rules_constant.rs:91`:

```rust
pub const TEST_SUPPORT_DIRS: &[&str] = &["common"];
```

ARCHITECTURE.md AES103 says a file "nested in a subdirectory" fires. Seven
`tests/common/mod.rs` support modules rely on the undocumented exception
(`crates/dispatcher`, `config-set…/tests/common`, `external-lint`, `maintenance`,
`mcp-server`, `quality-rules`, `shared`).

### B5 — `tests/` and `benches/` are excluded from *every* rule, but the document says otherwise

`DEFAULT_IGNORED_PATHS` (`crates/shared/src/common/taxonomy_default_constant.rs:7-20`) prunes
`tests` and `benches` from the walk, so AES101/102, AES201–205, AES301–305 and AES401–406 never
see them. ARCHITECTURE.md repeatedly says "Scope: Every scanned source file" and never lists
tests/benches as out of scope (except AES103, which *only* covers them). Someone reading the doc
would expect AES304 (`unwrap()` is forbidden) to cover test files — it does not, by design.

### B6 — AES304's documented pattern table ≠ the shipped pattern list

ARCHITECTURE.md lists `FIXME` / `HACK` / `XXX` as forbidden bypass patterns. The shipped list
(`config_architecture_rules.rs`, AES304 rows) is:

```
#[allow( ,unwrap, expect, panic, todo, unimplemented, unreachable, noqa,
type: ignore, eslint-disable, // @ts-ignore, // @ts-expect-error, // @ts-nocheck
```

`FIXME`/`HACK`/`XXX` are absent (only in the empty-config fallback), and
`// @ts-nocheck` is present in code but absent from the document. The `// @ts-ignore` entries
also require the literal `// ` prefix, so a bare `@ts-ignore` on a `/* */` line is not matched.
`raise NotImplementedError`, `assert False` and `throw new <Error>` *are* enforced, but by
hard-coded language branches (`capabilities_check_bypass_checker.rs:283-330`), not by the
pattern list — a distinction the document does not make.

### B7 — AES402: five sub-checks implemented, one documented; "primitive" means something narrower

`crates/role-rules/src/capabilities_contract_rust_role_auditor.rs:33-49` runs
`_contract_primitive`, `check_contract_default_body`, `check_contract_aggregate_method_count`,
`check_contract_dispatch_bag`, `check_contract_untyped_return`. ARCHITECTURE.md AES402
documents only the first. There is no §Sub-checks table like the ones AES403/AES405/AES406 get.

Also, "primitive type" in the checker means `String` (with collection/`Result` exemptions),
numeric types and `char` (`crates/shared/src/role_rules/utility_signature_parser.rs:413-439`) —
`&str` and `bool` are accepted. 10 contract trait methods in this workspace use `&str`
parameters and are compliant today; the document's wording suggests they are not.

### B8 — AES401's Rust metadata path has no type-alias check (see A2)

The document says "non-const declaration" (any kind); the code checks struct/enum/trait/fn/impl
on the fast path and type aliases only on the fallback path.

### B9 — The document gives two different paths for the same thing

`ARCHITECTURE.md:506` (Member Types) → Shared members live in `crates/shared/<domain>/`.
`ARCHITECTURE.md:522,524` (AES702) → a feature must mirror into `shared/src/<feature>/`.
The repository uses `crates/shared/src/<domain>/` — AES702's path. The Member Types row is
wrong, and AES701's "Scope | `shared/` layer folders under member dirs" inherits the ambiguity.

### B10 — AES406 scope differs

The shipped rule rows use `surface(command|controller|page|router|entry)` as the smart tier
(`config_architecture_rules.rs:635-656`). ARCHITECTURE.md's AES406 section never mentions the
`router` or `entry` tiers (AES102's surface row does list `router`), and says the rule has
"one sub-check MEDIUM" while three exist
(`crates/role-rules/src/capabilities_surface_role_auditor.rs:155-164` — the bypass check).

### B11 — AES201 and AES202 share one matrix, presented as two rules

Rows 15–40 and 52–77 of `config_architecture_rules.rs` are byte-identical apart from the code,
and `merge_config` folds both into each layer definition. The document presents allowed /
mandatory / forbidden as a single 13-row matrix but then splits the enforcement across AES201
(forbidden) and AES202 (mandatory). Not a defect, but the doc's "Allowed / Mandatory" columns
suggest a single check.

### B12 — AES305 has no documented threshold

`crates/quality-rules/src/capabilities_code_duplication_analyzer.rs:43-49` uses a default
`75.0` % threshold over windows of ≥10 lines when `duplication_threshold` is unset. The
document says only "exceeds the duplication threshold".

---

## 4. Verified clean (so you can see what was actually checked)

| Check | Result |
| ----- | ------ |
| AES101 shape (lowercase, ≥3 words, barrel/entry excepted) | 450/450 production files pass |
| AES102 suffix policy (strict layers) / forbidden suffixes | 0 violations across all 7 layers |
| AES103 test/bench prefixes, flat layout | 0 violations (7 legal `common/` support modules) |
| AES201 forbidden imports, 404 scoped files | 0 layer-path violations (only the invisible A1 macro edge) |
| AES202 mandatory imports | 2 (A1) |
| AES205 cycles in the crate dependency graph | 0 |
| AES301/302 line limits (scanned files) | 0 |
| AES401 constant purity / entity primitives | 1 (A2/A3), 0 primitive field types in `_entity`/`_error`/`_event` |
| AES402 contract primitives | 0 (`&str`/`bool` allowed by code) |
| AES403 8 sub-checks (type budget, implementor, single protocol, block order/markers, local const, embedded test, helper visibility) | 0 |
| AES404 utility purity (macro bodies and `#[cfg(test)]` excluded, as the checker does) | 0 |
| AES405 all 11 sub-checks | 0 |
| AES701 shared purity + DATA/BACKLOG pair | 0 |
| AES702 feature health (agent, capabilities, root container, FRD+BACKLOG, shared mirror) | 0 for all 17 feature crates |
| AES703 surface purity + DESIGN/BACKLOG | 0 for all 4 surface crates |
| AES704 test-suite coverage (7 categories + bench) | 0 |
| AES705 member-root placement | 0 (only sanctioned `root_*` files) |
| Layer direction of crate dependencies (surface → feature crate) | 0 — commands reach capabilities only through dispatcher actions and contracts |

| AES501–AES506 orphan analysis (wiring graph) | 5 files, A10; AES502 and AES506 empty |
| Root-container liveness | 1 dead container (`report-formatter`, A9) |

Not assessed (needs a real AST / the tool): AES601–AES610 document-shape audits,
AES203/AES204 macro-expansion checks.

---

## 5. Recommended fixes, in order

1. **Fix A1, then widen the detector.** Move `domain_error_vo!` (and the other three macros in
   `utility_value_object_generator.rs`) into a taxonomy-layer file, so taxonomy error files
   import a `taxonomy_*` path; then teach `detect_module_layer`
   (`architecture_layer_detector.rs:77-120`) to resolve `#[macro_export]` macros to their
   defining file so this class of edge is not invisible again.
2. **Fix A2/A3 and close B8.** Add `type_definitions` to the Rust branch of
   `check_constant_with_metadata`, skip `#[cfg(test)]` items in the AST parser, and move
   `DocH2Contract` next to `taxonomy_doc_rules_template_vo`, or leave the file as documented.
3. **Fix A4.** Rename the two unprefixed files (`taxonomy_layer_detector_vo.rs`?
   better: `utility_layer_detector.rs` + `taxonomy_architecture_rules_constant.rs`), and make
   AES101 reject a missing layer prefix for production source files — otherwise layer-scoped
   rules keep silently skipping them.
4. **Settle AES303 (A5/B3).** Either document "opt-in via `mandatory_class_definition`" and
   exempt the utility layer, or delete the rule from the index. Do not leave it documented as
   active HIGH.
5. **Reconcile the rule inventory (B1).** Register AES606–610/705/706 in `static RULES` and in
   `lint_arwaky.config.yaml` (plus AES103/AES704 in the config) so every documented code can be
   suppressed or excepted.
6. **Split the AES60x namespace (B2).** Renumber the legacy category constants
   (`RULE_CODE_FR_FORMAT` … `RULE_CODE_PR_FOLDER_OWNERSHIP`) out of AES601–607 — they collide
   with the documented per-document codes and leak into the shared contract docs.
7. **Document the real AES304 list (B6/A6)** — including which patterns are config-driven and
   which are hard-coded language branches, and whether FIXME/HACK/XXX are meant to be live.
8. **Write down the two scope exclusions (B4/B5)**: `tests/`+`benches/` are outside the scan, and
   `tests/common/` is the one legal sub-directory. Both are behaviour an author cannot infer.
9. **Add the AES402 sub-check table (B7)** and state what "primitive" means for signatures.
10. **Fix the doc-internal path conflict (B9)** and add the root-package files (A8) to the layout
    section.
11. **Wire or delete A10 + A9.** Either construct `OutputNormalizer`, `FilterBuilder` and
    `FilesystemPipelineOrchestrator` from their containers (and make `LoggingContainer` /
    `FilesystemContainer` wire the seam the doc describes), or drop the files and the protocol
    traits they implement. Wire `resolve_skill_hint_for_file` into the CLI violation output it
    was written for, or delete it together with `utility_skill_hint_vo.rs`; same for
    `utility_tool_executor`. Then run `scripts/audit_module_reachability.py` in CI — the current
    rule set cannot see this class of defect for root containers at all, so A9 needs a check of
    its own.

---

## Appendix — reproduction

```bash
python3 scripts/audit_architecture_compliance.py          # human-readable
python3 scripts/audit_architecture_compliance.py --json   # scripts/architecture_audit.json
python3 scripts/audit_module_reachability.py              # AES501–AES506 wiring graph
```

`audit_module_reachability.py` reuses the same file scoping and text normalisation, then builds
a consumer → producer graph over the 450 production files and walks liveness down from the
`root_*`/entry layers. Barrels are excluded as consumers (`pub mod` is a declaration, `pub use`
is not wiring per AES502) and generic method names such as `new`/`build` are not treated as
ownership evidence — both calibrations were needed before the results were stable.

The script mirrors the shipped checkers' scoping (`DEFAULT_IGNORED_PATHS`, barrel/entry
exceptions, the `tests/common/` support dir, `macro_rules!` stripping, `#[cfg(test)]`
skipping, string-literal blanking) so that its findings line up with what
`lint-arwaky-cli scan .` would report. It is a review aid: it does not replace
`cargo nextest run --workspace` or `lint-arwaky-cli scan .`, both of which should be run in CI
to confirm A1–A7.

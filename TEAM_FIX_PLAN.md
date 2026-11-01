# Team Fix Plan — `crates/` vs ARCHITECTURE.md

**Single source of work for this remediation.**
Source audit: [`ARCHITECTURE_AUDIT.md`](ARCHITECTURE_AUDIT.md) (commit `ce0a219`, v3.12.2).
Audit items: `A1–A10` (code conflicts) and `B1–B12` (doc ↔ code drift).
This document turns all 22 items into assignable tickets. Nothing here is speculative —
every ticket cites the file, the line, and the command that proves it is fixed.

**Rules of engagement** (read before your first ticket)

1. **One ticket = one PR.** No mixing a code fix with a doc fix in the same PR; reviewers
   cannot tell which change made CI green.
2. **ARCHITECTURE.md is authoritative** (`AGENTS.md`: "ARCHITECTURE.md wins on ambiguity").
   If a ticket says "the code is right, the doc is wrong", the ticket says so explicitly and
   the doc changes — you do not silently diverge.
3. **Never make CI green by silencing.** No new `#[allow(...)]`, no `// lint-arwaky-disable`,
   no `enabled: false` in `lint_arwaky.config.yaml`, no renaming a rule. If a rule fires on
   your PR, the fix goes in the code or the doc, and the reason goes in the PR body.
4. **Tickets in Wave 1 change the linter itself.** Until they land, the linter under-reports.
   Do not judge any other ticket by "the scan is clean" before Wave 1 is merged.
5. Every PR passes the four gates in [§7](#7-verification-commands) before review.

**Decisions already taken (do not re-litigate)**

| # | Decision | Consequence |
| - | -------- | ----------- |
| D1 | Scope = all `A1–A10` + `B1–B12` | 22 tickets, 4 waves, below |
| D2 | Format = task tickets | this file |
| D3 | Dead code is **deleted**, not wired | T05–T07 delete; **T08 is the one exception** (deleting the container breaks AES702 — see the ticket) |
| D4 | English, direct | — |

**TL;DR for the team lead**
Wave 1 (T01–T04) fixes detection gaps: 2 files are invisible to every layer rule and macro
imports are invisible to AES201 — fix these first or you will "fix" a repo you cannot measure.
Wave 2 (T05–T08) deletes 5 dead modules (6 files / ~1,100 lines, of which ~330 relocate in
T02) and rewires the 1 dead container: ~800 net lines gone, 0 behaviour change. Wave 3 (T09–T11) needs a decision from the architecture owner, not code. Wave 4
(T12–T21) is documentation and can run in parallel from day one with a docs-only reviewer.

---

## 1. Ticket index

| Wave | ID | Audit | Rule(s) | Area | Size | Depends on |
| ---- | -- | ----- | ------- | ---- | ---- | ---------- |
| 0 | T00 | — | baseline | all | XS | — |
| 1 | T01 | A4 | AES101 + AES102/201/202/401/404 coverage | `shared-common`, `shared-config-system` | M | T00 |
| 1 | T02 | A1 | AES201 row 3, AES202 | `shared-common`, `shared-import-rules` | L | T01 (same file) |
| 1 | T03 | A2 | AES401 | `role-rules`, `shared-doc-rules`, `shared-filesystem` | M | T00 |
| 1 | T04 | A3 | AES401 | `shared-filesystem`, `shared-doc-rules` | S | T00 |
| 2 | T05 | A10.1 | AES503, AES502 | `external-lint`, `shared-external-lint` | M | T00 |
| 2 | T06 | A10.2 | AES505 | `filesystem` | S | T00 |
| 2 | T07 | A10.3 | AES504, AES501 | `shared-cli-commands`, `shared-maintenance`, `shared-common` | M | T06 (same shape) |
| 2 | T08 | A9 | AES702 liveness | `report-formatter`, `root package` | S | — |
| 3 | T09 | A5 + B3 | AES303 | `quality-rules`, `docs` | M | decision |
| 3 | T10 | A6 + B6 | AES304 | `quality-rules`, `docs` | S | decision |
| 3 | T11 | A7 + B5 | AES301 scope | `docs` | S | decision |
| 4 | T12 | A8 | layout | `docs` | XS | — |
| 4 | T13 | B1 | rule inventory | `shared-config-system`, `config` | M | T14 (same file) |
| 4 | T14 | B2 | AES601–607 | `shared-doc-rules`, `dispatcher`, `docs` | M | — |
| 4 | T15 | B4 | AES103 | `docs` | XS | — |
| 4 | T16 | B7 | AES402 | `docs` | S | — |
| 4 | T17 | B8 | AES401 | `docs` | XS | T03 |
| 4 | T18 | B9 | AES701/702 | `docs` | XS | — |
| 4 | T19 | B10 | AES406 | `docs` | S | — |
| 4 | T20 | B11 | AES201/202 | `docs` | XS | — |
| 4 | T21 | B12 | AES305 | `docs` | XS | — |

Sizes: **XS** ≤ 2 h · **S** ≈ half a day · **M** 1–2 days · **L** 2–4 days.

---

## 2. Wave 0 — baseline

### T00 · Record the baseline before touching anything (XS)

Otherwise nobody can prove the 22 tickets improved anything.

**Do**

1. On `main`, run the three commands in §6 and save the output:

   ```bash
   python3 scripts/audit_architecture_compliance.py > /tmp/arch-before.txt
   python3 scripts/audit_module_reachability.py    > /tmp/reach-before.txt
   lint-arwaky-cli check . | tail -5               > /tmp/selflint-before.txt
   ```

2. Paste the four headline numbers into the tracking issue: **129 static findings**,
   **5 dead modules**, **1 dead container**, self-lint summary.
3. Keep `/tmp/arch-before.txt` attached to the tracking issue.

**Accept**

- [ ] Baseline numbers recorded and visible to the team.
- [ ] Tracking issue lists all 22 ticket IDs as sub-tasks.

---

## 3. Wave 1 — make the linter honest (T01–T04)

> Until this wave lands, the tool cannot see the files it is supposed to police. Do these first.

### T01 · A4 — give the two unprefixed files a layer prefix (M)

**Severity:** HIGH · **Rules:** AES101 (pattern), and the reason AES102/201/202/401/404 skip two files.

**Why:** AES101 checks only the *shape* of a stem (`^[a-z0-9]+(_[a-z0-9]+){2,}$`,
`crates/naming-rules/src/capabilities_naming_convention_checker.rs:132-176`) and never requires
a layer prefix. Two production files therefore have no layer at all, and every layer-scoped
rule silently skips them.

**Files**

| From | To | Why that layer |
| ---- | -- | -------------- |
| `crates/shared/src/common/architecture_layer_detector.rs` | `…/common/utility_layer_detector.rs` | stateless standalone functions → `utility` (AES102 utility = flexible, any suffix) |
| `crates/shared/src/config_system/config_architecture_rules.rs` | `…/config_system/taxonomy_architecture_rules_constant.rs` | 690 lines of `static`/`const` tables → `taxonomy` + `constant` (allowed suffix) |

**Do**

1. `git mv` both files.
2. Update the module declarations: `crates/shared/src/common/mod.rs:2`, and the
   `config_system/mod.rs` line that declares `config_architecture_rules`.
3. Update every importer:
   `grep -rn "architecture_layer_detector\|config_architecture_rules" crates --include=*.rs`.
4. Fix whatever the rules now report on these two files. **Expect new findings** — that is the
   point of the ticket. Known-good news: the detector holds no stateful statics (verified), so
   AES404 should stay quiet.
5. Harden the checker: AES101 must reject a production `*.rs` file whose stem carries no
   recognized layer prefix (barrels and test/bench files excepted). Add the unit test.

**Accept**

- [ ] Both files carry a prefix and a legal suffix.
- [ ] `python3 scripts/audit_architecture_compliance.py` → `AES101-unprefixed` **2 → 0**.
- [ ] AES101 rejects a hypothetical `common_helper_file.rs` (new test).
- [ ] All importers compile; no `#[allow]` added.

**Verify**

```bash
cargo nextest run -p shared-common-lint-arwaky -p shared-config-system-lint-arwaky
cargo nextest run -p naming-rules-lint-arwaky
python3 scripts/audit_architecture_compliance.py | head -20
```

**Note:** do this **before T02** — both tickets edit the same detector file.

---

### T02 · A1 — move the four value-object macros out of the utility layer (L)

**Severity:** CRITICAL · **Rules:** AES201 row 3 (`taxonomy` must not import `utility`), AES202.

**Why:** `crates/shared/src/common/utility_value_object_generator.rs` defines four
`#[macro_export]` macros that **15 taxonomy files** depend on. The audit names only the two
`taxonomy_*_error.rs` files because those are the only ones that *also* fail AES202 — the
boundary breach is 15× wider:

```bash
grep -rln "string_value_object!\|domain_error_vo!\|list_wrapper_vo!\|primitive_value_object!" \
  crates --include=*.rs | grep -v /tests/
```

The linter misses it because `#[macro_export]` addresses macros at the crate root
(`use shared_common::domain_error_vo;`) — the path carries no `taxonomy_`/`utility_` segment, so
`detect_module_layer` returns `None` and AES201 skips the edge
(`crates/shared/src/import_rules/utility_import_resolver.rs:35-100`).

**Do**

1. Create `crates/shared/src/common/taxonomy_value_object_vo.rs`.
   Move the four macros **verbatim, still `#[macro_export]`**:
   `string_value_object` (`:27`), `domain_error_vo` (`:146`), `list_wrapper_vo` (`:234`),
   `primitive_value_object` (`:272`). Keep the `// PURPOSE:` header convention.
2. Delete `utility_value_object_generator.rs` and its `pub mod` line
   (`crates/shared/src/common/mod.rs:27`).
3. Leave call sites untouched — `#[macro_export]` keeps `use shared_common::string_value_object;`
   valid.
4. Teach the detector to resolve a macro import to its **defining file**: build
   `macro name → file` once per scan from `#[macro_export] macro_rules! <name>`, next to the
   existing prefix table in `utility_layer_detector.rs` (T01).
5. Add an import-rules unit test: a taxonomy file importing an exported macro defined in a
   taxonomy file passes AES201; the same import pointing at a utility-defined macro fails with
   the utility layer named.

**Accept**

- [ ] `grep -rn "value_object_generator" crates --include=*.rs` → only `tests/` snapshots, if any.
- [ ] `python3 scripts/audit_architecture_compliance.py` → `AES202` **2 → 0**.
- [ ] AES201 reports no taxonomy→utility edge for the macros.
- [ ] New test proves macro-definition resolution.

**Verify**

```bash
cargo nextest run -p shared-common-lint-arwaky -p shared-import-rules-lint-arwaky -p import-rules-lint-arwaky
python3 scripts/audit_architecture_compliance.py | grep AES202
```

**Traps**

- Do **not** name the new file `*_constant.rs`: AES401 would start policing a file that is
  nothing but macro bodies. `_vo` is correct.
- The macros use `$crate::taxonomy_path_vo::…` paths; those resolve through crate-root
  re-exports and need no change.

---

### T03 · A2 — AES401 must see Rust type aliases, and `DocH2Contract` must move (M)

**Severity:** HIGH · **Rules:** AES401.

**Why:** `crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs:268` declares
`pub type DocH2Contract = (…)`. The checker's *fallback* path forbids type aliases
(`crates/role-rules/src/capabilities_taxonomy_role_auditor.rs:413-442`) but the normal
metadata path (`:263-327`) reads struct/enum/trait/fn/impl only — even though
`RustMetadata::type_definitions` exists
(`crates/shared/src/filesystem/taxonomy_filesystem_vo.rs:102`). So the repo's own violation is
invisible on the fast path.

**Do**

1. In `check_constant_with_metadata`, iterate `metadata.type_definitions` and raise AES401 per
   item, with the same message shape as the fallback path.
2. Move `pub type DocH2Contract` out of the constant file into
   `crates/shared/src/doc_rules/taxonomy_doc_h2_contract_vo.rs` (or beside
   `taxonomy_doc_rules_template_vo.rs` — pick whichever its consumers already import).
   Update every `grep -rn "DocH2Contract"` site.
3. Unit test: a `_constant` file containing `pub type X = ();` produces one AES401.

**Accept**

- [ ] AES401 findings on `taxonomy_doc_rules_constant.rs` = 0 (auditor moves to 0 for this file).
- [ ] New test fails on the old code and passes on the new.
- [ ] T17 (doc wording) merged or queued with this PR.

**Verify**

```bash
cargo nextest run -p role-rules-lint-arwaky -p shared-doc-rules-lint-arwaky
python3 scripts/audit_architecture_compliance.py | grep AES401
```

---

### T04 · A3 — the AST parser must skip `#[cfg(test)]`, and the inline tests must move (S)

**Severity:** HIGH · **Rules:** AES401.

**Why:** `taxonomy_doc_rules_constant.rs:395-396` carries `#[cfg(test)] mod tests { … }`.
`crates/shared/src/filesystem/utility_ast_rust.rs` has no `#[cfg(test)]` handling at all
(verified: no `cfg(test)` / `attribute_item` branch), so every test `fn` inside is collected as a
production function definition.

**Do**

1. In the parser, skip any item nested under an `#[cfg(test)]` attribute (walk
   `attribute_item` siblings; mirror how the parser already skips other attributes).
2. Move the inline `mod tests` block into the crate's test target
   (`crates/shared/tests/…`, the convention these sub-crates already use; register a `[[test]]`
   entry like `crates/shared/src/common/Cargo.toml:48-50`).
3. Parser unit test: a `#[cfg(test)] mod` containing `fn` yields zero `function_definitions`.

**Accept**

- [ ] No `_constant.rs` file contains `#[cfg(test)]` (`grep -rn "cfg(test)" crates/**/taxonomy_*constant*.rs` → empty).
- [ ] Parser test green; AES401 on the constant file = 0.

**Verify**

```bash
cargo nextest run -p shared-filesystem-lint-arwaky -p shared-doc-rules-lint-arwaky
```

---

## 4. Wave 2 — delete the dead wiring (T05–T08)

> Each of these files is unreachable from every entry point. Verified with
> `scripts/audit_module_reachability.py`, then by hand. **Deleting them changes no behaviour.**

### T05 · A10.1 — delete `OutputNormalizer` and `INormalizeProtocol` (M)

**Rules:** AES503, AES502.

**Files to delete / edit**

| Action | Path |
| ------ | ---- |
| delete | `crates/external-lint/src/capabilities_output_normalizer.rs` (whole file) |
| edit | `crates/external-lint/src/lib.rs:70-71` — remove `pub mod capabilities_output_normalizer;` and `pub use capabilities_output_normalizer::OutputNormalizer;` |
| delete | `INormalizeProtocol` trait in `crates/shared/src/external_lint/contract_external_lint_protocol.rs` |
| edit | `crates/shared/src/external_lint/mod.rs:20` — remove the `pub use … INormalizeProtocol;` |
| edit | `crates/external-lint/tests/contract_external_lint.rs:464-470` — remove the "OutputNormalizer implements INormalizeProtocol" block |
| edit | `crates/external-lint/tests/unit_external_lint_md_markdownlint_adapter.rs:547-≈575` — remove both normalizer blocks |
| edit | `crates/shared/tests/acceptance_shared_contracts.rs:93,218`, `crates/shared/tests/contract_shared.rs:15,360,405`, `crates/shared/tests/e2e_shared_contracts.rs:89` — drop the `INormalizeProtocol` assertions |

**Do**

1. Delete the capability file and the trait (a trait with zero implementors is exactly the
   AES502 case — do not keep it "for later").
2. Delete the test assertions that construct the unit struct. Do **not** delete whole test
   files that cover other things; remove only the normalizer blocks.
3. Update PRD traceability: `PRD.md:45` and `:211/:216` claim normalization is done here.
   Re-point the sentence at the adapter that actually normalizes output, or close the FR in the
   PR body. **This is required** — a dangling FR reference is how the dead trait comes back.
4. If the arch owner decides the normalizer is product intent after all, stop and convert this
   ticket to a wiring task instead (construct it in `root_external_lint_container.rs`). The
   default per D3 is delete.

**Accept**

- [ ] `grep -rn "OutputNormalizer\|INormalizeProtocol" crates --include=*.rs` → empty.
- [ ] `scripts/audit_module_reachability.py` no longer lists the file.
- [ ] PRD sentence updated or FR closed.
- [ ] No `#[allow(dead_code)]` anywhere in the diff.

**Verify**

```bash
cargo nextest run -p external-lint-lint-arwaky -p shared-external-lint-lint-arwaky
python3 scripts/audit_module_reachability.py
```

---

### T06 · A10.2 — delete the second filesystem orchestrator (S)

**Rules:** AES505.

**Why:** `crates/filesystem/src/agent_filesystem_pipeline_orchestrator.rs` implements
`IFilesystemAggregate` for `FilesystemPipelineOrchestrator`, but
`crates/filesystem/src/root_filesystem_container.rs` wires `agent_filesystem_orchestrator` only,
and no other file in the workspace names the type. Two orchestrators, one reachable.

**Do**

1. Delete the file and `crates/filesystem/src/lib.rs:21`.
2. Keep `IFilesystemAggregate` — the live orchestrator implements it.
3. `grep -rn "FilesystemPipelineOrchestrator\|pipeline_orchestrator" crates docs *.md` and clean
   any doc mention (FRD/BACKLOG of the feature crate included).

**Accept**

- [ ] No reference to `FilesystemPipelineOrchestrator` anywhere.
- [ ] `scripts/audit_module_reachability.py` → AES505 **1 → 0**.

**Verify**

```bash
cargo nextest run -p filesystem-lint-arwaky
python3 scripts/audit_module_reachability.py
```

---

### T07 · A10.3 — delete the two test-only `shared` utilities and the orphan taxonomy VO (M)

**Rules:** AES504, AES501.

**Files**

| Action | Path | Evidence |
| ------ | ---- | -------- |
| delete | `crates/shared/src/cli_commands/utility_skill_hint_resolver.rs` | only callers are `crates/cli-commands/tests/unit_cli_formatting.rs:2,103,115,125` and `crates/cli-commands/tests/unit_utility_skill_hint_resolver.rs` |
| edit | `crates/shared/src/cli_commands/mod.rs:4,20,21` | drop `pub mod` + both `pub use` lines |
| delete | `crates/cli-commands/tests/unit_utility_skill_hint_resolver.rs` + its `[[test]]` entry | whole file tests only the deleted resolver |
| edit | `crates/cli-commands/tests/unit_cli_formatting.rs` | remove the two skill-hint assertions; keep the formatting ones |
| delete | `crates/shared/src/common/taxonomy_skill_hint_vo.rs` | only consumer was the deleted resolver (`:7-8`) |
| edit | `crates/shared/src/common/mod.rs:23,99,100` | drop `pub mod` + `SkillHint` / `resolve_skill_hint` re-exports |
| delete | `crates/shared/tests/unit_common_taxonomy_skill_hint_vo.rs` + `crates/shared/src/common/Cargo.toml:48-50` `[[test]]` | tests only the deleted VO |
| delete | `crates/shared/src/maintenance/utility_tool_executor.rs` | only caller is `crates/maintenance/tests/unit_maintenance_tool_executor.rs` |
| edit | `crates/shared/src/maintenance/mod.rs:9` | drop `pub mod utility_tool_executor;` |
| delete | `crates/maintenance/tests/unit_maintenance_tool_executor.rs` + `[[test]]` entry | — |

**Do**

1. Delete the six files above; remove every `pub mod` / `pub use` / `[[test]]` registration in
   the same commit (an orphaned `[[test]]` path is a hard build error).
2. Do **not** delete `crates/cli-commands/tests/unit_cli_formatting.rs` — it covers formatting
   that is alive; trim only the skill-hint assertions.
3. If the team wants to keep skill hints as a feature, stop: that is a T-wiring task (construct
   the resolver from the CLI violation printer). Default per D3 is delete.

**Accept**

- [ ] `grep -rn "skill_hint\|SkillHint\|utility_tool_executor\|run_tool_in_dir" crates --include=*.rs` → empty.
- [ ] `scripts/audit_module_reachability.py` → AES504 **3 → 1** (the one remaining hit is the
      `utility_release_version.rs` false positive documented in the audit), AES501 **1 → 0**.

**Verify**

```bash
cargo nextest run -p shared-cli-commands-lint-arwaky -p cli-commands -p shared-maintenance-lint-arwaky -p maintenance-lint-arwaky -p shared-common-lint-arwaky
python3 scripts/audit_module_reachability.py
```

**Note:** the reachability script's `utility_release_version.rs` finding is a known false
positive (`include!(OUT_DIR/release_version.rs)`). Do **not** delete that file — six production
call sites use `shared_common::RELEASE_VERSION`. Fix the script instead if you want a clean run:
skip files containing `include!(concat!(env!("OUT_DIR")`.

---

### T08 · A9 — resolve the dead `report-formatter` container (S) — *the one exception to D3*

**Rules:** AES702 (requires a `root_*_container.rs` per feature crate), AES501–506 (cannot see it).

**Why:** `crates/report-formatter/src/root_report_formatter_container.rs:17` defines
`ReportFormatterContainer`; nothing else in the workspace mentions the type. The four formatters
are alive only because `crates/root_cli_main_entry.rs` constructs them directly (3 references).

**Why not delete:** deleting the container removes the file AES702 *requires* for a feature
crate, so the self-lint goes red in the same PR and you would have to weaken AES702 to land it.
Deleting "dead code" that a rule mandates is not a fix; it is trading one finding for another.

**Do (recommended)**

1. Move the four formatter constructions from `crates/root_cli_main_entry.rs` into
   `ReportFormatterContainer::new()` (the container already has the shape:
   `Default` + accessors at `:21-45`).
2. Delete the duplicated construction in the entry point; the entry now holds the container and
   calls its accessor.
3. Result: the container is live, AES702 stays satisfied, and the duplication that *caused* the
   dead container disappears.

**If the owner insists on deletion:** the same PR must (a) delete the container, (b) relax AES702
to accept a feature crate that exposes formatters through the entry point, and (c) record the
exception in `ARCHITECTURE.md` §AES702. Three changes, one reviewer sign-off — say so in the PR.

**Accept**

- [ ] `grep -rn "ReportFormatterContainer" crates --include=*.rs` shows a constructor call outside the container file.
- [ ] `lint-arwaky-cli check .` still passes AES702.
- [ ] No duplicated `::new()` of the four formatters left in `root_cli_main_entry.rs`.

**Verify**

```bash
cargo nextest run -p report-formatter-lint-arwaky
lint-arwaky-cli check . | grep -i "AES702\|AES505"
```

---

## 5. Wave 3 — rule-semantics decisions (T09–T11)

> These need the architecture owner in the room. The code is fine; the *contract* is not.
> Each ticket ends with a written decision appended to `ARCHITECTURE.md` — an undocumented
> decision is how we got here.

### T09 · A5 + B3 — settle AES303 (`mandatory_class_definition`) (M · decision)

**Why:** ARCHITECTURE.md lists AES303 as an active **HIGH** rule for "every scanned source file".
The shipped default is `false`
(`crates/shared/src/config_system/taxonomy_architecture_rules_constant.rs` line ~688 after T01;
gate at `crates/quality-rules/src/capabilities_mandatory_definition_checker.rs:52`). If enabled,
**101 production files** fire — most of them `utility_*`, which AES404 *requires* to have no
struct/trait/impl. The two rules are mutually unsatisfiable for the utility layer.

**Decide (one of)**

| Option | Consequence |
| ------ | ----------- |
| **A (recommended)** keep disabled, document as opt-in + exempt the utility layer | smallest change, honest doc, no code churn |
| B | enable only for `contract`/`capabilities`/`agent`/`surface` layers and fix the ~30 resulting findings |
| C | delete AES303 from the index and the rule table | removes a documented capability; needs release note |

**Do (for option A)**

1. ARCHITECTURE.md AES303 section: mark **"opt-in — `code_analysis.mandatory_class_definition`"**,
   state the utility-layer exemption and cite AES404 as the reason.
2. `lint_arwaky.config.yaml`: add `AES303: { enabled: false }` with a one-line comment naming the
   flag (so the switch is discoverable, not hidden in Rust).
3. Test: default value of `mandatory_class_definition` is `false`, and the checker returns early.

**Accept**

- [ ] Doc, config, and code agree on "off by default".
- [ ] The 101-file list is no longer a surprise: the doc explains it.
- [ ] Decision recorded in the PR body (option chosen + who approved).

---

### T10 · A6 + B6 — align AES304's pattern list with the doc (S · decision)

**Why:** ARCHITECTURE.md lists `FIXME` / `HACK` / `XXX` as forbidden patterns. The shipped list
(`taxonomy_architecture_rules_constant.rs`, AES304 rows) is
`#[allow(`, `unwrap`, `expect`, `panic`, `todo`, `unimplemented`, `unreachable`, `noqa`,
`type: ignore`, `eslint-disable`, `// @ts-ignore`, `// @ts-expect-error`, `// @ts-nocheck` —
`FIXME`/`HACK`/`XXX` appear **only** in the empty-config fallback
(`crates/quality-rules/src/capabilities_check_bypass_checker.rs:395-420`). The doc also omits
`// @ts-nocheck`, and never mentions that `raise NotImplementedError`, `assert False` and
`throw new Error` are enforced by hard-coded language branches (`:283-330`), not the list.

**Decide**

- **Option A (recommended):** the code is right — `FIXME`/`HACK`/`XXX` stay fallback-only.
  Update the doc: real pattern table, `// @ts-nocheck` added, the literal `// ` prefix
  requirement stated, the three language branches documented as separate checks.
- **Option B:** activate the three markers in the shipped list. Then also reword the two
  comments that spell them literally — `crates/auto-fix/src/capabilities_bypass_fix.rs:114` and
  `crates/shared/src/auto_fix/utility_word_boundary.rs:9` — or they report CRITICAL on the next
  self-lint (this is exactly audit item A6, informational today).

**Accept**

- [ ] Doc table == shipped list, entry for entry (diff them, do not eyeball).
- [ ] `// @ts-nocheck` documented if present in code.
- [ ] Option B ⇒ both comments reworded, self-lint clean.

---

### T11 · A7 + B5 — document what the scan actually skips (S · decision)

**Why:** `DEFAULT_IGNORED_PATHS` (`crates/shared/src/common/taxonomy_default_constant.rs:7-20`)
prunes `tests` and `benches` from the production walk, and
`crates/filesystem/src/agent_filesystem_orchestrator.rs:501` applies it. So AES101/102,
AES201–205, AES301–305 and AES401–406 never see test code. Every "Scope: Every scanned source
file" line in ARCHITECTURE.md therefore over-promises — the audit's A7 is only a symptom
(two test files exceed the 1000-line AES301 ceiling and are never scanned:
`crates/structure-rules/tests/contract_structure_rules.rs` 1243 lines,
`crates/role-rules/tests/unit_role_rules_capabilities_role_auditor.rs` 1047 lines).

**Do**

1. Add a **"Scope & exclusions"** section near the top of ARCHITECTURE.md: what the walk covers
   (`crates/` production sources), what it skips (`tests/`, `benches/`, build output, vendored
   dirs), and that AES103 is the only rule that inspects test paths.
2. Add one line to each group card (1–4, 6–7): "scope excludes `tests/` and `benches/`".
3. Decide the two long test files: **recommended** split them for readability, but record
   explicitly that AES301 does not apply, so nobody "fixes" a non-finding later.

**Accept**

- [ ] A reader can tell from the doc alone whether a test file is judged by AES304.
- [ ] Two long test files either split or explicitly declared out of scope.

---

## 6. Wave 4 — documentation alignment (T12–T21)

> Docs-only tickets. Can run in parallel with everything above, with a docs reviewer. Each one is
> a paragraph, not a rewrite — keep them small and reviewable.

### T12 · A8 — document the root package (XS)

`crates/lib.rs`, `crates/root_cli_main_entry.rs`, `crates/root_mcp_main_entry.rs`,
`crates/root_tui_main_entry.rs`, `crates/root_entry_container.rs` live at the `crates/` *member
root*, outside any crate folder, and the General Layout section cannot express that.
Add them to the layout tree with a note that AES705 targets `capabilities_*`/`agent_*`/
`surface_*`/`utility_*` only, so and `root_*` at a member root is legal.

**Accept:** the layout section lists them; AES705's scope is stated.

---

### T13 · B1 — register the seven missing rules and nine config entries (M)

**Why:** 41 codes documented, 34 in `static RULES`, 32 in `lint_arwaky.config.yaml`.
Unregistered rules still run (unregistered ⇒ enabled, `agent_role_orchestrator.rs:272-279`) but
can never be disabled or excepted.

**Do**

1. Add to `static RULES`: **AES606, AES607, AES608, AES609, AES610, AES705, AES706**.
2. Add to `lint_arwaky.config.yaml` (`architecture.rules`): the seven above **plus
   AES103 and AES704**.
3. Add a test asserting: every code in ARCHITECTURE.md's Rule Index is in `RULES` and in the
   config ⇒ no future code ships half-registered.
4. Run the self-lint. Registering flips nothing today (they already run), but a mis-set severity
   will show up — fix the severity in the table, not in the config.

**Accept**

- [ ] 41 / 41 / 41 (documented = registered = configurable).
- [ ] `python3 scripts/audit_architecture_compliance.py` `_meta` block shows `41 41 41`.
- [ ] New coverage test in CI.

---

### T14 · B2 — split the AES60x namespace (M)

**Why:** `crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs:11-313` assigns AES601–607
to *checks* (`RULE_CODE_FR_FORMAT` … `RULE_CODE_PR_FOLDER_OWNERSHIP`), while `:380-389` assigns
the same numbers to *documents* (`RULE_CODE_AGENTS_DOC` … `RULE_CODE_DESIGN_DOC`) per
ARCHITECTURE.md. A finding stamped `AES602` can mean "README" or "section structure". The shared
contract still propagates the legacy meaning (`crates/shared/src/doc_rules/contract_doc_protocol.rs:58-92`).

**Do**

1. Move the seven legacy category constants to a distinct code space (e.g. `AESD01…AESD07` or a
   `DOC_*` prefix) — they are internal categories, not ARCHITECTURE.md rules.
2. Keep the one live caller working: `crates/dispatcher/src/surface_check_scope_action.rs:519`
   uses `RULE_CODE_TEMPLATE_CONFORMANCE`.
3. Update `contract_doc_protocol.rs:58-92` text to the documented per-document codes.
4. If any external consumer keys on those literals, add a translation table in `DATA.md` and say
   so in the PR (this is the only backward-compat risk in the ticket).

**Accept**

- [ ] No `"AES60[1-7]"` literal remains in the legacy category block.
- [ ] `grep -rn '"AES60[1-9]"\|"AES610"' crates` shows only per-document meanings.
- [ ] Self-lint unchanged or explained.

---

### T15 · B4 — document the `tests/common/` exception (XS)

`TEST_SUPPORT_DIRS = ["common"]` lives in
`crates/shared/src/naming_rules/taxonomy_naming_rules_constant.rs:91`, so AES103 tolerates
`tests/common/mod.rs` — undocumented, and seven crates rely on it (`dispatcher`, `config-set`,
`external-lint`, `maintenance`, `mcp-server`, `quality-rules`, `shared`).
Add the exception to AES103's AE… card ("one legal sub-directory: `tests/common/`").

**Accept:** AES103 section states the exception and names the constant.

---

### T16 · B7 — document AES402's five sub-checks and the real meaning of "primitive" (S)

`crates/role-rules/src/capabilities_contract_rust_role_auditor.rs:33-49` runs five checks
(`_contract_primitive`, `check_contract_default_body`, `check_contract_aggregate_method_count`,
`check_contract_dispatch_bag`, `check_contract_untyped_return`); the doc mentions one.
"Primitive" in code means `String` (with `Result`/collection exemptions), numeric types and
`char` — `&str` and `bool` are **accepted** (`crates/shared/src/role_rules/utility_signature_parser.rs:413-439`),
and 10 contract methods use `&str` today.

**Do:** add a `§Sub-checks` table like AES403/AES405 already have; define "primitive" precisely;
note the `&str`/`bool` acceptance so nobody rewrites 10 compliant methods.

**Accept:** table lists 5 checks with their severities; wording matches the parser.

---

### T17 · B8 — AES401 wording: type aliases are non-const declarations (XS, pairs with T03)

After T03 the code and doc agree; this ticket only makes the wording explicit:
"`pub type` counts as a non-const declaration in a `_constant` file."

**Accept:** AES401 card updated; T03 test referenced in the PR body.

---

### T18 · B9 — fix the shared-path conflict (XS)

`ARCHITECTURE.md:506` (Member Types) says `crates/shared/<domain>/`; `:522,524` (AES702) says
`shared/src/<feature>/`. The repository uses `crates/shared/src/<domain>/` — AES702's version.
Fix the Member Types row and state AES701's scope in the same terms.

**Accept:** both places name one path; `grep` shows no other variant.

---

### T19 · B10 — AES406: document the `router`/`entry` tiers and the three sub-checks (S)

Shipped rows use `surface(command|controller|page|router|entry)` as the smart tier
(`taxonomy_architecture_rules_constant.rs:635-656` after T01); the doc's AES406 section mentions
neither `router` nor `entry`, and says "one sub-check MEDIUM" while three exist
(`crates/role-rules/src/capabilities_surface_role_auditor.rs:155-164` — the bypass check).

**Accept:** AES406 lists the tiers and all three sub-checks with severities.

---

### T20 · B11 — say that AES201 and AES202 are one matrix (XS)

Rows 15–40 and 52–77 of `taxonomy_architecture_rules_constant.rs` are identical apart from the
code, and `merge_config` folds both into each layer definition. The doc presents allowed /
mandatory / forbidden as one 13-row matrix, then splits enforcement across two rules.

**Accept:** one sentence in the matrix section: "allowed/forbidden = AES201, mandatory = AES202,
same source table."

---

### T21 · B12 — document AES305's threshold (XS)

`crates/quality-rules/src/capabilities_code_duplication_analyzer.rs:43-49`: default `75 %`,
windows of ≥10 lines, when `duplication_threshold` is unset. The doc says only "exceeds the
duplication threshold".

**Accept:** AES305 card states the default and the window size.

---

## 7. Verification commands

```bash
# gates that CI runs (AGENTS.md) — every PR
CARGO_INCREMENTAL=0 cargo fmt --all -- --check
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2
lint-arwaky-cli check .

# progress metric (must trend down / to zero)
python3 scripts/audit_architecture_compliance.py     # baseline 129
python3 scripts/audit_module_reachability.py         # baseline 5 files

# targeted, while working one ticket
CARGO_INCREMENTAL=0 cargo nextest run -p <crate>
CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <test-file>
```

Package names for the crates touched here:
`shared-common-lint-arwaky`, `shared-config-system-lint-arwaky`, `shared-doc-rules-lint-arwaky`,
`shared-filesystem-lint-arwaky`, `shared-import-rules-lint-arwaky`, `shared-external-lint-lint-arwaky`,
`shared-cli-commands-lint-arwaky`, `shared-maintenance-lint-arwaky`, `external-lint-lint-arwaky`,
`filesystem-lint-arwaky`, `logging-lint-arwaky`, `report-formatter-lint-arwaky`,
`role-rules-lint-arwaky`, `quality-rules-lint-arwaky`, `naming-rules-lint-arwaky`, `cli-commands`.

---

## 8. Definition of done (whole plan)

- [ ] `python3 scripts/audit_architecture_compliance.py` → **0** `AES101-unprefixed`,
      **0** `AES202`, **0** `AES303` (by policy, see T09), **0** `AES401`, and the
      `DOC-DRIFT-inventory` block shows **41 / 41 / 41**.
- [ ] `python3 scripts/audit_module_reachability.py` → only the documented
      `utility_release_version.rs` false positive remains (or the script skips `include!` files
      and prints nothing).
- [ ] `lint-arwaky-cli check .` green, including AES702 after T08.
- [ ] All four CI gates green; total test count not reduced except for the explicitly deleted
      dead-code tests in T05/T07 (state the delta in the tracking issue).
- [ ] ARCHITECTURE.md updated for T09–T21; every ticket that changed a rule's meaning links the
      doc line in its PR body.
- [ ] Tracking issue closed with the before/after numbers from T00.

---

## 9. Traps and calibration notes

1. **Wave order is not optional.** T01/T02 change how the linter resolves files and macro
   imports. If you fix T09–T21 first, you are polishing findings that will move.
2. **T01 before T02** — same file (`architecture_layer_detector.rs`). Otherwise you resolve a
   conflict twice.
3. **Do not trust "0 findings" from the current scanner.** The audit found the scanner blind to:
   layer-unprefixed files (A4), macro-export imports (A1), type aliases (A2), `#[cfg(test)]`
   items (A3), and every root container's liveness (A9). Those are exactly T01–T04 and T08.
4. **The reachability script has four hard-won calibrations** (documented in
   `scripts/audit_module_reachability.py`): edges go consumer → dependency; `pub mod x;` is not a
   consumer; `pub use` is not wiring (AES502); generic names like `new`/`build` are not ownership
   evidence. Regressing any of them silently restores the false "0 orphans" result.
5. **`utility_release_version.rs` is a false positive** — `include!(concat!(env!("OUT_DIR"), …))`.
   Do not delete it; six production call sites use `shared_common::RELEASE_VERSION`.
6. **Test changes are part of the deliverable**, not collateral damage: T05/T07 edit 5 test
   files and delete 3 more (plus their `[[test]]` registrations) because they assert code that
   does nothing. State the delta in the PR body so the coverage drop is intentional and
   reviewable.
7. **No Rust toolchain in the audit environment.** Everything in `ARCHITECTURE_AUDIT.md` was
   derived statically; the first thing every ticket does is prove the claim with the real tool
   chain. If a ticket's premise does not reproduce locally, stop and comment on the tracking
   issue instead of "fixing" a non-issue.

---

## 10. Traceability — audit item → ticket

| Audit | Ticket | Audit | Ticket |
| ----- | ------ | ----- | ------ |
| A1 | T02 | B1 | T13 |
| A2 | T03 | B2 | T14 |
| A3 | T04 | B3 | T09 |
| A4 | T01 | B4 | T15 |
| A5 | T09 | B5 | T11 |
| A6 | T10 (informational) | B6 | T10 |
| A7 | T11 | B7 | T16 |
| A8 | T12 | B8 | T17 |
| A9 | T08 | B9 | T18 |
| A10.1 | T05 | B10 | T19 |
| A10.2 | T06 | B11 | T20 |
| A10.3 | T07 | B12 | T21 |

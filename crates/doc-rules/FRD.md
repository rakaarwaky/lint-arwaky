# Doc Rules — FRD

## Feature Overview

The doc-rules crate enforces the AES document-invariant contract across the workspace. It audits every recognized `.md` document against seven rules (AES601–AES607): FR-ID format and field completeness with FR/protocol class parity, section structure shape, spec purity, crosslink integrity, document heading structure, template body-shape conformance with required-document presence, and PR-ID to module-folder ownership. AES606 is answered by four capability seams — table-shape conformance, required-document presence, member-document presence, and template-body placeholder conformance — so this document declares ten requirements against ten protocol classes. Findings carry a machine-readable `violation_type` so consumers can route on them without parsing prose.

Each rule is one capability behind one protocol seam; AES606 carries four seams, so the ten requirements below map one-for-one onto the ten capability seams the agent coordinates. The agent walks the document chain once, hands every capability the same audit context, and merges what they report; the filesystem walk itself lives in the shared utility layer, because an agent may not perform I/O. No external tool is involved.

**Path scoping.** The audit request carries one path, and that path is the **audit root**, not a filter. This differs from `scan`/`check`, which index the whole workspace and then narrow the findings to the target: `docs <path>` discovers the document chain starting at `<path>` and audits only what it finds there. Two consequences follow for a sub-directory run. First, root-level documents outside `<path>` are not audited at all. Second, crosslink and master-document invariants (AES604) resolve against `<path>` — a crate directory has no sibling `PRD.md`/`ROADMAP.md`, so the invariants that look for them behave as they would in a workspace that has none. A whole-workspace audit is the intended default; a sub-directory run is a narrower, self-contained audit, not a filtered view of the full one.

## Functional Requirements

### FR-DOC-001: FR Format, PR-ID Family, Protocol-Class Parity, and BACKLOG Qualifier Alignment (AES601)

| Aspect | Detail|
|--------|-------|
| Description | Audit every recognized requirement document for FR-ID shape, ID-family membership, protocol-class parity, and BACKLOG qualifier alignment. |
| Business Rules | Every FRD heading carries the feature prefix `FR-<FEATURE>-NNN` and a title; every FRD FR-ID owns exactly one BACKLOG `## Backlog` row and every such row points at an existing FR-ID; the FR-heading count equals the count of `pub trait I*Protocol` in the feature's contract module; DS/DT/PR families get the same bidirectional check in DESIGN/DELIVERY Task/PRD; the BACKLOG row qualifier must match its ID family. |
| Edge Cases | A bare `FR-001:` heading fires `id_missing_feature_prefix`; a stray BACKLOG row fires `backlog_id_not_fr` (DS/DT/PR equivalents fire their own codes); a missing row fires `backlog_id_missing`; a qualifier mismatch fires `backlog_qualifier_mismatch`; a count drift between FRs and protocol classes fires one `protocol_count_mismatch` finding on the FRD; documents declaring none of the audited families stay silent. |
| Capability | `FrFormatChecker` implements `IFrFormatProtocol` (`audit_fr_format`). |
| Protocol | `IFrFormatProtocol` in `shared/doc_rules`. |
| Method | `audit_fr_format(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` — the collected document chain plus kernel-source locations resolved from the audit root. |
| Output | `Vec<DocFinding>` — one finding per violation, code `AES601`, stamped on the offending document at the heading line. |
| Error | Audit-only: violations surface as findings, never as process failure; the run still completes and there is no auto-fix. |
| Event | `lint-arwaky-cli docs .` sends `DocRequest::AuditAll`; `scan`/`check` do not run this seam. |
| Test | `acceptance_FR_DOC_001` suite — equal requirement/class counts are accepted while both drift directions are rejected with the parity message; `contract_doc_rules` suite — `aes601_fires_on_bare_fr_id` and `aes601_does_not_validate_fr_row_labels`. |
| Metric | Zero `AES601` findings from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-002: Section Structure (AES602)

| Aspect | Detail|
|--------|-------|
| Description | Report section-order and sanctioned-shape violations so every document reads in the order its template fixes. |
| Business Rules | The FRD's H2s run Feature Overview → Functional Requirements → Public Interface → Reference, and every other doc type uses its own ordered H2 set; fixed sections carry only their sanctioned H3 shapes (Protocol API / Aggregate API subsections, no per-protocol H3, no duplicates); headings inside fenced code blocks are ignored; a demoted H4 under a sanctioned section is accepted. |
| Edge Cases | A missing sanctioned subsection, a duplicated API subsection, a section parked under the wrong parent, prose-only bodies under a mandated section, and an H2 outside the template each fire their own finding; headings in fences never fire; a conforming document reports nothing. |
| Capability | `SectionStructureChecker` implements `ISectionStructureProtocol` (`audit_section_structure`). |
| Protocol | `ISectionStructureProtocol` in `shared/doc_rules`. |
| Method | `audit_section_structure(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` carrying the parsed section tree of each recognized document. |
| Output | `Vec<DocFinding>` — code `AES602`, violation type `order_violation` plus the per-shape message, anchored to the drifting heading line. |
| Error | Audit-only: findings, no auto-fix and no hard failure; the audit continues across all documents. |
| Event | `lint-arwaky-cli docs .` (doc-audit pass). |
| Test | `contract_doc_rules` suite — `aes602_fires_when_sections_are_out_of_order`, `aes602_fires_when_api_contract_has_a_per_protocol_h3`, `aes602_accepts_the_three_sanctioned_h3_shapes`, `aes602_fires_when_scenarios_have_no_bullets`, and the remaining `aes602_*` family; `acceptance_FR_DOC_002` suite — `one_request_runs_every_invariant_and_reports_each_defect`. |
| Metric | Zero `AES602` findings from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-003: Spec Purity (AES603)

| Aspect | Detail|
|--------|-------|
| Description | Keep specifications pure: no source-file path named in spec prose, no status vocabulary leaking out of the master documents. |
| Business Rules | A spec names layers and roles rather than concrete source-file paths (`source_file_named`); progress words belong to ROADMAP/BACKLOG table definitions only, never to spec prose (`status_leak`); prose-only documents are audited with the same rules as tables. |
| Edge Cases | One line may carry both defects and must produce two distinct findings so consumers can route on `violation_type`; legitimate code spans inside fragments that are not file references do not fire; junk text that could be misread as either class is classified consistently. |
| Capability | `SpecPurityChecker` implements `ISpecPurityProtocol` (`audit_spec_purity`). |
| Protocol | `ISpecPurityProtocol` in `shared/doc_rules`. |
| Method | `audit_spec_purity(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with the text of each spec document. |
| Output | `Vec<DocFinding>` — code `AES603`, types `source_file_named` or `status_leak`, anchored to the offending line. |
| Error | Audit-only: findings, never a failure; no rewrite is attempted. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `contract_doc_rules` suite — `aes603_fires_when_a_spec_names_a_source_file`, `aes603_fires_when_a_spec_carries_status`, `aes603_distinguishes_status_leak_from_source_file_naming`, `prose_only_documents_are_audited_for_purity`. |
| Metric | Zero `AES603` findings from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-004: Crosslinks and State-Vocabulary Placement (AES604)

| Aspect | Detail|
|--------|-------|
| Description | Enforce the crosslink set every document owes the chain and keep master-only state sections in the master documents. |
| Business Rules | Each document carries its mandated links (FRD owes PRD and BACKLOG, so a missing pair fires `no_prd_link`/`no_backlog_link`; the other types carry their own `no_*_link` codes); only ROADMAP/BACKLOG masters hold `Roadmap Status` and the state vocabulary — a feature backlog restating it fires `state_vocab_restated`. |
| Edge Cases | README ships a placeholder table and is exempt from the restatement rule; ARCHITECTURE.md is out of scope for AES604; a master-only section restated inside a feature document fires even when every required link is present. |
| Capability | `CrosslinkChecker` implements `ICrosslinkProtocol` (`audit_crosslinks`). |
| Protocol | `ICrosslinkProtocol` in `shared/doc_rules`. |
| Method | `audit_crosslinks(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` carrying the collected link sets and section ownership per document. |
| Output | `Vec<DocFinding>` — code `AES604`, one finding per missing link or restated master section, stamped on the document. |
| Error | Audit-only: findings, no auto-fix; absent targets resolve against the audit root, so a sub-directory run behaves as a workspace that has no siblings. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `contract_doc_rules` suite — `aes604_fires_when_the_frd_omits_its_backlog_link`, `aes604_fires_when_the_frd_omits_its_prd_link`, `aes604_fires_when_a_feature_backlog_restates_master_sections`, `aes604_fires_when_a_feature_backlog_restates_roadmap_status`. |
| Metric | Zero `AES604` findings from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-005: Document Heading Structure (AES605)

| Aspect | Detail|
|--------|-------|
| Description | Enforce each document's heading contract: one H1, the required H2 set, and the verbatim lines the template fixes. |
| Business Rules | Exactly one H1 per document and no unfilled curly placeholder slot in it (`h1_count`); every required H2 present (`h2_missing`) and no off-template H2 (`h2_unexpected`); lines the template fixes must appear verbatim in order, with a curly placeholder slot as the only permitted variation (`line_drift`); frontmatter rules apply where the doc type defines them (`frontmatter_missing`). |
| Edge Cases | A `#` inside a commented command is not an H1; extra free-form H3s are allowed; a missing required H2 names the document in its finding; project-specific H2 slots absorb authored content without firing. |
| Capability | `HeadingStructureChecker` implements `IDocHeadingProtocol` (`audit_doc_heading`). |
| Protocol | `IDocHeadingProtocol` in `shared/doc_rules`. |
| Method | `audit_doc_heading(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with heading maps built from each document's H1/H2/H3 lines against its `DOC_HEADING_CONTRACTS` entry. |
| Output | `Vec<DocFinding>` — code `AES605`, one of `h1_count`, `h2_missing`, `h2_unexpected`, `line_drift`, `frontmatter_missing`, anchored to the offending heading or line. |
| Error | Audit-only: findings; an empty document is handled here rather than by the template-body seam. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `contract_doc_rules` suite — the `aes605_*` family (`aes605_fires_when_no_h1`, `aes605_fires_when_required_h2_is_absent`, `aes605_fires_on_an_h2_outside_the_template`, `aes605_does_not_falsely_fire_on_sharp_commented_commands`, `aes605_allows_the_project_specific_h2_set`, `aes605_accepts_one_h1_and_every_required_h2`). |
| Metric | Zero `AES605` findings from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-006: Template Body-Shape Conformance (AES606)

| Aspect | Detail|
|--------|-------|
| Description | Check that each mandated section carries the exact table shape — columns and entry-table row labels — that its template fixes. |
| Business Rules | A section the template fixes as a table must be a table (`section_not_table`); a table must carry every fixed column (`table_missing_columns`) and nothing more — an undeclared header cell fires `table_extra_columns`; an entry table's first-column labels must equal the template's fixed row set with none missing and none extra (`entry_missing_row`); a data row that repeats the header row ships column names where values belong (`table_header_repeated`); a table under a section whose template shows none fires `table_not_in_template` (Design Overview, the Design Requirements prefix); a DESIGN H3 outside the `DS-<NAME>-NNN` pattern or under `Design Overview`/`Reference` fires `h3_unexpected`; header-only tables and pipe-delimited separators are recognized before any verdict. |
| Edge Cases | A parent H2 with no body is silent — absence is AES605's question; a bullet list or prose under a mandated section fires `section_not_table`; a table whose data row copies the header fires `table_header_repeated`; a component-overview table under `## Design Requirements` fires `table_not_in_template` while the prose above the first DS entry stays silent; tables inside fenced examples are blanked before any verdict; the finding anchors to the offending H3 so a reader lands on the broken table, not the section head. |
| Capability | `TemplateConformanceChecker` implements `ITemplateConformanceProtocol` (`audit_template_conformance`, plus the `check_entry_tables` helper). |
| Protocol | `ITemplateConformanceProtocol` in `shared/doc_rules`. |
| Method | `audit_template_conformance(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with each document's section list and the profile tables registered for its type. |
| Output | `Vec<DocFinding>` — code `AES606`, types `section_not_table`, `table_missing_columns`, `table_extra_columns`, `entry_missing_row`, `table_header_repeated`, `table_not_in_template`, or `h3_unexpected`, each carrying the missing/extra labels in `message` and a fix line. |
| Error | Audit-only: findings, no rewriting of the document. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `unit_doc_template_conformance` suite — `aes606_accepts_a_backlog_carrying_every_mandated_table`, `aes606_rejects_bullet_bodies_under_a_backlog_section`, `aes606_rejects_a_table_missing_one_mandated_column`, `aes606_rejects_prose_under_a_roadmap_state_definitions`; `unit_capabilities_doc_template_conformance` suite covers the entry-table and empty-parent branches plus `design_entry_rejects_repeated_header_row`, `design_entry_accepts_template_blank_data_row`, `design_entry_rejects_extra_header_columns`, `design_rejects_a_table_not_in_template`, `design_rejects_h3_outside_the_ds_pattern`. |
| Metric | Zero `AES606` shape findings (`section_not_table`, `table_missing_columns`, `table_extra_columns`, `entry_missing_row`, `table_header_repeated`, `table_not_in_template`, `h3_unexpected`) from `lint-arwaky-cli docs .` on a conforming workspace. |

### FR-DOC-007: Required-Document Presence (AES606)

| Aspect | Detail|
|--------|-------|
| Description | Report every required root document that does not exist at a real workspace root. |
| Business Rules | `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, and `CONTRIBUTING.md` must exist when the audit root is a workspace root; `FRD.md`, `DATA.md`, and a root `BACKLOG.md` are intentionally not required (they are per-feature files and ROADMAP is the root master); the check is root-gated on `Cargo.toml`, `pyproject.toml`, or `package.json` at the audit root. |
| Edge Cases | A sub-directory audit (`docs crates/`) is silent because it is self-contained and owes no root set; a root lacking all markers stays silent; each missing document produces its own finding, never a combined one. |
| Capability | `RequiredDocChecker` implements `IRequiredDocProtocol` (`audit_required_docs`). |
| Protocol | `IRequiredDocProtocol` in `shared/doc_rules`. |
| Method | `audit_required_docs(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with the collected document set and workspace-root marker detection. |
| Output | `Vec<DocFinding>` — code `AES606`, violation type `missing_required_doc`, one per absent file, anchored at line 0, with the HOW-TO template named in `why`/`fix`. |
| Error | Audit-only: a missing document is a finding; the run never aborts and offers no auto-create. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `unit_doc_template_conformance` suite — `aes606_reports_a_missing_required_root_doc` and `aes606_stays_silent_for_required_docs_when_the_root_is_present`. |
| Metric | Zero `missing_required_doc` findings from `lint-arwaky-cli docs .` on a workspace that ships the full root layout. |

### FR-DOC-008: Member-Folder Document Presence (AES606)

| Aspect | Detail|
|--------|-------|
| Description | Report every member folder that lacks the document pair its role mandates. |
| Business Rules | A folder counts as a feature only when it holds an `*_orchestrator` file and then owes its per-role document pair; `shared` is the kernel and owes `DATA.md` + `BACKLOG.md`; surface folders owe the design pair; `none`-role folders owe nothing; `exempt_members` suppresses listed folders; strictness follows config unless the request force-overrides it. |
| Edge Cases | A feature missing one file of its pair fires one finding per missing file; strict-off suppresses findings, force-strict overrides the config off switch; an exempt member stays silent even under strict. |
| Capability | `MemberDocChecker` implements `IMemberDocProtocol` (`audit_member_docs`). |
| Protocol | `IMemberDocProtocol` in `shared/doc_rules`. |
| Method | `audit_member_docs(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with each member folder, its detected role, and the effective strictness. |
| Output | `Vec<DocFinding>` — code `AES606` with the missing-document violation type, one per absent file, stamped on the member folder. |
| Error | Audit-only: findings; a compliant folder yields an empty vector rather than an error. |
| Event | `lint-arwaky-cli docs .`. |
| Test | `unit_doc_member_and_template_body` suite — `member_doc_checker_feature_missing_pair_fires_findings`, `member_doc_checker_kernel_owes_data_and_backlog`, `member_doc_checker_surface_owes_design_and_backlog`, `member_doc_checker_exempt_member_is_suppressed`, `member_doc_checker_strict_off_suppresses_findings`, `member_doc_checker_force_strict_overrides_config_off`, `member_doc_checker_compliant_feature_no_findings`, `member_doc_checker_none_role_owes_nothing`. |
| Metric | Zero member-document findings from `lint-arwaky-cli docs .` on a workspace where every role folder carries its pair. |

### FR-DOC-009: Template Body & Placeholder Conformance (AES606)

| Aspect | Detail|
|--------|-------|
| Description | Check each registered document's body against its template: sections may not be deleted and unfilled placeholder tokens may not survive strict mode. |
| Business Rules | Every required section of the document's profile stays present (`template_body_checker_deleted_required_section_fires`); under strict placeholders, a catalog token surviving in stripped prose fires `unfilled_placeholder`; tokens inside fenced code are never matched; an extended heading can still satisfy a required section. |
| Edge Cases | Strict-off suppresses placeholder findings while section-deletion findings still fire; a zero-byte document is AES605's `h1_count` question and stays silent here; the prose strip blanks inline code, so a type like `Vec<DocFinding>` never misfires. |
| Capability | `TemplateBodyChecker` implements `ITemplateBodyProtocol` (`audit_template_body`). |
| Protocol | `ITemplateBodyProtocol` in `shared/doc_rules`. |
| Method | `audit_template_body(&self, context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` with each document's text, its `TemplateProfile` (required H2s and placeholder catalog), and the effective strictness from `DocRulesConfigRef`. |
| Output | `Vec<DocFinding>` — code `AES606`, types `unfilled_placeholder` or the deleted-section violation, anchored at the offending line. |
| Error | Audit-only: findings; delimiters never abort the walk over the remaining documents. |
| Event | `lint-arwaky-cli docs .`, honoring `strict_override`/config for placeholder strictness. |
| Test | `unit_doc_member_and_template_body` suite — `template_body_checker_unfilled_placeholder_fires_when_strict_on`, `template_body_checker_strict_off_suppresses_placeholder_findings`, `template_body_checker_placeholder_in_code_fence_not_detected`, `template_body_checker_deleted_required_section_fires`, `template_body_checker_extended_heading_satisfies_required_section`. |
| Metric | Zero `unfilled_placeholder` and zero deleted-section findings from `lint-arwaky-cli docs .` on templates copied and fully filled. |

### FR-DOC-010: PR-ID Module-Folder Ownership (AES607)

| Aspect | Detail|
|--------|-------|
| Description | Every `PR-<PROJECT>-NNN` item declared in the root PRD's `Project Requirements` must own exactly one top-level module folder under `crates\|modules\|packages` whose name matches the PR-ID's project identifier (`PROJECT`). A PRD whose PR-IDs all share one project identifier is satisfied when the layout root exists and is non-empty (that project owns the whole folder set). |
| Business Rules | The root PRD is the only document that declares PR-IDs in `Project Requirements`. Ownership is keyed on the project identifier segment (`PR-LA-001` → `LA`), not on strict cardinality-1 per PR-ID. One identifier may own the whole layout. A PRD declaring no PR-IDs stays silent. A root without a workspace-root marker (`Cargo.toml`, `pyproject.toml`, `package.json`) stays silent. A workspace with no `crates\|modules\|packages` layout root stays silent. |
| Edge Cases | No matching folder for the identifier fires one finding per PR-ID, anchored to its `Project Requirements` heading line, stamped on the PRD document with code `AES607` and violation type `pr_folder_not_owned`. `shared`/`kernel` folders are skipped as kernel dirs. `exempt_members` does not apply — this is a root-PRD-level check, not a per-member check. |
| Capability | `PrFolderOwnershipChecker` implements `IPrFolderOwnershipProtocol` (`audit_pr_folder_ownership`). |
| Protocol | `IPrFolderOwnershipProtocol` in `shared/doc_rules`. |
| Method | `audit_pr_folder_ownership(context: &DocAuditContext) -> Vec<DocFinding>` |
| Input | `DocAuditContext` carrying the document chain rooted at the workspace root. |
| Output | `Vec<DocFinding>` — one finding per PR-ID whose project identifier owns no top-level module folder under `crates\|modules\|packages`. |
| Error | Not applicable; audit-only violation, no auto-fix. |
| Event | `lint-arwaky-cli docs .` (doc-audit pass); not run by `scan`/`check`. |
| Test | Capability test suite covers: clean-matching-folder, missing-folder fires one finding, multi-project membership, single-project owns whole non-empty layout, empty layout fires each PR-ID, non-root silent, no PR-IDs silent, no layout root silent. |
| Metric | Zero `AES607` / `pr_folder_not_owned` findings from `lint-arwaky-cli docs .` on a conforming project. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Agent | `DocOrchestrator` — the agent-layer orchestrator behind the docs command. |
| Aggregate | `IDocRunnerAggregate` — the feature's single front door. |
| Method | `execute(&self, request: DocRequest) -> DocResponse`. |
| Request | `DocRequest::AuditAll { root, config, strict_override }` — the docs command's single entry (built by `DocRequest::audit_all`). |
| Response | `DocResponse::Findings { findings }` — all `DocFinding` rows from the ten seams merged (helpers `count()`, `is_clean()`, `into_findings()`). |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|

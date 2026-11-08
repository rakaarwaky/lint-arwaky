# Step 32: Document Structure: BACKLOG.md Conformance

## What the rule checks

AES607 (rule code `AES607`, checker `capabilities_doc_backlog` in the `doc-rules` crate) verifies BACKLOG.md conformance.

- **BACKLOG ID alignment:** every BACKLOG entry H3 token (e.g. `{{FR-AutoFix-005}}`) must match a declared ID in the paired spec document (FRD.md's FR headings, DATA.md's DT headings, DESIGN.md's DS headings). A BACKLOG entry that cites an undeclared ID fires `dt_backlog_id_not_dt` (or `pr_backlog_id_missing` at root).
- **Reverse direction:** every declared spec ID must have a matching BACKLOG entry. An FR/DATA/DESIGN ID with no BACKLOG H3 fires `id_no_entry`.
- **H2 shape:** BACKLOG.md must carry the required H2 sections in template order.

## Common violation causes

- BACKLOG.md missing a required H2 section.
- H2 heading text drifted from the registered template.
- BACKLOG entry `{{ID}}` cites an ID not declared in the paired FRD/DATA/DESIGN.
- A spec ID (e.g. `FR-AutoFix-005`) has no matching BACKLOG H3 entry.

## How to fix (concrete steps)

1. Identify the violated rule code (AES607) and the specific defect subtype from the scan report.
2. For `dt_backlog_id_not_dt` (a spec ID has no BACKLOG entry): add a BACKLOG `### <ID>` section after the related existing entry, with `State`, `Done`, `In Progress`, `Risk`, `Updated` rows.
3. For a BACKLOG entry citing an undeclared ID: either remove the entry or add the matching ID to the paired spec document.
4. Re-run `lint-arwaky-cli docs crates/shared/skills` or `lint-arwaky-cli scan .` and confirm AES607 no longer fires.

## Verify

```bash
lint-arwaky-cli scan . --filter AES607
# Confirm no AES607 finding remains.
```

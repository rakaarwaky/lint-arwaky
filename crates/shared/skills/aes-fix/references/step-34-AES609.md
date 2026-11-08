# Step 34: Document Structure: FRD.md Conformance

## What the rule checks

AES609 (rule code `AES609`, checker `capabilities_doc_frd` in the `doc-rules` crate) verifies FRD.md conformance.

- **H2 shape:** FRD.md must carry the required H2 sections in template order.
- **FR protocol parity:** every `FR-<FEATURE>-NNN:` heading must have a matching `pub trait I*Protocol` (excluding `I*Aggregate`) in the feature's shared contract module (`crates/shared/src/<feature>/`). The two counts must be equal.
  - `fr_count` = number of H3/H4 headings matching `(?m)^#{2,4}\s+(?P<id>FR-[A-Za-z0-9_]+-\d+):`
  - `protocol_count` = number of `pub trait I*Protocol` lines in the shared module's `.rs` files, where the trait name starts with `I`, ends with `Protocol`, and does not contain `Aggregate`.

## Common violation causes

- FRD.md missing a required H2 section.
- H2 heading text drifted from the registered template.
- **`protocol_count_mismatch`:** the FRD declares a different number of FR headings than the shared module declares protocol traits. This fires when:
  - A protocol trait was added to the shared module without a matching FR heading in the FRD, or
  - An FR heading was removed from the FRD without removing its protocol trait, or
  - The counts were misaligned when the feature was scaffolded.

## How to fix (concrete steps)

1. Identify the violated rule code (AES609) and the specific violation subtype from the scan report.
2. For H2 shape violations: add the missing section or restore drifted headings per FRD.md's template contract.
3. For `protocol_count_mismatch`:
   - Count the FR headings in the FRD (`grep -cP '^#{2,4}\s+FR-[A-Za-z0-9_]+-\d+:' <feature>/FRD.md`).
   - Count the protocol traits in the shared module (`grep -c 'pub trait I' crates/shared/src/<feature>/*.rs | awk '{s+=$1}END{print s}'`).
   - If `protocol_count > fr_count`: add the missing FR heading(s) to the FRD with full Business Logic / Edge Cases / Technical Specification / Test / Metric sections, and add a matching BACKLOG entry in `<feature>/BACKLOG.md`.
   - If `fr_count > protocol_count`: either split the FR into more protocol traits or merge the FR headings down to match the protocol count.
4. Add the matching BACKLOG entry for any new FR ID (AES607 requires every FR/DATA/DESIGN ID to have a BACKLOG H3).
5. Re-run `lint-arwaky-cli scan .` and confirm AES609 no longer fires.

## Verify

```bash
lint-arwaky-cli scan . --filter AES609
# Confirm no AES609 finding remains.
```

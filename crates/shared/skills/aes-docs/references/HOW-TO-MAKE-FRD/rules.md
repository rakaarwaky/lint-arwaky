# FRD Rules

**Purpose:** Tell an engineer exactly what to build and tell QA exactly what to test — without a single follow-up question to the author.
**Audience:** Engineers, QA, Tech Lead.
**Scope:** One FRD per feature folder.
**Location:** Inside the feature's directory.
**Length:** 50–1000 lines.

## Rules

General rules for writing an FRD. These apply across all sections.

1. **Not a feature → no FRD.** A folder is only a valid feature folder when it contains an `agent_*_orchestrator` file. Folders without an orchestrator are not features and must not carry `FRD.md` or `BACKLOG.md`. Cross-cutting kernel / shared layers (e.g. `modules/shared`) have no orchestrator and therefore have **no `FRD.md` and no `BACKLOG.md`**. Pairing (`spec-without-backlog` / `backlog-without-spec`) applies only to feature folders.
2. **Requirement IDs are the contract.** `FR-<FEATURE-NAME-ID>-NNN`, unique within the feature and stable forever.
3. **A requirement is testable, or it is a wish.** State input, output, business rules, edge cases, error handling.
4. **Nothing about the current build.** Shipped/backlog status is a BACKLOG.md claim with evidence, never an FRD sentence.
5. **One observable behaviour per entry.** A reviewer can check it without asking; its `Test` subsection carries observable behaviour + expected result (never "tests pass"), its `Metric` subsection a falsifiable target value.
6. **No acceptance criteria outside the entry tables.** Each FR entry's `Technical Specification`, `Test`, and `Metric` tables — every filled row — IS the acceptance specification; don't duplicate acceptance prose elsewhere.
7. **No source-file extensions in a spec.** Never name `.py`, `.rs`, `.ts`, `.tsx`.
8. **Spec purity — the FRD is a promise.** The FRD states what the feature must do, never how the current build does it. Implementation identifiers (capability names, protocol trait / interface names, capability method names) appear only under the `Capability`, `Protocol`, and `Method` aspect subsections and in `## Public Interface`; every other subsection states behaviour in caller-observable terms.
9. **Caller-observable identifiers may stay.** Error codes, event names, output destination paths, and similar caller-visible vocabulary belong in the `Input`, `Output`, `Error`, and `Event` subsections; they are part of the promise, not implementation.
10. **Purity test.** If a sentence in any subsection names an implementation identifier that is not one of the `Capability` / `Protocol` / `Method` / Public-Interface identifiers above, or leaks build status, move the identifier or cut the sentence.

A `DESIGN.md` in the feature folder is a legitimate input to enumerating FR IDs, but it is not one of the four required Reference links.

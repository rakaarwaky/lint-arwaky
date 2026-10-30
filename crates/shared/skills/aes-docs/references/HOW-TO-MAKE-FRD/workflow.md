# FRD Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the FR ID set first** — enumerate which `FR-<FEATURE-NAME-ID>-NNN` entries the feature needs and why, derived from the feature's deliverables and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per FR ID** — spawn exactly N subagents, one per FR ID; each delegate researches its own requirement (the feature's docs: DESIGN, BACKLOG, PRD plus architecture context) and writes ONLY its own `### FR-<FEATURE-NAME-ID>-NNN` heading plus its `**Description:**` paragraph and its five H4 subsections — `Business Logic`, `Edge Cases`, `Technical Specification`, `Test`, `Metric` — seeding each per the **Seed rows** contract in the Template section. If the FRD needs 15 FR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Functional Requirements` in ID order; the integrator polishes against the Rules above (testable, ID contract, no source extensions, no status leaks, spec purity) and does not invent entries for IDs that were never declared. Fill Feature Overview, Public Interface and Reference here too.
4. **Verify** — run `lint-arwaky-cli docs .` plus the parity/structure checks listed in the Verify section.

## Verify

```bash
lint-arwaky-cli docs .
```
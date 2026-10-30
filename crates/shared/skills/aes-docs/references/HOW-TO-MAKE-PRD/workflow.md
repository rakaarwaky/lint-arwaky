# PRD Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the PR ID set first** — enumerate which `PR-<PROJECT-NAME-ID>-NNN` entries the project needs and why they must exist, derived from the project's top-level deliverable folders and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per PR ID** — spawn exactly N subagents, one per PR ID; each delegate researches its own subject (the folder's FRD, BACKLOG, and ARCHITECTURE docs plus architecture context) and writes ONLY its own `### PR-<PROJECT-NAME-ID>-NNN` heading plus its three H4 aspect subsections (`#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`), seeded per the seed-row contract. If the PRD needs 15 PR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Project Requirements` in ID order; the integrator polishes against the Rules above (no impl detail, testable metrics, explicit exclusions, open-ended seed rows) and does not invent entries for IDs that were never declared.
4. **Verify** — run `lint-arwaky-cli docs .` and the parity/structure checks already listed in the Verify section.

## Verify

```bash
lint-arwaky-cli docs .
```

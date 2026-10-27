# CONTRIBUTING

## Contribution Overview
Fixtures are edited only through the doc-rules test suite.

## Contribution Guidelines
Keep every document in the closed H2 layout its template defines.

## Workflow

| Step | Action | Detail |
|---|---|---|
| 1 | Edit a document | Preserve the closed H2 set |
| 2 | Run the scanner | `lint-arwaky-cli docs .` |
| 3 | Verify | The fixture still fails the way the test expects |

## Commit Convention

| Type | Usage | Example |
|---|---|---|
| fix | Repair a fixture document | `fix(fixture): close the ROADMAP H2 set` |

## Reference

| Artifact | Purpose | Link |
|---|---|---|
| PRD | Product requirements | [PRD.md](PRD.md) |
| ARCHITECTURE | Layer contract | [ARCHITECTURE.md](ARCHITECTURE.md) |

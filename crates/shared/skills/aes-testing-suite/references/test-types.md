# Test types

The **test types** decide which prefixes, which directories, and which coverage floor apply.

## Test chain

`contract_` (seam exists) → `unit_` / `integration_` (behaviour + wiring) → `dogfood_` (live service validation, skip in CI) → `smoke_` / `e2e_` / `acceptance_` (app + requirement) → `bench_` (nightly)

Each prefix answers one question. A test in the wrong directory or with the wrong prefix is the defect this skill exists to prevent.

## Test-type reference

| Prefix | Directory | Scope | Speed | Runs when |
| ------ | --------- | ----- | ----- | --------- |
| `contract_` | tests/ | Protocol/trait/interface impl exists | ms | Every PR |
| `unit_` | tests/ | One public function | ms | Every PR |
| `integration_` | tests/ | Module / crate / package + DI wiring | ms–s | Every PR |
| `dogfood_` | tests/ | CLI against live service/session; skip if unavailable | s–min | Local only; skip CI |
| `smoke_` | tests/ | App boots + responds | <5s | Every PR |
| `e2e_` | tests/ | Full request lifecycle | s | Every PR (critical path) |
| `acceptance_` | tests/ | Business requirement met | s | Every PR / release gate |
| `bench_` | benches/ | Performance regression | s–min | Release gate / nightly |

Coverage targets: capabilities 70% · agent 60% · utility 50%.

## Enforcement

Both halves of this layout are machine-checked. Run the linter rather than eyeballing a tree.

| Code | Crate | Question it answers |
| ---- | ----- | ------------------- |
| AES103 | `naming-rules` | Is each file's prefix legal, and does either directory nest? |
| AES704 | `structure-rules` | Is every category present — one file per type, plus the benchmark? |

```bash
lint-arwaky-cli scan .          # both fire on a non-conforming layout
```

Two consequences worth knowing before you name a file:

- **Support prefixes are legal but satisfy no category.** `regression_`, `behavioral_`, `mock_`, and `fixture_` pass AES103 because the repository already follows those conventions (TEST.md §2.0 requires `regression_`), but a folder carrying only regression guards still owes all seven test types under AES704.
- **Every file under `tests/` carries a legal prefix at the flat top level.** There are no sanctioned subdirectories — the prefix is the virtual folder.

Split directory tree, config file, and run commands: **read the language HOW-TO** — do not restate them here.

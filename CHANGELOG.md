# Changelog

This changelog shows the current release in full plus a summary of the previous release.
Older releases are available as git tags.
## [3.17.1] - 2026-11-21

### Fixed

- *(orphan-rules, doc-rules)* Add TypeScript support for AES609 and orphan barrel fallbacks (#1446)
- *(import-resolution)* Resolve .js/.jsx/.ts/.tsx specifiers across extensions (#1448)
- *(role-rules)* Skip AES405 for single-protocol features (fixes #1456)
- *(mergify)* Use lint check name instead of self-lint-aes
- *(mergify)* Match check-success condition names to CI job names

### Other

- Release workflow, cliff.toml, bump.sh git-cliff integration
# Changelog
## 3.17.0 (2026-11-17)

### Features

- feat(naming-rules): ignore conftest.py by default in AES103 (#1441)

### Bug Fixes

- fix(structure-rules): exempt shared folders from PR id requirement (AES706) (#1438)
- fix: resolve Cargo.toml lint warnings (missing [lints] inheritance, redundant homepage) (#1429)
- fix(orphan-rules): resolve remaining UX issues AES501-AES506 (#1385) (#1421)
- fix(doc-rules): resolve all UX issues AES601-AES610 (#1397) (#1422)
- fix(AES609): count Python protocol classes with base classes correctly (#1420)

### Maintenance

- chore(ci): speed up CI jobs (#1389)
- chore(ci): enable sccache in coverage job (#1387)

## Unreleased

### Features

- feat(naming-rules): one finding template for AES101/AES102/AES103 - the problem, the action and the rule in three distinct fields, one clause per line, ASCII only (#1378)
- feat(naming-rules): per-rule inspected counts and a naming summary, so a clean run is distinguishable from a run that evaluated nothing (#1378)
- feat(naming-rules): evaluate every file by path, including a file with no content, so a zero-byte file with a broken name is no longer reported as clean (#1378)
- feat(naming-rules): name the exemption list in every fix, and derive the worked example from the directory the finding reports (#1378)

### Bug Fixes

- fix(structure-rules): rename the AES706 violation names off lint-arwaky's own segment — `pr_la_id_mismatch` → `pr_id_mismatch`, `pr_la_missing_frd` → `pr_missing_frd`, `pr_la_fr_id_mismatch` → `pr_fr_id_mismatch`, `pr_la_folder_without_pr_la_id` → `pr_folder_without_id` — so a project using `PR-PSD-001` or `PR-CALC-001` ids is not told its violation is about `la` (#1375)
- fix(structure-rules): drop non-crate directories (`__pycache__`, `node_modules`, `target`, `dist`, `.venv`, …) from the AES706 crate folder order, so a Python bytecode cache no longer takes position 0 and shifts every `PR-<segment>` id onto its predecessor's folder (#1375)
- fix(structure-rules): scope the AES706 folder order per member dir, and audit one `PR-<segment>` sequence per language against its own member's folders instead of one flattened multi-language list (#1375)

### Documentation

- docs(naming-rules): add the developer journey, one rendered output sample per rule, the severity model, the suppression section and the 72-of-80 column contract (#1378)
- docs(architecture): retitle AES706 "PR ID Folder Mapping" and list its current violation names, segment-neutral wording, and per-member folder order (#1375)

### Removed

- feat(external-lint): remove the markdownlint adapter from external-lint. The `markdownlint` adapter entry and `markdownlint::*` ignored rules in config files are now ignored; markdown files are no longer linted by markdownlint.


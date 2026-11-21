# Changelog

This changelog shows the current release in full plus a summary of the previous release.
Older releases are available as git tags.
## [3.18.0] - 2026-11-21

### Fixed

- *(orphan-rules, doc-rules)* Add TypeScript support for AES609 and orphan barrel fallbacks (#1446)
- *(import-resolution)* Resolve .js/.jsx/.ts/.tsx specifiers across extensions (#1448)
- *(role-rules)* Skip AES405 for single-protocol features (fixes #1456)
- *(mergify)* Use lint check name instead of self-lint-aes
- *(mergify)* Match check-success condition names to CI job names

### Other

- Release workflow, cliff.toml, bump.sh git-cliff integration

## [3.17.0] - 2026-11-17

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

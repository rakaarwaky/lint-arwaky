# Changelog

This changelog shows the current release in full plus a summary of the previous release.
Older releases are available as git tags.
## [3.18.0] - 2026-11-21

### Fixed

- *(orphan-rules, doc-rules)* Add TypeScript support for AES609 and orphan barrel fallbacks (#1446)
- *(import-resolution)* Resolve .js/.jsx/.ts/.tsx specifiers across extensions (#1448)
- *(role-rules)* Skip AES405 for single-protocol features (fixes #1456)
## [3.17.0] - 2026-11-17

### New

- *(naming-rules)* Ignore conftest.py by default in AES103 (#1441)

### Fixed

- *(AES609)* Count Python protocol classes with base classes correctly (#1420)
- *(doc-rules)* Resolve all UX issues AES601-AES610 (#1397) (#1422)
- *(orphan-rules)* Resolve remaining UX issues AES501-AES506 (#1385) (#1421)
- Resolve Cargo.toml lint warnings (missing [lints] inheritance, redundant homepage) (#1429)
- *(structure-rules)* Exempt shared folders from PR id requirement (AES706) (#1438)
- *(release)* Capture scan exit code in smoke test (#1444)

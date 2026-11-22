# Changelog

This changelog shows the current release in full plus a summary of the previous release.
Older releases are available as git tags.
## [3.18.1] - 2026-11-22

### Fixed

- *(cli-commands)* Fix bench compile for ScanRequest/ScanMode path and LintResult fields
- Remove unused root dependencies to resolve Clippy and Security Audit
- Remove unused workspace dependencies to resolve Clippy warnings
## [3.18.0] - 2026-11-21

### Fixed

- *(orphan-rules, doc-rules)* Add TypeScript support for AES609 and orphan barrel fallbacks (#1446)
- *(import-resolution)* Resolve .js/.jsx/.ts/.tsx specifiers across extensions (#1448)
- *(role-rules)* Skip AES405 for single-protocol features (fixes #1456)

# Feature Backlog: Filesystem

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 5 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p filesystem --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p filesystem --lib --tests` after any code change to this crate

## Backlog Status

### FR-Filesystem-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Parse Source Files & Extract Imports (`IParserProtocol`); source: `crates/filesystem/src/root_filesystem_container.rs`, `crates/filesystem/src/capabilities_ast_parser.rs`; tests: `crates/filesystem/tests/unit_filesystem_ast_parser.rs`, `crates/config-system/tests/acceptance_FR_001.rs`, `crates/filesystem/tests/unit_filesystem_import_extractor.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Filesystem-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Build the Project Dependency Graph (`IGraphProtocol`); source: `crates/filesystem/src/capabilities_dependency_graph.rs`, `crates/filesystem/src/root_filesystem_container.rs`; tests: `crates/filesystem/tests/unit_filesystem_import_resolution.rs`, `crates/config-system/tests/acceptance_FR_002.rs`, `crates/filesystem/tests/unit_filesystem_ignored_paths.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Filesystem-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Read, Write, Discover & Mutate the File Tree (`IFileSystemIOProtocol, `IsMemberPath`); source: `crates/filesystem/src/root_filesystem_container.rs`, `crates/filesystem/src/capabilities_filesystem_io.rs`; tests: `crates/filesystem/tests/unit_filesystem_io.rs`, `crates/config-system/tests/acceptance_FR_003.rs`, `crates/filesystem/tests/unit_filesystem_ast_python.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Filesystem-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Resolve External Tool Binaries (`IToolResolutionProtocol`); source: `crates/filesystem/src/capabilities_tool_resolution.rs`, `crates/filesystem/src/root_filesystem_container.rs`; tests: `crates/filesystem/tests/unit_filesystem_import_resolution.rs`, `crates/config-system/tests/acceptance_FR_004.rs`, `crates/filesystem/tests/unit_filesystem_tool_resolution.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### FR-Filesystem-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Detect Workspace Root & Project Languages (`IWorkspaceProtocol, `IFilesystemAggregate`); source: `crates/filesystem/src/root_filesystem_container.rs`, `crates/filesystem/src/agent_filesystem_orchestrator.rs`; tests: `crates/filesystem/tests/unit_filesystem_import_resolution.rs`, `crates/filesystem/tests/unit_filesystem_container_wiring.rs`, `crates/filesystem/tests/unit_filesystem_project_languages.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |

---
trigger: always
description: "Gap workspace agent guide."
---
# AGENTS

## Agent Overview
This workspace is a scanner fixture; do not add implementation code.

## Runtime

| Aspect | Detail|
|---|---|
| Language | None; fixture only|
| Environment | No build or test commands are defined|
| Artifacts | None|
| Package Manager | None|
| Version Probe | None|
| Setup | None|
| Activation | None|

## Project Structure

```text
.
├── crates/
│   ├── cli_commands/
│   ├── code_analysis/
│   ├── di_containers/
│   ├── import_violations/
│   ├── layer_rules/
│   ├── naming_violations/
│   ├── orphan_feature/
│   ├── shared/
│   ├── shared_common/
│   ├── surface_orphan_test/
│   ├── taxonomy/
│   ├── test_suite_violation/
│   └── utility_orphan_test/
├── modules/
│   ├── cli_commands/
│   ├── code_analysis/
│   ├── contract/
│   ├── di_containers/
│   ├── import_violations/
│   ├── language_adapters/
│   ├── layer_rules/
│   ├── naming_violations/
│   ├── shared/
│   ├── shared_common/
│   ├── surface_layer/
│   └── test_suite_violation/
├── packages/
│   ├── cli_commands/
│   ├── import_violations/
│   ├── layer_rules/
│   ├── naming_violations/
│   ├── shared/
│   ├── shared_common/
│   ├── surface_layer_probe/
│   └── test_suite_violation/
└── AGENTS.md
```

## Commands

| Category | Command | Scope | CI Gate|
|---|---|---|---|
| Scan | `lint-arwaky-cli docs .` | whole tree | —|

## Documentation Awareness

| Document | Agent Must|
|---|---|
| PRD | Read before touching the fixture documents|

## Reference

| Artifact | Purpose | Link|
|---|---|---|
| PRD | Product requirements | [PRD.md](PRD.md)|
| FRD | Feature requirements | [FRD.md](FRD.md)|
| DATA | Data contracts | [DATA.md](DATA.md)|
| DESIGN | Design contracts | [DESIGN.md](DESIGN.md)|
| ARCHITECTURE | Layer contract | [ARCHITECTURE.md](ARCHITECTURE.md)|

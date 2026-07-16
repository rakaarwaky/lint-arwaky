# Lint Arwaky Skills

Skills for lint-arwaky MCP server and CLI. Each skill is a self-contained directory with a `SKILL.md` file defining rules, patterns, and workflows.

## Structure

```
.agents/skills/
├── build-verify/          # Build + test quality gates (Rust/Cargo)
├── add-docs-python/       # Docstrings and type hints (PEP 257)
├── add-docs-rust/         # Docstrings and documentation comments
├── cleanup-files-python/  # PEP 8 formatting, unused imports (Black/pycodestyle)
├── cleanup-files-rust/    # cargo fmt + clippy cleanup
├── consolidate-files-python/  # Module consolidation (single responsibility)
├── consolidate-files-rust/      # Crate/file consolidation
├── create-agent-python/     # Agent layer: orchestration only, aggregate ABCs
├── create-agent-rust/       # Agent layer: orchestration only, aggregate traits
├── create-capabilities-python/  # Capabilities: protocol ABCs, zero I/O
├── create-capabilities-rust/      # Capabilities: protocol traits, zero I/O
├── create-contract-python/    # Contract layer: pure ABC definitions
├── create-contract-rust/        # Contract layer: pure trait definitions
├── create-infrastructure-python/  # Infrastructure: port ABCs, zero business logic
├── create-infrastructure-rust/      # Infrastructure: port traits, zero business logic
├── create-surface-python/       # Surface: smart/utility/passive types, AES406
├── create-surface-rust/         # Surface: smart/utility/passive types, AES406
├── create-taxonomy-python/      # Taxonomy: strict suffixes, dataclass rules
├── create-taxonomy-rust/        # Taxonomy: strict suffixes, derive macros
├── create-test-python/          # pytest conventions, fixtures, mocking
├── create-test-rust/            # cargo test, integration tests, mocks
├── fix-bypass-python/           # Remove type:ignore/noqa — fix root causes
├── fix-bypass-rust/             # Remove clippy/cargo deny suppressions
├── lint-arwaky-python/          # CLI scanner and MCP server for Python
└── lint-arwaky-rust/            # CLI scanner and MCP server for Rust
```

## Categories

### AES Layer Skills (6 pairs)

Core architecture skills following the 7-layer AES model:

| Skill                     | Purpose                                                                                              |
| ------------------------- | ---------------------------------------------------------------------------------------------------- |
| `create-capabilities-*`   | Protocol traits/ABCs, zero I/O, 3-block structure                                                    |
| `create-infrastructure-*` | Port traits/ABCs, zero business logic, I/O only                                                      |
| `create-agent-*`          | Aggregate traits/ABCs, orchestration only, zero computation/I/O/business                             |
| `create-taxonomy-*`       | Strict suffixes (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`), dataclass placement |
| `create-contract-*`       | Pure trait/ABC definitions with `_port`, `_protocol`, `_aggregate` suffixes                          |
| `create-surface-*`        | Smart/utility/passive types, AES406 role violations                                                  |

### Utility Skills (6 pairs)

Cross-cutting concerns:

| Skill                 | Purpose                                                    |
| --------------------- | ---------------------------------------------------------- |
| `add-docs-*`          | Docstrings and documentation comments                      |
| `cleanup-files-*`     | Formatting, unused imports, PEP 8 / cargo fmt + clippy     |
| `consolidate-files-*` | Module/crate consolidation following single responsibility |
| `create-test-*`       | Test conventions, fixtures, mocking, coverage              |
| `fix-bypass-*`        | Remove suppression comments, fix root causes               |
| `lint-arwaky-*`       | CLI scanner and MCP server for project validation          |

### Quality Gates (1)

| Skill          | Purpose                                                                                  |
| -------------- | ---------------------------------------------------------------------------------------- |
| `build-verify` | Build workspace, verify clean compile, run tests. Decision tree for fast feedback loops. |

## Naming Convention

All skills follow the pattern `<action>-<language>`:

- **Action**: What the skill does (create, fix, cleanup, add, lint, etc.)
- **Language**: Target language (`python` or `rust`)

Skills are organized flat — one level deep under `.agents/skills/`.

## Usage

Each skill is invoked by trigger keywords defined in its `SKILL.md` file. The AI agent matches user requests to the appropriate skill based on triggers and context.

## AES Architecture Reference

Full 7-layer specification: [ARCHITECTURE.md](../../ARCHITECTURE.md)
